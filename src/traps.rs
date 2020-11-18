use ::libc;
extern "C" {
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut adj_dex_safe: [byte_hack; 0];
    #[no_mangle]
    static mut sex_info: [player_sex; 3];
    #[no_mangle]
    static mut energy_use: s32b;
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
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut sp_ptr: *mut player_sex;
    #[no_mangle]
    static mut rp_ptr: *mut player_race;
    #[no_mangle]
    static mut rmp_ptr: *mut player_race_mod;
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
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut c_name: *mut libc::c_char;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut t_name: *mut libc::c_char;
    #[no_mangle]
    static mut item_tester_tval: byte_hack;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut max_t_idx: u16b;
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut deity_info: *mut deity_type;
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cave_valid_bold(y: libc::c_int, x: libc::c_int) -> bool_;
    #[no_mangle]
    fn no_lite() -> bool_;
    #[no_mangle]
    fn note_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn place_floor_convert_glass(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn disturb(stop_search: libc::c_int, flush_output: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn attack_special(m_ptr: *mut monster_type, special: s32b,
                      dam: libc::c_int);
    #[no_mangle]
    fn test_hit_fire(chance: libc::c_int, ac: libc::c_int, vis: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn critical_shot(weight: libc::c_int, plus: libc::c_int, dam: libc::c_int,
                     skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn tot_dam_aux(o_ptr: *mut object_type, tdam: libc::c_int,
                   m_ptr: *mut monster_type, special: *mut s32b) -> s16b;
    #[no_mangle]
    fn breakage_chance(o_ptr: *mut object_type) -> libc::c_int;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn autosave_checkpoint();
    #[no_mangle]
    fn monster_check_experience(m_idx: libc::c_int, silent: bool_);
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn delete_monster_idx(i: libc::c_int);
    #[no_mangle]
    fn delete_monster(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn update_mon(m_idx: libc::c_int, full: bool_);
    #[no_mangle]
    fn summon_specific(y1: libc::c_int, x1: libc::c_int, lev: libc::c_int,
                       type_0: libc::c_int) -> bool_;
    #[no_mangle]
    fn message_pain(m_idx: libc::c_int, dam: libc::c_int);
    #[no_mangle]
    fn inc_stack_size_ex(item: libc::c_int, delta: libc::c_int,
                         opt: optimize_flag, desc: describe_flag);
    #[no_mangle]
    fn takeoff_set(a_idx: s16b, set_idx: s16b) -> bool_;
    #[no_mangle]
    fn wield_set(a_idx: s16b, set_idx: s16b, silent: bool_) -> bool_;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn index_to_label(i: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn wield_slot(o_ptr: *mut object_type) -> s16b;
    #[no_mangle]
    fn inven_item_optimize(item: libc::c_int) -> bool_;
    #[no_mangle]
    fn floor_item_increase(item: libc::c_int, num: libc::c_int);
    #[no_mangle]
    fn floor_item_optimize(item: libc::c_int);
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn delete_object_idx(o_idx: libc::c_int);
    #[no_mangle]
    fn delete_object(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
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
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn acquirement(y1: libc::c_int, x1: libc::c_int, num: libc::c_int,
                   great: bool_, known: bool_);
    #[no_mangle]
    fn pick_trap(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn combine_pack();
    #[no_mangle]
    fn reorder_pack();
    #[no_mangle]
    fn floor_carry(y: libc::c_int, x: libc::c_int, j_ptr: *mut object_type)
     -> s16b;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn project(who: libc::c_int, rad: libc::c_int, y: libc::c_int,
               x: libc::c_int, dam: libc::c_int, typ: libc::c_int,
               flg: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_image(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn inc_piety(god: libc::c_int, amt: s32b);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn set_poisoned(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn redraw_stuff();
    #[no_mangle]
    fn set_afraid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn set_slow(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn aggravate_monsters(who: libc::c_int);
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_blind(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_food(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn dec_stat(stat: libc::c_int, amount: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn lose_exp(amount: s32b);
    #[no_mangle]
    fn teleport_player(dis: libc::c_int);
    #[no_mangle]
    fn earthquake(cy: libc::c_int, cx: libc::c_int, r: libc::c_int);
    #[no_mangle]
    fn curse_armor() -> bool_;
    #[no_mangle]
    fn curse_weapon() -> bool_;
    #[no_mangle]
    fn do_dec_stat(stat: libc::c_int, mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_quantity(prompt: cptr, max: s32b) -> s32b;
    #[no_mangle]
    fn teleport_monster_to(m_idx: libc::c_int, ny: libc::c_int,
                           nx: libc::c_int);
    #[no_mangle]
    fn lite_room(y1: libc::c_int, x1: libc::c_int);
    #[no_mangle]
    fn genocide_aux(player_cast: bool_, typ: libc::c_char) -> bool_;
    #[no_mangle]
    fn destroy_area(y1: libc::c_int, x1: libc::c_int, r: libc::c_int,
                    full: bool_, bypass: bool_);
    #[no_mangle]
    fn unlite_room(y1: libc::c_int, x1: libc::c_int);
    #[no_mangle]
    fn project_m(who: libc::c_int, r: libc::c_int, y: libc::c_int,
                 x: libc::c_int, dam: libc::c_int, typ: libc::c_int) -> bool_;
    #[no_mangle]
    fn mon_take_hit(m_idx: libc::c_int, dam: libc::c_int, fear: *mut bool_,
                    note: cptr) -> bool_;
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
pub struct player_sex {
    pub title: cptr,
    pub winner: cptr,
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
pub struct deity_type {
    pub name: cptr,
    pub desc: [[libc::c_char; 80]; 10],
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
pub type optimize_flag = libc::c_uint;
pub const NO_OPTIMIZE: optimize_flag = 1;
pub const OPTIMIZE: optimize_flag = 0;
pub type describe_flag = libc::c_uint;
pub const NO_DESCRIBE: describe_flag = 1;
pub const DESCRIBE: describe_flag = 0;
/* File: traps.c */
/* Purpose: handle traps */
/* the below copyright probably still applies, but it is heavily changed
 * copied, adapted & re-engineered by JK.
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
#[no_mangle]
pub unsafe extern "C" fn do_player_trap_call_out() -> bool_ {
    let mut i: s16b = 0;
    let mut sn: s16b = 0;
    let mut cx: s16b = 0;
    let mut cy: s16b = 0;
    let mut h_index: s16b = 0 as libc::c_int as s16b;
    let mut h_level: s16b = 0 as libc::c_int as s16b;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut ident: bool_ = 0 as libc::c_int as bool_;
    i = 1 as libc::c_int as s16b;
    while (i as libc::c_int) < m_max as libc::c_int {
        m_ptr = &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Paranoia -- Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).level as libc::c_int >= h_level as libc::c_int {
                h_level = (*m_ptr).level as s16b;
                h_index = i
            }
        }
        i += 1
    }
    /* if the level is empty of monsters, h_index will be 0 */
    if h_index == 0 { return 0 as libc::c_int as bool_ }
    m_ptr = &mut *m_list.offset(h_index as isize) as *mut monster_type;
    sn = 0 as libc::c_int as s16b;
    i = 0 as libc::c_int as s16b;
    while (i as libc::c_int) < 8 as libc::c_int {
        cx =
            ((*p_ptr).px as libc::c_int + ddx[i as usize] as libc::c_int) as
                s16b;
        cy =
            ((*p_ptr).py as libc::c_int + ddy[i as usize] as libc::c_int) as
                s16b;
        /* Skip non-empty grids */
        if !(cave_valid_bold(cy as libc::c_int, cx as libc::c_int) == 0) {
            if !((*cave[cy as usize].offset(cx as isize)).feat as libc::c_int
                     == 0x3 as libc::c_int) {
                if !(cx as libc::c_int == (*p_ptr).px as libc::c_int &&
                         cy as libc::c_int == (*p_ptr).py as libc::c_int) {
                    sn += 1;
                    /* Randomize choice */
                    if !(Rand_div(sn as s32b) > 0 as libc::c_int) {
                        (*cave[cy as usize].offset(cx as isize)).m_idx =
                            h_index;
                        (*cave[(*m_ptr).fy as
                                   usize].offset((*m_ptr).fx as isize)).m_idx
                            = 0 as libc::c_int as s16b;
                        (*m_ptr).fx = cx as byte_hack;
                        (*m_ptr).fy = cy as byte_hack;
                        /* we do not change the sublevel! */
                        ident = 1 as libc::c_int as bool_;
                        update_mon(h_index as libc::c_int,
                                   1 as libc::c_int as bool_);
                        monster_desc(m_name.as_mut_ptr(), m_ptr,
                                     0x8 as libc::c_int);
                        msg_format(b"You hear a rapid-shifting wail, and %s appears!\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                        break ;
                    }
                }
            }
        }
        i += 1
    }
    return ident;
}
unsafe extern "C" fn do_trap_teleport_away(mut i_ptr: *mut object_type,
                                           mut y: s16b, mut x: s16b)
 -> bool_ {
    let mut ident: bool_ = 0 as libc::c_int as bool_;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut o_idx: s16b = 0 as libc::c_int as s16b;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut x1: s16b = 0;
    let mut y1: s16b = 0;
    if i_ptr.is_null() { return 0 as libc::c_int as bool_ }
    if (*i_ptr).name1 as libc::c_int == 13 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    while o_idx as libc::c_int == 0 as libc::c_int {
        x1 = Rand_div(cur_wid as s32b) as s16b;
        y1 = Rand_div(cur_hgt as s32b) as s16b;
        /* Obtain grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y1 as isize)).offset(x1 as isize)
                as *mut cave_type;
        /* Require floor space (or shallow terrain) -KMW- */
        if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
               0x10 as libc::c_long == 0 {
            continue ;
        }
        o_idx =
            drop_near(i_ptr, 0 as libc::c_int, y1 as libc::c_int,
                      x1 as libc::c_int)
    }
    o_ptr = &mut *o_list.offset(o_idx as isize) as *mut object_type;
    x1 = (*o_ptr).ix as s16b;
    y1 = (*o_ptr).iy as s16b;
    if (*p_ptr).blind == 0 {
        note_spot(y as libc::c_int, x as libc::c_int);
        lite_spot(y as libc::c_int, x as libc::c_int);
        ident = 1 as libc::c_int as bool_;
        object_desc(o_name.as_mut_ptr(), i_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        if (*cave[y1 as usize].offset(x1 as isize)).info as libc::c_int &
               0x20 as libc::c_int != 0 as libc::c_int {
            lite_spot(y1 as libc::c_int, x1 as libc::c_int);
            msg_format(b"The %s suddenly stands elsewhere.\x00" as *const u8
                           as *const libc::c_char, o_name.as_mut_ptr());
        } else {
            msg_format(b"You suddenly don\'t see the %s any more!\x00" as
                           *const u8 as *const libc::c_char,
                       o_name.as_mut_ptr());
        }
    } else {
        msg_print(b"You hear something move.\x00" as *const u8 as
                      *const libc::c_char);
    }
    return ident;
}
/*
 * this handles a trap that places walls around the player
 */
unsafe extern "C" fn player_handle_trap_of_walls() -> bool_ {
    let mut ident: bool_ = 0;
    let mut dx: s16b = 0;
    let mut dy: s16b = 0;
    let mut cx: s16b = 0;
    let mut cy: s16b = 0;
    let mut sx: s16b = 0 as libc::c_int as s16b;
    let mut sy: s16b = 0 as libc::c_int as s16b;
    let mut sn: s16b = 0;
    let mut i: s16b = 0;
    let mut cv_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut map: [[bool_; 5]; 5] =
        [[0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
          0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
          0 as libc::c_int as bool_],
         [0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
          0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
          0 as libc::c_int as bool_],
         [0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
          0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
          0 as libc::c_int as bool_],
         [0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
          0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
          0 as libc::c_int as bool_],
         [0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
          0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
          0 as libc::c_int as bool_]];
    dy = -(2 as libc::c_int) as s16b;
    while dy as libc::c_int <= 2 as libc::c_int {
        dx = -(2 as libc::c_int) as s16b;
        while dx as libc::c_int <= 2 as libc::c_int {
            /* Extract the location */
            cx = ((*p_ptr).px as libc::c_int + dx as libc::c_int) as s16b;
            cy = ((*p_ptr).py as libc::c_int + dy as libc::c_int) as s16b;
            if cy as libc::c_int > 0 as libc::c_int &&
                   cx as libc::c_int > 0 as libc::c_int &&
                   (cy as libc::c_int) <
                       cur_hgt as libc::c_int - 1 as libc::c_int &&
                   (cx as libc::c_int) <
                       cur_wid as libc::c_int - 1 as libc::c_int {
                cv_ptr =
                    &mut *(*cave.as_mut_ptr().offset(cy as
                                                         isize)).offset(cx as
                                                                            isize)
                        as *mut cave_type;
                if !((*cv_ptr).m_idx != 0) {
                    /* Lose room and vault */
                    (*cv_ptr).info =
                        ((*cv_ptr).info as libc::c_int &
                             !(0x8 as libc::c_int | 0x4 as libc::c_int)) as
                            u16b;
                    /* Lose light and knowledge */
                    (*cv_ptr).info =
                        ((*cv_ptr).info as libc::c_int &
                             !(0x2 as libc::c_int | 0x1 as libc::c_int)) as
                            u16b;
                    /* Skip the center */
                    if !(dx == 0 && dy == 0) {
                        /* test for dungeon level */
                        if !(Rand_div(100 as libc::c_int) + 1 as libc::c_int >
                                 10 as libc::c_int +
                                     *max_dlv.offset(dungeon_type as isize) as
                                         libc::c_int) {
                            /* Damage this grid */
                            map[(2 as libc::c_int + dx as libc::c_int) as
                                    usize][(2 as libc::c_int +
                                                dy as libc::c_int) as usize] =
                                1 as libc::c_int as bool_
                        }
                    }
                }
            }
            dx += 1
        }
        dy += 1
    }
    dy = -(2 as libc::c_int) as s16b;
    while dy as libc::c_int <= 2 as libc::c_int {
        dx = -(2 as libc::c_int) as s16b;
        while dx as libc::c_int <= 2 as libc::c_int {
            /* Extract the location */
            cx = ((*p_ptr).px as libc::c_int + dx as libc::c_int) as s16b;
            cy = ((*p_ptr).py as libc::c_int + dy as libc::c_int) as s16b;
            /* if monster on square */
            /* Skip unaffected grids */
            if !(map[(2 as libc::c_int + dx as libc::c_int) as
                         usize][(2 as libc::c_int + dy as libc::c_int) as
                                    usize] == 0) {
                cv_ptr =
                    &mut *(*cave.as_mut_ptr().offset(cy as
                                                         isize)).offset(cx as
                                                                            isize)
                        as *mut cave_type;
                if (*cv_ptr).m_idx != 0 {
                    let mut m_ptr: *mut monster_type =
                        &mut *m_list.offset((*cv_ptr).m_idx as isize) as
                            *mut monster_type;
                    let mut r_ptr: *mut monster_race =
                        if !(*m_ptr).sr_ptr.is_null() {
                            (*m_ptr).sr_ptr
                        } else {
                            race_info_idx((*m_ptr).r_idx as libc::c_int,
                                          (*m_ptr).ego as libc::c_int)
                        };
                    /* if monster can co-exist with rock */
                    if (*r_ptr).flags2 &
                           0x80000 as libc::c_int as libc::c_uint == 0 &&
                           (*r_ptr).flags2 &
                               0x40000 as libc::c_int as libc::c_uint == 0 {
                        let mut m_name: [libc::c_char; 80] = [0; 80];
                        /* Most monsters cannot co-exist with rock */
                        /* if sn */
                        sn = 0 as libc::c_int as s16b;
                        if (*r_ptr).flags1 &
                               0x20000 as libc::c_int as libc::c_uint == 0 {
                            /* Assume not safe */
                            /* Monster can move to escape the wall */
                            /* Look for safety */
                            i = 0 as libc::c_int as s16b;
                            while (i as libc::c_int) < 8 as libc::c_int {
                                /* Access the grid */
                                cy =
                                    ((*p_ptr).py as libc::c_int +
                                         ddy[i as usize] as libc::c_int) as
                                        s16b;
                                cx =
                                    ((*p_ptr).px as libc::c_int +
                                         ddx[i as usize] as libc::c_int) as
                                        s16b;
                                /* discontinue for loop - safe grid found */
                                /* Skip non-empty grids */
                                if (*f_info.offset((*cave[cy as
                                                              usize].offset(cx
                                                                                as
                                                                                isize)).feat
                                                       as isize)).flags1 as
                                       libc::c_long & 0x10 as libc::c_long !=
                                       0 &&
                                       (*cave[cy as
                                                  usize].offset(cx as
                                                                    isize)).feat
                                           as libc::c_int !=
                                           0xaf as libc::c_int &&
                                       (*cave[cy as
                                                  usize].offset(cx as
                                                                    isize)).o_idx
                                           as libc::c_int == 0 as libc::c_int
                                       &&
                                       (*f_info.offset((*cave[cy as
                                                                  usize].offset(cx
                                                                                    as
                                                                                    isize)).feat
                                                           as isize)).flags1
                                           as libc::c_long &
                                           0x40 as libc::c_long == 0 {
                                    /* Hack -- no safety on glyph of warding */
                                    if !((*cave[cy as
                                                    usize].offset(cx as
                                                                      isize)).feat
                                             as libc::c_int ==
                                             0x3 as libc::c_int) {
                                        /* Important -- Skip "quake" grids */
                                        if !(map[(2 as libc::c_int +
                                                      (cx as libc::c_int -
                                                           (*p_ptr).px as
                                                               libc::c_int))
                                                     as
                                                     usize][(2 as libc::c_int
                                                                 +
                                                                 (cy as
                                                                      libc::c_int
                                                                      -
                                                                      (*p_ptr).py
                                                                          as
                                                                          libc::c_int))
                                                                as usize] !=
                                                 0) {
                                            /* Count "safe" grids */
                                            sn += 1;
                                            /* Randomize choice */
                                            if !(Rand_div(sn as s32b) >
                                                     0 as libc::c_int) {
                                                /* Save the safe grid */
                                                sx = cx;
                                                sy = cy;
                                                ident =
                                                    1 as libc::c_int as bool_;
                                                break ;
                                            }
                                        }
                                    }
                                }
                                i += 1
                            }
                        }
                        monster_desc(m_name.as_mut_ptr(), m_ptr,
                                     0 as libc::c_int);
                        msg_format(b"%^s wails out in pain!\x00" as *const u8
                                       as *const libc::c_char,
                                   m_name.as_mut_ptr());
                        (*m_ptr).csleep = 0 as libc::c_int as s16b;
                        (*m_ptr).hp -=
                            if sn as libc::c_int != 0 {
                                damroll(4 as libc::c_int as s16b,
                                        8 as libc::c_int as s16b)
                            } else { 200 as libc::c_int };
                        if (*m_ptr).hp < 0 as libc::c_int {
                            /* Describe the monster */
                            /* Scream in pain */
                            /* Monster is certainly awake */
                            /* Apply damage directly */
                            /* Delete (not kill) "dead" monsters */
                            /* Message */
                            msg_format(b"%^s is entombed in the rock!\x00" as
                                           *const u8 as *const libc::c_char,
                                       m_name.as_mut_ptr());
                            /* Delete the monster */
                            delete_monster_idx((*cave[cy as
                                                          usize].offset(cx as
                                                                            isize)).m_idx
                                                   as libc::c_int);
                            /* No longer safe */
                            sn = 0 as libc::c_int as s16b
                        }
                        if sn != 0 {
                            let mut m_idx: s16b =
                                (*cave[cy as
                                           usize].offset(cx as isize)).m_idx;
                            /* Hack -- Escape from the rock */
                            /* Update the new location */
                            (*cave[sy as usize].offset(sx as isize)).m_idx =
                                m_idx;
                            /* Update the old location */
                            (*cave[cy as usize].offset(cx as isize)).m_idx =
                                0 as libc::c_int as s16b;
                            /* Move the monster */
                            (*m_ptr).fy = sy as byte_hack;
                            (*m_ptr).fx = sx as byte_hack;
                            /* do not change fz */
						/* don't make rock on that square! */
                            if sx as libc::c_int >=
                                   (*p_ptr).px as libc::c_int -
                                       2 as libc::c_int &&
                                   sx as libc::c_int <=
                                       (*p_ptr).px as libc::c_int +
                                           2 as libc::c_int &&
                                   sy as libc::c_int >=
                                       (*p_ptr).py as libc::c_int -
                                           2 as libc::c_int &&
                                   sy as libc::c_int <=
                                       (*p_ptr).py as libc::c_int +
                                           2 as libc::c_int {
                                map[(2 as libc::c_int +
                                         (sx as libc::c_int -
                                              (*p_ptr).px as libc::c_int)) as
                                        usize][(2 as libc::c_int +
                                                    (sy as libc::c_int -
                                                         (*p_ptr).py as
                                                             libc::c_int)) as
                                                   usize] =
                                    0 as libc::c_int as bool_
                            }
                            /* Update the monster (new location) */
                            update_mon(m_idx as libc::c_int,
                                       1 as libc::c_int as bool_);
                            /* Redraw the old grid */
                            lite_spot(cy as libc::c_int, cx as libc::c_int);
                            /* Redraw the new grid */
                            lite_spot(sy as libc::c_int, sx as libc::c_int);
                        }
                    }
                }
            }
            dx += 1
        }
        dy += 1
    }
    /* Examine the quaked region */
    dy = -(2 as libc::c_int) as s16b;
    while dy as libc::c_int <= 2 as libc::c_int {
        dx = -(2 as libc::c_int) as s16b;
        while dx as libc::c_int <= 2 as libc::c_int {
            /* Extract the location */
            cx = ((*p_ptr).px as libc::c_int + dx as libc::c_int) as s16b;
            cy = ((*p_ptr).py as libc::c_int + dy as libc::c_int) as s16b;
            /* Skip unaffected grids */
            if !(map[(2 as libc::c_int + dx as libc::c_int) as
                         usize][(2 as libc::c_int + dy as libc::c_int) as
                                    usize] == 0) {
                /* Access the cave grid */
                cv_ptr =
                    &mut *(*cave.as_mut_ptr().offset(cy as
                                                         isize)).offset(cx as
                                                                            isize)
                        as *mut cave_type;
                /* Paranoia -- never affect player */
                if !(dy == 0 && dx == 0) {
                    /* Destroy location (if valid) */
                    if (cx as libc::c_int) < cur_wid as libc::c_int &&
                           (cy as libc::c_int) < cur_hgt as libc::c_int &&
                           cave_valid_bold(cy as libc::c_int,
                                           cx as libc::c_int) as libc::c_int
                               != 0 {
                        let mut floor: bool_ =
                            ((*f_info.offset((*cave[cy as
                                                        usize].offset(cx as
                                                                          isize)).feat
                                                 as isize)).flags1 as
                                 libc::c_long & 0x10 as libc::c_long) as
                                bool_;
                        /* Delete any object that is still there */
                        delete_object(cy as libc::c_int, cx as libc::c_int);
                        if floor != 0 {
                            cave_set_feat(cy as libc::c_int,
                                          cx as libc::c_int,
                                          0x3a as libc::c_int);
                        } else {
                            /* Clear previous contents, add floor */
                            cave_set_feat(cy as libc::c_int,
                                          cx as libc::c_int,
                                          0x1 as libc::c_int);
                        }
                    }
                }
            }
            dx += 1
        }
        dy += 1
    }
    /* Mega-Hack -- Forget the view and lite */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x10000 as libc::c_long) as u32b;
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                  0x200000 as libc::c_long)) as u32b;
    /* Update the monsters */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long) as u32b;
    /* Update the health bar */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x800 as libc::c_long) as u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
    handle_stuff();
    msg_print(b"Suddenly the cave shifts around you. The air is getting stale!\x00"
                  as *const u8 as *const libc::c_char);
    ident = 1 as libc::c_int as bool_;
    return ident;
}
/*
 * this function handles arrow & dagger traps, in various types.
 * num = number of missiles
 * tval, sval = kind of missiles
 * dd,ds = damage roll for missiles
 * poison_dam = additional poison damage
 * name = name given if you should die from it...
 *
 * return value = ident (always TRUE)
 */
unsafe extern "C" fn player_handle_missile_trap(mut num: s16b, mut tval: s16b,
                                                mut sval: s16b, mut dd: s16b,
                                                mut ds: s16b, mut pdam: s16b,
                                                mut name: cptr) -> bool_ {
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
    let mut i: s16b = 0;
    let mut k_idx: s16b =
        lookup_kind(tval as libc::c_int, sval as libc::c_int);
    let mut i_name: [libc::c_char; 80] = [0; 80];
    o_ptr = &mut forge;
    object_prep(o_ptr, k_idx as libc::c_int);
    (*o_ptr).number = num as byte_hack;
    apply_magic(o_ptr, *max_dlv.offset(dungeon_type as isize) as libc::c_int,
                0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                0 as libc::c_int as bool_);
    object_desc(i_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                0 as libc::c_int);
    msg_format(b"Suddenly %s hit%s you!\x00" as *const u8 as
                   *const libc::c_char, i_name.as_mut_ptr(),
               if num as libc::c_int == 1 as libc::c_int {
                   b"\x00" as *const u8 as *const libc::c_char
               } else { b"s\x00" as *const u8 as *const libc::c_char });
    i = 0 as libc::c_int as s16b;
    while (i as libc::c_int) < num as libc::c_int {
        take_hit(damroll(dd, ds), name);
        redraw_stuff();
        if pdam as libc::c_int > 0 as libc::c_int {
            if !((*p_ptr).resist_pois as libc::c_int != 0 ||
                     (*p_ptr).oppose_pois as libc::c_int != 0) {
                set_poisoned((*p_ptr).poisoned as libc::c_int +
                                 pdam as libc::c_int);
            }
        }
        i += 1
    }
    drop_near(o_ptr, -(1 as libc::c_int), (*p_ptr).py as libc::c_int,
              (*p_ptr).px as libc::c_int);
    return 1 as libc::c_int as bool_;
}
/*
 * this function handles a "breath" type trap - acid bolt, lightning balls etc.
 */
unsafe extern "C" fn player_handle_breath_trap(mut rad: s16b,
                                               mut type_0: s16b,
                                               mut trap: u16b) -> bool_ {
    let mut t_ptr: *mut trap_type =
        &mut *t_info.offset(trap as isize) as *mut trap_type;
    let mut ident: bool_ = 0;
    let mut my_dd: s16b = 0;
    let mut my_ds: s16b = 0;
    let mut dam: s16b = 0;
    my_dd = (*t_ptr).dd;
    my_ds = (*t_ptr).ds;
    /* these traps gets nastier as levels progress */
    if *max_dlv.offset(dungeon_type as isize) as libc::c_int >
           2 as libc::c_int * (*t_ptr).minlevel as libc::c_int {
        my_dd =
            (my_dd as libc::c_int +
                 *max_dlv.offset(dungeon_type as isize) as libc::c_int /
                     15 as libc::c_int) as s16b;
        my_ds =
            (my_ds as libc::c_int +
                 *max_dlv.offset(dungeon_type as isize) as libc::c_int /
                     15 as libc::c_int) as s16b
    }
    dam = damroll(my_dd, my_ds) as s16b;
    ident =
        project(-(2 as libc::c_int), rad as libc::c_int,
                (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                dam as libc::c_int, type_0 as libc::c_int,
                0x40 as libc::c_int | 0x1 as libc::c_int);
    return ident;
}
/*
 * This function damages the player by a trap
 */
unsafe extern "C" fn trap_hit(mut trap: s16b) {
    let mut dam: s16b = 0;
    let mut t_ptr: *mut trap_type =
        &mut *t_info.offset(trap as isize) as *mut trap_type;
    dam = damroll((*t_ptr).dd, (*t_ptr).ds) as s16b;
    take_hit(dam as libc::c_int,
             t_name.offset((*t_ptr).name as libc::c_int as isize) as cptr);
}
/*
 * this function activates one trap type, and returns
 * a bool_ indicating if this trap is now identified
 */
#[no_mangle]
pub unsafe extern "C" fn player_activate_trap_type(mut y: s16b, mut x: s16b,
                                                   mut i_ptr:
                                                       *mut object_type,
                                                   mut item: s16b) -> bool_ {
    let mut ident: bool_ = 0 as libc::c_int as bool_;
    let mut trap: s16b = 0;
    let mut k: s16b = 0;
    let mut l: s16b = 0;
    trap = (*cave[y as usize].offset(x as isize)).t_idx;
    if !i_ptr.is_null() { trap = (*i_ptr).pval as s16b }
    if i_ptr.is_null() &&
           (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int !=
               0 as libc::c_int {
        i_ptr =
            &mut *o_list.offset((*(*cave.as_mut_ptr().offset(y as
                                                                 isize)).offset(x
                                                                                    as
                                                                                    isize)).o_idx
                                    as isize) as *mut object_type
    }
    match trap as libc::c_int {
        1 => {
            /* stat traps */
            ident = do_dec_stat(0 as libc::c_int, 1 as libc::c_int)
        }
        2 => { ident = do_dec_stat(0 as libc::c_int, 2 as libc::c_int) }
        3 => { ident = do_dec_stat(0 as libc::c_int, 3 as libc::c_int) }
        4 => { ident = do_dec_stat(1 as libc::c_int, 1 as libc::c_int) }
        5 => { ident = do_dec_stat(1 as libc::c_int, 2 as libc::c_int) }
        6 => { ident = do_dec_stat(1 as libc::c_int, 3 as libc::c_int) }
        7 => { ident = do_dec_stat(2 as libc::c_int, 1 as libc::c_int) }
        8 => { ident = do_dec_stat(2 as libc::c_int, 2 as libc::c_int) }
        9 => { ident = do_dec_stat(2 as libc::c_int, 3 as libc::c_int) }
        10 => { ident = do_dec_stat(3 as libc::c_int, 1 as libc::c_int) }
        11 => { ident = do_dec_stat(3 as libc::c_int, 2 as libc::c_int) }
        12 => { ident = do_dec_stat(3 as libc::c_int, 3 as libc::c_int) }
        13 => { ident = do_dec_stat(4 as libc::c_int, 1 as libc::c_int) }
        14 => { ident = do_dec_stat(4 as libc::c_int, 2 as libc::c_int) }
        15 => { ident = do_dec_stat(4 as libc::c_int, 3 as libc::c_int) }
        16 => { ident = do_dec_stat(5 as libc::c_int, 1 as libc::c_int) }
        17 => { ident = do_dec_stat(5 as libc::c_int, 2 as libc::c_int) }
        18 => { ident = do_dec_stat(5 as libc::c_int, 3 as libc::c_int) }
        20 => {
            /* Trap of Curse Weapon */
            ident = curse_weapon()
        }
        21 => {
            /* Trap of Curse Armor */
            ident = curse_armor()
        }
        22 => {
            /* Earthquake Trap */
            msg_print(b"As you touch the trap, the ground starts to shake.\x00"
                          as *const u8 as *const libc::c_char);
            earthquake(y as libc::c_int, x as libc::c_int, 10 as libc::c_int);
            ident = 1 as libc::c_int as bool_
        }
        23 => {
            /* Poison Needle Trap */
            if !((*p_ptr).resist_pois as libc::c_int != 0 ||
                     (*p_ptr).oppose_pois as libc::c_int != 0) {
                msg_print(b"You prick yourself on a poisoned needle.\x00" as
                              *const u8 as *const libc::c_char);
                set_poisoned((*p_ptr).poisoned as libc::c_int +
                                 Rand_div(15 as libc::c_int) +
                                 10 as libc::c_int);
                ident = 1 as libc::c_int as bool_
            } else {
                msg_print(b"You prick yourself on a needle.\x00" as *const u8
                              as *const libc::c_char);
            }
        }
        24 => {
            /* Summon Monster Trap */
            msg_print(b"A spell hangs in the air.\x00" as *const u8 as
                          *const libc::c_char);
            k = 0 as libc::c_int as s16b;
            while (k as libc::c_int) <
                      Rand_div(3 as libc::c_int) + 1 as libc::c_int {
                ident =
                    (ident as libc::c_int |
                         summon_specific(y as libc::c_int, x as libc::c_int,
                                         *max_dlv.offset(dungeon_type as
                                                             isize) as
                                             libc::c_int, 0 as libc::c_int) as
                             libc::c_int) as bool_;
                k += 1
            }
        }
        25 => {
            /* Summon Undead Trap */
            msg_print(b"A mighty spell hangs in the air.\x00" as *const u8 as
                          *const libc::c_char);
            k = 0 as libc::c_int as s16b;
            while (k as libc::c_int) <
                      Rand_div(3 as libc::c_int) + 1 as libc::c_int {
                ident =
                    (ident as libc::c_int |
                         summon_specific(y as libc::c_int, x as libc::c_int,
                                         *max_dlv.offset(dungeon_type as
                                                             isize) as
                                             libc::c_int, 17 as libc::c_int)
                             as libc::c_int) as bool_;
                k += 1
            }
        }
        26 => {
            /* Summon Greater Undead Trap */
            msg_print(b"An old and evil spell hangs in the air.\x00" as
                          *const u8 as *const libc::c_char);
            k = 0 as libc::c_int as s16b;
            while (k as libc::c_int) <
                      Rand_div(3 as libc::c_int) + 1 as libc::c_int {
                ident =
                    (ident as libc::c_int |
                         summon_specific(y as libc::c_int, x as libc::c_int,
                                         *max_dlv.offset(dungeon_type as
                                                             isize) as
                                             libc::c_int, 21 as libc::c_int)
                             as libc::c_int) as bool_;
                k += 1
            }
        }
        27 => {
            /* Teleport Trap */
            msg_print(b"The world whirls around you.\x00" as *const u8 as
                          *const libc::c_char);
            teleport_player(66 as libc::c_int / 22 as libc::c_int *
                                67 as libc::c_int);
            ident = 1 as libc::c_int as bool_
        }
        28 => {
            /* Paralyzing Trap */
            if (*p_ptr).free_act == 0 {
                msg_print(b"You touch a poisoned part and can\'t move.\x00" as
                              *const u8 as *const libc::c_char);
                set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                  Rand_div(10 as libc::c_int) +
                                  10 as libc::c_int);
                ident = 1 as libc::c_int as bool_
            } else {
                msg_print(b"You prick yourself on a needle.\x00" as *const u8
                              as *const libc::c_char);
            }
        }
        29 => {
            /* Explosive Device */
            msg_print(b"A hidden explosive device explodes in your face.\x00"
                          as *const u8 as *const libc::c_char);
            take_hit(damroll(5 as libc::c_int as s16b,
                             8 as libc::c_int as s16b),
                     b"an explosion\x00" as *const u8 as *const libc::c_char);
            ident = 1 as libc::c_int as bool_
        }
        30 => {
            /* Teleport Away Trap */
            let mut item_0: libc::c_int = 0;
            let mut amt: libc::c_int = 0;
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
            /* teleport away all items */
            while (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int
                      != 0 as libc::c_int {
                item_0 =
                    (*cave[y as usize].offset(x as isize)).o_idx as
                        libc::c_int;
                o_ptr =
                    &mut *o_list.offset(item_0 as isize) as *mut object_type;
                amt = (*o_ptr).number as libc::c_int;
                ident = do_trap_teleport_away(o_ptr, y, x);
                floor_item_increase(item_0, -amt);
                floor_item_optimize(item_0);
            }
        }
        31 => {
            /* Lose Memory Trap */
            lose_exp((*p_ptr).exp / 4 as libc::c_int);
            ident =
                (ident as libc::c_int |
                     dec_stat(2 as libc::c_int,
                              Rand_div(20 as libc::c_int) + 10 as libc::c_int,
                              2 as libc::c_int) as libc::c_int) as bool_;
            ident =
                (ident as libc::c_int |
                     dec_stat(1 as libc::c_int,
                              Rand_div(20 as libc::c_int) + 10 as libc::c_int,
                              2 as libc::c_int) as libc::c_int) as bool_;
            if (*p_ptr).resist_conf == 0 {
                ident =
                    (ident as libc::c_int |
                         set_confused((*p_ptr).confused as libc::c_int +
                                          Rand_div(100 as libc::c_int) +
                                          50 as libc::c_int) as libc::c_int)
                        as bool_
            }
            if ident != 0 {
                msg_print(b"You suddenly don\'t remember what you were doing.\x00"
                              as *const u8 as *const libc::c_char);
            } else {
                msg_print(b"You feel an alien force probing your mind.\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
        32 => {
            /* Bitter Regret Trap */
            msg_print(b"An age-old and hideous-sounding spell reverberates off the walls.\x00"
                          as *const u8 as *const libc::c_char);
            ident =
                (ident as libc::c_int |
                     dec_stat(3 as libc::c_int, 25 as libc::c_int,
                              1 as libc::c_int) as libc::c_int) as bool_;
            ident =
                (ident as libc::c_int |
                     dec_stat(2 as libc::c_int, 25 as libc::c_int,
                              1 as libc::c_int) as libc::c_int) as bool_;
            ident =
                (ident as libc::c_int |
                     dec_stat(4 as libc::c_int, 25 as libc::c_int,
                              1 as libc::c_int) as libc::c_int) as bool_;
            ident =
                (ident as libc::c_int |
                     dec_stat(0 as libc::c_int, 25 as libc::c_int,
                              1 as libc::c_int) as libc::c_int) as bool_;
            ident =
                (ident as libc::c_int |
                     dec_stat(5 as libc::c_int, 25 as libc::c_int,
                              1 as libc::c_int) as libc::c_int) as bool_;
            ident =
                (ident as libc::c_int |
                     dec_stat(1 as libc::c_int, 25 as libc::c_int,
                              1 as libc::c_int) as libc::c_int) as bool_
        }
        33 => {
            /* Bowel Cramps Trap */
            msg_print(b"A wretched-smelling gas cloud upsets your stomach.\x00"
                          as *const u8 as *const libc::c_char);
            set_food(100 as libc::c_int - 1 as libc::c_int);
            set_poisoned(0 as libc::c_int);
            if (*p_ptr).free_act == 0 {
                set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                  Rand_div(dun_level as s32b) +
                                  6 as libc::c_int);
            }
            ident = 1 as libc::c_int as bool_
        }
        34 => {
            /* Blindness/Confusion Trap */
            msg_print(b"A powerful magic protected this.\x00" as *const u8 as
                          *const libc::c_char);
            if (*p_ptr).resist_blind == 0 {
                ident =
                    (ident as libc::c_int |
                         set_blind((*p_ptr).blind as libc::c_int +
                                       Rand_div(100 as libc::c_int) +
                                       100 as libc::c_int) as libc::c_int) as
                        bool_
            }
            if (*p_ptr).resist_conf == 0 {
                ident =
                    (ident as libc::c_int |
                         set_confused((*p_ptr).confused as libc::c_int +
                                          Rand_div(20 as libc::c_int) +
                                          15 as libc::c_int) as libc::c_int)
                        as bool_
            }
        }
        35 => {
            /* Aggravation Trap */
            msg_print(b"You hear a hollow noise echoing through the dungeons.\x00"
                          as *const u8 as *const libc::c_char);
            aggravate_monsters(1 as libc::c_int);
        }
        36 => {
            /* Multiplication Trap */
            msg_print(b"You hear a loud click.\x00" as *const u8 as
                          *const libc::c_char);
            k = -(1 as libc::c_int) as s16b;
            while k as libc::c_int <= 1 as libc::c_int {
                l = -(1 as libc::c_int) as s16b;
                while l as libc::c_int <= 1 as libc::c_int {
                    if (*p_ptr).py as libc::c_int + l as libc::c_int >
                           0 as libc::c_int &&
                           (*p_ptr).px as libc::c_int + k as libc::c_int >
                               0 as libc::c_int &&
                           ((*p_ptr).py as libc::c_int + l as libc::c_int) <
                               cur_hgt as libc::c_int - 1 as libc::c_int &&
                           ((*p_ptr).px as libc::c_int + k as libc::c_int) <
                               cur_wid as libc::c_int - 1 as libc::c_int &&
                           (*cave[((*p_ptr).py as libc::c_int +
                                       l as libc::c_int) as
                                      usize].offset(((*p_ptr).px as
                                                         libc::c_int +
                                                         k as libc::c_int) as
                                                        isize)).t_idx == 0 {
                        place_trap((*p_ptr).py as libc::c_int +
                                       l as libc::c_int,
                                   (*p_ptr).px as libc::c_int +
                                       k as libc::c_int);
                    }
                    l += 1
                }
                k += 1
            }
            ident = 1 as libc::c_int as bool_
        }
        37 => {
            /*
			 * please note that magical stealing is not so
			 * easily circumvented
			 */
            if (*p_ptr).paralyzed == 0 &&
                   Rand_div(160 as libc::c_int) <
                       *adj_dex_safe.as_mut_ptr().offset((*p_ptr).stat_ind[3
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                                                             as isize) as
                           libc::c_int + (*p_ptr).lev as libc::c_int {
                /* Saving throw message */
                msg_print(b"Your backpack seems to vibrate strangely!\x00" as
                              *const u8 as *const libc::c_char);
            } else {
                /* Find an item */
                k = 0 as libc::c_int as s16b;
                while (k as libc::c_int) < Rand_div(10 as libc::c_int) {
                    let mut i_name: [libc::c_char; 80] = [0; 80];
                    let mut j_ptr: *mut object_type = 0 as *mut object_type;
                    let mut q_ptr: *mut object_type = 0 as *mut object_type;
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
                    /* Pick an item */
                    let mut i: s16b = Rand_div(23 as libc::c_int) as s16b;
                    /* Obtain the item */
                    j_ptr =
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as
                                                                         isize)
                            as *mut object_type;
                    /* Accept real items */
                    if !((*j_ptr).k_idx == 0) {
                        /* Don't steal artifacts  -CFT */
                        if !((*j_ptr).tval as libc::c_int ==
                                 102 as libc::c_int ||
                                 (if (*j_ptr).name1 as libc::c_int != 0 {
                                      1 as libc::c_int
                                  } else { 0 as libc::c_int }) != 0 ||
                                 (if (*j_ptr).art_name as libc::c_int != 0 {
                                      1 as libc::c_int
                                  } else { 0 as libc::c_int }) != 0 ||
                                 (if (*k_info.offset((*j_ptr).k_idx as
                                                         isize)).flags3 as
                                         libc::c_long & 0x8000 as libc::c_long
                                         != 0 {
                                      1 as libc::c_int
                                  } else { 0 as libc::c_int }) != 0) {
                            /* Get a description */
                            object_desc(i_name.as_mut_ptr(), j_ptr,
                                        0 as libc::c_int, 3 as libc::c_int);
                            /* Message */
                            msg_format(b"%sour %s (%c) was stolen!\x00" as
                                           *const u8 as *const libc::c_char,
                                       if (*j_ptr).number as libc::c_int >
                                              1 as libc::c_int {
                                           b"One of y\x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"Y\x00" as *const u8 as
                                               *const libc::c_char
                                       }, i_name.as_mut_ptr(),
                                       index_to_label(i as libc::c_int) as
                                           libc::c_int);
                            /* Create the item */
                            q_ptr = &mut forge;
                            object_copy(q_ptr, j_ptr);
                            (*q_ptr).number = 1 as libc::c_int as byte_hack;
                            /* Drop it somewhere */
                            do_trap_teleport_away(q_ptr, y, x);
                            inc_stack_size_ex(i as libc::c_int,
                                              -(1 as libc::c_int), OPTIMIZE,
                                              NO_DESCRIBE);
                            ident = 1 as libc::c_int as bool_
                        }
                    }
                    k += 1
                }
            }
        }
        38 => {
            /* Summon Fast Quylthulgs Trap */
            k = 0 as libc::c_int as s16b;
            while (k as libc::c_int) <
                      Rand_div(3 as libc::c_int) + 1 as libc::c_int {
                ident =
                    (ident as libc::c_int |
                         summon_specific(y as libc::c_int, x as libc::c_int,
                                         *max_dlv.offset(dungeon_type as
                                                             isize) as
                                             libc::c_int, 57 as libc::c_int)
                             as libc::c_int) as bool_;
                k += 1
            }
            if ident != 0 {
                msg_print(b"You suddenly have company.\x00" as *const u8 as
                              *const libc::c_char);
                set_slow((*p_ptr).slow as libc::c_int +
                             (Rand_div(25 as libc::c_int) + 1 as libc::c_int)
                             + 15 as libc::c_int);
            }
        }
        39 => {
            /* Trap of Sinking */
            msg_print(b"You fell through a trap door!\x00" as *const u8 as
                          *const libc::c_char);
            if (*p_ptr).ffall != 0 {
                if dungeon_flags1 as libc::c_long & 0x800000 as libc::c_long
                       != 0 {
                    msg_print(b"You float gently down to the previous level.\x00"
                                  as *const u8 as *const libc::c_char);
                } else {
                    msg_print(b"You float gently down to the next level.\x00"
                                  as *const u8 as *const libc::c_char);
                }
            } else {
                take_hit(damroll(2 as libc::c_int as s16b,
                                 8 as libc::c_int as s16b),
                         b"a trap door\x00" as *const u8 as
                             *const libc::c_char);
            }
            /* Still alive and autosave enabled */
            if (*p_ptr).chp as libc::c_int >= 0 as libc::c_int {
                autosave_checkpoint();
            }
            if dungeon_flags1 as libc::c_long & 0x800000 as libc::c_long != 0
               {
                dun_level -= 1
            } else { dun_level += 1 }
            /* Leaving */
            (*p_ptr).leaving = 1 as libc::c_int as bool_
        }
        40 => {
            /* Trap of Mana Drain */
            if (*p_ptr).csp as libc::c_int > 0 as libc::c_int {
                (*p_ptr).csp = 0 as libc::c_int as s16b;
                (*p_ptr).csp_frac = 0 as libc::c_int as u16b;
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long)
                        as u32b;
                msg_print(b"You sense a great loss.\x00" as *const u8 as
                              *const libc::c_char);
                ident = 1 as libc::c_int as bool_
            } else if (*p_ptr).msp as libc::c_int == 0 as libc::c_int {
                /* no sense saying this unless you never have mana */
                msg_format(b"Suddenly you feel glad you\'re a mere %s\x00" as
                               *const u8 as *const libc::c_char,
                           c_name.offset((*spp_ptr).title as isize));
            } else {
                msg_print(b"Your head feels dizzy for a moment.\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
        41 => {
            /* Trap of Missing Money */
            let mut gold: s32b =
                (*p_ptr).au / 10 as libc::c_int +
                    (Rand_div(25 as libc::c_int) + 1 as libc::c_int);
            if gold < 2 as libc::c_int { gold = 2 as libc::c_int }
            if gold > 5000 as libc::c_int {
                gold =
                    (*p_ptr).au / 20 as libc::c_int +
                        (Rand_div(3000 as libc::c_int) + 1 as libc::c_int)
            }
            if gold > (*p_ptr).au { gold = (*p_ptr).au }
            (*p_ptr).au -= gold;
            if gold <= 0 as libc::c_int {
                msg_print(b"You feel something touching you.\x00" as *const u8
                              as *const libc::c_char);
            } else if (*p_ptr).au != 0 {
                msg_print(b"Your purse feels lighter.\x00" as *const u8 as
                              *const libc::c_char);
                msg_format(b"%ld coins were stolen!\x00" as *const u8 as
                               *const libc::c_char, gold as libc::c_long);
                ident = 1 as libc::c_int as bool_
            } else {
                msg_print(b"Your purse feels empty.\x00" as *const u8 as
                              *const libc::c_char);
                msg_print(b"All of your coins were stolen!\x00" as *const u8
                              as *const libc::c_char);
                ident = 1 as libc::c_int as bool_
            }
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x100 as libc::c_long) as
                    u32b
        }
        42 => {
            /* Trap of No Return */
            let mut j_ptr_0: *mut object_type =
                0 as *mut object_type; /* a long time */
            let mut j: s16b = 0;
            j = 0 as libc::c_int as s16b;
            while (j as libc::c_int) < 24 as libc::c_int {
                if !((*p_ptr).inventory[j as usize].k_idx == 0) {
                    j_ptr_0 =
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset(j as
                                                                         isize)
                            as *mut object_type;
                    if (*j_ptr_0).tval as libc::c_int == 70 as libc::c_int &&
                           (*j_ptr_0).sval as libc::c_int == 11 as libc::c_int
                       {
                        inc_stack_size_ex(j as libc::c_int,
                                          -((*j_ptr_0).number as libc::c_int),
                                          OPTIMIZE, NO_DESCRIBE);
                        combine_pack();
                        reorder_pack();
                        if ident == 0 {
                            msg_print(b"A small fire works its way through your backpack. Some scrolls are burnt.\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                        } else {
                            msg_print(b"The fire hasn\'t finished.\x00" as
                                          *const u8 as *const libc::c_char);
                        }
                        ident = 1 as libc::c_int as bool_
                    } else if (*j_ptr_0).tval as libc::c_int ==
                                  67 as libc::c_int &&
                                  (*j_ptr_0).pval == 3 as libc::c_int {
                        (*j_ptr_0).timeout = 0 as libc::c_int as s16b;
                        if ident == 0 {
                            msg_print(b"You feel the air stabilise around you.\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                        }
                        ident = 1 as libc::c_int as bool_
                    }
                }
                j += 1
            }
            if ident == 0 && (*p_ptr).word_recall as libc::c_int != 0 {
                msg_print(b"You feel like staying around.\x00" as *const u8 as
                              *const libc::c_char);
                (*p_ptr).word_recall = 0 as libc::c_int as s16b;
                ident = 1 as libc::c_int as bool_
            }
        }
        43 => {
            /* Trap of Silent Switching */
            let mut i_0: s16b = 0;
            let mut j_0: s16b = 0;
            let mut slot1: s16b = 0;
            let mut slot2: s16b = 0;
            let mut j_ptr_1: *mut object_type = 0 as *mut object_type;
            let mut k_ptr: *mut object_type = 0 as *mut object_type;
            let mut f1: u32b = 0;
            let mut f2: u32b = 0;
            let mut f3: u32b = 0;
            let mut f4: u32b = 0;
            let mut f5: u32b = 0;
            let mut esp: u32b = 0;
            i_0 = 24 as libc::c_int as s16b;
            while (i_0 as libc::c_int) < 52 as libc::c_int {
                j_ptr_1 =
                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(i_0 as isize)
                        as *mut object_type;
                if !((*j_ptr_1).k_idx == 0) {
                    /* Do not allow this trap to touch the One Ring */
                    object_flags(j_ptr_1, &mut f1, &mut f2, &mut f3, &mut f4,
                                 &mut f5, &mut esp);
                    if !(f3 as libc::c_long & 0x80000000 as libc::c_long != 0)
                       {
                        slot1 = wield_slot(j_ptr_1);
                        j_0 = 0 as libc::c_int as s16b;
                        while (j_0 as libc::c_int) < 24 as libc::c_int {
                            k_ptr =
                                &mut *(*p_ptr).inventory.as_mut_ptr().offset(j_0
                                                                                 as
                                                                                 isize)
                                    as *mut object_type;
                            if !((*k_ptr).k_idx == 0) {
                                /* Do not allow this trap to touch the One Ring */
                                object_flags(k_ptr, &mut f1, &mut f2, &mut f3,
                                             &mut f4, &mut f5, &mut esp);
                                if !(f3 as libc::c_long &
                                         0x80000000 as libc::c_long != 0) {
                                    /* this is a crude hack, but it prevent wielding 6 torches... */
                                    if !((*k_ptr).number as libc::c_int >
                                             1 as libc::c_int) {
                                        slot2 = wield_slot(k_ptr);
                                        /* a chance of 4 in 5 of switching something, then 2 in 5 to do it again */
                                        if slot1 as libc::c_int ==
                                               slot2 as libc::c_int &&
                                               Rand_div(100 as libc::c_int) <
                                                   80 as libc::c_int -
                                                       ident as libc::c_int *
                                                           40 as libc::c_int {
                                            let mut tmp_obj: object_type =
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
                                            if (*p_ptr).inventory[j_0 as
                                                                      usize].name1
                                                   != 0 {
                                                wield_set((*p_ptr).inventory[j_0
                                                                                 as
                                                                                 usize].name1
                                                              as s16b,
                                                          (*a_info.offset((*p_ptr).inventory[j_0
                                                                                                 as
                                                                                                 usize].name1
                                                                              as
                                                                              isize)).set,
                                                          0 as libc::c_int as
                                                              bool_);
                                            }
                                            if (*p_ptr).inventory[i_0 as
                                                                      usize].name1
                                                   != 0 {
                                                takeoff_set((*p_ptr).inventory[i_0
                                                                                   as
                                                                                   usize].name1
                                                                as s16b,
                                                            (*a_info.offset((*p_ptr).inventory[i_0
                                                                                                   as
                                                                                                   usize].name1
                                                                                as
                                                                                isize)).set);
                                            }
                                            tmp_obj =
                                                (*p_ptr).inventory[j_0 as
                                                                       usize];
                                            (*p_ptr).inventory[j_0 as usize] =
                                                (*p_ptr).inventory[i_0 as
                                                                       usize];
                                            (*p_ptr).inventory[i_0 as usize] =
                                                tmp_obj;
                                            ident = 1 as libc::c_int as bool_
                                        }
                                    }
                                }
                            }
                            j_0 += 1
                        }
                    }
                }
                i_0 += 1
            }
            if ident != 0 {
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as
                        u32b;
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as
                        u32b;
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long | 0x20 as libc::c_long)
                        as u32b;
                msg_print(b"You somehow feel like another person.\x00" as
                              *const u8 as *const libc::c_char);
            } else {
                msg_print(b"You feel a lack of useful items.\x00" as *const u8
                              as *const libc::c_char);
            }
        }
        44 => {
            /* Trap of Walls */
            ident = player_handle_trap_of_walls()
        }
        45 => {
            /* Trap of Calling Out */
            ident = do_player_trap_call_out();
            if ident == 0 {
                /* Increase "afraid" */
                if (*p_ptr).resist_fear != 0 {
                    msg_print(b"You feel as if you had a nightmare!\x00" as
                                  *const u8 as *const libc::c_char);
                } else if Rand_div(100 as libc::c_int) <
                              (*p_ptr).skill_sav as libc::c_int {
                    msg_print(b"You remember having a nightmare!\x00" as
                                  *const u8 as *const libc::c_char);
                } else if set_afraid((*p_ptr).afraid as libc::c_int +
                                         3 as libc::c_int +
                                         (Rand_div(40 as libc::c_int) +
                                              1 as libc::c_int)) != 0 {
                    msg_print(b"You have a vision of a powerful enemy.\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
        }
        47 => {
            /* Trap of Charges Drain */
            /* Find an item */
            k = 0 as libc::c_int as s16b;
            while (k as libc::c_int) < 10 as libc::c_int {
                let mut i_1: s16b = Rand_div(23 as libc::c_int) as s16b;
                let mut j_ptr_2: *mut object_type =
                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(i_1 as isize)
                        as *mut object_type;
                /* Drain charged wands/staffs
				   Hack -- don't let artifacts get drained */
                if ((*j_ptr_2).tval as libc::c_int == 55 as libc::c_int ||
                        (*j_ptr_2).tval as libc::c_int == 65 as libc::c_int)
                       && (*j_ptr_2).pval != 0 &&
                       !((*j_ptr_2).tval as libc::c_int == 102 as libc::c_int
                             ||
                             (if (*j_ptr_2).name1 as libc::c_int != 0 {
                                  1 as libc::c_int
                              } else { 0 as libc::c_int }) != 0 ||
                             (if (*j_ptr_2).art_name as libc::c_int != 0 {
                                  1 as libc::c_int
                              } else { 0 as libc::c_int }) != 0 ||
                             (if (*k_info.offset((*j_ptr_2).k_idx as
                                                     isize)).flags3 as
                                     libc::c_long & 0x8000 as libc::c_long !=
                                     0 {
                                  1 as libc::c_int
                              } else { 0 as libc::c_int }) != 0) {
                    ident = 1 as libc::c_int as bool_;
                    (*j_ptr_2).pval =
                        (*j_ptr_2).pval /
                            (Rand_div(4 as libc::c_int) + 1 as libc::c_int +
                                 1 as libc::c_int);
                    /* 60% chance of only 1 */
                    if Rand_div(10 as libc::c_int) + 1 as libc::c_int >
                           3 as libc::c_int {
                        break ;
                    }
                }
                k += 1
            }
            if ident != 0 {
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long | 0x1 as libc::c_long) as
                        u32b;
                /* Combine / Reorder the pack */
                (*p_ptr).notice =
                    ((*p_ptr).notice as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
                msg_print(b"Your backpack seems to be turned upside down.\x00"
                              as *const u8 as *const libc::c_char);
            } else {
                msg_print(b"You hear a wail of great disappointment.\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
        48 => {
            /* Trap of Stair Movement */
            let mut cx: s16b = 0; /* 20 stairs per level is enough? */
            let mut cy: s16b = 0;
            let mut i_2: s16b = 0;
            let mut j_1: s16b = 0;
            let mut cnt: s16b = 0 as libc::c_int as s16b;
            let mut cnt_seen: s16b = 0 as libc::c_int as s16b;
            let mut tmps: s16b = 0;
            let mut tmpx: s16b = 0;
            let mut tmpspecial: s16b = 0;
            let mut tmpspecial2: s16b = 0;
            let mut tmpf: u32b = 0;
            let mut seen: bool_ = 0 as libc::c_int as bool_;
            let mut index_x: [s16b; 20] = [0; 20];
            let mut index_y: [s16b; 20] = [0; 20];
            let mut cv_ptr: *mut cave_type = 0 as *mut cave_type;
            if *max_dlv.offset(dungeon_type as isize) as libc::c_int ==
                   99 as libc::c_int {
                /* no sense in relocating that stair! */
                msg_print(b"You have a feeling that this trap could be dangerous.\x00"
                              as *const u8 as *const libc::c_char);
            } else {
                cx = 0 as libc::c_int as s16b;
                while (cx as libc::c_int) < cur_wid as libc::c_int {
                    cy = 0 as libc::c_int as s16b;
                    while (cy as libc::c_int) < cur_hgt as libc::c_int {
                        cv_ptr =
                            &mut *(*cave.as_mut_ptr().offset(cy as
                                                                 isize)).offset(cx
                                                                                    as
                                                                                    isize)
                                as *mut cave_type;
                        if !((*cv_ptr).feat as libc::c_int !=
                                 0x6 as libc::c_int &&
                                 (*cv_ptr).feat as libc::c_int !=
                                     0x7 as libc::c_int &&
                                 (*cv_ptr).feat as libc::c_int !=
                                     0xe as libc::c_int &&
                                 (*cv_ptr).feat as libc::c_int !=
                                     0xd as libc::c_int) {
                            index_x[cnt as usize] = cx;
                            index_y[cnt as usize] = cy;
                            cnt += 1
                        }
                        cy += 1
                    }
                    cx += 1
                }
                if cnt as libc::c_int == 0 as libc::c_int {
                    if wizard != 0 {
                        msg_print(b"Executing moving stairs trap on level with no stairs!\x00"
                                      as *const u8 as *const libc::c_char);
                    }
                } else {
                    i_2 = 0 as libc::c_int as s16b;
                    while (i_2 as libc::c_int) < cnt as libc::c_int {
                        seen = 0 as libc::c_int as bool_;
                        j_1 = 0 as libc::c_int as s16b;
                        while (j_1 as libc::c_int) < 10 as libc::c_int {
                            /* try 10 times to relocate */
                            let mut cv_ptr_0: *mut cave_type =
                                &mut *(*cave.as_mut_ptr().offset(*index_y.as_mut_ptr().offset(i_2
                                                                                                  as
                                                                                                  isize)
                                                                     as
                                                                     isize)).offset(*index_x.as_mut_ptr().offset(i_2
                                                                                                                     as
                                                                                                                     isize)
                                                                                        as
                                                                                        isize)
                                    as *mut cave_type;
                            let mut cv_ptr2: *mut cave_type =
                                0 as *mut cave_type;
                            cx = Rand_div(cur_wid as s32b) as s16b;
                            cy = Rand_div(cur_hgt as s32b) as s16b;
                            if !(cx as libc::c_int ==
                                     index_x[i_2 as usize] as libc::c_int ||
                                     cy as libc::c_int ==
                                         index_y[i_2 as usize] as libc::c_int)
                               {
                                cv_ptr2 =
                                    &mut *(*cave.as_mut_ptr().offset(cy as
                                                                         isize)).offset(cx
                                                                                            as
                                                                                            isize)
                                        as *mut cave_type;
                                if !(cave_valid_bold(cy as libc::c_int,
                                                     cx as libc::c_int) == 0
                                         ||
                                         (*cv_ptr2).o_idx as libc::c_int !=
                                             0 as libc::c_int) {
                                    /* don't put anything in vaults */
                                    if !((*cv_ptr2).info as libc::c_int &
                                             0x4 as libc::c_int != 0) {
                                        tmpx = (*cv_ptr2).mimic as s16b;
                                        tmps = (*cv_ptr2).info as s16b;
                                        tmpf = (*cv_ptr2).feat as u32b;
                                        tmpspecial = (*cv_ptr2).special;
                                        tmpspecial2 = (*cv_ptr2).special2;
                                        (*cave[cy as
                                                   usize].offset(cx as
                                                                     isize)).mimic
                                            = (*cv_ptr_0).mimic;
                                        (*cave[cy as
                                                   usize].offset(cx as
                                                                     isize)).info
                                            = (*cv_ptr_0).info;
                                        (*cave[cy as
                                                   usize].offset(cx as
                                                                     isize)).special
                                            = (*cv_ptr_0).special;
                                        (*cave[cy as
                                                   usize].offset(cx as
                                                                     isize)).special2
                                            = (*cv_ptr_0).special2;
                                        cave_set_feat(cy as libc::c_int,
                                                      cx as libc::c_int,
                                                      (*cv_ptr_0).feat as
                                                          libc::c_int);
                                        (*cv_ptr_0).mimic = tmpx as byte_hack;
                                        (*cv_ptr_0).info = tmps as u16b;
                                        (*cv_ptr_0).special = tmpspecial;
                                        (*cv_ptr_0).special2 = tmpspecial2;
                                        cave_set_feat(index_y[i_2 as usize] as
                                                          libc::c_int,
                                                      index_x[i_2 as usize] as
                                                          libc::c_int,
                                                      tmpf as libc::c_int);
                                        /* if we are placing walls in rooms, make them rubble instead */
                                        if (*cv_ptr_0).info as libc::c_int &
                                               0x8 as libc::c_int != 0 &&
                                               (*cv_ptr_0).feat as libc::c_int
                                                   >= 0x38 as libc::c_int &&
                                               (*cv_ptr_0).feat as libc::c_int
                                                   <= 0x3f as libc::c_int {
                                            cave_set_feat(index_y[i_2 as
                                                                      usize]
                                                              as libc::c_int,
                                                          index_x[i_2 as
                                                                      usize]
                                                              as libc::c_int,
                                                          0x31 as
                                                              libc::c_int);
                                        }
                                        if (*cave[cy as
                                                      usize].offset(cx as
                                                                        isize)).info
                                               as libc::c_int &
                                               0x20 as libc::c_int !=
                                               0 as libc::c_int {
                                            note_spot(cy as libc::c_int,
                                                      cx as libc::c_int);
                                            lite_spot(cy as libc::c_int,
                                                      cx as libc::c_int);
                                            seen = 1 as libc::c_int as bool_
                                        } else {
                                            (*cv_ptr2).info =
                                                ((*cv_ptr2).info as
                                                     libc::c_int &
                                                     !(0x1 as libc::c_int)) as
                                                    u16b
                                        }
                                        if (*cave[index_y[i_2 as usize] as
                                                      usize].offset(index_x[i_2
                                                                                as
                                                                                usize]
                                                                        as
                                                                        isize)).info
                                               as libc::c_int &
                                               0x20 as libc::c_int !=
                                               0 as libc::c_int {
                                            note_spot(index_y[i_2 as usize] as
                                                          libc::c_int,
                                                      index_x[i_2 as usize] as
                                                          libc::c_int);
                                            lite_spot(index_y[i_2 as usize] as
                                                          libc::c_int,
                                                      index_x[i_2 as usize] as
                                                          libc::c_int);
                                            seen = 1 as libc::c_int as bool_
                                        } else {
                                            (*cv_ptr_0).info =
                                                ((*cv_ptr_0).info as
                                                     libc::c_int &
                                                     !(0x1 as libc::c_int)) as
                                                    u16b
                                        }
                                        break ;
                                    }
                                }
                            }
                            j_1 += 1
                        }
                        if seen != 0 { cnt_seen += 1 }
                        i_2 += 1
                    }
                    ident =
                        (cnt_seen as libc::c_int > 0 as libc::c_int) as
                            libc::c_int as bool_;
                    if ident as libc::c_int != 0 &&
                           cnt_seen as libc::c_int > 1 as libc::c_int {
                        msg_print(b"You see some stairs move.\x00" as
                                      *const u8 as *const libc::c_char);
                    } else if ident != 0 {
                        msg_print(b"You see a stair move.\x00" as *const u8 as
                                      *const libc::c_char);
                    } else {
                        msg_print(b"You hear distant scraping noises.\x00" as
                                      *const u8 as *const libc::c_char);
                    }
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x4000000 as libc::c_long) as u32b
                }
            }
        }
        49 => {
            /* Trap of New Trap */
            /* if we're on a floor or on a door, place a new trap */
            if item as libc::c_int == -(1 as libc::c_int) ||
                   item as libc::c_int == -(2 as libc::c_int) {
                place_trap(y as libc::c_int, x as libc::c_int);
                if (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x20 as libc::c_int != 0 as libc::c_int {
                    note_spot(y as libc::c_int, x as libc::c_int);
                    lite_spot(y as libc::c_int, x as libc::c_int);
                }
            } else {
                /* re-trap the chest */
                place_trap(y as libc::c_int, x as libc::c_int);
            }
            msg_print(b"You hear a noise, and then its echo.\x00" as *const u8
                          as *const libc::c_char);
            ident = 0 as libc::c_int as bool_
        }
        170 => {
            /* Trap of Acquirement */
            /* Get a nice thing */
            msg_print(b"You notice something falling off the trap.\x00" as
                          *const u8 as *const libc::c_char);
            acquirement(y as libc::c_int, x as libc::c_int, 1 as libc::c_int,
                        1 as libc::c_int as bool_, 0 as libc::c_int as bool_);
            /* If we're on a floor or on a door, place a new trap */
            if item as libc::c_int == -(1 as libc::c_int) ||
                   item as libc::c_int == -(2 as libc::c_int) {
                place_trap(y as libc::c_int, x as libc::c_int);
                if (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x20 as libc::c_int != 0 as libc::c_int {
                    note_spot(y as libc::c_int, x as libc::c_int);
                    lite_spot(y as libc::c_int, x as libc::c_int);
                }
            } else {
                /* Re-trap the chest */
                place_trap(y as libc::c_int, x as libc::c_int);
            }
            msg_print(b"You hear a noise, and then its echo.\x00" as *const u8
                          as *const libc::c_char);
            /* Never known */
            ident = 0 as libc::c_int as bool_
        }
        50 => {
            /* Trap of Scatter Items */
            let mut i_3: s16b = 0;
            let mut j_2: s16b = 0;
            let mut message: bool_ = 0 as libc::c_int as bool_;
            i_3 = 0 as libc::c_int as s16b;
            while (i_3 as libc::c_int) < 23 as libc::c_int {
                if !((*p_ptr).inventory[i_3 as usize].k_idx == 0) {
                    if !(Rand_div(10 as libc::c_int) < 3 as libc::c_int) {
                        j_2 = 0 as libc::c_int as s16b;
                        while (j_2 as libc::c_int) < 10 as libc::c_int {
                            let mut tmp_obj_0: object_type =
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
                            let mut j_ptr_3: *mut object_type =
                                &mut tmp_obj_0;
                            let mut cx_0: s16b =
                                (x as libc::c_int + 15 as libc::c_int -
                                     Rand_div(30 as libc::c_int)) as s16b;
                            let mut cy_0: s16b =
                                (y as libc::c_int + 15 as libc::c_int -
                                     Rand_div(30 as libc::c_int)) as s16b;
                            if cy_0 as libc::c_int > 0 as libc::c_int &&
                                   cx_0 as libc::c_int > 0 as libc::c_int &&
                                   (cy_0 as libc::c_int) <
                                       cur_hgt as libc::c_int -
                                           1 as libc::c_int &&
                                   (cx_0 as libc::c_int) <
                                       cur_wid as libc::c_int -
                                           1 as libc::c_int {
                                if (*f_info.offset((*cave[cy_0 as
                                                              usize].offset(cx_0
                                                                                as
                                                                                isize)).feat
                                                       as isize)).flags1 as
                                       libc::c_long & 0x10 as libc::c_long !=
                                       0 &&
                                       (*cave[cy_0 as
                                                  usize].offset(cx_0 as
                                                                    isize)).feat
                                           as libc::c_int !=
                                           0xaf as libc::c_int {
                                    object_copy(j_ptr_3,
                                                &mut *(*p_ptr).inventory.as_mut_ptr().offset(i_3
                                                                                                 as
                                                                                                 isize));
                                    inc_stack_size_ex(i_3 as libc::c_int,
                                                      -(999 as libc::c_int),
                                                      OPTIMIZE, NO_DESCRIBE);
                                    (*p_ptr).notice =
                                        ((*p_ptr).notice as libc::c_long |
                                             (0x1 as libc::c_long |
                                                  0x2 as libc::c_long)) as
                                            u32b;
                                    floor_carry(cy_0 as libc::c_int,
                                                cx_0 as libc::c_int, j_ptr_3);
                                    if message == 0 {
                                        msg_print(b"You feel light-footed.\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        message = 1 as libc::c_int as bool_
                                    }
                                    if (*cave[cy_0 as
                                                  usize].offset(cx_0 as
                                                                    isize)).info
                                           as libc::c_int &
                                           0x20 as libc::c_int !=
                                           0 as libc::c_int {
                                        let mut i_name_0: [libc::c_char; 80] =
                                            [0; 80];
                                        object_desc(i_name_0.as_mut_ptr(),
                                                    &mut tmp_obj_0,
                                                    1 as libc::c_int,
                                                    3 as libc::c_int);
                                        note_spot(cy_0 as libc::c_int,
                                                  cx_0 as libc::c_int);
                                        lite_spot(cy_0 as libc::c_int,
                                                  cx_0 as libc::c_int);
                                        ident = 1 as libc::c_int as bool_;
                                        msg_format(b"Suddenly %s appear%s!\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   i_name_0.as_mut_ptr(),
                                                   if (*j_ptr_3).number as
                                                          libc::c_int >
                                                          1 as libc::c_int {
                                                       b"\x00" as *const u8 as
                                                           *const libc::c_char
                                                   } else {
                                                       b"s\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                   });
                                    }
                                    break ;
                                }
                            }
                            j_2 += 1
                        }
                    }
                }
                i_3 += 1
            }
            ident = message
        }
        46 | 51 => { }
        52 => {
            /* Trap of Wasting Wands */
            let mut i_4: s16b = 0;
            let mut j_ptr_4: *mut object_type = 0 as *mut object_type;
            i_4 = 0 as libc::c_int as s16b;
            while (i_4 as libc::c_int) < 23 as libc::c_int {
                if !((*p_ptr).inventory[i_4 as usize].k_idx == 0) {
                    j_ptr_4 =
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset(i_4 as
                                                                         isize)
                            as *mut object_type;
                    if (*j_ptr_4).tval as libc::c_int == 65 as libc::c_int &&
                           Rand_div(5 as libc::c_int) == 1 as libc::c_int {
                        if (*j_ptr_4).ident as libc::c_int &
                               0x8 as libc::c_int != 0 ||
                               (*k_info.offset((*j_ptr_4).k_idx as
                                                   isize)).easy_know as
                                   libc::c_int != 0 &&
                                   (*k_info.offset((*j_ptr_4).k_idx as
                                                       isize)).aware as
                                       libc::c_int != 0 {
                            ident = 1 as libc::c_int as bool_
                        }
                        /* Create a Wand of Nothing */
                        object_prep(j_ptr_4,
                                    lookup_kind(65 as libc::c_int,
                                                2 as libc::c_int) as
                                        libc::c_int);
                        hack_apply_magic_power = -(99 as libc::c_int);
                        apply_magic(j_ptr_4, 0 as libc::c_int,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_);
                        (*j_ptr_4).ident =
                            ((*j_ptr_4).ident as libc::c_int &
                                 !(0x8 as libc::c_int)) as byte_hack;
                        (*p_ptr).notice =
                            ((*p_ptr).notice as libc::c_long |
                                 (0x1 as libc::c_long | 0x2 as libc::c_long))
                                as u32b
                    } else if (*j_ptr_4).tval as libc::c_int ==
                                  55 as libc::c_int &&
                                  Rand_div(5 as libc::c_int) ==
                                      1 as libc::c_int {
                        if (*j_ptr_4).ident as libc::c_int &
                               0x8 as libc::c_int != 0 ||
                               (*k_info.offset((*j_ptr_4).k_idx as
                                                   isize)).easy_know as
                                   libc::c_int != 0 &&
                                   (*k_info.offset((*j_ptr_4).k_idx as
                                                       isize)).aware as
                                       libc::c_int != 0 {
                            ident = 1 as libc::c_int as bool_
                        }
                        /* Create a Staff of Nothing */
                        object_prep(j_ptr_4,
                                    lookup_kind(55 as libc::c_int,
                                                2 as libc::c_int) as
                                        libc::c_int);
                        hack_apply_magic_power = -(99 as libc::c_int);
                        apply_magic(j_ptr_4, 0 as libc::c_int,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_);
                        (*j_ptr_4).ident =
                            ((*j_ptr_4).ident as libc::c_int &
                                 !(0x8 as libc::c_int)) as byte_hack;
                        (*p_ptr).notice =
                            ((*p_ptr).notice as libc::c_long |
                                 (0x1 as libc::c_long | 0x2 as libc::c_long))
                                as u32b
                    }
                }
                i_4 += 1
            }
            if ident != 0 {
                msg_print(b"You have lost trust in your backpack!\x00" as
                              *const u8 as *const libc::c_char);
            } else {
                msg_print(b"You hear an echoing cry of rage.\x00" as *const u8
                              as *const libc::c_char);
            }
        }
        53 => {
            /* Trap of Filling */
            let mut nx: s16b = 0;
            let mut ny: s16b = 0;
            nx = (x as libc::c_int - 8 as libc::c_int) as s16b;
            while nx as libc::c_int <= x as libc::c_int + 8 as libc::c_int {
                ny = (y as libc::c_int - 8 as libc::c_int) as s16b;
                while ny as libc::c_int <= y as libc::c_int + 8 as libc::c_int
                      {
                    if ny as libc::c_int > 0 as libc::c_int &&
                           nx as libc::c_int > 0 as libc::c_int &&
                           (ny as libc::c_int) <
                               cur_hgt as libc::c_int - 1 as libc::c_int &&
                           (nx as libc::c_int) <
                               cur_wid as libc::c_int - 1 as libc::c_int {
                        if Rand_div(distance(ny as libc::c_int,
                                             nx as libc::c_int,
                                             y as libc::c_int,
                                             x as libc::c_int)) >
                               3 as libc::c_int {
                            place_trap(ny as libc::c_int, nx as libc::c_int);
                        }
                    }
                    ny += 1
                }
                nx += 1
            }
            msg_print(b"The floor vibrates in a strange way.\x00" as *const u8
                          as *const libc::c_char);
            ident = 0 as libc::c_int as bool_
        }
        54 => {
            let mut j_ptr_5: *mut object_type = 0 as *mut object_type;
            let mut j_3: s16b = 0;
            let mut chance: s16b = 75 as libc::c_int as s16b;
            let mut f1_0: u32b = 0;
            let mut f2_0: u32b = 0;
            let mut f3_0: u32b = 0;
            let mut f4_0: u32b = 0;
            let mut f5_0: u32b = 0;
            let mut esp_0: u32b = 0;
            j_3 = 0 as libc::c_int as s16b;
            while (j_3 as libc::c_int) < 52 as libc::c_int {
                /* don't bother the overflow slot */
                if !(j_3 as libc::c_int == 23 as libc::c_int) {
                    if !((*p_ptr).inventory[j_3 as usize].k_idx == 0) {
                        j_ptr_5 =
                            &mut *(*p_ptr).inventory.as_mut_ptr().offset(j_3
                                                                             as
                                                                             isize)
                                as *mut object_type;
                        object_flags(j_ptr_5, &mut f1_0, &mut f2_0, &mut f3_0,
                                     &mut f4_0, &mut f5_0, &mut esp_0);
                        /* is it a non-artifact speed item? */
                        if (*j_ptr_5).name1 == 0 &&
                               f1_0 as libc::c_long & 0x1000 as libc::c_long
                                   != 0 {
                            if (Rand_div(100 as libc::c_int) +
                                    1 as libc::c_int) < chance as libc::c_int
                               {
                                (*j_ptr_5).pval =
                                    (*j_ptr_5).pval / 2 as libc::c_int;
                                if (*j_ptr_5).pval == 0 as libc::c_int {
                                    (*j_ptr_5).pval -= 1
                                }
                                chance =
                                    (chance as libc::c_int / 2 as libc::c_int)
                                        as s16b;
                                ident = 1 as libc::c_int as bool_
                            }
                            inven_item_optimize(j_3 as libc::c_int);
                        }
                    }
                }
                j_3 += 1
            }
            if ident == 0 {
                msg_print(b"You feel some things in your pack vibrating.\x00"
                              as *const u8 as *const libc::c_char);
            } else {
                combine_pack();
                reorder_pack();
                msg_print(b"You suddenly feel you have time for self-reflection.\x00"
                              as *const u8 as *const libc::c_char);
                /* Recalculate bonuses */
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as
                        u32b;
                /* Recalculate mana */
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long | 0x20 as libc::c_long)
                        as u32b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long |
                              0x8 as libc::c_long)) as u32b
            }
        }
        110 => {
            /*
		 * single missile traps
		 */
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           17 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           4 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Arrow Trap\x00" as *const u8 as
                                               *const libc::c_char)
        }
        111 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           18 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           5 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Bolt Trap\x00" as *const u8 as
                                               *const libc::c_char)
        }
        112 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           17 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           6 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Seeker Arrow Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        113 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           18 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           10 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Seeker Bolt Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        114 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           17 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           4 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           (10 as libc::c_int +
                                                (Rand_div(20 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Arrow Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        115 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           18 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           5 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           (15 as libc::c_int +
                                                (Rand_div(30 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Bolt Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        116 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           17 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           6 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           (30 as libc::c_int +
                                                (Rand_div(50 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Seeker Arrow Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        117 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           18 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           10 as libc::c_int as s16b,
                                           (40 as libc::c_int +
                                                (Rand_div(70 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Seeker Bolt Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        118 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           23 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Dagger Trap\x00" as *const u8 as
                                               *const libc::c_char)
        }
        119 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           23 as libc::c_int as s16b,
                                           4 as libc::c_int as s16b,
                                           3 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Dagger Trap\x00" as *const u8 as
                                               *const libc::c_char)
        }
        120 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           23 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           (15 as libc::c_int +
                                                (Rand_div(20 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Dagger Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        121 => {
            ident =
                player_handle_missile_trap(1 as libc::c_int as s16b,
                                           23 as libc::c_int as s16b,
                                           4 as libc::c_int as s16b,
                                           3 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           (20 as libc::c_int +
                                                (Rand_div(30 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Dagger Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        122 => {
            /*
		 * multiple missile traps
		 * numbers range from 2 (level 0 to 14) to 10 (level 120 and up)
		 */
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           17 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           4 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Arrow Trap\x00" as *const u8 as
                                               *const libc::c_char)
        }
        123 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           18 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           5 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Bolt Trap\x00" as *const u8 as
                                               *const libc::c_char)
        }
        124 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           17 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           6 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Seeker Arrow Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        125 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           18 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           10 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Seeker Bolt Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        126 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           17 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           4 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           (10 as libc::c_int +
                                                (Rand_div(20 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Arrow Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        127 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           18 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           5 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           (15 as libc::c_int +
                                                (Rand_div(30 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Bolt Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        128 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           17 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           6 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           (30 as libc::c_int +
                                                (Rand_div(50 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Seeker Arrow Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        129 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           18 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           10 as libc::c_int as s16b,
                                           (40 as libc::c_int +
                                                (Rand_div(70 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Seeker Bolt Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        130 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           23 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Dagger Trap\x00" as *const u8 as
                                               *const libc::c_char)
        }
        131 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           23 as libc::c_int as s16b,
                                           4 as libc::c_int as s16b,
                                           3 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           0 as libc::c_int as s16b,
                                           b"Dagger Trap\x00" as *const u8 as
                                               *const libc::c_char)
        }
        132 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           23 as libc::c_int as s16b,
                                           1 as libc::c_int as s16b,
                                           2 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           (15 as libc::c_int +
                                                (Rand_div(20 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Dagger Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        133 => {
            ident =
                player_handle_missile_trap((2 as libc::c_int +
                                                *max_dlv.offset(dungeon_type
                                                                    as isize)
                                                    as libc::c_int /
                                                    15 as libc::c_int) as
                                               s16b,
                                           23 as libc::c_int as s16b,
                                           4 as libc::c_int as s16b,
                                           3 as libc::c_int as s16b,
                                           8 as libc::c_int as s16b,
                                           (20 as libc::c_int +
                                                (Rand_div(30 as libc::c_int) +
                                                     1 as libc::c_int)) as
                                               s16b,
                                           b"Poison Dagger Trap\x00" as
                                               *const u8 as
                                               *const libc::c_char)
        }
        140 => {
            let mut i_5: s16b = 0;
            let mut message_0: bool_ = 0 as libc::c_int as bool_;
            i_5 = 0 as libc::c_int as s16b;
            while (i_5 as libc::c_int) < 23 as libc::c_int {
                let mut tmp_obj_1: object_type =
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
                if !((*p_ptr).inventory[i_5 as usize].k_idx == 0) {
                    if !((Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                             80 as libc::c_int) {
                        if !((*p_ptr).inventory[i_5 as usize].name1 as
                                 libc::c_int == 13 as libc::c_int) {
                            tmp_obj_1 = (*p_ptr).inventory[i_5 as usize];
                            /* drop carefully */
                            drop_near(&mut tmp_obj_1, 0 as libc::c_int,
                                      y as libc::c_int, x as libc::c_int);
                            inc_stack_size_ex(i_5 as libc::c_int,
                                              -(999 as libc::c_int), OPTIMIZE,
                                              NO_DESCRIBE);
                            (*p_ptr).notice =
                                ((*p_ptr).notice as libc::c_long |
                                     (0x1 as libc::c_long |
                                          0x2 as libc::c_long)) as u32b;
                            if message_0 == 0 {
                                msg_print(b"You are startled by a sudden sound.\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                message_0 = 1 as libc::c_int as bool_
                            }
                            ident = 1 as libc::c_int as bool_
                        }
                    }
                }
                i_5 += 1
            }
            if ident == 0 {
                msg_print(b"You hear a sudden, strange sound.\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
        141 => {
            let mut i_6: s16b = 0;
            let mut message_1: bool_ = 0 as libc::c_int as bool_;
            i_6 = 0 as libc::c_int as s16b;
            while (i_6 as libc::c_int) < 23 as libc::c_int {
                let mut tmp_obj_2: object_type =
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
                if !((*p_ptr).inventory[i_6 as usize].k_idx == 0) {
                    if !((Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                             10 as libc::c_int) {
                        if !((*p_ptr).inventory[i_6 as usize].name1 as
                                 libc::c_int == 13 as libc::c_int) {
                            tmp_obj_2 = (*p_ptr).inventory[i_6 as usize];
                            /* drop carefully */
                            drop_near(&mut tmp_obj_2, 0 as libc::c_int,
                                      y as libc::c_int, x as libc::c_int);
                            inc_stack_size_ex(i_6 as libc::c_int,
                                              -(999 as libc::c_int), OPTIMIZE,
                                              NO_DESCRIBE);
                            (*p_ptr).notice =
                                ((*p_ptr).notice as libc::c_long |
                                     (0x1 as libc::c_long |
                                          0x2 as libc::c_long)) as u32b;
                            if message_1 == 0 {
                                msg_print(b"You are greatly startled by a sudden sound.\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                message_1 = 1 as libc::c_int as bool_
                            }
                            ident = 1 as libc::c_int as bool_
                        }
                    }
                }
                i_6 += 1
            }
            if ident == 0 {
                msg_print(b"You hear a sudden, strange sound.\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
        142 => {
            let mut i_7: s16b = 0;
            let mut message_2: bool_ = 0 as libc::c_int as bool_;
            i_7 = 0 as libc::c_int as s16b;
            while (i_7 as libc::c_int) < 52 as libc::c_int {
                let mut tmp_obj_3: object_type =
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
                if !((*p_ptr).inventory[i_7 as usize].k_idx == 0) {
                    if !((Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                             30 as libc::c_int) {
                        if !((*p_ptr).inventory[i_7 as usize].name1 as
                                 libc::c_int == 13 as libc::c_int) {
                            tmp_obj_3 = (*p_ptr).inventory[i_7 as usize];
                            /* drop carefully */
                            drop_near(&mut tmp_obj_3, 0 as libc::c_int,
                                      y as libc::c_int, x as libc::c_int);
                            inc_stack_size_ex(i_7 as libc::c_int,
                                              -(999 as libc::c_int), OPTIMIZE,
                                              NO_DESCRIBE);
                            (*p_ptr).notice =
                                ((*p_ptr).notice as libc::c_long |
                                     (0x1 as libc::c_long |
                                          0x2 as libc::c_long)) as u32b;
                            if message_2 == 0 {
                                msg_print(b"You are completely startled by a sudden sound.\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                message_2 = 1 as libc::c_int as bool_
                            }
                            ident = 1 as libc::c_int as bool_
                        }
                    }
                }
                i_7 += 1
            }
            if ident == 0 {
                msg_print(b"You hear a sudden, strange sound.\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
        171 => {
            /* Bolt Trap */
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          1 as libc::c_int as s16b,
                                          171 as libc::c_int as u16b)
        }
        172 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          2 as libc::c_int as s16b,
                                          172 as libc::c_int as u16b)
        }
        173 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          3 as libc::c_int as s16b,
                                          173 as libc::c_int as u16b)
        }
        174 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          4 as libc::c_int as s16b,
                                          174 as libc::c_int as u16b)
        }
        175 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          5 as libc::c_int as s16b,
                                          175 as libc::c_int as u16b)
        }
        60 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          1 as libc::c_int as s16b,
                                          60 as libc::c_int as u16b)
        }
        61 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          2 as libc::c_int as s16b,
                                          61 as libc::c_int as u16b)
        }
        62 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          3 as libc::c_int as s16b,
                                          62 as libc::c_int as u16b)
        }
        63 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          4 as libc::c_int as s16b,
                                          63 as libc::c_int as u16b)
        }
        64 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          5 as libc::c_int as s16b,
                                          64 as libc::c_int as u16b)
        }
        65 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          12 as libc::c_int as s16b,
                                          65 as libc::c_int as u16b)
        }
        66 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          14 as libc::c_int as s16b,
                                          66 as libc::c_int as u16b)
        }
        67 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          15 as libc::c_int as s16b,
                                          67 as libc::c_int as u16b)
        }
        68 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          16 as libc::c_int as s16b,
                                          68 as libc::c_int as u16b)
        }
        69 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          20 as libc::c_int as s16b,
                                          69 as libc::c_int as u16b)
        }
        70 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          21 as libc::c_int as s16b,
                                          70 as libc::c_int as u16b)
        }
        71 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          22 as libc::c_int as s16b,
                                          71 as libc::c_int as u16b)
        }
        72 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          23 as libc::c_int as s16b,
                                          72 as libc::c_int as u16b)
        }
        73 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          24 as libc::c_int as s16b,
                                          73 as libc::c_int as u16b)
        }
        74 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          26 as libc::c_int as s16b,
                                          74 as libc::c_int as u16b)
        }
        75 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          28 as libc::c_int as s16b,
                                          75 as libc::c_int as u16b)
        }
        76 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          30 as libc::c_int as s16b,
                                          76 as libc::c_int as u16b)
        }
        77 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          31 as libc::c_int as s16b,
                                          77 as libc::c_int as u16b)
        }
        78 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          32 as libc::c_int as s16b,
                                          78 as libc::c_int as u16b)
        }
        79 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          33 as libc::c_int as s16b,
                                          79 as libc::c_int as u16b)
        }
        80 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          34 as libc::c_int as s16b,
                                          80 as libc::c_int as u16b)
        }
        81 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          35 as libc::c_int as s16b,
                                          81 as libc::c_int as u16b)
        }
        82 => {
            /* Ball Trap */
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          1 as libc::c_int as s16b,
                                          82 as libc::c_int as u16b)
        }
        83 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          2 as libc::c_int as s16b,
                                          83 as libc::c_int as u16b)
        }
        84 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          3 as libc::c_int as s16b,
                                          84 as libc::c_int as u16b)
        }
        85 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          4 as libc::c_int as s16b,
                                          85 as libc::c_int as u16b)
        }
        86 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          5 as libc::c_int as s16b,
                                          86 as libc::c_int as u16b)
        }
        87 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          12 as libc::c_int as s16b,
                                          87 as libc::c_int as u16b)
        }
        88 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          14 as libc::c_int as s16b,
                                          88 as libc::c_int as u16b)
        }
        89 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          15 as libc::c_int as s16b,
                                          89 as libc::c_int as u16b)
        }
        90 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          16 as libc::c_int as s16b,
                                          90 as libc::c_int as u16b)
        }
        91 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          20 as libc::c_int as s16b,
                                          91 as libc::c_int as u16b)
        }
        92 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          21 as libc::c_int as s16b,
                                          92 as libc::c_int as u16b)
        }
        93 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          22 as libc::c_int as s16b,
                                          93 as libc::c_int as u16b)
        }
        94 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          23 as libc::c_int as s16b,
                                          94 as libc::c_int as u16b)
        }
        95 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          24 as libc::c_int as s16b,
                                          95 as libc::c_int as u16b)
        }
        96 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          26 as libc::c_int as s16b,
                                          96 as libc::c_int as u16b)
        }
        97 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          28 as libc::c_int as s16b,
                                          97 as libc::c_int as u16b)
        }
        98 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          30 as libc::c_int as s16b,
                                          98 as libc::c_int as u16b)
        }
        99 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          31 as libc::c_int as s16b,
                                          99 as libc::c_int as u16b)
        }
        100 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          32 as libc::c_int as s16b,
                                          100 as libc::c_int as u16b)
        }
        101 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          33 as libc::c_int as s16b,
                                          101 as libc::c_int as u16b)
        }
        102 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          34 as libc::c_int as s16b,
                                          102 as libc::c_int as u16b)
        }
        103 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          35 as libc::c_int as s16b,
                                          103 as libc::c_int as u16b)
        }
        150 => {
            /* -SC- */
            msg_print(b"Gas sprouts out... you feel yourself transmute.\x00"
                          as *const u8 as *const libc::c_char);
            (*p_ptr).psex = 0 as libc::c_int as byte_hack;
            sp_ptr =
                &mut *sex_info.as_mut_ptr().offset((*p_ptr).psex as isize) as
                    *mut player_sex;
            ident = 1 as libc::c_int as bool_;
            trap_hit(trap);
        }
        151 => {
            msg_print(b"Gas sprouts out... you feel yourself transmute.\x00"
                          as *const u8 as *const libc::c_char);
            (*p_ptr).psex = 1 as libc::c_int as byte_hack;
            sp_ptr =
                &mut *sex_info.as_mut_ptr().offset((*p_ptr).psex as isize) as
                    *mut player_sex;
            ident = 1 as libc::c_int as bool_;
            trap_hit(trap);
        }
        152 => {
            msg_print(b"Gas sprouts out... you feel yourself transmute.\x00"
                          as *const u8 as *const libc::c_char);
            (*p_ptr).psex = 2 as libc::c_int as byte_hack;
            sp_ptr =
                &mut *sex_info.as_mut_ptr().offset((*p_ptr).psex as isize) as
                    *mut player_sex;
            ident = 1 as libc::c_int as bool_;
            trap_hit(trap);
        }
        153 => {
            msg_print(b"Colors are scintillating around you. You see your past running before your eyes.\x00"
                          as *const u8 as *const libc::c_char);
            (*p_ptr).age =
                ((*p_ptr).age as libc::c_int +
                     (Rand_div(((*rp_ptr).b_age as libc::c_int +
                                    (*rmp_ptr).b_age as libc::c_int) /
                                   2 as libc::c_int) + 1 as libc::c_int)) as
                    s16b;
            ident = 1 as libc::c_int as bool_;
            trap_hit(trap);
        }
        154 => {
            let mut tmp: s16b = 0;
            msg_print(b"Heavy fumes sprout out... you feel yourself transmute.\x00"
                          as *const u8 as *const libc::c_char);
            if (*p_ptr).psex as libc::c_int == 0 as libc::c_int {
                tmp =
                    ((*rp_ptr).f_b_ht as libc::c_int +
                         (*rmp_ptr).f_b_ht as libc::c_int) as s16b
            } else {
                tmp =
                    ((*rp_ptr).m_b_ht as libc::c_int +
                         (*rmp_ptr).m_b_ht as libc::c_int) as s16b
            }
            (*p_ptr).ht =
                ((*p_ptr).ht as libc::c_int +
                     (Rand_div(tmp as libc::c_int / 4 as libc::c_int) +
                          1 as libc::c_int)) as s16b;
            ident = 1 as libc::c_int as bool_;
            trap_hit(trap);
        }
        155 => {
            let mut tmp_0: s16b = 0;
            msg_print(b"Heavy fumes sprout out... you feel yourself transmute.\x00"
                          as *const u8 as *const libc::c_char);
            if (*p_ptr).psex as libc::c_int == 0 as libc::c_int {
                tmp_0 =
                    ((*rp_ptr).f_b_ht as libc::c_int +
                         (*rmp_ptr).f_b_ht as libc::c_int) as s16b
            } else {
                tmp_0 =
                    ((*rp_ptr).m_b_ht as libc::c_int +
                         (*rmp_ptr).m_b_ht as libc::c_int) as s16b
            }
            (*p_ptr).ht =
                ((*p_ptr).ht as libc::c_int -
                     (Rand_div(tmp_0 as libc::c_int / 4 as libc::c_int) +
                          1 as libc::c_int)) as s16b;
            if (*p_ptr).ht as libc::c_int <=
                   tmp_0 as libc::c_int / 4 as libc::c_int {
                (*p_ptr).ht =
                    (tmp_0 as libc::c_int / 4 as libc::c_int) as s16b
            }
            ident = 1 as libc::c_int as bool_;
            trap_hit(trap);
        }
        158 => {
            /* Trap of Divine Anger */
            if (*p_ptr).pgod as libc::c_int == 0 as libc::c_int {
                msg_format(b"Suddenly you feel glad you\'re a mere %s\x00" as
                               *const u8 as *const libc::c_char,
                           c_name.offset((*spp_ptr).title as isize));
            } else {
                let mut name: cptr = 0 as *const libc::c_char;
                name = (*deity_info.offset((*p_ptr).pgod as isize)).name;
                msg_format(b"You feel you have angered %s.\x00" as *const u8
                               as *const libc::c_char, name);
                inc_piety((*p_ptr).pgod as libc::c_int,
                          -(3000 as libc::c_int));
            }
        }
        159 => {
            /* Trap of Divine Wrath */
            if (*p_ptr).pgod as libc::c_int == 0 as libc::c_int {
                msg_format(b"Suddenly you feel glad you\'re a mere %s\x00" as
                               *const u8 as *const libc::c_char,
                           c_name.offset((*spp_ptr).title as isize));
            } else {
                let mut name_0: cptr = 0 as *const libc::c_char;
                name_0 = (*deity_info.offset((*p_ptr).pgod as isize)).name;
                msg_format(b"%s quakes in rage: ``Thou art supremely insolent, mortal!!\'\'\x00"
                               as *const u8 as *const libc::c_char, name_0);
                inc_piety((*p_ptr).pgod as libc::c_int,
                          -(500 as libc::c_int) *
                              (*p_ptr).lev as libc::c_int);
            }
        }
        160 => {
            /* Trap of hallucination */
            msg_print(b"Scintillating colors hypnotise you for a moment.\x00"
                          as *const u8 as *const libc::c_char);
            set_image(80 as libc::c_int);
        }
        161 => {
            /* Bolt Trap */
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          72 as libc::c_int as s16b,
                                          trap as u16b)
        }
        162 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          73 as libc::c_int as s16b,
                                          trap as u16b)
        }
        164 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          79 as libc::c_int as s16b,
                                          trap as u16b)
        }
        165 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          80 as libc::c_int as s16b,
                                          trap as u16b)
        }
        166 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          85 as libc::c_int as s16b,
                                          trap as u16b)
        }
        167 => {
            ident =
                player_handle_breath_trap(1 as libc::c_int as s16b,
                                          86 as libc::c_int as s16b,
                                          trap as u16b)
        }
        168 => {
            /* Ball Trap */
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          73 as libc::c_int as s16b,
                                          168 as libc::c_int as u16b)
        }
        169 => {
            ident =
                player_handle_breath_trap(3 as libc::c_int as s16b,
                                          85 as libc::c_int as s16b,
                                          168 as libc::c_int as u16b)
        }
        _ => {
            msg_print(format(b"Executing unknown trap %d\x00" as *const u8 as
                                 *const libc::c_char, trap as libc::c_int) as
                          cptr);
        }
    }
    return ident;
}
#[no_mangle]
pub unsafe extern "C" fn player_activate_door_trap(mut y: s16b, mut x: s16b) {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut ident: bool_ = 0 as libc::c_int as bool_;
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Return if trap or door not found */
    if (*c_ptr).t_idx as libc::c_int == 0 as libc::c_int ||
           (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
               0x1000 as libc::c_long == 0 {
        return
    }
    /* Disturb */
    disturb(0 as libc::c_int, 0 as libc::c_int);
    /* Message */
    msg_print(b"You found a trap!\x00" as *const u8 as *const libc::c_char);
    /* Pick a trap */
    pick_trap(y as libc::c_int, x as libc::c_int);
    /* Hit the trap */
    ident =
        player_activate_trap_type(y, x, 0 as *mut object_type,
                                  -(1 as libc::c_int) as s16b);
    if ident != 0 {
        (*t_info.offset((*c_ptr).t_idx as isize)).ident =
            1 as libc::c_int as bool_;
        msg_format(b"You identified that trap as %s.\x00" as *const u8 as
                       *const libc::c_char,
                   t_name.offset((*t_info.offset((*c_ptr).t_idx as
                                                     isize)).name as
                                     libc::c_int as isize));
    };
}
/*
 * Places a random trap at the given location.
 *
 * The location must be a valid, empty, clean, floor grid.
 */
#[no_mangle]
pub unsafe extern "C" fn place_trap(mut y: libc::c_int, mut x: libc::c_int) {
    let mut trap: s16b = 0;
    let mut t_ptr: *mut trap_type = 0 as *mut trap_type;
    let mut cnt: libc::c_int = 0;
    let mut flags: u32b = 0;
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    /* No traps in town or on first level */
    if dun_level as libc::c_int <= 1 as libc::c_int { return }
    /*
	 * Avoid open doors -- because DOOR flag is added to make much more
	 * important processing faster
	 */
    if (*c_ptr).feat as libc::c_int == 0x4 as libc::c_int { return }
    if (*c_ptr).feat as libc::c_int == 0x5 as libc::c_int { return }
    /* Traps only appears on empty floor */
    if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
             0x10 as libc::c_long != 0 &&
             (*c_ptr).feat as libc::c_int != 0xaf as libc::c_int) &&
           (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
               0x1000 as libc::c_long == 0 {
        return
    }
    /* Set flags */
    if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
           0x1000 as libc::c_long != 0 {
        flags = 0x2 as libc::c_int as u32b
    } else { flags = 0x4 as libc::c_int as u32b }
    /* Try 100 times */
    cnt = 100 as libc::c_int;
    loop  {
        let fresh0 = cnt;
        cnt = cnt - 1;
        if !(fresh0 != 0) { break ; }
        trap =
            (Rand_div(max_t_idx as libc::c_int - 1 as libc::c_int) +
                 1 as libc::c_int) as s16b;
        t_ptr = &mut *t_info.offset(trap as isize) as *mut trap_type;
        /* No traps below their minlevel */
        if (*t_ptr).minlevel as libc::c_int > dun_level as libc::c_int {
            continue ;
        }
        /* is this a correct trap now?   */
        if (*t_ptr).flags & flags == 0 { continue ; }
        /*
		 * Hack -- No trap door at the bottom of dungeon or in flat
		 * (non dungeon) places or on quest levels
		 */
        if trap as libc::c_int == 39 as libc::c_int &&
               ((*d_ptr).maxdepth as libc::c_int == dun_level as libc::c_int
                    ||
                    dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long
                        != 0 || is_quest(dun_level as libc::c_int) != 0) {
            continue ;
        }
        /* How probable is this trap */
        if !(Rand_div(100 as libc::c_int) <
                 (*t_ptr).probability as libc::c_int) {
            continue ;
        }
        (*c_ptr).t_idx = trap;
        break ;
    };
}
/*
 * Places a random trap on the given chest.
 *
 * The object must be a valid chest.
 */
#[no_mangle]
pub unsafe extern "C" fn place_trap_object(mut o_ptr: *mut object_type) {
    let mut trap: s16b = 0;
    let mut t_ptr: *mut trap_type = 0 as *mut trap_type;
    let mut cnt: libc::c_int = 0;
    /* No traps in town or on first level */
    if dun_level as libc::c_int <= 1 as libc::c_int {
        /* empty chest were already looted, therefore known */
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x8 as libc::c_int) as byte_hack;
        return
    }
    /* Try 100 times */
    cnt = 100 as libc::c_int;
    loop  {
        let fresh1 = cnt;
        cnt = cnt - 1;
        if !(fresh1 != 0) { break ; }
        trap =
            (Rand_div(max_t_idx as libc::c_int - 1 as libc::c_int) +
                 1 as libc::c_int) as s16b;
        t_ptr = &mut *t_info.offset(trap as isize) as *mut trap_type;
        /* no traps below their minlevel */
        if (*t_ptr).minlevel as libc::c_int > dun_level as libc::c_int {
            continue ;
        }
        /* Is this a correct trap now? */
        if (*t_ptr).flags & 0x1 as libc::c_int as libc::c_uint == 0 {
            continue ;
        }
        /* How probable is this trap */
        if !(Rand_div(100 as libc::c_int) <
                 (*t_ptr).probability as libc::c_int) {
            continue ;
        }
        (*o_ptr).pval = trap as s32b;
        break ;
    };
}
/* Dangerous trap placing function */
#[no_mangle]
pub unsafe extern "C" fn wiz_place_trap(mut y: libc::c_int,
                                        mut x: libc::c_int,
                                        mut idx: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Dangerous enough as it is... */
    if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
             0x10 as libc::c_long != 0 &&
             (*c_ptr).feat as libc::c_int != 0xaf as libc::c_int) &&
           (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
               0x1000 as libc::c_long == 0 {
        return
    }
    (*c_ptr).t_idx = idx as s16b;
}
/*
 * Here begin monster traps code
 */
/*
 * Hook to determine if an object is a device
 */
unsafe extern "C" fn item_tester_hook_device(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int == 67 as libc::c_int &&
           (*o_ptr).pval != 0 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 55 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
/*
 * Hook to determine if an object is a potion
 */
unsafe extern "C" fn item_tester_hook_potion(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int == 71 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 72 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
/*
 * The trap setting code for rogues -MWK-
 *
 * Also, it will fail or give weird results if the tvals are resorted!
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_set_trap() {
    let mut item_kit: libc::c_int = 0;
    let mut item_load: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    let mut i_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut c: cptr = 0 as *const libc::c_char;
    let mut object_type_body: object_type =
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
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Check some conditions */
    if (*p_ptr).blind != 0 {
        msg_print(b"You can\'t see anything.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if no_lite() != 0 {
        msg_print(b"You don\'t dare to set a trap in the darkness.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Only set traps on clean floor grids */
    if !((*f_info.offset((*cave[(*p_ptr).py as
                                    usize].offset((*p_ptr).px as isize)).feat
                             as isize)).flags1 as libc::c_long &
             0x10 as libc::c_long != 0 &&
             (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat
                 as libc::c_int != 0xaf as libc::c_int &&
             (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).o_idx
                 as libc::c_int == 0 as libc::c_int &&
             (*f_info.offset((*cave[(*p_ptr).py as
                                        usize].offset((*p_ptr).px as
                                                          isize)).feat as
                                 isize)).flags1 as libc::c_long &
                 0x40 as libc::c_long == 0) {
        msg_print(b"You cannot set a trap on this.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Restrict choices to trapkits */
    item_tester_tval = 46 as libc::c_int as byte_hack;
    /* Get an item */
    q = b"Use which trapping kit? \x00" as *const u8 as *const libc::c_char;
    s = b"You have no trapping kits.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item_kit, q, s, 0x2 as libc::c_int) == 0 { return }
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item_kit as isize) as
            *mut object_type;
    /* Trap kits need a second object */
    match (*o_ptr).sval as libc::c_int {
        2 => { item_tester_tval = 17 as libc::c_int as byte_hack }
        3 => { item_tester_tval = 18 as libc::c_int as byte_hack }
        1 => { item_tester_tval = 16 as libc::c_int as byte_hack }
        4 => {
            item_tester_hook =
                Some(item_tester_hook_potion as
                         unsafe extern "C" fn(_: *mut object_type) -> bool_)
        }
        5 => { item_tester_tval = 70 as libc::c_int as byte_hack }
        6 => {
            item_tester_hook =
                Some(item_tester_hook_device as
                         unsafe extern "C" fn(_: *mut object_type) -> bool_)
        }
        _ => {
            msg_print(b"Unknown trapping kit type!\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    /* Get the second item */
    q = b"Load with what? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to load that trap with.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item_load, q, s, 0x2 as libc::c_int) == 0 { return }
    /* Get the second object */
    j_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item_load as isize) as
            *mut object_type;
    /* Assume a single object */
    num = 1 as libc::c_int;
    /* In some cases, take multiple objects to load */
    if (*o_ptr).sval as libc::c_int != 6 as libc::c_int {
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        if f3 as libc::c_long & 0x80000 as libc::c_long != 0 &&
               (*o_ptr).pval > 0 as libc::c_int {
            num += (*o_ptr).pval
        }
        if f2 as libc::c_long & (0x1 as libc::c_long | 0x2 as libc::c_long) !=
               0 {
            num = 99 as libc::c_int
        }
        if num > (*j_ptr).number as libc::c_int {
            num = (*j_ptr).number as libc::c_int
        }
        c =
            format(b"How many (1-%d)? \x00" as *const u8 as
                       *const libc::c_char, num) as cptr;
        /* Ask for number of items to use */
        num = get_quantity(c, num)
    }
    /* Canceled */
    if num == 0 { return }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Get local object */
    i_ptr = &mut object_type_body;
    /* Obtain local object for trap content */
    object_copy(i_ptr, j_ptr);
    /* Set number */
    (*i_ptr).number = num as byte_hack;
    /* Drop it here */
    (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).special =
        floor_carry((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                    i_ptr);
    /* Obtain local object for trap trigger kit */
    object_copy(i_ptr, o_ptr);
    /* Set number */
    (*i_ptr).number = 1 as libc::c_int as byte_hack;
    /* Drop it here */
    (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).special2 =
        floor_carry((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                    i_ptr);
    /* Modify, Describe, Optimize */
    inc_stack_size_ex(item_kit, -(1 as libc::c_int), NO_OPTIMIZE, DESCRIBE);
    inc_stack_size_ex(item_load, -num, NO_OPTIMIZE, DESCRIBE);
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        if inven_item_optimize(i) != 0 { break ; }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int { inven_item_optimize(i); i += 1 }
    /* Actually set the trap */
    cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                  0xaf as libc::c_int);
}
/*
 * Monster hitting a rod trap -MWK-
 *
 * Return TRUE if the monster died
 */
#[no_mangle]
pub unsafe extern "C" fn mon_hit_trap_aux_rod(mut m_idx: libc::c_int,
                                              mut o_ptr: *mut object_type)
 -> bool_ {
    let mut dam: libc::c_int = 0 as libc::c_int;
    let mut typ: libc::c_int = 0 as libc::c_int;
    let mut rad: libc::c_int = 0 as libc::c_int;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut y: libc::c_int = (*m_ptr).fy as libc::c_int;
    let mut x: libc::c_int = (*m_ptr).fx as libc::c_int;
    /* Depend on rod type */
    match (*o_ptr).pval {
        29 => { (*m_ptr).smart |= 0x800000 as libc::c_int as libc::c_uint }
        6 => { (*m_ptr).smart |= 0x800000 as libc::c_int as libc::c_uint }
        4 => {
            typ = 17 as libc::c_int; /* and heal conf? */
            dam =
                damroll(2 as libc::c_int as s16b, 15 as libc::c_int as s16b);
            rad = 3 as libc::c_int;
            lite_room(y, x);
        }
        8 => {
            typ = 53 as libc::c_int;
            dam = damroll(3 as libc::c_int as s16b, 4 as libc::c_int as s16b)
        }
        9 => { typ = 53 as libc::c_int; dam = 300 as libc::c_int }
        11 => { typ = 54 as libc::c_int; dam = 50 as libc::c_int }
        13 => {
            typ = 63 as libc::c_int;
            dam = 20 as libc::c_int * 5 as libc::c_int
        }
        14 => { }
        15 => {
            typ = 17 as libc::c_int;
            dam = damroll(6 as libc::c_int as s16b, 8 as libc::c_int as s16b)
        }
        16 => { typ = 57 as libc::c_int; dam = 50 as libc::c_int }
        17 => { typ = 55 as libc::c_int; dam = 50 as libc::c_int }
        18 => { typ = 58 as libc::c_int; dam = 75 as libc::c_int }
        19 => { typ = 52 as libc::c_int; dam = 50 as libc::c_int }
        20 => {
            typ = 3 as libc::c_int;
            dam = damroll(6 as libc::c_int as s16b, 8 as libc::c_int as s16b)
        }
        21 => {
            typ = 1 as libc::c_int;
            dam = damroll(3 as libc::c_int as s16b, 8 as libc::c_int as s16b)
        }
        22 => {
            typ = 5 as libc::c_int;
            dam = damroll(8 as libc::c_int as s16b, 8 as libc::c_int as s16b)
        }
        23 => {
            typ = 4 as libc::c_int;
            dam = damroll(5 as libc::c_int as s16b, 8 as libc::c_int as s16b)
        }
        24 => {
            typ = 3 as libc::c_int;
            dam = 60 as libc::c_int;
            rad = 2 as libc::c_int
        }
        25 => {
            typ = 1 as libc::c_int;
            dam = 32 as libc::c_int;
            rad = 2 as libc::c_int
        }
        26 => {
            typ = 5 as libc::c_int;
            dam = 72 as libc::c_int;
            rad = 2 as libc::c_int
        }
        27 => {
            typ = 4 as libc::c_int;
            dam = 48 as libc::c_int;
            rad = 2 as libc::c_int
        }
        _ => { return 0 as libc::c_int as bool_ }
    }
    /* Actually hit the monster */
    if typ != 0 {
        project(-(2 as libc::c_int), rad, y, x, dam, typ,
                0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x1 as libc::c_int);
    }
    return if (*cave[y as usize].offset(x as isize)).m_idx as libc::c_int ==
                  0 as libc::c_int {
               1 as libc::c_int
           } else { 0 as libc::c_int } as bool_;
}
/*
 * Monster hitting a staff trap -MWK-
 *
 * Return TRUE if the monster died
 */
#[no_mangle]
pub unsafe extern "C" fn mon_hit_trap_aux_staff(mut m_idx: libc::c_int,
                                                mut o_ptr: *mut object_type)
 -> bool_ {
    return 0 as libc::c_int as bool_;
}
/*
 * Monster hitting a scroll trap -MWK-
 *
 * Return TRUE if the monster died
 */
#[no_mangle]
pub unsafe extern "C" fn mon_hit_trap_aux_scroll(mut m_idx: libc::c_int,
                                                 mut sval: libc::c_int)
 -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut dam: libc::c_int = 0 as libc::c_int;
    let mut typ: libc::c_int = 0 as libc::c_int;
    let mut rad: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = (*m_ptr).fy as libc::c_int;
    let mut x: libc::c_int = (*m_ptr).fx as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut current_block_52: u64;
    /* Depend on scroll type */
    match sval {
        11 => {
            /* should these? */
            current_block_52 = 8090765090562817671;
        }
        2 | 3 | 7 | 12 | 13 | 25 | 26 | 27 | 14 | 15 | 16 | 17 | 18 | 20 | 21
        | 22 | 29 | 30 | 32 | 38 | 39 | 37 => {
            current_block_52 = 8090765090562817671;
        }
        0 => {
            unlite_room(y, x);
            typ = 18 as libc::c_int;
            dam = 10 as libc::c_int;
            rad = 3 as libc::c_int;
            current_block_52 = 12930649117290160518;
        }
        1 => { aggravate_monsters(m_idx); return 0 as libc::c_int as bool_ }
        4 => {
            k = 0 as libc::c_int;
            while k < Rand_div(3 as libc::c_int) + 1 as libc::c_int {
                summon_specific(y, x, dun_level as libc::c_int,
                                0 as libc::c_int);
                k += 1
            }
            return 0 as libc::c_int as bool_
        }
        5 => {
            k = 0 as libc::c_int;
            while k < Rand_div(3 as libc::c_int) + 1 as libc::c_int {
                summon_specific(y, x, dun_level as libc::c_int,
                                17 as libc::c_int);
                k += 1
            }
            return 0 as libc::c_int as bool_
        }
        8 => {
            typ = 63 as libc::c_int;
            dam = 10 as libc::c_int;
            current_block_52 = 12930649117290160518;
        }
        9 => {
            typ = 63 as libc::c_int;
            dam = 100 as libc::c_int;
            current_block_52 = 12930649117290160518;
        }
        10 => { delete_monster(y, x); return 1 as libc::c_int as bool_ }
        24 => {
            lite_room(y, x);
            typ = 17 as libc::c_int;
            dam = damroll(2 as libc::c_int as s16b, 8 as libc::c_int as s16b);
            rad = 2 as libc::c_int;
            current_block_52 = 12930649117290160518;
        }
        28 => {
            (*m_ptr).smart |= 0x800000 as libc::c_int as libc::c_uint;
            return 0 as libc::c_int as bool_
        }
        33 => {
            typ = 79 as libc::c_int;
            dam = damroll(1 as libc::c_int as s16b, 4 as libc::c_int as s16b);
            current_block_52 = 12930649117290160518;
        }
        34 => {
            typ = 79 as libc::c_int;
            dam = damroll(2 as libc::c_int as s16b, 4 as libc::c_int as s16b);
            current_block_52 = 12930649117290160518;
        }
        35 => {
            typ = 79 as libc::c_int;
            dam = damroll(4 as libc::c_int as s16b, 4 as libc::c_int as s16b);
            current_block_52 = 12930649117290160518;
        }
        36 => {
            typ = 56 as libc::c_int;
            dam =
                damroll(5 as libc::c_int as s16b, 10 as libc::c_int as s16b);
            current_block_52 = 12930649117290160518;
        }
        41 => {
            destroy_area(y, x, 15 as libc::c_int, 1 as libc::c_int as bool_,
                         0 as libc::c_int as bool_);
            return 0 as libc::c_int as bool_
        }
        42 => {
            typ = 67 as libc::c_int;
            rad = 5 as libc::c_int;
            dam = 60 as libc::c_int;
            current_block_52 = 12930649117290160518;
        }
        44 => {
            let mut r_ptr: *mut monster_race =
                &mut *r_info.offset((*m_ptr).r_idx as isize) as
                    *mut monster_race;
            genocide_aux(0 as libc::c_int as bool_, (*r_ptr).d_char);
            /* although there's no point in a multiple genocide trap... */
            return ((*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0)
                       as libc::c_int as bool_
        }
        45 => {
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                delete_monster(y + ddy[k as usize] as libc::c_int,
                               x + ddx[k as usize] as libc::c_int);
                k += 1
            }
            delete_monster(y, x);
            return 1 as libc::c_int as bool_
        }
        46 => {
            acquirement(y, x, 1 as libc::c_int, 1 as libc::c_int as bool_,
                        0 as libc::c_int as bool_);
            return 0 as libc::c_int as bool_
        }
        47 => {
            acquirement(y, x,
                        Rand_div(2 as libc::c_int) + 1 as libc::c_int +
                            1 as libc::c_int, 1 as libc::c_int as bool_,
                        0 as libc::c_int as bool_);
            return 0 as libc::c_int as bool_
        }
        _ => { return 0 as libc::c_int as bool_ }
    }
    match current_block_52 {
        12930649117290160518 => { }
        _ =>
        /* these don't work :-( */
        {
            return 0 as libc::c_int as bool_
        }
    }
    /* Actually hit the monster */
    project(-(2 as libc::c_int), rad, y, x, dam, typ,
            0x40 as libc::c_int | 0x20 as libc::c_int | 0x1 as libc::c_int);
    return if (*cave[y as usize].offset(x as isize)).m_idx as libc::c_int ==
                  0 as libc::c_int {
               1 as libc::c_int
           } else { 0 as libc::c_int } as bool_;
}
/*
 * Monster hitting a wand trap -MWK-
 *
 * Return TRUE if the monster died
 */
#[no_mangle]
pub unsafe extern "C" fn mon_hit_trap_aux_wand(mut m_idx: libc::c_int,
                                               mut o_ptr: *mut object_type)
 -> bool_ {
    return 0 as libc::c_int as bool_;
}
/*
 * Monster hitting a potions trap -MWK-
 *
 * Return TRUE if the monster died
 */
#[no_mangle]
pub unsafe extern "C" fn mon_hit_trap_aux_potion(mut m_idx: libc::c_int,
                                                 mut o_ptr: *mut object_type)
 -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut dam: libc::c_int = 0 as libc::c_int;
    let mut typ: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = (*m_ptr).fy as libc::c_int;
    let mut x: libc::c_int = (*m_ptr).fx as libc::c_int;
    let mut sval: libc::c_int = (*o_ptr).sval as libc::c_int;
    /* Depend on potion type */
    if (*o_ptr).tval as libc::c_int == 71 as libc::c_int {
        let mut current_block_46: u64;
        match sval {
            15 => {
                /* ??? */
                current_block_46 = 9658365607264005620;
            }
            0 | 1 | 2 | 5 | 16 | 17 | 18 | 19 | 20 | 21 | 24 | 25 | 26 | 27 |
            30 | 31 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 |
            51 | 52 | 53 | 55 | 56 | 57 | 58 => {
                current_block_46 = 9658365607264005620;
            }
            59 => {
                if ((*m_ptr).level as libc::c_int) < 150 as libc::c_int {
                    (*m_ptr).exp =
                        ((if (*m_ptr).level as libc::c_int + 1 as libc::c_int
                                 > 150 as libc::c_int {
                              150 as libc::c_int
                          } else {
                              ((*m_ptr).level as libc::c_int) +
                                  1 as libc::c_int
                          }) *
                             (if (*m_ptr).level as libc::c_int +
                                     1 as libc::c_int > 150 as libc::c_int {
                                  150 as libc::c_int
                              } else {
                                  ((*m_ptr).level as libc::c_int) +
                                      1 as libc::c_int
                              }) *
                             (if (*m_ptr).level as libc::c_int +
                                     1 as libc::c_int > 150 as libc::c_int {
                                  150 as libc::c_int
                              } else {
                                  ((*m_ptr).level as libc::c_int) +
                                      1 as libc::c_int
                              }) * 6 as libc::c_int) as u32b;
                    monster_check_experience(m_idx,
                                             0 as libc::c_int as bool_);
                }
                return 0 as libc::c_int as bool_
            }
            4 => {
                typ = 55 as libc::c_int;
                dam =
                    damroll(4 as libc::c_int as s16b,
                            6 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            6 => {
                typ = 2 as libc::c_int;
                dam =
                    damroll(8 as libc::c_int as s16b,
                            6 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            9 => {
                typ = 22 as libc::c_int;
                dam =
                    damroll(4 as libc::c_int as s16b,
                            6 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            7 => {
                typ = 16 as libc::c_int;
                dam = 10 as libc::c_int;
                current_block_46 = 13707613154239713890;
            }
            11 => {
                typ = 57 as libc::c_int;
                dam =
                    damroll(4 as libc::c_int as s16b,
                            6 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            13 => {
                typ = 56 as libc::c_int;
                dam =
                    damroll(10 as libc::c_int as s16b,
                            10 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            22 => {
                typ = 81 as libc::c_int;
                dam =
                    damroll(20 as libc::c_int as s16b,
                            20 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            23 => {
                typ = 31 as libc::c_int;
                dam =
                    damroll(100 as libc::c_int as s16b,
                            20 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            28 => {
                (*m_ptr).monfear = 0 as libc::c_int as byte_hack;
                return 0 as libc::c_int as bool_
            }
            29 => {
                typ = 54 as libc::c_int;
                dam =
                    damroll(5 as libc::c_int as s16b,
                            10 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            32 | 33 => {
                (*m_ptr).monfear = 0 as libc::c_int as byte_hack;
                typ = 53 as libc::c_int;
                dam =
                    damroll(2 as libc::c_int as s16b,
                            10 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            34 => {
                typ = 53 as libc::c_int;
                dam =
                    damroll(3 as libc::c_int as s16b,
                            4 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            35 => {
                typ = 53 as libc::c_int;
                dam =
                    damroll(4 as libc::c_int as s16b,
                            6 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            36 => {
                typ = 53 as libc::c_int;
                dam =
                    damroll(6 as libc::c_int as s16b,
                            8 as libc::c_int as s16b);
                current_block_46 = 13707613154239713890;
            }
            37 => {
                typ = 53 as libc::c_int;
                dam = 300 as libc::c_int;
                current_block_46 = 13707613154239713890;
            }
            38 => {
                typ = 53 as libc::c_int;
                dam = 1000 as libc::c_int;
                current_block_46 = 13707613154239713890;
            }
            39 => {
                let mut r_ptr: *mut monster_race =
                    &mut *r_info.offset((*m_ptr).r_idx as isize) as
                        *mut monster_race;
                if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
                   {
                    typ = 79 as libc::c_int;
                    dam =
                        damroll(20 as libc::c_int as s16b,
                                20 as libc::c_int as s16b)
                } else { typ = 53 as libc::c_int; dam = 5000 as libc::c_int }
                current_block_46 = 13707613154239713890;
            }
            _ => { return 0 as libc::c_int as bool_ }
        }
        match current_block_46 {
            13707613154239713890 => { }
            _ =>
            /* Nothing happens */
            {
                return 0 as libc::c_int as bool_
            }
        }
    }
    /* Actually hit the monster */
    project_m(-(2 as libc::c_int), 0 as libc::c_int, y, x, dam, typ);
    return if (*cave[y as usize].offset(x as isize)).m_idx as libc::c_int ==
                  0 as libc::c_int {
               1 as libc::c_int
           } else { 0 as libc::c_int } as bool_;
}
/*
 * Monster hitting a monster trap -MWK-
 * Returns True if the monster died, false otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn mon_hit_trap(mut m_idx: libc::c_int) -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*m_ptr).r_idx as isize) as *mut monster_race;
    let mut kit_o_ptr: *mut object_type = 0 as *mut object_type;
    let mut load_o_ptr: *mut object_type = 0 as *mut object_type;
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut object_type_body: object_type =
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
    let mut mx: libc::c_int = (*m_ptr).fx as libc::c_int;
    let mut my: libc::c_int = (*m_ptr).fy as libc::c_int;
    let mut difficulty: libc::c_int = 0;
    let mut smartness: libc::c_int = 0;
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    let mut disarm: bool_ = 0 as libc::c_int as bool_;
    let mut remove: bool_ = 0 as libc::c_int as bool_;
    let mut dead: bool_ = 0 as libc::c_int as bool_;
    let mut fear: bool_ = 0 as libc::c_int as bool_;
    let mut special: s32b = 0 as libc::c_int;
    let mut dam: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut shots: libc::c_int = 0;
    let mut mul: libc::c_int = 0 as libc::c_int;
    let mut breakage: libc::c_int = -(1 as libc::c_int);
    let mut cost: libc::c_int = 0 as libc::c_int;
    /* Get the trap objects */
    kit_o_ptr =
        &mut *o_list.offset((*(*cave.as_mut_ptr().offset(my as
                                                             isize)).offset(mx
                                                                                as
                                                                                isize)).special2
                                as isize) as *mut object_type;
    load_o_ptr =
        &mut *o_list.offset((*(*cave.as_mut_ptr().offset(my as
                                                             isize)).offset(mx
                                                                                as
                                                                                isize)).special
                                as isize) as *mut object_type;
    j_ptr = &mut object_type_body;
    /* Get trap properties */
    object_flags(kit_o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Can set off check */
	/* Ghosts only set off Ghost traps */
    if (*r_ptr).flags2 & 0x40000 as libc::c_int as libc::c_uint != 0 &&
           f2 as libc::c_long & 0x4 as libc::c_long == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Some traps are specialized to some creatures */
    if f2 as libc::c_long &
           (0x10 as libc::c_long | 0x20 as libc::c_long |
                0x100 as libc::c_long | 0x200 as libc::c_long |
                0x400 as libc::c_long) != 0 {
        let mut affect: bool_ = 0 as libc::c_int as bool_;
        if f2 as libc::c_long & 0x10 as libc::c_long != 0 &&
               (*r_ptr).flags3 & 0x8 as libc::c_int as libc::c_uint != 0 {
            affect = 1 as libc::c_int as bool_
        }
        if f2 as libc::c_long & 0x20 as libc::c_long != 0 &&
               (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 {
            affect = 1 as libc::c_int as bool_
        }
        if f2 as libc::c_long & 0x200 as libc::c_long != 0 &&
               (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
            affect = 1 as libc::c_int as bool_
        }
        if f2 as libc::c_long & 0x400 as libc::c_long != 0 &&
               (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0 {
            affect = 1 as libc::c_int as bool_
        }
        if f2 as libc::c_long & 0x100 as libc::c_long != 0 &&
               (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 {
            affect = 1 as libc::c_int as bool_
        }
        /* Don't set it off if forbidden */
        if affect == 0 { return 0 as libc::c_int as bool_ }
    }
    /* Get detection difficulty */
    difficulty = 25 as libc::c_int;
    /* Some traps are well-hidden */
    if f1 as libc::c_long & 0x100 as libc::c_long != 0 {
        difficulty += 10 as libc::c_int * (*kit_o_ptr).pval
    }
    /* Get monster smartness for trap detection */
	/* Higher level monsters are smarter */
    smartness = (*r_ptr).level as libc::c_int;
    /* Smart monsters are better at detecting traps */
    if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint != 0 {
        smartness += 10 as libc::c_int
    }
    /* Some monsters have already noticed one of out traps */
    if (*m_ptr).smart & 0x800000 as libc::c_int as libc::c_uint != 0 {
        smartness += 20 as libc::c_int
    }
    /* Stupid monsters are no good at detecting traps */
    if (*r_ptr).flags2 &
           (0x1 as libc::c_int | 0x40 as libc::c_int) as libc::c_uint != 0 {
        smartness = -(150 as libc::c_int)
    }
    /* Check if the monster notices the trap */
    if Rand_div(300 as libc::c_int) + 1 as libc::c_int >
           difficulty - smartness + 150 as libc::c_int {
        notice = 1 as libc::c_int as bool_
    }
    /* Disarm check */
    if notice != 0 {
        /* The next traps will be easier to spot! */
        (*m_ptr).smart |= 0x800000 as libc::c_int as libc::c_uint;
        /* Get trap disarming difficulty */
        difficulty =
            (*kit_o_ptr).ac as libc::c_int + (*kit_o_ptr).to_a as libc::c_int;
        /* Get monster disarming ability */
		/* Higher level monsters are better */
        smartness = (*r_ptr).level as libc::c_int / 5 as libc::c_int;
        /* Smart monsters are better at disarming */
        if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint != 0 {
            smartness *= 2 as libc::c_int
        }
        /* Stupid monsters never disarm traps */
        if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 {
            smartness = -(150 as libc::c_int)
        }
        /* Nonsmart animals never disarm traps */
        if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 &&
               (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint == 0 {
            smartness = -(150 as libc::c_int)
        }
        /* Check if the monster disarms the trap */
        if Rand_div(120 as libc::c_int) + 1 as libc::c_int >
               difficulty - smartness + 80 as libc::c_int {
            disarm = 1 as libc::c_int as bool_
        }
    }
    /* If disarmed, remove the trap and print a message */
    if disarm != 0 {
        remove = 1 as libc::c_int as bool_;
        if (*m_ptr).ml != 0 {
            /* Get the name */
            monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
            /* Print a message */
            msg_format(b"%^s disarms a trap!\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr());
        }
    } else {
        /* Otherwise, activate the trap! */
        /* Message for visible monster */
        if (*m_ptr).ml != 0 {
            /* Get the name */
            monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
            /* Print a message */
            msg_format(b"%^s sets off a trap!\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr());
        }
        if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
               80 as libc::c_int {
            (*m_ptr).smart |= 0x800000 as libc::c_int as libc::c_uint
        }
        match (*kit_o_ptr).sval as libc::c_int {
            2 | 3 | 1 => {
                /* Next time be more careful */
                /* Get number of shots */
                shots = 1 as libc::c_int;
                if f3 as libc::c_long & 0x80000 as libc::c_long != 0 {
                    shots += (*kit_o_ptr).pval
                }
                if shots <= 0 as libc::c_int { shots = 1 as libc::c_int }
                if shots > (*load_o_ptr).number as libc::c_int {
                    shots = (*load_o_ptr).number as libc::c_int
                }
                loop  {
                    let fresh2 = shots;
                    shots = shots - 1;
                    if !(fresh2 != 0 && dead == 0) { break ; }
                    /* Total base damage */
                    dam =
                        damroll((*load_o_ptr).dd as s16b,
                                (*load_o_ptr).ds as s16b) +
                            (*load_o_ptr).to_d as libc::c_int +
                            (*kit_o_ptr).to_d as libc::c_int;
                    /* Total hit probability */
                    chance =
                        ((*kit_o_ptr).to_h as libc::c_int +
                             (*load_o_ptr).to_h as libc::c_int +
                             20 as libc::c_int) * 3 as libc::c_int;
                    /* Damage multiplier */
                    if (*kit_o_ptr).sval as libc::c_int == 2 as libc::c_int {
                        mul = 3 as libc::c_int
                    }
                    if (*kit_o_ptr).sval as libc::c_int == 3 as libc::c_int {
                        mul = 4 as libc::c_int
                    }
                    if (*kit_o_ptr).sval as libc::c_int == 1 as libc::c_int {
                        mul = 2 as libc::c_int
                    }
                    if f3 as libc::c_long & 0x40000 as libc::c_long != 0 {
                        mul += (*kit_o_ptr).pval
                    }
                    if mul < 0 as libc::c_int { mul = 0 as libc::c_int }
                    /* Multiply damage */
                    dam *= mul;
                    /* Check if we hit the monster */
                    if test_hit_fire(chance, (*r_ptr).ac as libc::c_int,
                                     1 as libc::c_int) != 0 {
                        /* Assume a default death */
                        let mut note_dies: cptr =
                            b" dies.\x00" as *const u8 as *const libc::c_char;
                        /* Some monsters get "destroyed" */
                        if (*r_ptr).flags3 &
                               0x10 as libc::c_int as libc::c_uint != 0 ||
                               (*r_ptr).flags3 &
                                   0x20 as libc::c_int as libc::c_uint != 0 ||
                               (*r_ptr).flags2 &
                                   0x1 as libc::c_int as libc::c_uint != 0 ||
                               !strchr(b"Evg\x00" as *const u8 as
                                           *const libc::c_char,
                                       (*r_ptr).d_char as
                                           libc::c_int).is_null() {
                            /* Special note at death */
                            note_dies =
                                b" is destroyed.\x00" as *const u8 as
                                    *const libc::c_char
                        }
                        /* Message if visible */
                        if (*m_ptr).ml != 0 {
                            /* describe the monster (again, just in case :-) */
                            monster_desc(m_name.as_mut_ptr(), m_ptr,
                                         0 as libc::c_int);
                            /* Print a message */
                            msg_format(b"%^s is hit by a missile.\x00" as
                                           *const u8 as *const libc::c_char,
                                       m_name.as_mut_ptr());
                        }
                        /* Apply slays, brand, critical hits */
                        dam =
                            tot_dam_aux(load_o_ptr, dam, m_ptr, &mut special)
                                as libc::c_int;
                        dam =
                            critical_shot((*load_o_ptr).weight,
                                          (*load_o_ptr).to_h as libc::c_int,
                                          dam, 23 as libc::c_int) as
                                libc::c_int;
                        /* No negative damage */
                        if dam < 0 as libc::c_int { dam = 0 as libc::c_int }
                        /* Hit the monster, check for death */
                        if mon_take_hit(m_idx, dam, &mut fear, note_dies) != 0
                           {
                            /* Dead monster */
                            dead = 1 as libc::c_int as bool_
                        } else {
                            /* No death */
                            /* Message */
                            message_pain(m_idx, dam);
                            if special != 0 {
                                attack_special(m_ptr, special, dam);
                            }
                            if fear as libc::c_int != 0 &&
                                   (*m_ptr).ml as libc::c_int != 0 {
                                /* Message */
                                msg_format(b"%^s flees in terror!\x00" as
                                               *const u8 as
                                               *const libc::c_char,
                                           m_name.as_mut_ptr());
                            }
                        }
                    }
                    /* Take note */
                    /* Exploding ammo */
                    if (*load_o_ptr).pval2 as libc::c_int != 0 as libc::c_int
                       {
                        let mut rad: libc::c_int = 0 as libc::c_int;
                        let mut dam_0: libc::c_int =
                            (damroll((*load_o_ptr).dd as s16b,
                                     (*load_o_ptr).ds as s16b) +
                                 (*load_o_ptr).to_d as libc::c_int) *
                                2 as libc::c_int;
                        let mut flag: libc::c_int =
                            0x8 as libc::c_int | 0x10 as libc::c_int |
                                0x20 as libc::c_int | 0x40 as libc::c_int |
                                0x1 as libc::c_int;
                        match (*load_o_ptr).sval as libc::c_int {
                            0 => {
                                rad = 2 as libc::c_int;
                                dam_0 /= 2 as libc::c_int
                            }
                            1 => { rad = 3 as libc::c_int }
                            2 => {
                                rad = 4 as libc::c_int;
                                dam_0 *= 2 as libc::c_int
                            }
                            _ => { }
                        }
                        project(0 as libc::c_int, rad, my, mx, dam_0,
                                (*load_o_ptr).pval2 as libc::c_int, flag);
                        breakage = 100 as libc::c_int
                    } else { breakage = breakage_chance(load_o_ptr) }
                    /* Copy and decrease ammo */
                    object_copy(j_ptr, load_o_ptr);
                    (*j_ptr).number = 1 as libc::c_int as byte_hack;
                    (*load_o_ptr).number =
                        (*load_o_ptr).number.wrapping_sub(1);
                    if (*load_o_ptr).number as libc::c_int <= 0 as libc::c_int
                       {
                        remove = 1 as libc::c_int as bool_;
                        delete_object_idx((*kit_o_ptr).next_o_idx as
                                              libc::c_int);
                        (*kit_o_ptr).next_o_idx = 0 as libc::c_int as s16b
                    }
                    /* Drop (or break) near that location */
                    drop_near(j_ptr, breakage, my, mx);
                }
            }
            4 => {
                /* Get number of shots */
                shots = 1 as libc::c_int;
                if f3 as libc::c_long & 0x80000 as libc::c_long != 0 {
                    shots += (*kit_o_ptr).pval
                }
                if shots <= 0 as libc::c_int { shots = 1 as libc::c_int }
                if shots > (*load_o_ptr).number as libc::c_int {
                    shots = (*load_o_ptr).number as libc::c_int
                }
                loop  {
                    let fresh3 = shots;
                    shots = shots - 1;
                    if !(fresh3 != 0 && dead == 0) { break ; }
                    /* Message if visible */
                    if (*m_ptr).ml != 0 {
                        /* describe the monster (again, just in case :-) */
                        monster_desc(m_name.as_mut_ptr(), m_ptr,
                                     0 as libc::c_int);
                        /* Print a message */
                        msg_format(b"%^s is hit by fumes.\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    /* Get the potion effect */
                    dead = mon_hit_trap_aux_potion(m_idx, load_o_ptr);
                    /* Copy and decrease ammo */
                    object_copy(j_ptr, load_o_ptr);
                    (*j_ptr).number = 1 as libc::c_int as byte_hack;
                    (*load_o_ptr).number =
                        (*load_o_ptr).number.wrapping_sub(1);
                    if (*load_o_ptr).number as libc::c_int <= 0 as libc::c_int
                       {
                        remove = 1 as libc::c_int as bool_;
                        delete_object_idx((*kit_o_ptr).next_o_idx as
                                              libc::c_int);
                        (*kit_o_ptr).next_o_idx = 0 as libc::c_int as s16b
                    }
                }
            }
            5 => {
                /* Get number of shots */
                shots = 1 as libc::c_int;
                if f3 as libc::c_long & 0x80000 as libc::c_long != 0 {
                    shots += (*kit_o_ptr).pval
                }
                if shots <= 0 as libc::c_int { shots = 1 as libc::c_int }
                if shots > (*load_o_ptr).number as libc::c_int {
                    shots = (*load_o_ptr).number as libc::c_int
                }
                loop  {
                    let fresh4 = shots;
                    shots = shots - 1;
                    if !(fresh4 != 0 && dead == 0) { break ; }
                    /* Message if visible */
                    if (*m_ptr).ml != 0 {
                        /* describe the monster (again, just in case :-) */
                        monster_desc(m_name.as_mut_ptr(), m_ptr,
                                     0 as libc::c_int);
                        /* Print a message */
                        msg_format(b"%^s activates a spell!\x00" as *const u8
                                       as *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                    /* Get the potion effect */
                    dead =
                        mon_hit_trap_aux_scroll(m_idx,
                                                (*load_o_ptr).sval as
                                                    libc::c_int);
                    /* Copy and decrease ammo */
                    object_copy(j_ptr, load_o_ptr);
                    (*j_ptr).number = 1 as libc::c_int as byte_hack;
                    (*load_o_ptr).number =
                        (*load_o_ptr).number.wrapping_sub(1);
                    if (*load_o_ptr).number as libc::c_int <= 0 as libc::c_int
                       {
                        remove = 1 as libc::c_int as bool_;
                        delete_object_idx((*kit_o_ptr).next_o_idx as
                                              libc::c_int);
                        (*kit_o_ptr).next_o_idx = 0 as libc::c_int as s16b
                    }
                }
            }
            6 => {
                if (*load_o_ptr).tval as libc::c_int == 67 as libc::c_int {
                    /* Extract mana cost of the rod tip */
                    let mut tf1: u32b = 0;
                    let mut tf2: u32b = 0;
                    let mut tf3: u32b = 0;
                    let mut tf4: u32b = 0;
                    let mut tf5: u32b = 0;
                    let mut tesp: u32b = 0;
                    let mut tip_o_ptr: *mut object_kind =
                        &mut *k_info.offset((lookup_kind as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int,
                                                                      _:
                                                                          libc::c_int)
                                                     ->
                                                         s16b)(66 as
                                                                   libc::c_int,
                                                               (*load_o_ptr).pval)
                                                as isize) as *mut object_kind;
                    object_flags(load_o_ptr, &mut tf1, &mut tf2, &mut tf3,
                                 &mut tf4, &mut tf5, &mut tesp);
                    cost =
                        if tf4 as libc::c_long & 0x8000 as libc::c_long != 0 {
                            ((*tip_o_ptr).pval) / 2 as libc::c_int
                        } else { (*tip_o_ptr).pval };
                    if cost <= 0 as libc::c_int { cost = 1 as libc::c_int }
                }
                /* Get number of shots */
                shots = 1 as libc::c_int;
                if f3 as libc::c_long & 0x80000 as libc::c_long != 0 {
                    shots += (*kit_o_ptr).pval
                }
                if shots <= 0 as libc::c_int { shots = 1 as libc::c_int }
                if (*load_o_ptr).tval as libc::c_int == 67 as libc::c_int {
                    if shots > (*load_o_ptr).timeout as libc::c_int / cost {
                        shots = (*load_o_ptr).timeout as libc::c_int / cost
                    }
                } else if shots > (*load_o_ptr).pval {
                    shots = (*load_o_ptr).pval
                }
                loop  {
                    let fresh5 = shots;
                    shots = shots - 1;
                    if !(fresh5 != 0 && dead == 0) { break ; }
                    /* Get the effect effect */
                    match (*load_o_ptr).tval as libc::c_int {
                        67 => {
                            dead = mon_hit_trap_aux_rod(m_idx, load_o_ptr)
                        }
                        65 => {
                            dead = mon_hit_trap_aux_wand(m_idx, load_o_ptr)
                        }
                        55 => {
                            dead = mon_hit_trap_aux_staff(m_idx, load_o_ptr)
                        }
                        _ => { }
                    }
                    if (*load_o_ptr).tval as libc::c_int == 67 as libc::c_int
                       {
                        /* decrease stored mana (timeout) for rods */
                        (*load_o_ptr).timeout =
                            ((*load_o_ptr).timeout as libc::c_int - cost) as
                                s16b
                    } else {
                        /* decrease charges for wands and staves */
                        (*load_o_ptr).pval -= 1
                    }
                }
            }
            _ => {
                msg_print(b"oops! nonexistant trap!\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        if f2 as libc::c_long & (0x1 as libc::c_long | 0x2 as libc::c_long) ==
               0 {
            remove = 1 as libc::c_int as bool_
        } else if f2 as libc::c_long & 0x1 as libc::c_long != 0 {
            remove =
                (Rand_div(5 as libc::c_int) + 1 as libc::c_int ==
                     1 as libc::c_int) as libc::c_int as bool_
        }
    }
    /* Actually activate the trap */
    /* Non-automatic traps are removed */
    /* Special trap effect -- teleport to */
    if f2 as libc::c_long & 0x8 as libc::c_long != 0 && disarm == 0 &&
           dead == 0 {
        teleport_monster_to(m_idx, (*p_ptr).py as libc::c_int,
                            (*p_ptr).px as libc::c_int);
    }
    /* Remove the trap if inactive now */
    if remove != 0 { place_floor_convert_glass(my, mx); }
    /* did it die? */
    return dead;
}
