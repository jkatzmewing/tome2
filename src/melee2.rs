use ::libc;
extern "C" {
    #[no_mangle]
    static mut ddd: [s16b; 9];
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut ddx_ddd: [s16b; 9];
    #[no_mangle]
    static mut ddy_ddd: [s16b; 9];
    #[no_mangle]
    static mut extract_energy: [byte_hack; 300];
    #[no_mangle]
    static mut inscription_info: [inscription_info_type; 8];
    #[no_mangle]
    static mut alive: bool_;
    #[no_mangle]
    static mut death: bool_;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut num_repro: s16b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut hack_m_idx: s16b;
    #[no_mangle]
    static mut total_friends: libc::c_int;
    #[no_mangle]
    static mut total_friend_levels: s32b;
    #[no_mangle]
    static mut summon_kin_type: libc::c_char;
    #[no_mangle]
    static mut disturb_near: bool_;
    #[no_mangle]
    static mut disturb_move: bool_;
    #[no_mangle]
    static mut disturb_minor: bool_;
    #[no_mangle]
    static mut disturb_other: bool_;
    #[no_mangle]
    static mut stupid_monsters: bool_;
    #[no_mangle]
    static mut disturb_pets: bool_;
    #[no_mangle]
    static mut flow_by_sound: bool_;
    #[no_mangle]
    static mut smart_learn: bool_;
    #[no_mangle]
    static mut smart_cheat: bool_;
    #[no_mangle]
    static mut testing_carry: bool_;
    #[no_mangle]
    static mut speak_unique: bool_;
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
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut max_o_idx: u16b;
    #[no_mangle]
    static mut doppleganger: s16b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut process_hooks_return: [hook_return; 20];
    #[no_mangle]
    fn process_hooks_ret(h_idx: libc::c_int, ret: *mut libc::c_char,
                         fmt: *mut libc::c_char, _: ...) -> bool_;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn los(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int, x2: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn place_floor_convert_glass(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn mmove2(y: *mut libc::c_int, x: *mut libc::c_int, y1: libc::c_int,
              x1: libc::c_int, y2: libc::c_int, x2: libc::c_int);
    #[no_mangle]
    fn projectable(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                   x2: libc::c_int) -> bool_;
    #[no_mangle]
    fn disturb(stop_search: libc::c_int, flush_output: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execute_inscription(i: byte_hack, y: byte_hack, x: byte_hack) -> bool_;
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn vstrnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr,
                vp: ::std::ffi::VaList) -> uint_hack;
    #[no_mangle]
    fn strfmt(buf: *mut libc::c_char, fmt: cptr, _: ...) -> uint_hack;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    fn get_xtra_line(file_name: *mut libc::c_char, m_ptr: *mut monster_type,
                     output: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn delete_monster_idx(i: libc::c_int);
    #[no_mangle]
    fn monster_death(m_idx: libc::c_int);
    #[no_mangle]
    fn gain_exp(amount: s32b);
    #[no_mangle]
    fn get_skill_scale(skill: libc::c_int, scale: u32b) -> s16b;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn monster_gain_exp(m_idx: libc::c_int, exp: u32b, silent: bool_);
    #[no_mangle]
    fn message_add(type_0: byte_hack, msg: cptr, color: byte_hack);
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn get_attack_power(effect: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn make_attack_normal(m_idx: libc::c_int, divis: byte_hack) -> bool_;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn summon_specific(y1: libc::c_int, x1: libc::c_int, lev: libc::c_int,
                       type_0: libc::c_int) -> bool_;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn summon_cyber();
    #[no_mangle]
    fn lose_all_info() -> bool_;
    #[no_mangle]
    fn trap_creation() -> bool_;
    #[no_mangle]
    fn unlite_area(dam: libc::c_int, rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn update_smart_learn(m_idx: libc::c_int, what: libc::c_int);
    #[no_mangle]
    fn teleport_player_level();
    #[no_mangle]
    fn teleport_player(dis: libc::c_int);
    #[no_mangle]
    fn teleport_player_to(ny: libc::c_int, nx: libc::c_int);
    #[no_mangle]
    fn teleport_away(m_idx: libc::c_int, dis: libc::c_int);
    #[no_mangle]
    fn quark_str(num: s16b) -> cptr;
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_slow(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_blind(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_afraid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn project(who: libc::c_int, rad: libc::c_int, y: libc::c_int,
               x: libc::c_int, dam: libc::c_int, typ: libc::c_int,
               flg: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_cut(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_image(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_dec_stat(stat: libc::c_int, mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn take_sanity_hit(damage: libc::c_int, hit_from: cptr);
    #[no_mangle]
    fn aggravate_monsters(who: libc::c_int);
    #[no_mangle]
    fn is_friend(m_ptr: *mut monster_type) -> libc::c_int;
    #[no_mangle]
    fn delete_object_idx(o_idx: libc::c_int);
    #[no_mangle]
    fn excise_object_idx(o_idx: libc::c_int);
    #[no_mangle]
    fn ai_possessor(m_idx: libc::c_int, o_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn mon_hit_trap(_: libc::c_int) -> bool_;
    #[no_mangle]
    fn update_mon(m_idx: libc::c_int, full: bool_);
    #[no_mangle]
    fn monster_can_cross_terrain(feat: byte_hack, r_ptr: *mut monster_race)
     -> bool_;
    #[no_mangle]
    static mut hack_message_pain_may_silent: bool_;
    #[no_mangle]
    fn earthquake(cy: libc::c_int, cx: libc::c_int, r: libc::c_int);
    #[no_mangle]
    fn is_enemy(m_ptr: *mut monster_type, t_ptr: *mut monster_type) -> bool_;
    #[no_mangle]
    fn delete_monster(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn get_pos_player(dis: libc::c_int, ny: *mut libc::c_int,
                      nx: *mut libc::c_int);
    #[no_mangle]
    fn fire_ball(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int,
                 rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn summon_specific_friendly(y1: libc::c_int, x1: libc::c_int,
                                lev: libc::c_int, type_0: libc::c_int,
                                Group_ok: bool_) -> bool_;
    #[no_mangle]
    fn unlite_room(y1: libc::c_int, x1: libc::c_int);
    #[no_mangle]
    fn ai_multiply(m_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn change_side(m_ptr: *mut monster_type) -> bool_;
    #[no_mangle]
    fn mon_take_hit(m_idx: libc::c_int, dam: libc::c_int, fear: *mut bool_,
                    note: cptr) -> bool_;
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
pub struct inscription_info_type {
    pub text: [libc::c_char; 40],
    pub when: byte_hack,
    pub know: bool_,
    pub mana: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union hook_return {
    pub num: s32b,
    pub str_0: cptr,
    pub o_ptr: *mut object_type,
    pub m_ptr: *mut monster_type,
}
/* File: melee2.c */
/* Purpose: Monster spells and movement */
/*
* Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
*
* This software may be copied and distributed for educational, research, and
* not for profit purposes provided that this copyright and statement are
* included in all such copies.
*/
/*
* This file has several additions to it by Keldon Jones (keldon@umr.edu)
* to improve the general quality of the AI (version 0.1.1).
*/
/*
 * Based on mon_take_hit... all monster attacks on
 * other monsters should use
 */
#[no_mangle]
pub unsafe extern "C" fn mon_take_hit_mon(mut s_idx: libc::c_int,
                                          mut m_idx: libc::c_int,
                                          mut dam: libc::c_int,
                                          mut fear: *mut bool_,
                                          mut note: cptr) -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut s_ptr: *mut monster_type =
        &mut *m_list.offset(s_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut div: s32b = 0;
    let mut new_exp: s32b = 0;
    let mut new_exp_frac: s32b = 0;
    /* Redraw (later) if needed */
    if health_who as libc::c_int == m_idx {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x800 as libc::c_long) as u32b
    }
    /* Some mosnters are immune to death */
    if (*r_ptr).flags7 & 0x400 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Wake it up */
    (*m_ptr).csleep = 0 as libc::c_int as s16b;
    /* Hurt it */
    (*m_ptr).hp -= dam;
    /* It is dead now... or is it? */
    if (*m_ptr).hp < 0 as libc::c_int {
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
               (*m_ptr).status as libc::c_int <= 1 as libc::c_int ||
               (*m_ptr).mflag & 0x2 as libc::c_int != 0 {
            (*m_ptr).hp = 1 as libc::c_int
        } else {
            let mut m_name: [libc::c_char; 80] = [0; 80];
            let mut dive: s32b = (*s_ptr).level as s32b;
            if dive == 0 { dive = 1 as libc::c_int }
            /* Extract monster name */
            monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
            /* Make a sound */
            if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 ||
                   (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
                   ||
                   (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0
                   ||
                   (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0
                   ||
                   !strchr(b"Evg\x00" as *const u8 as *const libc::c_char,
                           (*r_ptr).d_char as libc::c_int).is_null() {
                sound(30 as libc::c_int);
            } else { sound(5 as libc::c_int); }
            /* Death by Missile/Spell attack */
            if !note.is_null() {
                cmonster_msg(12 as libc::c_int as libc::c_char,
                             b"%^s%s\x00" as *const u8 as *const libc::c_char,
                             m_name.as_mut_ptr(), note);
            } else if !((*m_ptr).ml == 0) {
                /* Death by Physical attack -- living monster */
                /* Death by Physical attack -- non-living monster */
                if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
                       ||
                       (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint
                           != 0 ||
                       (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint !=
                           0 ||
                       (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint
                           != 0 ||
                       !strchr(b"Evg\x00" as *const u8 as *const libc::c_char,
                               (*r_ptr).d_char as libc::c_int).is_null() {
                    cmonster_msg(12 as libc::c_int as libc::c_char,
                                 b"%^s is destroyed.\x00" as *const u8 as
                                     *const libc::c_char,
                                 m_name.as_mut_ptr());
                } else {
                    cmonster_msg(12 as libc::c_int as libc::c_char,
                                 b"%^s is killed.\x00" as *const u8 as
                                     *const libc::c_char,
                                 m_name.as_mut_ptr());
                }
            }
            dive = (*r_ptr).mexp * (*m_ptr).level as libc::c_int / dive;
            if dive == 0 { dive = 1 as libc::c_int }
            /* Monster gains some xp */
            monster_gain_exp(s_idx, dive as u32b, 0 as libc::c_int as bool_);
            /* Monster lore skill allows gaining xp from pets */
            if get_skill(48 as libc::c_int) as libc::c_int != 0 &&
                   (*s_ptr).status as libc::c_int >= 3 as libc::c_int {
                /* Maximum player level */
                div = (*p_ptr).max_plv as s32b;
                /* Give some experience for the kill */
                new_exp =
                    ((*r_ptr).mexp as libc::c_long *
                         (*m_ptr).level as libc::c_long / div as libc::c_long)
                        as s32b;
                /* Handle fractional experience */
                new_exp_frac =
                    ((*r_ptr).mexp as libc::c_long *
                         (*m_ptr).level as libc::c_long % div as libc::c_long
                         * 0x10000 as libc::c_long / div as libc::c_long +
                         (*p_ptr).exp_frac as libc::c_long) as s32b;
                /* Keep track of experience */
                if new_exp_frac as libc::c_long >= 0x10000 as libc::c_long {
                    new_exp += 1;
                    (*p_ptr).exp_frac =
                        (new_exp_frac - 0x10000 as libc::c_int) as u16b
                } else { (*p_ptr).exp_frac = new_exp_frac as u16b }
                /*
				 * Factor the xp by the skill level
				 * Note that a score of 50 in the skill makes the gain be 120% of the exp
				 */
                new_exp =
                    new_exp *
                        get_skill_scale(48 as libc::c_int,
                                        120 as libc::c_int as u32b) as
                            libc::c_int / 100 as libc::c_int;
                /* Gain experience */
                gain_exp(new_exp);
            }
            /* When an Unique dies, it stays dead */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
                (*r_ptr).max_num = 0 as libc::c_int as s16b
            }
            /* Generate treasure */
            monster_death(m_idx);
            /* Delete the monster */
            delete_monster_idx(m_idx);
            /* Not afraid */
            *fear = 0 as libc::c_int as bool_;
            /* Monster is dead */
            return 1 as libc::c_int as bool_
        }
    }
    /* Mega-Hack -- Pain cancels fear */
    if (*m_ptr).monfear as libc::c_int != 0 && dam > 0 as libc::c_int {
        let mut tmp: libc::c_int = Rand_div(dam) + 1 as libc::c_int;
        /* Cure a little fear */
        if tmp < (*m_ptr).monfear as libc::c_int {
            /* Reduce fear */
            (*m_ptr).monfear =
                ((*m_ptr).monfear as libc::c_int - tmp) as byte_hack
        } else {
            /* Cure all the fear */
            /* Cure fear */
            (*m_ptr).monfear = 0 as libc::c_int as byte_hack;
            *fear = 0 as libc::c_int as bool_
        }
    }
    /* No more fear */
    /* Sometimes a monster gets scared by damage */
    if (*m_ptr).monfear == 0 &&
           (*r_ptr).flags3 & 0x10000000 as libc::c_int as libc::c_uint == 0 {
        let mut percentage: libc::c_int = 0;
        /* Percentage of fully healthy */
        percentage =
            (100 as libc::c_long * (*m_ptr).hp as libc::c_long /
                 (*m_ptr).maxhp as libc::c_long) as libc::c_int;
        /*
		* Run (sometimes) if at 10% or less of max hit points,
		* or (usually) when hit for half its current hit points
		*/
        if percentage <= 10 as libc::c_int &&
               Rand_div(10 as libc::c_int) < percentage ||
               dam >= (*m_ptr).hp &&
                   Rand_div(100 as libc::c_int) < 80 as libc::c_int {
            /* Hack -- note fear */
            *fear = 1 as libc::c_int as bool_;
            /* XXX XXX XXX Hack -- Add some timed fear */
            (*m_ptr).monfear =
                (Rand_div(10 as libc::c_int) + 1 as libc::c_int +
                     (if dam >= (*m_ptr).hp && percentage > 7 as libc::c_int {
                          20 as libc::c_int
                      } else {
                          (11 as libc::c_int - percentage) * 5 as libc::c_int
                      })) as byte_hack
        }
    }
    /* ALLOW_FEAR */
    /* Not dead yet */
    return 0 as libc::c_int as bool_;
}
/*
* And now for Intelligent monster attacks (including spells).
*
* Original idea and code by "DRS" (David Reeves Sward).
* Major modifications by "BEN" (Ben Harrison).
*
* Give monsters more intelligent attack/spell selection based on
* observations of previous attacks on the player, and/or by allowing
* the monster to "cheat" and know the player status.
*
* Maintain an idea of the player status, and use that information
* to occasionally eliminate "ineffective" spell attacks.  We could
* also eliminate ineffective normal attacks, but there is no reason
* for the monster to do this, since he gains no benefit.
* Note that MINDLESS monsters are not allowed to use this code.
* And non-INTELLIGENT monsters only use it partially effectively.
*
* Actually learn what the player resists, and use that information
* to remove attacks or spells before using them.  This will require
* much less space, if I am not mistaken.  Thus, each monster gets a
* set of 32 bit flags, "smart", build from the various "SM_*" flags.
*
* This has the added advantage that attacks and spells are related.
* The "smart_learn" option means that the monster "learns" the flags
* that should be set, and "smart_cheat" means that he "knows" them.
* So "smart_cheat" means that the "smart" field is always up to date,
* while "smart_learn" means that the "smart" field is slowly learned.
* Both of them have the same effect on the "choose spell" routine.
*/
/*
* Internal probability routine
*/
unsafe extern "C" fn int_outof(mut r_ptr: *mut monster_race,
                               mut prob: libc::c_int) -> bool_ {
    /* Non-Smart monsters are half as "smart" */
    if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint == 0 {
        prob = prob / 2 as libc::c_int
    }
    /* Roll the dice */
    return (Rand_div(100 as libc::c_int) < prob) as libc::c_int as bool_;
}
/*
 * Remove the "bad" spells from a spell list
 */
unsafe extern "C" fn remove_bad_spells(mut m_idx: libc::c_int,
                                       mut f4p: *mut u32b, mut f5p: *mut u32b,
                                       mut f6p: *mut u32b) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut f4: u32b = *f4p;
    let mut f5: u32b = *f5p;
    let mut f6: u32b = *f6p;
    let mut smart: u32b = 0 as libc::c_long as u32b;
    /* Too stupid to know anything */
    if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 { return }
    /* Must be cheating or learning */
    if smart_cheat == 0 && smart_learn == 0 { return }
    /* Update acquired knowledge */
    if smart_learn != 0 {
        /* Hack -- Occasionally forget player status */
        if (*m_ptr).smart != 0 &&
               Rand_div(100 as libc::c_int) < 1 as libc::c_int {
            (*m_ptr).smart = 0 as libc::c_long as u32b
        }
        /* Use the memorized flags */
        smart = (*m_ptr).smart
    }
    /* Cheat if requested */
    if smart_cheat != 0 {
        /* Know basic info */
        if (*p_ptr).resist_acid != 0 {
            smart |= 0x1 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).oppose_acid != 0 {
            smart |= 0x10000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).immune_acid != 0 {
            smart |= 0x1000000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_elec != 0 {
            smart |= 0x2 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).oppose_elec != 0 {
            smart |= 0x20000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).immune_elec != 0 {
            smart |= 0x2000000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_fire != 0 {
            smart |= 0x4 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).oppose_fire != 0 {
            smart |= 0x40000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).immune_fire != 0 {
            smart |= 0x4000000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_cold != 0 {
            smart |= 0x8 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).oppose_cold != 0 {
            smart |= 0x80000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).immune_cold != 0 {
            smart |= 0x8000000 as libc::c_int as libc::c_uint
        }
        /* Know poison info */
        if (*p_ptr).resist_pois != 0 {
            smart |= 0x10 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).oppose_pois != 0 {
            smart |= 0x100000 as libc::c_int as libc::c_uint
        }
        /* Know special resistances */
        if (*p_ptr).resist_neth != 0 {
            smart |= 0x20 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_lite != 0 {
            smart |= 0x40 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_dark != 0 {
            smart |= 0x80 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_fear != 0 {
            smart |= 0x100 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_conf != 0 {
            smart |= 0x200 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_chaos != 0 {
            smart |= 0x400 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_disen != 0 {
            smart |= 0x800 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_blind != 0 {
            smart |= 0x1000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_nexus != 0 {
            smart |= 0x2000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_sound != 0 {
            smart |= 0x4000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).resist_shard != 0 {
            smart |= 0x8000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).reflect != 0 {
            smart |= 0x20000000 as libc::c_int as libc::c_uint
        }
        /* Know bizarre "resistances" */
        if (*p_ptr).free_act != 0 {
            smart |= 0x40000000 as libc::c_int as libc::c_uint
        }
        if (*p_ptr).msp == 0 { smart |= 0x80000000 as libc::c_uint }
    }
    /* Nothing known */
    if smart == 0 { return }
    if smart & 0x1000000 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f4 &= !(0x100 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x1 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x10000 as libc::c_int) as libc::c_uint
        }
    } else if smart & 0x10000 as libc::c_int as libc::c_uint != 0 &&
                  smart & 0x1 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f4 &= !(0x100 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f5 &= !(0x1 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f5 &= !(0x10000 as libc::c_int) as libc::c_uint
        }
    } else if smart & 0x10000 as libc::c_int as libc::c_uint != 0 ||
                  smart & 0x1 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f4 &= !(0x100 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f5 &= !(0x1 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f5 &= !(0x10000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x2000000 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f4 &= !(0x200 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x2 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x20000 as libc::c_int) as libc::c_uint
        }
    } else if smart & 0x20000 as libc::c_int as libc::c_uint != 0 &&
                  smart & 0x2 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f4 &= !(0x200 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f5 &= !(0x2 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f5 &= !(0x20000 as libc::c_int) as libc::c_uint
        }
    } else if smart & 0x20000 as libc::c_int as libc::c_uint != 0 ||
                  smart & 0x2 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f4 &= !(0x200 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f5 &= !(0x2 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f5 &= !(0x20000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x4000000 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f4 &= !(0x400 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x4 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x40000 as libc::c_int) as libc::c_uint
        }
    } else if smart & 0x40000 as libc::c_int as libc::c_uint != 0 &&
                  smart & 0x4 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f4 &= !(0x400 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f5 &= !(0x4 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f5 &= !(0x40000 as libc::c_int) as libc::c_uint
        }
    } else if smart & 0x40000 as libc::c_int as libc::c_uint != 0 ||
                  smart & 0x4 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f4 &= !(0x400 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f5 &= !(0x4 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f5 &= !(0x40000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x8000000 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f4 &= !(0x800 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x8 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x80000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x2000000 as libc::c_int) as libc::c_uint
        }
    } else if smart & 0x80000 as libc::c_int as libc::c_uint != 0 &&
                  smart & 0x8 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f4 &= !(0x800 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f5 &= !(0x8 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f5 &= !(0x80000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f5 &= !(0x2000000 as libc::c_int) as libc::c_uint
        }
    } else if smart & 0x80000 as libc::c_int as libc::c_uint != 0 ||
                  smart & 0x8 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f4 &= !(0x800 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f5 &= !(0x8 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f5 &= !(0x80000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f5 &= !(0x2000000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x100000 as libc::c_int as libc::c_uint != 0 &&
           smart & 0x10 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f4 &= !(0x1000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 80 as libc::c_int) != 0 {
            f5 &= !(0x10 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 40 as libc::c_int) != 0 {
            f4 &= !(0x10000000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 40 as libc::c_int) != 0 {
            f4 &= !(0x20000000 as libc::c_int) as libc::c_uint
        }
    } else if smart & 0x100000 as libc::c_int as libc::c_uint != 0 ||
                  smart & 0x10 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f4 &= !(0x1000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 30 as libc::c_int) != 0 {
            f5 &= !(0x10 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x20 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f4 &= !(0x2000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f5 &= !(0x20 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f5 &= !(0x200000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x40 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f4 &= !(0x4000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x80 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f4 &= !(0x8000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f5 &= !(0x100 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x100 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x8000000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x200 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x20000000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f4 &= !(0x10000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x400 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x20000000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f4 &= !(0x10000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f4 &= !(0x40000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f4 &= !(0x40000000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x800 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f4 &= !(0x80000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x1000 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x10000000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x2000 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f4 &= !(0x100000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f6 &= !(0x100 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x4000 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f4 &= !(0x20000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x8000 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 50 as libc::c_int) != 0 {
            f4 &= !(0x1000000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 20 as libc::c_int) != 0 {
            f4 &= !(0x8 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x20000000 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x80000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x40000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x10000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x20000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x100000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x200000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x400000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x800000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x1000000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x2000000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x4000000 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f4 &= !(0x10 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f4 &= !(0x20 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f4 &= !(0x40 as libc::c_int) as libc::c_uint
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f4 &= !(0x80 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x40000000 as libc::c_int as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x80000000 as libc::c_uint)
        }
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x40000000 as libc::c_int) as libc::c_uint
        }
    }
    if smart & 0x80000000 as libc::c_uint != 0 {
        if int_outof(r_ptr, 100 as libc::c_int) != 0 {
            f5 &= !(0x200 as libc::c_int) as libc::c_uint
        }
    }
    /* XXX XXX XXX No spells left? */
	/* if (!f4 && !f5 && !f6) ... */
    *f4p = f4;
    *f5p = f5;
    *f6p = f6;
}
/*
 * Determine if there is a space near the player in which
 * a summoned creature can appear
 */
unsafe extern "C" fn summon_possible(mut y1: libc::c_int, mut x1: libc::c_int)
 -> bool_ {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Start at the player's location, and check 2 grids in each dir */
    y = y1 - 2 as libc::c_int;
    while y <= y1 + 2 as libc::c_int {
        x = x1 - 2 as libc::c_int;
        while x <= x1 + 2 as libc::c_int {
            /* Ignore illegal locations */
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Only check a circular area */
                if !(distance(y1, x1, y, x) > 2 as libc::c_int) {
                    /* Hack: no summon on glyph of warding */
                    if !((*cave[y as usize].offset(x as isize)).feat as
                             libc::c_int == 0x3 as libc::c_int) {
                        if !((*cave[y as usize].offset(x as isize)).feat as
                                 libc::c_int == 0x40 as libc::c_int) {
                            /* Nor on the between */
                            if (*cave[y as usize].offset(x as isize)).feat as
                                   libc::c_int == 0xa0 as libc::c_int {
                                return 0 as libc::c_int as bool_
                            }
                            /* ...nor on the Pattern */
                            if !((*cave[y as usize].offset(x as isize)).feat
                                     as libc::c_int >= 0x41 as libc::c_int &&
                                     (*cave[y as
                                                usize].offset(x as
                                                                  isize)).feat
                                         as libc::c_int <=
                                         0x49 as libc::c_int) {
                                /* Require empty floor grid in line of sight */
                                if (*f_info.offset((*cave[y as
                                                              usize].offset(x
                                                                                as
                                                                                isize)).feat
                                                       as isize)).flags1 as
                                       libc::c_long & 0x10 as libc::c_long !=
                                       0 &&
                                       (*cave[y as
                                                  usize].offset(x as
                                                                    isize)).feat
                                           as libc::c_int !=
                                           0xaf as libc::c_int &&
                                       (*cave[y as
                                                  usize].offset(x as
                                                                    isize)).m_idx
                                           == 0 &&
                                       !(y == (*p_ptr).py as libc::c_int &&
                                             x == (*p_ptr).px as libc::c_int)
                                       &&
                                       los(y1, x1, y, x) as libc::c_int != 0 {
                                    return 1 as libc::c_int as bool_
                                }
                            }
                        }
                    }
                }
            }
            x += 1
        }
        y += 1
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Determine if a bolt spell will hit the player.
 *
 * This is exactly like "projectable", but it will return FALSE if a monster
 * is in the way.
 */
unsafe extern "C" fn clean_shot(mut y1: libc::c_int, mut x1: libc::c_int,
                                mut y2: libc::c_int, mut x2: libc::c_int)
 -> bool_ {
    let mut dist: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Start at the initial location */
    y = y1;
    x = x1;
    /* See "project()" and "projectable()" */
    dist = 0 as libc::c_int;
    while dist <= 18 as libc::c_int {
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
        /* Never pass through monsters */
        if dist != 0 &&
               (*cave[y as usize].offset(x as isize)).m_idx as libc::c_int >
                   0 as libc::c_int {
            if is_friend(&mut *m_list.offset((*(*cave.as_mut_ptr().offset(y as
                                                                              isize)).offset(x
                                                                                                 as
                                                                                                 isize)).m_idx
                                                 as isize)) < 0 as libc::c_int
               {
                break ;
            }
        }
        /* Check for arrival at "final target" */
        if x == x2 && y == y2 { return 1 as libc::c_int as bool_ }
        /* Calculate the new location */
        mmove2(&mut y, &mut x, y1, x1, y2, x2);
        dist += 1
    }
    /* Assume obstruction */
    return 0 as libc::c_int as bool_;
}
/*
 * Cast a bolt at the player
 * Stop if we hit a monster
 * Affect monsters and the player
 */
unsafe extern "C" fn bolt(mut m_idx: libc::c_int, mut typ: libc::c_int,
                          mut dam_hp: libc::c_int) {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    /* Target the player with a bolt attack */
    project(m_idx, 0 as libc::c_int, (*p_ptr).py as libc::c_int,
            (*p_ptr).px as libc::c_int, dam_hp, typ, flg);
}
/*
 * Return TRUE if a spell is good for hurting the player (directly).
 */
unsafe extern "C" fn spell_attack(mut spell: byte_hack) -> bool_ {
    /* All RF4 spells hurt (except for shriek, multiply, summon animal) */
    if spell as libc::c_int >= 96 as libc::c_int + 3 as libc::c_int &&
           spell as libc::c_int <= 96 as libc::c_int + 31 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Various "ball" spells */
    if spell as libc::c_int >= 128 as libc::c_int &&
           spell as libc::c_int <= 128 as libc::c_int + 8 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* "Cause wounds" and "bolt" spells */
    if spell as libc::c_int >= 128 as libc::c_int + 12 as libc::c_int &&
           spell as libc::c_int <= 128 as libc::c_int + 26 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Hand of Doom */
    if spell as libc::c_int == 160 as libc::c_int + 1 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Doesn't hurt */
    return 0 as libc::c_int as bool_;
}
/*
 * Return TRUE if a spell is good for escaping.
 */
unsafe extern "C" fn spell_escape(mut spell: byte_hack) -> bool_ {
    /* Blink or Teleport */
    if spell as libc::c_int == 160 as libc::c_int + 4 as libc::c_int ||
           spell as libc::c_int == 160 as libc::c_int + 5 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Teleport the player away */
    if spell as libc::c_int == 160 as libc::c_int + 7 as libc::c_int ||
           spell as libc::c_int == 160 as libc::c_int + 8 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Isn't good for escaping */
    return 0 as libc::c_int as bool_;
}
/*
 * Return TRUE if a spell is good for annoying the player.
 */
unsafe extern "C" fn spell_annoy(mut spell: byte_hack) -> bool_ {
    /* Shriek */
    if spell as libc::c_int == 96 as libc::c_int + 0 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Brain smash, et al (added curses) */
    if spell as libc::c_int >= 128 as libc::c_int + 9 as libc::c_int &&
           spell as libc::c_int <= 128 as libc::c_int + 14 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Scare, confuse, blind, slow, paralyze */
    if spell as libc::c_int >= 128 as libc::c_int + 27 as libc::c_int &&
           spell as libc::c_int <= 128 as libc::c_int + 31 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Teleport to */
    if spell as libc::c_int == 160 as libc::c_int + 6 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Darkness, make traps, cause amnesia */
    if spell as libc::c_int >= 160 as libc::c_int + 9 as libc::c_int &&
           spell as libc::c_int <= 160 as libc::c_int + 11 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Doesn't annoy */
    return 0 as libc::c_int as bool_;
}
/*
 * Return TRUE if a spell summons help.
 */
unsafe extern "C" fn spell_summon(mut spell: byte_hack) -> bool_ {
    /* RF4_S_ANIMAL, RF6_S_ANIMALS */
    if spell as libc::c_int == 96 as libc::c_int + 2 as libc::c_int ||
           spell as libc::c_int == 160 as libc::c_int + 3 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* All other summon spells */
    if spell as libc::c_int >= 160 as libc::c_int + 13 as libc::c_int &&
           spell as libc::c_int <= 160 as libc::c_int + 31 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Doesn't summon */
    return 0 as libc::c_int as bool_;
}
/*
 * Return TRUE if a spell is good in a tactical situation.
 */
unsafe extern "C" fn spell_tactic(mut spell: byte_hack) -> bool_ {
    /* Blink */
    if spell as libc::c_int == 160 as libc::c_int + 4 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Not good */
    return 0 as libc::c_int as bool_;
}
/*
 * Return TRUE if a spell hastes.
 */
unsafe extern "C" fn spell_haste(mut spell: byte_hack) -> bool_ {
    /* Haste self */
    if spell as libc::c_int == 160 as libc::c_int + 0 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Not a haste spell */
    return 0 as libc::c_int as bool_;
}
/*
 * Return TRUE if a spell is good for healing.
 */
unsafe extern "C" fn spell_heal(mut spell: byte_hack) -> bool_ {
    /* Heal */
    if spell as libc::c_int == 160 as libc::c_int + 2 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* No healing */
    return 0 as libc::c_int as bool_;
}
/*
 * Have a monster choose a spell from a list of "useful" spells.
 *
 * Note that this list does NOT include spells that will just hit
 * other monsters, and the list is restricted when the monster is
 * "desperate".  Should that be the job of this function instead?
 *
 * Stupid monsters will just pick a spell randomly.  Smart monsters
 * will choose more "intelligently".
 *
 * Use the helper functions above to put spells into categories.
 *
 * This function may well be an efficiency bottleneck.
 */
unsafe extern "C" fn choose_attack_spell(mut m_idx: libc::c_int,
                                         mut spells: *mut byte_hack,
                                         mut num: byte_hack) -> libc::c_int {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut escape: [byte_hack; 96] = [0; 96];
    let mut escape_num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut attack: [byte_hack; 96] = [0; 96];
    let mut attack_num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut summon: [byte_hack; 96] = [0; 96];
    let mut summon_num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut tactic: [byte_hack; 96] = [0; 96];
    let mut tactic_num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut annoy: [byte_hack; 96] = [0; 96];
    let mut annoy_num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut haste: [byte_hack; 96] = [0; 96];
    let mut haste_num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut heal: [byte_hack; 96] = [0; 96];
    let mut heal_num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut i: libc::c_int = 0;
    /* Stupid monsters choose randomly */
    if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 {
        /* Pick at random */
        return *spells.offset(Rand_div(num as s32b) as isize) as libc::c_int
    }
    /* Categorize spells */
    i = 0 as libc::c_int;
    while i < num as libc::c_int {
        /* Escape spell? */
        if spell_escape(*spells.offset(i as isize)) != 0 {
            let fresh0 = escape_num;
            escape_num = escape_num.wrapping_add(1);
            escape[fresh0 as usize] = *spells.offset(i as isize)
        }
        /* Attack spell? */
        if spell_attack(*spells.offset(i as isize)) != 0 {
            let fresh1 = attack_num;
            attack_num = attack_num.wrapping_add(1);
            attack[fresh1 as usize] = *spells.offset(i as isize)
        }
        /* Summon spell? */
        if spell_summon(*spells.offset(i as isize)) != 0 {
            let fresh2 = summon_num;
            summon_num = summon_num.wrapping_add(1);
            summon[fresh2 as usize] = *spells.offset(i as isize)
        }
        /* Tactical spell? */
        if spell_tactic(*spells.offset(i as isize)) != 0 {
            let fresh3 = tactic_num;
            tactic_num = tactic_num.wrapping_add(1);
            tactic[fresh3 as usize] = *spells.offset(i as isize)
        }
        /* Annoyance spell? */
        if spell_annoy(*spells.offset(i as isize)) != 0 {
            let fresh4 = annoy_num;
            annoy_num = annoy_num.wrapping_add(1);
            annoy[fresh4 as usize] = *spells.offset(i as isize)
        }
        /* Haste spell? */
        if spell_haste(*spells.offset(i as isize)) != 0 {
            let fresh5 = haste_num;
            haste_num = haste_num.wrapping_add(1);
            haste[fresh5 as usize] = *spells.offset(i as isize)
        }
        /* Heal spell? */
        if spell_heal(*spells.offset(i as isize)) != 0 {
            let fresh6 = heal_num;
            heal_num = heal_num.wrapping_add(1);
            heal[fresh6 as usize] = *spells.offset(i as isize)
        }
        i += 1
    }
    /* ** Try to pick an appropriate spell type ***/
    /* Hurt badly or afraid, attempt to flee */
    if (*m_ptr).hp < (*m_ptr).maxhp / 3 as libc::c_int ||
           (*m_ptr).monfear as libc::c_int != 0 {
        /* Choose escape spell if possible */
        if escape_num != 0 {
            return escape[Rand_div(escape_num as s32b) as usize] as
                       libc::c_int
        }
    }
    /* Still hurt badly, couldn't flee, attempt to heal */
    if (*m_ptr).hp < (*m_ptr).maxhp / 3 as libc::c_int {
        /* Choose heal spell if possible */
        if heal_num != 0 {
            return heal[Rand_div(heal_num as s32b) as usize] as libc::c_int
        }
    }
    /* Player is close and we have attack spells, blink away */
    if distance((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                (*m_ptr).fy as libc::c_int, (*m_ptr).fx as libc::c_int) <
           4 as libc::c_int && attack_num as libc::c_int != 0 &&
           Rand_div(100 as libc::c_int) < 75 as libc::c_int {
        /* Choose tactical spell */
        if tactic_num != 0 {
            return tactic[Rand_div(tactic_num as s32b) as usize] as
                       libc::c_int
        }
    }
    /* We're hurt (not badly), try to heal */
    if (*m_ptr).hp < (*m_ptr).maxhp * 3 as libc::c_int / 4 as libc::c_int &&
           Rand_div(100 as libc::c_int) < 75 as libc::c_int {
        /* Choose heal spell if possible */
        if heal_num != 0 {
            return heal[Rand_div(heal_num as s32b) as usize] as libc::c_int
        }
    }
    /* Summon if possible (sometimes) */
    if summon_num as libc::c_int != 0 &&
           Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        /* Choose summon spell */
        return summon[Rand_div(summon_num as s32b) as usize] as libc::c_int
    }
    /* Attack spell (most of the time) */
    if attack_num as libc::c_int != 0 &&
           Rand_div(100 as libc::c_int) < 85 as libc::c_int {
        /* Choose attack spell */
        return attack[Rand_div(attack_num as s32b) as usize] as libc::c_int
    }
    /* Try another tactical spell (sometimes) */
    if tactic_num as libc::c_int != 0 &&
           Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        /* Choose tactic spell */
        return tactic[Rand_div(tactic_num as s32b) as usize] as libc::c_int
    }
    /* Haste self if we aren't already somewhat hasted (rarely) */
    if haste_num as libc::c_int != 0 &&
           Rand_div(100 as libc::c_int) <
               20 as libc::c_int + (*m_ptr).speed as libc::c_int -
                   (*m_ptr).mspeed as libc::c_int {
        /* Choose haste spell */
        return haste[Rand_div(haste_num as s32b) as usize] as libc::c_int
    }
    /* Annoy player (most of the time) */
    if annoy_num as libc::c_int != 0 &&
           Rand_div(100 as libc::c_int) < 85 as libc::c_int {
        /* Choose annoyance spell */
        return annoy[Rand_div(annoy_num as s32b) as usize] as libc::c_int
    }
    /* Choose no spell */
    return 0 as libc::c_int;
}
/*
 * Cast a breath (or ball) attack at the player
 * Pass over any monsters that may be in the way
 * Affect grids, objects, monsters, and the player
 */
unsafe extern "C" fn breath(mut m_idx: libc::c_int, mut typ: libc::c_int,
                            mut dam_hp: libc::c_int, mut rad: libc::c_int) {
    let mut flg: libc::c_int =
        0x10 as libc::c_int | 0x20 as libc::c_int | 0x40 as libc::c_int;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    /* Determine the radius of the blast */
    if rad < 1 as libc::c_int {
        rad =
            if (*r_ptr).flags2 & 0x1000 as libc::c_int as libc::c_uint != 0 {
                3 as libc::c_int
            } else { 2 as libc::c_int }
    }
    /* Target the player with a ball attack */
    project(m_idx, rad, (*p_ptr).py as libc::c_int,
            (*p_ptr).px as libc::c_int, dam_hp, typ, flg);
}
/*
 * Monster casts a breath (or ball) attack at another monster.
 * Pass over any monsters that may be in the way
 * Affect grids, objects, monsters, and the player
 */
unsafe extern "C" fn monst_breath_monst(mut m_idx: libc::c_int,
                                        mut y: libc::c_int,
                                        mut x: libc::c_int,
                                        mut typ: libc::c_int,
                                        mut dam_hp: libc::c_int,
                                        mut rad: libc::c_int) {
    let mut flg: libc::c_int =
        0x10 as libc::c_int | 0x20 as libc::c_int | 0x40 as libc::c_int;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    /* Determine the radius of the blast */
    if rad < 1 as libc::c_int {
        rad =
            if (*r_ptr).flags2 & 0x1000 as libc::c_int as libc::c_uint != 0 {
                3 as libc::c_int
            } else { 2 as libc::c_int }
    }
    project(m_idx, rad, y, x, dam_hp, typ, flg);
}
/*
 * Monster casts a bolt at another monster
 * Stop if we hit a monster
 * Affect monsters and the player
 */
unsafe extern "C" fn monst_bolt_monst(mut m_idx: libc::c_int,
                                      mut y: libc::c_int, mut x: libc::c_int,
                                      mut typ: libc::c_int,
                                      mut dam_hp: libc::c_int) {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    project(m_idx, 0 as libc::c_int, y, x, dam_hp, typ, flg);
}
#[no_mangle]
pub unsafe extern "C" fn monster_msg(mut fmt: cptr, mut args: ...) {
    let mut vp: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* Format the args, save the length */
    vstrnfmt(buf.as_mut_ptr(), 1024 as libc::c_int as uint_hack, fmt,
             vp.as_va_list());
    /* End the Varargs Stuff */
    /* Display */
    if disturb_other != 0 {
        msg_print(buf.as_mut_ptr() as cptr);
    } else {
        message_add(1 as libc::c_int as byte_hack, buf.as_mut_ptr() as cptr,
                    1 as libc::c_int as byte_hack);
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x40 as libc::c_long) as u32b
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmonster_msg(mut a: libc::c_char, mut fmt: cptr,
                                      mut args: ...) {
    let mut vp: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* Format the args, save the length */
    vstrnfmt(buf.as_mut_ptr(), 1024 as libc::c_int as uint_hack, fmt,
             vp.as_va_list());
    /* End the Varargs Stuff */
    /* Display */
    if disturb_other != 0 {
        cmsg_print(a as byte_hack, buf.as_mut_ptr() as cptr);
    } else {
        message_add(1 as libc::c_int as byte_hack, buf.as_mut_ptr() as cptr,
                    a as byte_hack);
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x40 as libc::c_long) as u32b
    };
}
/*
 * Monster tries to 'cast a spell' (or breath, etc)
 * at another monster.
 */
#[no_mangle]
pub static mut monst_spell_monst_spell: libc::c_int = -(1 as libc::c_int);
unsafe extern "C" fn monst_spell_monst(mut m_idx: libc::c_int) -> bool_ {
    let mut y: libc::c_int = 0 as libc::c_int; /* monster level */
    let mut x: libc::c_int = 0 as libc::c_int; /* Attacker */
    let mut i: libc::c_int = 1 as libc::c_int; /* Putative target */
    let mut k: libc::c_int = 0; /* racial spell flags */
    let mut t_idx: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut thrown_spell: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut spell: [byte_hack; 96] = [0; 96];
    let mut num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut t_name: [libc::c_char; 80] = [0; 80];
    let mut m_poss: [libc::c_char; 80] = [0; 80];
    let mut ddesc: [libc::c_char; 80] = [0; 80];
    let mut rlev: libc::c_int = 0;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut t_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut tr_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut f6: u32b = 0;
    let mut direct: bool_ = 1 as libc::c_int as bool_;
    let mut wake_up: bool_ = 0 as libc::c_int as bool_;
    /* Extract the blind-ness */
    let mut blind: bool_ =
        if (*p_ptr).blind as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    /* Extract the "see-able-ness" */
    let mut seen: bool_ =
        (blind == 0 && (*m_ptr).ml as libc::c_int != 0) as libc::c_int as
            bool_;
    let mut see_m: bool_ = 0;
    let mut see_t: bool_ = 0;
    let mut see_either: bool_ = 0;
    let mut see_both: bool_ = 0;
    let mut friendly: bool_ = 0 as libc::c_int as bool_;
    if is_friend(m_ptr) > 0 as libc::c_int {
        friendly = 1 as libc::c_int as bool_
    }
    /* Cannot cast spells when confused */
    if (*m_ptr).confused != 0 { return 0 as libc::c_int as bool_ }
    /* Hack -- Extract the spell probability */
    chance =
        ((*r_ptr).freq_inate as libc::c_int +
             (*r_ptr).freq_spell as libc::c_int) / 2 as libc::c_int;
    /* Not allowed to cast spells */
    if chance == 0 && monst_spell_monst_spell == -(1 as libc::c_int) {
        return 0 as libc::c_int as bool_
    }
    if Rand_div(100 as libc::c_int) >= chance &&
           monst_spell_monst_spell == -(1 as libc::c_int) {
        return 0 as libc::c_int as bool_
    }
    /* Target location */
    if (*m_ptr).target as libc::c_int > -(1 as libc::c_int) {
        if (*m_ptr).target as libc::c_int > 0 as libc::c_int {
            i = (*m_ptr).target as libc::c_int
        } else { return 0 as libc::c_int as bool_ }
    } else { return 0 as libc::c_int as bool_ }
    t_idx = i;
    t_ptr = &mut *m_list.offset(t_idx as isize) as *mut monster_type;
    tr_ptr =
        if !(*t_ptr).sr_ptr.is_null() {
            (*t_ptr).sr_ptr
        } else {
            race_info_idx((*t_ptr).r_idx as libc::c_int,
                          (*t_ptr).ego as libc::c_int)
        };
    /* Hack -- no fighting >100 squares from player */
    if (*t_ptr).cdis as libc::c_int > 18 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Monster must be projectable */
    if projectable((*m_ptr).fy as libc::c_int, (*m_ptr).fx as libc::c_int,
                   (*t_ptr).fy as libc::c_int, (*t_ptr).fx as libc::c_int) ==
           0 {
        return 0 as libc::c_int as bool_
    }
    /* OK -- we-ve got a target */
    y = (*t_ptr).fy as libc::c_int;
    x = (*t_ptr).fx as libc::c_int;
    /* Extract the monster level */
    rlev =
        if (*m_ptr).level as libc::c_int >= 1 as libc::c_int {
            (*m_ptr).level as libc::c_int
        } else { 1 as libc::c_int };
    /* Extract the racial spell flags */
    f4 = (*r_ptr).flags4;
    f5 = (*r_ptr).flags5;
    f6 = (*r_ptr).flags6;
    /* Hack -- allow "desperate" spells */
    if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint != 0 &&
           (*m_ptr).hp < (*m_ptr).maxhp / 10 as libc::c_int &&
           Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        /* Require intelligent spells */
        f4 &= 0x4 as libc::c_int as libc::c_uint;
        f5 &=
            0x80000000 as libc::c_uint |
                0x40000000 as libc::c_int as libc::c_uint |
                0x20000000 as libc::c_int as libc::c_uint |
                0x10000000 as libc::c_int as libc::c_uint |
                0x8000000 as libc::c_int as libc::c_uint;
        f6 &=
            (0x10 as libc::c_int | 0x20 as libc::c_int | 0x100 as libc::c_int
                 | 0x80 as libc::c_int | 0x4 as libc::c_int |
                 0x1 as libc::c_int | 0x400 as libc::c_int |
                 0x10000 as libc::c_int | 0x20000 as libc::c_int |
                 0x40000 as libc::c_int | 0x80000 as libc::c_int |
                 0x100000 as libc::c_int | 0x200000 as libc::c_int |
                 0x400000 as libc::c_int | 0x800000 as libc::c_int |
                 0x1000000 as libc::c_int | 0x8000000 as libc::c_int |
                 0x4000000 as libc::c_int | 0x2000000 as libc::c_int |
                 0x20000000 as libc::c_int | 0x10000000 as libc::c_int |
                 0x40000000 as libc::c_int) as libc::c_uint |
                0x80000000 as libc::c_uint |
                0x8000 as libc::c_int as libc::c_uint |
                0x2000 as libc::c_int as libc::c_uint |
                0x4000 as libc::c_int as libc::c_uint |
                0x8 as libc::c_int as libc::c_uint;
        /* No spells left */
        if f4 == 0 && f5 == 0 && f6 == 0 &&
               monst_spell_monst_spell == -(1 as libc::c_int) {
            return 0 as libc::c_int as bool_
        }
    }
    /* Extract the "inate" spells */
    k = 0 as libc::c_int;
    while k < 32 as libc::c_int {
        if f4 as libc::c_long & (1 as libc::c_long) << k != 0 {
            let fresh7 = num;
            num = num.wrapping_add(1);
            spell[fresh7 as usize] =
                (k + 32 as libc::c_int * 3 as libc::c_int) as byte_hack
        }
        k += 1
    }
    /* Extract the "normal" spells */
    k = 0 as libc::c_int;
    while k < 32 as libc::c_int {
        if f5 as libc::c_long & (1 as libc::c_long) << k != 0 {
            let fresh8 = num;
            num = num.wrapping_add(1);
            spell[fresh8 as usize] =
                (k + 32 as libc::c_int * 4 as libc::c_int) as byte_hack
        }
        k += 1
    }
    /* Extract the "bizarre" spells */
    k = 0 as libc::c_int;
    while k < 32 as libc::c_int {
        if f6 as libc::c_long & (1 as libc::c_long) << k != 0 {
            let fresh9 = num;
            num = num.wrapping_add(1);
            spell[fresh9 as usize] =
                (k + 32 as libc::c_int * 5 as libc::c_int) as byte_hack
        }
        k += 1
    }
    /* No spells left */
    if num == 0 { return 0 as libc::c_int as bool_ }
    /* Stop if player is dead or gone */
    if alive == 0 || death as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Handle "leaving" */
    if (*p_ptr).leaving != 0 { return 0 as libc::c_int as bool_ }
    /* Get the monster name (or "it") */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    /* Get the monster possessive ("his"/"her"/"its") */
    monster_desc(m_poss.as_mut_ptr(), m_ptr, 0x22 as libc::c_int);
    /* Get the target's name (or "it") */
    monster_desc(t_name.as_mut_ptr(), t_ptr, 0 as libc::c_int);
    /* Hack -- Get the "died from" name */
    monster_desc(ddesc.as_mut_ptr(), m_ptr, 0x88 as libc::c_int);
    /* Choose a spell to cast */
    thrown_spell = spell[Rand_div(num as s32b) as usize] as libc::c_int;
    /* Force a spell ? */
    if monst_spell_monst_spell > -(1 as libc::c_int) {
        thrown_spell = monst_spell_monst_spell;
        monst_spell_monst_spell = -(1 as libc::c_int)
    }
    see_m = seen;
    see_t =
        (blind == 0 && (*t_ptr).ml as libc::c_int != 0) as libc::c_int as
            bool_;
    see_either =
        (see_m as libc::c_int != 0 || see_t as libc::c_int != 0) as
            libc::c_int as bool_;
    see_both =
        (see_m as libc::c_int != 0 && see_t as libc::c_int != 0) as
            libc::c_int as bool_;
    match thrown_spell {
        96 => {
            /* RF4_SHRIEK */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if see_m == 0 {
                    monster_msg(b"You hear a shriek.\x00" as *const u8 as
                                    *const libc::c_char);
                } else {
                    monster_msg(b"%^s shrieks at %s.\x00" as *const u8 as
                                    *const libc::c_char, m_name.as_mut_ptr(),
                                t_name.as_mut_ptr());
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        98 => {
            /* RF4_S_ANIMAL */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons an animal!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 42 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 42 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear something appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        99 => {
            /* RF4_ROCKET */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear an explosion!\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s shoots something.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s fires a rocket at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_breath_monst(m_idx, y, x, 72 as libc::c_int,
                               if (*m_ptr).hp / 4 as libc::c_int >
                                      800 as libc::c_int {
                                   800 as libc::c_int
                               } else { ((*m_ptr).hp) / 4 as libc::c_int },
                               2 as libc::c_int);
        }
        100 => {
            /* RF4_ARROW_1 */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear a strange noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s makes a strange noise.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s fires an arrow at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(10 as libc::c_int);
            monst_bolt_monst(m_idx, y, x, 11 as libc::c_int,
                             damroll(1 as libc::c_int as s16b,
                                     6 as libc::c_int as s16b));
        }
        101 => {
            /* RF4_ARROW_2 */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear a strange noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s makes a strange noise.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s fires an arrow at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(10 as libc::c_int);
            monst_bolt_monst(m_idx, y, x, 11 as libc::c_int,
                             damroll(3 as libc::c_int as s16b,
                                     6 as libc::c_int as s16b));
        }
        102 => {
            /* RF4_ARROW_3 */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear a strange noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s makes a strange noise.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s fires a missile at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(10 as libc::c_int);
            monst_bolt_monst(m_idx, y, x, 11 as libc::c_int,
                             damroll(5 as libc::c_int as s16b,
                                     6 as libc::c_int as s16b));
        }
        103 => {
            /* RF4_ARROW_4 */
            if see_either == 0 {
                monster_msg(b"You hear a strange noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind != 0 {
                monster_msg(b"%^s makes a strange noise.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s fires a missile at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(10 as libc::c_int);
            monst_bolt_monst(m_idx, y, x, 11 as libc::c_int,
                             damroll(7 as libc::c_int as s16b,
                                     6 as libc::c_int as s16b));
        }
        104 => {
            /* RF4_BR_ACID */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes acid at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 3 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      1600 as libc::c_int {
                                   1600 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        105 => {
            /* RF4_BR_ELEC */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes lightning at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 1 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      1600 as libc::c_int {
                                   1600 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        106 => {
            /* RF4_BR_FIRE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes fire at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 5 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      1600 as libc::c_int {
                                   1600 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        107 => {
            /* RF4_BR_COLD */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes frost at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 4 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      1600 as libc::c_int {
                                   1600 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        108 => {
            /* RF4_BR_POIS */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes gas at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 2 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      800 as libc::c_int {
                                   800 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        109 => {
            /* RF4_BR_NETH */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes nether at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 31 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      550 as libc::c_int {
                                   550 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        110 => {
            /* RF4_BR_LITE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes light at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 15 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      400 as libc::c_int {
                                   400 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        111 => {
            /* RF4_BR_DARK */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes darkness at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 16 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      400 as libc::c_int {
                                   400 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        112 => {
            /* RF4_BR_CONF */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes confusion at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 22 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      400 as libc::c_int {
                                   400 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        113 => {
            /* RF4_BR_SOUN */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes sound at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 21 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      400 as libc::c_int {
                                   400 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        114 => {
            /* RF4_BR_CHAO */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes chaos at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 30 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      600 as libc::c_int {
                                   600 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        115 => {
            /* RF4_BR_DISE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes disenchantment at %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 32 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      500 as libc::c_int {
                                   500 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        116 => {
            /* RF4_BR_NEXU */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes nexus at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 33 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      250 as libc::c_int {
                                   250 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        117 => {
            /* RF4_BR_TIME */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes time at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 34 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      150 as libc::c_int {
                                   150 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        118 => {
            /* RF4_BR_INER */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes inertia at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 24 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      200 as libc::c_int {
                                   200 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        119 => {
            /* RF4_BR_GRAV */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes gravity at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 35 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      200 as libc::c_int {
                                   200 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        120 => {
            /* RF4_BR_SHAR */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes shards at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 20 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      400 as libc::c_int {
                                   400 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        121 => {
            /* RF4_BR_PLAS */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes plasma at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 12 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      150 as libc::c_int {
                                   150 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        122 => {
            /* RF4_BR_WALL */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes force at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 23 as libc::c_int,
                               if (*m_ptr).hp / 6 as libc::c_int >
                                      200 as libc::c_int {
                                   200 as libc::c_int
                               } else { ((*m_ptr).hp) / 6 as libc::c_int },
                               0 as libc::c_int);
        }
        123 => {
            /* RF4_BR_MANA */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes magical energy at %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 26 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      250 as libc::c_int {
                                   250 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        124 => {
            /* RF4_BA_NUKE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a ball of radiation at %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 73 as libc::c_int,
                               rlev +
                                   damroll(10 as libc::c_int as s16b,
                                           6 as libc::c_int as s16b),
                               2 as libc::c_int);
        }
        125 => {
            /* RF4_BR_NUKE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes toxic waste at %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 73 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      800 as libc::c_int {
                                   800 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        126 => {
            /* RF4_BA_CHAO */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble frighteningly.\x00" as
                                *const u8 as *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles frighteningly.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s invokes a raw Chaos upon %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 30 as libc::c_int,
                               rlev * 2 as libc::c_int +
                                   damroll(10 as libc::c_int as s16b,
                                           10 as libc::c_int as s16b),
                               4 as libc::c_int);
        }
        127 => {
            /* RF4_BR_DISI -> Breathe Disintegration */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear breathing noise.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s breathes.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s breathes disintegration at %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            sound(39 as libc::c_int);
            monst_breath_monst(m_idx, y, x, 81 as libc::c_int,
                               if (*m_ptr).hp / 3 as libc::c_int >
                                      300 as libc::c_int {
                                   300 as libc::c_int
                               } else { ((*m_ptr).hp) / 3 as libc::c_int },
                               0 as libc::c_int);
        }
        128 => {
            /* RF5_BA_ACID */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts an acid ball at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_breath_monst(m_idx, y, x, 3 as libc::c_int,
                               Rand_div(rlev * 3 as libc::c_int) +
                                   1 as libc::c_int + 15 as libc::c_int,
                               2 as libc::c_int);
        }
        129 => {
            /* RF5_BA_ELEC */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a lightning ball at %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            monst_breath_monst(m_idx, y, x, 1 as libc::c_int,
                               Rand_div(rlev * 3 as libc::c_int /
                                            2 as libc::c_int) +
                                   1 as libc::c_int + 8 as libc::c_int,
                               2 as libc::c_int);
        }
        130 => {
            /* RF5_BA_FIRE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a fire ball at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_breath_monst(m_idx, y, x, 5 as libc::c_int,
                               Rand_div(rlev * 7 as libc::c_int /
                                            2 as libc::c_int) +
                                   1 as libc::c_int + 10 as libc::c_int,
                               2 as libc::c_int);
        }
        131 => {
            /* RF5_BA_COLD */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a frost ball at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_breath_monst(m_idx, y, x, 4 as libc::c_int,
                               Rand_div(rlev * 3 as libc::c_int /
                                            2 as libc::c_int) +
                                   1 as libc::c_int + 10 as libc::c_int,
                               2 as libc::c_int);
        }
        132 => {
            /* RF5_BA_POIS */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a stinking cloud at %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            monst_breath_monst(m_idx, y, x, 2 as libc::c_int,
                               damroll(12 as libc::c_int as s16b,
                                       2 as libc::c_int as s16b),
                               2 as libc::c_int);
        }
        133 => {
            /* RF5_BA_NETH */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a nether ball at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_breath_monst(m_idx, y, x, 31 as libc::c_int,
                               50 as libc::c_int +
                                   damroll(10 as libc::c_int as s16b,
                                           10 as libc::c_int as s16b) + rlev,
                               2 as libc::c_int);
        }
        134 => {
            /* RF5_BA_WATE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble.\x00" as *const u8 as
                                *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s gestures fluidly at %s.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monster_msg(b"%^s is engulfed in a whirlpool.\x00" as *const u8 as
                            *const libc::c_char, t_name.as_mut_ptr());
            monst_breath_monst(m_idx, y, x, 14 as libc::c_int,
                               Rand_div(rlev * 5 as libc::c_int /
                                            2 as libc::c_int) +
                                   1 as libc::c_int + 50 as libc::c_int,
                               4 as libc::c_int);
        }
        135 => {
            /* RF5_BA_MANA */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble powerfully.\x00" as
                                *const u8 as *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles powerfully.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s invokes a mana storm upon %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            monst_breath_monst(m_idx, y, x, 26 as libc::c_int,
                               rlev * 5 as libc::c_int +
                                   damroll(10 as libc::c_int as s16b,
                                           10 as libc::c_int as s16b),
                               4 as libc::c_int);
        }
        136 => {
            /* RF5_BA_DARK */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_either == 0 {
                monster_msg(b"You hear someone mumble powerfully.\x00" as
                                *const u8 as *const libc::c_char);
            } else if blind != 0 {
                monster_msg(b"%^s mumbles powerfully.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s invokes a darkness storm upon %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            monst_breath_monst(m_idx, y, x, 16 as libc::c_int,
                               rlev * 5 as libc::c_int +
                                   damroll(10 as libc::c_int as s16b,
                                           10 as libc::c_int as s16b),
                               4 as libc::c_int);
        }
        137 => {
            /* Attack power */
            let mut r1: libc::c_int =
                (Rand_div(rlev) + 1 as libc::c_int) / 2 as libc::c_int +
                    1 as libc::c_int;
            if see_m != 0 {
                /* Basic message */
                monster_msg(b"%^s draws psychic energy from %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            /* Heal the monster */
            if (*m_ptr).hp < (*m_ptr).maxhp {
                if !((*tr_ptr).flags4 != 0 || (*tr_ptr).flags5 != 0 ||
                         (*tr_ptr).flags6 != 0) {
                    if see_both != 0 {
                        monster_msg(b"%^s is unaffected!\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    /* Heal */
                    (*m_ptr).hp += 6 as libc::c_int * r1;
                    if (*m_ptr).hp > (*m_ptr).maxhp {
                        (*m_ptr).hp = (*m_ptr).maxhp
                    }
                    /* Redraw (later) if needed */
                    if health_who as libc::c_int == m_idx {
                        (*p_ptr).redraw =
                            ((*p_ptr).redraw as libc::c_long |
                                 0x800 as libc::c_long) as u32b
                    }
                    /* Special message */
                    if seen != 0 {
                        monster_msg(b"%^s appears healthier.\x00" as *const u8
                                        as *const libc::c_char,
                                    m_name.as_mut_ptr());
                    }
                }
            }
            wake_up = 1 as libc::c_int as bool_
        }
        138 => {
            /* RF5_MIND_BLAST */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if !(seen == 0) {
                    monster_msg(b"%^s gazes intently at %s.\x00" as *const u8
                                    as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr());
                }
                /* Attempt a saving throw */
                if (*tr_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0
                       ||
                       (*tr_ptr).flags3 &
                           0x40000000 as libc::c_int as libc::c_uint != 0 ||
                       (*t_ptr).level as libc::c_int >
                           Rand_div((if (rlev - 10 as libc::c_int) <
                                            1 as libc::c_int {
                                         1 as libc::c_int
                                     } else { (rlev) - 10 as libc::c_int })) +
                               1 as libc::c_int + 10 as libc::c_int {
                    /* Memorize a flag */
                    if (*tr_ptr).flags3 &
                           0x40000000 as libc::c_int as libc::c_uint != 0 {
                        if seen != 0 {
                            (*tr_ptr).r_flags3 |=
                                0x40000000 as libc::c_int as libc::c_uint
                        }
                    }
                    /* No obvious effect */
                    if see_t != 0 {
                        monster_msg(b"%^s is unaffected!\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    let mut fear: bool_ = 0;
                    monster_msg(b"%^s is blasted by psionic energy.\x00" as
                                    *const u8 as *const libc::c_char,
                                t_name.as_mut_ptr());
                    (*t_ptr).confused =
                        ((*t_ptr).confused as libc::c_int +
                             (Rand_div(4 as libc::c_int) + 4 as libc::c_int))
                            as byte_hack;
                    mon_take_hit_mon(m_idx, t_idx,
                                     damroll(8 as libc::c_int as s16b,
                                             8 as libc::c_int as s16b),
                                     &mut fear,
                                     b" collapses, a mindless husk.\x00" as
                                         *const u8 as *const libc::c_char);
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        139 => {
            /* RF5_BRAIN_SMASH */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if !(seen == 0) {
                    monster_msg(b"%^s gazes intently at %s.\x00" as *const u8
                                    as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr());
                }
                /* Attempt a saving throw */
                if (*tr_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0
                       ||
                       (*tr_ptr).flags3 &
                           0x40000000 as libc::c_int as libc::c_uint != 0 ||
                       (*t_ptr).level as libc::c_int >
                           Rand_div((if (rlev - 10 as libc::c_int) <
                                            1 as libc::c_int {
                                         1 as libc::c_int
                                     } else { (rlev) - 10 as libc::c_int })) +
                               1 as libc::c_int + 10 as libc::c_int {
                    /* Memorize a flag */
                    if (*tr_ptr).flags3 &
                           0x40000000 as libc::c_int as libc::c_uint != 0 {
                        if seen != 0 {
                            (*tr_ptr).r_flags3 |=
                                0x40000000 as libc::c_int as libc::c_uint
                        }
                    }
                    /* No obvious effect */
                    if see_t != 0 {
                        monster_msg(b"%^s is unaffected!\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    let mut fear_0: bool_ = 0;
                    if see_t != 0 {
                        monster_msg(b"%^s is blasted by psionic energy.\x00"
                                        as *const u8 as *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                    (*t_ptr).confused =
                        ((*t_ptr).confused as libc::c_int +
                             (Rand_div(4 as libc::c_int) + 4 as libc::c_int))
                            as byte_hack;
                    (*t_ptr).mspeed =
                        ((*t_ptr).mspeed as libc::c_int -
                             (Rand_div(4 as libc::c_int) + 4 as libc::c_int))
                            as byte_hack;
                    (*t_ptr).stunned =
                        ((*t_ptr).stunned as libc::c_int +
                             (Rand_div(4 as libc::c_int) + 4 as libc::c_int))
                            as byte_hack;
                    mon_take_hit_mon(m_idx, t_idx,
                                     damroll(12 as libc::c_int as s16b,
                                             15 as libc::c_int as s16b),
                                     &mut fear_0,
                                     b" collapses, a mindless husk.\x00" as
                                         *const u8 as *const libc::c_char);
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        140 => {
            /* RF5_CAUSE_1 */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if blind as libc::c_int != 0 || see_m == 0 {
                    monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                    *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    monster_msg(b"%^s points at %s and curses.\x00" as
                                    *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr());
                }
                if (*t_ptr).level as libc::c_int >
                       Rand_div((if (rlev - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (rlev) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                    if see_t != 0 {
                        monster_msg(b"%^s resists!\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    let mut fear_1: bool_ = 0;
                    mon_take_hit_mon(m_idx, t_idx,
                                     damroll(3 as libc::c_int as s16b,
                                             8 as libc::c_int as s16b),
                                     &mut fear_1,
                                     b" is destroyed.\x00" as *const u8 as
                                         *const libc::c_char);
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        141 => {
            /* RF5_CAUSE_2 */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if blind as libc::c_int != 0 || see_m == 0 {
                    monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                    *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    monster_msg(b"%^s points at %s and curses horribly.\x00"
                                    as *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr());
                }
                if (*t_ptr).level as libc::c_int >
                       Rand_div((if (rlev - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (rlev) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                    if see_t != 0 {
                        monster_msg(b"%^s resists!\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    let mut fear_2: bool_ = 0;
                    mon_take_hit_mon(m_idx, t_idx,
                                     damroll(8 as libc::c_int as s16b,
                                             8 as libc::c_int as s16b),
                                     &mut fear_2,
                                     b" is destroyed.\x00" as *const u8 as
                                         *const libc::c_char);
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        142 => {
            /* RF5_CAUSE_3 */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if blind as libc::c_int != 0 || see_m == 0 {
                    monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                    *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    monster_msg(b"%^s points at %s, incanting terribly!\x00"
                                    as *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr());
                }
                if (*t_ptr).level as libc::c_int >
                       Rand_div((if (rlev - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (rlev) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                    if see_t != 0 {
                        monster_msg(b"%^s resists!\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    let mut fear_3: bool_ = 0;
                    mon_take_hit_mon(m_idx, t_idx,
                                     damroll(10 as libc::c_int as s16b,
                                             15 as libc::c_int as s16b),
                                     &mut fear_3,
                                     b" is destroyed.\x00" as *const u8 as
                                         *const libc::c_char);
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        143 => {
            /* RF5_CAUSE_4 */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if blind as libc::c_int != 0 || see_m == 0 {
                    monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                    *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    monster_msg(b"%^s points at %s, screaming the word \'DIE!\'\x00"
                                    as *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr());
                }
                if (*t_ptr).level as libc::c_int >
                       Rand_div((if (rlev - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (rlev) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                    if see_t != 0 {
                        monster_msg(b"%^s resists!\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    let mut fear_4: bool_ = 0;
                    mon_take_hit_mon(m_idx, t_idx,
                                     damroll(15 as libc::c_int as s16b,
                                             15 as libc::c_int as s16b),
                                     &mut fear_4,
                                     b" is destroyed.\x00" as *const u8 as
                                         *const libc::c_char);
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        144 => {
            /* RF5_BO_ACID */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts an acid bolt at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_bolt_monst(m_idx, y, x, 3 as libc::c_int,
                             damroll(7 as libc::c_int as s16b,
                                     8 as libc::c_int as s16b) +
                                 rlev / 3 as libc::c_int);
        }
        145 => {
            /* RF5_BO_ELEC */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a lightning bolt at %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            monst_bolt_monst(m_idx, y, x, 1 as libc::c_int,
                             damroll(4 as libc::c_int as s16b,
                                     8 as libc::c_int as s16b) +
                                 rlev / 3 as libc::c_int);
        }
        146 => {
            /* RF5_BO_FIRE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a fire bolt at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_bolt_monst(m_idx, y, x, 5 as libc::c_int,
                             damroll(9 as libc::c_int as s16b,
                                     8 as libc::c_int as s16b) +
                                 rlev / 3 as libc::c_int);
        }
        147 => {
            /* RF5_BO_COLD */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a frost bolt at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_bolt_monst(m_idx, y, x, 4 as libc::c_int,
                             damroll(6 as libc::c_int as s16b,
                                     8 as libc::c_int as s16b) +
                                 rlev / 3 as libc::c_int);
        }
        148 => { }
        149 => {
            /* RF5_BO_NETH */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a nether bolt at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_bolt_monst(m_idx, y, x, 31 as libc::c_int,
                             30 as libc::c_int +
                                 damroll(5 as libc::c_int as s16b,
                                         5 as libc::c_int as s16b) +
                                 rlev * 3 as libc::c_int / 2 as libc::c_int);
        }
        150 => {
            /* RF5_BO_WATE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a water bolt at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_bolt_monst(m_idx, y, x, 14 as libc::c_int,
                             damroll(10 as libc::c_int as s16b,
                                     10 as libc::c_int as s16b) + rlev);
        }
        151 => {
            /* RF5_BO_MANA */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a mana bolt at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_bolt_monst(m_idx, y, x, 26 as libc::c_int,
                             Rand_div(rlev * 7 as libc::c_int /
                                          2 as libc::c_int) + 1 as libc::c_int
                                 + 50 as libc::c_int);
        }
        152 => {
            /* RF5_BO_PLAS */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a plasma bolt at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_bolt_monst(m_idx, y, x, 12 as libc::c_int,
                             10 as libc::c_int +
                                 damroll(8 as libc::c_int as s16b,
                                         7 as libc::c_int as s16b) + rlev);
        }
        153 => {
            /* RF5_BO_ICEE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts an ice bolt at %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            t_name.as_mut_ptr());
            }
            monst_bolt_monst(m_idx, y, x, 28 as libc::c_int,
                             damroll(6 as libc::c_int as s16b,
                                     6 as libc::c_int as s16b) + rlev);
        }
        154 => {
            /* RF5_MISSILE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s casts a magic missile at %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(), t_name.as_mut_ptr());
            }
            monst_bolt_monst(m_idx, y, x, 10 as libc::c_int,
                             damroll(2 as libc::c_int as s16b,
                                     6 as libc::c_int as s16b) +
                                 rlev / 3 as libc::c_int);
        }
        155 => {
            /* RF5_SCARE */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if blind as libc::c_int != 0 || see_m == 0 {
                    monster_msg(b"%^s mumbles, and you hear scary noises.\x00"
                                    as *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr());
                } else {
                    monster_msg(b"%^s casts a fearful illusion at %s.\x00" as
                                    *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr());
                }
                if (*tr_ptr).flags3 &
                       0x10000000 as libc::c_int as libc::c_uint != 0 {
                    if see_t != 0 {
                        monster_msg(b"%^s refuses to be frightened.\x00" as
                                        *const u8 as *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else if (*t_ptr).level as libc::c_int >
                              Rand_div((if (rlev - 10 as libc::c_int) <
                                               1 as libc::c_int {
                                            1 as libc::c_int
                                        } else {
                                            (rlev) - 10 as libc::c_int
                                        })) + 1 as libc::c_int +
                                  10 as libc::c_int {
                    if see_t != 0 {
                        monster_msg(b"%^s refuses to be frightened.\x00" as
                                        *const u8 as *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    if (*t_ptr).monfear == 0 && see_t as libc::c_int != 0 {
                        monster_msg(b"%^s flees in terror!\x00" as *const u8
                                        as *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                    (*t_ptr).monfear =
                        ((*t_ptr).monfear as libc::c_int +
                             (Rand_div(4 as libc::c_int) + 4 as libc::c_int))
                            as byte_hack
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        156 => {
            /* RF5_BLIND */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if blind as libc::c_int != 0 || see_m == 0 {
                    monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                    *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    monster_msg(b"%^s casts a spell, burning %s%s eyes.\x00"
                                    as *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr(),
                                if strcmp(t_name.as_mut_ptr(),
                                          b"it\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                    b"s\x00" as *const u8 as
                                        *const libc::c_char
                                } else {
                                    b"\'s\x00" as *const u8 as
                                        *const libc::c_char
                                });
                }
                if (*tr_ptr).flags3 &
                       0x40000000 as libc::c_int as libc::c_uint != 0 {
                    /* Simulate blindness with confusion */
                    if see_t != 0 {
                        monster_msg(b"%^s is unaffected.\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else if (*t_ptr).level as libc::c_int >
                              Rand_div((if (rlev - 10 as libc::c_int) <
                                               1 as libc::c_int {
                                            1 as libc::c_int
                                        } else {
                                            (rlev) - 10 as libc::c_int
                                        })) + 1 as libc::c_int +
                                  10 as libc::c_int {
                    if see_t != 0 {
                        monster_msg(b"%^s is unaffected.\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    if see_t != 0 {
                        monster_msg(b"%^s is blinded!\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                    (*t_ptr).confused =
                        ((*t_ptr).confused as libc::c_int +
                             (12 as libc::c_int +
                                  Rand_div(4 as libc::c_int) as byte_hack as
                                      libc::c_int)) as byte_hack
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        157 => {
            /* RF5_CONF */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if blind as libc::c_int != 0 || see_m == 0 {
                    monster_msg(b"%^s mumbles, and you hear puzzling noises.\x00"
                                    as *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr());
                } else {
                    monster_msg(b"%^s creates a mesmerising illusion in front of %s.\x00"
                                    as *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr());
                }
                if (*tr_ptr).flags3 &
                       0x40000000 as libc::c_int as libc::c_uint != 0 {
                    if see_t != 0 {
                        monster_msg(b"%^s disbelieves the feeble spell.\x00"
                                        as *const u8 as *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else if (*t_ptr).level as libc::c_int >
                              Rand_div((if (rlev - 10 as libc::c_int) <
                                               1 as libc::c_int {
                                            1 as libc::c_int
                                        } else {
                                            (rlev) - 10 as libc::c_int
                                        })) + 1 as libc::c_int +
                                  10 as libc::c_int {
                    if see_t != 0 {
                        monster_msg(b"%^s disbelieves the feeble spell.\x00"
                                        as *const u8 as *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    if see_t != 0 {
                        monster_msg(b"%^s seems confused.\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                    (*t_ptr).confused =
                        ((*t_ptr).confused as libc::c_int +
                             (12 as libc::c_int +
                                  Rand_div(4 as libc::c_int) as byte_hack as
                                      libc::c_int)) as byte_hack
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        158 => {
            /* RF5_SLOW */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if blind == 0 && see_either as libc::c_int != 0 {
                    monster_msg(b"%^s drains power from %s%s muscles.\x00" as
                                    *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr(),
                                if strcmp(t_name.as_mut_ptr(),
                                          b"it\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                    b"s\x00" as *const u8 as
                                        *const libc::c_char
                                } else {
                                    b"\'s\x00" as *const u8 as
                                        *const libc::c_char
                                });
                }
                if (*tr_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0
                   {
                    if see_t != 0 {
                        monster_msg(b"%^s is unaffected.\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else if (*t_ptr).level as libc::c_int >
                              Rand_div((if (rlev - 10 as libc::c_int) <
                                               1 as libc::c_int {
                                            1 as libc::c_int
                                        } else {
                                            (rlev) - 10 as libc::c_int
                                        })) + 1 as libc::c_int +
                                  10 as libc::c_int {
                    if see_t != 0 {
                        monster_msg(b"%^s is unaffected.\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    (*t_ptr).mspeed =
                        ((*t_ptr).mspeed as libc::c_int - 10 as libc::c_int)
                            as byte_hack;
                    if see_t != 0 {
                        monster_msg(b"%^s starts moving slower.\x00" as
                                        *const u8 as *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        159 => {
            /* RF5_HOLD */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if blind == 0 && see_m as libc::c_int != 0 {
                    monster_msg(b"%^s stares intently at %s.\x00" as *const u8
                                    as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr());
                }
                if (*tr_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0
                       ||
                       (*tr_ptr).flags3 &
                           0x20000000 as libc::c_int as libc::c_uint != 0 {
                    if see_t != 0 {
                        monster_msg(b"%^s is unaffected.\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else if (*t_ptr).level as libc::c_int >
                              Rand_div((if (rlev - 10 as libc::c_int) <
                                               1 as libc::c_int {
                                            1 as libc::c_int
                                        } else {
                                            (rlev) - 10 as libc::c_int
                                        })) + 1 as libc::c_int +
                                  10 as libc::c_int {
                    if see_t != 0 {
                        monster_msg(b"%^s is unaffected.\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else {
                    (*t_ptr).stunned =
                        ((*t_ptr).stunned as libc::c_int +
                             (Rand_div(4 as libc::c_int) + 1 as libc::c_int +
                                  4 as libc::c_int)) as byte_hack;
                    if see_t != 0 {
                        monster_msg(b"%^s is paralyzed!\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        160 => {
            /* RF6_HASTE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s concentrates on %s body.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            m_poss.as_mut_ptr());
            }
            /* Allow quick speed increases to base+10 */
            if ((*m_ptr).mspeed as libc::c_int) <
                   (*m_ptr).speed as libc::c_int + 10 as libc::c_int {
                if see_m != 0 {
                    monster_msg(b"%^s starts moving faster.\x00" as *const u8
                                    as *const libc::c_char,
                                m_name.as_mut_ptr());
                }
                (*m_ptr).mspeed =
                    ((*m_ptr).mspeed as libc::c_int + 10 as libc::c_int) as
                        byte_hack
            } else if ((*m_ptr).mspeed as libc::c_int) <
                          (*m_ptr).speed as libc::c_int + 20 as libc::c_int {
                if see_m != 0 {
                    monster_msg(b"%^s starts moving faster.\x00" as *const u8
                                    as *const libc::c_char,
                                m_name.as_mut_ptr());
                }
                (*m_ptr).mspeed =
                    ((*m_ptr).mspeed as libc::c_int + 2 as libc::c_int) as
                        byte_hack
            }
        }
        161 => {
            /* Allow small speed increases to base+20 */
            /* RF6_HAND_DOOM */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if see_m == 0 {
                    monster_msg(b"You hear someone invoke the Hand of Doom!\x00"
                                    as *const u8 as *const libc::c_char);
                } else if blind == 0 {
                    monster_msg(b"%^s invokes the Hand of Doom on %s.\x00" as
                                    *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr(), t_name.as_mut_ptr());
                } else {
                    monster_msg(b"You hear someone invoke the Hand of Doom!\x00"
                                    as *const u8 as *const libc::c_char);
                }
                if (*tr_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0
                   {
                    if blind == 0 && see_t as libc::c_int != 0 {
                        monster_msg(b"^%s is unaffected!\x00" as *const u8 as
                                        *const libc::c_char,
                                    t_name.as_mut_ptr());
                    }
                } else if (*m_ptr).level as libc::c_int +
                              (Rand_div(20 as libc::c_int) + 1 as libc::c_int)
                              >
                              (*t_ptr).level as libc::c_int +
                                  10 as libc::c_int +
                                  (Rand_div(20 as libc::c_int) +
                                       1 as libc::c_int) {
                    (*t_ptr).hp =
                        (*t_ptr).hp -
                            (65 as libc::c_int +
                                 (Rand_div(25 as libc::c_int) +
                                      1 as libc::c_int)) * (*t_ptr).hp /
                                100 as libc::c_int;
                    if (*t_ptr).hp < 1 as libc::c_int {
                        (*t_ptr).hp = 1 as libc::c_int
                    }
                } else if see_t != 0 {
                    monster_msg(b"%^s resists!\x00" as *const u8 as
                                    *const libc::c_char, t_name.as_mut_ptr());
                }
                wake_up = 1 as libc::c_int as bool_
            }
        }
        162 => {
            /* RF6_HEAL */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            /* Message */
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s concentrates on %s wounds.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            m_poss.as_mut_ptr());
            }
            /* Heal some */
            (*m_ptr).hp += rlev * 6 as libc::c_int;
            /* Fully healed */
            if (*m_ptr).hp >= (*m_ptr).maxhp {
                /* Fully healed */
                (*m_ptr).hp = (*m_ptr).maxhp;
                /* Message */
                if seen != 0 {
                    monster_msg(b"%^s looks completely healed!\x00" as
                                    *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr());
                } else {
                    monster_msg(b"%^s sounds completely healed!\x00" as
                                    *const u8 as *const libc::c_char,
                                m_name.as_mut_ptr());
                }
            } else if seen != 0 {
                monster_msg(b"%^s looks healthier.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s sounds healthier.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            }
            /* Partially healed */
            /* Message */
            /* Redraw (later) if needed */
            if health_who as libc::c_int == m_idx {
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long | 0x800 as libc::c_long)
                        as u32b
            }
            /* Cancel fear */
            if (*m_ptr).monfear != 0 {
                /* Cancel fear */
                (*m_ptr).monfear = 0 as libc::c_int as byte_hack;
                /* Message */
                if see_m != 0 {
                    monster_msg(b"%^s recovers %s courage.\x00" as *const u8
                                    as *const libc::c_char,
                                m_name.as_mut_ptr(), m_poss.as_mut_ptr());
                }
            }
        }
        163 => {
            /* RF6_S_ANIMALS */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons some animals!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 42 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 42 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many things appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        164 => {
            /* RF6_BLINK */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if see_m != 0 {
                monster_msg(b"%^s blinks away.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            }
            teleport_away(m_idx, 10 as libc::c_int);
        }
        165 => {
            /* RF6_TPORT */
            if !(dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if see_m != 0 {
                    monster_msg(b"%^s teleports away.\x00" as *const u8 as
                                    *const libc::c_char, m_name.as_mut_ptr());
                }
                teleport_away(m_idx,
                              20 as libc::c_int * 2 as libc::c_int +
                                  5 as libc::c_int);
            }
        }
        166 => { }
        167 => {
            /* RF6_TELE_AWAY */
            if !(dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0) {
                if !(direct == 0) {
                    let mut resists_tele: bool_ = 0 as libc::c_int as bool_;
                    if disturb_other != 0 {
                        disturb(1 as libc::c_int, 0 as libc::c_int);
                    }
                    monster_msg(b"%^s teleports %s away.\x00" as *const u8 as
                                    *const libc::c_char, m_name.as_mut_ptr(),
                                t_name.as_mut_ptr());
                    if (*tr_ptr).flags3 &
                           0x200000 as libc::c_int as libc::c_uint != 0 {
                        if (*tr_ptr).flags1 &
                               0x1 as libc::c_int as libc::c_uint != 0 {
                            if see_t != 0 {
                                (*tr_ptr).r_flags3 |=
                                    0x200000 as libc::c_int as libc::c_uint;
                                monster_msg(b"%^s is unaffected!\x00" as
                                                *const u8 as
                                                *const libc::c_char,
                                            t_name.as_mut_ptr());
                            }
                            resists_tele = 1 as libc::c_int as bool_
                        } else if (*t_ptr).level as libc::c_int >
                                      Rand_div(100 as libc::c_int) +
                                          1 as libc::c_int {
                            if see_t != 0 {
                                (*tr_ptr).r_flags3 |=
                                    0x200000 as libc::c_int as libc::c_uint;
                                monster_msg(b"%^s resists!\x00" as *const u8
                                                as *const libc::c_char,
                                            t_name.as_mut_ptr());
                            }
                            resists_tele = 1 as libc::c_int as bool_
                        }
                    }
                    if resists_tele == 0 {
                        teleport_away(t_idx,
                                      20 as libc::c_int * 2 as libc::c_int +
                                          5 as libc::c_int);
                    }
                }
            }
        }
        168 => { }
        169 => {
            /* RF6_DARKNESS */
            if !(direct == 0) {
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                if blind != 0 {
                    monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                    *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    monster_msg(b"%^s gestures in shadow.\x00" as *const u8 as
                                    *const libc::c_char, m_name.as_mut_ptr());
                }
                if seen != 0 {
                    monster_msg(b"%^s is surrounded by darkness.\x00" as
                                    *const u8 as *const libc::c_char,
                                t_name.as_mut_ptr());
                }
                project(m_idx, 3 as libc::c_int, y, x, 0 as libc::c_int,
                        18 as libc::c_int,
                        0x10 as libc::c_int | 0x40 as libc::c_int);
                /* Lite up the room */
                unlite_room(y, x);
            }
        }
        170 => { }
        171 => { }
        173 => {
            /* RF6_S_BUG */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically codes some software bugs.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 51 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 51 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many things appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        174 => {
            /* RF6_S_RNG */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically codes some RNGs.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 52 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 52 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many things appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        175 => {
            /* RF6_S_THUNDERLORD */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons a Thunderlord!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 49 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 49 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear something appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        176 => {
            /* RF6_SUMMON_KIN */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int); /* Big hack */
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons %s %s.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr(),
                            m_poss.as_mut_ptr(),
                            if (*r_ptr).flags1 &
                                   0x1 as libc::c_int as libc::c_uint != 0 {
                                b"minions\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"kin\x00" as *const u8 as *const libc::c_char
                            });
            }
            summon_kin_type = (*r_ptr).d_char;
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 40 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 40 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many things appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        177 => {
            /* RF6_S_HI_DEMON */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons greater demons!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear heavy steps nearby.\x00" as *const u8
                                as *const libc::c_char);
            }
            if friendly != 0 {
                summon_specific_friendly(y, x, rlev, 39 as libc::c_int,
                                         1 as libc::c_int as bool_);
            } else { summon_cyber(); }
        }
        178 => {
            /* RF6_S_MONSTER */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons help!\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 46 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 0 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear something appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        179 => {
            /* RF6_S_MONSTERS */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons monsters!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 46 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 0 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many things appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        180 => {
            /* RF6_S_ANT */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons ants.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 11 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 11 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many things appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        181 => {
            /* RF6_S_SPIDER */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons spiders.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 12 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 12 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many things appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        182 => {
            /* RF6_S_HOUND */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons hounds.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 13 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 13 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many things appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        183 => {
            /* RF6_S_HYDRA */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons hydras.\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 14 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 14 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many things appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        184 => {
            /* RF6_S_ANGEL */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons an angel!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 15 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 15 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear something appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        185 => {
            /* RF6_S_DEMON */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons a demon!\x00" as *const u8
                                as *const libc::c_char, m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 16 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 16 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear something appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        186 => {
            /* RF6_S_UNDEAD */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons an undead adversary!\x00"
                                as *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 17 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 17 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear something appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        187 => {
            /* RF6_S_DRAGON */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons a dragon!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 18 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 18 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear something appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        188 => {
            /* RF6_S_HI_UNDEAD */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons greater undead!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 44 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 21 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many creepy things appear nearby.\x00"
                                as *const u8 as *const libc::c_char);
            }
        }
        189 => {
            /* RF6_S_HI_DRAGON */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons ancient dragons!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 45 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 22 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many powerful things appear nearby.\x00"
                                as *const u8 as *const libc::c_char);
            }
        }
        190 => {
            /* RF6_S_WRAITH */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons a wraith!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                count +=
                    summon_specific(y, x, rlev, 31 as libc::c_int) as
                        libc::c_int;
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear immortal beings appear nearby.\x00" as
                                *const u8 as *const libc::c_char);
            }
        }
        191 => {
            /* RF6_S_UNIQUE */
            if disturb_other != 0 {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
            if blind as libc::c_int != 0 || see_m == 0 {
                monster_msg(b"%^s mumbles.\x00" as *const u8 as
                                *const libc::c_char, m_name.as_mut_ptr());
            } else {
                monster_msg(b"%^s magically summons special opponents!\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr());
            }
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                if friendly == 0 {
                    count +=
                        summon_specific(y, x, rlev, 32 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                if friendly != 0 {
                    count +=
                        summon_specific_friendly(y, x, rlev,
                                                 44 as libc::c_int,
                                                 1 as libc::c_int as bool_) as
                            libc::c_int
                } else {
                    count +=
                        summon_specific(y, x, rlev, 21 as libc::c_int) as
                            libc::c_int
                }
                k += 1
            }
            if blind as libc::c_int != 0 && count != 0 {
                monster_msg(b"You hear many powerful things appear nearby.\x00"
                                as *const u8 as *const libc::c_char);
            }
        }
        97 | 172 | _ => { }
    }
    if wake_up != 0 { (*t_ptr).csleep = 0 as libc::c_int as s16b }
    /* Remember what the monster did, if we saw it */
    if seen != 0 {
        /* Inate spell */
        if thrown_spell < 32 as libc::c_int * 4 as libc::c_int {
            (*r_ptr).r_flags4 =
                ((*r_ptr).r_flags4 as libc::c_long |
                     (1 as libc::c_long) <<
                         thrown_spell - 32 as libc::c_int * 3 as libc::c_int)
                    as u32b;
            if ((*r_ptr).r_cast_inate as libc::c_int) < 255 as libc::c_int {
                (*r_ptr).r_cast_inate = (*r_ptr).r_cast_inate.wrapping_add(1)
            }
        } else if thrown_spell < 32 as libc::c_int * 5 as libc::c_int {
            (*r_ptr).r_flags5 =
                ((*r_ptr).r_flags5 as libc::c_long |
                     (1 as libc::c_long) <<
                         thrown_spell - 32 as libc::c_int * 4 as libc::c_int)
                    as u32b;
            if ((*r_ptr).r_cast_spell as libc::c_int) < 255 as libc::c_int {
                (*r_ptr).r_cast_spell = (*r_ptr).r_cast_spell.wrapping_add(1)
            }
        } else if thrown_spell < 32 as libc::c_int * 6 as libc::c_int {
            (*r_ptr).r_flags6 =
                ((*r_ptr).r_flags6 as libc::c_long |
                     (1 as libc::c_long) <<
                         thrown_spell - 32 as libc::c_int * 5 as libc::c_int)
                    as u32b;
            if ((*r_ptr).r_cast_spell as libc::c_int) < 255 as libc::c_int {
                (*r_ptr).r_cast_spell = (*r_ptr).r_cast_spell.wrapping_add(1)
            }
        }
    }
    /* Bolt or Ball */
    /* Special spell */
    /* Always take note of monsters that kill you ---
		* even accidentally */
    if death as libc::c_int != 0 &&
           ((*r_ptr).r_deaths as libc::c_int) < 32767 as libc::c_int {
        (*r_ptr).r_deaths += 1
    }
    /* A spell was cast */
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn curse_equipment(mut chance: libc::c_int,
                                         mut heavy_chance: libc::c_int) {
    let mut changed: bool_ = 0 as libc::c_int as bool_;
    let mut o1: u32b = 0;
    let mut o2: u32b = 0;
    let mut o3: u32b = 0;
    let mut o4: u32b = 0;
    let mut esp: u32b = 0;
    let mut o5: u32b = 0;
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset((24 as libc::c_int +
                                                          (Rand_div as
                                                               unsafe extern "C" fn(_:
                                                                                        s32b)
                                                                   ->
                                                                       s32b)(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 +
                                                                                 (52
                                                                                      as
                                                                                      libc::c_int
                                                                                      -
                                                                                      1
                                                                                          as
                                                                                          libc::c_int)
                                                                                 -
                                                                                 24
                                                                                     as
                                                                                     libc::c_int))
                                                         as isize) as
            *mut object_type;
    if Rand_div(100 as libc::c_int) + 1 as libc::c_int > chance { return }
    if (*o_ptr).k_idx == 0 { return }
    object_flags(o_ptr, &mut o1, &mut o2, &mut o3, &mut o4, &mut o5,
                 &mut esp);
    /* Extra, biased saving throw for blessed items */
    if o3 as libc::c_long & 0x10000000 as libc::c_long != 0 &&
           Rand_div(888 as libc::c_int) + 1 as libc::c_int > chance {
        let mut o_name: [libc::c_char; 256] = [0; 256];
        object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        msg_format(b"Your %s resist%s cursing!\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                       b"\x00" as *const u8 as *const libc::c_char
                   } else { b"s\x00" as *const u8 as *const libc::c_char });
        /* Hmmm -- can we wear multiple items? If not, this is unnecessary */
        return
    }
    if Rand_div(100 as libc::c_int) + 1 as libc::c_int <= heavy_chance &&
           ((*o_ptr).name1 as libc::c_int != 0 ||
                (*o_ptr).name2 as libc::c_int != 0 ||
                (*o_ptr).art_name as libc::c_int != 0) {
        if o3 as libc::c_long & 0x40000000 as libc::c_long == 0 {
            changed = 1 as libc::c_int as bool_
        }
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x40000000 as libc::c_long)
                as u32b;
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x20000000 as libc::c_long)
                as u32b;
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as byte_hack
    } else {
        if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int == 0 {
            changed = 1 as libc::c_int as bool_
        }
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x20000000 as libc::c_long)
                as u32b;
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as byte_hack
    }
    if changed != 0 {
        msg_print(b"There is a malignant black aura surrounding you...\x00" as
                      *const u8 as *const libc::c_char);
        if (*o_ptr).note != 0 {
            if streq(quark_str((*o_ptr).note as s16b),
                     b"uncursed\x00" as *const u8 as *const libc::c_char) != 0
               {
                (*o_ptr).note = 0 as libc::c_int as u16b
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn curse_equipment_dg(mut chance: libc::c_int,
                                            mut heavy_chance: libc::c_int) {
    let mut changed: bool_ = 0 as libc::c_int as bool_;
    let mut o1: u32b = 0;
    let mut o2: u32b = 0;
    let mut o3: u32b = 0;
    let mut o4: u32b = 0;
    let mut esp: u32b = 0;
    let mut o5: u32b = 0;
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset((24 as libc::c_int +
                                                          (Rand_div as
                                                               unsafe extern "C" fn(_:
                                                                                        s32b)
                                                                   ->
                                                                       s32b)(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 +
                                                                                 (52
                                                                                      as
                                                                                      libc::c_int
                                                                                      -
                                                                                      1
                                                                                          as
                                                                                          libc::c_int)
                                                                                 -
                                                                                 24
                                                                                     as
                                                                                     libc::c_int))
                                                         as isize) as
            *mut object_type;
    if Rand_div(100 as libc::c_int) + 1 as libc::c_int > chance { return }
    if (*o_ptr).k_idx == 0 { return }
    object_flags(o_ptr, &mut o1, &mut o2, &mut o3, &mut o4, &mut o5,
                 &mut esp);
    /* Extra, biased saving throw for blessed items */
    if o3 as libc::c_long & 0x10000000 as libc::c_long != 0 &&
           Rand_div(888 as libc::c_int) + 1 as libc::c_int > chance {
        let mut o_name: [libc::c_char; 256] = [0; 256];
        object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        msg_format(b"Your %s resist%s cursing!\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                       b"\x00" as *const u8 as *const libc::c_char
                   } else { b"s\x00" as *const u8 as *const libc::c_char });
        /* Hmmm -- can we wear multiple items? If not, this is unnecessary */
		/* DG -- Yes we can, in the quiver */
        return
    }
    if Rand_div(100 as libc::c_int) + 1 as libc::c_int <= heavy_chance &&
           ((*o_ptr).name1 as libc::c_int != 0 ||
                (*o_ptr).name2 as libc::c_int != 0 ||
                (*o_ptr).art_name as libc::c_int != 0) {
        if o3 as libc::c_long & 0x40000000 as libc::c_long == 0 {
            changed = 1 as libc::c_int as bool_
        }
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x40000000 as libc::c_long)
                as u32b;
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x20000000 as libc::c_long)
                as u32b;
        (*o_ptr).art_flags4 =
            ((*o_ptr).art_flags4 as libc::c_long | 0x20 as libc::c_long) as
                u32b;
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as byte_hack
    } else {
        if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int == 0 {
            changed = 1 as libc::c_int as bool_
        }
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x20000000 as libc::c_long)
                as u32b;
        (*o_ptr).art_flags4 =
            ((*o_ptr).art_flags4 as libc::c_long | 0x20 as libc::c_long) as
                u32b;
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as byte_hack
    }
    if changed != 0 {
        msg_print(b"There is a malignant black aura surrounding you...\x00" as
                      *const u8 as *const libc::c_char);
        if (*o_ptr).note != 0 {
            if streq(quark_str((*o_ptr).note as s16b),
                     b"uncursed\x00" as *const u8 as *const libc::c_char) != 0
               {
                (*o_ptr).note = 0 as libc::c_int as u16b
            }
        }
    };
}
/*
 * Creatures can cast spells, shoot missiles, and breathe.
 *
 * Returns "TRUE" if a spell (or whatever) was (successfully) cast.
 *
 * XXX XXX XXX This function could use some work, but remember to
 * keep it as optimized as possible, while retaining generic code.
 *
 * Verify the various "blind-ness" checks in the code.
 *
 * XXX XXX XXX Note that several effects should really not be "seen"
 * if the player is blind.  See also "effects.c" for other "mistakes".
 *
 * Perhaps monsters should breathe at locations *near* the player,
 * since this would allow them to inflict "partial" damage.
 *
 * Perhaps smart monsters should decline to use "bolt" spells if
 * there is a monster in the way, unless they wish to kill it.
 *
 * Note that, to allow the use of the "track_target" option at some
 * later time, certain non-optimal things are done in the code below,
 * including explicit checks against the "direct" variable, which is
 * currently always true by the time it is checked, but which should
 * really be set according to an explicit "projectable()" test, and
 * the use of generic "x,y" locations instead of the player location,
 * with those values being initialized with the player location.
 *
 * It will not be possible to "correctly" handle the case in which a
 * monster attempts to attack a location which is thought to contain
 * the player, but which in fact is nowhere near the player, since this
 * might induce all sorts of messages about the attack itself, and about
 * the effects of the attack, which the player might or might not be in
 * a position to observe.  Thus, for simplicity, it is probably best to
 * only allow "faulty" attacks by a monster if one of the important grids
 * (probably the initial or final grid) is in fact in view of the player.
 * It may be necessary to actually prevent spell attacks except when the
 * monster actually has line of sight to the player.  Note that a monster
 * could be left in a bizarre situation after the player ducked behind a
 * pillar and then teleported away, for example.
 *
 * Note that certain spell attacks do not use the "project()" function
 * but "simulate" it via the "direct" variable, which is always at least
 * as restrictive as the "project()" function.  This is necessary to
 * prevent "blindness" attacks and such from bending around walls, etc,
 * and to allow the use of the "track_target" option in the future.
 *
 * Note that this function attempts to optimize the use of spells for the
 * cases in which the monster has no spells, or has spells but cannot use
 * them, or has spells but they will have no "useful" effect.  Note that
 * this function has been an efficiency bottleneck in the past.
 *
 * Note the special "MFLAG_NICE" flag, which prevents a monster from using
 * any spell attacks until the player has had a single chance to move.
 */
#[no_mangle]
pub unsafe extern "C" fn make_attack_spell(mut m_idx: libc::c_int) -> bool_ {
    let mut k: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut thrown_spell: libc::c_int = 0;
    let mut rlev: libc::c_int = 0;
    let mut failrate: libc::c_int = 0;
    let mut spell: [byte_hack; 96] = [0; 96];
    let mut num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut f6: u32b = 0;
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
    let mut no_inate: bool_ = 0 as libc::c_int as bool_;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    /* Summon count */
    let mut count: libc::c_int = 0 as libc::c_int;
    /* Extract the blind-ness */
    let mut blind: bool_ =
        if (*p_ptr).blind as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    /* Extract the "see-able-ness" */
    let mut seen: bool_ =
        (blind == 0 && (*m_ptr).ml as libc::c_int != 0) as libc::c_int as
            bool_;
    /* Assume "normal" target */
    let mut normal: bool_ = 1 as libc::c_int as bool_;
    /* Assume "projectable" */
    let mut direct: bool_ = 1 as libc::c_int as bool_;
    /* Target location */
    if (*m_ptr).target as libc::c_int > -(1 as libc::c_int) {
        if (*m_ptr).target == 0 {
            y = (*p_ptr).py as libc::c_int;
            x = (*p_ptr).px as libc::c_int
        } else { return 0 as libc::c_int as bool_ }
    } else { return 0 as libc::c_int as bool_ }
    /* Cannot cast spells when confused */
    if (*m_ptr).confused != 0 { return 0 as libc::c_int as bool_ }
    /* Cannot cast spells when nice */
    if (*m_ptr).mflag & 0x20 as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    if is_friend(m_ptr) >= 0 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Cannot attack the player if mortal and player fated to never die by the ... */
    if (*r_ptr).flags7 & 0x20 as libc::c_int as libc::c_uint != 0 &&
           (*p_ptr).no_mortal as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Extract the spell probability */
    chance =
        ((*r_ptr).freq_inate as libc::c_int +
             (*r_ptr).freq_spell as libc::c_int) / 2 as libc::c_int;
    /* Not allowed to cast spells */
    if chance == 0 { return 0 as libc::c_int as bool_ }
    if stupid_monsters != 0 {
        /* Only do spells occasionally */
        if Rand_div(100 as libc::c_int) >= chance {
            return 0 as libc::c_int as bool_
        }
    } else {
        if Rand_div(100 as libc::c_int) >= chance {
            return 0 as libc::c_int as bool_
        }
        /* Sometimes forbid inate attacks (breaths) */
        if Rand_div(100 as libc::c_int) >= chance * 2 as libc::c_int {
            no_inate = 1 as libc::c_int as bool_
        }
    }
    /* XXX XXX XXX Handle "track_target" option (?) */
    /* Hack -- require projectable player */
    if normal != 0 {
        /* Check range */
        if (*m_ptr).cdis as libc::c_int > 18 as libc::c_int {
            return 0 as libc::c_int as bool_
        }
        /* Check path */
        if projectable((*m_ptr).fy as libc::c_int, (*m_ptr).fx as libc::c_int,
                       y, x) == 0 {
            return 0 as libc::c_int as bool_
        }
    }
    /* Extract the monster level */
    rlev =
        if (*m_ptr).level as libc::c_int >= 1 as libc::c_int {
            (*m_ptr).level as libc::c_int
        } else { 1 as libc::c_int };
    /* Extract the racial spell flags */
    f4 = (*r_ptr).flags4;
    f5 = (*r_ptr).flags5;
    f6 = (*r_ptr).flags6;
    if stupid_monsters == 0 {
        /* Forbid inate attacks sometimes */
        if no_inate != 0 { f4 = 0 as libc::c_long as u32b }
    }
    /* Hack -- allow "desperate" spells */
    if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint != 0 &&
           (*m_ptr).hp < (*m_ptr).maxhp / 10 as libc::c_int &&
           Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        /* Require intelligent spells */
        f4 &= 0x4 as libc::c_int as libc::c_uint;
        f5 &=
            0x80000000 as libc::c_uint |
                0x40000000 as libc::c_int as libc::c_uint |
                0x20000000 as libc::c_int as libc::c_uint |
                0x10000000 as libc::c_int as libc::c_uint |
                0x8000000 as libc::c_int as libc::c_uint;
        f6 &=
            (0x10 as libc::c_int | 0x20 as libc::c_int | 0x100 as libc::c_int
                 | 0x80 as libc::c_int | 0x4 as libc::c_int |
                 0x1 as libc::c_int | 0x400 as libc::c_int |
                 0x10000 as libc::c_int | 0x20000 as libc::c_int |
                 0x40000 as libc::c_int | 0x80000 as libc::c_int |
                 0x100000 as libc::c_int | 0x200000 as libc::c_int |
                 0x400000 as libc::c_int | 0x800000 as libc::c_int |
                 0x1000000 as libc::c_int | 0x8000000 as libc::c_int |
                 0x4000000 as libc::c_int | 0x2000000 as libc::c_int |
                 0x20000000 as libc::c_int | 0x10000000 as libc::c_int |
                 0x40000000 as libc::c_int) as libc::c_uint |
                0x80000000 as libc::c_uint |
                0x8000 as libc::c_int as libc::c_uint |
                0x2000 as libc::c_int as libc::c_uint |
                0x4000 as libc::c_int as libc::c_uint |
                0x8 as libc::c_int as libc::c_uint;
        /* No spells left */
        if f4 == 0 && f5 == 0 && f6 == 0 { return 0 as libc::c_int as bool_ }
    }
    /* Remove the "ineffective" spells */
    remove_bad_spells(m_idx, &mut f4, &mut f5, &mut f6);
    /* No spells left */
    if f4 == 0 && f5 == 0 && f6 == 0 { return 0 as libc::c_int as bool_ }
    if stupid_monsters == 0 {
        /* Check for a clean bolt shot */
        if (f4 &
                (0x10 as libc::c_int | 0x20 as libc::c_int |
                     0x40 as libc::c_int | 0x80 as libc::c_int) as
                    libc::c_uint != 0 ||
                f5 &
                    (0x10000 as libc::c_int | 0x20000 as libc::c_int |
                         0x40000 as libc::c_int | 0x80000 as libc::c_int |
                         0x100000 as libc::c_int | 0x200000 as libc::c_int |
                         0x400000 as libc::c_int | 0x800000 as libc::c_int |
                         0x1000000 as libc::c_int | 0x2000000 as libc::c_int |
                         0x4000000 as libc::c_int) as libc::c_uint != 0 ||
                f6 as libc::c_long & 0 as libc::c_long != 0) &&
               (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint == 0 &&
               clean_shot((*m_ptr).fy as libc::c_int,
                          (*m_ptr).fx as libc::c_int, y, x) == 0 {
            /* Remove spells that will only hurt friends */
            f4 &=
                !(0x10 as libc::c_int | 0x20 as libc::c_int |
                      0x40 as libc::c_int | 0x80 as libc::c_int) as
                    libc::c_uint;
            f5 &=
                !(0x10000 as libc::c_int | 0x20000 as libc::c_int |
                      0x40000 as libc::c_int | 0x80000 as libc::c_int |
                      0x100000 as libc::c_int | 0x200000 as libc::c_int |
                      0x400000 as libc::c_int | 0x800000 as libc::c_int |
                      0x1000000 as libc::c_int | 0x2000000 as libc::c_int |
                      0x4000000 as libc::c_int) as libc::c_uint;
            f6 = (f6 as libc::c_long & !(0 as libc::c_long)) as u32b
        }
        /* Check for a possible summon */
        if (f4 & 0x4 as libc::c_int as libc::c_uint != 0 ||
                f5 as libc::c_long & 0 as libc::c_long != 0 ||
                f6 &
                    ((0x10000 as libc::c_int | 0x20000 as libc::c_int |
                          0x40000 as libc::c_int | 0x80000 as libc::c_int |
                          0x100000 as libc::c_int | 0x200000 as libc::c_int |
                          0x400000 as libc::c_int | 0x800000 as libc::c_int |
                          0x1000000 as libc::c_int | 0x2000000 as libc::c_int
                          | 0x4000000 as libc::c_int |
                          0x8000000 as libc::c_int | 0x10000000 as libc::c_int
                          | 0x20000000 as libc::c_int |
                          0x40000000 as libc::c_int) as libc::c_uint |
                         0x80000000 as libc::c_uint |
                         0x8000 as libc::c_int as libc::c_uint |
                         0x2000 as libc::c_int as libc::c_uint |
                         0x4000 as libc::c_int as libc::c_uint |
                         0x8 as libc::c_int as libc::c_uint) != 0) &&
               (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint == 0 &&
               summon_possible(y, x) == 0 {
            /* Remove summoning spells */
            f4 &= !(0x4 as libc::c_int) as libc::c_uint;
            f5 = (f5 as libc::c_long & !(0 as libc::c_long)) as u32b;
            f6 &=
                !((0x10000 as libc::c_int | 0x20000 as libc::c_int |
                       0x40000 as libc::c_int | 0x80000 as libc::c_int |
                       0x100000 as libc::c_int | 0x200000 as libc::c_int |
                       0x400000 as libc::c_int | 0x800000 as libc::c_int |
                       0x1000000 as libc::c_int | 0x2000000 as libc::c_int |
                       0x4000000 as libc::c_int | 0x8000000 as libc::c_int |
                       0x10000000 as libc::c_int | 0x20000000 as libc::c_int |
                       0x40000000 as libc::c_int) as libc::c_uint |
                      0x80000000 as libc::c_uint |
                      0x8000 as libc::c_int as libc::c_uint |
                      0x2000 as libc::c_int as libc::c_uint |
                      0x4000 as libc::c_int as libc::c_uint |
                      0x8 as libc::c_int as libc::c_uint)
        }
        /* No spells left */
        if f4 == 0 && f5 == 0 && f6 == 0 { return 0 as libc::c_int as bool_ }
    }
    /* Extract the "inate" spells */
    k = 0 as libc::c_int;
    while k < 32 as libc::c_int {
        if f4 as libc::c_long & (1 as libc::c_long) << k != 0 {
            let fresh10 = num;
            num = num.wrapping_add(1);
            spell[fresh10 as usize] =
                (k + 32 as libc::c_int * 3 as libc::c_int) as byte_hack
        }
        k += 1
    }
    /* Extract the "normal" spells */
    k = 0 as libc::c_int;
    while k < 32 as libc::c_int {
        if f5 as libc::c_long & (1 as libc::c_long) << k != 0 {
            let fresh11 = num;
            num = num.wrapping_add(1);
            spell[fresh11 as usize] =
                (k + 32 as libc::c_int * 4 as libc::c_int) as byte_hack
        }
        k += 1
    }
    /* Extract the "bizarre" spells */
    k = 0 as libc::c_int;
    while k < 32 as libc::c_int {
        if f6 as libc::c_long & (1 as libc::c_long) << k != 0 {
            let fresh12 = num;
            num = num.wrapping_add(1);
            spell[fresh12 as usize] =
                (k + 32 as libc::c_int * 5 as libc::c_int) as byte_hack
        }
        k += 1
    }
    /* No spells left */
    if num == 0 { return 0 as libc::c_int as bool_ }
    /* Stop if player is dead or gone */
    if alive == 0 || death as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Stop if player is leaving */
    if (*p_ptr).leaving != 0 { return 0 as libc::c_int as bool_ }
    /* Get the monster name (or "it") */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    if stupid_monsters != 0 {
        /* Choose a spell to cast */
        thrown_spell = spell[Rand_div(num as s32b) as usize] as libc::c_int
    } else {
        thrown_spell = choose_attack_spell(m_idx, spell.as_mut_ptr(), num);
        /* Abort if no spell was chosen */
        if thrown_spell == 0 { return 0 as libc::c_int as bool_ }
        /* Calculate spell failure rate */
        failrate =
            25 as libc::c_int - (rlev + 3 as libc::c_int) / 4 as libc::c_int;
        /* Hack -- Stupid monsters will never fail (for jellies and such) */
        if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 {
            failrate = 0 as libc::c_int
        }
        /* Check for spell failure (inate attacks never fail) */
        if thrown_spell >= 128 as libc::c_int &&
               Rand_div(100 as libc::c_int) < failrate {
            /* Message */
            msg_format(b"%^s tries to cast a spell, but fails.\x00" as
                           *const u8 as *const libc::c_char,
                       m_name.as_mut_ptr());
            return 1 as libc::c_int as bool_
        }
    }
    /* Can the player disrupt its puny attempts? */
    if (*p_ptr).antimagic_dis as libc::c_int >= (*m_ptr).cdis as libc::c_int
           && Rand_div(100 as libc::c_int) < (*p_ptr).antimagic as libc::c_int
           && thrown_spell >= 128 as libc::c_int {
        let mut m_poss: [libc::c_char; 80] = [0; 80];
        /* Get monster's possessive noun form ("the Illusionist's") */
        monster_desc(m_poss.as_mut_ptr(), m_ptr, 0x6 as libc::c_int);
        msg_format(b"Your anti-magic field disrupts %s spell.\x00" as
                       *const u8 as *const libc::c_char, m_poss.as_mut_ptr());
    } else {
        let mut m_poss_0: [libc::c_char; 80] = [0; 80];
        let mut ddesc: [libc::c_char; 80] = [0; 80];
        /* Get the monster possessive ("his"/"her"/"its") */
        monster_desc(m_poss_0.as_mut_ptr(), m_ptr, 0x22 as libc::c_int);
        /* Hack -- Get the "died from" name */
        monster_desc(ddesc.as_mut_ptr(), m_ptr, 0x88 as libc::c_int);
        /* Cast the spell. */
        match thrown_spell {
            96 => {
                /* RF4_SHRIEK */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    msg_format(b"%^s makes a high pitched shriek.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                    aggravate_monsters(m_idx);
                }
            }
            98 => {
                /* RF4_S_ANIMAL */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons an animal!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 1 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 42 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear something appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            99 => {
                /* RF4_ROCKET */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s shoots something.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s fires a rocket.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 72 as libc::c_int,
                       if (*m_ptr).hp / 4 as libc::c_int > 800 as libc::c_int
                          {
                           800 as libc::c_int
                       } else { ((*m_ptr).hp) / 4 as libc::c_int },
                       2 as libc::c_int);
                update_smart_learn(m_idx, 16 as libc::c_int);
            }
            100 => {
                /* RF4_ARROW_1 */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s makes a strange noise.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s fires an arrow.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 11 as libc::c_int,
                     damroll(1 as libc::c_int as s16b,
                             6 as libc::c_int as s16b));
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            101 => {
                /* RF4_ARROW_2 */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s makes a strange noise.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s fires an arrow!\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 11 as libc::c_int,
                     damroll(3 as libc::c_int as s16b,
                             6 as libc::c_int as s16b));
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            102 => {
                /* RF4_ARROW_3 */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s makes a strange noise.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s fires a missile.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 11 as libc::c_int,
                     damroll(5 as libc::c_int as s16b,
                             6 as libc::c_int as s16b));
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            103 => {
                /* RF4_ARROW_4 */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s makes a strange noise.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s fires a missile!\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 11 as libc::c_int,
                     damroll(7 as libc::c_int as s16b,
                             6 as libc::c_int as s16b));
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            104 => {
                /* RF4_BR_ACID */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes acid.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 3 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 1600 as libc::c_int
                          {
                           1600 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 1 as libc::c_int);
            }
            105 => {
                /* RF4_BR_ELEC */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes lightning.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 1 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 1600 as libc::c_int
                          {
                           1600 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 2 as libc::c_int);
            }
            106 => {
                /* RF4_BR_FIRE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes fire.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 5 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 1600 as libc::c_int
                          {
                           1600 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 3 as libc::c_int);
            }
            107 => {
                /* RF4_BR_COLD */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes frost.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 4 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 1600 as libc::c_int
                          {
                           1600 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 4 as libc::c_int);
            }
            108 => {
                /* RF4_BR_POIS */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes gas.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 2 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 800 as libc::c_int
                          {
                           800 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 5 as libc::c_int);
            }
            109 => {
                /* RF4_BR_NETH */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes nether.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 31 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 550 as libc::c_int
                          {
                           550 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 6 as libc::c_int);
            }
            110 => {
                /* RF4_BR_LITE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes light.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 15 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 400 as libc::c_int
                          {
                           400 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 7 as libc::c_int);
            }
            111 => {
                /* RF4_BR_DARK */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes darkness.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 16 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 400 as libc::c_int
                          {
                           400 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 8 as libc::c_int);
            }
            112 => {
                /* RF4_BR_CONF */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes confusion.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 22 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 400 as libc::c_int
                          {
                           400 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 10 as libc::c_int);
            }
            113 => {
                /* RF4_BR_SOUN */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes sound.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 21 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 400 as libc::c_int
                          {
                           400 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 15 as libc::c_int);
            }
            114 => {
                /* RF4_BR_CHAO */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes chaos.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 30 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 600 as libc::c_int
                          {
                           600 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 11 as libc::c_int);
            }
            115 => {
                /* RF4_BR_DISE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes disenchantment.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                breath(m_idx, 32 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 500 as libc::c_int
                          {
                           500 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 12 as libc::c_int);
            }
            116 => {
                /* RF4_BR_NEXU */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes nexus.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 33 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 250 as libc::c_int
                          {
                           250 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 14 as libc::c_int);
            }
            117 => {
                /* RF4_BR_TIME */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes time.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 34 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 150 as libc::c_int
                          {
                           150 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
            }
            118 => {
                /* RF4_BR_INER */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes inertia.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 24 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 200 as libc::c_int
                          {
                           200 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
            }
            119 => {
                /* RF4_BR_GRAV */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes gravity.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 35 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 200 as libc::c_int
                          {
                           200 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
            }
            120 => {
                /* RF4_BR_SHAR */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes shards.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 20 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 400 as libc::c_int
                          {
                           400 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 16 as libc::c_int);
            }
            121 => {
                /* RF4_BR_PLAS */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes plasma.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 12 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 150 as libc::c_int
                          {
                           150 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
            }
            122 => {
                /* RF4_BR_WALL */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes force.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 23 as libc::c_int,
                       if (*m_ptr).hp / 6 as libc::c_int > 200 as libc::c_int
                          {
                           200 as libc::c_int
                       } else { ((*m_ptr).hp) / 6 as libc::c_int },
                       0 as libc::c_int);
            }
            123 => {
                /* RF4_BR_MANA */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes magical energy.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                breath(m_idx, 26 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 250 as libc::c_int
                          {
                           250 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
            }
            124 => {
                /* RF4_BA_NUKE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a ball of radiation.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                breath(m_idx, 73 as libc::c_int,
                       rlev +
                           damroll(10 as libc::c_int as s16b,
                                   6 as libc::c_int as s16b),
                       2 as libc::c_int);
                update_smart_learn(m_idx, 5 as libc::c_int);
            }
            125 => {
                /* RF4_BR_NUKE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes toxic waste.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                breath(m_idx, 73 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 800 as libc::c_int
                          {
                           800 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
                update_smart_learn(m_idx, 5 as libc::c_int);
            }
            126 => {
                /* RF4_BA_CHAO */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles frighteningly.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s invokes a raw chaos.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 30 as libc::c_int,
                       rlev * 2 as libc::c_int +
                           damroll(10 as libc::c_int as s16b,
                                   10 as libc::c_int as s16b),
                       4 as libc::c_int);
                update_smart_learn(m_idx, 11 as libc::c_int);
            }
            127 => {
                /* RF4_BR_DISI -> Disintegration breath! */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s breathes.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s breathes disintegration.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                breath(m_idx, 81 as libc::c_int,
                       if (*m_ptr).hp / 3 as libc::c_int > 300 as libc::c_int
                          {
                           300 as libc::c_int
                       } else { ((*m_ptr).hp) / 3 as libc::c_int },
                       0 as libc::c_int);
            }
            128 => {
                /* RF5_BA_ACID */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts an acid ball.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 3 as libc::c_int,
                       Rand_div(rlev * 3 as libc::c_int) + 1 as libc::c_int +
                           15 as libc::c_int, 2 as libc::c_int);
                update_smart_learn(m_idx, 1 as libc::c_int);
            }
            129 => {
                /* RF5_BA_ELEC */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a lightning ball.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                breath(m_idx, 1 as libc::c_int,
                       Rand_div(rlev * 3 as libc::c_int / 2 as libc::c_int) +
                           1 as libc::c_int + 8 as libc::c_int,
                       2 as libc::c_int);
                update_smart_learn(m_idx, 2 as libc::c_int);
            }
            130 => {
                /* RF5_BA_FIRE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a fire ball.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 5 as libc::c_int,
                       Rand_div(rlev * 7 as libc::c_int / 2 as libc::c_int) +
                           1 as libc::c_int + 10 as libc::c_int,
                       2 as libc::c_int);
                update_smart_learn(m_idx, 3 as libc::c_int);
            }
            131 => {
                /* RF5_BA_COLD */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a frost ball.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 4 as libc::c_int,
                       Rand_div(rlev * 3 as libc::c_int / 2 as libc::c_int) +
                           1 as libc::c_int + 10 as libc::c_int,
                       2 as libc::c_int);
                update_smart_learn(m_idx, 4 as libc::c_int);
            }
            132 => {
                /* RF5_BA_POIS */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a stinking cloud.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                breath(m_idx, 2 as libc::c_int,
                       damroll(12 as libc::c_int as s16b,
                               2 as libc::c_int as s16b), 2 as libc::c_int);
                update_smart_learn(m_idx, 5 as libc::c_int);
            }
            133 => {
                /* RF5_BA_NETH */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a nether ball.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                breath(m_idx, 31 as libc::c_int,
                       50 as libc::c_int +
                           damroll(10 as libc::c_int as s16b,
                                   10 as libc::c_int as s16b) + rlev,
                       2 as libc::c_int);
                update_smart_learn(m_idx, 6 as libc::c_int);
            }
            134 => {
                /* RF5_BA_WATE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s gestures fluidly.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                msg_print(b"You are engulfed in a whirlpool.\x00" as *const u8
                              as *const libc::c_char);
                breath(m_idx, 14 as libc::c_int,
                       Rand_div(rlev * 5 as libc::c_int / 2 as libc::c_int) +
                           1 as libc::c_int + 50 as libc::c_int,
                       4 as libc::c_int);
            }
            135 => {
                /* RF5_BA_MANA */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles powerfully.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s invokes a mana storm.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                breath(m_idx, 26 as libc::c_int,
                       rlev * 5 as libc::c_int +
                           damroll(10 as libc::c_int as s16b,
                                   10 as libc::c_int as s16b),
                       4 as libc::c_int);
            }
            136 => {
                /* RF5_BA_DARK */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles powerfully.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s invokes a darkness storm.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                breath(m_idx, 16 as libc::c_int,
                       rlev * 5 as libc::c_int +
                           damroll(10 as libc::c_int as s16b,
                                   10 as libc::c_int as s16b),
                       4 as libc::c_int);
                update_smart_learn(m_idx, 8 as libc::c_int);
            }
            137 => {
                /* RF5_DRAIN_MANA */
                if !(direct == 0) {
                    if (*p_ptr).csp != 0 {
                        let mut r1: libc::c_int = 0;
                        /* Disturb if legal */
                        disturb(1 as libc::c_int, 0 as libc::c_int);
                        /* Basic message */
                        msg_format(b"%^s draws psychic energy from you!\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                        /* Attack power */
                        r1 =
                            (Rand_div(rlev) + 1 as libc::c_int) /
                                2 as libc::c_int + 1 as libc::c_int;
                        /* Full drain */
                        if r1 >= (*p_ptr).csp as libc::c_int {
                            r1 = (*p_ptr).csp as libc::c_int;
                            (*p_ptr).csp = 0 as libc::c_int as s16b;
                            (*p_ptr).csp_frac = 0 as libc::c_int as u16b
                        } else {
                            /* Partial drain */
                            (*p_ptr).csp =
                                ((*p_ptr).csp as libc::c_int - r1) as s16b
                        }
                        /* Redraw mana */
                        (*p_ptr).redraw =
                            ((*p_ptr).redraw as libc::c_long |
                                 0x80 as libc::c_long) as u32b;
                        /* Window stuff */
                        (*p_ptr).window =
                            ((*p_ptr).window as libc::c_long |
                                 0x8 as libc::c_long) as u32b;
                        /* Heal the monster */
                        if (*m_ptr).hp < (*m_ptr).maxhp {
                            /* Heal */
                            (*m_ptr).hp += 6 as libc::c_int * r1;
                            if (*m_ptr).hp > (*m_ptr).maxhp {
                                (*m_ptr).hp = (*m_ptr).maxhp
                            }
                            /* Redraw (later) if needed */
                            if health_who as libc::c_int == m_idx {
                                (*p_ptr).redraw =
                                    ((*p_ptr).redraw as libc::c_long |
                                         0x800 as libc::c_long) as u32b
                            }
                            /* Special message */
                            if seen != 0 {
                                msg_format(b"%^s appears healthier.\x00" as
                                               *const u8 as
                                               *const libc::c_char,
                                           m_name.as_mut_ptr());
                            }
                        }
                    }
                    update_smart_learn(m_idx, 31 as libc::c_int);
                }
            }
            138 => {
                /* RF5_MIND_BLAST */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if seen == 0 {
                        msg_print(b"You feel something focusing on your mind.\x00"
                                      as *const u8 as *const libc::c_char);
                    } else {
                        msg_format(b"%^s gazes deep into your eyes.\x00" as
                                       *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if Rand_div(100 as libc::c_int) <
                           (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        msg_print(b"Your mind is blasted by psionic energy.\x00"
                                      as *const u8 as *const libc::c_char);
                        if (*p_ptr).resist_conf == 0 {
                            set_confused((*p_ptr).confused as libc::c_int +
                                             Rand_div(4 as libc::c_int) +
                                             4 as libc::c_int);
                        }
                        if (*p_ptr).resist_chaos == 0 &&
                               Rand_div(3 as libc::c_int) + 1 as libc::c_int
                                   == 1 as libc::c_int {
                            set_image((*p_ptr).image as libc::c_int +
                                          Rand_div(250 as libc::c_int) +
                                          150 as libc::c_int);
                        }
                        take_sanity_hit(damroll(8 as libc::c_int as s16b,
                                                8 as libc::c_int as s16b),
                                        ddesc.as_mut_ptr() as cptr);
                    }
                }
            }
            139 => {
                /* RF5_BRAIN_SMASH */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if seen == 0 {
                        msg_print(b"You feel something focusing on your mind.\x00"
                                      as *const u8 as *const libc::c_char);
                    } else {
                        msg_format(b"%^s looks deep into your eyes.\x00" as
                                       *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if Rand_div(100 as libc::c_int) <
                           (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        msg_print(b"Your mind is blasted by psionic energy.\x00"
                                      as *const u8 as *const libc::c_char);
                        take_sanity_hit(damroll(12 as libc::c_int as s16b,
                                                15 as libc::c_int as s16b),
                                        ddesc.as_mut_ptr() as cptr);
                        if (*p_ptr).resist_blind == 0 {
                            set_blind((*p_ptr).blind as libc::c_int +
                                          8 as libc::c_int +
                                          Rand_div(8 as libc::c_int));
                        }
                        if (*p_ptr).resist_conf == 0 {
                            set_confused((*p_ptr).confused as libc::c_int +
                                             Rand_div(4 as libc::c_int) +
                                             4 as libc::c_int);
                        }
                        if (*p_ptr).free_act == 0 {
                            set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                              Rand_div(4 as libc::c_int) +
                                              4 as libc::c_int);
                        }
                        set_slow((*p_ptr).slow as libc::c_int +
                                     Rand_div(4 as libc::c_int) +
                                     4 as libc::c_int);
                        while Rand_div(100 as libc::c_int) >
                                  (*p_ptr).skill_sav as libc::c_int {
                            do_dec_stat(1 as libc::c_int, 2 as libc::c_int);
                        }
                        while Rand_div(100 as libc::c_int) >
                                  (*p_ptr).skill_sav as libc::c_int {
                            do_dec_stat(2 as libc::c_int, 2 as libc::c_int);
                        }
                        if (*p_ptr).resist_chaos == 0 {
                            set_image((*p_ptr).image as libc::c_int +
                                          Rand_div(250 as libc::c_int) +
                                          150 as libc::c_int);
                        }
                    }
                }
            }
            140 => {
                /* RF5_CAUSE_1 */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s mumbles.\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s points at you and curses.\x00" as
                                       *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if Rand_div(100 as libc::c_int) <
                           (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        curse_equipment(33 as libc::c_int, 0 as libc::c_int);
                        take_hit(damroll(3 as libc::c_int as s16b,
                                         8 as libc::c_int as s16b),
                                 ddesc.as_mut_ptr() as cptr);
                    }
                }
            }
            141 => {
                /* RF5_CAUSE_2 */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s mumbles.\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s points at you and curses horribly.\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if Rand_div(100 as libc::c_int) <
                           (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        curse_equipment(50 as libc::c_int, 5 as libc::c_int);
                        take_hit(damroll(8 as libc::c_int as s16b,
                                         8 as libc::c_int as s16b),
                                 ddesc.as_mut_ptr() as cptr);
                    }
                }
            }
            142 => {
                /* RF5_CAUSE_3 */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s mumbles loudly.\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s points at you, incanting terribly!\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if Rand_div(100 as libc::c_int) <
                           (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        curse_equipment(80 as libc::c_int, 15 as libc::c_int);
                        take_hit(damroll(10 as libc::c_int as s16b,
                                         15 as libc::c_int as s16b),
                                 ddesc.as_mut_ptr() as cptr);
                    }
                }
            }
            143 => {
                /* RF5_CAUSE_4 */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s screams the word \'DIE!\'\x00" as
                                       *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s points at you, screaming the word DIE!\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if Rand_div(100 as libc::c_int) <
                           (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        take_hit(damroll(15 as libc::c_int as s16b,
                                         15 as libc::c_int as s16b),
                                 ddesc.as_mut_ptr() as cptr);
                        set_cut((*p_ptr).cut as libc::c_int +
                                    damroll(10 as libc::c_int as s16b,
                                            10 as libc::c_int as s16b));
                    }
                }
            }
            144 => {
                /* RF5_BO_ACID */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a acid bolt.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 3 as libc::c_int,
                     damroll(7 as libc::c_int as s16b,
                             8 as libc::c_int as s16b) +
                         rlev / 3 as libc::c_int);
                update_smart_learn(m_idx, 1 as libc::c_int);
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            145 => {
                /* RF5_BO_ELEC */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a lightning bolt.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                bolt(m_idx, 1 as libc::c_int,
                     damroll(4 as libc::c_int as s16b,
                             8 as libc::c_int as s16b) +
                         rlev / 3 as libc::c_int);
                update_smart_learn(m_idx, 2 as libc::c_int);
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            146 => {
                /* RF5_BO_FIRE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a fire bolt.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 5 as libc::c_int,
                     damroll(9 as libc::c_int as s16b,
                             8 as libc::c_int as s16b) +
                         rlev / 3 as libc::c_int);
                update_smart_learn(m_idx, 3 as libc::c_int);
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            147 => {
                /* RF5_BO_COLD */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a frost bolt.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 4 as libc::c_int,
                     damroll(6 as libc::c_int as s16b,
                             8 as libc::c_int as s16b) +
                         rlev / 3 as libc::c_int);
                update_smart_learn(m_idx, 4 as libc::c_int);
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            149 => {
                /* RF5_BO_NETH */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a nether bolt.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 31 as libc::c_int,
                     30 as libc::c_int +
                         damroll(5 as libc::c_int as s16b,
                                 5 as libc::c_int as s16b) +
                         rlev * 3 as libc::c_int / 2 as libc::c_int);
                update_smart_learn(m_idx, 6 as libc::c_int);
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            150 => {
                /* RF5_BO_WATE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a water bolt.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 14 as libc::c_int,
                     damroll(10 as libc::c_int as s16b,
                             10 as libc::c_int as s16b) + rlev);
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            151 => {
                /* RF5_BO_MANA */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a mana bolt.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 26 as libc::c_int,
                     Rand_div(rlev * 7 as libc::c_int / 2 as libc::c_int) +
                         1 as libc::c_int + 50 as libc::c_int);
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            152 => {
                /* RF5_BO_PLAS */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a plasma bolt.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 12 as libc::c_int,
                     10 as libc::c_int +
                         damroll(8 as libc::c_int as s16b,
                                 7 as libc::c_int as s16b) + rlev);
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            153 => {
                /* RF5_BO_ICEE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts an ice bolt.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                bolt(m_idx, 28 as libc::c_int,
                     damroll(6 as libc::c_int as s16b,
                             6 as libc::c_int as s16b) + rlev);
                update_smart_learn(m_idx, 4 as libc::c_int);
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            154 => {
                /* RF5_MISSILE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s casts a magic missile.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                bolt(m_idx, 10 as libc::c_int,
                     damroll(2 as libc::c_int as s16b,
                             6 as libc::c_int as s16b) +
                         rlev / 3 as libc::c_int);
                update_smart_learn(m_idx, 32 as libc::c_int);
            }
            155 => {
                /* RF5_SCARE */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s mumbles, and you hear scary noises.\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s casts a fearful illusion.\x00" as
                                       *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if (*p_ptr).resist_fear != 0 {
                        msg_print(b"You refuse to be frightened.\x00" as
                                      *const u8 as *const libc::c_char);
                    } else if Rand_div(100 as libc::c_int) <
                                  (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You refuse to be frightened.\x00" as
                                      *const u8 as *const libc::c_char);
                    } else {
                        set_afraid((*p_ptr).afraid as libc::c_int +
                                       Rand_div(4 as libc::c_int) +
                                       4 as libc::c_int);
                    }
                    update_smart_learn(m_idx, 9 as libc::c_int);
                }
            }
            156 => {
                /* RF5_BLIND */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s mumbles.\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s casts a spell, burning your eyes!\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if (*p_ptr).resist_blind != 0 {
                        msg_print(b"You are unaffected!\x00" as *const u8 as
                                      *const libc::c_char);
                    } else if Rand_div(100 as libc::c_int) <
                                  (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        set_blind(12 as libc::c_int +
                                      Rand_div(4 as libc::c_int));
                    }
                    update_smart_learn(m_idx, 13 as libc::c_int);
                }
            }
            157 => {
                /* RF5_CONF */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s mumbles, and you hear puzzling noises.\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s creates a mesmerizing illusion.\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if (*p_ptr).resist_conf != 0 {
                        msg_print(b"You disbelieve the feeble spell.\x00" as
                                      *const u8 as *const libc::c_char);
                    } else if Rand_div(100 as libc::c_int) <
                                  (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You disbelieve the feeble spell.\x00" as
                                      *const u8 as *const libc::c_char);
                    } else {
                        set_confused((*p_ptr).confused as libc::c_int +
                                         Rand_div(4 as libc::c_int) +
                                         4 as libc::c_int);
                    }
                    update_smart_learn(m_idx, 10 as libc::c_int);
                }
            }
            158 => {
                /* RF5_SLOW */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    msg_format(b"%^s drains power from your muscles!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                    if (*p_ptr).free_act != 0 {
                        msg_print(b"You are unaffected!\x00" as *const u8 as
                                      *const libc::c_char);
                    } else if Rand_div(100 as libc::c_int) <
                                  (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        set_slow((*p_ptr).slow as libc::c_int +
                                     Rand_div(4 as libc::c_int) +
                                     4 as libc::c_int);
                    }
                    update_smart_learn(m_idx, 30 as libc::c_int);
                }
            }
            159 => {
                /* RF5_HOLD */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s mumbles.\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s stares deep into your eyes!\x00" as
                                       *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if (*p_ptr).free_act != 0 {
                        msg_print(b"You are unaffected!\x00" as *const u8 as
                                      *const libc::c_char);
                    } else if Rand_div(100 as libc::c_int) <
                                  (*p_ptr).skill_sav as libc::c_int {
                        msg_format(b"You resist the effects!\x00" as *const u8
                                       as *const libc::c_char);
                    } else {
                        set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                          Rand_div(4 as libc::c_int) +
                                          4 as libc::c_int);
                    }
                    update_smart_learn(m_idx, 30 as libc::c_int);
                }
            }
            160 => {
                /* RF6_HASTE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s concentrates on %s body.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr(), m_poss_0.as_mut_ptr());
                }
                /* Allow quick speed increases to base+10 */
                if ((*m_ptr).mspeed as libc::c_int) <
                       (*m_ptr).speed as libc::c_int + 10 as libc::c_int {
                    msg_format(b"%^s starts moving faster.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                    (*m_ptr).mspeed =
                        ((*m_ptr).mspeed as libc::c_int + 10 as libc::c_int)
                            as byte_hack
                } else if ((*m_ptr).mspeed as libc::c_int) <
                              (*m_ptr).speed as libc::c_int +
                                  20 as libc::c_int {
                    msg_format(b"%^s starts moving faster.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                    (*m_ptr).mspeed =
                        ((*m_ptr).mspeed as libc::c_int + 2 as libc::c_int) as
                            byte_hack
                }
            }
            161 => {
                /* Allow small speed increases to base+20 */
                /* RF6_HAND_DOOM */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                msg_format(b"%^s invokes the Hand of Doom!\x00" as *const u8
                               as *const libc::c_char, m_name.as_mut_ptr());
                if Rand_div(100 as libc::c_int) <
                       (*p_ptr).skill_sav as libc::c_int {
                    msg_format(b"You resist the effects!\x00" as *const u8 as
                                   *const libc::c_char);
                } else {
                    let mut dummy: libc::c_int =
                        (65 as libc::c_int +
                             (Rand_div(25 as libc::c_int) + 1 as libc::c_int))
                            * (*p_ptr).chp as libc::c_int /
                            100 as libc::c_int;
                    msg_print(b"Your feel your life fade away!\x00" as
                                  *const u8 as *const libc::c_char);
                    take_hit(dummy, m_name.as_mut_ptr() as cptr);
                    curse_equipment(100 as libc::c_int, 20 as libc::c_int);
                    if ((*p_ptr).chp as libc::c_int) < 1 as libc::c_int {
                        (*p_ptr).chp = 1 as libc::c_int as s16b
                    }
                }
            }
            162 => {
                /* RF6_HEAL */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                /* Message */
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s concentrates on %s wounds.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr(), m_poss_0.as_mut_ptr());
                }
                /* Heal some */
                (*m_ptr).hp += rlev * 6 as libc::c_int;
                /* Fully healed */
                if (*m_ptr).hp >= (*m_ptr).maxhp {
                    /* Fully healed */
                    (*m_ptr).hp = (*m_ptr).maxhp;
                    /* Message */
                    if seen != 0 {
                        msg_format(b"%^s looks completely healed!\x00" as
                                       *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s sounds completely healed!\x00" as
                                       *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                } else if seen != 0 {
                    msg_format(b"%^s looks healthier.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s sounds healthier.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
                /* Partially healed */
                /* Message */
                /* Redraw (later) if needed */
                if health_who as libc::c_int == m_idx {
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x800 as libc::c_long) as u32b
                }
                /* Cancel fear */
                if (*m_ptr).monfear != 0 {
                    /* Cancel fear */
                    (*m_ptr).monfear = 0 as libc::c_int as byte_hack;
                    /* Message */
                    msg_format(b"%^s recovers %s courage.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr(),
                               m_poss_0.as_mut_ptr());
                }
            }
            163 => {
                /* RF6_S_ANIMALS */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons some animals!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 4 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 42 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear something appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            164 => {
                /* RF6_BLINK */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                msg_format(b"%^s blinks away.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
                teleport_away(m_idx, 10 as libc::c_int);
            }
            165 => {
                /* RF6_TPORT */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                msg_format(b"%^s teleports away.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
                teleport_away(m_idx,
                              20 as libc::c_int * 2 as libc::c_int +
                                  5 as libc::c_int);
            }
            166 => {
                /* RF6_TELE_TO */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    msg_format(b"%^s commands you to return.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                    teleport_player_to((*m_ptr).fy as libc::c_int,
                                       (*m_ptr).fx as libc::c_int);
                }
            }
            167 => {
                /* RF6_TELE_AWAY */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    msg_format(b"%^s teleports you away.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                    teleport_player(100 as libc::c_int);
                }
            }
            168 => {
                /* RF6_TELE_LEVEL */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s mumbles strangely.\x00" as *const u8
                                       as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s gestures at your feet.\x00" as
                                       *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    if (*p_ptr).resist_nexus != 0 {
                        msg_print(b"You are unaffected!\x00" as *const u8 as
                                      *const libc::c_char);
                    } else if Rand_div(100 as libc::c_int) <
                                  (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else { teleport_player_level(); }
                    update_smart_learn(m_idx, 14 as libc::c_int);
                }
            }
            169 => {
                /* RF6_DARKNESS */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s mumbles.\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s gestures in shadow.\x00" as *const u8
                                       as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    unlite_area(0 as libc::c_int, 3 as libc::c_int);
                }
            }
            170 => {
                /* RF6_TRAPS */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    if blind != 0 {
                        msg_format(b"%^s mumbles, and then cackles evilly.\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    } else {
                        msg_format(b"%^s casts a spell and cackles evilly.\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    trap_creation();
                }
            }
            171 => {
                /* RF6_FORGET */
                if !(direct == 0) {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    msg_format(b"%^s tries to blank your mind.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                    if Rand_div(100 as libc::c_int) <
                           (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else if lose_all_info() != 0 {
                        msg_print(b"Your memories fade away.\x00" as *const u8
                                      as *const libc::c_char);
                    }
                }
            }
            173 => {
                /* RF6_S_BUG */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically codes some software bugs.\x00"
                                   as *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 6 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 51 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many things appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            174 => {
                /* RF6_S_RNG */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically codes some RNGs.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 6 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 52 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many things appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            175 => {
                /* RF6_S_THUNDERLORD */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons a Thunderlord!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 1 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 49 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear something appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            176 => {
                /* RF6_SUMMON_KIN */
                disturb(1 as libc::c_int, 0 as libc::c_int); /* Big hack */
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons %s %s.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr(), m_poss_0.as_mut_ptr(),
                               if (*r_ptr).flags1 &
                                      0x1 as libc::c_int as libc::c_uint != 0
                                  {
                                   b"minions\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"kin\x00" as *const u8 as
                                       *const libc::c_char
                               });
                }
                summon_kin_type = (*r_ptr).d_char;
                k = 0 as libc::c_int;
                while k < 6 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 40 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many things appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            177 => {
                /* RF6_S_HI_DEMON */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons greater demons!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear heavy steps nearby.\x00" as *const u8
                                  as *const libc::c_char);
                }
                summon_cyber();
            }
            178 => {
                /* RF6_S_MONSTER */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons help!\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 1 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 0 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear something appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            179 => {
                /* RF6_S_MONSTERS */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons monsters!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 8 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 0 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many things appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            180 => {
                /* RF6_S_ANT */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons ants.\x00" as *const u8
                                   as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 6 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 11 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many things appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            181 => {
                /* RF6_S_SPIDER */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons spiders.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 6 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 12 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many things appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            182 => {
                /* RF6_S_HOUND */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons hounds.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 6 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 13 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many things appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            183 => {
                /* RF6_S_HYDRA */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons hydras.\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 6 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 14 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many things appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            184 => {
                /* RF6_S_ANGEL */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons an angel!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 1 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 15 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear something appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            185 => {
                /* RF6_S_DEMON */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons a demon!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 1 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 16 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear something appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            186 => {
                /* RF6_S_UNDEAD */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons an undead adversary!\x00"
                                   as *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 1 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 17 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear something appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            187 => {
                /* RF6_S_DRAGON */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons a dragon!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 1 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 18 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear something appear nearby.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            188 => {
                /* RF6_S_HI_UNDEAD */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons greater undead!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 8 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 21 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many creepy things appear nearby.\x00"
                                  as *const u8 as *const libc::c_char);
                }
            }
            189 => {
                /* RF6_S_HI_DRAGON */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons ancient dragons!\x00"
                                   as *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 8 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 22 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many powerful things appear nearby.\x00"
                                  as *const u8 as *const libc::c_char);
                }
            }
            190 => {
                /* RF6_S_WRAITH */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons Wraith!\x00" as
                                   *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 8 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 31 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear immortal beings appear nearby.\x00"
                                  as *const u8 as *const libc::c_char);
                }
            }
            191 => {
                /* RF6_S_UNIQUE */
                disturb(1 as libc::c_int, 0 as libc::c_int);
                if blind != 0 {
                    msg_format(b"%^s mumbles.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                } else {
                    msg_format(b"%^s magically summons special opponents!\x00"
                                   as *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr());
                }
                k = 0 as libc::c_int;
                while k < 8 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 32 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                k = 0 as libc::c_int;
                while k < 8 as libc::c_int {
                    count +=
                        summon_specific(y, x, rlev, 21 as libc::c_int) as
                            libc::c_int;
                    k += 1
                }
                if blind as libc::c_int != 0 && count != 0 {
                    msg_print(b"You hear many powerful things appear nearby.\x00"
                                  as *const u8 as *const libc::c_char);
                }
            }
            97 | 148 | 172 | _ => { }
        }
    }
    /* Remember what the monster did to us */
    if seen != 0 {
        /* Inate spell */
        if thrown_spell < 32 as libc::c_int * 4 as libc::c_int {
            (*r_ptr).r_flags4 =
                ((*r_ptr).r_flags4 as libc::c_long |
                     (1 as libc::c_long) <<
                         thrown_spell - 32 as libc::c_int * 3 as libc::c_int)
                    as u32b;
            if ((*r_ptr).r_cast_inate as libc::c_int) < 255 as libc::c_int {
                (*r_ptr).r_cast_inate = (*r_ptr).r_cast_inate.wrapping_add(1)
            }
        } else if thrown_spell < 32 as libc::c_int * 5 as libc::c_int {
            (*r_ptr).r_flags5 =
                ((*r_ptr).r_flags5 as libc::c_long |
                     (1 as libc::c_long) <<
                         thrown_spell - 32 as libc::c_int * 4 as libc::c_int)
                    as u32b;
            if ((*r_ptr).r_cast_spell as libc::c_int) < 255 as libc::c_int {
                (*r_ptr).r_cast_spell = (*r_ptr).r_cast_spell.wrapping_add(1)
            }
        } else if thrown_spell < 32 as libc::c_int * 6 as libc::c_int {
            (*r_ptr).r_flags6 =
                ((*r_ptr).r_flags6 as libc::c_long |
                     (1 as libc::c_long) <<
                         thrown_spell - 32 as libc::c_int * 5 as libc::c_int)
                    as u32b;
            if ((*r_ptr).r_cast_spell as libc::c_int) < 255 as libc::c_int {
                (*r_ptr).r_cast_spell = (*r_ptr).r_cast_spell.wrapping_add(1)
            }
        }
    }
    /* Bolt or Ball */
    /* Special spell */
    /* Always take note of monsters that kill you */
    if death as libc::c_int != 0 &&
           ((*r_ptr).r_deaths as libc::c_int) < 32767 as libc::c_int {
        (*r_ptr).r_deaths += 1
    }
    /* A spell was cast */
    return 1 as libc::c_int as bool_;
}
/*
 * Returns whether a given monster will try to run from the player.
 *
 * Monsters will attempt to avoid very powerful players.  See below.
 *
 * Because this function is called so often, little details are important
 * for efficiency.  Like not using "mod" or "div" when possible.  And
 * attempting to check the conditions in an optimal order.  Note that
 * "(x << 2) == (x * 4)" if "x" has enough bits to hold the result.
 *
 * Note that this function is responsible for about one to five percent
 * of the processor use in normal conditions...
 */
unsafe extern "C" fn mon_will_run(mut m_idx: libc::c_int) -> libc::c_int {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut p_lev: u16b = 0;
    let mut m_lev: u16b = 0;
    let mut p_chp: u16b = 0;
    let mut p_mhp: u16b = 0;
    let mut m_chp: u16b = 0;
    let mut m_mhp: u16b = 0;
    let mut p_val: u32b = 0;
    let mut m_val: u32b = 0;
    /* Keep monsters from running too far away */
    if (*m_ptr).cdis as libc::c_int > 20 as libc::c_int + 5 as libc::c_int {
        return 0 as libc::c_int
    }
    /* Friends don't run away */
    if is_friend(m_ptr) >= 0 as libc::c_int { return 0 as libc::c_int }
    /* All "afraid" monsters will run away */
    if (*m_ptr).monfear != 0 { return 1 as libc::c_int }
    /* Nearby monsters will not become terrified */
    if (*m_ptr).cdis as libc::c_int <= 5 as libc::c_int {
        return 0 as libc::c_int
    }
    /* Examine player power (level) */
    p_lev = (*p_ptr).lev as u16b;
    /* Examine monster power (level plus morale) */
    m_lev =
        ((*m_ptr).level as libc::c_int + (m_idx & 0x8 as libc::c_int) +
             25 as libc::c_int) as u16b;
    /* Optimize extreme cases below */
    if m_lev as libc::c_int > p_lev as libc::c_int + 4 as libc::c_int {
        return 0 as libc::c_int
    }
    if m_lev as libc::c_int + 4 as libc::c_int <= p_lev as libc::c_int {
        return 1 as libc::c_int
    }
    /* Examine player health */
    p_chp = (*p_ptr).chp as u16b;
    p_mhp = (*p_ptr).mhp as u16b;
    /* Examine monster health */
    m_chp = (*m_ptr).hp as u16b;
    m_mhp = (*m_ptr).maxhp as u16b;
    /* Prepare to optimize the calculation */
    p_val =
        (p_lev as libc::c_int * p_mhp as libc::c_int +
             ((p_chp as libc::c_int) << 2 as libc::c_int)) as
            u32b; /* div p_mhp */
    m_val =
        (m_lev as libc::c_int * m_mhp as libc::c_int +
             ((m_chp as libc::c_int) << 2 as libc::c_int)) as
            u32b; /* div m_mhp */
    /* Strong players scare strong monsters */
    if p_val.wrapping_mul(m_mhp as libc::c_uint) >
           m_val.wrapping_mul(p_mhp as libc::c_uint) {
        return 1 as libc::c_int
    }
    /* Assume no terror */
    return 0 as libc::c_int;
}
/*
* Choose the "best" direction for "flowing"
*
* Note that ghosts and rock-eaters are never allowed to "flow",
* since they should move directly towards the player.
*
* Prefer "non-diagonal" directions, but twiddle them a little
* to angle slightly towards the player's actual location.
*
* Allow very perceptive monsters to track old "spoor" left by
* previous locations occupied by the player.  This will tend
* to have monsters end up either near the player or on a grid
* recently occupied by the player (and left via "teleport").
*
* Note that if "smell" is turned on, all monsters get vicious.
*
* Also note that teleporting away from a location will cause
* the monsters who were chasing you to converge on that location
* as long as you are still near enough to "annoy" them without
* being close enough to chase directly.  I have no idea what will
* happen if you combine "smell" with low "aaf" values.
*/
/*
* Provide a location to flee to, but give the player a wide berth.
*
* A monster may wish to flee to a location that is behind the player,
* but instead of heading directly for it, the monster should "swerve"
* around the player so that he has a smaller chance of getting hit.
*/
unsafe extern "C" fn get_fear_moves_aux(mut m_idx: libc::c_int,
                                        mut yp: *mut libc::c_int,
                                        mut xp: *mut libc::c_int) -> bool_ {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut fy: libc::c_int = 0;
    let mut fx: libc::c_int = 0;
    let mut gy: libc::c_int = 0 as libc::c_int;
    let mut gx: libc::c_int = 0 as libc::c_int;
    let mut when: libc::c_int = 0 as libc::c_int;
    let mut score: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    /* Monster flowing disabled */
    if flow_by_sound == 0 { return 0 as libc::c_int as bool_ }
    /* Monster location */
    fy = (*m_ptr).fy as libc::c_int;
    fx = (*m_ptr).fx as libc::c_int;
    /* Desired destination */
    y1 = fy - *yp;
    x1 = fx - *xp;
    /* The player is not currently near the monster grid */
    if ((*cave[fy as usize].offset(fx as isize)).when as libc::c_int) <
           (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).when as
               libc::c_int {
        /* No reason to attempt flowing */
        return 0 as libc::c_int as bool_
    }
    /* Monster is too far away to use flow information */
    if (*cave[fy as usize].offset(fx as isize)).cost as libc::c_int >
           32 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*cave[fy as usize].offset(fx as isize)).cost as libc::c_int >
           (*r_ptr).aaf as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Check nearby grids, diagonals first */
    i = 7 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut dis: libc::c_int = 0;
        let mut s: libc::c_int = 0;
        /* Get the location */
        y = fy + ddy_ddd[i as usize] as libc::c_int;
        x = fx + ddx_ddd[i as usize] as libc::c_int;
        /* Ignore illegal locations */
        if !((*cave[y as usize].offset(x as isize)).when as libc::c_int ==
                 0 as libc::c_int) {
            /* Ignore ancient locations */
            if !(((*cave[y as usize].offset(x as isize)).when as libc::c_int)
                     < when) {
                /* Calculate distance of this grid from our destination */
                dis = distance(y, x, y1, x1);
                /* Score this grid */
                s =
                    5000 as libc::c_int / (dis + 3 as libc::c_int) -
                        500 as libc::c_int /
                            ((*cave[y as usize].offset(x as isize)).cost as
                                 libc::c_int + 1 as libc::c_int);
                /* No negative scores */
                if s < 0 as libc::c_int { s = 0 as libc::c_int }
                /* Ignore lower scores */
                if !(s < score) {
                    /* Save the score and time */
                    when =
                        (*cave[y as usize].offset(x as isize)).when as
                            libc::c_int;
                    score = s;
                    /* Save the location */
                    gy = y;
                    gx = x
                }
            }
        }
        i -= 1
    }
    /* No legal move (?) */
    if when == 0 { return 0 as libc::c_int as bool_ }
    /* Find deltas */
    *yp = fy - gy;
    *xp = fx - gx;
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
* Choose a "safe" location near a monster for it to run toward.
*
* A location is "safe" if it can be reached quickly and the player
* is not able to fire into it (it isn't a "clean shot").  So, this will
* cause monsters to "duck" behind walls.  Hopefully, monsters will also
* try to run towards corridor openings if they are in a room.
*
* This function may take lots of CPU time if lots of monsters are
* fleeing.
*
* Return TRUE if a safe location is available.
*/
unsafe extern "C" fn find_safety(mut m_idx: libc::c_int,
                                 mut yp: *mut libc::c_int,
                                 mut xp: *mut libc::c_int) -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut fy: libc::c_int = (*m_ptr).fy as libc::c_int;
    let mut fx: libc::c_int = (*m_ptr).fx as libc::c_int;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut dis: libc::c_int = 0;
    let mut gy: libc::c_int = 0 as libc::c_int;
    let mut gx: libc::c_int = 0 as libc::c_int;
    let mut gdis: libc::c_int = 0 as libc::c_int;
    /* Start with adjacent locations, spread further */
    d = 1 as libc::c_int;
    while d < 10 as libc::c_int {
        /* Check nearby locations */
        y = fy - d;
        while y <= fy + d {
            let mut current_block_10: u64;
            x = fx - d;
            while x <= fx + d {
                /* Skip illegal locations */
                if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                       y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       x < cur_wid as libc::c_int - 1 as libc::c_int {
                    /* Skip locations in a wall */
                    if (*f_info.offset((*cave[y as
                                                  usize].offset(x as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x10 as libc::c_long != 0 &&
                           (*cave[y as usize].offset(x as isize)).feat as
                               libc::c_int != 0xaf as libc::c_int {
                        /* Check distance */
                        if !(distance(y, x, fy, fx) != d) {
                            /* Check for "availability" (if monsters can flow) */
                            if flow_by_sound != 0 {
                                /* Ignore grids very far from the player */
                                if ((*cave[y as
                                               usize].offset(x as isize)).when
                                        as libc::c_int) <
                                       (*cave[(*p_ptr).py as
                                                  usize].offset((*p_ptr).px as
                                                                    isize)).when
                                           as libc::c_int {
                                    current_block_10 = 6937071982253665452;
                                } else if (*cave[y as
                                                     usize].offset(x as
                                                                       isize)).cost
                                              as libc::c_int >
                                              (*cave[fy as
                                                         usize].offset(fx as
                                                                           isize)).cost
                                                  as libc::c_int +
                                                  2 as libc::c_int * d {
                                    current_block_10 = 6937071982253665452;
                                } else {
                                    current_block_10 = 12039483399334584727;
                                }
                            } else {
                                current_block_10 = 12039483399334584727;
                            }
                            match current_block_10 {
                                6937071982253665452 => { }
                                _ => {
                                    /* Ignore too-distant grids */
                                    /* Check for absence of shot */
                                    if projectable(y, x,
                                                   (*p_ptr).py as libc::c_int,
                                                   (*p_ptr).px as libc::c_int)
                                           == 0 {
                                        /* Calculate distance from player */
                                        dis =
                                            distance(y, x,
                                                     (*p_ptr).py as
                                                         libc::c_int,
                                                     (*p_ptr).px as
                                                         libc::c_int);
                                        /* Remember if further than previous */
                                        if dis > gdis {
                                            gy = y;
                                            gx = x;
                                            gdis = dis
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                x += 1
            }
            y += 1
        }
        /* Check for success */
        if gdis > 0 as libc::c_int {
            /* Good location */
            *yp = fy - gy;
            *xp = fx - gx;
            /* Found safe place */
            return 1 as libc::c_int as bool_
        }
        d += 1
    }
    /* No safe place */
    return 0 as libc::c_int as bool_;
}
/*
 * Choose a good hiding place near a monster for it to run toward.
 *
 * Pack monsters will use this to "ambush" the player and lure him out
 * of corridors into open space so they can swarm him.
 *
 * Return TRUE if a good location is available.
 */
unsafe extern "C" fn find_hiding(mut m_idx: libc::c_int,
                                 mut yp: *mut libc::c_int,
                                 mut xp: *mut libc::c_int) -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut fy: libc::c_int = (*m_ptr).fy as libc::c_int;
    let mut fx: libc::c_int = (*m_ptr).fx as libc::c_int;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut dis: libc::c_int = 0;
    let mut gy: libc::c_int = 0 as libc::c_int;
    let mut gx: libc::c_int = 0 as libc::c_int;
    let mut gdis: libc::c_int = 999 as libc::c_int;
    let mut min: libc::c_int = 0;
    /* Closest distance to get */
    min =
        distance((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int, fy,
                 fx) * 3 as libc::c_int / 4 as libc::c_int + 2 as libc::c_int;
    /* Start with adjacent locations, spread further */
    d = 1 as libc::c_int;
    while d < 10 as libc::c_int {
        /* Check nearby locations */
        y = fy - d;
        while y <= fy + d {
            x = fx - d;
            while x <= fx + d {
                /* Skip illegal locations */
                if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                       y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       x < cur_wid as libc::c_int - 1 as libc::c_int {
                    /* Skip locations in a wall */
                    if (*f_info.offset((*cave[y as
                                                  usize].offset(x as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x10 as libc::c_long != 0 &&
                           (*cave[y as usize].offset(x as isize)).feat as
                               libc::c_int != 0xaf as libc::c_int {
                        /* Check distance */
                        if !(distance(y, x, fy, fx) != d) {
                            /* Check for hidden, available grid */
                            if !((*cave[y as usize].offset(x as isize)).info
                                     as libc::c_int & 0x10 as libc::c_int !=
                                     0 as libc::c_int) &&
                                   clean_shot(fy, fx, y, x) as libc::c_int !=
                                       0 {
                                /* Calculate distance from player */
                                dis =
                                    distance(y, x, (*p_ptr).py as libc::c_int,
                                             (*p_ptr).px as libc::c_int);
                                /* Remember if closer than previous */
                                if dis < gdis && dis >= min {
                                    gy = y;
                                    gx = x;
                                    gdis = dis
                                }
                            }
                        }
                    }
                }
                x += 1
            }
            y += 1
        }
        /* Check for success */
        if gdis < 999 as libc::c_int {
            /* Good location */
            *yp = fy - gy;
            *xp = fx - gx;
            /* Found good place */
            return 1 as libc::c_int as bool_
        }
        d += 1
    }
    /* No good place */
    return 0 as libc::c_int as bool_;
}
/* Find an appropriate corpse */
#[no_mangle]
pub unsafe extern "C" fn find_corpse(mut m_ptr: *mut monster_type,
                                     mut y: *mut libc::c_int,
                                     mut x: *mut libc::c_int) {
    let mut k: libc::c_int = 0;
    let mut last: libc::c_int = -(1 as libc::c_int);
    k = 0 as libc::c_int;
    while k < max_o_idx as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *o_list.offset(k as isize) as *mut object_type;
        let mut rt_ptr: *mut monster_race = 0 as *mut monster_race;
        let mut rt2_ptr: *mut monster_race = 0 as *mut monster_race;
        if !((*o_ptr).k_idx == 0) {
            if !((*o_ptr).tval as libc::c_int != 9 as libc::c_int) {
                if !((*o_ptr).sval as libc::c_int != 1 as libc::c_int &&
                         (*o_ptr).sval as libc::c_int != 2 as libc::c_int) {
                    rt_ptr =
                        &mut *r_info.offset((*o_ptr).pval2 as isize) as
                            *mut monster_race;
                    /* Cannot incarnate into a higher level monster */
                    if !((*rt_ptr).level as libc::c_int >
                             (*m_ptr).level as libc::c_int) {
                        /* Must be in LOS */
                        if !(los((*m_ptr).fy as libc::c_int,
                                 (*m_ptr).fx as libc::c_int,
                                 (*o_ptr).iy as libc::c_int,
                                 (*o_ptr).ix as libc::c_int) == 0) {
                            if last != -(1 as libc::c_int) {
                                rt2_ptr =
                                    &mut *r_info.offset((*o_list.offset(last
                                                                            as
                                                                            isize)).pval2
                                                            as isize) as
                                        *mut monster_race;
                                if (*rt_ptr).level as libc::c_int >
                                       (*rt2_ptr).level as libc::c_int {
                                    last = k
                                }
                            } else { last = k }
                        }
                    }
                }
            }
        }
        k += 1
    }
    /* Must be ok now */
    if last != -(1 as libc::c_int) {
        *y = (*o_list.offset(last as isize)).iy as libc::c_int;
        *x = (*o_list.offset(last as isize)).ix as libc::c_int
    };
}
/*
 * Choose target
 */
unsafe extern "C" fn get_target_monster(mut m_idx: libc::c_int) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = -(1 as libc::c_int);
    let mut d: libc::c_int = 9999 as libc::c_int;
    /* Process the monsters (backwards) */
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        /* Access the monster */
        let mut t_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        /* hack should call the function for ego monsters ... but no_target i not meant to be added by ego and it speeds up the code */
        let mut rt_ptr: *mut monster_race =
            &mut *r_info.offset((*t_ptr).r_idx as isize) as *mut monster_race;
        let mut dd: libc::c_int = 0;
        /* Ignore "dead" monsters */
        if !((*t_ptr).r_idx == 0) {
            if !(m_idx == i) {
                /* Cannot be targeted */
                if !((*rt_ptr).flags7 & 0x800 as libc::c_int as libc::c_uint
                         != 0) {
                    if is_enemy(m_ptr, t_ptr) as libc::c_int != 0 &&
                           (los((*m_ptr).fy as libc::c_int,
                                (*m_ptr).fx as libc::c_int,
                                (*t_ptr).fy as libc::c_int,
                                (*t_ptr).fx as libc::c_int) as libc::c_int !=
                                0 &&
                                {
                                    dd =
                                        distance((*m_ptr).fy as libc::c_int,
                                                 (*m_ptr).fx as libc::c_int,
                                                 (*t_ptr).fy as libc::c_int,
                                                 (*t_ptr).fx as libc::c_int);
                                    (dd) < d
                                }) {
                        t = i;
                        d = dd
                    }
                }
            }
        }
        i -= 1
    }
    /* Hack */
    if is_friend(m_ptr) < 0 as libc::c_int &&
           los((*m_ptr).fy as libc::c_int, (*m_ptr).fx as libc::c_int,
               (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int) as
               libc::c_int != 0 &&
           distance((*m_ptr).fy as libc::c_int, (*m_ptr).fx as libc::c_int,
                    (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int) <
               d {
        t = 0 as libc::c_int
    }
    (*m_ptr).target = t as s16b;
}
/*
 * Choose "logical" directions for monster movement
 */
unsafe extern "C" fn get_moves(mut m_idx: libc::c_int,
                               mut mm: *mut libc::c_int) -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut y: libc::c_int = 0;
    let mut ay: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut ax: libc::c_int = 0;
    let mut move_val: libc::c_int = 0 as libc::c_int;
    let mut y2: libc::c_int = (*p_ptr).py as libc::c_int;
    let mut x2: libc::c_int = (*p_ptr).px as libc::c_int;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    /* Oups get nearer */
    if is_friend(m_ptr) > 0 as libc::c_int &&
           (*m_ptr).cdis as libc::c_int >
               (*p_ptr).pet_follow_distance as libc::c_int {
        y2 = (*p_ptr).py as libc::c_int;
        x2 = (*p_ptr).px as libc::c_int
    } else if (*m_ptr).target == 0 {
        y2 = (*p_ptr).py as libc::c_int;
        x2 = (*p_ptr).px as libc::c_int
    } else if (*m_ptr).target as libc::c_int > 0 as libc::c_int {
        y2 = (*m_list.offset((*m_ptr).target as isize)).fy as libc::c_int;
        x2 = (*m_list.offset((*m_ptr).target as isize)).fx as libc::c_int
    }
    /* Use the target */
    /* Hack doppleganger confuses monsters(even pets) */
    if doppleganger != 0 {
        if Rand_div(100 as libc::c_int) < 70 as libc::c_int {
            y2 = (*m_list.offset(doppleganger as isize)).fy as libc::c_int;
            x2 = (*m_list.offset(doppleganger as isize)).fx as libc::c_int
        }
    }
    /* A possessor is not interrested in the player, it only wants a corpse */
    if (*r_ptr).flags7 & 0x200 as libc::c_int as libc::c_uint != 0 {
        find_corpse(m_ptr, &mut y2, &mut x2);
    }
    /* Let quests redefine AI */
    if (*r_ptr).flags7 & 0x2000 as libc::c_int as libc::c_uint != 0 {
        if process_hooks_ret(19 as libc::c_int,
                             b"dd\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                             b"(d)\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, m_idx) != 0 {
            y2 = process_hooks_return[0 as libc::c_int as usize].num;
            x2 = process_hooks_return[1 as libc::c_int as usize].num
        }
    }
    if m_idx == (*p_ptr).control as libc::c_int {
        if (*r_ptr).flags7 & 0x20000 as libc::c_int as libc::c_uint != 0 ||
               Rand_div(100 as libc::c_int) < 85 as libc::c_int {
            if distance((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int,
                        (*m_ptr).fy as libc::c_int,
                        (*m_ptr).fx as libc::c_int) < 50 as libc::c_int {
                y2 =
                    (*m_ptr).fy as libc::c_int +
                        ddy[(*p_ptr).control_dir as usize] as libc::c_int;
                x2 =
                    (*m_ptr).fx as libc::c_int +
                        ddx[(*p_ptr).control_dir as usize] as libc::c_int
            }
        }
    }
    /* Extract the "pseudo-direction" */
    y = (*m_ptr).fy as libc::c_int - y2;
    x = (*m_ptr).fx as libc::c_int - x2;
    /* Tease the player */
    if (*r_ptr).flags7 & 0x1000 as libc::c_int as libc::c_uint != 0 {
        if distance((*m_ptr).fy as libc::c_int, (*m_ptr).fx as libc::c_int,
                    y2, x2) < 4 as libc::c_int {
            y = -y;
            x = -x
        }
    }
    /* Death orbs .. */
    if (*r_ptr).flags2 & 0x100 as libc::c_int as libc::c_uint != 0 {
        if los((*m_ptr).fy as libc::c_int, (*m_ptr).fx as libc::c_int, y2, x2)
               == 0 {
            return 0 as libc::c_int as bool_
        }
    }
    if stupid_monsters == 0 && is_friend(m_ptr) < 0 as libc::c_int {
        let mut tx: libc::c_int = x2;
        let mut ty: libc::c_int = y2;
        /*
		* Animal packs try to get the player out of corridors
		* (...unless they can move through walls -- TY)
		*/
        if (*r_ptr).flags1 & 0x2000 as libc::c_int as libc::c_uint != 0 &&
               (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 &&
               !((*r_ptr).flags2 & 0x40000 as libc::c_int as libc::c_uint != 0
                     ||
                     (*r_ptr).flags2 & 0x80000 as libc::c_int as libc::c_uint
                         != 0) {
            let mut i: libc::c_int = 0;
            let mut room: libc::c_int = 0 as libc::c_int;
            /* Count room grids next to player */
            i = 0 as libc::c_int;
            while i < 8 as libc::c_int {
                /* Check grid */
                if (*cave[(ty + ddy_ddd[i as usize] as libc::c_int) as
                              usize].offset((tx +
                                                 ddx_ddd[i as usize] as
                                                     libc::c_int) as
                                                isize)).info as libc::c_int &
                       0x8 as libc::c_int != 0 {
                    /* One more room grid */
                    room += 1
                }
                i += 1
            }
            /* Not in a room and strong player */
            if room < 8 as libc::c_int &&
                   (*p_ptr).chp as libc::c_int >
                       (*p_ptr).mhp as libc::c_int * 3 as libc::c_int /
                           4 as libc::c_int {
                /* Find hiding place */
                if find_hiding(m_idx, &mut y, &mut x) != 0 {
                    done = 1 as libc::c_int as bool_
                }
            }
        }
        /* Monster groups try to surround the player */
        if done == 0 &&
               (*r_ptr).flags1 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            let mut i_0: libc::c_int = 0;
            /* Find an empty square near the target to fill */
            i_0 = 0 as libc::c_int;
            while i_0 < 8 as libc::c_int {
                /* Pick squares near target (semi-randomly) */
                y2 =
                    ty +
                        ddy_ddd[(m_idx + i_0 & 7 as libc::c_int) as usize] as
                            libc::c_int;
                x2 =
                    tx +
                        ddx_ddd[(m_idx + i_0 & 7 as libc::c_int) as usize] as
                            libc::c_int;
                /* Already there? */
                if (*m_ptr).fy as libc::c_int == y2 &&
                       (*m_ptr).fx as libc::c_int == x2 {
                    /* Attack the target */
                    y2 = ty;
                    x2 = tx;
                    break ;
                } else {
                    /* Ignore filled grids */
                    if (*f_info.offset((*cave[y2 as
                                                  usize].offset(x2 as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x10 as libc::c_long != 0 &&
                           (*cave[y2 as usize].offset(x2 as isize)).feat as
                               libc::c_int != 0xaf as libc::c_int &&
                           (*cave[y2 as usize].offset(x2 as isize)).m_idx == 0
                           &&
                           !(y2 == (*p_ptr).py as libc::c_int &&
                                 x2 == (*p_ptr).px as libc::c_int) {
                        break ;
                    }
                    i_0 += 1
                }
            }
            /* Extract the new "pseudo-direction" */
            y = (*m_ptr).fy as libc::c_int - y2;
            x = (*m_ptr).fx as libc::c_int - x2;
            /* Done */
            done = 1 as libc::c_int as bool_
        }
    }
    /* Apply fear if possible and necessary */
    if stupid_monsters as libc::c_int != 0 ||
           is_friend(m_ptr) > 0 as libc::c_int {
        if mon_will_run(m_idx) != 0 {
            /* XXX XXX Not very "smart" */
            y = -y;
            x = -x
        }
    } else if done == 0 && mon_will_run(m_idx) != 0 {
        /* Try to find safe place */
        if find_safety(m_idx, &mut y, &mut x) == 0 {
            /* This is not a very "smart" method XXX XXX */
            y = -y;
            x = -x
        } else if flow_by_sound != 0 {
            /* Attempt to avoid the player */
            /* Adjust movement */
            get_fear_moves_aux(m_idx, &mut y, &mut x);
        }
    }
    if stupid_monsters == 0 {
        /* Check for no move */
        if x == 0 && y == 0 { return 0 as libc::c_int as bool_ }
    }
    /* Extract the "absolute distances" */
    ax = if x < 0 as libc::c_int { -x } else { x };
    ay = if y < 0 as libc::c_int { -y } else { y };
    /* Do something weird */
    if y < 0 as libc::c_int { move_val += 8 as libc::c_int }
    if x > 0 as libc::c_int { move_val += 4 as libc::c_int }
    /* Prevent the diamond maneuvre */
    if ay > ax << 1 as libc::c_int {
        move_val += 1;
        move_val += 1
    } else if ax > ay << 1 as libc::c_int { move_val += 1 }
    /* Extract some directions */
    match move_val {
        0 => {
            *mm.offset(0 as libc::c_int as isize) = 9 as libc::c_int;
            if ay > ax {
                *mm.offset(1 as libc::c_int as isize) = 8 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 6 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 7 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 3 as libc::c_int
            } else {
                *mm.offset(1 as libc::c_int as isize) = 6 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 8 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 3 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 7 as libc::c_int
            }
        }
        1 | 9 => {
            *mm.offset(0 as libc::c_int as isize) = 6 as libc::c_int;
            if y < 0 as libc::c_int {
                *mm.offset(1 as libc::c_int as isize) = 3 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 9 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 2 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 8 as libc::c_int
            } else {
                *mm.offset(1 as libc::c_int as isize) = 9 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 3 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 8 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 2 as libc::c_int
            }
        }
        2 | 6 => {
            *mm.offset(0 as libc::c_int as isize) = 8 as libc::c_int;
            if x < 0 as libc::c_int {
                *mm.offset(1 as libc::c_int as isize) = 9 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 7 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 6 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 4 as libc::c_int
            } else {
                *mm.offset(1 as libc::c_int as isize) = 7 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 9 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 4 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 6 as libc::c_int
            }
        }
        4 => {
            *mm.offset(0 as libc::c_int as isize) = 7 as libc::c_int;
            if ay > ax {
                *mm.offset(1 as libc::c_int as isize) = 8 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 4 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 9 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 1 as libc::c_int
            } else {
                *mm.offset(1 as libc::c_int as isize) = 4 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 8 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 9 as libc::c_int
            }
        }
        5 | 13 => {
            *mm.offset(0 as libc::c_int as isize) = 4 as libc::c_int;
            if y < 0 as libc::c_int {
                *mm.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 7 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 2 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 8 as libc::c_int
            } else {
                *mm.offset(1 as libc::c_int as isize) = 7 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 1 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 8 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 2 as libc::c_int
            }
        }
        8 => {
            *mm.offset(0 as libc::c_int as isize) = 3 as libc::c_int;
            if ay > ax {
                *mm.offset(1 as libc::c_int as isize) = 2 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 6 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 9 as libc::c_int
            } else {
                *mm.offset(1 as libc::c_int as isize) = 6 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 2 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 9 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 1 as libc::c_int
            }
        }
        10 | 14 => {
            *mm.offset(0 as libc::c_int as isize) = 2 as libc::c_int;
            if x < 0 as libc::c_int {
                *mm.offset(1 as libc::c_int as isize) = 3 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 1 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 6 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 4 as libc::c_int
            } else {
                *mm.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 3 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 4 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 6 as libc::c_int
            }
        }
        12 => {
            *mm.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
            if ay > ax {
                *mm.offset(1 as libc::c_int as isize) = 2 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 4 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 3 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 7 as libc::c_int
            } else {
                *mm.offset(1 as libc::c_int as isize) = 4 as libc::c_int;
                *mm.offset(2 as libc::c_int as isize) = 2 as libc::c_int;
                *mm.offset(3 as libc::c_int as isize) = 7 as libc::c_int;
                *mm.offset(4 as libc::c_int as isize) = 3 as libc::c_int
            }
        }
        _ => { }
    }
    /* Wants to move... */
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn check_hit2(mut power: libc::c_int,
                                    mut level: libc::c_int,
                                    mut ac: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    /* Percentile dice */
    k = Rand_div(100 as libc::c_int);
    /* Hack -- Always miss or hit */
    if k < 10 as libc::c_int { return (k < 5 as libc::c_int) as libc::c_int }
    /* Calculate the "attack quality" */
    i = power + level * 3 as libc::c_int;
    /* Power and Level compete against Armor */
    if i > 0 as libc::c_int &&
           Rand_div(i) + 1 as libc::c_int >
               ac * 3 as libc::c_int / 4 as libc::c_int {
        return 1 as libc::c_int
    }
    /* Assume miss */
    return 0 as libc::c_int;
}
/* Monster attacks monster */
unsafe extern "C" fn monst_attack_monst(mut m_idx: libc::c_int,
                                        mut t_idx: libc::c_int) -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut t_ptr: *mut monster_type =
        &mut *m_list.offset(t_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
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
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut t_name: [libc::c_char; 80] = [0; 80];
    let mut ddesc: [libc::c_char; 80] = [0; 80];
    let mut temp: [libc::c_char; 80] = [0; 80];
    let mut blinked: bool_ = 0 as libc::c_int as bool_;
    let mut touched: bool_ = 0 as libc::c_int as bool_;
    let mut explode: bool_ = 0 as libc::c_int as bool_;
    let mut fear: bool_ = 0 as libc::c_int as bool_;
    let mut y_saver: byte_hack = (*t_ptr).fy;
    let mut x_saver: byte_hack = (*t_ptr).fx;
    /* Not allowed to attack */
    if (*r_ptr).flags1 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*tr_ptr).flags7 & 0x100000 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Total armor */
    ac = (*t_ptr).ac as libc::c_int;
    /* Extract the effective monster level */
    rlev =
        if (*m_ptr).level as libc::c_int >= 1 as libc::c_int {
            (*m_ptr).level as libc::c_int
        } else { 1 as libc::c_int };
    /* Get the monster name (or "it") */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    /* Get the monster name (or "it") */
    monster_desc(t_name.as_mut_ptr(), t_ptr, 0 as libc::c_int);
    /* Get the "died from" information (i.e. "a kobold") */
    monster_desc(ddesc.as_mut_ptr(), m_ptr, 0x88 as libc::c_int);
    /* Assume no blink */
    blinked = 0 as libc::c_int as bool_;
    if !((*m_ptr).ml as libc::c_int != 0 || (*t_ptr).ml as libc::c_int != 0) {
        monster_msg(b"You hear noise.\x00" as *const u8 as
                        *const libc::c_char);
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
            (*m_ptr).blow[ap_cnt as usize].effect as libc::c_int;
        let mut method: libc::c_int =
            (*m_ptr).blow[ap_cnt as usize].method as libc::c_int;
        let mut d_dice: libc::c_int =
            (*m_ptr).blow[ap_cnt as usize].d_dice as libc::c_int;
        let mut d_side: libc::c_int =
            (*m_ptr).blow[ap_cnt as usize].d_side as libc::c_int;
        if t_ptr == m_ptr {
            /* Paranoia */
            if wizard != 0 {
                monster_msg(b"Monster attacking self?\x00" as *const u8 as
                                *const libc::c_char);
            }
            break ;
        } else {
            /* Stop attacking if the target dies! */
            if (*t_ptr).fx as libc::c_int != x_saver as libc::c_int ||
                   (*t_ptr).fy as libc::c_int != y_saver as libc::c_int {
                break ;
            }
            /* Hack -- no more attacks */
            if method == 0 { break ; }
            (blinked) != 0;
            /* Extract visibility (before blink) */
            if (*m_ptr).ml != 0 { visible = 1 as libc::c_int as bool_ }
            /* Extract the attack "power" */
            power = get_attack_power(effect);
            /* Monster hits*/
            if effect == 0 || check_hit2(power, rlev, ac) != 0 {
                /* Always disturbing */
                if disturb_other != 0 {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
                /* Describe the attack method */
                match method {
                    1 => {
                        act =
                            b"hits %s.\x00" as *const u8 as
                                *const libc::c_char;
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
                            b"kicks %s.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_
                    }
                    5 => {
                        act =
                            b"claws %s.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_
                    }
                    6 => {
                        act =
                            b"bites %s.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_
                    }
                    7 => {
                        act =
                            b"stings %s.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_
                    }
                    8 => {
                        act =
                            b"XXX1\'s %s.\x00" as *const u8 as
                                *const libc::c_char
                    }
                    9 => {
                        act =
                            b"butts %s.\x00" as *const u8 as
                                *const libc::c_char;
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
                    16 => {
                        act =
                            b"explodes.\x00" as *const u8 as
                                *const libc::c_char;
                        explode = 1 as libc::c_int as bool_;
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
                    strfmt(temp.as_mut_ptr(), act, t_name.as_mut_ptr());
                    if (*m_ptr).ml as libc::c_int != 0 ||
                           (*t_ptr).ml as libc::c_int != 0 {
                        monster_msg(b"%^s %s\x00" as *const u8 as
                                        *const libc::c_char,
                                    m_name.as_mut_ptr(), temp.as_mut_ptr());
                    }
                }
                /* Hack -- assume all attacks are obvious */
                obvious = 1 as libc::c_int as bool_;
                /* Roll out the damage */
                damage = damroll(d_dice as s16b, d_side as s16b);
                /* Hack need more punch against monsters */
                damage *= 3 as libc::c_int;
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
                    32 | 14 => { pt = 22 as libc::c_int }
                    15 => { pt = 66 as libc::c_int }
                    16 => { pt = 57 as libc::c_int }
                    13 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 33 => { }
                    24 => {
                        if damage > 23 as libc::c_int {
                            /* Prevent destruction of quest levels and town */
                            if is_quest(dun_level as libc::c_int) == 0 &&
                                   dun_level as libc::c_int != 0 {
                                earthquake((*m_ptr).fy as libc::c_int,
                                           (*m_ptr).fx as libc::c_int,
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
                    if explode == 0 {
                        project(m_idx, 0 as libc::c_int,
                                (*t_ptr).fy as libc::c_int,
                                (*t_ptr).fx as libc::c_int,
                                if pt == 57 as libc::c_int {
                                    (*m_ptr).level as libc::c_int
                                } else { damage }, pt,
                                0x40 as libc::c_int | 0x8 as libc::c_int);
                    }
                    if touched != 0 {
                        /* Aura fire */
                        if (*tr_ptr).flags2 &
                               0x4000 as libc::c_int as libc::c_uint != 0 &&
                               (*r_ptr).flags3 &
                                   0x40000 as libc::c_int as libc::c_uint == 0
                           {
                            if (*m_ptr).ml as libc::c_int != 0 ||
                                   (*t_ptr).ml as libc::c_int != 0 {
                                blinked = 0 as libc::c_int as bool_;
                                monster_msg(b"%^s is suddenly very hot!\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            m_name.as_mut_ptr());
                                if (*t_ptr).ml != 0 {
                                    (*tr_ptr).r_flags2 |=
                                        0x4000 as libc::c_int as libc::c_uint
                                }
                            }
                            project(t_idx, 0 as libc::c_int,
                                    (*m_ptr).fy as libc::c_int,
                                    (*m_ptr).fx as libc::c_int,
                                    damroll((1 as libc::c_int +
                                                 (*t_ptr).level as libc::c_int
                                                     / 26 as libc::c_int) as
                                                s16b,
                                            (1 as libc::c_int +
                                                 (*t_ptr).level as libc::c_int
                                                     / 17 as libc::c_int) as
                                                s16b), 5 as libc::c_int,
                                    0x40 as libc::c_int | 0x8 as libc::c_int);
                        }
                        /* Aura elec */
                        if (*tr_ptr).flags2 &
                               0x8000 as libc::c_int as libc::c_uint != 0 &&
                               (*r_ptr).flags3 &
                                   0x20000 as libc::c_int as libc::c_uint == 0
                           {
                            if (*m_ptr).ml as libc::c_int != 0 ||
                                   (*t_ptr).ml as libc::c_int != 0 {
                                blinked = 0 as libc::c_int as bool_;
                                monster_msg(b"%^s gets zapped!\x00" as
                                                *const u8 as
                                                *const libc::c_char,
                                            m_name.as_mut_ptr());
                                if (*t_ptr).ml != 0 {
                                    (*tr_ptr).r_flags2 |=
                                        0x8000 as libc::c_int as libc::c_uint
                                }
                            }
                            project(t_idx, 0 as libc::c_int,
                                    (*m_ptr).fy as libc::c_int,
                                    (*m_ptr).fx as libc::c_int,
                                    damroll((1 as libc::c_int +
                                                 (*t_ptr).level as libc::c_int
                                                     / 26 as libc::c_int) as
                                                s16b,
                                            (1 as libc::c_int +
                                                 (*t_ptr).level as libc::c_int
                                                     / 17 as libc::c_int) as
                                                s16b), 1 as libc::c_int,
                                    0x40 as libc::c_int | 0x8 as libc::c_int);
                        }
                    }
                }
            } else {
                /* Monster missed player */
                /* Analyze failed attacks */
                match method {
                    1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => {
                        /* Visible monsters */
                        if (*m_ptr).ml != 0 {
                            /* Disturbing */
                            disturb(1 as libc::c_int, 0 as libc::c_int);
                            /* Message */
                            monster_msg(b"%^s misses %s.\x00" as *const u8 as
                                            *const libc::c_char,
                                        m_name.as_mut_ptr(),
                                        t_name.as_mut_ptr());
                        }
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
    }
    if explode != 0 {
        sound(64 as libc::c_int);
        mon_take_hit_mon(m_idx, m_idx, (*m_ptr).hp + 1 as libc::c_int,
                         &mut fear,
                         b" explodes into tiny shreds.\x00" as *const u8 as
                             *const libc::c_char);
        blinked = 0 as libc::c_int as bool_
    }
    /* Blink away */
    if blinked != 0 {
        if (*m_ptr).ml != 0 {
            monster_msg(b"The thief flees laughing!\x00" as *const u8 as
                            *const libc::c_char);
        } else {
            monster_msg(b"You hear laughter!\x00" as *const u8 as
                            *const libc::c_char);
        }
        teleport_away(m_idx,
                      20 as libc::c_int * 2 as libc::c_int +
                          5 as libc::c_int);
    }
    return 1 as libc::c_int as bool_;
}
/*
 * Hack -- local "player stealth" value (see below)
 */
static mut noise: u32b = 0 as libc::c_long as u32b;
/* Determine whether the player is invisible to a monster */
unsafe extern "C" fn player_invis(mut m_ptr: *mut monster_type) -> bool_ {
    let mut inv: s16b = 0;
    let mut mlv: s16b = 0;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    inv = (*p_ptr).invis;
    mlv = (*m_ptr).level as s16b;
    if (*r_ptr).flags3 & 0x80000000 as libc::c_uint != 0 {
        mlv = (mlv as libc::c_int + 10 as libc::c_int) as s16b
    }
    if (*r_ptr).flags3 & 0x8 as libc::c_int as libc::c_uint != 0 {
        mlv = (mlv as libc::c_int + 20 as libc::c_int) as s16b
    }
    if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
        mlv = (mlv as libc::c_int + 15 as libc::c_int) as s16b
    }
    if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 {
        mlv = (mlv as libc::c_int + 15 as libc::c_int) as s16b
    }
    if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 {
        mlv = (mlv as libc::c_int + 15 as libc::c_int) as s16b
    }
    if (*r_ptr).flags3 & 0x1 as libc::c_int as libc::c_uint != 0 {
        mlv = (mlv as libc::c_int - 15 as libc::c_int) as s16b
    }
    if (*r_ptr).flags3 & 0x2 as libc::c_int as libc::c_uint != 0 {
        mlv = (mlv as libc::c_int - 10 as libc::c_int) as s16b
    }
    if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 {
        mlv = (mlv as libc::c_int / 2 as libc::c_int) as s16b
    }
    if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint != 0 {
        mlv =
            (mlv as libc::c_int * 5 as libc::c_int / 4 as libc::c_int) as s16b
    }
    if (*m_ptr).mflag & 0x2 as libc::c_int != 0 {
        inv = 0 as libc::c_int as s16b
    }
    if (*r_ptr).flags2 & 0x10 as libc::c_int as libc::c_uint != 0 {
        inv = 0 as libc::c_int as s16b
    }
    if (*m_ptr).mflag & 0x8 as libc::c_int != 0 {
        inv = 0 as libc::c_int as s16b
    }
    if (mlv as libc::c_int) < 1 as libc::c_int {
        mlv = 1 as libc::c_int as s16b
    }
    return (inv as libc::c_int >=
                Rand_div(mlv as libc::c_int * 2 as libc::c_int) +
                    1 as libc::c_int) as libc::c_int as bool_;
}
/*
 * Process a monster
 *
 * The monster is known to be within 100 grids of the player
 *
 * In several cases, we directly update the monster lore
 *
 * Note that a monster is only allowed to "reproduce" if there
 * are a limited number of "reproducing" monsters on the current
 * level.  This should prevent the level from being "swamped" by
 * reproducing monsters.  It also allows a large mass of mice to
 * prevent a louse from multiplying, but this is a small price to
 * pay for a simple multiplication method.
 *
 * XXX Monster fear is slightly odd, in particular, monsters will
 * fixate on opening a door even if they cannot open it.  Actually,
 * the same thing happens to normal monsters when they hit a door
 *
 * XXX XXX XXX In addition, monsters which *cannot* open or bash
 * down a door will still stand there trying to open it...
 *
 * XXX Technically, need to check for monster in the way
 * combined with that monster being in a wall (or door?)
 *
 * A "direction" of "5" means "pick a random direction".
 */
unsafe extern "C" fn process_monster(mut m_idx: libc::c_int,
                                     mut is_frien: bool_) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset((*m_ptr).fy as
                                             isize)).offset((*m_ptr).fx as
                                                                isize) as
            *mut cave_type;
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut oy: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut mm: [libc::c_int; 8] = [0; 8];
    let mut y_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut do_turn: bool_ = 0;
    let mut do_move: bool_ = 0;
    let mut do_view: bool_ = 0;
    let mut did_open_door: bool_ = 0;
    let mut did_bash_door: bool_ = 0;
    let mut did_take_item: bool_ = 0;
    let mut did_kill_item: bool_ = 0;
    let mut did_move_body: bool_ = 0;
    let mut did_kill_body: bool_ = 0;
    let mut did_pass_wall: bool_ = 0;
    let mut did_kill_wall: bool_ = 0;
    let mut gets_angry: bool_ = 0 as libc::c_int as bool_;
    let mut inv: bool_ = 0;
    let mut xxx: bool_ = 0 as libc::c_int as bool_;
    inv = player_invis(m_ptr);
    if (*r_ptr).flags9 & 0x800 as libc::c_int as libc::c_uint != 0 {
        doppleganger = m_idx as s16b
    }
    /* Handle "bleeding" */
    if (*m_ptr).bleeding != 0 {
        let mut d_0: libc::c_int =
            1 as libc::c_int + (*m_ptr).maxhp / 50 as libc::c_int;
        if d_0 > (*m_ptr).bleeding as libc::c_int {
            d_0 = (*m_ptr).bleeding as libc::c_int
        }
        /* Exit if the monster dies */
        if mon_take_hit(m_idx, d_0, &mut xxx,
                        b" bleeds to death.\x00" as *const u8 as
                            *const libc::c_char) != 0 {
            return
        }
        /* Hack -- Recover from bleeding */
        if (*m_ptr).bleeding as libc::c_int > d_0 {
            /* Recover somewhat */
            (*m_ptr).bleeding =
                ((*m_ptr).bleeding as libc::c_int - d_0) as s16b
        } else {
            /* Fully recover */
            /* Recover fully */
            (*m_ptr).bleeding = 0 as libc::c_int as s16b;
            if (*m_ptr).ml != 0 {
                let mut m_name: [libc::c_char; 80] = [0; 80];
                /* Get the monster name */
                monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
                /* Dump a message */
                msg_format(b"%^s is no longer bleeding.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
                /* Hack -- Update the health bar */
                if health_who as libc::c_int == m_idx {
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x800 as libc::c_long) as u32b
                }
            }
        }
    }
    /* Message if visible */
    /* Handle "poisoned" */
    if (*m_ptr).poisoned != 0 {
        let mut d_1: libc::c_int =
            (*m_ptr).poisoned as libc::c_int / 10 as libc::c_int;
        if d_1 < 1 as libc::c_int { d_1 = 1 as libc::c_int }
        /* Exit if the monster dies */
        if mon_take_hit(m_idx, d_1, &mut xxx,
                        b" dies of poison.\x00" as *const u8 as
                            *const libc::c_char) != 0 {
            return
        }
        /* Hack -- Recover from bleeding */
        if (*m_ptr).poisoned as libc::c_int > d_1 {
            /* Recover somewhat */
            (*m_ptr).poisoned =
                ((*m_ptr).poisoned as libc::c_int - d_1) as s16b
        } else {
            /* Fully recover */
            /* Recover fully */
            (*m_ptr).poisoned = 0 as libc::c_int as s16b;
            if (*m_ptr).ml != 0 {
                let mut m_name_0: [libc::c_char; 80] = [0; 80];
                /* Get the monster name */
                monster_desc(m_name_0.as_mut_ptr(), m_ptr, 0 as libc::c_int);
                /* Dump a message */
                msg_format(b"%^s is no longer poisoned.\x00" as *const u8 as
                               *const libc::c_char, m_name_0.as_mut_ptr());
                /* Hack -- Update the health bar */
                if health_who as libc::c_int == m_idx {
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x800 as libc::c_long) as u32b
                }
            }
        }
    }
    /* Message if visible */
    /* Handle "sleep" */
    if (*m_ptr).csleep != 0 {
        let mut notice: u32b = 0 as libc::c_int as u32b;
        /* Hack -- handle non-aggravation */
        if (*p_ptr).aggravate == 0 {
            notice = Rand_div(1024 as libc::c_int) as u32b
        }
        /* Hack -- See if monster "notices" player */
        if notice.wrapping_mul(notice).wrapping_mul(notice) <= noise {
            /* Hack -- amount of "waking" */
            let mut d_2: libc::c_int = 1 as libc::c_int;
            /* Wake up faster near the player */
            if ((*m_ptr).cdis as libc::c_int) < 50 as libc::c_int {
                d_2 = 100 as libc::c_int / (*m_ptr).cdis as libc::c_int
            }
            /* Hack -- handle aggravation */
            if (*p_ptr).aggravate != 0 {
                d_2 = (*m_ptr).csleep as libc::c_int
            }
            /* Still asleep */
            if (*m_ptr).csleep as libc::c_int > d_2 {
                /* Monster wakes up "a little bit" */
                (*m_ptr).csleep =
                    ((*m_ptr).csleep as libc::c_int - d_2) as s16b;
                /* Notice the "not waking up" */
                if (*m_ptr).ml != 0 {
                    /* Hack -- Count the ignores */
                    if ((*r_ptr).r_ignore as libc::c_int) < 255 as libc::c_int
                       {
                        (*r_ptr).r_ignore = (*r_ptr).r_ignore.wrapping_add(1)
                    }
                }
            } else {
                /* Just woke up */
                /* Reset sleep counter */
                (*m_ptr).csleep = 0 as libc::c_int as s16b;
                if (*m_ptr).ml != 0 {
                    let mut m_name_1: [libc::c_char; 80] = [0; 80];
                    /* Acquire the monster name */
                    monster_desc(m_name_1.as_mut_ptr(), m_ptr,
                                 0 as libc::c_int);
                    /* Dump a message */
                    msg_format(b"%^s wakes up.\x00" as *const u8 as
                                   *const libc::c_char,
                               m_name_1.as_mut_ptr());
                    /* Hack -- Count the wakings */
                    if ((*r_ptr).r_wake as libc::c_int) < 255 as libc::c_int {
                        (*r_ptr).r_wake = (*r_ptr).r_wake.wrapping_add(1)
                    }
                }
            }
        }
        /* Notice the "waking up" */
        /* Still sleeping */
        if (*m_ptr).csleep != 0 { return }
    }
    /* Handle "stun" */
    if (*m_ptr).stunned != 0 {
        let mut d_3: libc::c_int = 1 as libc::c_int;
        /* Make a "saving throw" against stun */
        if Rand_div(5000 as libc::c_int) <=
               (*m_ptr).level as libc::c_int * (*m_ptr).level as libc::c_int {
            /* Recover fully */
            d_3 = (*m_ptr).stunned as libc::c_int
        }
        /* Hack -- Recover from stun */
        if (*m_ptr).stunned as libc::c_int > d_3 {
            /* Recover somewhat */
            (*m_ptr).stunned =
                ((*m_ptr).stunned as libc::c_int - d_3) as byte_hack
        } else {
            /* Fully recover */
            /* Recover fully */
            (*m_ptr).stunned = 0 as libc::c_int as byte_hack;
            if (*m_ptr).ml != 0 {
                let mut m_name_2: [libc::c_char; 80] = [0; 80];
                /* Acquire the monster name */
                monster_desc(m_name_2.as_mut_ptr(), m_ptr, 0 as libc::c_int);
                /* Dump a message */
                msg_format(b"%^s is no longer stunned.\x00" as *const u8 as
                               *const libc::c_char, m_name_2.as_mut_ptr());
            }
        }
        /* Message if visible */
        /* Still stunned */
        if (*m_ptr).stunned != 0 { return }
    }
    /* Handle confusion */
    if (*m_ptr).confused != 0 {
        /* Amount of "boldness" */
        let mut d_4: libc::c_int =
            Rand_div((*m_ptr).level as libc::c_int / 10 as libc::c_int +
                         1 as libc::c_int) + 1 as libc::c_int;
        /* Still confused */
        if (*m_ptr).confused as libc::c_int > d_4 {
            /* Reduce the confusion */
            (*m_ptr).confused =
                ((*m_ptr).confused as libc::c_int - d_4) as byte_hack
        } else {
            /* Recovered */
            /* No longer confused */
            (*m_ptr).confused = 0 as libc::c_int as byte_hack;
            if (*m_ptr).ml != 0 {
                let mut m_name_3: [libc::c_char; 80] = [0; 80];
                /* Acquire the monster name */
                monster_desc(m_name_3.as_mut_ptr(), m_ptr, 0 as libc::c_int);
                /* Dump a message */
                msg_format(b"%^s is no longer confused.\x00" as *const u8 as
                               *const libc::c_char, m_name_3.as_mut_ptr());
            }
        }
    }
    /* Message if visible */
    /* No one wants to be your friend if you're aggravating */
    if (*m_ptr).status as libc::c_int > 0 as libc::c_int &&
           ((*m_ptr).status as libc::c_int) < 4 as libc::c_int &&
           (*p_ptr).aggravate as libc::c_int != 0 &&
           (*r_ptr).flags7 & 0x10 as libc::c_int as libc::c_uint == 0 {
        gets_angry = 1 as libc::c_int as bool_
    }
    /* Paranoia... no friendly uniques outside wizard mode -- TY */
    if (*m_ptr).status as libc::c_int > 0 as libc::c_int &&
           ((*m_ptr).status as libc::c_int) < 4 as libc::c_int && wizard == 0
           && (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
           (*r_ptr).flags7 & 0x10 as libc::c_int as libc::c_uint == 0 {
        gets_angry = 1 as libc::c_int as bool_
    }
    if gets_angry != 0 {
        let mut m_name_4: [libc::c_char; 80] = [0; 80];
        monster_desc(m_name_4.as_mut_ptr(), m_ptr, 0 as libc::c_int);
        match is_friend(m_ptr) {
            1 => {
                msg_format(b"%^s suddenly becomes hostile!\x00" as *const u8
                               as *const libc::c_char, m_name_4.as_mut_ptr());
                change_side(m_ptr);
            }
            _ => { }
        }
    }
    /* Handle "fear" */
    if (*m_ptr).monfear != 0 {
        /* Amount of "boldness" */
        let mut d_5: libc::c_int =
            Rand_div((*m_ptr).level as libc::c_int / 10 as libc::c_int +
                         1 as libc::c_int) + 1 as libc::c_int;
        /* Still afraid */
        if (*m_ptr).monfear as libc::c_int > d_5 {
            /* Reduce the fear */
            (*m_ptr).monfear =
                ((*m_ptr).monfear as libc::c_int - d_5) as byte_hack
        } else {
            /* Recover from fear, take note if seen */
            /* No longer afraid */
            (*m_ptr).monfear = 0 as libc::c_int as byte_hack;
            if (*m_ptr).ml != 0 {
                let mut m_name_5: [libc::c_char; 80] = [0; 80];
                let mut m_poss: [libc::c_char; 80] = [0; 80];
                /* Acquire the monster name/poss */
                monster_desc(m_name_5.as_mut_ptr(), m_ptr, 0 as libc::c_int);
                monster_desc(m_poss.as_mut_ptr(), m_ptr, 0x22 as libc::c_int);
                /* Dump a message */
                msg_format(b"%^s recovers %s courage.\x00" as *const u8 as
                               *const libc::c_char, m_name_5.as_mut_ptr(),
                           m_poss.as_mut_ptr());
            }
        }
    }
    /* Visual note */
    /* Get the origin */
    oy = (*m_ptr).fy as libc::c_int;
    ox = (*m_ptr).fx as libc::c_int;
    /* Attempt to "multiply" if able and allowed */
    if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 &&
           (num_repro as libc::c_int) < 100 as libc::c_int {
        if ai_multiply(m_idx) != 0 { return }
    }
    if speak_unique != 0 {
        if Rand_div(8 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
            if (*cave[oy as usize].offset(ox as isize)).info as libc::c_int &
                   0x20 as libc::c_int != 0 as libc::c_int &&
                   (*r_ptr).flags2 & 0x4 as libc::c_int as libc::c_uint != 0 {
                let mut m_name_6: [libc::c_char; 80] = [0; 80];
                let mut monmessage: [libc::c_char; 80] = [0; 80];
                /* Acquire the monster name/poss */
                if (*m_ptr).ml != 0 {
                    monster_desc(m_name_6.as_mut_ptr(), m_ptr,
                                 0 as libc::c_int);
                } else {
                    strcpy(m_name_6.as_mut_ptr(),
                           b"It\x00" as *const u8 as *const libc::c_char);
                }
                /* xtra_line function by Matt Graham--allow uniques to */
				/* say "unique" things based on their monster index.   */
				/* Try for the unique's lines in "monspeak.txt" first. */
				/* 0 is SUCCESS, of course....                         */
                if process_hooks(33 as libc::c_int,
                                 b"(d,s)\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 m_idx, m_name_6.as_mut_ptr()) == 0 {
                    if get_xtra_line(b"monspeak.txt\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char, m_ptr,
                                     monmessage.as_mut_ptr()) !=
                           0 as libc::c_int {
                        /* Get a message from old defaults if new don't work */
                        if is_friend(m_ptr) > 0 as libc::c_int {
                            get_rnd_line(b"speakpet.txt\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         monmessage.as_mut_ptr());
                        } else if (*m_ptr).monfear != 0 {
                            get_rnd_line(b"monfear.txt\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         monmessage.as_mut_ptr());
                        } else {
                            get_rnd_line(b"bravado.txt\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         monmessage.as_mut_ptr());
                        }
                    }
                    msg_format(b"%^s %s\x00" as *const u8 as
                                   *const libc::c_char, m_name_6.as_mut_ptr(),
                               monmessage.as_mut_ptr());
                }
            }
        }
    }
    /* Need a new target ? */
    if (*m_ptr).target as libc::c_int == -(1 as libc::c_int) ||
           Rand_div(100 as libc::c_int) < 10 as libc::c_int {
        get_target_monster(m_idx);
    }
    /* Attempt to cast a spell */
    if make_attack_spell(m_idx) != 0 { return }
    /*
	 * Attempt to cast a spell at an enemy other than the player
	 * (may slow the game a smidgeon, but I haven't noticed.)
	 */
    hack_message_pain_may_silent = 1 as libc::c_int as bool_;
    if monst_spell_monst(m_idx) != 0 {
        hack_message_pain_may_silent = 0 as libc::c_int as bool_;
        return
    }
    hack_message_pain_may_silent = 0 as libc::c_int as bool_;
    /* Hack -- Assume no movement */
    mm[3 as libc::c_int as usize] = 0 as libc::c_int;
    mm[2 as libc::c_int as usize] = mm[3 as libc::c_int as usize];
    mm[1 as libc::c_int as usize] = mm[2 as libc::c_int as usize];
    mm[0 as libc::c_int as usize] = mm[1 as libc::c_int as usize];
    mm[7 as libc::c_int as usize] = 0 as libc::c_int;
    mm[6 as libc::c_int as usize] = mm[7 as libc::c_int as usize];
    mm[5 as libc::c_int as usize] = mm[6 as libc::c_int as usize];
    mm[4 as libc::c_int as usize] = mm[5 as libc::c_int as usize];
    /* Confused -- 100% random */
    if (*m_ptr).confused as libc::c_int != 0 ||
           inv as libc::c_int == 1 as libc::c_int &&
               (*m_ptr).target as libc::c_int == 0 as libc::c_int {
        /* Try four "random" directions */
        mm[3 as libc::c_int as usize] = 5 as libc::c_int;
        mm[2 as libc::c_int as usize] = mm[3 as libc::c_int as usize];
        mm[1 as libc::c_int as usize] = mm[2 as libc::c_int as usize];
        mm[0 as libc::c_int as usize] = mm[1 as libc::c_int as usize]
    } else if (*r_ptr).flags1 & 0x80000 as libc::c_int as libc::c_uint != 0 &&
                  (*r_ptr).flags1 & 0x40000 as libc::c_int as libc::c_uint !=
                      0 && Rand_div(100 as libc::c_int) < 75 as libc::c_int {
        /* 75% random movement */
        /* Memorize flags */
        if (*m_ptr).ml != 0 {
            (*r_ptr).r_flags1 |= 0x80000 as libc::c_int as libc::c_uint
        }
        if (*m_ptr).ml != 0 {
            (*r_ptr).r_flags1 |= 0x40000 as libc::c_int as libc::c_uint
        }
        /* Try four "random" directions */
        mm[3 as libc::c_int as usize] = 5 as libc::c_int;
        mm[2 as libc::c_int as usize] = mm[3 as libc::c_int as usize];
        mm[1 as libc::c_int as usize] = mm[2 as libc::c_int as usize];
        mm[0 as libc::c_int as usize] = mm[1 as libc::c_int as usize]
    } else if (*r_ptr).flags1 & 0x80000 as libc::c_int as libc::c_uint != 0 &&
                  Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        /* 50% random movement */
        /* Memorize flags */
        if (*m_ptr).ml != 0 {
            (*r_ptr).r_flags1 |= 0x80000 as libc::c_int as libc::c_uint
        }
        /* Try four "random" directions */
        mm[3 as libc::c_int as usize] = 5 as libc::c_int;
        mm[2 as libc::c_int as usize] = mm[3 as libc::c_int as usize];
        mm[1 as libc::c_int as usize] = mm[2 as libc::c_int as usize];
        mm[0 as libc::c_int as usize] = mm[1 as libc::c_int as usize]
    } else if (*r_ptr).flags1 & 0x40000 as libc::c_int as libc::c_uint != 0 &&
                  Rand_div(100 as libc::c_int) < 25 as libc::c_int {
        /* 25% random movement */
        /* Memorize flags */
        if (*m_ptr).ml != 0 {
            (*r_ptr).r_flags1 |= 0x40000 as libc::c_int as libc::c_uint
        }
        /* Try four "random" directions */
        mm[3 as libc::c_int as usize] = 5 as libc::c_int;
        mm[2 as libc::c_int as usize] = mm[3 as libc::c_int as usize];
        mm[1 as libc::c_int as usize] = mm[2 as libc::c_int as usize];
        mm[0 as libc::c_int as usize] = mm[1 as libc::c_int as usize]
    } else if stupid_monsters != 0 {
        /* Normal movement */
        /* Logical moves */
        get_moves(m_idx, mm.as_mut_ptr());
    } else if get_moves(m_idx, mm.as_mut_ptr()) == 0 { return }
    /* Logical moves, may do nothing */
    /* Paranoia -- quest code could delete it */
    if (*c_ptr).m_idx == 0 { return }
    /* Assume nothing */
    do_turn = 0 as libc::c_int as bool_;
    do_move = 0 as libc::c_int as bool_;
    do_view = 0 as libc::c_int as bool_;
    /* Assume nothing */
    did_open_door = 0 as libc::c_int as bool_;
    did_bash_door = 0 as libc::c_int as bool_;
    did_take_item = 0 as libc::c_int as bool_;
    did_kill_item = 0 as libc::c_int as bool_;
    did_move_body = 0 as libc::c_int as bool_;
    did_kill_body = 0 as libc::c_int as bool_;
    did_pass_wall = 0 as libc::c_int as bool_;
    did_kill_wall = 0 as libc::c_int as bool_;
    /* Take a zero-terminated array of "directions" */
    i = 0 as libc::c_int;
    while mm[i as usize] != 0 {
        /* Get the direction */
        d = mm[i as usize];
        /* Hack -- allow "randomized" motion */
        if d == 5 as libc::c_int {
            d = ddd[Rand_div(8 as libc::c_int) as usize] as libc::c_int
        }
        /* Get the destination */
        ny = oy + ddy[d as usize] as libc::c_int;
        nx = ox + ddx[d as usize] as libc::c_int;
        /* Access that cave grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(ny as isize)).offset(nx as isize)
                as *mut cave_type;
        /* Access that cave grid's contents */
        y_ptr =
            &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
        /* Floor is open? */
        if (*f_info.offset((*cave[ny as usize].offset(nx as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[ny as usize].offset(nx as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int {
            /* Go ahead and move */
            do_move = 1 as libc::c_int as bool_
        } else if (*c_ptr).feat as libc::c_int == 0xaf as libc::c_int {
            /* Floor is trapped? */
            /* Go ahead and move */
            do_move = 1 as libc::c_int as bool_
        }
        /* Hack -- check for Glyph of Warding */
        if (*c_ptr).feat as libc::c_int == 0x3 as libc::c_int &&
               (*r_ptr).flags1 & 0x10000 as libc::c_int as libc::c_uint == 0 {
            /* Assume no move allowed */
            do_move = 0 as libc::c_int as bool_;
            /* Break the ward */
            if (Rand_div(550 as libc::c_int) + 1 as libc::c_int) <
                   (*m_ptr).level as libc::c_int {
                /* Describe observable breakage */
                if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int != 0 {
                    msg_print(b"The rune of protection is broken!\x00" as
                                  *const u8 as *const libc::c_char);
                }
                /* Forget the rune */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int & !(0x1 as libc::c_int)) as
                        u16b;
                /* Break the rune */
                place_floor_convert_glass(ny, nx);
                /* Allow movement */
                do_move = 1 as libc::c_int as bool_
            }
        } else if (*cave[ny as usize].offset(nx as isize)).feat as libc::c_int
                      == 0x60 as libc::c_int &&
                      (*r_ptr).flags9 & 0x200 as libc::c_int as libc::c_uint
                          != 0 {
            do_move = 1 as libc::c_int as bool_;
            /* Hack -- trees are obstacle */
            /* Forget the tree */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int & !(0x1 as libc::c_int)) as
                    u16b;
            /* Notice */
            cave_set_feat(ny, nx, 0x59 as libc::c_int);
        } else if ny == (*p_ptr).py as libc::c_int &&
                      nx == (*p_ptr).px as libc::c_int {
            do_move = 1 as libc::c_int as bool_
        } else if (*c_ptr).m_idx != 0 {
            /* Hack -- player 'in' wall */
            /* Possibly a monster to attack */
            do_move = 1 as libc::c_int as bool_
        } else if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                        libc::c_long & 0x40 as libc::c_long != 0) {
            /* Permanent wall */
            /* Some monsters can fly */
            if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long
                   & 0x4 as libc::c_long != 0 &&
                   (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint != 0 {
                /* Pass through walls/doors/rubble */
                do_move = 1 as libc::c_int as bool_
            } else if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                          libc::c_long & 0x80 as libc::c_long != 0 &&
                          (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint
                              != 0 {
                /* Some monsters can fly */
                /* Pass through trees/... */
                do_move = 1 as libc::c_int as bool_
            } else if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                          libc::c_long & 0x8 as libc::c_long != 0 &&
                          (*r_ptr).flags2 &
                              0x40000 as libc::c_int as libc::c_uint != 0 {
                /* Monster moves through walls (and doors) */
                /* Pass through walls/doors/rubble */
                do_move = 1 as libc::c_int as bool_;
                /* Monster went through a wall */
                did_pass_wall = 1 as libc::c_int as bool_
            } else if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                          libc::c_long & 0x8 as libc::c_long != 0 &&
                          (*r_ptr).flags2 &
                              0x80000 as libc::c_int as libc::c_uint != 0 {
                /* Monster destroys walls (and doors) */
                /* Eat through walls/doors/rubble */
                do_move = 1 as libc::c_int as bool_;
                /* Monster destroyed a wall */
                did_kill_wall = 1 as libc::c_int as bool_;
                if Rand_div(20 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    msg_print(b"There is a grinding sound.\x00" as *const u8
                                  as *const libc::c_char);
                }
                /* Forget the wall */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int & !(0x1 as libc::c_int)) as
                        u16b;
                /* Notice */
                cave_set_feat(ny, nx, 0x1 as libc::c_int);
                /* Note changes to viewable region */
                if (*cave[ny as usize].offset(nx as isize)).info as
                       libc::c_int & 0x20 as libc::c_int != 0 as libc::c_int {
                    do_view = 1 as libc::c_int as bool_
                }
            } else if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                          libc::c_long & 0x8 as libc::c_long != 0 &&
                          (*r_ptr).flags2 &
                              0x40000 as libc::c_int as libc::c_uint != 0 {
                /* Monster moves through walls (and doors) */
                /* Pass through walls/doors/rubble */
                do_move = 1 as libc::c_int as bool_;
                /* Monster went through a wall */
                did_pass_wall = 1 as libc::c_int as bool_
            } else if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                          libc::c_long & 0x10000 as libc::c_long != 0 &&
                          (*r_ptr).flags7 &
                              0x40 as libc::c_int as libc::c_uint != 0 {
                /* Monster moves through webs */
                /* Pass through webs */
                do_move = 1 as libc::c_int as bool_
            } else if (*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
                          (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int
                          ||
                          (*c_ptr).feat as libc::c_int == 0x30 as libc::c_int
             {
                let mut may_bash: bool_ = 1 as libc::c_int as bool_;
                /* Handle doors and secret doors */
                /* Take a turn */
                do_turn = 1 as libc::c_int as bool_;
                if (*r_ptr).flags2 & 0x10000 as libc::c_int as libc::c_uint !=
                       0 &&
                       (is_friend(m_ptr) <= 0 as libc::c_int ||
                            (*p_ptr).pet_open_doors as libc::c_int != 0) {
                    /* Closed doors and secret doors */
                    if (*c_ptr).feat as libc::c_int == 0x20 as libc::c_int ||
                           (*c_ptr).feat as libc::c_int == 0x30 as libc::c_int
                       {
                        /* The door is open */
                        did_open_door = 1 as libc::c_int as bool_;
                        /* Do not bash the door */
                        may_bash = 0 as libc::c_int as bool_
                    } else if ((*c_ptr).feat as libc::c_int) <
                                  0x20 as libc::c_int + 0x8 as libc::c_int {
                        let mut k: libc::c_int = 0;
                        /* Locked doors (not jammed) */
                        /* Door power */
                        k =
                            (*c_ptr).feat as libc::c_int - 0x20 as libc::c_int
                                & 0x7 as libc::c_int;
                        /* Try to unlock it XXX XXX XXX */
                        if Rand_div((*m_ptr).hp / 10 as libc::c_int) > k {
                            /* Unlock the door */
                            cave_set_feat(ny, nx,
                                          0x20 as libc::c_int +
                                              0 as libc::c_int);
                            /* Do not bash the door */
                            may_bash = 0 as libc::c_int as bool_
                        }
                    }
                }
                /* Stuck doors -- attempt to bash them down if allowed */
                if may_bash as libc::c_int != 0 &&
                       (*r_ptr).flags2 &
                           0x20000 as libc::c_int as libc::c_uint != 0 &&
                       (is_friend(m_ptr) <= 0 as libc::c_int ||
                            (*p_ptr).pet_open_doors as libc::c_int != 0) {
                    let mut k_0: libc::c_int = 0;
                    /* Door power */
                    k_0 =
                        (*c_ptr).feat as libc::c_int - 0x20 as libc::c_int &
                            0x7 as libc::c_int;
                    /* Attempt to Bash XXX XXX XXX */
                    if Rand_div((*m_ptr).hp / 10 as libc::c_int) > k_0 {
                        /* Message */
                        msg_print(b"You hear a door burst open!\x00" as
                                      *const u8 as *const libc::c_char);
                        /* Disturb (sometimes) */
                        if disturb_minor != 0 {
                            disturb(0 as libc::c_int, 0 as libc::c_int);
                        }
                        /* The door was bashed open */
                        did_bash_door = 1 as libc::c_int as bool_;
                        /* Hack -- fall into doorway */
                        do_move = 1 as libc::c_int as bool_
                    }
                }
                /* Deal with doors in the way */
                if did_open_door as libc::c_int != 0 ||
                       did_bash_door as libc::c_int != 0 {
                    /* It's no longer hidden */
                    (*cave[ny as usize].offset(nx as isize)).mimic =
                        0 as libc::c_int as byte_hack;
                    /* Break down the door */
                    if did_bash_door as libc::c_int != 0 &&
                           Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                        cave_set_feat(ny, nx, 0x5 as libc::c_int);
                    } else {
                        /* Open the door */
                        cave_set_feat(ny, nx, 0x4 as libc::c_int);
                    }
                    /* Handle viewable doors */
                    if (*cave[ny as usize].offset(nx as isize)).info as
                           libc::c_int & 0x20 as libc::c_int !=
                           0 as libc::c_int {
                        do_view = 1 as libc::c_int as bool_
                    }
                }
            } else if do_move as libc::c_int != 0 &&
                          (*c_ptr).feat as libc::c_int == 0x40 as libc::c_int
                          &&
                          (*r_ptr).flags1 &
                              0x10000 as libc::c_int as libc::c_uint == 0 {
                /* Assume no move allowed */
                do_move = 0 as libc::c_int as bool_;
                /* Break the ward */
                if (Rand_div(99 as libc::c_int) + 1 as libc::c_int) <
                       (*m_ptr).level as libc::c_int {
                    /* Describe observable breakage */
                    if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int != 0
                       {
                        if ny == (*p_ptr).py as libc::c_int &&
                               nx == (*p_ptr).px as libc::c_int {
                            msg_print(b"The rune explodes!\x00" as *const u8
                                          as *const libc::c_char);
                            fire_ball(26 as libc::c_int, 0 as libc::c_int,
                                      2 as libc::c_int *
                                          ((*p_ptr).lev as libc::c_int /
                                               2 as libc::c_int +
                                               damroll(7 as libc::c_int as
                                                           s16b,
                                                       7 as libc::c_int as
                                                           s16b)),
                                      2 as libc::c_int);
                        } else {
                            msg_print(b"An explosive rune was disarmed.\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                        }
                    }
                    /* Forget the rune */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int & !(0x1 as libc::c_int))
                            as u16b;
                    /* Break the rune */
                    place_floor_convert_glass(ny, nx);
                    /* Allow movement */
                    do_move = 1 as libc::c_int as bool_
                }
            } else if (*cave[ny as usize].offset(nx as isize)).feat as
                          libc::c_int == 0xa0 as libc::c_int {
                nx =
                    (*cave[ny as usize].offset(nx as isize)).special as
                        libc::c_int & 255 as libc::c_int;
                ny =
                    (*cave[ny as usize].offset(nx as isize)).special as
                        libc::c_int >> 8 as libc::c_int;
                get_pos_player(10 as libc::c_int, &mut ny, &mut nx);
                /* Hack -- the Between teleport the monsters too */
                /* Access that cave grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(ny as
                                                         isize)).offset(nx as
                                                                            isize)
                        as *mut cave_type;
                /* Access that cave grid's contents */
                y_ptr =
                    &mut *m_list.offset((*c_ptr).m_idx as isize) as
                        *mut monster_type;
                if (*r_ptr).flags3 & 0x80000 as libc::c_int as libc::c_uint ==
                       0 {
                    if (*m_ptr).hp -
                           distance(ny, nx, oy, ox) * 2 as libc::c_int <=
                           0 as libc::c_int {
                        ny = oy + ddy[d as usize] as libc::c_int;
                        nx = ox + ddx[d as usize] as libc::c_int;
                        do_move = 0 as libc::c_int as bool_
                    } else {
                        (*m_ptr).hp -=
                            distance(ny, nx, oy, ox) * 2 as libc::c_int;
                        do_move = 1 as libc::c_int as bool_
                    }
                } else { do_move = 1 as libc::c_int as bool_ }
            }
        }
        /* Execute the inscription -- MEGA HACK -- */
        if (*c_ptr).inscription as libc::c_int != 0 &&
               (*c_ptr).inscription as libc::c_int != 6 as libc::c_int {
            if inscription_info[(*c_ptr).inscription as usize].when as
                   libc::c_int & 0x4 as libc::c_int != 0 {
                let mut t: bool_ = 0;
                t =
                    execute_inscription((*c_ptr).inscription as byte_hack,
                                        ny as byte_hack, nx as byte_hack);
                if t == 0 && do_move as libc::c_int != 0 {
                    /* Hack -- attack the player even if on the inscription */
                    if ny == (*p_ptr).py as libc::c_int &&
                           nx == (*p_ptr).px as libc::c_int {
                        do_move = 1 as libc::c_int as bool_
                    } else { do_move = 0 as libc::c_int as bool_ }
                }
            }
        }
        /* Some monsters never attack */
        if do_move as libc::c_int != 0 && ny == (*p_ptr).py as libc::c_int &&
               nx == (*p_ptr).px as libc::c_int &&
               (*r_ptr).flags1 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            /* Do not move */
            do_move = 0 as libc::c_int as bool_
        }
        /* The player is in the way.  Attack him. */
        if do_move as libc::c_int != 0 && ny == (*p_ptr).py as libc::c_int &&
               nx == (*p_ptr).px as libc::c_int {
            /* Do the attack */
            make_attack_normal(m_idx, 1 as libc::c_int as byte_hack);
            /* Do not move */
            do_move = 0 as libc::c_int as bool_;
            /* Took a turn */
            do_turn = 1 as libc::c_int as bool_
        }
        if (*cave[ny as usize].offset(nx as isize)).feat as libc::c_int >=
               0x41 as libc::c_int &&
               (*cave[ny as usize].offset(nx as isize)).feat as libc::c_int <=
                   0x49 as libc::c_int &&
               do_turn as libc::c_int == 0 as libc::c_int {
            do_move = 0 as libc::c_int as bool_
        }
        /* A monster is in the way */
        if do_move as libc::c_int != 0 && (*c_ptr).m_idx as libc::c_int != 0 {
            let mut z_ptr: *mut monster_race =
                if !(*y_ptr).sr_ptr.is_null() {
                    (*y_ptr).sr_ptr
                } else {
                    race_info_idx((*y_ptr).r_idx as libc::c_int,
                                  (*y_ptr).ego as libc::c_int)
                };
            let mut m2_ptr: *mut monster_type =
                &mut *m_list.offset((*c_ptr).m_idx as isize) as
                    *mut monster_type;
            /* Assume no movement */
            do_move = 0 as libc::c_int as bool_;
            /* Kill weaker monsters */
            if (*r_ptr).flags2 & 0x200000 as libc::c_int as libc::c_uint != 0
                   && (*r_ptr).mexp > (*z_ptr).mexp &&
                   ((*f_info.offset((*cave[ny as
                                               usize].offset(nx as
                                                                 isize)).feat
                                        as isize)).flags1 as libc::c_long &
                        0x10 as libc::c_long != 0 &&
                        (*cave[ny as usize].offset(nx as isize)).feat as
                            libc::c_int != 0xaf as libc::c_int) &&
                   !(is_friend(m_ptr) > 0 as libc::c_int &&
                         is_friend(m2_ptr) > 0 as libc::c_int) &&
                   (*z_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0
                   &&
                   (*m2_ptr).mflag &
                       (0x2 as libc::c_int | 0x200 as libc::c_int) == 0 &&
                   is_friend(m2_ptr) <= 0 as libc::c_int {
                /* Allow movement */
                do_move = 1 as libc::c_int as bool_;
                /* Monster ate another monster */
                did_kill_body = 1 as libc::c_int as bool_;
                /* XXX XXX XXX Message */
                /* Kill the monster */
                delete_monster(ny, nx);
                /* Hack -- get the empty monster */
                y_ptr =
                    &mut *m_list.offset((*c_ptr).m_idx as isize) as
                        *mut monster_type
            } else if is_enemy(m_ptr, m2_ptr) as libc::c_int != 0 ||
                          (*m_ptr).confused as libc::c_int != 0 {
                do_move = 0 as libc::c_int as bool_;
                /* Attack 'enemies' */
                /* attack */
                if (*m2_ptr).r_idx as libc::c_int != 0 &&
                       (*m2_ptr).hp >= 0 as libc::c_int {
                    hack_message_pain_may_silent = 1 as libc::c_int as bool_;
                    if monst_attack_monst(m_idx,
                                          (*c_ptr).m_idx as libc::c_int) != 0
                       {
                        hack_message_pain_may_silent =
                            0 as libc::c_int as bool_;
                        return
                    }
                    hack_message_pain_may_silent = 0 as libc::c_int as bool_
                }
            } else if (*r_ptr).flags2 &
                          0x100000 as libc::c_int as libc::c_uint != 0 &&
                          (*r_ptr).mexp > (*z_ptr).mexp &&
                          ((*f_info.offset((*cave[ny as
                                                      usize].offset(nx as
                                                                        isize)).feat
                                               as isize)).flags1 as
                               libc::c_long & 0x10 as libc::c_long != 0 &&
                               (*cave[ny as usize].offset(nx as isize)).feat
                                   as libc::c_int != 0xaf as libc::c_int) &&
                          ((*f_info.offset((*cave[(*m_ptr).fy as
                                                      usize].offset((*m_ptr).fx
                                                                        as
                                                                        isize)).feat
                                               as isize)).flags1 as
                               libc::c_long & 0x10 as libc::c_long != 0 &&
                               (*cave[(*m_ptr).fy as
                                          usize].offset((*m_ptr).fx as
                                                            isize)).feat as
                                   libc::c_int != 0xaf as libc::c_int) {
                /* Push past weaker monsters (unless leaving a wall) */
                /* Allow movement */
                do_move = 1 as libc::c_int as bool_;
                /* XXX XXX XXX Message */
                did_move_body = 1 as libc::c_int as bool_
            }
        }
        /* Monster pushed past another monster */
        /*
		 * Check if monster can cross terrain
		 * This is checked after the normal attacks
		 * to allow monsters to attack an enemy,
		 * even if it can't enter the terrain.
		 */
        if do_move as libc::c_int != 0 &&
               monster_can_cross_terrain((*c_ptr).feat, r_ptr) == 0 {
            /* Assume no move allowed */
            do_move = 0 as libc::c_int as bool_
        }
        /* Some monsters never move */
        if do_move as libc::c_int != 0 &&
               (*r_ptr).flags1 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            /* Hack -- memorize lack of attacks */
			/* if (m_ptr->ml) r_ptr->r_flags1 |= (RF1_NEVER_MOVE); */
            /* Do not move */
            do_move = 0 as libc::c_int as bool_
        }
        /* Creature has been allowed move */
        if do_move != 0 {
            let mut this_o_idx: s16b = 0;
            let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
            /* Take a turn */
            do_turn = 1 as libc::c_int as bool_;
            /* Hack -- Update the old location */
            (*cave[oy as usize].offset(ox as isize)).m_idx = (*c_ptr).m_idx;
            /* Mega-Hack -- move the old monster, if any */
            if (*c_ptr).m_idx != 0 {
                /* Move the old monster */
                (*y_ptr).fy = oy as byte_hack;
                (*y_ptr).fx = ox as byte_hack;
                /* Update the old monster */
                update_mon((*c_ptr).m_idx as libc::c_int,
                           1 as libc::c_int as bool_);
                /* Wake up the moved monster */
                (*m_list.offset((*c_ptr).m_idx as isize)).csleep =
                    0 as libc::c_int as s16b;
                /*
				 * Update monster light -- I'm too lazy to check flags
				 * here, and those ego monster_race functions aren't
				 * re-entrant XXX XXX XXX
				 */
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long |
                         0x200000 as libc::c_long) as u32b
            }
            /* Hack -- Update the new location */
            (*c_ptr).m_idx = m_idx as s16b;
            /* Move the monster */
            (*m_ptr).fy = ny as byte_hack;
            (*m_ptr).fx = nx as byte_hack;
            /* Update the monster */
            update_mon(m_idx, 1 as libc::c_int as bool_);
            /* Redraw the old grid */
            lite_spot(oy, ox);
            /* Redraw the new grid */
            lite_spot(ny, nx);
            /* Execute the inscription -- MEGA HACK -- */
            if (*c_ptr).inscription as libc::c_int == 6 as libc::c_int {
                if inscription_info[(*c_ptr).inscription as usize].when as
                       libc::c_int & 0x4 as libc::c_int != 0 {
                    execute_inscription((*c_ptr).inscription as byte_hack,
                                        ny as byte_hack, nx as byte_hack);
                }
            }
            /* Possible disturb */
            if (*m_ptr).ml as libc::c_int != 0 &&
                   (disturb_move as libc::c_int != 0 ||
                        (*m_ptr).mflag & 0x1 as libc::c_int != 0 &&
                            disturb_near as libc::c_int != 0) {
                /* Disturb */
                if is_friend(m_ptr) < 0 as libc::c_int ||
                       disturb_pets as libc::c_int != 0 {
                    disturb(0 as libc::c_int, 0 as libc::c_int);
                }
            }
            /* Check for monster trap */
            if (*c_ptr).feat as libc::c_int == 0xaf as libc::c_int {
                if mon_hit_trap(m_idx) != 0 { return }
            } else {
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
                    /* Skip gold */
                    if !((*o_ptr).tval as libc::c_int == 100 as libc::c_int) {
                        /* Incarnate ? */
                        if (*o_ptr).tval as libc::c_int == 9 as libc::c_int &&
                               (*r_ptr).flags7 &
                                   0x200 as libc::c_int as libc::c_uint != 0
                               &&
                               ((*o_ptr).sval as libc::c_int ==
                                    1 as libc::c_int ||
                                    (*o_ptr).sval as libc::c_int ==
                                        2 as libc::c_int) {
                            if ai_possessor(m_idx, this_o_idx as libc::c_int)
                                   != 0 {
                                return
                            }
                        }
                        /* Take or Kill objects on the floor */
					/* rr9: Pets will no longer pick up/destroy items */
                        if ((*r_ptr).flags2 &
                                0x400000 as libc::c_int as libc::c_uint != 0
                                &&
                                (is_friend(m_ptr) <= 0 as libc::c_int ||
                                     (*p_ptr).pet_pickup_items as libc::c_int
                                         != 0) ||
                                (*r_ptr).flags2 &
                                    0x800000 as libc::c_int as libc::c_uint !=
                                    0) && is_friend(m_ptr) <= 0 as libc::c_int
                           {
                            let mut f1: u32b = 0;
                            let mut f2: u32b = 0;
                            let mut f3: u32b = 0;
                            let mut f4: u32b = 0;
                            let mut f5: u32b = 0;
                            let mut esp: u32b = 0;
                            let mut flg3: u32b = 0 as libc::c_long as u32b;
                            let mut m_name_7: [libc::c_char; 80] = [0; 80];
                            let mut o_name: [libc::c_char; 80] = [0; 80];
                            /* Extract some flags */
                            object_flags(o_ptr, &mut f1, &mut f2, &mut f3,
                                         &mut f4, &mut f5, &mut esp);
                            /* Acquire the object name */
                            object_desc(o_name.as_mut_ptr(), o_ptr,
                                        1 as libc::c_int, 3 as libc::c_int);
                            /* Acquire the monster name */
                            monster_desc(m_name_7.as_mut_ptr(), m_ptr,
                                         0x4 as libc::c_int);
                            /* React to objects that hurt the monster */
                            if f5 as libc::c_long & 0x8 as libc::c_long != 0 {
                                flg3 |= 0x10 as libc::c_int as libc::c_uint
                            }
                            if f5 as libc::c_long & 0x10 as libc::c_long != 0
                               {
                                flg3 |= 0x20 as libc::c_int as libc::c_uint
                            }
                            if f1 as libc::c_long & 0x800000 as libc::c_long
                                   != 0 {
                                flg3 |= 0x8 as libc::c_int as libc::c_uint
                            }
                            if f1 as libc::c_long & 0x200000 as libc::c_long
                                   != 0 {
                                flg3 |= 0x2 as libc::c_int as libc::c_uint
                            }
                            if f1 as libc::c_long & 0x400000 as libc::c_long
                                   != 0 {
                                flg3 |= 0x4 as libc::c_int as libc::c_uint
                            }
                            if f1 as libc::c_long & 0x100000 as libc::c_long
                                   != 0 {
                                flg3 |= 0x1 as libc::c_int as libc::c_uint
                            }
                            if f1 as libc::c_long & 0x80000 as libc::c_long !=
                                   0 {
                                flg3 |= 0x10 as libc::c_int as libc::c_uint
                            }
                            if f1 as libc::c_long & 0x40000 as libc::c_long !=
                                   0 {
                                flg3 |= 0x20 as libc::c_int as libc::c_uint
                            }
                            if f1 as libc::c_long & 0x10000 as libc::c_long !=
                                   0 {
                                flg3 |= 0x80 as libc::c_int as libc::c_uint
                            }
                            if f1 as libc::c_long & 0x20000 as libc::c_long !=
                                   0 {
                                flg3 |= 0x40 as libc::c_int as libc::c_uint
                            }
                            /* The object cannot be picked up by the monster */
                            if (*o_ptr).tval as libc::c_int ==
                                   102 as libc::c_int ||
                                   (if (*o_ptr).name1 as libc::c_int != 0 {
                                        1 as libc::c_int
                                    } else { 0 as libc::c_int }) != 0 ||
                                   (if (*o_ptr).art_name as libc::c_int != 0 {
                                        1 as libc::c_int
                                    } else { 0 as libc::c_int }) != 0 ||
                                   (if (*k_info.offset((*o_ptr).k_idx as
                                                           isize)).flags3 as
                                           libc::c_long &
                                           0x8000 as libc::c_long != 0 {
                                        1 as libc::c_int
                                    } else { 0 as libc::c_int }) != 0 ||
                                   (*r_ptr).flags3 & flg3 != 0 ||
                                   (*o_ptr).art_name as libc::c_int != 0 {
                                /* Only give a message for "take_item" */
                                if (*r_ptr).flags2 &
                                       0x400000 as libc::c_int as libc::c_uint
                                       != 0 {
                                    /* Take note */
                                    did_take_item = 1 as libc::c_int as bool_;
                                    /* Describe observable situations */
                                    if (*m_ptr).ml as libc::c_int != 0 &&
                                           (*cave[ny as
                                                      usize].offset(nx as
                                                                        isize)).info
                                               as libc::c_int &
                                               0x20 as libc::c_int !=
                                               0 as libc::c_int {
                                        /* Dump a message */
                                        msg_format(b"%^s tries to pick up %s, but fails.\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   m_name_7.as_mut_ptr(),
                                                   o_name.as_mut_ptr());
                                    }
                                }
                            } else if (*r_ptr).flags2 &
                                          0x400000 as libc::c_int as
                                              libc::c_uint != 0 {
                                /* Pick up the item */
                                /* Take note */
                                did_take_item = 1 as libc::c_int as bool_;
                                /* Describe observable situations */
                                if (*cave[ny as
                                              usize].offset(nx as isize)).info
                                       as libc::c_int & 0x20 as libc::c_int !=
                                       0 as libc::c_int {
                                    /* Dump a message */
                                    msg_format(b"%^s picks up %s.\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               m_name_7.as_mut_ptr(),
                                               o_name.as_mut_ptr());
                                }
                                /* Option */
                                if testing_carry != 0 {
                                    /* Excise the object */
                                    excise_object_idx(this_o_idx as
                                                          libc::c_int);
                                    /* Forget mark */
                                    (*o_ptr).marked =
                                        0 as libc::c_int as byte_hack;
                                    /* Forget location */
                                    (*o_ptr).ix =
                                        0 as libc::c_int as byte_hack;
                                    (*o_ptr).iy = (*o_ptr).ix;
                                    /* Memorize monster */
                                    (*o_ptr).held_m_idx = m_idx as s16b;
                                    /* Build a stack */
                                    (*o_ptr).next_o_idx = (*m_ptr).hold_o_idx;
                                    /* Carry object */
                                    (*m_ptr).hold_o_idx = this_o_idx
                                } else {
                                    /* Nope */
                                    /* Delete the object */
                                    delete_object_idx(this_o_idx as
                                                          libc::c_int);
                                }
                            } else {
                                /* Destroy the item */
                                /* Take note */
                                did_kill_item = 1 as libc::c_int as bool_;
                                if (*cave[ny as
                                              usize].offset(nx as isize)).info
                                       as libc::c_int & 0x20 as libc::c_int !=
                                       0 as libc::c_int {
                                    /* Dump a message */
                                    msg_format(b"%^s crushes %s.\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               m_name_7.as_mut_ptr(),
                                               o_name.as_mut_ptr());
                                }
                                delete_object_idx(this_o_idx as libc::c_int);
                            }
                        }
                    }
                    this_o_idx = next_o_idx
                }
            }
            /* Describe observable situations */
            /* Delete the object */
            /* Update monster light */
            if (*r_ptr).flags9 & 0x4 as libc::c_int as libc::c_uint != 0 {
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long |
                         0x200000 as libc::c_long) as u32b
            }
        }
        /* Stop when done */
        if do_turn != 0 { break ; }
        i += 1
    }
    /* If we haven't done anything, try casting a spell again */
    if do_turn == 0 && do_move == 0 && (*m_ptr).monfear == 0 &&
           stupid_monsters == 0 && is_friend(m_ptr) < 0 as libc::c_int {
        /* Cast spell */
        if make_attack_spell(m_idx) != 0 { return }
    }
    /* Notice changes in view */
    if do_view != 0 {
        /* Update some things */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                      0x1000000 as libc::c_long | 0x200000 as libc::c_long))
                as u32b
    }
    /* Learn things from observable monster */
    if (*m_ptr).ml != 0 {
        /* Monster opened a door */
        if did_open_door != 0 {
            (*r_ptr).r_flags2 |= 0x10000 as libc::c_int as libc::c_uint
        }
        /* Monster bashed a door */
        if did_bash_door != 0 {
            (*r_ptr).r_flags2 |= 0x20000 as libc::c_int as libc::c_uint
        }
        /* Monster tried to pick something up */
        if did_take_item != 0 {
            (*r_ptr).r_flags2 |= 0x400000 as libc::c_int as libc::c_uint
        }
        /* Monster tried to crush something */
        if did_kill_item != 0 {
            (*r_ptr).r_flags2 |= 0x800000 as libc::c_int as libc::c_uint
        }
        /* Monster pushed past another monster */
        if did_move_body != 0 {
            (*r_ptr).r_flags2 |= 0x100000 as libc::c_int as libc::c_uint
        }
        /* Monster ate another monster */
        if did_kill_body != 0 {
            (*r_ptr).r_flags2 |= 0x200000 as libc::c_int as libc::c_uint
        }
        /* Monster passed through a wall */
        if did_pass_wall != 0 {
            (*r_ptr).r_flags2 |= 0x40000 as libc::c_int as libc::c_uint
        }
        /* Monster destroyed a wall */
        if did_kill_wall != 0 {
            (*r_ptr).r_flags2 |= 0x80000 as libc::c_int as libc::c_uint
        }
    }
    /* Hack -- get "bold" if out of options */
    if do_turn == 0 && do_move == 0 && (*m_ptr).monfear as libc::c_int != 0 {
        /* No longer afraid */
        (*m_ptr).monfear = 0 as libc::c_int as byte_hack;
        /* XXX XXX XXX Actually do something now (?) */
        if (*m_ptr).ml != 0 {
            let mut m_name_8: [libc::c_char; 80] = [0; 80];
            /* Message if seen */
            /* Acquire the monster name */
            monster_desc(m_name_8.as_mut_ptr(), m_ptr, 0 as libc::c_int);
            /* Dump a message */
            msg_format(b"%^s turns to fight!\x00" as *const u8 as
                           *const libc::c_char, m_name_8.as_mut_ptr());
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn summon_maint(mut m_idx: libc::c_int) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    /* Can you pay? */
    if (*p_ptr).maintain_sum.wrapping_div(10000 as libc::c_int as
                                              libc::c_uint) as s32b >
           (*p_ptr).csp as libc::c_int {
        let mut m_name: [libc::c_char; 80] = [0; 80];
        monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
        msg_format(b"You lose control of %s.\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr());
        /* Well, then, I guess I'm dead. */
        delete_monster_idx(m_idx);
    } else {
        let mut cl: s32b = 0;
        let mut ml: s32b = 0;
        let mut floor: s32b = 0;
        let mut cost: s32b = 0;
        cl =
            get_skill_scale(44 as libc::c_int, 100 as libc::c_int as u32b) as
                s32b;
        ml = (*m_ptr).level as libc::c_int * 10000 as libc::c_int;
        /* Floor = 19 * ml / 990 + 8 / 199
		   This gives a floor of 0.1 at level 1 and a floor of 2 at level 100

		   Since ml is multiplied by 10000 already, we multiply the 8/199 too
		   */
        floor =
            ml * 19 as libc::c_int / 990 as libc::c_int +
                80000 as libc::c_int / 199 as libc::c_int;
        cost = (ml / cl - 10000 as libc::c_int) / 4 as libc::c_int;
        if cost < floor { cost = floor }
        /* Well, then I'll take my wages from you. */
        (*p_ptr).maintain_sum =
            ((*p_ptr).maintain_sum as
                 libc::c_uint).wrapping_add(cost as libc::c_uint) as u32b as
                u32b
    };
}
/*
 * Process all the "live" monsters, once per game turn.
 *
 * During each game turn, we scan through the list of all the "live" monsters,
 * (backwards, so we can excise any "freshly dead" monsters), energizing each
 * monster, and allowing fully energized monsters to move, attack, pass, etc.
 *
 * Note that monsters can never move in the monster array (except when the
 * "compact_monsters()" function is called by "dungeon()" or "save_player()").
 *
 * This function is responsible for at least half of the processor time
 * on a normal system with a "normal" amount of monsters and a player doing
 * normal things.
 *
 * When the player is resting, virtually 90% of the processor time is spent
 * in this function, and its children, "process_monster()" and "make_move()".
 *
 * Most of the rest of the time is spent in "update_view()" and "lite_spot()",
 * especially when the player is running.
 *
 * Note the special "MFLAG_BORN" flag, which allows us to ignore "fresh"
 * monsters while they are still being "born".  A monster is "fresh" only
 * during the turn in which it is created, and we use the "hack_m_idx" to
 * determine if the monster is yet to be processed during the current turn.
 *
 * Note the special "MFLAG_NICE" flag, which allows the player to get one
 * move before any "nasty" monsters get to use their spell attacks.
 *
 * Note that when the "knowledge" about the currently tracked monster
 * changes (flags, attacks, spells), we induce a redraw of the monster
 * recall window.
 */
#[no_mangle]
pub unsafe extern "C" fn process_monsters() {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut fx: libc::c_int = 0;
    let mut fy: libc::c_int = 0;
    let mut test: bool_ = 0;
    let mut is_frien: bool_ = 0 as libc::c_int as bool_;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut old_monster_race_idx: libc::c_int = 0;
    let mut old_r_flags1: u32b = 0 as libc::c_long as u32b;
    let mut old_r_flags2: u32b = 0 as libc::c_long as u32b;
    let mut old_r_flags3: u32b = 0 as libc::c_long as u32b;
    let mut old_r_flags4: u32b = 0 as libc::c_long as u32b;
    let mut old_r_flags5: u32b = 0 as libc::c_long as u32b;
    let mut old_r_flags6: u32b = 0 as libc::c_long as u32b;
    let mut old_r_blows0: byte_hack = 0 as libc::c_int as byte_hack;
    let mut old_r_blows1: byte_hack = 0 as libc::c_int as byte_hack;
    let mut old_r_blows2: byte_hack = 0 as libc::c_int as byte_hack;
    let mut old_r_blows3: byte_hack = 0 as libc::c_int as byte_hack;
    let mut old_r_cast_inate: byte_hack = 0 as libc::c_int as byte_hack;
    let mut old_r_cast_spell: byte_hack = 0 as libc::c_int as byte_hack;
    /* Check the doppleganger */
    if doppleganger as libc::c_int != 0 &&
           (*r_info.offset((*m_list.offset(doppleganger as isize)).r_idx as
                               isize)).flags9 &
               0x800 as libc::c_int as libc::c_uint == 0 {
        doppleganger = 0 as libc::c_int as s16b
    }
    /* Memorize old race */
    old_monster_race_idx = monster_race_idx as libc::c_int;
    /* Acquire knowledge */
    if monster_race_idx != 0 {
        /* Acquire current monster */
        r_ptr =
            &mut *r_info.offset(monster_race_idx as isize) as
                *mut monster_race;
        /* Memorize flags */
        old_r_flags1 = (*r_ptr).r_flags1;
        old_r_flags2 = (*r_ptr).r_flags2;
        old_r_flags3 = (*r_ptr).r_flags3;
        old_r_flags4 = (*r_ptr).r_flags4;
        old_r_flags5 = (*r_ptr).r_flags5;
        old_r_flags6 = (*r_ptr).r_flags6;
        /* Memorize blows */
        old_r_blows0 = (*r_ptr).r_blows[0 as libc::c_int as usize];
        old_r_blows1 = (*r_ptr).r_blows[1 as libc::c_int as usize];
        old_r_blows2 = (*r_ptr).r_blows[2 as libc::c_int as usize];
        old_r_blows3 = (*r_ptr).r_blows[3 as libc::c_int as usize];
        /* Memorize castings */
        old_r_cast_inate = (*r_ptr).r_cast_inate;
        old_r_cast_spell = (*r_ptr).r_cast_spell
    }
    /* Hack -- calculate the "player noise" */
    noise =
        ((1 as libc::c_long) <<
             30 as libc::c_int - (*p_ptr).skill_stl as libc::c_int) as u32b;
    /* Process the monsters (backwards) */
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        /* Access the monster */
        m_ptr = &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Handle "leaving" */
        if (*p_ptr).leaving != 0 { break ; }
        /* Ignore "dead" monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Calculate "upkeep" for friendly monsters */
            if (*m_ptr).status as libc::c_int == 3 as libc::c_int {
                total_friends += 1;
                total_friend_levels += (*m_ptr).level as libc::c_int
            }
            /* Handle "fresh" monsters */
            if (*m_ptr).mflag & 0x10 as libc::c_int != 0 {
                /* No longer "fresh" */
                (*m_ptr).mflag &= !(0x10 as libc::c_int)
            } else {
                /* Obtain the energy boost */
                e = extract_energy[(*m_ptr).mspeed as usize] as libc::c_int;
                /* Give this monster some energy */
                (*m_ptr).energy =
                    ((*m_ptr).energy as libc::c_int + e) as byte_hack;
                /* Not enough energy to move */
                if !(((*m_ptr).energy as libc::c_int) < 100 as libc::c_int) {
                    /* Use up "some" energy */
                    (*m_ptr).energy =
                        ((*m_ptr).energy as libc::c_int - 100 as libc::c_int)
                            as byte_hack;
                    /* Hack -- Require proximity */
                    if !((*m_ptr).cdis as libc::c_int >= 100 as libc::c_int) {
                        /* Access the race */
                        r_ptr =
                            if !(*m_ptr).sr_ptr.is_null() {
                                (*m_ptr).sr_ptr
                            } else {
                                race_info_idx((*m_ptr).r_idx as libc::c_int,
                                              (*m_ptr).ego as libc::c_int)
                            };
                        /* Access the location */
                        fx = (*m_ptr).fx as libc::c_int;
                        fy = (*m_ptr).fy as libc::c_int;
                        /* Assume no move */
                        test = 0 as libc::c_int as bool_;
                        /* Control monster aint affected by distance */
                        if (*p_ptr).control as libc::c_int == i {
                            test = 1 as libc::c_int as bool_
                        } else if (*m_ptr).mflag & 0x4 as libc::c_int != 0 {
                            test = 1 as libc::c_int as bool_
                        } else if (*m_ptr).cdis as libc::c_int <=
                                      (*r_ptr).aaf as libc::c_int {
                            /* No free upkeep on partial summons just because they're out
		 * of line of sight. */
                            /* Handle "sensing radius" */
                            /* We can "sense" the player */
                            test = 1 as libc::c_int as bool_
                        } else if (*m_ptr).cdis as libc::c_int <=
                                      20 as libc::c_int &&
                                      ((*cave[fy as
                                                  usize].offset(fx as
                                                                    isize)).info
                                           as libc::c_int &
                                           0x20 as libc::c_int !=
                                           0 as libc::c_int ||
                                           (*p_ptr).aggravate as libc::c_int
                                               != 0) {
                            /* Handle "sight" and "aggravation" */
                            /* We can "see" or "feel" the player */
                            test = 1 as libc::c_int as bool_
                        } else if flow_by_sound as libc::c_int != 0 &&
                                      (*cave[(*p_ptr).py as
                                                 usize].offset((*p_ptr).px as
                                                                   isize)).when
                                          as libc::c_int ==
                                          (*cave[fy as
                                                     usize].offset(fx as
                                                                       isize)).when
                                              as libc::c_int &&
                                      ((*cave[fy as
                                                  usize].offset(fx as
                                                                    isize)).cost
                                           as libc::c_int) < 32 as libc::c_int
                                      &&
                                      ((*cave[fy as
                                                  usize].offset(fx as
                                                                    isize)).cost
                                           as libc::c_int) <
                                          (*r_ptr).aaf as libc::c_int {
                            /* Hack -- Monsters can "smell" the player from far away */
		/* Note that most monsters have "aaf" of "20" or so */
                            /* We can "smell" the player */
                            test = 1 as libc::c_int as bool_
                        }
                        /* Running away wont save them ! */
                        if (*m_ptr).poisoned as libc::c_int != 0 ||
                               (*m_ptr).bleeding as libc::c_int != 0 {
                            test = 1 as libc::c_int as bool_
                        }
                        /* Do nothing */
                        if !(test == 0) {
                            /* Save global index */
                            hack_m_idx = i as s16b;
                            if is_friend(m_ptr) > 0 as libc::c_int {
                                is_frien = 1 as libc::c_int as bool_
                            }
                            /* Process the monster */
                            process_monster(i, is_frien);
                            /* Hack -- notice death or departure */
                            if alive == 0 || death as libc::c_int != 0 {
                                break ;
                            }
                            /* If it's still alive and friendly, charge upkeep. */
                            if (*m_ptr).mflag & 0x4 as libc::c_int != 0 {
                                summon_maint(i);
                            }
                            /* Notice leaving */
                            if (*p_ptr).leaving != 0 { break ; }
                        }
                    }
                }
            }
        }
        /* Skip */
        i -= 1
    }
    /* Reset global index */
    hack_m_idx = 0 as libc::c_int as s16b;
    /* Tracking a monster race (the same one we were before) */
    if monster_race_idx as libc::c_int != 0 &&
           monster_race_idx as libc::c_int == old_monster_race_idx {
        /* Acquire monster race */
        r_ptr =
            &mut *r_info.offset(monster_race_idx as isize) as
                *mut monster_race;
        /* Check for knowledge change */
        if old_r_flags1 != (*r_ptr).r_flags1 ||
               old_r_flags2 != (*r_ptr).r_flags2 ||
               old_r_flags3 != (*r_ptr).r_flags3 ||
               old_r_flags4 != (*r_ptr).r_flags4 ||
               old_r_flags5 != (*r_ptr).r_flags5 ||
               old_r_flags6 != (*r_ptr).r_flags6 ||
               old_r_blows0 as libc::c_int !=
                   (*r_ptr).r_blows[0 as libc::c_int as usize] as libc::c_int
               ||
               old_r_blows1 as libc::c_int !=
                   (*r_ptr).r_blows[1 as libc::c_int as usize] as libc::c_int
               ||
               old_r_blows2 as libc::c_int !=
                   (*r_ptr).r_blows[2 as libc::c_int as usize] as libc::c_int
               ||
               old_r_blows3 as libc::c_int !=
                   (*r_ptr).r_blows[3 as libc::c_int as usize] as libc::c_int
               ||
               old_r_cast_inate as libc::c_int !=
                   (*r_ptr).r_cast_inate as libc::c_int ||
               old_r_cast_spell as libc::c_int !=
                   (*r_ptr).r_cast_spell as libc::c_int {
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x100 as libc::c_long) as
                    u32b
        }
    };
}
