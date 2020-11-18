use ::libc;
extern "C" {
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut ddx_ddd: [s16b; 9];
    #[no_mangle]
    static mut ddy_ddd: [s16b; 9];
    #[no_mangle]
    static mut adj_str_blow: [byte_hack; 0];
    #[no_mangle]
    static mut adj_dex_safe: [byte_hack; 0];
    #[no_mangle]
    static mut between_exits: [between_exit; 2];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut command_rep: s16b;
    #[no_mangle]
    static mut command_dir: s16b;
    #[no_mangle]
    static mut command_new: s16b;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut create_up_stair: bool_;
    #[no_mangle]
    static mut create_down_stair: bool_;
    #[no_mangle]
    static mut create_up_shaft: bool_;
    #[no_mangle]
    static mut create_down_shaft: bool_;
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
    static mut object_level: s16b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut opening_chest: bool_;
    #[no_mangle]
    static mut leaving_quest: libc::c_int;
    #[no_mangle]
    static mut always_pickup: bool_;
    #[no_mangle]
    static mut flush_failure: bool_;
    #[no_mangle]
    static mut confirm_stairs: bool_;
    #[no_mangle]
    static mut show_weights: bool_;
    #[no_mangle]
    static mut delay_factor: byte_hack;
    #[no_mangle]
    static mut panel_row_min: s16b;
    #[no_mangle]
    static mut panel_row_max: s16b;
    #[no_mangle]
    static mut panel_col_min: s16b;
    #[no_mangle]
    static mut panel_col_max: s16b;
    #[no_mangle]
    static mut target_col: s16b;
    #[no_mangle]
    static mut target_row: s16b;
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
    static mut tval_to_attr: [byte_hack; 128];
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut d_text: *mut libc::c_char;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut t_name: *mut libc::c_char;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut item_tester_tval: byte_hack;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut ambush_flag: bool_;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut generate_encounter: bool_;
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut deity_info: *mut deity_type;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn no_lite() -> bool_;
    #[no_mangle]
    fn move_cursor_relative(row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn print_rel(c: libc::c_char, a: byte_hack, y: libc::c_int,
                 x: libc::c_int);
    #[no_mangle]
    fn note_spot(y: libc::c_int, x: libc::c_int);
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
    fn health_track(m_idx: libc::c_int);
    #[no_mangle]
    fn monster_race_track(r_idx: libc::c_int, ego: libc::c_int);
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
    fn search();
    #[no_mangle]
    fn carry(pickup: libc::c_int);
    #[no_mangle]
    fn py_attack(y: libc::c_int, x: libc::c_int, max_blow: libc::c_int);
    #[no_mangle]
    fn player_can_enter(feature: byte_hack) -> bool_;
    #[no_mangle]
    fn move_player(dir: libc::c_int, do_pickup: libc::c_int, disarm: bool_);
    #[no_mangle]
    fn move_player_aux(dir: libc::c_int, do_pickup: libc::c_int,
                       run: libc::c_int, disarm: bool_);
    #[no_mangle]
    fn run_step(dir: libc::c_int);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn index_to_label(i: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn get_skill_scale(skill: libc::c_int, scale: u32b) -> s16b;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn get_flevel() -> libc::c_int;
    #[no_mangle]
    fn autosave_checkpoint();
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn enter_quest();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn swap_position(lty: libc::c_int, ltx: libc::c_int);
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn player_activate_door_trap(y: s16b, x: s16b);
    #[no_mangle]
    fn gain_exp(amount: s32b);
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn make_object(j_ptr: *mut object_type, good: bool_, great: bool_,
                   theme: obj_theme) -> bool_;
    #[no_mangle]
    fn make_gold(j_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn player_activate_trap_type(y: s16b, x: s16b, i_ptr: *mut object_type,
                                 item: s16b) -> bool_;
    #[no_mangle]
    fn get_rep_dir(dp: *mut libc::c_int) -> bool_;
    #[no_mangle]
    static mut easy_open: bool_;
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn place_object(y: libc::c_int, x: libc::c_int, good: bool_, great: bool_,
                    where_0: libc::c_int);
    #[no_mangle]
    fn place_gold(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    static mut easy_disarm: bool_;
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn fire_ball(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int,
                 rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn inc_stack_size(item: libc::c_int, delta: libc::c_int);
    #[no_mangle]
    fn change_wild_mode();
    #[no_mangle]
    fn teleport_player_directed(rad: libc::c_int, dir: libc::c_int);
    #[no_mangle]
    fn teleport_player(dis: libc::c_int);
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn project(who: libc::c_int, rad: libc::c_int, y: libc::c_int,
               x: libc::c_int, dam: libc::c_int, typ: libc::c_int,
               flg: libc::c_int) -> bool_;
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn message_pain(m_idx: libc::c_int, dam: libc::c_int);
    #[no_mangle]
    fn mon_take_hit(m_idx: libc::c_int, dam: libc::c_int, fear: *mut bool_,
                    note: cptr) -> bool_;
    #[no_mangle]
    fn change_side(m_ptr: *mut monster_type) -> bool_;
    #[no_mangle]
    fn is_friend(m_ptr: *mut monster_type) -> libc::c_int;
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn target_okay() -> bool_;
    #[no_mangle]
    fn set_disrupt_shield(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_invuln(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn get_aim_dir(dp: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn potion_smash_effect(who: libc::c_int, y: libc::c_int, x: libc::c_int,
                           o_sval: libc::c_int) -> bool_;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn inc_stack_size_ex(item: libc::c_int, delta: libc::c_int,
                         opt: optimize_flag, desc: describe_flag);
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn py_pickup_floor(pickup: libc::c_int);
    #[no_mangle]
    fn fetch(dir: libc::c_int, wgt: libc::c_int, require_los: bool_);
    #[no_mangle]
    fn teleport_player_to(ny: libc::c_int, nx: libc::c_int);
    #[no_mangle]
    fn tgt_pt(x: *mut libc::c_int, y: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn inc_piety(god: libc::c_int, amt: s32b);
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn wisdom_scale(max: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn follow_god(god: libc::c_int, silent: bool_);
    #[no_mangle]
    fn show_god_info(ext: bool_) -> bool_;
    #[no_mangle]
    fn screen_load();
    #[no_mangle]
    fn inven_carry(o_ptr: *mut object_type, final_0: bool_) -> s16b;
    #[no_mangle]
    fn verify(prompt: cptr, item: libc::c_int) -> bool_;
    #[no_mangle]
    fn screen_save();
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
pub struct deity_type {
    pub name: cptr,
    pub desc: [[libc::c_char; 80]; 10],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct between_exit {
    pub corresp: s16b,
    pub dungeon: bool_,
    pub wild_x: s16b,
    pub wild_y: s16b,
    pub px: s16b,
    pub py: s16b,
    pub d_idx: s16b,
    pub level: s16b,
}
pub type optimize_flag = libc::c_uint;
pub const NO_OPTIMIZE: optimize_flag = 1;
pub const OPTIMIZE: optimize_flag = 0;
pub type describe_flag = libc::c_uint;
pub const NO_DESCRIBE: describe_flag = 1;
pub const DESCRIBE: describe_flag = 0;
/*
 * Try to bash an altar
 */
unsafe extern "C" fn do_cmd_bash_altar(mut y: libc::c_int, mut x: libc::c_int)
 -> bool_ {
    msg_print(b"Are you mad? You want to anger the gods?\x00" as *const u8 as
                  *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
/*
 * Try to bash a fountain
 */
unsafe extern "C" fn do_cmd_bash_fountain(mut y: libc::c_int,
                                          mut x: libc::c_int) -> bool_ {
    let mut bash: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut more: bool_ = 1 as libc::c_int as bool_;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    if (*p_ptr).body_monster as libc::c_int != 0 as libc::c_int &&
           (*r_ptr).flags2 & 0x20000 as libc::c_int as libc::c_uint == 0 {
        msg_print(b"You cannot do that.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Message */
    msg_print(b"You smash into the fountain!\x00" as *const u8 as
                  *const libc::c_char);
    /* Hack -- Bash power based on strength */
	/* (Ranges from 3 to 20 to 100 to 200) */
    bash =
        *adj_str_blow.as_mut_ptr().offset((*p_ptr).stat_ind[0 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int;
    /* Compare bash power to door power XXX XXX XXX */
    temp = bash - 50 as libc::c_int;
    /* Hack -- always have a chance */
    if temp < 1 as libc::c_int { temp = 1 as libc::c_int }
    /* Hack -- attempt to bash down the door */
    if Rand_div(200 as libc::c_int) < temp {
        /* Message */
        msg_print(b"The fountain breaks!\x00" as *const u8 as
                      *const libc::c_char);
        fire_ball(14 as libc::c_int, 5 as libc::c_int,
                  damroll(6 as libc::c_int as s16b, 8 as libc::c_int as s16b),
                  2 as libc::c_int);
        cave_set_feat(y, x, 0xbb as libc::c_int);
        more = 0 as libc::c_int as bool_
    }
    return more;
}
/*
 * Go up one level
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_go_up() {
    let mut go_up: bool_ = 0 as libc::c_int as bool_;
    let mut go_up_many: bool_ = 0 as libc::c_int as bool_;
    let mut prob_traveling: bool_ = 0 as libc::c_int as bool_;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut oldl: libc::c_int = dun_level as libc::c_int;
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    /* Player grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                             isize)).offset((*p_ptr).px as
                                                                isize) as
            *mut cave_type;
    /* Can we ? */
    if process_hooks(18 as libc::c_int,
                     b"(s)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                     b"up\x00" as *const u8 as *const libc::c_char) != 0 {
        return
    }
    /* Normal up stairs */
    if (*c_ptr).feat as libc::c_int == 0x6 as libc::c_int ||
           (*c_ptr).feat as libc::c_int == 0xb4 as libc::c_int {
        if dun_level == 0 {
            go_up = 1 as libc::c_int as bool_
        } else if dungeon_flags2 as libc::c_long & 0x10 as libc::c_long != 0 {
            go_up =
                get_check(b"Leave this unique level forever? \x00" as
                              *const u8 as *const libc::c_char)
        } else if confirm_stairs != 0 {
            go_up =
                get_check(b"Really leave the level? \x00" as *const u8 as
                              *const libc::c_char)
        } else { go_up = 1 as libc::c_int as bool_ }
    } else if (*c_ptr).feat as libc::c_int == 0xe as libc::c_int {
        if dun_level as libc::c_int == 1 as libc::c_int {
            go_up = 1 as libc::c_int as bool_
        } else if dungeon_flags2 as libc::c_long & 0x10 as libc::c_long != 0 {
            go_up =
                get_check(b"Leave this unique level forever? \x00" as
                              *const u8 as *const libc::c_char)
        } else if confirm_stairs != 0 {
            go_up_many =
                get_check(b"Really leave the level? \x00" as *const u8 as
                              *const libc::c_char)
        } else { go_up_many = 1 as libc::c_int as bool_ }
    } else if (*c_ptr).feat as libc::c_int == 0x9 as libc::c_int {
        leaving_quest = (*p_ptr).inside_quest as libc::c_int;
        if dungeon_flags2 as libc::c_long & 0x10 as libc::c_long != 0 &&
               get_check(b"Leave this unique level forever? \x00" as *const u8
                             as *const libc::c_char) == 0 {
            return
        }
        (*p_ptr).inside_quest = (*c_ptr).special;
        dun_level = 0 as libc::c_int as s16b;
        (*p_ptr).oldpx = 0 as libc::c_int as s16b;
        (*p_ptr).oldpy = 0 as libc::c_int as s16b;
        (*p_ptr).leaving = 1 as libc::c_int as bool_;
        return
    } else {
        /* Shaft up */
        /* Quest exit */
        /* Exits to previous area in flat terrains */
        if dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long == 0 &&
               (*p_ptr).prob_travel as libc::c_int != 0 &&
               (*p_ptr).inside_quest == 0 {
            if (*d_ptr).mindepth as libc::c_int == dun_level as libc::c_int {
                return
            }
            if dungeon_flags2 as libc::c_long & 0x4000 as libc::c_long != 0 {
                msg_print(b"Some powerful force prevents your from teleporting.\x00"
                              as *const u8 as *const libc::c_char);
                return
            }
            prob_traveling = 1 as libc::c_int as bool_;
            if confirm_stairs != 0 {
                if get_check(b"Really leave the level? \x00" as *const u8 as
                                 *const libc::c_char) != 0 {
                    go_up = 1 as libc::c_int as bool_
                }
            } else { go_up = 1 as libc::c_int as bool_ }
        } else {
            msg_print(b"I see no up staircase here.\x00" as *const u8 as
                          *const libc::c_char);
            return
        }
    }
    if go_up as libc::c_int != 0 || go_up_many as libc::c_int != 0 {
        energy_use = 0 as libc::c_int;
        /* Success */
        if (*c_ptr).feat as libc::c_int == 0xb4 as libc::c_int {
            msg_print(b"You enter the previous area.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            msg_print(b"You enter a maze of up staircases.\x00" as *const u8
                          as *const libc::c_char);
        }
        autosave_checkpoint();
        if (*p_ptr).inside_quest != 0 {
            dun_level = 1 as libc::c_int as s16b;
            leaving_quest = (*p_ptr).inside_quest as libc::c_int;
            (*p_ptr).inside_quest = (*c_ptr).special
        }
        /* Create a way back */
        if go_up_many != 0 {
            create_down_shaft = 1 as libc::c_int as bool_
        } else { create_down_stair = 1 as libc::c_int as bool_ }
        /* New depth */
        if go_up != 0 {
            dun_level -= 1
        } else {
            dun_level =
                (dun_level as libc::c_int -
                     (Rand_div(3 as libc::c_int) + 1 as libc::c_int +
                          1 as libc::c_int)) as s16b;
            if dun_level as libc::c_int <= 0 as libc::c_int {
                dun_level = 0 as libc::c_int as s16b
            }
        }
        if (*c_ptr).special as libc::c_int != 0 && prob_traveling == 0 {
            dun_level = oldl as s16b;
            dun_level = get_flevel() as s16b;
            dungeon_type = (*c_ptr).special as byte_hack;
            dun_level =
                (dun_level as libc::c_int +
                     (*d_info.offset(dungeon_type as isize)).mindepth as
                         libc::c_int) as s16b
        }
        /* Leaving */
        (*p_ptr).leaving = 1 as libc::c_int as bool_
    };
}
/*
 * Returns TRUE if we are in the Between...
 */
unsafe extern "C" fn between_effect() -> bool_ {
    let mut bx: byte_hack = 0;
    let mut by: byte_hack = 0;
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
           libc::c_int == 0xa0 as libc::c_int {
        bx =
            ((*cave[(*p_ptr).py as
                        usize].offset((*p_ptr).px as isize)).special as
                 libc::c_int & 255 as libc::c_int) as byte_hack;
        by =
            ((*cave[(*p_ptr).py as
                        usize].offset((*p_ptr).px as isize)).special as
                 libc::c_int >> 8 as libc::c_int) as byte_hack;
        msg_print(b"You fall into the void.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(b"Brrrr! It\'s deadly cold.\x00" as *const u8 as
                      *const libc::c_char);
        swap_position(by as libc::c_int, bx as libc::c_int);
        /* To avoid being teleported back */
        energy_use = 100 as libc::c_int;
        return 1 as libc::c_int as bool_
    } else if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat
                  as libc::c_int == 0xb0 as libc::c_int {
        let mut be_ptr: *mut between_exit =
            &mut *between_exits.as_mut_ptr().offset((*(*cave.as_mut_ptr().offset((*p_ptr).py
                                                                                     as
                                                                                     isize)).offset((*p_ptr).px
                                                                                                        as
                                                                                                        isize)).special
                                                        as isize) as
                *mut between_exit;
        (*p_ptr).wild_mode = 0 as libc::c_int as bool_;
        (*p_ptr).wilderness_x = (*be_ptr).wild_x as s32b;
        (*p_ptr).wilderness_y = (*be_ptr).wild_y as s32b;
        (*p_ptr).px = (*be_ptr).px;
        (*p_ptr).oldpx = (*p_ptr).px;
        (*p_ptr).py = (*be_ptr).py;
        (*p_ptr).oldpy = (*p_ptr).py;
        dungeon_type = (*be_ptr).d_idx as byte_hack;
        dun_level = (*be_ptr).level;
        (*p_ptr).leaving = 1 as libc::c_int as bool_;
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/*
 * Go down one level
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_go_down() {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut go_down: bool_ = 0 as libc::c_int as bool_;
    let mut go_down_many: bool_ = 0 as libc::c_int as bool_;
    let mut prob_traveling: bool_ = 0 as libc::c_int as bool_;
    let mut fall_trap: bool_ = 0 as libc::c_int as bool_;
    let mut i: libc::c_char = 0;
    let mut old_dun: libc::c_int = dun_level as libc::c_int;
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    /*  MUST be actived now */
    if between_effect() != 0 { return }
    /* Player grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                             isize)).offset((*p_ptr).px as
                                                                isize) as
            *mut cave_type;
    if (*p_ptr).astral as libc::c_int != 0 &&
           dun_level as libc::c_int == 98 as libc::c_int {
        return
    }
    if (*c_ptr).t_idx as libc::c_int == 39 as libc::c_int {
        fall_trap = 1 as libc::c_int as bool_
    }
    /* test if on special level */
    if dungeon_flags2 as libc::c_long & 0x10 as libc::c_long != 0 {
        prt(b"Leave this unique level forever (y/n) ? \x00" as *const u8 as
                *const libc::c_char, 0 as libc::c_int, 0 as libc::c_int);
        flush();
        i = inkey();
        prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
            0 as libc::c_int);
        if i as libc::c_int != 'y' as i32 { return }
    }
    /* Can we ? */
    if process_hooks(18 as libc::c_int,
                     b"(s)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                     b"down\x00" as *const u8 as *const libc::c_char) != 0 {
        return
    }
    /* Normal up stairs */
    if (*c_ptr).feat as libc::c_int == 0xd as libc::c_int {
        if dun_level == 0 {
            go_down = 1 as libc::c_int as bool_;
            /* Save old player position */
            (*p_ptr).oldpx = (*p_ptr).px;
            (*p_ptr).oldpy = (*p_ptr).py
        } else if confirm_stairs != 0 {
            if get_check(b"Really leave the level? \x00" as *const u8 as
                             *const libc::c_char) != 0 {
                go_down_many = 1 as libc::c_int as bool_
            }
        } else { go_down_many = 1 as libc::c_int as bool_ }
    } else if (*c_ptr).feat as libc::c_int == 0x7 as libc::c_int ||
                  (*c_ptr).feat as libc::c_int == 0xb3 as libc::c_int {
        if (*p_ptr).prob_travel != 0 {
            if (*d_ptr).maxdepth as libc::c_int == dun_level as libc::c_int {
                return
            }
        }
        if dun_level == 0 {
            go_down = 1 as libc::c_int as bool_;
            /* Normal stairs */
            /* Save old player position */
            (*p_ptr).oldpx = (*p_ptr).px;
            (*p_ptr).oldpy = (*p_ptr).py
        } else if confirm_stairs != 0 {
            if get_check(b"Really leave the level? \x00" as *const u8 as
                             *const libc::c_char) != 0 {
                go_down = 1 as libc::c_int as bool_
            }
        } else { go_down = 1 as libc::c_int as bool_ }
    } else if (*c_ptr).feat as libc::c_int == 0x8 as libc::c_int {
        /* Handle quest areas -KMW- */
        /* Enter quest level */
        enter_quest();
        return
    } else {
        if dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long == 0 &&
               (*p_ptr).prob_travel as libc::c_int != 0 &&
               (*p_ptr).inside_quest == 0 {
            if (*d_ptr).maxdepth as libc::c_int == dun_level as libc::c_int {
                return
            }
            if dungeon_flags2 as libc::c_long & 0x4000 as libc::c_long != 0 {
                msg_print(b"Some powerfull force prevents your from teleporting.\x00"
                              as *const u8 as *const libc::c_char);
                return
            }
            prob_traveling = 1 as libc::c_int as bool_;
            if confirm_stairs != 0 {
                if get_check(b"Really leave the level? \x00" as *const u8 as
                                 *const libc::c_char) != 0 {
                    go_down = 1 as libc::c_int as bool_
                }
            } else { go_down = 1 as libc::c_int as bool_ }
        } else if fall_trap == 0 {
            msg_print(b"I see no down staircase here.\x00" as *const u8 as
                          *const libc::c_char);
            return
        }
    }
    if go_down as libc::c_int != 0 || go_down_many as libc::c_int != 0 {
        energy_use = 0 as libc::c_int;
        if fall_trap != 0 {
            msg_print(b"You deliberately jump through the trap door.\x00" as
                          *const u8 as *const libc::c_char);
        } else if (*c_ptr).feat as libc::c_int == 0xb3 as libc::c_int {
            msg_print(b"You enter the next area.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            msg_print(b"You enter a maze of down staircases.\x00" as *const u8
                          as *const libc::c_char);
        }
        autosave_checkpoint();
        /* Go down */
        if go_down != 0 {
            dun_level += 1
        } else if go_down_many != 0 {
            let mut i_0: libc::c_int =
                Rand_div(3 as libc::c_int) + 1 as libc::c_int +
                    1 as libc::c_int;
            let mut j: libc::c_int = 0;
            j = 1 as libc::c_int;
            while j < i_0 {
                dun_level += 1;
                if is_quest(dun_level as libc::c_int + i_0 - 1 as libc::c_int)
                       != 0 {
                    break ;
                }
                if (*d_ptr).maxdepth as libc::c_int ==
                       dun_level as libc::c_int {
                    break ;
                }
                j += 1
            }
        }
        /* We change place */
        if (*c_ptr).special as libc::c_int != 0 && prob_traveling == 0 {
            if (*d_info.offset((*c_ptr).special as isize)).min_plev as
                   libc::c_int <= (*p_ptr).lev as libc::c_int {
                let mut d_ptr_0: *mut dungeon_info_type =
                    &mut *d_info.offset((*c_ptr).special as isize) as
                        *mut dungeon_info_type;
                /* Do the lua scripts refuse ? ;) */
                if process_hooks(53 as libc::c_int,
                                 b"(d)\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 (*c_ptr).special as libc::c_int) != 0 {
                    dun_level = old_dun as s16b;
                    return
                }
                /* Ok go in the new dungeon */
                dungeon_type = (*c_ptr).special as byte_hack;
                d_ptr_0 =
                    &mut *d_info.offset(dungeon_type as isize) as
                        *mut dungeon_info_type;
                if (*p_ptr).wilderness_x == (*d_ptr_0).ix &&
                       (*p_ptr).wilderness_y == (*d_ptr_0).iy {
                    dun_level = (*d_ptr_0).mindepth
                } else if (*p_ptr).wilderness_x == (*d_ptr_0).ox &&
                              (*p_ptr).wilderness_y == (*d_ptr_0).oy {
                    dun_level = (*d_ptr_0).maxdepth
                } else { dun_level = (*d_ptr_0).mindepth }
                msg_format(b"You go into %s\x00" as *const u8 as
                               *const libc::c_char,
                           d_text.offset((*d_info.offset(dungeon_type as
                                                             isize)).text as
                                             isize));
            } else {
                msg_print(b"You don\'t feel yourself experienced enough to go there...\x00"
                              as *const u8 as *const libc::c_char);
                dun_level = old_dun as s16b;
                return
            }
        }
        /* Leaving */
        (*p_ptr).leaving = 1 as libc::c_int as bool_;
        if fall_trap == 0 {
            /* Create a way back */
            if go_down_many != 0 {
                create_up_shaft = 1 as libc::c_int as bool_
            } else { create_up_stair = 1 as libc::c_int as bool_ }
        }
    };
}
/*
 * Simple command to "search" for one turn
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_search() {
    /* Allow repeated command */
    if command_arg != 0 {
        /* Set repeat count */
        command_rep = (command_arg as libc::c_int - 1 as libc::c_int) as s16b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Cancel the arg */
        command_arg = 0 as libc::c_int as s16b
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Search */
    search();
}
/*
 * Hack -- toggle search mode
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_toggle_search() {
    /* Stop searching */
    if (*p_ptr).searching != 0 {
        /* Clear the searching flag */
        (*p_ptr).searching = 0 as libc::c_int as byte_hack;
        /* Recalculate bonuses */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b
    } else {
        /* Start searching */
        /* Set the searching flag */
        (*p_ptr).searching = 1 as libc::c_int as byte_hack;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long |
                 (0x100000 as libc::c_long | 0x200000 as libc::c_long)) as
                u32b
    };
}
/* Update stuff */
/* Redraw stuff */
/*
 * Determine if a grid contains a chest
 */
unsafe extern "C" fn chest_check(mut y: libc::c_int, mut x: libc::c_int)
 -> s16b {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    /* Scan all objects in the grid */
    this_o_idx = (*c_ptr).o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Skip unknown chests XXX XXX */
		/* if (!o_ptr->marked) continue; */
        /* Check for chest */
        if (*o_ptr).tval as libc::c_int == 7 as libc::c_int {
            return this_o_idx
        }
        this_o_idx = next_o_idx
    }
    /* No chest */
    return 0 as libc::c_int as s16b;
}
/*
 * Allocates objects upon opening a chest    -BEN-
 *
 * Disperse treasures from the given chest, centered at (x,y).
 *
 * Small chests often contain "gold", while Large chests always contain
 * items.  Wooden chests contain 2 items, Iron chests contain 4 items,
 * and Steel chests contain 6 items.  The "value" of the items in a
 * chest is based on the "power" of the chest, which is in turn based
 * on the level on which the chest is generated.
 */
unsafe extern "C" fn chest_death(mut y: libc::c_int, mut x: libc::c_int,
                                 mut o_idx: s16b) {
    let mut number: libc::c_int = 0;
    let mut small: bool_ = 0;
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
    let mut o_ptr: *mut object_type =
        &mut *o_list.offset(o_idx as isize) as *mut object_type;
    /* Small chests often hold "gold" */
    small =
        (((*o_ptr).sval as libc::c_int) < 4 as libc::c_int) as libc::c_int as
            bool_;
    /* Determine how much to drop (see above) */
    number =
        (*o_ptr).sval as libc::c_int % 4 as libc::c_int * 2 as libc::c_int;
    /* Zero pval means empty chest */
    if (*o_ptr).pval == 0 { number = 0 as libc::c_int }
    /* Opening a chest */
    opening_chest = 1 as libc::c_int as bool_;
    /* Determine the "value" of the items */
    object_level =
        ((if (*o_ptr).pval < 0 as libc::c_int {
              -(*o_ptr).pval
          } else { (*o_ptr).pval }) + 10 as libc::c_int) as s16b;
    let mut current_block_8: u64;
    /* Drop some objects (non-chests) */
    while number > 0 as libc::c_int {
        /* Get local object */
        q_ptr = &mut forge;
        /* Wipe the object */
        object_wipe(q_ptr);
        /* Small chests often drop gold */
        if small as libc::c_int != 0 &&
               Rand_div(100 as libc::c_int) < 75 as libc::c_int {
            /* Make some gold */
            if make_gold(q_ptr) == 0 {
                current_block_8 = 13513818773234778473;
            } else { current_block_8 = 17407779659766490442; }
        } else if make_object(q_ptr, 0 as libc::c_int as bool_,
                              0 as libc::c_int as bool_,
                              (*d_info.offset(dungeon_type as isize)).objs) ==
                      0 {
            current_block_8 = 13513818773234778473;
        } else { current_block_8 = 17407779659766490442; }
        match current_block_8 {
            17407779659766490442 => {
                /* Otherwise drop an item */
                /* Make an object */
                /* Drop it in the dungeon */
                drop_near(q_ptr, -(1 as libc::c_int), y, x);
            }
            _ => { }
        }
        number -= 1
    }
    /* Reset the object level */
    object_level = dun_level;
    /* No longer opening a chest */
    opening_chest = 0 as libc::c_int as bool_;
    /* Empty */
    (*o_ptr).pval = 0 as libc::c_int;
    (*o_ptr).pval2 = 0 as libc::c_int as s16b;
    /* Known */
    object_known(o_ptr);
}
/*
 * Chests have traps too.
 *
 * Exploding chest destroys contents (and traps).
 * Note that the chest itself is never destroyed.
 */
unsafe extern "C" fn chest_trap(mut y: libc::c_int, mut x: libc::c_int,
                                mut o_idx: s16b) {
    let mut trap: libc::c_int = 0;
    let mut o_ptr: *mut object_type =
        &mut *o_list.offset(o_idx as isize) as *mut object_type;
    let mut ident: bool_ = 0 as libc::c_int as bool_;
    /* Ignore disarmed chests */
    if (*o_ptr).pval <= 0 as libc::c_int { return }
    /* Obtain the trap */
    trap = (*o_ptr).pval;
    /* Message */
    msg_print(b"You found a trap!\x00" as *const u8 as *const libc::c_char);
    /* Set off trap */
    ident = player_activate_trap_type(y as s16b, x as s16b, o_ptr, o_idx);
    if ident != 0 {
        (*t_info.offset((*o_ptr).pval as isize)).ident =
            1 as libc::c_int as bool_;
        msg_format(b"You identified the trap as %s.\x00" as *const u8 as
                       *const libc::c_char,
                   t_name.offset((*t_info.offset(trap as isize)).name as
                                     libc::c_int as isize));
    };
}
/*
 * Attempt to open the given chest at the given location
 *
 * Assume there is no monster blocking the destination
 *
 * Returns TRUE if repeated commands may continue
 */
unsafe extern "C" fn do_cmd_open_chest(mut y: libc::c_int, mut x: libc::c_int,
                                       mut o_idx: s16b) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut flag: bool_ = 1 as libc::c_int as bool_;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    let mut o_ptr: *mut object_type =
        &mut *o_list.offset(o_idx as isize) as *mut object_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    if (*p_ptr).body_monster as libc::c_int != 0 as libc::c_int &&
           (*r_ptr).flags2 & 0x10000 as libc::c_int as libc::c_uint == 0 {
        msg_print(b"You cannot open chests.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Attempt to unlock it */
    if (*o_ptr).pval > 0 as libc::c_int {
        /* Assume locked, and thus not open */
        flag = 0 as libc::c_int as bool_;
        /* Get the "disarm" factor */
        i = (*p_ptr).skill_dis as libc::c_int;
        /* Penalize some conditions */
        if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0
           {
            i = i / 10 as libc::c_int
        }
        if (*p_ptr).confused as libc::c_int != 0 ||
               (*p_ptr).image as libc::c_int != 0 {
            i = i / 10 as libc::c_int
        }
        /* Extract the difficulty */
        j = i - (*o_ptr).pval;
        /* Always have a small chance of success */
        if j < 2 as libc::c_int { j = 2 as libc::c_int }
        /* Success -- May still have traps */
        if Rand_div(100 as libc::c_int) < j {
            msg_print(b"You have picked the lock.\x00" as *const u8 as
                          *const libc::c_char);
            gain_exp(1 as libc::c_int);
            flag = 1 as libc::c_int as bool_
        } else {
            /* Failure -- Keep trying */
            /* We may continue repeating */
            more = 1 as libc::c_int as bool_;
            if flush_failure != 0 { flush(); }
            msg_print(b"You failed to pick the lock.\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    /* Allowed to open */
    if flag != 0 {
        /* Apply chest traps, if any */
        chest_trap(y, x, o_idx);
        /* Let the Chest drop items */
        chest_death(y, x, o_idx);
    }
    /* Result */
    return more;
}
/*
 * Original code by TNB, improvement for Angband 2.9.3 by rr9
 * Slightly modified for ToME because of its trap implementation
 */
/*
 * Return TRUE if the given grid is an open door
 */
unsafe extern "C" fn is_open(mut c_ptr: *mut cave_type) -> bool_ {
    return ((*c_ptr).feat as libc::c_int == 0x4 as libc::c_int) as libc::c_int
               as bool_;
}
/*
 * Return TRUE if the given grid is a closed door
 */
unsafe extern "C" fn is_closed(mut c_ptr: *mut cave_type) -> bool_ {
    let mut feat: byte_hack = 0;
    if (*c_ptr).mimic != 0 {
        feat = (*c_ptr).mimic
    } else { feat = (*c_ptr).feat }
    return (feat as libc::c_int >= 0x20 as libc::c_int &&
                feat as libc::c_int <= 0x2f as libc::c_int) as libc::c_int as
               bool_;
}
/*
 * Return TRUE if the given grid has a trap
 */
unsafe extern "C" fn is_trap(mut c_ptr: *mut cave_type) -> bool_ {
    return ((*c_ptr).info as libc::c_int & 0x100 as libc::c_int !=
                0 as libc::c_int) as libc::c_int as bool_;
}
/*
 * Return the number of doors/traps around (or under)
 * the character using the filter function 'test'
 */
unsafe extern "C" fn count_feats(mut y: *mut libc::c_int,
                                 mut x: *mut libc::c_int,
                                 mut test:
                                     Option<unsafe extern "C" fn(_:
                                                                     *mut cave_type)
                                                -> bool_>, mut under: bool_)
 -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut xx: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    /* Clear match counter */
    count = 0 as libc::c_int;
    /* Check around (and under) the character */
    d = 0 as libc::c_int;
    while d < 9 as libc::c_int {
        /* Ignore current grid if told so -- See tables.c */
        if !(d == 8 as libc::c_int && under == 0) {
            /* Extract adjacent (legal) location */
            yy =
                (*p_ptr).py as libc::c_int +
                    ddy_ddd[d as usize] as libc::c_int;
            xx =
                (*p_ptr).px as libc::c_int +
                    ddx_ddd[d as usize] as libc::c_int;
            /* Paranoia */
            if yy > 0 as libc::c_int && xx > 0 as libc::c_int &&
                   yy < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   xx < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Must have knowledge */
                if !((*cave[yy as usize].offset(xx as isize)).info as
                         libc::c_int & 0x1 as libc::c_int == 0) {
                    /* Not looking for this feature */
                    if !(Some(test.expect("non-null function pointer")).expect("non-null function pointer")(&mut *(*cave.as_mut_ptr().offset(yy
                                                                                                                                                 as
                                                                                                                                                 isize)).offset(xx
                                                                                                                                                                    as
                                                                                                                                                                    isize))
                             == 0) {
                        /* Count it */
                        count += 1;
                        /* Remember the location. Only meaningful if there's
		   exactly one match */
                        *y = yy;
                        *x = xx
                    }
                }
            }
        }
        d += 1
    }
    /* All done */
    return count;
}
/*
 * Return the number of chests around (or under) the character.
 * If requested, count only trapped chests.
 */
unsafe extern "C" fn count_chests(mut y: *mut libc::c_int,
                                  mut x: *mut libc::c_int, mut trapped: bool_)
 -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut o_idx: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Count how many matches */
    count = 0 as libc::c_int;
    /* Check around (and under) the character */
    d = 0 as libc::c_int;
    while d < 9 as libc::c_int {
        /* Extract adjacent (legal) location */
        let mut yy: libc::c_int =
            (*p_ptr).py as libc::c_int + ddy_ddd[d as usize] as libc::c_int;
        let mut xx: libc::c_int =
            (*p_ptr).px as libc::c_int + ddx_ddd[d as usize] as libc::c_int;
        /* No (visible) chest is there */
        o_idx = chest_check(yy, xx) as libc::c_int;
        if !(o_idx == 0 as libc::c_int) {
            /* Grab the object */
            o_ptr = &mut *o_list.offset(o_idx as isize) as *mut object_type;
            /* Already open */
            if !((*o_ptr).pval == 0 as libc::c_int) {
                /* No (known) traps here */
                if !(trapped as libc::c_int != 0 &&
                         (!((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int
                                != 0 ||
                                (*k_info.offset((*o_ptr).k_idx as
                                                    isize)).easy_know as
                                    libc::c_int != 0 &&
                                    (*k_info.offset((*o_ptr).k_idx as
                                                        isize)).aware as
                                        libc::c_int != 0) ||
                              (*o_ptr).pval == 0)) {
                    /* OK */
                    count += 1;
                    /* Remember the location. Only useful if only one match */
                    *y = yy;
                    *x = xx
                }
            }
        }
        d += 1
    }
    /* All done */
    return count;
}
/*
 * Convert an adjacent location to a direction.
 */
unsafe extern "C" fn coords_to_dir(mut y: libc::c_int, mut x: libc::c_int)
 -> libc::c_int {
    let mut d: [[libc::c_int; 3]; 3] =
        [[7 as libc::c_int, 4 as libc::c_int, 1 as libc::c_int],
         [8 as libc::c_int, 5 as libc::c_int, 2 as libc::c_int],
         [9 as libc::c_int, 6 as libc::c_int, 3 as libc::c_int]];
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    dy = y - (*p_ptr).py as libc::c_int;
    dx = x - (*p_ptr).px as libc::c_int;
    /* Paranoia */
    if (if dx < 0 as libc::c_int { -dx } else { dx }) > 1 as libc::c_int ||
           (if dy < 0 as libc::c_int { -dy } else { dy }) > 1 as libc::c_int {
        return 0 as libc::c_int
    }
    return d[(dx + 1 as libc::c_int) as
                 usize][(dy + 1 as libc::c_int) as usize];
}
/*
 * Perform the basic "open" command on doors
 *
 * Assume destination is a closed/locked/jammed door
 *
 * Assume there is no monster blocking the destination
 *
 * Returns TRUE if repeated commands may continue
 */
unsafe extern "C" fn do_cmd_open_aux(mut y: libc::c_int, mut x: libc::c_int,
                                     mut dir: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    if (*p_ptr).body_monster as libc::c_int != 0 as libc::c_int &&
           (*r_ptr).flags2 & 0x10000 as libc::c_int as libc::c_uint == 0 {
        msg_print(b"You cannot open doors.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Get requested grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Jammed door */
    if (*c_ptr).feat as libc::c_int >=
           0x20 as libc::c_int + 0x8 as libc::c_int {
        /* Stuck */
        msg_print(b"The door appears to be stuck.\x00" as *const u8 as
                      *const libc::c_char);
    } else if (*c_ptr).feat as libc::c_int >=
                  0x20 as libc::c_int + 0x1 as libc::c_int {
        /* Locked door */
        /* Disarm factor */
        i = (*p_ptr).skill_dis as libc::c_int;
        /* Penalize some conditions */
        if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0
           {
            i = i / 10 as libc::c_int
        }
        if (*p_ptr).confused as libc::c_int != 0 ||
               (*p_ptr).image as libc::c_int != 0 {
            i = i / 10 as libc::c_int
        }
        /* Extract the lock power */
        j = (*c_ptr).feat as libc::c_int - 0x20 as libc::c_int;
        /* Extract the difficulty XXX XXX XXX */
        j = i - j * 4 as libc::c_int;
        /* Always have a small chance of success */
        if j < 2 as libc::c_int { j = 2 as libc::c_int }
        /* Success */
        if Rand_div(100 as libc::c_int) < j {
            /* Message */
            msg_print(b"You have picked the lock.\x00" as *const u8 as
                          *const libc::c_char);
            /* Set off trap */
            if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
                player_activate_door_trap(y as s16b, x as s16b);
            }
            /* Open the door */
            cave_set_feat(y, x, 0x4 as libc::c_int);
            /* Update some things */
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long |
                     (0x100000 as libc::c_long | 0x1000000 as libc::c_long |
                          0x200000 as libc::c_long)) as u32b;
            /* Sound */
            sound(22 as libc::c_int);
            /* Experience */
            gain_exp(1 as libc::c_int);
        } else {
            /* Failure */
            /* Failure */
            if flush_failure != 0 { flush(); }
            msg_print(b"You failed to pick the lock.\x00" as *const u8 as
                          *const libc::c_char);
            more = 1 as libc::c_int as bool_
        }
    } else {
        /* Message */
        /* We may keep trying */
        /* Closed door */
        /* Set off trap */
        if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
            player_activate_door_trap(y as s16b, x as s16b);
        }
        cave_set_feat(y, x, 0x4 as libc::c_int);
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x1000000 as libc::c_long |
                      0x200000 as libc::c_long)) as u32b;
        sound(22 as libc::c_int);
    }
    /* Open the door */
    /* Update some things */
    /* Sound */
    /* Result */
    return more;
}
/*
 * Open a closed/locked/jammed door or a closed/locked chest.
 *
 * Unlocking a locked door/chest is worth one experience point.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_open() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut o_idx: s16b = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    if (*p_ptr).body_monster as libc::c_int != 0 as libc::c_int &&
           (*r_ptr).flags2 & 0x10000 as libc::c_int as libc::c_uint == 0 {
        msg_print(b"You cannot open doors.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Option: Pick a direction */
    if easy_open != 0 {
        let mut num_doors: libc::c_int = 0;
        let mut num_chests: libc::c_int = 0;
        /* Count closed doors (locked or jammed) */
        num_doors =
            count_feats(&mut y, &mut x,
                        Some(is_closed as
                                 unsafe extern "C" fn(_: *mut cave_type)
                                     -> bool_), 0 as libc::c_int as bool_);
        /* Count chests (locked) */
        num_chests = count_chests(&mut y, &mut x, 0 as libc::c_int as bool_);
        /* There is nothing the player can open */
        if num_doors + num_chests == 0 as libc::c_int {
            /* Message */
            msg_print(b"You see nothing there to open.\x00" as *const u8 as
                          *const libc::c_char);
            /* Done */
            return
        } else {
            /* Set direction if there is only one target */
            if num_doors + num_chests == 1 as libc::c_int {
                command_dir = coords_to_dir(y, x) as s16b
            }
        }
    }
    /* Allow repeated command */
    if command_arg != 0 {
        /* Set repeat count */
        command_rep = (command_arg as libc::c_int - 1 as libc::c_int) as s16b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Cancel the arg */
        command_arg = 0 as libc::c_int as s16b
    }
    /* Get a "repeated" direction */
    if get_rep_dir(&mut dir) != 0 {
        /* Get requested location */
        y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
        x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
        /* Get requested grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Check for chest */
        o_idx = chest_check(y, x);
        /* Nothing useful */
        if !((*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
                 (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int) &&
               o_idx == 0 {
            /* Message */
            msg_print(b"You see nothing there to open.\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*c_ptr).m_idx != 0 {
            /* Monster in the way */
            /* Take a turn */
            energy_use = 100 as libc::c_int;
            /* Message */
            msg_print(b"There is a monster in the way!\x00" as *const u8 as
                          *const libc::c_char);
            /* Attack */
            py_attack(y, x, -(1 as libc::c_int));
        } else if o_idx != 0 {
            /* Handle chests */
            /* Open the chest */
            more = do_cmd_open_chest(y, x, o_idx)
        } else {
            /* Handle doors */
            /* Open the door */
            more = do_cmd_open_aux(y, x, dir)
        }
    }
    /* Process the appropriate hooks */
    process_hooks(1 as libc::c_int,
                  b"(d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, is_quest(dun_level as libc::c_int));
    /* Cancel repeat unless we may continue */
    if more == 0 { disturb(0 as libc::c_int, 0 as libc::c_int); };
}
/*
 * Perform the basic "close" command
 *
 * Assume destination is an open/broken door
 *
 * Assume there is no monster blocking the destination
 *
 * Returns TRUE if repeated commands may continue
 */
unsafe extern "C" fn do_cmd_close_aux(mut y: libc::c_int, mut x: libc::c_int,
                                      mut dir: libc::c_int) -> bool_ {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    if (*p_ptr).body_monster as libc::c_int != 0 as libc::c_int &&
           (*r_ptr).flags2 & 0x10000 as libc::c_int as libc::c_uint == 0 {
        msg_print(b"You cannot close doors.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Get grid and contents */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Set off trap */
    if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
        player_activate_door_trap(y as s16b, x as s16b);
    }
    /* Broken door */
    if (*c_ptr).feat as libc::c_int == 0x5 as libc::c_int {
        /* Message */
        msg_print(b"The door appears to be broken.\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        /* Open door */
        /* Close the door */
        cave_set_feat(y, x, 0x20 as libc::c_int + 0 as libc::c_int);
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x1000000 as libc::c_long |
                      0x200000 as libc::c_long)) as u32b;
        sound(23 as libc::c_int);
    }
    /* Update some things */
    /* Sound */
    /* Result */
    return more;
}
/*
 * Close an open door.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_close() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    /* Option: Pick a direction */
    if easy_open != 0 {
        let mut num_doors: libc::c_int = 0;
        /* Count open doors */
        num_doors =
            count_feats(&mut y, &mut x,
                        Some(is_open as
                                 unsafe extern "C" fn(_: *mut cave_type)
                                     -> bool_), 0 as libc::c_int as bool_);
        /* There are no doors the player can close */
        if num_doors == 0 as libc::c_int {
            /* Message */
            msg_print(b"You see nothing there to close.\x00" as *const u8 as
                          *const libc::c_char);
            /* Done */
            return
        } else {
            /* Exactly one closeable door */
            if num_doors == 1 as libc::c_int {
                command_dir = coords_to_dir(y, x) as s16b
            }
        }
    }
    /* Allow repeated command */
    if command_arg != 0 {
        /* Set repeat count */
        command_rep = (command_arg as libc::c_int - 1 as libc::c_int) as s16b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Cancel the arg */
        command_arg = 0 as libc::c_int as s16b
    }
    /* Get a "repeated" direction */
    if get_rep_dir(&mut dir) != 0 {
        /* Get requested location */
        y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
        x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
        /* Get grid and contents */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Require open/broken door */
        if (*c_ptr).feat as libc::c_int != 0x4 as libc::c_int &&
               (*c_ptr).feat as libc::c_int != 0x5 as libc::c_int {
            /* Message */
            msg_print(b"You see nothing there to close.\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*c_ptr).m_idx != 0 {
            /* Monster in the way */
            /* Take a turn */
            energy_use = 100 as libc::c_int;
            /* Message */
            msg_print(b"There is a monster in the way!\x00" as *const u8 as
                          *const libc::c_char);
            /* Attack */
            py_attack(y, x, -(1 as libc::c_int));
        } else {
            /* Close the door */
            /* Close the door */
            more = do_cmd_close_aux(y, x, dir)
        }
    }
    /* Cancel repeat unless we may continue */
    if more == 0 { disturb(0 as libc::c_int, 0 as libc::c_int); };
}
/*
 * Determine if a given grid may be "tunneled"
 */
unsafe extern "C" fn do_cmd_tunnel_test(mut y: libc::c_int,
                                        mut x: libc::c_int) -> bool_ {
    /* Must have knowledge(execpt on "forget" levels) */
    if (*cave[y as usize].offset(x as isize)).info as libc::c_int &
           0x1 as libc::c_int == 0 {
        /* Message */
        msg_print(b"You see nothing there.\x00" as *const u8 as
                      *const libc::c_char);
        /* Nope */
        return 0 as libc::c_int as bool_
    }
    /* Must be a wall/door/etc */
    if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x10 as libc::c_long != 0 &&
           (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
               0xaf as libc::c_int {
        /* Message */
        msg_print(b"You see nothing there to tunnel.\x00" as *const u8 as
                      *const libc::c_char);
        /* Nope */
        return 0 as libc::c_int as bool_
    }
    /* Must be tunnelable */
    if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x8000 as libc::c_long == 0 {
        /* Message */
        msg_print(f_text.offset((*f_info.offset((*cave[y as
                                                           usize].offset(x as
                                                                             isize)).feat
                                                    as isize)).tunnel as
                                    isize) as cptr);
        /* Nope */
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Tunnel through wall.  Assumes valid location.
 *
 * Note that it is impossible to "extend" rooms past their
 * outer walls (which are actually part of the room).
 *
 * This will, however, produce grids which are NOT illuminated
 * (or darkened) along with the rest of the room.
 */
unsafe extern "C" fn twall(mut y: libc::c_int, mut x: libc::c_int,
                           mut feat: byte_hack) -> bool_ {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Paranoia -- Require a wall or door or some such */
    if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x10 as libc::c_long != 0 &&
           (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
               0xaf as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Forget the wall */
    (*c_ptr).info =
        ((*c_ptr).info as libc::c_int & !(0x1 as libc::c_int)) as u16b;
    /* Remove the feature */
    cave_set_feat(y, x, feat as libc::c_int);
    /* Update some things */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x200000 as libc::c_long)) as
            u32b;
    /* Result */
    return 1 as libc::c_int as bool_;
}
/*
 * Perform the basic "tunnel" command
 *
 * Assumes that the destination is a wall, a vein, a secret
 * door, or rubble.
 *
 * Assumes that no monster is blocking the destination
 *
 * Returns TRUE if repeated commands may continue
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_tunnel_aux(mut y: libc::c_int,
                                           mut x: libc::c_int,
                                           mut dir: libc::c_int) -> bool_ {
    let mut skill_req: libc::c_int = 0 as libc::c_int;
    let mut skill_req_1pct: libc::c_int = 0 as libc::c_int;
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut f_ptr: *mut feature_type =
        &mut *f_info.offset((*c_ptr).feat as isize) as *mut feature_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    /* Must be have something to dig with (except for sandwalls) */
    if ((*c_ptr).feat as libc::c_int) < 0x62 as libc::c_int ||
           (*c_ptr).feat as libc::c_int > 0x64 as libc::c_int {
        if (*p_ptr).inventory[51 as libc::c_int as usize].k_idx == 0 ||
               (*p_ptr).inventory[51 as libc::c_int as usize].tval as
                   libc::c_int != 20 as libc::c_int {
            msg_print(b"You need to have a shovel or pick in your tool slot.\x00"
                          as *const u8 as *const libc::c_char);
            return 0 as libc::c_int as bool_
        }
    }
    /* Verify legality */
    if do_cmd_tunnel_test(y, x) == 0 { return 0 as libc::c_int as bool_ }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Get grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Sound */
    sound(21 as libc::c_int);
    /* Titanium */
    if (*f_ptr).flags1 as libc::c_long & 0x40 as libc::c_long != 0 {
        msg_print(f_text.offset((*f_ptr).tunnel as isize) as cptr);
    } else if (*c_ptr).feat as libc::c_int == 0x60 as libc::c_int ||
                  (*c_ptr).feat as libc::c_int == 0x5c as libc::c_int {
        /* Chop Down */
        skill_req = 10 as libc::c_int;
        skill_req_1pct = 14 as libc::c_int;
        if (*p_ptr).skill_dig as libc::c_int >
               10 as libc::c_int + Rand_div(400 as libc::c_int) &&
               twall(y, x, 0x59 as libc::c_int as byte_hack) as libc::c_int !=
                   0 {
            msg_print(b"You have cleared away the trees.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            /* Keep trying */
            /* We may continue chopping */
            msg_print(f_text.offset((*f_ptr).tunnel as isize) as cptr);
            more = 1 as libc::c_int as bool_;
            if Rand_div(100 as libc::c_int) < 25 as libc::c_int { search(); }
        }
    } else if (*c_ptr).feat as libc::c_int >= 0x38 as libc::c_int &&
                  (*c_ptr).feat as libc::c_int <= 0x3b as libc::c_int {
        /* Occasional Search XXX XXX */
        /* Granite */
        /* Tunnel */
        skill_req = 40 as libc::c_int;
        skill_req_1pct = 56 as libc::c_int;
        if (*p_ptr).skill_dig as libc::c_int >
               40 as libc::c_int + Rand_div(1600 as libc::c_int) &&
               twall(y, x, 0x1 as libc::c_int as byte_hack) as libc::c_int !=
                   0 {
            msg_print(b"You have finished the tunnel.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            /* Keep trying */
            /* We may continue tunelling */
            msg_print(f_text.offset((*f_ptr).tunnel as isize) as cptr);
            more = 1 as libc::c_int as bool_
        }
    } else if (*c_ptr).feat as libc::c_int >= 0x32 as libc::c_int &&
                  (*c_ptr).feat as libc::c_int <= 0x37 as libc::c_int ||
                  (*c_ptr).feat as libc::c_int >= 0x62 as libc::c_int &&
                      (*c_ptr).feat as libc::c_int <= 0x64 as libc::c_int {
        let mut okay: bool_ = 0 as libc::c_int as bool_;
        let mut gold: bool_ = 0 as libc::c_int as bool_;
        let mut hard: bool_ = 0 as libc::c_int as bool_;
        let mut soft: bool_ = 0 as libc::c_int as bool_;
        /* Quartz / Magma / Sandwall */
        /* Found gold */
        if (*c_ptr).feat as libc::c_int >= 0x34 as libc::c_int &&
               (*c_ptr).feat as libc::c_int <= 0x37 as libc::c_int {
            gold = 1 as libc::c_int as bool_
        }
        if (*c_ptr).feat as libc::c_int == 0x63 as libc::c_int ||
               (*c_ptr).feat as libc::c_int == 0x64 as libc::c_int {
            gold = 1 as libc::c_int as bool_;
            soft = 1 as libc::c_int as bool_
        } else if (*c_ptr).feat as libc::c_int - 0x32 as libc::c_int &
                      0x1 as libc::c_int != 0 {
            hard = 1 as libc::c_int as bool_
        }
        /* Extract "quartz" flag XXX XXX XXX */
        /* Quartz */
        if hard != 0 {
            skill_req = 20 as libc::c_int;
            skill_req_1pct = 28 as libc::c_int;
            okay =
                ((*p_ptr).skill_dig as libc::c_int >
                     20 as libc::c_int + Rand_div(800 as libc::c_int)) as
                    libc::c_int as bool_
        } else if soft != 0 {
            skill_req = 5 as libc::c_int;
            skill_req_1pct = 8 as libc::c_int;
            okay =
                ((*p_ptr).skill_dig as libc::c_int >
                     5 as libc::c_int + Rand_div(250 as libc::c_int)) as
                    libc::c_int as bool_
        } else {
            /* Sandwall */
            /* Magma */
            skill_req = 10 as libc::c_int;
            skill_req_1pct = 14 as libc::c_int;
            okay =
                ((*p_ptr).skill_dig as libc::c_int >
                     10 as libc::c_int + Rand_div(400 as libc::c_int)) as
                    libc::c_int as bool_
        }
        /* Success */
        if okay as libc::c_int != 0 &&
               twall(y, x, 0x1 as libc::c_int as byte_hack) as libc::c_int !=
                   0 {
            /* Found treasure */
            if gold != 0 {
                /* Place some gold */
                place_gold(y, x);
                /* Message */
                msg_print(b"You have found something!\x00" as *const u8 as
                              *const libc::c_char);
            } else {
                /* Found nothing */
                /* Message */
                msg_print(b"You have finished the tunnel.\x00" as *const u8 as
                              *const libc::c_char);
            }
        } else {
            /* Failure */
            /* Message, continue digging */
            msg_print(f_text.offset((*f_ptr).tunnel as isize) as cptr);
            more = 1 as libc::c_int as bool_
        }
    } else if (*c_ptr).feat as libc::c_int == 0x31 as libc::c_int {
        /* Rubble */
        /* Remove the rubble */
        skill_req = 0 as libc::c_int;
        skill_req_1pct = 2 as libc::c_int;
        if (*p_ptr).skill_dig as libc::c_int > Rand_div(200 as libc::c_int) &&
               twall(y, x,
                     (*d_info.offset(dungeon_type as isize)).floor1 as
                         byte_hack) as libc::c_int != 0 {
            /* Message */
            msg_print(b"You have removed the rubble.\x00" as *const u8 as
                          *const libc::c_char);
            /* Hack -- place an object */
            if Rand_div(100 as libc::c_int) < 10 as libc::c_int {
                /* Create a simple object */
                place_object(y, x, 0 as libc::c_int as bool_,
                             0 as libc::c_int as bool_, 5 as libc::c_int);
                /* Observe new object */
                if (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x10 as libc::c_int != 0 as libc::c_int {
                    msg_print(b"You have found something!\x00" as *const u8 as
                                  *const libc::c_char);
                }
            }
        } else {
            /* Message, keep digging */
            msg_print(f_text.offset((*f_ptr).tunnel as isize) as cptr);
            more = 1 as libc::c_int as bool_
        }
    } else if (*c_ptr).feat as libc::c_int >= 0x30 as libc::c_int {
        /* Secret doors */
        /* Tunnel */
        skill_req = 30 as libc::c_int;
        skill_req_1pct = 42 as libc::c_int;
        if (*p_ptr).skill_dig as libc::c_int >
               30 as libc::c_int + Rand_div(1200 as libc::c_int) &&
               twall(y, x, 0x1 as libc::c_int as byte_hack) as libc::c_int !=
                   0 {
            msg_print(b"You have finished the tunnel.\x00" as *const u8 as
                          *const libc::c_char);
            (*c_ptr).mimic = 0 as libc::c_int as byte_hack;
            lite_spot(y, x);
            /* Set off trap */
            if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
                player_activate_door_trap(y as s16b, x as s16b);
            }
        } else {
            /* Keep trying */
            let mut feat: libc::c_int = 0;
            if (*c_ptr).mimic != 0 {
                feat = (*c_ptr).mimic as libc::c_int
            } else { feat = (*c_ptr).feat as libc::c_int }
            /* We may continue tunelling */
            msg_print(f_text.offset((*f_info.offset(feat as isize)).tunnel as
                                        isize) as cptr);
            more = 1 as libc::c_int as bool_;
            /* Occasional Search XXX XXX */
            if Rand_div(100 as libc::c_int) < 25 as libc::c_int { search(); }
        }
    } else {
        /* Doors */
        /* Tunnel */
        skill_req = 30 as libc::c_int;
        skill_req_1pct = 42 as libc::c_int;
        if (*p_ptr).skill_dig as libc::c_int >
               30 as libc::c_int + Rand_div(1200 as libc::c_int) &&
               twall(y, x, 0x1 as libc::c_int as byte_hack) as libc::c_int !=
                   0 {
            msg_print(b"You have finished the tunnel.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            /* Keep trying */
            /* We may continue tunelling */
            msg_print(f_text.offset((*f_ptr).tunnel as isize) as cptr);
            more = 1 as libc::c_int as bool_
        }
    }
    if more as libc::c_int != 0 &&
           Rand_div(100 as libc::c_int) < 2 as libc::c_int {
        if ((*p_ptr).skill_dig as libc::c_int) < skill_req {
            msg_print(b"You fail to make even the slightest of progress.\x00"
                          as *const u8 as *const libc::c_char);
            more = 0 as libc::c_int as bool_
        } else if ((*p_ptr).skill_dig as libc::c_int) < skill_req_1pct {
            msg_print(b"This will take some time.\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    /* Notice new floor grids */
    if !((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                             isize)).flags1 as libc::c_long &
             0x10 as libc::c_long != 0 &&
             (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                 0xaf as libc::c_int) {
        /* Update some things */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                      0x1000000 as libc::c_long | 0x200000 as libc::c_long))
                as u32b
    }
    /* Result */
    return more;
}
/*
 * Tunnels through "walls" (including rubble and closed doors)
 *
 * Note that you must tunnel in order to hit invisible monsters
 * in walls, though moving into walls still takes a turn anyway.
 *
 * Digging is very difficult without a "digger" weapon, but can be
 * accomplished by strong players using heavy weapons.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_tunnel() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    if (*p_ptr).wild_mode != 0 { return }
    /* Allow repeated command */
    if command_arg != 0 {
        /* Set repeat count */
        command_rep = (command_arg as libc::c_int - 1 as libc::c_int) as s16b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Cancel the arg */
        command_arg = 0 as libc::c_int as s16b
    }
    /* Get a direction to tunnel, or Abort */
    if get_rep_dir(&mut dir) != 0 {
        /* Get location */
        y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
        x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
        /* Get grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* No tunnelling through doors */
        if (*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
               (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int ||
               (*c_ptr).feat as libc::c_int == 0x4a as libc::c_int {
            /* Message */
            msg_print(b"You cannot tunnel through doors.\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                      libc::c_long & 0x10 as libc::c_long != 0 &&
                      (*c_ptr).feat as libc::c_int != 0xaf as libc::c_int {
            /* No tunnelling through air */
            /* Message */
            msg_print(b"You cannot tunnel through air.\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*c_ptr).m_idx != 0 {
            /* A monster is in the way */
            /* Take a turn */
            energy_use = 100 as libc::c_int;
            /* Message */
            msg_print(b"There is a monster in the way!\x00" as *const u8 as
                          *const libc::c_char);
            /* Attack */
            py_attack(y, x, -(1 as libc::c_int));
        } else {
            /* Try digging */
            /* Tunnel through walls */
            more = do_cmd_tunnel_aux(y, x, dir)
        }
    }
    /* Cancel repetition unless we can continue */
    if more == 0 { disturb(0 as libc::c_int, 0 as libc::c_int); };
}
/*
 * easy_open_door --
 *
 *	If there is a jammed/closed/locked door at the given location,
 *	then attempt to unlock/open it. Return TRUE if an attempt was
 *	made (successful or not), otherwise return FALSE.
 *
 *	The code here should be nearly identical to that in
 *	do_cmd_open_test() and do_cmd_open_aux().
 */
#[no_mangle]
pub unsafe extern "C" fn easy_open_door(mut y: libc::c_int,
                                        mut x: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    if (*p_ptr).body_monster as libc::c_int != 0 as libc::c_int &&
           (*r_ptr).flags2 & 0x10000 as libc::c_int as libc::c_uint == 0 {
        msg_print(b"You cannot open doors.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Must be a closed door */
    if !((*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
             (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int) {
        /* Nope */
        return 0 as libc::c_int as bool_
    }
    /* Jammed door */
    if (*c_ptr).feat as libc::c_int >=
           0x20 as libc::c_int + 0x8 as libc::c_int {
        /* Stuck */
        msg_print(b"The door appears to be stuck.\x00" as *const u8 as
                      *const libc::c_char);
    } else if (*c_ptr).feat as libc::c_int >=
                  0x20 as libc::c_int + 0x1 as libc::c_int {
        /* Locked door */
        /* Disarm factor */
        i = (*p_ptr).skill_dis as libc::c_int;
        /* Penalize some conditions */
        if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0
           {
            i = i / 10 as libc::c_int
        }
        if (*p_ptr).confused as libc::c_int != 0 ||
               (*p_ptr).image as libc::c_int != 0 {
            i = i / 10 as libc::c_int
        }
        /* Extract the lock power */
        j = (*c_ptr).feat as libc::c_int - 0x20 as libc::c_int;
        /* Extract the difficulty XXX XXX XXX */
        j = i - j * 4 as libc::c_int;
        /* Always have a small chance of success */
        if j < 2 as libc::c_int { j = 2 as libc::c_int }
        /* Success */
        if Rand_div(100 as libc::c_int) < j {
            /* Message */
            msg_print(b"You have picked the lock.\x00" as *const u8 as
                          *const libc::c_char);
            /* Set off trap */
            if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
                player_activate_door_trap(y as s16b, x as s16b);
            }
            /* Open the door */
            cave_set_feat(y, x, 0x4 as libc::c_int);
            /* Update some things */
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long |
                     (0x100000 as libc::c_long | 0x1000000 as libc::c_long |
                          0x200000 as libc::c_long)) as u32b;
            /* Sound */
            sound(22 as libc::c_int);
            /* Process the appropriate hooks */
            process_hooks(1 as libc::c_int,
                          b"(d)\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          is_quest(dun_level as libc::c_int));
            /* Experience */
            gain_exp(1 as libc::c_int);
        } else {
            /* Failure */
            /* Failure */
            if flush_failure != 0 { flush(); }
            msg_print(b"You failed to pick the lock.\x00" as *const u8 as
                          *const libc::c_char);
        }
    } else {
        /* Message */
        /* Closed door */
        /* Set off trap */
        if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
            player_activate_door_trap(y as s16b, x as s16b);
        }
        cave_set_feat(y, x, 0x4 as libc::c_int);
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x1000000 as libc::c_long |
                      0x200000 as libc::c_long)) as u32b;
        sound(22 as libc::c_int);
    }
    /* Open the door */
    /* Update some things */
    /* Sound */
    /* Result */
    return 1 as libc::c_int as bool_;
}
/*
 * Perform the basic "disarm" command
 *
 * Assume destination is a visible trap
 *
 * Assume there is no monster blocking the destination
 *
 * Returns TRUE if repeated commands may continue
 */
unsafe extern "C" fn do_cmd_disarm_chest(mut y: libc::c_int,
                                         mut x: libc::c_int, mut o_idx: s16b)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    let mut o_ptr: *mut object_type =
        &mut *o_list.offset(o_idx as isize) as *mut object_type;
    let mut t_ptr: *mut trap_type =
        &mut *t_info.offset((*o_ptr).pval as isize) as *mut trap_type;
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Get the "disarm" factor */
    i = (*p_ptr).skill_dis as libc::c_int;
    /* Penalize some conditions */
    if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0 {
        i = i / 10 as libc::c_int
    }
    if (*p_ptr).confused as libc::c_int != 0 ||
           (*p_ptr).image as libc::c_int != 0 {
        i = i / 10 as libc::c_int
    }
    /* Extract the difficulty */
    j = i - (*t_ptr).difficulty as libc::c_int * 3 as libc::c_int;
    /* Always have a small chance of success */
    if j < 2 as libc::c_int { j = 2 as libc::c_int }
    /* Must find the trap first. */
    if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
             (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                 libc::c_int != 0 &&
                 (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                     libc::c_int != 0) {
        msg_print(b"I don\'t see any traps.\x00" as *const u8 as
                      *const libc::c_char);
    } else if (*o_ptr).pval <= 0 as libc::c_int {
        msg_print(b"The chest is not trapped.\x00" as *const u8 as
                      *const libc::c_char);
    } else if Rand_div(100 as libc::c_int) < j {
        msg_print(b"You have disarmed the chest.\x00" as *const u8 as
                      *const libc::c_char);
        gain_exp((*t_ptr).difficulty as libc::c_int * 3 as libc::c_int);
        (*o_ptr).pval = 0 as libc::c_int - (*o_ptr).pval
    } else if i > 5 as libc::c_int &&
                  Rand_div(i) + 1 as libc::c_int > 5 as libc::c_int {
        /* Already disarmed/unlocked */
        /* Success (get a lot of experience) */
        /* Failure -- Keep trying */
        /* We may keep trying */
        more = 1 as libc::c_int as bool_;
        if flush_failure != 0 { flush(); }
        msg_print(b"You failed to disarm the chest.\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        /* Failure -- Set off the trap */
        msg_print(b"You set off a trap!\x00" as *const u8 as
                      *const libc::c_char);
        sound(55 as libc::c_int);
        chest_trap(y, x, o_idx);
    }
    /* Result */
    return more;
}
/*
 * Perform the basic "disarm" command
 *
 * Assume destination is a visible trap
 *
 * Assume there is no monster blocking the destination
 *
 * Returns TRUE if repeated commands may continue
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_disarm_aux(mut y: libc::c_int,
                                           mut x: libc::c_int,
                                           mut dir: libc::c_int,
                                           mut do_pickup: libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut power: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut name: cptr = 0 as *const libc::c_char;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Get grid and contents */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Access trap name */
    if (*t_info.offset((*c_ptr).t_idx as isize)).ident != 0 {
        name =
            t_name.offset((*t_info.offset((*c_ptr).t_idx as isize)).name as
                              libc::c_int as isize) as cptr
    } else { name = b"unknown trap\x00" as *const u8 as *const libc::c_char }
    /* Get the "disarm" factor */
    i = (*p_ptr).skill_dis as libc::c_int;
    /* Penalize some conditions */
    if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0 {
        i = i / 10 as libc::c_int
    }
    if (*p_ptr).confused as libc::c_int != 0 ||
           (*p_ptr).image as libc::c_int != 0 {
        i = i / 10 as libc::c_int
    }
    /* XXX XXX XXX Variable power? */
    /* Extract trap "power" */
    power =
        (*t_info.offset((*c_ptr).t_idx as isize)).difficulty as libc::c_int;
    /* Extract the difficulty */
    j = i - power;
    /* Always have a small chance of success */
    if j < 2 as libc::c_int { j = 2 as libc::c_int }
    /* Success */
    if Rand_div(100 as libc::c_int) < j {
        /* Message */
        msg_format(b"You have disarmed the %s.\x00" as *const u8 as
                       *const libc::c_char, name);
        /* Reward */
        gain_exp(power);
        /* Forget the trap */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int &
                 !(0x1 as libc::c_int | 0x100 as libc::c_int)) as u16b;
        /* Remove the trap */
        (*c_ptr).t_idx = 0 as libc::c_int as s16b;
        /* Move the player onto the trap */
        if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
               0x1000 as libc::c_long == 0 {
            move_player_aux(dir, do_pickup, 0 as libc::c_int,
                            1 as libc::c_int as bool_);
        }
        /* Remove trap attr from grid */
        note_spot(y, x);
        lite_spot(y, x);
    } else if i > 5 as libc::c_int &&
                  Rand_div(i) + 1 as libc::c_int > 5 as libc::c_int {
        /* Failure -- Keep trying */
        /* Failure */
        if flush_failure != 0 { flush(); }
        /* Message */
        msg_format(b"You failed to disarm the %s.\x00" as *const u8 as
                       *const libc::c_char, name);
        /* We may keep trying */
        more = 1 as libc::c_int as bool_
    } else {
        /* Failure -- Set off the trap */
        /* Message */
        msg_format(b"You set off the %s!\x00" as *const u8 as
                       *const libc::c_char, name);
        if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
               0x1000 as libc::c_long == 0 {
            move_player_aux(dir, do_pickup, 0 as libc::c_int,
                            0 as libc::c_int as bool_);
        }
    }
    /* Move the player onto the trap */
    /* Result */
    return more;
}
/*
 * Disamrs the monster traps(no failure)
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_disarm_mon_trap(mut y: libc::c_int,
                                                mut x: libc::c_int) {
    msg_print(b"You disarm the monster trap.\x00" as *const u8 as
                  *const libc::c_char);
    place_floor_convert_glass(y, x);
    let ref mut fresh0 =
        (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).special2;
    *fresh0 = 0 as libc::c_int as s16b;
    (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).special =
        *fresh0;
}
/*
 * Disarms a trap, or chest
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_disarm() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut o_idx: s16b = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    /* Option: Pick a direction */
    if easy_disarm != 0 {
        let mut num_traps: libc::c_int = 0;
        let mut num_chests: libc::c_int = 0;
        /* Count visible traps */
        num_traps =
            count_feats(&mut y, &mut x,
                        Some(is_trap as
                                 unsafe extern "C" fn(_: *mut cave_type)
                                     -> bool_), 1 as libc::c_int as bool_);
        /* Count chests (trapped) */
        num_chests = count_chests(&mut y, &mut x, 1 as libc::c_int as bool_);
        /* See if only one target */
        if num_traps != 0 || num_chests != 0 {
            if num_traps + num_chests <= 1 as libc::c_int {
                command_dir = coords_to_dir(y, x) as s16b
            }
        }
    }
    /* Allow repeated command */
    if command_arg != 0 {
        /* Set repeat count */
        command_rep = (command_arg as libc::c_int - 1 as libc::c_int) as s16b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Cancel the arg */
        command_arg = 0 as libc::c_int as s16b
    }
    /* Get a direction (or abort) */
    if get_rep_dir(&mut dir) != 0 {
        /* Get location */
        y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
        x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
        /* Get grid and contents */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Check for chests */
        o_idx = chest_check(y, x);
        /* Disarm a trap */
        if ((*c_ptr).t_idx as libc::c_int == 0 as libc::c_int ||
                (*c_ptr).info as libc::c_int & 0x100 as libc::c_int == 0) &&
               o_idx == 0 &&
               (*c_ptr).feat as libc::c_int != 0xaf as libc::c_int {
            /* Message */
            msg_print(b"You see nothing there to disarm.\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*c_ptr).m_idx != 0 {
            /* Monster in the way */
            /* Message */
            msg_print(b"There is a monster in the way!\x00" as *const u8 as
                          *const libc::c_char);
            /* Attack */
            py_attack(y, x, -(1 as libc::c_int));
        } else if o_idx != 0 {
            /* Disarm chest */
            /* Disarm the chest */
            more = do_cmd_disarm_chest(y, x, o_idx)
        } else if (*c_ptr).feat as libc::c_int == 0xaf as libc::c_int {
            do_cmd_disarm_mon_trap(y, x);
            more = 0 as libc::c_int as bool_
        } else {
            more = do_cmd_disarm_aux(y, x, dir, always_pickup as libc::c_int)
        }
    }
    /* Disarm trap */
    /* Disarm the trap */
    /* Cancel repeat unless told not to */
    if more == 0 { disturb(0 as libc::c_int, 0 as libc::c_int); };
}
/*
 * Perform the basic "bash" command
 *
 * Assume destination is a closed/locked/jammed door
 *
 * Assume there is no monster blocking the destination
 *
 * Returns TRUE if repeated commands may continue
 */
unsafe extern "C" fn do_cmd_bash_aux(mut y: libc::c_int, mut x: libc::c_int,
                                     mut dir: libc::c_int) -> bool_ {
    let mut bash: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    if (*p_ptr).body_monster as libc::c_int != 0 as libc::c_int &&
           (*r_ptr).flags2 & 0x20000 as libc::c_int as libc::c_uint == 0 {
        msg_print(b"You cannot do that.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Get grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Message */
    msg_print(b"You smash into the door!\x00" as *const u8 as
                  *const libc::c_char);
    /* Hack -- Bash power based on strength */
	/* (Ranges from 3 to 20 to 100 to 200) */
    bash =
        *adj_str_blow.as_mut_ptr().offset((*p_ptr).stat_ind[0 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int;
    /* Extract door power */
    temp =
        (*c_ptr).feat as libc::c_int - 0x20 as libc::c_int &
            0x7 as libc::c_int;
    /* Compare bash power to door power XXX XXX XXX */
    temp = bash - temp * 10 as libc::c_int;
    /* Hack -- always have a chance */
    if temp < 1 as libc::c_int { temp = 1 as libc::c_int }
    /* Hack -- attempt to bash down the door */
    if Rand_div(100 as libc::c_int) < temp {
        /* Message */
        msg_print(b"The door crashes open!\x00" as *const u8 as
                      *const libc::c_char);
        /* Break down the door */
        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
            /* Set off trap */
            if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
                player_activate_door_trap(y as s16b, x as s16b);
            }
            cave_set_feat(y, x, 0x5 as libc::c_int);
        } else {
            /* Open the door */
            /* Set off trap */
            if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
                player_activate_door_trap(y as s16b, x as s16b);
            }
            cave_set_feat(y, x, 0x4 as libc::c_int);
        }
        /* Sound */
        sound(22 as libc::c_int);
        /* Hack -- Fall through the door. Can't disarm while falling. */
        move_player_aux(dir, always_pickup as libc::c_int, 0 as libc::c_int,
                        0 as libc::c_int as bool_);
        /* Update some things */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x200000 as libc::c_long)) as
                u32b;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long) as
                u32b
    } else if Rand_div(100 as libc::c_int) <
                  *adj_dex_safe.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                        as isize) as
                      libc::c_int + (*p_ptr).lev as libc::c_int {
        /* Saving throw against stun */
        /* Message */
        msg_print(b"The door holds firm.\x00" as *const u8 as
                      *const libc::c_char);
        /* Allow repeated bashing */
        more = 1 as libc::c_int as bool_
    } else {
        /* High dexterity yields coolness */
        /* Message */
        msg_print(b"You are off-balance.\x00" as *const u8 as
                      *const libc::c_char);
        set_paralyzed((*p_ptr).paralyzed as libc::c_int + 2 as libc::c_int +
                          Rand_div(2 as libc::c_int));
    }
    /* Hack -- Lose balance ala paralysis */
    /* Result */
    return more;
}
/*
 * Bash open a door, success based on character strength
 *
 * For a closed door, pval is positive if locked; negative if stuck.
 *
 * For an open door, pval is positive for a broken door.
 *
 * A closed door can be opened - harder if locked. Any door might be
 * bashed open (and thereby broken). Bashing a door is (potentially)
 * faster! You move into the door way. To open a stuck door, it must
 * be bashed. A closed door can be jammed (see do_cmd_spike()).
 *
 * Creatures can also open or bash doors, see elsewhere.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_bash() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    if (*p_ptr).body_monster as libc::c_int != 0 as libc::c_int &&
           (*r_ptr).flags2 & 0x20000 as libc::c_int as libc::c_uint == 0 {
        msg_print(b"You cannot do that.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Allow repeated command */
    if command_arg != 0 {
        /* Set repeat count */
        command_rep = (command_arg as libc::c_int - 1 as libc::c_int) as s16b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Cancel the arg */
        command_arg = 0 as libc::c_int as s16b
    }
    /* Get a "repeated" direction */
    if get_rep_dir(&mut dir) != 0 {
        /* Bash location */
        y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
        x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
        /* Get grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Nothing useful */
        if (((*c_ptr).feat as libc::c_int) < 0x20 as libc::c_int ||
                (*c_ptr).feat as libc::c_int > 0x2f as libc::c_int) &&
               (((*c_ptr).feat as libc::c_int) < 0xa1 as libc::c_int ||
                    (*c_ptr).feat as libc::c_int > 0xab as libc::c_int) &&
               (*c_ptr).feat as libc::c_int != 0x2 as libc::c_int {
            /* Message */
            msg_print(b"You see nothing there to bash.\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*c_ptr).m_idx != 0 {
            /* Monster in the way */
            /* Take a turn */
            energy_use = 100 as libc::c_int;
            /* Message */
            msg_print(b"There is a monster in the way!\x00" as *const u8 as
                          *const libc::c_char);
            /* Attack */
            py_attack(y, x, -(1 as libc::c_int));
        } else if (*c_ptr).feat as libc::c_int >= 0xa1 as libc::c_int &&
                      (*c_ptr).feat as libc::c_int <= 0xab as libc::c_int {
            more = do_cmd_bash_altar(y, x)
        } else if (*c_ptr).feat as libc::c_int == 0x2 as libc::c_int {
            more = do_cmd_bash_fountain(y, x)
        } else {
            /* Bash a closed door */
            /* Bash the door */
            more = do_cmd_bash_aux(y, x, dir)
        }
    }
    /* Unless valid action taken, cancel bash */
    if more == 0 { disturb(0 as libc::c_int, 0 as libc::c_int); };
}
/*
 * Manipulate an adjacent grid in some way
 *
 * Attack monsters, tunnel through walls, disarm traps, open doors.
 *
 * Consider confusion XXX XXX XXX
 *
 * This command must always take a turn, to prevent free detection
 * of invisible monsters.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_alter() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    /* Allow repeated command */
    if command_arg != 0 {
        /* Set repeat count */
        command_rep = (command_arg as libc::c_int - 1 as libc::c_int) as s16b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Cancel the arg */
        command_arg = 0 as libc::c_int as s16b
    }
    /* Get a direction */
    if get_rep_dir(&mut dir) != 0 {
        /* Get location */
        y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
        x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
        /* Get grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Take a turn */
        energy_use = 100 as libc::c_int;
        /* Attack monsters */
        if (*c_ptr).m_idx != 0 {
            /* Attack */
            py_attack(y, x, -(1 as libc::c_int));
        } else if (*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
                      (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int {
            /* Open closed doors */
            /* Tunnel */
            more = do_cmd_open_aux(y, x, dir)
        } else if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                      libc::c_long & 0x8000 as libc::c_long != 0 {
            /* Tunnel through walls */
            /* Tunnel */
            more = do_cmd_tunnel_aux(y, x, dir)
        } else if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
            /* Disarm traps */
            /* Tunnel */
            more = do_cmd_disarm_aux(y, x, dir, always_pickup as libc::c_int)
        } else {
            /* Oops */
            /* Oops */
            msg_print(b"You attack the empty air.\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    /* Cancel repetition unless we can continue */
    if more == 0 { disturb(0 as libc::c_int, 0 as libc::c_int); };
}
/*
 * Find the index of some "spikes", if possible.
 *
 * XXX XXX XXX Let user choose a pile of spikes, perhaps?
 */
unsafe extern "C" fn get_spike(mut ip: *mut libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    /* Check every item in the pack */
    i = 0 as libc::c_int;
    while i < 23 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Check the "tval" code */
            if (*o_ptr).tval as libc::c_int == 5 as libc::c_int {
                /* Save the spike index */
                *ip = i;
                /* Success */
                return 1 as libc::c_int as bool_
            }
        }
        i += 1
    }
    /* Oops */
    return 0 as libc::c_int as bool_;
}
/*
 * Jam a closed door with a spike
 *
 * This command may NOT be repeated
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_spike() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut item: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Get a "repeated" direction */
    if get_rep_dir(&mut dir) != 0 {
        /* Get location */
        y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
        x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
        /* Get grid and contents */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Require closed door */
        if !((*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
                 (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int) {
            /* Message */
            msg_print(b"You see nothing there to spike.\x00" as *const u8 as
                          *const libc::c_char);
        } else if get_spike(&mut item) == 0 {
            /* Get a spike */
            /* Message */
            msg_print(b"You have no spikes!\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*c_ptr).m_idx != 0 {
            /* Is a monster in the way? */
            /* Take a turn */
            energy_use = 100 as libc::c_int;
            /* Message */
            msg_print(b"There is a monster in the way!\x00" as *const u8 as
                          *const libc::c_char);
            /* Attack */
            py_attack(y, x, -(1 as libc::c_int));
        } else {
            /* Go for it */
            /* Take a turn */
            energy_use = 100 as libc::c_int;
            msg_print(b"You jam the door with a spike.\x00" as *const u8 as
                          *const libc::c_char);
            if ((*c_ptr).feat as libc::c_int) <
                   0x20 as libc::c_int + 0x8 as libc::c_int {
                (*c_ptr).feat =
                    ((*c_ptr).feat as libc::c_int + 0x8 as libc::c_int) as
                        byte_hack
            }
            if ((*c_ptr).feat as libc::c_int) < 0x2f as libc::c_int {
                (*c_ptr).feat = (*c_ptr).feat.wrapping_add(1)
            }
            inc_stack_size(item, -(1 as libc::c_int));
        }
    };
}
unsafe extern "C" fn do_cmd_walk_jump(mut pickup: libc::c_int,
                                      mut disarm: bool_) {
    let mut dir: libc::c_int = 0;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    /* Successful jamming */
    /* Convert "locked" to "stuck" XXX XXX XXX */
    /* Add one spike to the door */
    /* Use up, and describe, a single spike, from the bottom */
    /* Allow repeated command */
    if command_arg != 0 {
        /* Set repeat count */
        command_rep = (command_arg as libc::c_int - 1 as libc::c_int) as s16b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Cancel the arg */
        command_arg = 0 as libc::c_int as s16b
    }
    /* Get a "repeated" direction */
    if get_rep_dir(&mut dir) != 0 {
        /* Take a turn */
        energy_use = 100 as libc::c_int;
        /* Actually move the character */
        move_player(dir, pickup, disarm);
        /* Allow more walking */
        more = 1 as libc::c_int as bool_
    }
    /* Hack -- In small scale wilderness it takes MUCH more time to move */
    energy_use *=
        if (*p_ptr).wild_mode as libc::c_int != 0 {
            (66 as libc::c_int + 198 as libc::c_int) / 2 as libc::c_int
        } else { 1 as libc::c_int };
    /* Hack again -- Is there a special encounter ??? */
    if (*p_ptr).wild_mode as libc::c_int != 0 &&
           Rand_div(100 as libc::c_int) <
               (*wf_info.offset((*(*wild_map.offset((*p_ptr).py as
                                                        isize)).offset((*p_ptr).px
                                                                           as
                                                                           isize)).feat
                                    as isize)).level -
                   (*p_ptr).lev as libc::c_int * 2 as libc::c_int {
        /* Go into large wilderness view */
        (*p_ptr).wilderness_x = (*p_ptr).px as s32b;
        (*p_ptr).wilderness_y = (*p_ptr).py as s32b;
        energy_use = 100 as libc::c_int;
        change_wild_mode();
        /* HACk -- set the encouter flag for the wilderness generation */
        generate_encounter = 1 as libc::c_int as bool_;
        (*p_ptr).oldpx = (198 as libc::c_int / 2 as libc::c_int) as s16b;
        (*p_ptr).oldpy = (66 as libc::c_int / 2 as libc::c_int) as s16b;
        /* Inform the player of his horrible fate :=) */
        msg_print(b"You are ambushed!\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Cancel repeat unless we may continue */
    if more == 0 { disturb(0 as libc::c_int, 0 as libc::c_int); };
}
/*
 * Support code for the "Walk" and "Jump" commands
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_walk(mut pickup: libc::c_int,
                                     mut disarm: bool_) {
    /* Move (usually pickup) */
    if (*p_ptr).immovable != 0 {
        do_cmd_unwalk();
    } else { do_cmd_walk_jump(pickup, disarm); };
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_run_run() {
    let mut dir: libc::c_int = 0;
    /* Hack -- no running when confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Get a "repeated" direction */
    if get_rep_dir(&mut dir) != 0 {
        /* Hack -- Set the run counter */
        running =
            if command_arg as libc::c_int != 0 {
                command_arg as libc::c_int
            } else { 1000 as libc::c_int } as s16b;
        /* First step */
        run_step(dir);
    }
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Start running.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_run() {
    if (*p_ptr).immovable != 0 { return } else { do_cmd_run_run(); };
}
/*
 * Stay still.  Search.  Enter stores.
 * Pick up treasure if "pickup" is true.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_stay(mut pickup: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                             isize)).offset((*p_ptr).px as
                                                                isize) as
            *mut cave_type;
    /* Allow repeated command */
    if command_arg != 0 {
        /* Set repeat count */
        command_rep = (command_arg as libc::c_int - 1 as libc::c_int) as s16b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Cancel the arg */
        command_arg = 0 as libc::c_int as s16b
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Spontaneous Searching */
    if (*p_ptr).skill_fos as libc::c_int >= 50 as libc::c_int ||
           0 as libc::c_int ==
               Rand_div(50 as libc::c_int - (*p_ptr).skill_fos as libc::c_int)
       {
        search();
    }
    /* Continuous Searching */
    if (*p_ptr).searching != 0 { search(); }
    /* Handle "objects" */
    carry(pickup);
    /* Hack -- enter a store if we are on one */
    if (*c_ptr).feat as libc::c_int == 0x4a as libc::c_int {
        /* Disturb */
        disturb(0 as libc::c_int, 0 as libc::c_int);
        /* Hack -- enter store */
        command_new = '_' as i32 as s16b
    };
}
/*
 * Resting allows a player to safely restore his hp	-RAK-
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_rest() {
    /* Can't rest on a Void Jumpgate -- too dangerous */
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
           libc::c_int == 0xa0 as libc::c_int {
        /* 'R&\n' is one of our favourite macros, so we have to do this */
        if flush_failure != 0 { flush(); }
        /* Tell the player why */
        msg_print(format(b"Resting on a %s is too dangerous!\x00" as *const u8
                             as *const libc::c_char,
                         f_name.offset((*f_info.offset((*cave[(*p_ptr).py as
                                                                  usize].offset((*p_ptr).px
                                                                                    as
                                                                                    isize)).feat
                                                           as isize)).name as
                                           isize)) as cptr);
        /* Done */
        return
    }
    /* Can't rest while undead, it would mean dying */
    if (*p_ptr).necro_extra & 0x8 as libc::c_int as libc::c_uint != 0 {
        /* 'R&\n' is one of our favourite macros, so we have to do this */
        if flush_failure != 0 { flush(); }
        /* Tell the player why */
        msg_print(b"Resting is impossible while undead!\x00" as *const u8 as
                      *const libc::c_char);
        /* Done */
        return
    }
    /* Prompt for time if needed */
    if command_arg as libc::c_int <= 0 as libc::c_int {
        let mut p: cptr =
            b"Rest (0-9999, \'*\' for HP/SP, \'&\' as needed): \x00" as
                *const u8 as *const libc::c_char;
        let mut out_val: [libc::c_char; 80] = [0; 80];
        /* Default */
        strcpy(out_val.as_mut_ptr(),
               b"&\x00" as *const u8 as *const libc::c_char);
        /* Ask for duration */
        if get_string(p, out_val.as_mut_ptr(), 4 as libc::c_int) == 0 {
            return
        }
        /* Rest until done */
        if out_val[0 as libc::c_int as usize] as libc::c_int == '&' as i32 {
            command_arg = -(2 as libc::c_int) as s16b
        } else if out_val[0 as libc::c_int as usize] as libc::c_int ==
                      '*' as i32 {
            command_arg = -(1 as libc::c_int) as s16b
        } else {
            /* Rest a lot */
            /* Rest some */
            command_arg = atoi(out_val.as_mut_ptr()) as s16b;
            if command_arg as libc::c_int <= 0 as libc::c_int { return }
        }
    }
    /* Paranoia */
    if command_arg as libc::c_int > 9999 as libc::c_int {
        command_arg = 9999 as libc::c_int as s16b
    }
    /* Take a turn XXX XXX XXX (?) */
    energy_use = 100 as libc::c_int;
    /* Save the rest code */
    resting = command_arg;
    /* Cancel searching */
    (*p_ptr).searching = 0 as libc::c_int as byte_hack;
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Redraw the state */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
    /* Refresh */
    Term_fresh();
}
/*
 * Determines the odds of an object breaking when thrown at a monster
 *
 * Note that artifacts never break, see the "drop_near()" function.
 */
#[no_mangle]
pub unsafe extern "C" fn breakage_chance(mut o_ptr: *mut object_type)
 -> libc::c_int {
    let mut reducer: libc::c_int =
        1 as libc::c_int +
            (if get_skill(23 as libc::c_int) as libc::c_int != 0 {
                 get_skill_scale(23 as libc::c_int, 10 as libc::c_int as u32b)
                     as libc::c_int
             } else { 0 as libc::c_int });
    /* Examine the item type */
    match (*o_ptr).tval as libc::c_int {
        77 | 71 | 72 | 2 | 80 => {
            /* Always break */
            return 100 as libc::c_int
        }
        39 | 70 | 1 => {
            /* Often break */
            return 50 as libc::c_int
        }
        17 => { return 50 as libc::c_int / reducer }
        65 | 5 => {
            /* Sometimes break */
            return 25 as libc::c_int
        }
        16 | 18 => { return 25 as libc::c_int / reducer }
        15 => { return 1 as libc::c_int }
        _ => { }
    }
    /* Rarely break */
    return 10 as libc::c_int;
}
/*
 * Return multiplier of an object
 */
#[no_mangle]
pub unsafe extern "C" fn get_shooter_mult(mut o_ptr: *mut object_type)
 -> libc::c_int {
    /* Assume a base multiplier */
    let mut tmul: libc::c_int = 1 as libc::c_int;
    /* Analyze the launcher */
    match (*o_ptr).sval as libc::c_int {
        2 => {
            /* Sling and ammo */
            tmul = 2 as libc::c_int
        }
        12 => {
            /* Short Bow and Arrow */
            tmul = 2 as libc::c_int
        }
        13 => {
            /* Long Bow and Arrow */
            tmul = 3 as libc::c_int
        }
        23 => {
            /* Light Crossbow and Bolt */
            tmul = 3 as libc::c_int
        }
        24 => {
            /* Heavy Crossbow and Bolt */
            tmul = 4 as libc::c_int
        }
        _ => { }
    }
    return tmul;
}
/*
 * Fire an object from the pack or floor.
 *
 * You may only fire items that "match" your missile launcher.
 *
 * You must use slings + pebbles/shots, bows + arrows, xbows + bolts.
 *
 * See "calc_bonuses()" for more calculations and such.
 *
 * Note that "firing" a missile is MUCH better than "throwing" it.
 *
 * Note: "unseen" monsters are very hard to hit.
 *
 * Objects are more likely to break if they "attempt" to hit a monster.
 *
 * Rangers (with Bows) and Anyone (with "Extra Shots") get extra shots.
 *
 * The "extra shot" code works by decreasing the amount of energy
 * required to make each shot, spreading the shots out over time.
 *
 * Note that when firing missiles, the launcher multiplier is applied
 * after all the bonuses are added in, making multipliers very useful.
 *
 * Note that Bows of "Extra Might" get extra range and an extra bonus
 * for the damage multiplier.
 *
 * Note that Bows of "Extra Shots" give an extra shot.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_fire() {
    let mut dir: libc::c_int = 0;
    let mut item: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut bx: libc::c_int = 0;
    let mut oldtdam: libc::c_int = 0;
    let mut tdam: libc::c_int = 0;
    let mut tdis: libc::c_int = 0;
    let mut thits: libc::c_int = 0;
    let mut tmul: libc::c_int = 0;
    let mut bonus: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut cur_dis: libc::c_int = 0;
    let mut visible: libc::c_int = 0;
    let mut breakage: libc::c_int = -(1 as libc::c_int);
    let mut num_pierce: libc::c_int = 0 as libc::c_int;
    let mut special: s32b = 0 as libc::c_int;
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
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    let mut hit_body: bool_ = 0 as libc::c_int as bool_;
    let mut missile_attr: byte_hack = 0;
    let mut missile_char: libc::c_char = 0;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut msec: libc::c_int =
        delay_factor as libc::c_int * delay_factor as libc::c_int *
            delay_factor as libc::c_int;
    /* Get the "bow" (if any) */
    j_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(27 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Require a launcher */
    if (*j_ptr).tval == 0 {
        msg_print(b"You have nothing with which to fire.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* XXX HACK */
    if (*j_ptr).tval as libc::c_int == 14 as libc::c_int {
        msg_print(b"You cannot fire with an instrument.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Get the "ammo" (if any) */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(50 as libc::c_int as
                                                         isize) as
            *mut object_type;
    item = 50 as libc::c_int;
    /* If nothing correct try to choose from the backpack */
    if (*p_ptr).tval_ammo as libc::c_int != (*o_ptr).tval as libc::c_int ||
           (*o_ptr).k_idx == 0 {
        /* Require proper missile */
        item_tester_tval = (*p_ptr).tval_ammo;
        /* Get an item */
        q =
            b"Your quiver is empty.  Fire which item? \x00" as *const u8 as
                *const libc::c_char;
        s =
            b"You have nothing to fire.\x00" as *const u8 as
                *const libc::c_char;
        if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int)
               == 0 {
            return
        }
        /* Access the item */
        o_ptr = get_object(item)
    }
    /* Get a direction (or cancel) */
    if get_aim_dir(&mut dir) == 0 { return }
    /* Get local object */
    q_ptr = &mut forge;
    /* Obtain a local object */
    object_copy(q_ptr, o_ptr);
    /* Single object */
    (*q_ptr).number = 1 as libc::c_int as byte_hack;
    /* Reduce stack and describe */
    inc_stack_size(item, -(1 as libc::c_int));
    /* Break goi/manashield */
    if (*p_ptr).invuln != 0 { set_invuln(0 as libc::c_int); }
    if (*p_ptr).disrupt_shield != 0 { set_disrupt_shield(0 as libc::c_int); }
    /* Sound */
    sound(10 as libc::c_int);
    /* Describe the object */
    object_desc(o_name.as_mut_ptr(), q_ptr, 0 as libc::c_int,
                3 as libc::c_int);
    /* Find the color and symbol for the object for throwing */
    missile_attr =
        if (*q_ptr).tval as libc::c_int == 102 as libc::c_int {
            random_artifacts[(*q_ptr).sval as usize].attr as libc::c_int
        } else if (*k_info.offset((*q_ptr).k_idx as isize)).flavor as
                      libc::c_int != 0 {
            misc_to_attr[(*k_info.offset((*q_ptr).k_idx as isize)).flavor as
                             usize] as libc::c_int
        } else {
            (*k_info.offset((*q_ptr).k_idx as isize)).x_attr as libc::c_int
        } as byte_hack;
    missile_char =
        if (*k_info.offset((*q_ptr).k_idx as isize)).flavor as libc::c_int !=
               0 {
            misc_to_char[(*k_info.offset((*q_ptr).k_idx as isize)).flavor as
                             usize] as libc::c_int
        } else {
            (*k_info.offset((*q_ptr).k_idx as isize)).x_char as libc::c_int
        } as libc::c_char;
    /* Use the proper number of shots */
    thits = (*p_ptr).num_fire as libc::c_int;
    /* Use a base distance */
    tdis = 10 as libc::c_int;
    /* Base damage from thrown object plus launcher bonus */
    tdam =
        damroll((*q_ptr).dd as s16b, (*q_ptr).ds as s16b) +
            (*q_ptr).to_d as libc::c_int + (*j_ptr).to_d as libc::c_int;
    /* Actually "fire" the object */
    bonus =
        (*p_ptr).to_h as libc::c_int + (*p_ptr).to_h_ranged as libc::c_int +
            (*q_ptr).to_h as libc::c_int + (*j_ptr).to_h as libc::c_int;
    chance = (*p_ptr).skill_thb as libc::c_int + bonus * 3 as libc::c_int;
    if chance < 5 as libc::c_int { chance = 5 as libc::c_int }
    tmul = get_shooter_mult(j_ptr);
    /* Get extra "power" from "extra might" */
    tmul += (*p_ptr).xtra_might as libc::c_int;
    /* Boost the damage */
    tdam *= tmul;
    /* Add in the player damage */
    tdam += (*p_ptr).to_d_ranged as libc::c_int;
    /* Base range */
    tdis = 10 as libc::c_int + 5 as libc::c_int * tmul;
    /* Take a (partial) turn */
    energy_use = 100 as libc::c_int / thits;
    /* piercing shots ? */
    if (*p_ptr).use_piercing_shots != 0 {
        num_pierce =
            get_skill(16 as libc::c_int) as libc::c_int / 10 as libc::c_int -
                1 as libc::c_int;
        num_pierce =
            if num_pierce < 0 as libc::c_int {
                0 as libc::c_int
            } else { num_pierce }
    }
    /* Start at the player */
    by = (*p_ptr).py as libc::c_int;
    bx = (*p_ptr).px as libc::c_int;
    y = (*p_ptr).py as libc::c_int;
    x = (*p_ptr).px as libc::c_int;
    /* Predict the "target" location */
    tx =
        (*p_ptr).px as libc::c_int +
            99 as libc::c_int * ddx[dir as usize] as libc::c_int;
    ty =
        (*p_ptr).py as libc::c_int +
            99 as libc::c_int * ddy[dir as usize] as libc::c_int;
    /* Check for "target request" */
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int
    }
    /* Hack -- Handle stuff */
    handle_stuff();
    oldtdam = tdam;
    loop  {
        /* Reset after a piercing shot */
        tdam = oldtdam;
        /* Travel until stopped */
        cur_dis = 0 as libc::c_int;
        while cur_dis <= tdis {
            /* Hack -- Stop at the target */
            if y == ty && x == tx { break ; }
            /* Calculate the new location (see "project()") */
            ny = y;
            nx = x;
            mmove2(&mut ny, &mut nx, by, bx, ty, tx);
            /* Stopped by walls/doors */
            if !((*f_info.offset((*cave[ny as usize].offset(nx as isize)).feat
                                     as isize)).flags1 as libc::c_long &
                     0x10 as libc::c_long != 0 &&
                     (*cave[ny as usize].offset(nx as isize)).feat as
                         libc::c_int != 0xaf as libc::c_int) {
                break ;
            }
            /* Advance the distance */
            cur_dis += 1;
            /* Save the new location */
            x = nx;
            y = ny;
            /* The player can see the (on screen) missile */
            if y >= panel_row_min as libc::c_int &&
                   y <= panel_row_max as libc::c_int &&
                   x >= panel_col_min as libc::c_int &&
                   x <= panel_col_max as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x10 as libc::c_int != 0 as libc::c_int {
                /* Draw, Hilite, Fresh, Pause, Erase */
                print_rel(missile_char, missile_attr, y, x);
                move_cursor_relative(y, x);
                Term_fresh();
                Term_xtra(13 as libc::c_int, msec);
                lite_spot(y, x);
                Term_fresh();
            } else {
                /* The player cannot see the missile */
                /* Pause anyway, for consistancy */
                Term_xtra(13 as libc::c_int, msec);
            }
            /* Monster here, Try to hit it */
            if !((*cave[y as usize].offset(x as isize)).m_idx != 0) {
                continue ;
            }
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
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
            /* Check the visibility */
            visible = (*m_ptr).ml as libc::c_int;
            /* Note the collision */
            hit_body = 1 as libc::c_int as bool_;
            /* Did we hit it (penalize range) */
            if test_hit_fire(chance - cur_dis, (*m_ptr).ac as libc::c_int,
                             (*m_ptr).ml as libc::c_int) != 0 {
                let mut fear: bool_ = 0 as libc::c_int as bool_;
                /* Assume a default death */
                let mut note_dies: cptr =
                    b" dies.\x00" as *const u8 as *const libc::c_char;
                /* Some monsters get "destroyed" */
                if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
                       ||
                       (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint
                           != 0 ||
                       (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint !=
                           0 ||
                       !strchr(b"Evg\x00" as *const u8 as *const libc::c_char,
                               (*r_ptr).d_char as libc::c_int).is_null() {
                    /* Special note at death */
                    note_dies =
                        b" is destroyed.\x00" as *const u8 as
                            *const libc::c_char
                }
                /* Handle unseen monster */
                if visible == 0 {
                    /* Invisible monster */
                    msg_format(b"The %s finds a mark.\x00" as *const u8 as
                                   *const libc::c_char, o_name.as_mut_ptr());
                } else {
                    /* Handle visible monster */
                    let mut m_name: [libc::c_char; 80] = [0; 80];
                    /* Get "the monster" or "it" */
                    monster_desc(m_name.as_mut_ptr(), m_ptr,
                                 0 as libc::c_int);
                    /* Message */
                    msg_format(b"The %s hits %s.\x00" as *const u8 as
                                   *const libc::c_char, o_name.as_mut_ptr(),
                               m_name.as_mut_ptr());
                    /* Hack -- Track this monster race */
                    if (*m_ptr).ml != 0 {
                        monster_race_track((*m_ptr).r_idx as libc::c_int,
                                           (*m_ptr).ego as libc::c_int);
                    }
                    /* Hack -- Track this monster */
                    if (*m_ptr).ml != 0 {
                        health_track((*c_ptr).m_idx as libc::c_int);
                    }
                    /* Anger friends */
                    let mut m_name_0: [libc::c_char; 80] = [0; 80];
                    monster_desc(m_name_0.as_mut_ptr(), m_ptr,
                                 0 as libc::c_int);
                    match is_friend(m_ptr) {
                        1 => {
                            msg_format(b"%^s gets angry!\x00" as *const u8 as
                                           *const libc::c_char,
                                       m_name_0.as_mut_ptr());
                            change_side(m_ptr);
                        }
                        0 => {
                            msg_format(b"%^s gets angry!\x00" as *const u8 as
                                           *const libc::c_char,
                                       m_name_0.as_mut_ptr());
                            (*m_ptr).status = -(1 as libc::c_int) as s16b
                        }
                        _ => { }
                    }
                }
                /* Apply special damage XXX XXX XXX */
                tdam =
                    tot_dam_aux(q_ptr, tdam, m_ptr, &mut special) as
                        libc::c_int;
                tdam =
                    critical_shot((*q_ptr).weight,
                                  (*q_ptr).to_h as libc::c_int, tdam,
                                  23 as libc::c_int) as libc::c_int;
                /* No negative damage */
                if tdam < 0 as libc::c_int { tdam = 0 as libc::c_int }
                /* Complex message */
                if wizard != 0 {
                    msg_format(b"You do %d (out of %d) damage.\x00" as
                                   *const u8 as *const libc::c_char, tdam,
                               (*m_ptr).hp);
                }
                /* Hit the monster, check for death */
                if !(mon_take_hit((*c_ptr).m_idx as libc::c_int, tdam,
                                  &mut fear, note_dies) != 0) {
                    /* No death */
                    /* Message */
                    message_pain((*c_ptr).m_idx as libc::c_int, tdam);
                    if special != 0 { attack_special(m_ptr, special, tdam); }
                    if fear as libc::c_int != 0 &&
                           (*m_ptr).ml as libc::c_int != 0 {
                        let mut m_name_1: [libc::c_char; 80] = [0; 80];
                        /* Sound */
                        sound(3 as libc::c_int);
                        /* Get the monster name (or "it") */
                        monster_desc(m_name_1.as_mut_ptr(), m_ptr,
                                     0 as libc::c_int);
                        /* Message */
                        msg_format(b"%^s flees in terror!\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name_1.as_mut_ptr());
                    }
                }
            }
            break ;
        }
        /* Take note */
        /* Exploding arrow ? */
        if (*q_ptr).pval2 as libc::c_int != 0 as libc::c_int {
            let mut rad: libc::c_int = 0 as libc::c_int;
            let mut dam: libc::c_int =
                (damroll((*q_ptr).dd as s16b, (*q_ptr).ds as s16b) +
                     (*q_ptr).to_d as libc::c_int) * 2 as libc::c_int;
            let mut flag: libc::c_int =
                0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
                    | 0x40 as libc::c_int | 0x1 as libc::c_int;
            match (*q_ptr).sval as libc::c_int {
                0 => { rad = 2 as libc::c_int; dam /= 2 as libc::c_int }
                1 => { rad = 3 as libc::c_int }
                2 => { rad = 4 as libc::c_int; dam *= 2 as libc::c_int }
                _ => { }
            }
            project(0 as libc::c_int, rad, y, x, dam,
                    (*q_ptr).pval2 as libc::c_int, flag);
        }
        /* Chance of breakage (during attacks) */
        j =
            if hit_body as libc::c_int != 0 {
                breakage_chance(q_ptr)
            } else { 0 as libc::c_int };
        /* Break ? */
        if (*q_ptr).pval2 as libc::c_int != 0 as libc::c_int ||
               Rand_div(100 as libc::c_int) < j {
            breakage = 100 as libc::c_int;
            break ;
        } else {
            /* If the ammo doesn't break, it can pierce through */
            if !(num_pierce != 0 && hit_body as libc::c_int != 0 &&
                     Rand_div(100 as libc::c_int) <
                         45 as libc::c_int +
                             get_skill(23 as libc::c_int) as libc::c_int) {
                break ;
            }
            num_pierce -= 1;
            hit_body = 0 as libc::c_int as bool_;
            /* If target isn't reached, continue moving to target */
            if !(tx < x && x < bx || bx < x && x < tx) &&
                   !(ty < y && y < by || by < y && y < ty) {
                /* Continue moving in same direction if we reached the target */
                let mut dx: libc::c_int = tx - bx;
                let mut dy: libc::c_int = ty - by;
                tx = x + 99 as libc::c_int * dx;
                ty = y + 99 as libc::c_int * dy;
                /* New base location */
                by = y;
                bx = x
            }
            msg_format(b"The %s pierces through!\x00" as *const u8 as
                           *const libc::c_char, o_name.as_mut_ptr());
        }
    }
    /* Drop (or break) near that location */
    drop_near(q_ptr, breakage, y, x);
}
/*
 * Why is this here? even if it's temporary boost...
 * Moved into player_type, hoping it might be useful in future extensions
 * -- pelpel
 */
/* int throw_mult = 1; */
/*
 * Throw an object from the pack or floor.
 *
 * Note: "unseen" monsters are very hard to hit.
 *
 * Should throwing a weapon do full damage?  Should it allow the magic
 * to hit bonus of the weapon to have an effect?  Should it ever cause
 * the item to be destroyed?  Should it do any damage at all?
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_throw() {
    let mut dir: libc::c_int = 0;
    let mut item: libc::c_int = 0;
    let mut special: s32b = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut tdam: libc::c_int = 0;
    let mut tdis: libc::c_int = 0;
    let mut mul: libc::c_int = 0;
    let mut div: libc::c_int = 0;
    let mut boulder_add: libc::c_int = 0 as libc::c_int;
    let mut boulder_mult: libc::c_int = 0 as libc::c_int;
    let mut cur_dis: libc::c_int = 0;
    let mut visible: libc::c_int = 0;
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
    let mut hit_body: bool_ = 0 as libc::c_int as bool_;
    let mut hit_wall: bool_ = 0 as libc::c_int as bool_;
    let mut missile_attr: byte_hack = 0;
    let mut missile_char: libc::c_char = 0;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut msec: libc::c_int =
        delay_factor as libc::c_int * delay_factor as libc::c_int *
            delay_factor as libc::c_int;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Get an item */
    q = b"Throw which item? \x00" as *const u8 as *const libc::c_char;
    s = b"You have nothing to throw.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Access the item */
    o_ptr = get_object(item);
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Hack - Cannot throw away 'no drop' cursed items */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 &&
           f4 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        /* Oops */
        msg_print(b"Hmmm, you seem to be unable to throw it.\x00" as *const u8
                      as *const libc::c_char);
        /* Nope */
        return
    }
    /* Boulder throwing */
    if (*o_ptr).tval as libc::c_int == 11 as libc::c_int &&
           (*o_ptr).sval as libc::c_int == 1 as libc::c_int &&
           get_skill(58 as libc::c_int) as libc::c_int != 0 {
        boulder_add =
            get_skill_scale(58 as libc::c_int, 80 as libc::c_int as u32b) as
                libc::c_int;
        boulder_mult =
            get_skill_scale(58 as libc::c_int, 6 as libc::c_int as u32b) as
                libc::c_int
    }
    /* Get a direction (or cancel) */
    if get_aim_dir(&mut dir) == 0 { return }
    /* Break goi/manashield */
    if (*p_ptr).invuln != 0 { set_invuln(0 as libc::c_int); }
    if (*p_ptr).disrupt_shield != 0 { set_disrupt_shield(0 as libc::c_int); }
    /* Get local object */
    q_ptr = &mut forge;
    /* Obtain a local object */
    object_copy(q_ptr, o_ptr);
    /*
	 * Hack -- If rods or wands are thrown, the total maximum timeout or
	 * charges need to be allocated between the two stacks.
	 */
    if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
        (*q_ptr).pval = (*o_ptr).pval / (*o_ptr).number as libc::c_int;
        if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
            (*o_ptr).pval -= (*q_ptr).pval
        }
    }
    /* Single object */
    (*q_ptr).number = 1 as libc::c_int as byte_hack;
    /* Reduce stack and describe */
    inc_stack_size(item, -(1 as libc::c_int));
    /* Description */
    object_desc(o_name.as_mut_ptr(), q_ptr, 0 as libc::c_int,
                3 as libc::c_int);
    /* Find the color and symbol for the object for throwing */
    missile_attr =
        if (*q_ptr).tval as libc::c_int == 102 as libc::c_int {
            random_artifacts[(*q_ptr).sval as usize].attr as libc::c_int
        } else if (*k_info.offset((*q_ptr).k_idx as isize)).flavor as
                      libc::c_int != 0 {
            misc_to_attr[(*k_info.offset((*q_ptr).k_idx as isize)).flavor as
                             usize] as libc::c_int
        } else {
            (*k_info.offset((*q_ptr).k_idx as isize)).x_attr as libc::c_int
        } as byte_hack;
    missile_char =
        if (*k_info.offset((*q_ptr).k_idx as isize)).flavor as libc::c_int !=
               0 {
            misc_to_char[(*k_info.offset((*q_ptr).k_idx as isize)).flavor as
                             usize] as libc::c_int
        } else {
            (*k_info.offset((*q_ptr).k_idx as isize)).x_char as libc::c_int
        } as libc::c_char;
    /* Extract a "distance multiplier" */
	/* Changed for 'launcher' corruption */
    mul =
        10 as libc::c_int +
            2 as libc::c_int *
                ((*p_ptr).throw_mult as libc::c_int - 1 as libc::c_int) +
            2 as libc::c_int * boulder_mult;
    /* Enforce a minimum "weight" of one pound */
    div =
        if (*q_ptr).weight > 10 as libc::c_int {
            (*q_ptr).weight
        } else { 10 as libc::c_int };
    /* Hack -- Distance -- Reward strength, penalize weight */
    tdis =
        (*adj_str_blow.as_mut_ptr().offset((*p_ptr).stat_ind[0 as libc::c_int
                                                                 as usize] as
                                               isize) as libc::c_int +
             20 as libc::c_int) * mul / div;
    /* Max distance of 10-18 */
    if tdis > mul { tdis = mul }
    /* Hack -- Base damage from thrown object */
    tdam =
        damroll((*q_ptr).dd as s16b, (*q_ptr).ds as s16b) +
            (*q_ptr).to_d as libc::c_int + boulder_add;
    tdam *= (*p_ptr).throw_mult as libc::c_int + boulder_mult;
    /* Chance of hitting - adjusted for Weaponmasters -- Gumby */
    chance =
        (*p_ptr).skill_tht as libc::c_int +
            (*p_ptr).to_h as libc::c_int * 3 as libc::c_int;
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Start at the player */
    y = (*p_ptr).py as libc::c_int;
    x = (*p_ptr).px as libc::c_int;
    /* Predict the "target" location */
    tx =
        (*p_ptr).px as libc::c_int +
            99 as libc::c_int * ddx[dir as usize] as libc::c_int;
    ty =
        (*p_ptr).py as libc::c_int +
            99 as libc::c_int * ddy[dir as usize] as libc::c_int;
    /* Check for "target request" */
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int
    }
    /* Hack -- Handle stuff */
    handle_stuff();
    /* Travel until stopped */
    cur_dis = 0 as libc::c_int;
    while cur_dis <= tdis {
        /* Hack -- Stop at the target */
        if y == ty && x == tx { break ; }
        /* Calculate the new location (see "project()") */
        ny = y;
        nx = x;
        mmove2(&mut ny, &mut nx, (*p_ptr).py as libc::c_int,
               (*p_ptr).px as libc::c_int, ty, tx);
        /* Stopped by walls/doors */
        if !((*f_info.offset((*cave[ny as usize].offset(nx as isize)).feat as
                                 isize)).flags1 as libc::c_long &
                 0x10 as libc::c_long != 0 &&
                 (*cave[ny as usize].offset(nx as isize)).feat as libc::c_int
                     != 0xaf as libc::c_int) {
            hit_wall = 1 as libc::c_int as bool_;
            break ;
        } else {
            /* Advance the distance */
            cur_dis += 1;
            /* Save the new location */
            x = nx;
            y = ny;
            /* The player can see the (on screen) missile */
            if y >= panel_row_min as libc::c_int &&
                   y <= panel_row_max as libc::c_int &&
                   x >= panel_col_min as libc::c_int &&
                   x <= panel_col_max as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x10 as libc::c_int != 0 as libc::c_int {
                /* Draw, Hilite, Fresh, Pause, Erase */
                print_rel(missile_char, missile_attr, y, x);
                move_cursor_relative(y, x);
                Term_fresh();
                Term_xtra(13 as libc::c_int, msec);
                lite_spot(y, x);
                Term_fresh();
            } else {
                /* The player cannot see the missile */
                /* Pause anyway, for consistancy */
                Term_xtra(13 as libc::c_int, msec);
            }
            /* Monster here, Try to hit it */
            if !((*cave[y as usize].offset(x as isize)).m_idx != 0) {
                continue ;
            }
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
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
            /* Check the visibility */
            visible = (*m_ptr).ml as libc::c_int;
            /* Note the collision */
            hit_body = 1 as libc::c_int as bool_;
            /* Did we hit it (penalize range) */
            if test_hit_fire(chance - cur_dis, (*m_ptr).ac as libc::c_int,
                             (*m_ptr).ml as libc::c_int) != 0 {
                let mut fear: bool_ = 0 as libc::c_int as bool_;
                /* Assume a default death */
                let mut note_dies: cptr =
                    b" dies.\x00" as *const u8 as *const libc::c_char;
                /* Some monsters get "destroyed" */
                if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
                       ||
                       (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint
                           != 0 ||
                       (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint !=
                           0 ||
                       !strchr(b"Evg\x00" as *const u8 as *const libc::c_char,
                               (*r_ptr).d_char as libc::c_int).is_null() {
                    /* Special note at death */
                    note_dies =
                        b" is destroyed.\x00" as *const u8 as
                            *const libc::c_char
                }
                /* Handle unseen monster */
                if visible == 0 {
                    /* Invisible monster */
                    msg_format(b"The %s finds a mark.\x00" as *const u8 as
                                   *const libc::c_char, o_name.as_mut_ptr());
                } else {
                    /* Handle visible monster */
                    let mut m_name: [libc::c_char; 80] = [0; 80];
                    /* Get "the monster" or "it" */
                    monster_desc(m_name.as_mut_ptr(), m_ptr,
                                 0 as libc::c_int);
                    /* Message */
                    msg_format(b"The %s hits %s.\x00" as *const u8 as
                                   *const libc::c_char, o_name.as_mut_ptr(),
                               m_name.as_mut_ptr());
                    /* Hack -- Track this monster race */
                    if (*m_ptr).ml != 0 {
                        monster_race_track((*m_ptr).r_idx as libc::c_int,
                                           (*m_ptr).ego as libc::c_int);
                    }
                    /* Hack -- Track this monster */
                    if (*m_ptr).ml != 0 {
                        health_track((*c_ptr).m_idx as libc::c_int);
                    }
                }
                /* Apply special damage XXX XXX XXX */
                tdam =
                    tot_dam_aux(q_ptr, tdam, m_ptr, &mut special) as
                        libc::c_int;
                tdam =
                    critical_shot((*q_ptr).weight,
                                  (*q_ptr).to_h as libc::c_int, tdam,
                                  if (*o_ptr).sval as libc::c_int ==
                                         1 as libc::c_int {
                                      58 as libc::c_int
                                  } else { 23 as libc::c_int }) as
                        libc::c_int;
                /* No negative damage */
                if tdam < 0 as libc::c_int { tdam = 0 as libc::c_int }
                /* Complex message */
                if wizard != 0 {
                    msg_format(b"You do %d (out of %d) damage.\x00" as
                                   *const u8 as *const libc::c_char, tdam,
                               (*m_ptr).hp);
                }
                /* Hit the monster, check for death */
                if !(mon_take_hit((*c_ptr).m_idx as libc::c_int, tdam,
                                  &mut fear, note_dies) != 0) {
                    /* No death */
                    /* Message */
                    message_pain((*c_ptr).m_idx as libc::c_int, tdam);
                    if special != 0 { attack_special(m_ptr, special, tdam); }
                    if !((*k_info.offset((*q_ptr).k_idx as isize)).tval as
                             libc::c_int == 71 as libc::c_int) {
                        let mut m_name_0: [libc::c_char; 80] = [0; 80];
                        monster_desc(m_name_0.as_mut_ptr(), m_ptr,
                                     0 as libc::c_int);
                        match is_friend(m_ptr) {
                            1 => {
                                msg_format(b"%^s gets angry!\x00" as *const u8
                                               as *const libc::c_char,
                                           m_name_0.as_mut_ptr());
                                change_side(m_ptr);
                            }
                            0 => {
                                msg_format(b"%^s gets angry!\x00" as *const u8
                                               as *const libc::c_char,
                                           m_name_0.as_mut_ptr());
                                (*m_ptr).status = -(1 as libc::c_int) as s16b
                            }
                            _ => { }
                        }
                    }
                    if fear as libc::c_int != 0 &&
                           (*m_ptr).ml as libc::c_int != 0 {
                        let mut m_name_1: [libc::c_char; 80] = [0; 80];
                        /* Anger friends */
                        /* Sound */
                        sound(3 as libc::c_int);
                        /* Get the monster name (or "it") */
                        monster_desc(m_name_1.as_mut_ptr(), m_ptr,
                                     0 as libc::c_int);
                        /* Message */
                        msg_format(b"%^s flees in terror!\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name_1.as_mut_ptr());
                    }
                }
            }
            break ;
        }
    }
    /* Take note */
    /* Chance of breakage (during attacks) */
    j =
        if hit_body as libc::c_int != 0 {
            breakage_chance(q_ptr)
        } else { 0 as libc::c_int };
    /* Potions smash open */
    if (*k_info.offset((*q_ptr).k_idx as isize)).tval as libc::c_int ==
           71 as libc::c_int {
        if hit_body as libc::c_int != 0 || hit_wall as libc::c_int != 0 ||
               (Rand_div(100 as libc::c_int) + 1 as libc::c_int) < j {
            /* Message */
            msg_format(b"The %s shatters!\x00" as *const u8 as
                           *const libc::c_char, o_name.as_mut_ptr());
            if potion_smash_effect(0 as libc::c_int, y, x,
                                   (*q_ptr).sval as libc::c_int) != 0 {
                if (*cave[y as usize].offset(x as isize)).m_idx != 0 {
                    let mut m_name_2: [libc::c_char; 80] = [0; 80];
                    monster_desc(m_name_2.as_mut_ptr(),
                                 &mut *m_list.offset((*(*cave.as_mut_ptr().offset(y
                                                                                      as
                                                                                      isize)).offset(x
                                                                                                         as
                                                                                                         isize)).m_idx
                                                         as isize),
                                 0 as libc::c_int);
                    match is_friend(&mut *m_list.offset((*(*cave.as_mut_ptr().offset(y
                                                                                         as
                                                                                         isize)).offset(x
                                                                                                            as
                                                                                                            isize)).m_idx
                                                            as isize)) {
                        1 => {
                            msg_format(b"%^s gets angry!\x00" as *const u8 as
                                           *const libc::c_char,
                                       m_name_2.as_mut_ptr());
                            change_side(&mut *m_list.offset((*(*cave.as_mut_ptr().offset(y
                                                                                             as
                                                                                             isize)).offset(x
                                                                                                                as
                                                                                                                isize)).m_idx
                                                                as isize));
                        }
                        0 => {
                            msg_format(b"%^s gets angry!\x00" as *const u8 as
                                           *const libc::c_char,
                                       m_name_2.as_mut_ptr());
                            (*m_list.offset((*cave[y as
                                                       usize].offset(x as
                                                                         isize)).m_idx
                                                as isize)).status =
                                -(1 as libc::c_int) as s16b
                        }
                        _ => { }
                    }
                }
            }
            return
        } else { j = 0 as libc::c_int }
    }
    /* Drop (or break) near that location */
    drop_near(q_ptr, j, y, x);
}
/*
 * Throw a boomerang object from the equipement(bow).
 *
 * Note: "unseen" monsters are very hard to hit.
 *
 * Should throwing a weapon do full damage?  Should it allow the magic
 * to hit bonus of the weapon to have an effect?  Should it ever cause
 * the item to be destroyed?  Should it do any damage at all?
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_boomerang() {
    let mut dir: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut tdam: libc::c_int = 0;
    let mut tdis: libc::c_int = 0;
    let mut mul: libc::c_int = 0;
    let mut div: libc::c_int = 0;
    let mut cur_dis: libc::c_int = 0;
    let mut visible: libc::c_int = 0;
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
    let mut hit_body: bool_ = 0 as libc::c_int as bool_;
    let mut missile_attr: byte_hack = 0;
    let mut missile_char: libc::c_char = 0;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut special: s32b = 0 as libc::c_int;
    let mut msec: libc::c_int =
        delay_factor as libc::c_int * delay_factor as libc::c_int *
            delay_factor as libc::c_int;
    /* Get the "bow" (if any) */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(27 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Get a direction (or cancel) */
    if get_aim_dir(&mut dir) == 0 { return }
    /* Get local object */
    q_ptr = &mut forge;
    /* Obtain a local object */
    object_copy(q_ptr, o_ptr);
    /* Single object */
    (*q_ptr).number = 1 as libc::c_int as byte_hack;
    /* Description */
    object_desc(o_name.as_mut_ptr(), q_ptr, 0 as libc::c_int,
                3 as libc::c_int);
    /* Find the color and symbol for the object for throwing */
    missile_attr =
        if (*q_ptr).tval as libc::c_int == 102 as libc::c_int {
            random_artifacts[(*q_ptr).sval as usize].attr as libc::c_int
        } else if (*k_info.offset((*q_ptr).k_idx as isize)).flavor as
                      libc::c_int != 0 {
            misc_to_attr[(*k_info.offset((*q_ptr).k_idx as isize)).flavor as
                             usize] as libc::c_int
        } else {
            (*k_info.offset((*q_ptr).k_idx as isize)).x_attr as libc::c_int
        } as byte_hack;
    missile_char =
        if (*k_info.offset((*q_ptr).k_idx as isize)).flavor as libc::c_int !=
               0 {
            misc_to_char[(*k_info.offset((*q_ptr).k_idx as isize)).flavor as
                             usize] as libc::c_int
        } else {
            (*k_info.offset((*q_ptr).k_idx as isize)).x_char as libc::c_int
        } as libc::c_char;
    /* Extract a "distance multiplier" */
	/* Changed for 'launcher' corruption */
    mul =
        10 as libc::c_int +
            2 as libc::c_int *
                ((*p_ptr).throw_mult as libc::c_int - 1 as libc::c_int);
    /* Enforce a minimum "weight" of one pound */
    div =
        if (*q_ptr).weight > 10 as libc::c_int {
            (*q_ptr).weight
        } else { 10 as libc::c_int };
    /* Hack -- Distance -- Reward strength, penalize weight */
    tdis =
        (*adj_str_blow.as_mut_ptr().offset((*p_ptr).stat_ind[0 as libc::c_int
                                                                 as usize] as
                                               isize) as libc::c_int +
             20 as libc::c_int) * mul / div;
    /* Max distance of 10-18 */
    if tdis > mul { tdis = mul }
    /* Hack -- Base damage from thrown object */
    tdam =
        damroll((*q_ptr).dd as s16b, (*q_ptr).ds as s16b) +
            (*q_ptr).to_d as libc::c_int;
    tdam *= (*p_ptr).throw_mult as libc::c_int;
    /* Chance of hitting */
    chance =
        (*p_ptr).skill_tht as libc::c_int +
            ((*p_ptr).to_h as libc::c_int +
                 (*p_ptr).to_h_ranged as libc::c_int) * 3 as libc::c_int;
    chance += get_skill(27 as libc::c_int) as libc::c_int;
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Start at the player */
    y = (*p_ptr).py as libc::c_int;
    x = (*p_ptr).px as libc::c_int;
    /* Predict the "target" location */
    tx =
        (*p_ptr).px as libc::c_int +
            99 as libc::c_int * ddx[dir as usize] as libc::c_int;
    ty =
        (*p_ptr).py as libc::c_int +
            99 as libc::c_int * ddy[dir as usize] as libc::c_int;
    /* Check for "target request" */
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int
    }
    /* Hack -- Handle stuff */
    handle_stuff();
    /* Travel until stopped */
    cur_dis = 0 as libc::c_int;
    while cur_dis <= tdis {
        /* Hack -- Stop at the target */
        if y == ty && x == tx { break ; }
        /* Calculate the new location (see "project()") */
        ny = y;
        nx = x;
        mmove2(&mut ny, &mut nx, (*p_ptr).py as libc::c_int,
               (*p_ptr).px as libc::c_int, ty, tx);
        /* Stopped by walls/doors */
        if !((*f_info.offset((*cave[ny as usize].offset(nx as isize)).feat as
                                 isize)).flags1 as libc::c_long &
                 0x10 as libc::c_long != 0 &&
                 (*cave[ny as usize].offset(nx as isize)).feat as libc::c_int
                     != 0xaf as libc::c_int) {
            break ;
        }
        /* Advance the distance */
        cur_dis += 1;
        /* Save the new location */
        x = nx;
        y = ny;
        /* The player can see the (on screen) missile */
        if y >= panel_row_min as libc::c_int &&
               y <= panel_row_max as libc::c_int &&
               x >= panel_col_min as libc::c_int &&
               x <= panel_col_max as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).info as libc::c_int &
                   0x10 as libc::c_int != 0 as libc::c_int {
            /* Draw, Hilite, Fresh, Pause, Erase */
            print_rel(missile_char, missile_attr, y, x);
            move_cursor_relative(y, x);
            Term_fresh();
            Term_xtra(13 as libc::c_int, msec);
            lite_spot(y, x);
            Term_fresh();
        } else {
            /* The player cannot see the missile */
            /* Pause anyway, for consistancy */
            Term_xtra(13 as libc::c_int, msec);
        }
        /* Monster here, Try to hit it */
        if !((*cave[y as usize].offset(x as isize)).m_idx != 0) { continue ; }
        let mut c_ptr: *mut cave_type =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
        let mut r_ptr: *mut monster_race =
            if !(*m_ptr).sr_ptr.is_null() {
                (*m_ptr).sr_ptr
            } else {
                race_info_idx((*m_ptr).r_idx as libc::c_int,
                              (*m_ptr).ego as libc::c_int)
            };
        /* Check the visibility */
        visible = (*m_ptr).ml as libc::c_int;
        /* Note the collision */
        hit_body = 1 as libc::c_int as bool_;
        /* Did we hit it (penalize range) */
        if test_hit_fire(chance - cur_dis, (*m_ptr).ac as libc::c_int,
                         (*m_ptr).ml as libc::c_int) != 0 {
            let mut fear: bool_ = 0 as libc::c_int as bool_;
            /* Assume a default death */
            let mut note_dies: cptr =
                b" dies.\x00" as *const u8 as *const libc::c_char;
            /* Some monsters get "destroyed" */
            if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 ||
                   (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
                   ||
                   (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0
                   ||
                   !strchr(b"Evg\x00" as *const u8 as *const libc::c_char,
                           (*r_ptr).d_char as libc::c_int).is_null() {
                /* Special note at death */
                note_dies =
                    b" is destroyed.\x00" as *const u8 as *const libc::c_char
            }
            /* Handle unseen monster */
            if visible == 0 {
                /* Invisible monster */
                msg_format(b"The %s finds a mark.\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr());
            } else {
                /* Handle visible monster */
                let mut m_name: [libc::c_char; 80] = [0; 80];
                /* Get "the monster" or "it" */
                monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
                /* Message */
                msg_format(b"The %s hits %s.\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr(),
                           m_name.as_mut_ptr());
                /* Hack -- Track this monster race */
                if (*m_ptr).ml != 0 {
                    monster_race_track((*m_ptr).r_idx as libc::c_int,
                                       (*m_ptr).ego as libc::c_int);
                }
                /* Hack -- Track this monster */
                if (*m_ptr).ml != 0 {
                    health_track((*c_ptr).m_idx as libc::c_int);
                }
            }
            /* Apply special damage XXX XXX XXX */
            tdam =
                tot_dam_aux(q_ptr, tdam, m_ptr, &mut special) as libc::c_int;
            tdam =
                critical_shot((*q_ptr).weight, (*q_ptr).to_h as libc::c_int,
                              tdam, 23 as libc::c_int) as libc::c_int;
            /* No negative damage */
            if tdam < 0 as libc::c_int { tdam = 0 as libc::c_int }
            /* Complex message */
            if wizard != 0 {
                msg_format(b"You do %d (out of %d) damage.\x00" as *const u8
                               as *const libc::c_char, tdam, (*m_ptr).hp);
            }
            /* Hit the monster, check for death */
            if !(mon_take_hit((*c_ptr).m_idx as libc::c_int, tdam, &mut fear,
                              note_dies) != 0) {
                /* No death */
                /* Message */
                message_pain((*c_ptr).m_idx as libc::c_int, tdam);
                if special != 0 { attack_special(m_ptr, special, tdam); }
                if !((*k_info.offset((*q_ptr).k_idx as isize)).tval as
                         libc::c_int == 71 as libc::c_int) {
                    let mut m_name_0: [libc::c_char; 80] = [0; 80];
                    monster_desc(m_name_0.as_mut_ptr(), m_ptr,
                                 0 as libc::c_int);
                    match is_friend(m_ptr) {
                        1 => {
                            msg_format(b"%^s gets angry!\x00" as *const u8 as
                                           *const libc::c_char,
                                       m_name_0.as_mut_ptr());
                            change_side(m_ptr);
                        }
                        0 => {
                            msg_format(b"%^s gets angry!\x00" as *const u8 as
                                           *const libc::c_char,
                                       m_name_0.as_mut_ptr());
                            (*m_ptr).status = -(1 as libc::c_int) as s16b
                        }
                        _ => { }
                    }
                }
                if fear as libc::c_int != 0 && (*m_ptr).ml as libc::c_int != 0
                   {
                    let mut m_name_1: [libc::c_char; 80] = [0; 80];
                    /* Anger friends */
                    /* Sound */
                    sound(3 as libc::c_int);
                    /* Get the monster name (or "it") */
                    monster_desc(m_name_1.as_mut_ptr(), m_ptr,
                                 0 as libc::c_int);
                    /* Message */
                    msg_format(b"%^s flees in terror!\x00" as *const u8 as
                                   *const libc::c_char,
                               m_name_1.as_mut_ptr());
                }
            }
            /* Take note */
            /* Chance of breakage (during attacks) */
            j =
                if hit_body as libc::c_int != 0 {
                    breakage_chance(o_ptr)
                } else { 0 as libc::c_int };
            /* Break the boomerang */
            if !((*o_ptr).art_name as libc::c_int != 0 ||
                     ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                          (if (*o_ptr).name1 as libc::c_int != 0 {
                               1 as libc::c_int
                           } else { 0 as libc::c_int }) != 0 ||
                          (if (*o_ptr).art_name as libc::c_int != 0 {
                               1 as libc::c_int
                           } else { 0 as libc::c_int }) != 0 ||
                          (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3
                                  as libc::c_long & 0x8000 as libc::c_long !=
                                  0 {
                               1 as libc::c_int
                           } else { 0 as libc::c_int }) != 0)) &&
                   Rand_div(100 as libc::c_int) < j {
                msg_print(format(b"Your %s is destroyed.\x00" as *const u8 as
                                     *const libc::c_char, o_name.as_mut_ptr())
                              as cptr);
                inc_stack_size_ex(27 as libc::c_int, -(1 as libc::c_int),
                                  OPTIMIZE, NO_DESCRIBE);
            }
        }
        break ;
    }
    /* Travel back to the player */
    cur_dis = 0 as libc::c_int;
    while cur_dis <= tdis {
        /* Hack -- Stop at the target */
        if y == (*p_ptr).py as libc::c_int && x == (*p_ptr).px as libc::c_int
           {
            break ;
        }
        /* Calculate the new location (see "project()") */
        ny = y;
        nx = x;
        mmove2(&mut ny, &mut nx, ty, tx, (*p_ptr).py as libc::c_int,
               (*p_ptr).px as libc::c_int);
        /* Advance the distance */
        cur_dis += 1;
        /* Save the new location */
        x = nx;
        y = ny;
        /* The player can see the (on screen) missile */
        if y >= panel_row_min as libc::c_int &&
               y <= panel_row_max as libc::c_int &&
               x >= panel_col_min as libc::c_int &&
               x <= panel_col_max as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).info as libc::c_int &
                   0x10 as libc::c_int != 0 as libc::c_int {
            /* Draw, Hilite, Fresh, Pause, Erase */
            print_rel(missile_char, missile_attr, y, x);
            move_cursor_relative(y, x);
            Term_fresh();
            Term_xtra(13 as libc::c_int, msec);
            lite_spot(y, x);
            Term_fresh();
        } else {
            /* The player cannot see the missile */
            /* Pause anyway, for consistancy */
            Term_xtra(13 as libc::c_int, msec);
        }
    };
}
/*
 * Try to ``walk'' using phase door.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_unwalk() {
    let mut dir: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut feat: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut more: bool_ = 0 as libc::c_int as bool_;
    if get_rep_dir(&mut dir) == 0 { return }
    y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
    x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    feat = (*c_ptr).feat as libc::c_int;
    /* Must have knowledge to know feature XXX XXX */
    if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int == 0 {
        feat = 0 as libc::c_int
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    energy_use *=
        if (*p_ptr).wild_mode as libc::c_int != 0 {
            (5 as libc::c_int * (66 as libc::c_int + 198 as libc::c_int)) /
                2 as libc::c_int
        } else { 1 as libc::c_int };
    /* Allow repeated command */
    if command_arg != 0 {
        /* Set repeat count */
        command_rep = (command_arg as libc::c_int - 1 as libc::c_int) as s16b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Cancel the arg */
        command_arg = 0 as libc::c_int as s16b
    }
    /* Attack monsters */
    if (*c_ptr).m_idx as libc::c_int > 0 as libc::c_int {
        /* Attack */
        py_attack(y, x, -(1 as libc::c_int));
    } else if dun_level == 0 && (*p_ptr).wild_mode == 0 &&
                  (x == 0 as libc::c_int ||
                       x == cur_wid as libc::c_int - 1 as libc::c_int ||
                       y == 0 as libc::c_int ||
                       y == cur_hgt as libc::c_int - 1 as libc::c_int) {
        /* Exit the area */
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
    } else if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                    libc::c_long & 0x10 as libc::c_long != 0 &&
                    (*c_ptr).feat as libc::c_int != 0xaf as libc::c_int) {
        teleport_player(10 as libc::c_int);
    } else if feat >= 0x8 as libc::c_int && feat <= 0xb as libc::c_int ||
                  feat >= 0x6 as libc::c_int && feat <= 0x7 as libc::c_int {
        move_player(dir, always_pickup as libc::c_int,
                    1 as libc::c_int as bool_);
        more = 0 as libc::c_int as bool_
    } else if (*p_ptr).wild_mode != 0 {
        /* Hack -- Ignore weird terrain types. */
        /* Enter quests */
        /* Hack -- Ignore wilderness mofe. */
        /* Chance to not blink right */
        if Rand_div(100 as libc::c_int) < 15 as libc::c_int {
            loop  {
                dir =
                    1 as libc::c_int +
                        Rand_div(1 as libc::c_int + 9 as libc::c_int -
                                     1 as libc::c_int);
                if !(dir == 5 as libc::c_int) { break ; }
            }
        }
        move_player(dir, always_pickup as libc::c_int,
                    1 as libc::c_int as bool_);
    } else {
        /* Walking semantics */
        teleport_player_directed(10 as libc::c_int, dir);
    }
    /* Cancel repetition unless we can continue */
    if more == 0 { disturb(0 as libc::c_int, 0 as libc::c_int); };
}
unsafe extern "C" fn tport_vertically(mut how: bool_) -> bool_ {
    /* arena or quest -KMW- */
    if (*p_ptr).inside_arena as libc::c_int != 0 ||
           (*p_ptr).inside_quest as libc::c_int != 0 {
        msg_print(b"There is no effect.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    if dungeon_flags2 as libc::c_long & 0x4000 as libc::c_long != 0 {
        msg_print(b"Some powerful force prevents you from teleporting.\x00" as
                      *const u8 as *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Go down */
    if how != 0 {
        if dun_level as libc::c_int >=
               (*d_info.offset(dungeon_type as isize)).maxdepth as libc::c_int
           {
            msg_print(b"The floor is impermeable.\x00" as *const u8 as
                          *const libc::c_char);
            return 0 as libc::c_int as bool_
        }
        msg_print(b"You sink through the floor.\x00" as *const u8 as
                      *const libc::c_char);
        dun_level += 1;
        (*p_ptr).leaving = 1 as libc::c_int as bool_
    } else {
        if (dun_level as libc::c_int) <
               (*d_info.offset(dungeon_type as isize)).mindepth as libc::c_int
           {
            msg_print(b"There is nothing above you but air.\x00" as *const u8
                          as *const libc::c_char);
            return 0 as libc::c_int as bool_
        }
        msg_print(b"You rise through the ceiling.\x00" as *const u8 as
                      *const libc::c_char);
        dun_level -= 1;
        (*p_ptr).leaving = 1 as libc::c_int as bool_
    }
    return 1 as libc::c_int as bool_;
}
/* File: cmd2.c */
/* Purpose: Movement commands (part 2) */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Do a special ``movement'' action. Meant to be used for ``immovable''
 * characters.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_immovable_special() {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut ij: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut foo: libc::c_int = (*p_ptr).immov_cntr as libc::c_int;
    let mut lose_sp: libc::c_int = 0 as libc::c_int;
    let mut lose_hp: libc::c_int = 0 as libc::c_int;
    let mut did_act: bool_ = 0 as libc::c_int as bool_;
    let mut did_load: bool_ = 0 as libc::c_int as bool_;
    if foo > 1 as libc::c_int {
        if (*p_ptr).csp as libc::c_int > foo / 2 as libc::c_int {
            msg_format(b"This will drain %d mana points!\x00" as *const u8 as
                           *const libc::c_char, foo / 2 as libc::c_int);
            if get_check(b"Proceed? \x00" as *const u8 as *const libc::c_char)
                   == 0 {
                return
            }
            lose_sp = foo / 2 as libc::c_int
        } else if (*p_ptr).chp as libc::c_int > foo / 2 as libc::c_int {
            msg_format(b"Warning: This will drain %d hit points!\x00" as
                           *const u8 as *const libc::c_char,
                       foo / 2 as libc::c_int);
            if get_check(b"Proceed? \x00" as *const u8 as *const libc::c_char)
                   == 0 {
                return
            }
            lose_hp = foo / 2 as libc::c_int
        } else {
            msg_print(b"You can\'t use your powers yet.\x00" as *const u8 as
                          *const libc::c_char);
            return
        }
    }
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    loop 
         /* Interact until done */
         /* Clear screen */
         {
        Term_clear();
        /* Ask for a choice */
        prt(b"Do what special action:\x00" as *const u8 as
                *const libc::c_char, 2 as libc::c_int, 0 as libc::c_int);
        /* Give some choices */
        prt(b"(a) Teleport to a specific place.\x00" as *const u8 as
                *const libc::c_char, 4 as libc::c_int, 5 as libc::c_int);
        prt(b"(b) Fetch an item.\x00" as *const u8 as *const libc::c_char,
            5 as libc::c_int, 5 as libc::c_int);
        prt(b"(c) Go up 50\'\x00" as *const u8 as *const libc::c_char,
            6 as libc::c_int, 5 as libc::c_int);
        prt(b"(d) Go down 50\'\x00" as *const u8 as *const libc::c_char,
            7 as libc::c_int, 5 as libc::c_int);
        /* Prompt */
        prt(b"Command: \x00" as *const u8 as *const libc::c_char,
            9 as libc::c_int, 0 as libc::c_int);
        /* Prompt */
        i = inkey() as libc::c_int;
        /* Done */
        if i == '\u{1b}' as i32 { break ; }
        /* Tele-to */
        if i == 'a' as i32 {
            Term_load();
            character_icky = 0 as libc::c_int as bool_;
            did_load = 1 as libc::c_int as bool_;
            if tgt_pt(&mut ii, &mut ij) == 0 { break ; }
            /* Teleport to the target */
            teleport_player_to(ij, ii);
            did_act = 1 as libc::c_int as bool_;
            break ;
        } else if i == 'b' as i32 {
            Term_load();
            character_icky = 0 as libc::c_int as bool_;
            did_load = 1 as libc::c_int as bool_;
            if get_aim_dir(&mut dir) == 0 { return }
            fetch(dir, (*p_ptr).lev as libc::c_int * 15 as libc::c_int,
                  0 as libc::c_int as bool_);
            py_pickup_floor(always_pickup as libc::c_int);
            did_act = 1 as libc::c_int as bool_;
            break ;
        } else if i == 'c' as i32 {
            Term_load();
            character_icky = 0 as libc::c_int as bool_;
            did_load = 1 as libc::c_int as bool_;
            if tport_vertically(0 as libc::c_int as bool_) == 0 { return }
            did_act = 1 as libc::c_int as bool_;
            break ;
        } else if i == 'd' as i32 {
            Term_load();
            character_icky = 0 as libc::c_int as bool_;
            did_load = 1 as libc::c_int as bool_;
            if tport_vertically(1 as libc::c_int as bool_) == 0 { return }
            did_act = 1 as libc::c_int as bool_;
            break ;
        } else {
            /* Fetch item */
            /* Move up */
            /* Move down */
            /* Unknown option */
            bell();
        }
    }
    /* Check if screen was restored before */
    if did_load == 0 {
        /* Restore the screen */
        Term_load();
        /* Leave "icky" mode */
        character_icky = 0 as libc::c_int as bool_
    }
    /* Apply stat losses if something was done */
    if did_act != 0 {
        (*p_ptr).immov_cntr =
            ((*p_ptr).immov_cntr as libc::c_int +
                 (101 as libc::c_int -
                      (*p_ptr).lev as libc::c_int * 2 as libc::c_int)) as
                s16b;
        if lose_sp != 0 {
            (*p_ptr).csp = ((*p_ptr).csp as libc::c_int - lose_sp) as s16b;
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as
                    u32b
        }
        if lose_hp != 0 {
            (*p_ptr).chp = ((*p_ptr).chp as libc::c_int - lose_hp) as s16b;
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as
                    u32b
        }
        energy_use = 100 as libc::c_int
    };
}
/* Can we sacrifice it ? */
unsafe extern "C" fn item_tester_hook_sacrifiable(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int {
        /* Corpses are */
        if (*o_ptr).tval as libc::c_int == 9 as libc::c_int &&
               (*o_ptr).sval as libc::c_int == 1 as libc::c_int {
            return 1 as libc::c_int as bool_
        }
        /* Books without any udun spells */
        if (*o_ptr).tval as libc::c_int == 111 as libc::c_int &&
               exec_lua(format(b"return udun_in_book(%d, %d)\x00" as *const u8
                                   as *const libc::c_char,
                               (*o_ptr).sval as libc::c_int, (*o_ptr).pval))
                   == 0 as libc::c_int {
            return 1 as libc::c_int as bool_
        }
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
/*
 * Handle sacrifices.
 * Grace is increased by value of sacrifice.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_sacrifice() {
    let mut on_what: byte_hack =
        (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat;
    /* Check valididty */
    if (on_what as libc::c_int) < 0xa1 as libc::c_int ||
           on_what as libc::c_int > 0xab as libc::c_int {
        show_god_info(0 as libc::c_int as bool_);
        return
    } else {
        let mut agod: libc::c_int =
            on_what as libc::c_int - 0xa1 as libc::c_int + 1 as libc::c_int;
        /* Not worshipping a god ? ahhhh! */
        if (*p_ptr).pgod as libc::c_int == 0 as libc::c_int {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < 10 as libc::c_int {
                if !(*deity_info.offset(agod as
                                            isize)).desc[i as
                                                             usize].as_mut_ptr().is_null()
                   {
                    msg_print((*deity_info.offset(agod as
                                                      isize)).desc[i as
                                                                       usize].as_mut_ptr()
                                  as cptr);
                }
                i += 1
            }
            if get_check(format(b"Do you want to worship %s? \x00" as
                                    *const u8 as *const libc::c_char,
                                (*deity_info.offset(agod as isize)).name) as
                             cptr) != 0 {
                follow_god(agod, 0 as libc::c_int as bool_);
                (*p_ptr).grace = -(200 as libc::c_int);
                inc_piety((*p_ptr).pgod as libc::c_int, 0 as libc::c_int);
            }
        } else if (*p_ptr).pgod as libc::c_int == agod {
            if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int {
                /* One can sacrifice some HP for piety or damage */
                if (*p_ptr).mhp as libc::c_int > 10 as libc::c_int &&
                       (*p_ptr).chp as libc::c_int > 10 as libc::c_int &&
                       get_check(b"Do you want to sacrifice a part of yourself? \x00"
                                     as *const u8 as *const libc::c_char) as
                           libc::c_int != 0 {
                    /* 10 HP = 300 * wis piety */
                    if get_check(b"Do you want to sacrifice for more piety instead of damage? \x00"
                                     as *const u8 as *const libc::c_char) != 0
                       {
                        let mut x: libc::c_int =
                            wisdom_scale(6 as libc::c_int);
                        if x < 1 as libc::c_int { x = 1 as libc::c_int }
                        (*p_ptr).hp_mod =
                            ((*p_ptr).hp_mod as libc::c_int -
                                 10 as libc::c_int) as s16b;
                        take_hit(10 as libc::c_int,
                                 b"self sacrifice to Melkor\x00" as *const u8
                                     as *const libc::c_char);
                        msg_print(b"Your life slips away, and Melkor seems happier.\x00"
                                      as *const u8 as *const libc::c_char);
                        inc_piety(4 as libc::c_int, x * 300 as libc::c_int);
                        (*p_ptr).update =
                            ((*p_ptr).update as libc::c_long |
                                 0x10 as libc::c_long) as u32b
                    } else {
                        /* 10 HP = +wis damage */
                        take_hit(10 as libc::c_int,
                                 b"self sacrifice to Melkor\x00" as *const u8
                                     as *const libc::c_char);
                        msg_print(b"Your life slips away, and your arms grow stronger.\x00"
                                      as *const u8 as *const libc::c_char);
                        (*p_ptr).melkor_sacrifice += 1;
                        (*p_ptr).update =
                            ((*p_ptr).update as libc::c_long |
                                 (0x1 as libc::c_long | 0x10 as libc::c_long))
                                as u32b
                    }
                } else {
                    let mut item: libc::c_int = 0;
                    let mut o_ptr: *mut object_type = 0 as *mut object_type;
                    /* Restrict choices to food */
                    item_tester_hook =
                        Some(item_tester_hook_sacrifiable as
                                 unsafe extern "C" fn(_: *mut object_type)
                                     -> bool_);
                    /* Get an item */
                    if get_item(&mut item,
                                b"Sacrifice which item? \x00" as *const u8 as
                                    *const libc::c_char,
                                b"You have nothing to sacrifice.\x00" as
                                    *const u8 as *const libc::c_char,
                                0x2 as libc::c_int) == 0 {
                        return
                    }
                    o_ptr = get_object(item);
                    /* Piety for corpses is based on monster level */
                    if (*o_ptr).tval as libc::c_int == 9 as libc::c_int {
                        inc_piety(4 as libc::c_int,
                                  2 as libc::c_int *
                                      (*r_info.offset((*o_ptr).pval2 as
                                                          isize)).level as
                                          libc::c_int);
                    }
                    /* In books it depends of the spell levels*/
                    if (*o_ptr).tval as libc::c_int == 111 as libc::c_int {
                        let mut x_0: libc::c_int =
                            exec_lua(format(b"return levels_in_book(%d, %d)\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            (*o_ptr).sval as libc::c_int,
                                            (*o_ptr).pval));
                        inc_piety(4 as libc::c_int, 2 as libc::c_int * x_0);
                    }
                    /* Remove the item */
                    inc_stack_size(item, -(1 as libc::c_int));
                }
            } else {
                process_hooks(64 as libc::c_int,
                              b"()\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char,
                              b"\x00" as *const u8 as *const libc::c_char);
            }
        }
    };
}
/*
 * scan_monst --
 *
 * Return a list of o_list[] indexes of items of the given monster
 */
#[no_mangle]
pub unsafe extern "C" fn scan_monst(mut items: *mut libc::c_int,
                                    mut item_num: *mut libc::c_int,
                                    mut m_idx: libc::c_int) -> bool_ {
    let mut this_o_idx: libc::c_int = 0;
    let mut next_o_idx: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    *item_num = 0 as libc::c_int;
    /* Scan all objects in the grid */
    this_o_idx = (*m_list.offset(m_idx as isize)).hold_o_idx as libc::c_int;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx as libc::c_int;
        /* Accept this item */
        let fresh1 = num;
        num = num + 1;
        *items.offset(fresh1 as isize) = this_o_idx;
        /* XXX Hack -- Enforce limit */
        if num == 23 as libc::c_int { break ; }
        this_o_idx = next_o_idx
    }
    /* Number of items */
    *item_num = num;
    /* Result */
    return (num != 0 as libc::c_int) as libc::c_int as bool_;
}
/*
 * Display a list of the items that the given monster carries.
 */
#[no_mangle]
pub unsafe extern "C" fn show_monster_inven(mut m_idx: libc::c_int,
                                            mut monst_list: *mut libc::c_int)
 -> byte_hack {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut lim: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut tmp_val: [libc::c_char; 80] = [0; 80];
    let mut out_index: [libc::c_int; 23] = [0; 23];
    let mut out_color: [byte_hack; 23] = [0; 23];
    let mut out_desc: [[libc::c_char; 80]; 23] = [[0; 80]; 23];
    let mut monst_num: libc::c_int = 0;
    /* Default length */
    len = 79 as libc::c_int - 50 as libc::c_int;
    /* Maximum space allowed for descriptions */
    lim = 79 as libc::c_int - 3 as libc::c_int;
    /* Require space for weight (if needed) */
    if show_weights != 0 { lim -= 9 as libc::c_int }
    /* Scan for objects on the monster */
    scan_monst(monst_list, &mut monst_num, m_idx);
    /* Display the p_ptr->inventory */
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < monst_num {
        o_ptr =
            &mut *o_list.offset(*monst_list.offset(i as isize) as isize) as
                *mut object_type;
        /* Describe the object */
        object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                    3 as libc::c_int);
        /* Hack -- enforce max length */
        o_name[lim as usize] = '\u{0}' as i32 as libc::c_char;
        /* Save the index */
        out_index[k as usize] = i;
        /* Acquire p_ptr->inventory color */
        out_color[k as usize] =
            tval_to_attr[((*o_ptr).tval as libc::c_int & 0x7f as libc::c_int)
                             as usize];
        /* Save the object description */
        strcpy(out_desc[k as usize].as_mut_ptr(), o_name.as_mut_ptr());
        /* Find the predicted "line length" */
        l =
            strlen(out_desc[k as
                                usize].as_mut_ptr()).wrapping_add(5 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong)
                as libc::c_int;
        /* Be sure to account for the weight */
        if show_weights != 0 { l += 9 as libc::c_int }
        /* Maintain the maximum length */
        if l > len { len = l }
        /* Advance to next "line" */
        k += 1;
        i += 1
    }
    /* Find the column to start in */
    col =
        if len > 76 as libc::c_int {
            0 as libc::c_int
        } else { (79 as libc::c_int) - len };
    /* Output each entry */
    j = 0 as libc::c_int;
    while j < k {
        /* Get the index */
        i = *monst_list.offset(out_index[j as usize] as isize);
        /* Get the item */
        o_ptr = &mut *o_list.offset(i as isize) as *mut object_type;
        /* Clear the line */
        prt(b"\x00" as *const u8 as *const libc::c_char, j + 1 as libc::c_int,
            if col != 0 { (col) - 2 as libc::c_int } else { col });
        /* Prepare an index --(-- */
        strnfmt(tmp_val.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%c)\x00" as *const u8 as *const libc::c_char,
                index_to_label(j) as libc::c_int);
        /* Clear the line with the (possibly indented) index */
        put_str(tmp_val.as_mut_ptr() as cptr, j + 1 as libc::c_int, col);
        /* Display the entry itself */
        c_put_str(out_color[j as usize],
                  out_desc[j as usize].as_mut_ptr() as cptr,
                  j + 1 as libc::c_int, col + 3 as libc::c_int);
        /* Display the weight if needed */
        if show_weights != 0 {
            let mut wgt: libc::c_int =
                (*o_ptr).weight * (*o_ptr).number as libc::c_int;
            strnfmt(tmp_val.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"%3d.%1d lb\x00" as *const u8 as *const libc::c_char,
                    wgt / 10 as libc::c_int, wgt % 10 as libc::c_int);
            put_str(tmp_val.as_mut_ptr() as cptr, j + 1 as libc::c_int,
                    71 as libc::c_int);
        }
        j += 1
    }
    /* Make a "shadow" below the list (only if needed) */
    if j != 0 && j < 23 as libc::c_int {
        prt(b"\x00" as *const u8 as *const libc::c_char, j + 1 as libc::c_int,
            if col != 0 { (col) - 2 as libc::c_int } else { col });
    }
    return monst_num as byte_hack;
}
/*
 * Steal an object from a monster
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_steal() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut dir: libc::c_int = 0 as libc::c_int;
    let mut item: libc::c_int = -(1 as libc::c_int);
    let mut k: libc::c_int = -(1 as libc::c_int);
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
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
    let mut num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    let mut monst_list: [libc::c_int; 23] = [0; 23];
    /* Only works on adjacent monsters */
    if get_rep_dir(&mut dir) == 0 { return }
    y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
    x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    if (*c_ptr).m_idx == 0 {
        msg_print(b"There is no monster there!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    m_ptr = &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
    /* There were no non-gold items */
    if (*m_ptr).hold_o_idx == 0 {
        msg_print(b"That monster has no objects!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* The monster is immune */
    if (*r_info.offset((*m_ptr).r_idx as isize)).flags7 &
           0x40000 as libc::c_int as libc::c_uint != 0 {
        msg_print(b"The monster is guarding the treasures.\x00" as *const u8
                      as *const libc::c_char);
        return
    }
    screen_save();
    num =
        show_monster_inven((*c_ptr).m_idx as libc::c_int,
                           monst_list.as_mut_ptr());
    /* Repeat until done */
    while done == 0 {
        let mut tmp_val: [libc::c_char; 80] = [0; 80];
        let mut which: libc::c_char = ' ' as i32 as libc::c_char;
        /* Build the prompt */
        strnfmt(tmp_val.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"Choose an item to steal (a-%c) or ESC:\x00" as *const u8 as
                    *const libc::c_char,
                'a' as i32 - 1 as libc::c_int + num as libc::c_int);
        /* Show the prompt */
        prt(tmp_val.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
        /* Get a key */
        which = inkey();
        /* Parse it */
        match which as libc::c_int {
            27 => { done = 1 as libc::c_int as bool_ }
            _ => {
                let mut ver: libc::c_int = 0;
                /* Extract "query" setting */
                ver =
                    *(*__ctype_b_loc()).offset(which as libc::c_int as isize)
                        as libc::c_int &
                        _ISupper as libc::c_int as libc::c_ushort as
                            libc::c_int;
                which = tolower(which as libc::c_int) as libc::c_char;
                k =
                    if *(*__ctype_b_loc()).offset(which as libc::c_int as
                                                      isize) as libc::c_int &
                           _ISlower as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        (which as libc::c_int) - 'a' as i32
                    } else { -(1 as libc::c_int) };
                if k < 0 as libc::c_int || k >= num as libc::c_int {
                    bell();
                } else if ver != 0 &&
                              verify(b"Try\x00" as *const u8 as
                                         *const libc::c_char,
                                     0 as libc::c_int -
                                         monst_list[k as usize]) == 0 {
                    done = 1 as libc::c_int as bool_
                } else {
                    /* Verify the item */
                    /* Accept that choice */
                    item = monst_list[k as usize];
                    done = 1 as libc::c_int as bool_
                }
            }
        }
    }
    if item != -(1 as libc::c_int) {
        let mut chance: libc::c_int = 0;
        chance =
            40 as libc::c_int -
                (*p_ptr).stat_ind[3 as libc::c_int as usize] as libc::c_int;
        chance +=
            (*o_list.offset(item as isize)).weight /
                (get_skill_scale(40 as libc::c_int, 19 as libc::c_int as u32b)
                     as libc::c_int + 1 as libc::c_int);
        chance +=
            get_skill_scale(40 as libc::c_int, 29 as libc::c_int as u32b) as
                libc::c_int + 1 as libc::c_int;
        chance -=
            if (*m_ptr).csleep as libc::c_int != 0 {
                10 as libc::c_int
            } else { 0 as libc::c_int };
        chance += (*m_ptr).level as libc::c_int;
        /* Failure check */
        if Rand_div(chance) >
               1 as libc::c_int +
                   get_skill_scale(40 as libc::c_int,
                                   25 as libc::c_int as u32b) as libc::c_int {
            /* Take a turn */
            energy_use = 100 as libc::c_int;
            /* Wake up */
            (*m_ptr).csleep = 0 as libc::c_int as s16b;
            /* Speed up because monsters are ANGRY when you try to thief them */
            (*m_ptr).mspeed =
                ((*m_ptr).mspeed as libc::c_int + 5 as libc::c_int) as
                    byte_hack;
            screen_load();
            msg_print(b"Oops! The monster is now really *ANGRY*!\x00" as
                          *const u8 as *const libc::c_char);
            return
        }
        /* Reconnect the objects list */
        if num as libc::c_int == 1 as libc::c_int {
            (*m_ptr).hold_o_idx = 0 as libc::c_int as s16b
        } else {
            if k > 0 as libc::c_int {
                (*o_list.offset(monst_list[(k - 1 as libc::c_int) as usize] as
                                    isize)).next_o_idx =
                    monst_list[(k + 1 as libc::c_int) as usize] as s16b
            }
            if k + 1 as libc::c_int >= num as libc::c_int {
                (*o_list.offset(monst_list[(k - 1 as libc::c_int) as usize] as
                                    isize)).next_o_idx =
                    0 as libc::c_int as s16b
            }
            if k == 0 as libc::c_int {
                (*m_ptr).hold_o_idx =
                    monst_list[(k + 1 as libc::c_int) as usize] as s16b
            }
        }
        /* Rogues gain some xp */
        if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                (*spp_ptr).flags1) as libc::c_long & 0x8000000 as libc::c_long
               != 0 {
            let mut max_point: s32b = 0;
            /* Max XP gained from stealing */
            max_point =
                (*o_list.offset(item as isize)).weight / 2 as libc::c_int +
                    (*m_ptr).level as libc::c_int * 10 as libc::c_int;
            /* Randomise it a bit, with half a max guaranteed */
            gain_exp(max_point / 2 as libc::c_int +
                         (Rand_div(max_point) + 1 as libc::c_int) /
                             2 as libc::c_int);
            /* Allow escape */
            if get_check(b"Phase door?\x00" as *const u8 as
                             *const libc::c_char) != 0 {
                teleport_player(10 as libc::c_int);
            }
        }
        /* Get the item */
        o_ptr = &mut forge;
        /* Special handling for gold */
        if (*o_list.offset(item as isize)).tval as libc::c_int ==
               100 as libc::c_int {
            /* Collect the gold */
            (*p_ptr).au += (*o_list.offset(item as isize)).pval;
            /* Redraw gold */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x100 as libc::c_long) as
                    u32b;
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as
                    u32b
        } else {
            object_copy(o_ptr, &mut *o_list.offset(item as isize));
            inven_carry(o_ptr, 0 as libc::c_int as bool_);
        }
        /* Delete it */
        (*o_list.offset(item as isize)).k_idx = 0 as libc::c_int as s16b
    }
    screen_load();
    /* Take a turn */
    energy_use = 100 as libc::c_int;
}
/*
 * Give an item to a monster
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_give() {
    let mut dir: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut item: libc::c_int = 0;
    /* Get a "repeated" direction */
    if get_rep_dir(&mut dir) == 0 { return }
    /* Get requested location */
    y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
    x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
    /* Get requested grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* No monster in the way */
    if (*c_ptr).m_idx as libc::c_int == 0 as libc::c_int {
        msg_print(b"There is no monster there.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Get an item */
    q =
        b"What item do you want to offer? \x00" as *const u8 as
            *const libc::c_char;
    s = b"You have nothing to offer.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int) == 0 { return }
    /* Process hooks if there are any */
    if process_hooks(11 as libc::c_int,
                     b"(d,d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, (*c_ptr).m_idx as libc::c_int,
                     item) == 0 {
        msg_print(b"The monster does not want your item.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Take a turn, even if the offer is declined */
    energy_use = 100 as libc::c_int;
}
/*
 * Chat with a monster
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_chat() {
    let mut dir: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Get a "repeated" direction */
    if get_rep_dir(&mut dir) == 0 { return }
    /* Get requested location */
    y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
    x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
    /* Get requested grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* No monster in the way */
    if (*c_ptr).m_idx as libc::c_int == 0 as libc::c_int {
        msg_print(b"There is no monster there.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Process hook if there are any */
    if process_hooks(32 as libc::c_int,
                     b"(d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, (*c_ptr).m_idx as libc::c_int) ==
           0 {
        msg_print(b"The monster does not want to chat.\x00" as *const u8 as
                      *const libc::c_char);
    };
    /* No energy spent */
}
