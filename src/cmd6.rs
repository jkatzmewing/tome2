use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut ddd: [s16b; 9];
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut activation_info: [activation; 51];
    #[no_mangle]
    static mut inscription_info: [inscription_info_type; 8];
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut command_wrk: s16b;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut flush_failure: bool_;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
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
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut e_info: *mut ego_item_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut d_name: *mut libc::c_char;
    #[no_mangle]
    static mut ANGBAND_DIR_FILE: cptr;
    #[no_mangle]
    static mut item_tester_tval: byte_hack;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut fates: [fate; 200];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
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
    fn maxroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
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
    fn no_lite() -> bool_;
    #[no_mangle]
    fn note_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn map_area();
    #[no_mangle]
    fn wiz_lite();
    #[no_mangle]
    fn wiz_lite_extra();
    #[no_mangle]
    fn wiz_dark();
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn py_attack(y: libc::c_int, x: libc::c_int, max_blow: libc::c_int);
    #[no_mangle]
    fn do_cmd_leave_body(drop_body: bool_) -> bool_;
    #[no_mangle]
    fn do_spin();
    #[no_mangle]
    fn do_cmd_rerate();
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn inc_stack_size(item: libc::c_int, delta: libc::c_int);
    #[no_mangle]
    fn set_food(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn resolve_mimic_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn gain_exp(amount: s32b);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_tried(o_ptr: *mut object_type);
    #[no_mangle]
    fn set_poisoned(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn summon_specific_friendly(y1: libc::c_int, x1: libc::c_int,
                                lev: libc::c_int, type_0: libc::c_int,
                                Group_ok: bool_) -> bool_;
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_stun(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_afraid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_fire(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_protevil(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_pois(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_cold(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_elec(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_acid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn apply_disenchant(mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn lose_exp(amount: s32b);
    #[no_mangle]
    fn set_image(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn cold_dam(dam: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn fire_dam(dam: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn elec_dam(dam: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn acid_dam(dam: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn take_sanity_hit(damage: libc::c_int, hit_from: cptr);
    #[no_mangle]
    fn do_dec_stat(stat: libc::c_int, mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_blind(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_cut(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn inven_carry(o_ptr: *mut object_type, final_0: bool_) -> s16b;
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn hp_player(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    fn do_res_stat(stat: libc::c_int, full: bool_) -> bool_;
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn item_tester_okay(o_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    static mut get_item_extra_hook:
           Option<unsafe extern "C" fn(_: *mut libc::c_int) -> bool_>;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn heal_insanity(val: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_mimic(v: libc::c_int, p: libc::c_int, level: libc::c_int) -> bool_;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn luck(min: libc::c_int, max: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn set_tim_invis(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_invis(v: libc::c_int, p: libc::c_int) -> bool_;
    #[no_mangle]
    fn gain_random_corruption(choose_mut: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_invuln(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn self_knowledge(fff: *mut FILE);
    #[no_mangle]
    fn identify_pack();
    #[no_mangle]
    fn detect_objects_normal(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_objects_gold(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_treasure(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_stairs(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_doors(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_traps(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_inc_stat(stat: libc::c_int) -> bool_;
    #[no_mangle]
    fn restore_level() -> bool_;
    #[no_mangle]
    fn set_shero(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_hero(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_fast(v: libc::c_int, p: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_infra(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn dec_stat(stat: libc::c_int, amount: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn teleport_player(dis: libc::c_int);
    #[no_mangle]
    fn lose_all_info() -> bool_;
    #[no_mangle]
    fn set_slow(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn screen_load();
    #[no_mangle]
    fn show_file(name: cptr, what: cptr, line: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn screen_save();
    #[no_mangle]
    fn get_line(fname: *mut libc::c_char, fdir: cptr,
                linbuf: *mut libc::c_char, line: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn reveal_wilderness_around_player(y: libc::c_int, x: libc::c_int,
                                       h: libc::c_int, w: libc::c_int);
    #[no_mangle]
    fn artifact_scroll() -> bool_;
    #[no_mangle]
    fn fire_ball(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int,
                 rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn acquirement(y1: libc::c_int, x1: libc::c_int, num: libc::c_int,
                   great: bool_, known: bool_);
    #[no_mangle]
    fn mass_genocide(player_cast: bool_) -> bool_;
    #[no_mangle]
    fn genocide(player_cast: bool_) -> bool_;
    #[no_mangle]
    fn dispel_undead(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn destroy_area(y1: libc::c_int, x1: libc::c_int, r: libc::c_int,
                    full: bool_, bypass: bool_);
    #[no_mangle]
    fn destroy_doors_touch() -> bool_;
    #[no_mangle]
    fn warding_glyph();
    #[no_mangle]
    fn set_blessed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_monsters_invis(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn lite_area(dam: libc::c_int, rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn recharge(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn enchant_spell(num_hit: libc::c_int, num_dam: libc::c_int,
                     num_ac: libc::c_int, num_pval: libc::c_int) -> bool_;
    #[no_mangle]
    fn remove_all_curse() -> bool_;
    #[no_mangle]
    fn remove_curse() -> bool_;
    #[no_mangle]
    fn identify_fully() -> bool_;
    #[no_mangle]
    fn ident_spell() -> bool_;
    #[no_mangle]
    fn recall_player(d: libc::c_int, f: libc::c_int);
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn teleport_player_level();
    #[no_mangle]
    fn trap_creation() -> bool_;
    #[no_mangle]
    fn summon_specific(y1: libc::c_int, x1: libc::c_int, lev: libc::c_int,
                       type_0: libc::c_int) -> bool_;
    #[no_mangle]
    fn aggravate_monsters(who: libc::c_int);
    #[no_mangle]
    fn unlite_area(dam: libc::c_int, rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn fate_desc(desc: *mut libc::c_char, fate: libc::c_int);
    #[no_mangle]
    fn reset_recall(no_trepas_max_depth: bool_) -> bool_;
    #[no_mangle]
    fn floor_item_charges(item: libc::c_int);
    #[no_mangle]
    fn inven_item_charges(item: libc::c_int);
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn call_chaos();
    #[no_mangle]
    fn fire_bolt_or_beam(prob: libc::c_int, typ: libc::c_int,
                         dir: libc::c_int, dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn poly_monster(dir: libc::c_int) -> bool_;
    #[no_mangle]
    fn drain_life(dir: libc::c_int, dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn slow_monster(dir: libc::c_int) -> bool_;
    #[no_mangle]
    fn sleep_monster(dir: libc::c_int) -> bool_;
    #[no_mangle]
    fn lite_line(dir: libc::c_int) -> bool_;
    #[no_mangle]
    fn disarm_trap(dir: libc::c_int) -> bool_;
    #[no_mangle]
    fn teleport_monster(dir: libc::c_int) -> bool_;
    #[no_mangle]
    fn probing() -> bool_;
    #[no_mangle]
    fn detect_all(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_cmd_home_trump();
    #[no_mangle]
    fn get_aim_dir(dp: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn inc_stack_size_ex(item: libc::c_int, delta: libc::c_int,
                         opt: optimize_flag, desc: describe_flag);
    #[no_mangle]
    fn set_shadow(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn fire_bolt(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn update_mon(m_idx: libc::c_int, full: bool_);
    #[no_mangle]
    fn earthquake(cy: libc::c_int, cx: libc::c_int, r: libc::c_int);
    #[no_mangle]
    fn get_pos_player(dis: libc::c_int, ny: *mut libc::c_int,
                      nx: *mut libc::c_int);
    #[no_mangle]
    fn tgt_pt(x: *mut libc::c_int, y: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn alchemy() -> bool_;
    #[no_mangle]
    fn wall_to_mud(dir: libc::c_int) -> bool_;
    #[no_mangle]
    fn explosive_rune();
    #[no_mangle]
    fn set_tim_esp(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn charm_monsters(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn charm_animals(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn charm_monster(dir: libc::c_int, plev: libc::c_int) -> bool_;
    #[no_mangle]
    fn control_one_undead(dir: libc::c_int, plev: libc::c_int) -> bool_;
    #[no_mangle]
    fn charm_animal(dir: libc::c_int, plev: libc::c_int) -> bool_;
    #[no_mangle]
    fn banish_evil(dist: libc::c_int) -> bool_;
    #[no_mangle]
    fn fire_beam(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn turn_monsters(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn sleep_monsters_touch() -> bool_;
    #[no_mangle]
    fn confuse_monster(dir: libc::c_int, plev: libc::c_int) -> bool_;
    #[no_mangle]
    fn dispel_good(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn dispel_evil(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_monsters_xxx(match_flag: u32b, rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn dispel_monsters(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn alter_reality();
    #[no_mangle]
    fn passwall(dir: libc::c_int, safe: bool_) -> bool_;
    #[no_mangle]
    fn enchant(o_ptr: *mut object_type, n: libc::c_int, eflag: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn apply_magic(o_ptr: *mut object_type, lev: libc::c_int, okay: bool_,
                   good: bool_, great: bool_);
    #[no_mangle]
    fn autosave_checkpoint();
    #[no_mangle]
    fn stair_creation();
    #[no_mangle]
    fn has_ability(ab: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn string_exec_lua(file: *mut libc::c_char) -> cptr;
    #[no_mangle]
    fn rune_exec(spell: *mut rune_spell, cost: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn inven_carry_okay(o_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn get_count(number: libc::c_int, max: libc::c_int);
    #[no_mangle]
    fn get_quantity(prompt: cptr, max: s32b) -> s32b;
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
pub struct ego_item_type {
    pub name: u32b,
    pub text: u32b,
    pub before: bool_,
    pub tval: [byte_hack; 10],
    pub min_sval: [byte_hack; 10],
    pub max_sval: [byte_hack; 10],
    pub rating: byte_hack,
    pub level: byte_hack,
    pub rarity: byte_hack,
    pub mrarity: byte_hack,
    pub max_to_h: s16b,
    pub max_to_d: s16b,
    pub max_to_a: s16b,
    pub activate: s16b,
    pub max_pval: s32b,
    pub cost: s32b,
    pub rar: [byte_hack; 5],
    pub flags1: [u32b; 5],
    pub flags2: [u32b; 5],
    pub flags3: [u32b; 5],
    pub flags4: [u32b; 5],
    pub flags5: [u32b; 5],
    pub esp: [u32b; 5],
    pub oflags1: [u32b; 5],
    pub oflags2: [u32b; 5],
    pub oflags3: [u32b; 5],
    pub oflags4: [u32b; 5],
    pub oflags5: [u32b; 5],
    pub oesp: [u32b; 5],
    pub fego: [u32b; 5],
    pub need_flags1: u32b,
    pub need_flags2: u32b,
    pub need_flags3: u32b,
    pub need_flags4: u32b,
    pub need_flags5: u32b,
    pub need_esp: u32b,
    pub forbid_flags1: u32b,
    pub forbid_flags2: u32b,
    pub forbid_flags3: u32b,
    pub forbid_flags4: u32b,
    pub forbid_flags5: u32b,
    pub forbid_esp: u32b,
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
pub struct activation {
    pub desc: [libc::c_char; 80],
    pub cost: u32b,
    pub spell: s16b,
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
pub struct inscription_info_type {
    pub text: [libc::c_char; 40],
    pub when: byte_hack,
    pub know: bool_,
    pub mana: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rune_spell {
    pub name: [libc::c_char; 30],
    pub type_0: s16b,
    pub rune2: s16b,
    pub mana: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union hook_return {
    pub num: s32b,
    pub str_0: cptr,
    pub o_ptr: *mut object_type,
    pub m_ptr: *mut monster_type,
}
pub type optimize_flag = libc::c_uint;
pub const NO_OPTIMIZE: optimize_flag = 1;
pub const OPTIMIZE: optimize_flag = 0;
pub type describe_flag = libc::c_uint;
pub const NO_DESCRIBE: describe_flag = 1;
pub const DESCRIBE: describe_flag = 0;
/*
 * General function to find an item by its name
 */
#[no_mangle]
pub static mut get_item_hook_find_obj_what: cptr = 0 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn get_item_hook_find_obj(mut item: *mut libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut buf2: [libc::c_char; 100] = [0; 100];
    strcpy(buf.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
    if get_string(get_item_hook_find_obj_what, buf.as_mut_ptr(),
                  79 as libc::c_int) == 0 {
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if !(item_tester_okay(o_ptr) == 0) {
            object_desc(buf2.as_mut_ptr(), o_ptr, -(1 as libc::c_int),
                        0 as libc::c_int);
            if strcmp(buf.as_mut_ptr(), buf2.as_mut_ptr()) == 0 {
                *item = i;
                return 1 as libc::c_int as bool_
            }
        }
        i += 1
    }
    return 0 as libc::c_int as bool_;
}
/*
 * This file includes code for eating food, drinking potions,
 * reading scrolls, aiming wands, using staffs, zapping rods,
 * and activating artifacts.
 *
 * In all cases, if the player becomes "aware" of the item's use
 * by testing it, mark it as "aware" and reward some experience
 * based on the object's level, always rounding up.  If the player
 * remains "unaware", mark that object "kind" as "tried".
 *
 * This code now correctly handles the unstacking of wands, staffs,
 * and rods.  Note the overly paranoid warning about potential pack
 * overflow, which allows the player to use and drop a stacked item.
 *
 * In all "unstacking" scenarios, the "used" object is "carried" as if
 * the player had just picked it up.  In particular, this means that if
 * the use of an item induces pack overflow, that item will be dropped.
 *
 * For simplicity, these routines induce a full "pack reorganization"
 * which not only combines similar items, but also reorganizes various
 * items to obey the current "sorting" method.  This may require about
 * 400 item comparisons, but only occasionally.
 *
 * There may be a BIG problem with any "effect" that can cause "changes"
 * to the p_ptr->inventory.  For example, a "scroll of recharging" can cause
 * a wand/staff to "disappear", moving the p_ptr->inventory up.  Luckily, the
 * scrolls all appear BEFORE the staffs/wands, so this is not a problem.
 * But, for example, a "staff of recharging" could cause MAJOR problems.
 * In such a case, it will be best to either (1) "postpone" the effect
 * until the end of the function, or (2) "change" the effect, say, into
 * giving a staff "negative" charges, or "turning a staff into a stick".
 * It seems as though a "rod of recharging" might in fact cause problems.
 * The basic problem is that the act of recharging (and destroying) an
 * item causes the inducer of that action to "move", causing "o_ptr" to
 * no longer point at the correct item, with horrifying results.
 *
 * Note that food/potions/scrolls no longer use bit-flags for effects,
 * but instead use the "sval" (which is also used to sort the objects).
 */
/*
 * Determine the effects of eating a corpse. A corpse can be
 * eaten whole or cut into pieces for later.
 */
unsafe extern "C" fn corpse_effect(mut o_ptr: *mut object_type,
                                   mut cutting: bool_) {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*o_ptr).pval2 as isize) as *mut monster_race;
    /* Assume no bad effects */
    let mut harmful: bool_ = 0 as libc::c_int as bool_;
    let mut method: byte_hack = 0;
    let mut effect: byte_hack = 0;
    let mut d_dice: byte_hack = 0;
    let mut d_side: byte_hack = 0;
    let mut i: libc::c_int = 0;
    let mut dam: libc::c_int = 0;
    let mut idam: libc::c_int = 0 as libc::c_int;
    let mut mdam: libc::c_int = 0;
    let mut brpow: libc::c_int = 0;
    let mut brdam: libc::c_int = 0 as libc::c_int;
    /* How much of the monster's breath attack remains */
    if (*o_ptr).pval <= (*r_ptr).weight {
        brpow = 0 as libc::c_int
    } else {
        brpow = ((*o_ptr).pval - (*r_ptr).weight) / 5 as libc::c_int;
        if brpow > (*r_ptr).weight / 5 as libc::c_int {
            brpow = (*r_ptr).weight / 5 as libc::c_int
        }
    }
    if (*o_ptr).weight <= 0 as libc::c_int {
        (*o_ptr).weight = 1 as libc::c_int
    }
    if (*o_ptr).pval <= 0 as libc::c_int { (*o_ptr).pval = 1 as libc::c_int }
    /*
	 * The breath is only discharged by accident or by slicing off pieces
	 * of meat, and only by corpses.
	 */
    if (*o_ptr).sval as libc::c_int != 1 as libc::c_int ||
           Rand_div((*o_ptr).weight / 5 as libc::c_int) != 0 && cutting == 0 {
        brpow = 0 as libc::c_int
    }
    /* Immediate effects - poison, acid, fire, etc. */
    if cutting == 0 {
        i = 0 as libc::c_int; /* if (!cutting) */
        while i < 4 as libc::c_int {
            /* skip empty blow slot */
            if !((*r_ptr).blow[i as usize].method == 0) {
                method = (*r_ptr).blow[i as usize].method;
                effect = (*r_ptr).blow[i as usize].effect;
                d_dice = (*r_ptr).blow[i as usize].d_dice;
                d_side = (*r_ptr).blow[i as usize].d_side;
                dam =
                    damroll(d_dice as s16b, d_side as s16b) * (*o_ptr).pval /
                        (*o_ptr).weight / 2 as libc::c_int;
                idam =
                    damroll(d_dice as s16b, d_side as s16b) *
                        (if (*o_ptr).weight / (*o_ptr).pval > 2 as libc::c_int
                            {
                             ((*o_ptr).weight) / (*o_ptr).pval
                         } else { 2 as libc::c_int });
                mdam =
                    maxroll(d_dice as s16b, d_side as s16b) *
                        2 as libc::c_int;
                /* Analyse method */
                match method as libc::c_int {
                    6 | 7 | 11 | 14 | 15 | 17 | 18 | 21 | 22 | 23 => { }
                    _ => {
                        /* Analyse effect */
                        match effect as libc::c_int {
                            2 => {
                                if !((*p_ptr).resist_pois as libc::c_int != 0
                                         ||
                                         (*p_ptr).oppose_pois as libc::c_int
                                             != 0) {
                                    set_poisoned((*p_ptr).poisoned as
                                                     libc::c_int + dam + idam
                                                     + 10 as libc::c_int);
                                    harmful = 1 as libc::c_int as bool_
                                }
                            }
                            9 => {
                                /* Total Immunity */
                                if !((*p_ptr).immune_acid as libc::c_int != 0
                                         || dam <= 0 as libc::c_int) {
                                    /* Resist the damage */
                                    if (*p_ptr).resist_acid != 0 {
                                        dam =
                                            (dam + 2 as libc::c_int) /
                                                3 as libc::c_int
                                    }
                                    if (*p_ptr).oppose_acid != 0 {
                                        dam =
                                            (dam + 2 as libc::c_int) /
                                                3 as libc::c_int
                                    }
                                    /* Take damage */
                                    take_hit(dam,
                                             b"acidic food\x00" as *const u8
                                                 as *const libc::c_char);
                                    harmful = 1 as libc::c_int as bool_
                                } else {
                                    set_oppose_acid((*p_ptr).oppose_acid as
                                                        libc::c_int + idam);
                                }
                            }
                            11 => {
                                /* Totally immune */
                                if (*p_ptr).immune_fire as libc::c_int != 0 ||
                                       dam <= 0 as libc::c_int {
                                    /* Resist the damage */
                                    if (*p_ptr).resist_fire != 0 {
                                        dam =
                                            (dam + 2 as libc::c_int) /
                                                3 as libc::c_int
                                    }
                                    if (*p_ptr).oppose_fire != 0 {
                                        dam =
                                            (dam + 2 as libc::c_int) /
                                                3 as libc::c_int
                                    }
                                    /* Take damage */
                                    take_hit(dam,
                                             b"a fiery meal\x00" as *const u8
                                                 as *const libc::c_char);
                                    harmful = 1 as libc::c_int as bool_
                                } else {
                                    set_oppose_fire((*p_ptr).oppose_fire as
                                                        libc::c_int + idam);
                                }
                            }
                            13 => {
                                if (*p_ptr).resist_blind == 0 {
                                    set_blind((*p_ptr).blind as libc::c_int +
                                                  dam * 2 as libc::c_int +
                                                  idam * 2 as libc::c_int +
                                                  20 as libc::c_int);
                                }
                            }
                            14 => {
                                if (*p_ptr).resist_conf == 0 {
                                    set_confused((*p_ptr).confused as
                                                     libc::c_int + dam + idam
                                                     + 10 as libc::c_int);
                                }
                                if (*p_ptr).resist_chaos == 0 &&
                                       Rand_div(mdam - dam) != 0 {
                                    set_image((*p_ptr).image as libc::c_int +
                                                  dam * 10 as libc::c_int +
                                                  idam * 10 as libc::c_int +
                                                  100 as libc::c_int);
                                }
                            }
                            32 => {
                                if (*p_ptr).resist_chaos == 0 &&
                                       Rand_div(mdam - dam) != 0 {
                                    set_image((*p_ptr).image as libc::c_int +
                                                  dam * 10 as libc::c_int +
                                                  idam * 10 as libc::c_int +
                                                  50 as libc::c_int);
                                }
                            }
                            15 => {
                                if (*p_ptr).resist_fear == 0 {
                                    set_afraid((*p_ptr).afraid as libc::c_int
                                                   + dam + idam +
                                                   10 as libc::c_int);
                                }
                            }
                            16 => {
                                if (*p_ptr).free_act == 0 {
                                    set_paralyzed((*p_ptr).paralyzed as
                                                      libc::c_int + dam + idam
                                                      + 10 as libc::c_int);
                                }
                            }
                            17 => {
                                do_dec_stat(0 as libc::c_int,
                                            2 as libc::c_int);
                            }
                            18 => {
                                do_dec_stat(1 as libc::c_int,
                                            2 as libc::c_int);
                            }
                            19 => {
                                do_dec_stat(2 as libc::c_int,
                                            2 as libc::c_int);
                            }
                            20 => {
                                do_dec_stat(3 as libc::c_int,
                                            2 as libc::c_int);
                            }
                            21 => {
                                do_dec_stat(4 as libc::c_int,
                                            2 as libc::c_int);
                            }
                            22 => {
                                do_dec_stat(5 as libc::c_int,
                                            2 as libc::c_int);
                            }
                            23 => {
                                /* Don't eat Morgoth's corpse :) */
                                do_dec_stat(0 as libc::c_int,
                                            2 as libc::c_int);
                                do_dec_stat(1 as libc::c_int,
                                            2 as libc::c_int);
                                do_dec_stat(2 as libc::c_int,
                                            2 as libc::c_int);
                                do_dec_stat(3 as libc::c_int,
                                            2 as libc::c_int);
                                do_dec_stat(4 as libc::c_int,
                                            2 as libc::c_int);
                                do_dec_stat(5 as libc::c_int,
                                            2 as libc::c_int);
                                (*o_ptr).pval = 1 as libc::c_int
                            }
                            31 => {
                                msg_print(b"You feel your sanity slipping away!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                take_sanity_hit(dam,
                                                b"eating an insane monster\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                            }
                            25 => {
                                /* Unlife is bad to eat */
                                msg_print(b"A black aura surrounds the corpse!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                if (*p_ptr).hold_life as libc::c_int != 0 &&
                                       Rand_div(100 as libc::c_int) <
                                           50 as libc::c_int {
                                    msg_print(b"You keep hold of your life force!\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                } else {
                                    let mut d: s32b =
                                        damroll(10 as libc::c_int as s16b,
                                                6 as libc::c_int as s16b) +
                                            (*p_ptr).exp / 100 as libc::c_int
                                                * 2 as libc::c_int;
                                    if (*p_ptr).hold_life != 0 {
                                        msg_print(b"You feel your life slipping away!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        lose_exp(d / 10 as libc::c_int);
                                    } else {
                                        msg_print(b"You feel your life draining away!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        lose_exp(d);
                                    }
                                }
                                (*o_ptr).pval = 1 as libc::c_int
                            }
                            26 => {
                                msg_print(b"A black aura surrounds the corpse!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                if (*p_ptr).hold_life as libc::c_int != 0 &&
                                       Rand_div(100 as libc::c_int) <
                                           50 as libc::c_int {
                                    msg_print(b"You keep hold of your life force!\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                } else {
                                    let mut d_0: s32b =
                                        damroll(20 as libc::c_int as s16b,
                                                6 as libc::c_int as s16b) +
                                            (*p_ptr).exp / 100 as libc::c_int
                                                * 2 as libc::c_int;
                                    if (*p_ptr).hold_life != 0 {
                                        msg_print(b"You feel your life slipping away!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        lose_exp(d_0 / 10 as libc::c_int);
                                    } else {
                                        msg_print(b"You feel your life draining away!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        lose_exp(d_0);
                                    }
                                }
                                (*o_ptr).pval = 1 as libc::c_int
                            }
                            27 => {
                                msg_print(b"A black aura surrounds the corpse!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                if (*p_ptr).hold_life as libc::c_int != 0 &&
                                       Rand_div(100 as libc::c_int) <
                                           50 as libc::c_int {
                                    msg_print(b"You keep hold of your life force!\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                } else {
                                    let mut d_1: s32b =
                                        damroll(40 as libc::c_int as s16b,
                                                6 as libc::c_int as s16b) +
                                            (*p_ptr).exp / 100 as libc::c_int
                                                * 2 as libc::c_int;
                                    if (*p_ptr).hold_life != 0 {
                                        msg_print(b"You feel your life slipping away!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        lose_exp(d_1 / 10 as libc::c_int);
                                    } else {
                                        msg_print(b"You feel your life draining away!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        lose_exp(d_1);
                                    }
                                }
                                (*o_ptr).pval = 1 as libc::c_int
                            }
                            28 => {
                                msg_print(b"A black aura surrounds the corpse!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                if (*p_ptr).hold_life as libc::c_int != 0 &&
                                       Rand_div(100 as libc::c_int) <
                                           50 as libc::c_int {
                                    msg_print(b"You keep hold of your life force!\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                } else {
                                    let mut d_2: s32b =
                                        damroll(80 as libc::c_int as s16b,
                                                6 as libc::c_int as s16b) +
                                            (*p_ptr).exp / 100 as libc::c_int
                                                * 2 as libc::c_int;
                                    if (*p_ptr).hold_life != 0 {
                                        msg_print(b"You feel your life slipping away!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        lose_exp(d_2 / 10 as libc::c_int);
                                    } else {
                                        msg_print(b"You feel your life draining away!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        lose_exp(d_2);
                                    }
                                }
                                (*o_ptr).pval = 1 as libc::c_int
                            }
                            1 | 3 | 4 | 5 | 6 | 7 | 8 | 10 | 12 | 24 | _ => {
                            }
                        }
                    }
                }
            }
            /* Methods that are meaningless after death */
            i += 1
        }
    }
    /*
	 * The organ that supplies breath attacks is not
	 * immediately emptied upon death, although some types
	 * of breath have no effect.
	 * AMHD's make rather risky meals, and deadly snacks.
	 */
    /* Acid */
    if (*r_ptr).flags4 & 0x100 as libc::c_int as libc::c_uint != 0 &&
           brpow > 0 as libc::c_int {
        brdam =
            if brpow / 3 as libc::c_int > 1600 as libc::c_int {
                1600 as libc::c_int
            } else { (brpow) / 3 as libc::c_int };
        msg_print(b"You are hit by a gush of acid!\x00" as *const u8 as
                      *const libc::c_char);
        /* Total Immunity */
        if !((*p_ptr).immune_acid as libc::c_int != 0 ||
                 brdam <= 0 as libc::c_int) {
            /* Take damage */
            acid_dam(brdam,
                     b"a gush of acid\x00" as *const u8 as
                         *const libc::c_char);
            harmful = 1 as libc::c_int as bool_
        }
        (*o_ptr).pval = 1 as libc::c_int
    } else if (*r_ptr).flags4 & 0x100 as libc::c_int as libc::c_uint != 0 {
        set_oppose_acid((*p_ptr).oppose_acid as libc::c_int +
                            Rand_div(10 as libc::c_int) + 10 as libc::c_int);
    }
    /* Electricity */
    if (*r_ptr).flags4 & 0x200 as libc::c_int as libc::c_uint != 0 &&
           brpow > 0 as libc::c_int {
        brdam =
            if brpow / 3 as libc::c_int > 1600 as libc::c_int {
                1600 as libc::c_int
            } else { (brpow) / 3 as libc::c_int };
        msg_print(b"You receive a heavy shock!\x00" as *const u8 as
                      *const libc::c_char);
        /* Total Immunity */
        if !((*p_ptr).immune_elec as libc::c_int != 0 ||
                 brdam <= 0 as libc::c_int) {
            /* Take damage */
            elec_dam(brdam,
                     b"an electric shock\x00" as *const u8 as
                         *const libc::c_char);
            harmful = 1 as libc::c_int as bool_
        }
        (*o_ptr).weight = (*o_ptr).weight - brpow;
        (*o_ptr).pval = (*o_ptr).weight
    } else if (*r_ptr).flags4 & 0x200 as libc::c_int as libc::c_uint != 0 {
        set_oppose_elec((*p_ptr).oppose_elec as libc::c_int +
                            Rand_div(10 as libc::c_int) + 10 as libc::c_int);
    }
    /* Fire */
    if (*r_ptr).flags4 & 0x400 as libc::c_int as libc::c_uint != 0 &&
           brpow > 0 as libc::c_int {
        brdam =
            if brpow / 3 as libc::c_int > 1600 as libc::c_int {
                1600 as libc::c_int
            } else { (brpow) / 3 as libc::c_int };
        msg_print(b"Roaring flames engulf you!\x00" as *const u8 as
                      *const libc::c_char);
        /* Total Immunity */
        if !((*p_ptr).immune_fire as libc::c_int != 0 ||
                 brdam <= 0 as libc::c_int) {
            /* Take damage */
            fire_dam(brdam,
                     b"an explosion\x00" as *const u8 as *const libc::c_char);
            harmful = 1 as libc::c_int as bool_
        }
        (*o_ptr).pval = 1 as libc::c_int
    } else if (*r_ptr).flags4 & 0x400 as libc::c_int as libc::c_uint != 0 {
        set_oppose_fire((*p_ptr).oppose_fire as libc::c_int +
                            Rand_div(10 as libc::c_int) + 10 as libc::c_int);
    }
    /* Cold */
    if (*r_ptr).flags4 & 0x800 as libc::c_int as libc::c_uint != 0 &&
           brpow > 0 as libc::c_int {
        brdam =
            if brpow / 3 as libc::c_int > 1600 as libc::c_int {
                1600 as libc::c_int
            } else { (brpow) / 3 as libc::c_int };
        msg_print(b"You are caught in a freezing liquid!\x00" as *const u8 as
                      *const libc::c_char);
        /* Total Immunity */
        if !((*p_ptr).immune_cold as libc::c_int != 0 ||
                 brdam <= 0 as libc::c_int) {
            /* Take damage */
            cold_dam(brdam,
                     b"a chilling blast\x00" as *const u8 as
                         *const libc::c_char);
            harmful = 1 as libc::c_int as bool_
        }
        (*o_ptr).weight = (*o_ptr).weight - brpow;
        (*o_ptr).pval = (*o_ptr).weight
    } else if (*r_ptr).flags4 & 0x800 as libc::c_int as libc::c_uint != 0 {
        set_oppose_cold((*p_ptr).oppose_cold as libc::c_int +
                            Rand_div(10 as libc::c_int) + 10 as libc::c_int);
    }
    /* Poison */
    if (*r_ptr).flags4 & 0x1000 as libc::c_int as libc::c_uint != 0 &&
           brpow > 0 as libc::c_int {
        brdam =
            if brpow / 3 as libc::c_int > 800 as libc::c_int {
                800 as libc::c_int
            } else { (brpow) / 3 as libc::c_int };
        msg_print(b"You are surrounded by toxic gases!\x00" as *const u8 as
                      *const libc::c_char);
        /* Resist the damage */
        if (*p_ptr).resist_pois != 0 {
            brdam = (brdam + 2 as libc::c_int) / 3 as libc::c_int
        }
        if (*p_ptr).oppose_pois != 0 {
            brdam = (brdam + 2 as libc::c_int) / 3 as libc::c_int
        }
        if !((*p_ptr).resist_pois as libc::c_int != 0 ||
                 (*p_ptr).oppose_pois as libc::c_int != 0) {
            set_poisoned((*p_ptr).poisoned as libc::c_int + Rand_div(brdam) +
                             10 as libc::c_int);
        }
        /* Take damage */
        take_hit(brdam,
                 b"toxic gases\x00" as *const u8 as *const libc::c_char);
        (*o_ptr).weight = (*o_ptr).weight - brpow;
        (*o_ptr).pval = (*o_ptr).weight;
        harmful = 1 as libc::c_int as bool_
    }
    /* Nether */
    if (*r_ptr).flags4 & 0x2000 as libc::c_int as libc::c_uint != 0 &&
           brpow > 0 as libc::c_int {
        brdam =
            if brpow / 6 as libc::c_int > 550 as libc::c_int {
                550 as libc::c_int
            } else { (brpow) / 6 as libc::c_int };
        msg_print(b"A black aura surrounds the corpse!\x00" as *const u8 as
                      *const libc::c_char);
        if (*p_ptr).resist_neth != 0 {
            brdam *= 6 as libc::c_int;
            brdam /=
                Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                    6 as libc::c_int
        } else if (*p_ptr).hold_life as libc::c_int != 0 &&
                      Rand_div(100 as libc::c_int) < 75 as libc::c_int {
            msg_print(b"You keep hold of your life force!\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*p_ptr).hold_life != 0 {
            msg_print(b"You feel your life slipping away!\x00" as *const u8 as
                          *const libc::c_char);
            lose_exp(200 as libc::c_int +
                         (*p_ptr).exp / 1000 as libc::c_int *
                             2 as libc::c_int);
        } else {
            msg_print(b"You feel your life draining away!\x00" as *const u8 as
                          *const libc::c_char);
            lose_exp(200 as libc::c_int +
                         (*p_ptr).exp / 100 as libc::c_int *
                             2 as libc::c_int);
        }
        /* Take damage */
        take_hit(brdam,
                 b"an unholy blast\x00" as *const u8 as *const libc::c_char);
        harmful = 1 as libc::c_int as bool_;
        (*o_ptr).weight = (*o_ptr).weight - brpow;
        (*o_ptr).pval = (*o_ptr).weight
    }
    /* Confusion */
    if (*r_ptr).flags4 & 0x10000 as libc::c_int as libc::c_uint != 0 &&
           brpow > 0 as libc::c_int {
        msg_print(b"A strange liquid splashes on you!\x00" as *const u8 as
                      *const libc::c_char);
        if (*p_ptr).resist_conf == 0 {
            set_confused((*p_ptr).confused as libc::c_int + brdam + idam +
                             10 as libc::c_int);
        }
        (*o_ptr).weight = (*o_ptr).weight - brpow;
        (*o_ptr).pval = (*o_ptr).weight
    }
    /* Chaos */
    if (*r_ptr).flags4 & 0x40000 as libc::c_int as libc::c_uint != 0 &&
           brpow > 0 as libc::c_int {
        brdam =
            if brpow / 6 as libc::c_int > 600 as libc::c_int {
                600 as libc::c_int
            } else { (brpow) / 6 as libc::c_int };
        msg_print(b"A swirling cloud surrounds you!\x00" as *const u8 as
                      *const libc::c_char);
        if (*p_ptr).resist_chaos != 0 {
            brdam *= 6 as libc::c_int;
            brdam /=
                Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                    6 as libc::c_int
        }
        if (*p_ptr).resist_conf == 0 {
            set_confused((*p_ptr).confused as libc::c_int +
                             Rand_div(20 as libc::c_int) + 10 as libc::c_int);
        }
        if (*p_ptr).resist_chaos == 0 {
            set_image((*p_ptr).image as libc::c_int +
                          (Rand_div(10 as libc::c_int) + 1 as libc::c_int));
        }
        if (*p_ptr).resist_neth == 0 && (*p_ptr).resist_chaos == 0 {
            if (*p_ptr).hold_life as libc::c_int != 0 &&
                   Rand_div(100 as libc::c_int) < 75 as libc::c_int {
                msg_print(b"You keep hold of your life force!\x00" as
                              *const u8 as *const libc::c_char);
            } else if (*p_ptr).hold_life != 0 {
                msg_print(b"You feel your life slipping away!\x00" as
                              *const u8 as *const libc::c_char);
                lose_exp(500 as libc::c_int +
                             (*p_ptr).exp / 1000 as libc::c_int *
                                 2 as libc::c_int);
            } else {
                msg_print(b"You feel your life draining away!\x00" as
                              *const u8 as *const libc::c_char);
                lose_exp(5000 as libc::c_int +
                             (*p_ptr).exp / 100 as libc::c_int *
                                 2 as libc::c_int);
            }
        }
        /* Take damage */
        take_hit(brdam,
                 b"chaotic forces\x00" as *const u8 as *const libc::c_char);
        (*o_ptr).pval = 1 as libc::c_int
    }
    /* Disenchantment */
    if (*r_ptr).flags4 & 0x80000 as libc::c_int as libc::c_uint != 0 &&
           brpow > 0 as libc::c_int {
        brdam =
            if brpow / 6 as libc::c_int > 500 as libc::c_int {
                500 as libc::c_int
            } else { (brpow) / 6 as libc::c_int };
        msg_print(b"You are blasted by raw mana!\x00" as *const u8 as
                      *const libc::c_char);
        if (*p_ptr).resist_disen != 0 {
            brdam *= 6 as libc::c_int;
            brdam /=
                Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                    6 as libc::c_int
        } else { apply_disenchant(0 as libc::c_int); }
        /* Take damage */
        take_hit(brdam, b"raw mana\x00" as *const u8 as *const libc::c_char);
        (*o_ptr).pval = 1 as libc::c_int
    }
    /* Plasma */
    if (*r_ptr).flags4 & 0x2000000 as libc::c_int as libc::c_uint != 0 &&
           brpow > 0 as libc::c_int {
        brdam =
            if brpow / 6 as libc::c_int > 150 as libc::c_int {
                150 as libc::c_int
            } else { (brpow) / 6 as libc::c_int };
        msg_print(b"Searing flames engulf the corpse!\x00" as *const u8 as
                      *const libc::c_char);
        /* Resist the damage */
        if (*p_ptr).resist_fire as libc::c_int != 0 ||
               (*p_ptr).oppose_fire as libc::c_int != 0 {
            brdam = (brdam + 2 as libc::c_int) / 3 as libc::c_int
        }
        if (*p_ptr).resist_sound == 0 {
            let mut k: libc::c_int =
                Rand_div((if brdam > 40 as libc::c_int {
                              35 as libc::c_int
                          } else {
                              (brdam * 3 as libc::c_int / 4 as libc::c_int) +
                                  5 as libc::c_int
                          })) + 1 as libc::c_int;
            set_stun((*p_ptr).stun as libc::c_int + k);
        }
        /* Take damage */
        take_hit(brdam,
                 b"an explosion\x00" as *const u8 as *const libc::c_char);
        harmful = 1 as libc::c_int as bool_;
        (*o_ptr).pval = 1 as libc::c_int
    }
    /* Hack -- Jellies are immune to acid only if they are already acidic */
    if !strchr(b"j\x00" as *const u8 as *const libc::c_char,
               (*r_ptr).d_char as libc::c_int).is_null() &&
           (*r_ptr).flags3 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        dam = damroll(8 as libc::c_int as s16b, 8 as libc::c_int as s16b);
        /* Total Immunity */
        if !((*p_ptr).immune_acid as libc::c_int != 0 ||
                 dam <= 0 as libc::c_int) {
            /* Resist the damage */
            if (*p_ptr).resist_acid != 0 {
                dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
            }
            if (*p_ptr).oppose_acid != 0 {
                dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
            }
            /* Take damage */
            take_hit(dam,
                     b"acidic food\x00" as *const u8 as *const libc::c_char);
        }
        harmful = 1 as libc::c_int as bool_
    }
    /*
	 * Hack -- Jellies, kobolds, spiders, icky things, molds, and mushrooms
	 * are immune to poison because their body already contains
	 * poisonous chemicals.
	 */
    if !strchr(b"ijkmS,\x00" as *const u8 as *const libc::c_char,
               (*r_ptr).d_char as libc::c_int).is_null() &&
           (*r_ptr).flags3 & 0x100000 as libc::c_int as libc::c_uint != 0 {
        if !((*p_ptr).resist_pois as libc::c_int != 0 ||
                 (*p_ptr).oppose_pois as libc::c_int != 0) {
            set_poisoned((*p_ptr).poisoned as libc::c_int +
                             Rand_div(15 as libc::c_int) + 10 as libc::c_int);
        }
        harmful = 1 as libc::c_int as bool_
    }
    /*
	 * Bad effects override good effects
	 * and hacked-up corpses lose intrinsics.
	 */
    if harmful == 0 && cutting == 0 &&
           (*o_ptr).sval as libc::c_int != 5 as libc::c_int {
        if (*r_ptr).flags3 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            set_oppose_acid((*p_ptr).oppose_acid as libc::c_int +
                                Rand_div(10 as libc::c_int) +
                                10 as libc::c_int);
        }
        if (*r_ptr).flags3 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            set_oppose_elec((*p_ptr).oppose_elec as libc::c_int +
                                Rand_div(10 as libc::c_int) +
                                10 as libc::c_int);
        }
        if (*r_ptr).flags3 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            set_oppose_fire((*p_ptr).oppose_fire as libc::c_int +
                                Rand_div(10 as libc::c_int) +
                                10 as libc::c_int);
        }
        if (*r_ptr).flags3 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            set_oppose_cold((*p_ptr).oppose_cold as libc::c_int +
                                Rand_div(10 as libc::c_int) +
                                10 as libc::c_int);
        }
        if (*r_ptr).flags3 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            set_oppose_pois((*p_ptr).oppose_pois as libc::c_int +
                                Rand_div(10 as libc::c_int) +
                                10 as libc::c_int);
        }
        if (*r_ptr).flags3 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            set_protevil((*p_ptr).protevil as libc::c_int +
                             Rand_div(25 as libc::c_int) +
                             3 as libc::c_int *
                                 (*r_ptr).level as libc::c_int);
        }
        if (*r_ptr).flags3 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            set_oppose_fire((*p_ptr).oppose_fire as libc::c_int +
                                Rand_div(20 as libc::c_int) +
                                20 as libc::c_int);
        }
        ((*r_ptr).flags2 & 0x400 as libc::c_int as libc::c_uint) != 0;
        ((*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint) != 0;
        ((*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint) != 0;
        if (*r_ptr).flags3 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            set_afraid(0 as libc::c_int);
        }
        if (*r_ptr).flags3 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            set_stun(0 as libc::c_int);
        }
        if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            set_confused(0 as libc::c_int);
        }
        if (*r_ptr).flags6 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     49 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     16 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     40 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     39 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     0 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let mut k_0: libc::c_int = 0;
            k_0 = 0 as libc::c_int;
            while k_0 < 8 as libc::c_int {
                summon_specific_friendly((*p_ptr).py as libc::c_int,
                                         (*p_ptr).px as libc::c_int,
                                         dun_level as libc::c_int,
                                         0 as libc::c_int,
                                         0 as libc::c_int as bool_);
                k_0 += 1
            }
        }
        if (*r_ptr).flags6 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     17 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     18 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     11 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     12 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     13 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     14 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     15 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     22 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     21 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     31 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
        if (*r_ptr).flags6 & 0x80000000 as libc::c_uint != 0 {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int,
                                     dun_level as libc::c_int,
                                     32 as libc::c_int,
                                     0 as libc::c_int as bool_);
        }
    };
}
/*
 * Hook to determine if an object is eatable
 */
unsafe extern "C" fn item_tester_hook_eatable(mut o_ptr: *mut object_type)
 -> bool_ {
    /* Foods and, well, corpses are edible */
    if (*o_ptr).tval as libc::c_int == 80 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 9 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
/*
 * Eat some food (from the pack or floor)
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_eat_food() {
    let mut item: libc::c_int = 0;
    let mut ident: libc::c_int = 0;
    let mut lev: libc::c_int = 0;
    let mut fval: libc::c_int = 0 as libc::c_int;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
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
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut destroy: bool_ = 1 as libc::c_int as bool_;
    /* Restrict choices to food  */
    item_tester_hook =
        Some(item_tester_hook_eatable as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Set up the extra finder */
    get_item_hook_find_obj_what =
        b"Food full name? \x00" as *const u8 as *const libc::c_char;
    get_item_extra_hook =
        Some(get_item_hook_find_obj as
                 unsafe extern "C" fn(_: *mut libc::c_int) -> bool_);
    /* Get an item */
    q = b"Eat which item? \x00" as *const u8 as *const libc::c_char;
    s = b"You have nothing to eat.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s,
                0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Sound */
    sound(16 as libc::c_int);
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Identity not known yet */
    ident = 0 as libc::c_int;
    /* Object level */
    lev = (*k_info.offset((*o_ptr).k_idx as isize)).level as libc::c_int;
    /* Scripted foods */
    if process_hooks_ret(55 as libc::c_int,
                         b"d\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"(O)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, o_ptr) != 0 {
        ident = process_hooks_return[0 as libc::c_int as usize].num
    } else if (*o_ptr).tval as libc::c_int == 80 as libc::c_int {
        /* (not quite) Normal foods */
        /* Analyze the food */
        match (*o_ptr).sval as libc::c_int {
            41 => {
                (*p_ptr).hp_mod =
                    ((*p_ptr).hp_mod as libc::c_int + 70 as libc::c_int) as
                        s16b;
                msg_print(b"As you eat it you begin to feel your life flow getting stronger.\x00"
                              as *const u8 as *const libc::c_char);
                ident = 1 as libc::c_int;
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long | 0x10 as libc::c_long)
                        as u32b
            }
            0 => {
                if !((*p_ptr).resist_pois as libc::c_int != 0 ||
                         (*p_ptr).oppose_pois as libc::c_int != 0) {
                    if set_poisoned((*p_ptr).poisoned as libc::c_int +
                                        Rand_div(10 as libc::c_int) +
                                        10 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                }
            }
            1 => {
                if (*p_ptr).resist_blind == 0 {
                    if set_blind((*p_ptr).blind as libc::c_int +
                                     Rand_div(200 as libc::c_int) +
                                     200 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                }
            }
            2 => {
                if (*p_ptr).resist_fear == 0 {
                    if set_afraid((*p_ptr).afraid as libc::c_int +
                                      Rand_div(10 as libc::c_int) +
                                      10 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                }
            }
            3 => {
                if (*p_ptr).resist_conf == 0 {
                    if set_confused((*p_ptr).confused as libc::c_int +
                                        Rand_div(10 as libc::c_int) +
                                        10 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                }
            }
            4 => {
                if (*p_ptr).resist_chaos == 0 {
                    if set_image((*p_ptr).image as libc::c_int +
                                     Rand_div(250 as libc::c_int) +
                                     250 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                }
            }
            5 => {
                if (*p_ptr).free_act == 0 {
                    if set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                         Rand_div(10 as libc::c_int) +
                                         10 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                }
            }
            6 => {
                take_hit(damroll(6 as libc::c_int as s16b,
                                 6 as libc::c_int as s16b),
                         b"poisonous food\x00" as *const u8 as
                             *const libc::c_char);
                do_dec_stat(0 as libc::c_int, 2 as libc::c_int);
                ident = 1 as libc::c_int
            }
            7 => {
                take_hit(damroll(6 as libc::c_int as s16b,
                                 6 as libc::c_int as s16b),
                         b"poisonous food\x00" as *const u8 as
                             *const libc::c_char);
                do_dec_stat(4 as libc::c_int, 2 as libc::c_int);
                ident = 1 as libc::c_int
            }
            8 => {
                take_hit(damroll(8 as libc::c_int as s16b,
                                 8 as libc::c_int as s16b),
                         b"poisonous food\x00" as *const u8 as
                             *const libc::c_char);
                do_dec_stat(1 as libc::c_int, 2 as libc::c_int);
                ident = 1 as libc::c_int
            }
            9 => {
                take_hit(damroll(8 as libc::c_int as s16b,
                                 8 as libc::c_int as s16b),
                         b"poisonous food\x00" as *const u8 as
                             *const libc::c_char);
                do_dec_stat(2 as libc::c_int, 2 as libc::c_int);
                ident = 1 as libc::c_int
            }
            10 => {
                take_hit(damroll(10 as libc::c_int as s16b,
                                 10 as libc::c_int as s16b),
                         b"poisonous food\x00" as *const u8 as
                             *const libc::c_char);
                do_dec_stat(4 as libc::c_int, 2 as libc::c_int);
                ident = 1 as libc::c_int
            }
            11 => {
                take_hit(damroll(10 as libc::c_int as s16b,
                                 10 as libc::c_int as s16b),
                         b"poisonous food\x00" as *const u8 as
                             *const libc::c_char);
                do_dec_stat(0 as libc::c_int, 2 as libc::c_int);
                ident = 1 as libc::c_int
            }
            12 => {
                if set_poisoned(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            13 => {
                if set_blind(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            14 => {
                if set_afraid(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            15 => {
                if set_confused(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            16 => {
                if hp_player(damroll(4 as libc::c_int as s16b,
                                     8 as libc::c_int as s16b)) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            17 => {
                if do_res_stat(0 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            18 => {
                if do_res_stat(4 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            19 => {
                if do_res_stat(0 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
                if do_res_stat(1 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
                if do_res_stat(2 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
                if do_res_stat(3 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
                if do_res_stat(4 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
                if do_res_stat(5 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            42 => {
                let mut rumour: [libc::c_char; 80] = [0; 80];
                msg_print(b"That tastes good.\x00" as *const u8 as
                              *const libc::c_char);
                msg_print(b"There is message in the cookie. It says:\x00" as
                              *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
                match Rand_div(20 as libc::c_int) + 1 as libc::c_int {
                    1 => {
                        get_rnd_line(b"chainswd.txt\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     rumour.as_mut_ptr());
                    }
                    2 => {
                        get_rnd_line(b"error.txt\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     rumour.as_mut_ptr());
                    }
                    3 | 4 | 5 => {
                        get_rnd_line(b"death.txt\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     rumour.as_mut_ptr());
                    }
                    _ => {
                        get_rnd_line(b"rumors.txt\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     rumour.as_mut_ptr());
                    }
                }
                msg_format(b"%s\x00" as *const u8 as *const libc::c_char,
                           rumour.as_mut_ptr());
                msg_print(0 as cptr);
                ident = 1 as libc::c_int
            }
            35 | 32 | 33 => {
                msg_print(b"That tastes good.\x00" as *const u8 as
                              *const libc::c_char);
                ident = 1 as libc::c_int
            }
            36 => {
                msg_print(b"That tastes good.\x00" as *const u8 as
                              *const libc::c_char);
                /* 2% chance of getting the mold power */
                if Rand_div(100 as libc::c_int) < 2 as libc::c_int {
                    (*p_ptr).powers_mod[19 as libc::c_int as usize] =
                        1 as libc::c_int as bool_;
                    (*p_ptr).update =
                        ((*p_ptr).update as libc::c_long |
                             0x80 as libc::c_long) as u32b
                }
                ident = 1 as libc::c_int
            }
            37 => {
                msg_print(b"That tastes very good.\x00" as *const u8 as
                              *const libc::c_char);
                set_poisoned(0 as libc::c_int);
                hp_player(damroll(4 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b));
                set_food(15000 as libc::c_int - 1 as libc::c_int);
                ident = 1 as libc::c_int
            }
            38 | 39 => {
                msg_print(b"That tastes good.\x00" as *const u8 as
                              *const libc::c_char);
                ident = 1 as libc::c_int;
                q_ptr = &mut forge;
                object_prep(q_ptr,
                            lookup_kind(2 as libc::c_int, 1 as libc::c_int) as
                                libc::c_int);
                (*q_ptr).number = 1 as libc::c_int as byte_hack;
                object_aware(q_ptr);
                object_known(q_ptr);
                (*q_ptr).ident =
                    ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as
                        byte_hack;
                inven_carry(q_ptr, 0 as libc::c_int as bool_);
            }
            40 => {
                msg_print(b"A fresh, clean essence rises, driving away wounds and poison.\x00"
                              as *const u8 as *const libc::c_char);
                set_poisoned(0 as libc::c_int);
                set_stun(0 as libc::c_int);
                set_cut(0 as libc::c_int);
                if (*p_ptr).black_breath != 0 {
                    msg_print(b"The hold of the Black Breath on you is broken!\x00"
                                  as *const u8 as *const libc::c_char);
                    (*p_ptr).black_breath = 0 as libc::c_int as bool_
                }
                ident = 1 as libc::c_int
            }
            _ => { }
        }
    } else {
        /* Corpses... */
        r_ptr =
            &mut *r_info.offset((*o_ptr).pval2 as isize) as *mut monster_race;
        /* Analyse the corpse */
        match (*o_ptr).sval as libc::c_int {
            1 => {
                let mut no_meat: bool_ = 0 as libc::c_int as bool_;
                /* Not all is edible. Apologies if messy. */
                /* Check weight -- they have to have some meat left */
                if (*r_ptr).flags9 & 0x2 as libc::c_int as libc::c_uint != 0 {
                    if (*o_ptr).weight <=
                           (*r_ptr).weight * 3 as libc::c_int /
                               5 as libc::c_int {
                        no_meat = 1 as libc::c_int as bool_
                    }
                } else if (*o_ptr).weight <=
                              (*r_ptr).weight * 7 as libc::c_int /
                                  20 as libc::c_int {
                    no_meat = 1 as libc::c_int as bool_
                }
                /* Non-skeletons are naturally have more allowances */
                /* Nothing left to eat */
                if no_meat != 0 {
                    msg_print(b"There is not enough meat.\x00" as *const u8 as
                                  *const libc::c_char);
                    return
                }
                /* Check freshness */
                if (*o_ptr).timeout == 0 {
                    msg_print(b"Ugh! Raw meat!\x00" as *const u8 as
                                  *const libc::c_char);
                } else {
                    msg_print(b"That tastes good.\x00" as *const u8 as
                                  *const libc::c_char);
                }
                /* A pound of raw meat */
                (*o_ptr).pval -= 10 as libc::c_int;
                (*o_ptr).weight -= 10 as libc::c_int;
                /* Corpses still have meat on them */
                destroy = 0 as libc::c_int as bool_;
                ident = 1 as libc::c_int
            }
            3 => {
                msg_print(b"You feel rather sick.\x00" as *const u8 as
                              *const libc::c_char);
                /* A pound of raw meat */
                (*o_ptr).pval -= 10 as libc::c_int;
                (*o_ptr).weight -= 10 as libc::c_int;
                /* Corpses still have meat on them */
                destroy = 0 as libc::c_int as bool_;
                ident = 1 as libc::c_int
            }
            5 => {
                /* Just meat */
                if (*o_ptr).timeout == 0 {
                    msg_print(b"You quickly swallow the meat.\x00" as
                                  *const u8 as *const libc::c_char);
                } else {
                    msg_print(b"That tastes good.\x00" as *const u8 as
                                  *const libc::c_char);
                }
                ident = 1 as libc::c_int;
                /* Those darn microorganisms */
                if (*o_ptr).timeout == 0 && (*o_ptr).weight > (*o_ptr).pval &&
                       !((*p_ptr).resist_pois as libc::c_int != 0 ||
                             (*p_ptr).oppose_pois as libc::c_int != 0) {
                    set_poisoned((*p_ptr).poisoned as libc::c_int +
                                     Rand_div((*o_ptr).weight - (*o_ptr).pval)
                                     + ((*o_ptr).weight - (*o_ptr).pval));
                }
            }
            _ => { }
        }
        corpse_effect(o_ptr, 0 as libc::c_int as bool_);
        /* Less nutritious than food rations, but much more of it. */
        fval =
            if (*o_ptr).timeout as libc::c_int != 0 {
                2000 as libc::c_int
            } else { 2500 as libc::c_int };
        /* Those darn microorganisms */
        if (*o_ptr).timeout == 0 &&
               (*o_ptr).weight - (*o_ptr).pval > 10 as libc::c_int &&
               !((*p_ptr).resist_pois as libc::c_int != 0 ||
                     (*p_ptr).oppose_pois as libc::c_int != 0) {
            set_poisoned((*p_ptr).poisoned as libc::c_int +
                             Rand_div((*o_ptr).weight - (*o_ptr).pval) +
                             ((*o_ptr).weight - (*o_ptr).pval));
        }
        /* Partially cured */
        if (*o_ptr).weight > (*o_ptr).timeout as libc::c_int {
            /* Adjust the "timeout" without overflowing */
            (*o_ptr).timeout =
                ((*o_ptr).timeout as libc::c_int *
                     (100 as libc::c_int * (*o_ptr).timeout as libc::c_int /
                          (*o_ptr).weight) / 100 as libc::c_int) as s16b
        }
    }
    /* Combine / Reorder the pack (later) */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* We have tried it */
    object_tried(o_ptr);
    /* The player is now aware of the object */
    if ident != 0 && (*k_info.offset((*o_ptr).k_idx as isize)).aware == 0 {
        object_aware(o_ptr);
        gain_exp((lev + ((*p_ptr).lev as libc::c_int >> 1 as libc::c_int)) /
                     (*p_ptr).lev as libc::c_int);
    }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    if fval == 0 { fval = (*o_ptr).pval }
    /* Food can feed the player, in a different ways */
    /* Vampires */
    if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
            (*spp_ptr).flags1) as libc::c_long & 0x200 as libc::c_long != 0 ||
           (*p_ptr).mimic_form as libc::c_int ==
               resolve_mimic_name(b"Vampire\x00" as *const u8 as
                                      *const libc::c_char) {
        /* Reduced nutritional benefit */
		/*		(void)set_food(p_ptr->food + (fval / 10)); -- No more */
        msg_print(b"Mere victuals hold scant sustenance for a being such as yourself.\x00"
                      as *const u8 as *const libc::c_char);
        /* Hungry */
        if ((*p_ptr).food as libc::c_int) < 2000 as libc::c_int {
            msg_print(b"Your hunger can only be satisfied with fresh blood!\x00"
                          as *const u8 as *const libc::c_char);
        }
    } else if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                   (*spp_ptr).flags1) as libc::c_long & 0x2000 as libc::c_long
                  != 0 {
        if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                (*spp_ptr).flags1) as libc::c_long & 0x400 as libc::c_long !=
               0 {
            msg_print(b"The food of mortals is poor sustenance for you.\x00"
                          as *const u8 as *const libc::c_char);
        } else {
            msg_print(b"Food is poor sustenance for you.\x00" as *const u8 as
                          *const libc::c_char);
        }
        set_food((*p_ptr).food as libc::c_int + fval / 40 as libc::c_int);
    } else {
        /* Those living in fresh */
        set_food((*p_ptr).food as libc::c_int + fval);
    }
    /* Destroy food? */
    if destroy != 0 { inc_stack_size(item, -(1 as libc::c_int)); };
}
/*
 * Cut a corpse up for convenient storage
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_cut_corpse() {
    let mut item: libc::c_int = 0;
    let mut meat: libc::c_int = 0 as libc::c_int;
    let mut not_meat: libc::c_int = 0 as libc::c_int;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut i_ptr: *mut object_type = 0 as *mut object_type;
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
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Restrict choices to corpses */
    item_tester_tval = 9 as libc::c_int as byte_hack;
    /* Get an item */
    q = b"Hack up which corpse? \x00" as *const u8 as *const libc::c_char;
    s = b"You have no corpses.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    r_ptr = &mut *r_info.offset((*o_ptr).pval2 as isize) as *mut monster_race;
    if (*o_ptr).sval as libc::c_int != 1 as libc::c_int &&
           (*o_ptr).sval as libc::c_int != 3 as libc::c_int {
        msg_print(b"You cannot split that.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    match (*o_ptr).sval as libc::c_int {
        1 => {
            if (*r_ptr).flags9 & 0x2 as libc::c_int as libc::c_uint != 0 {
                not_meat =
                    (*r_ptr).weight * 3 as libc::c_int / 5 as libc::c_int
            } else {
                not_meat =
                    (*r_ptr).weight * 7 as libc::c_int / 20 as libc::c_int
            }
            meat =
                (*r_ptr).weight + (*r_ptr).weight / 10 as libc::c_int -
                    not_meat
        }
        3 => {
            not_meat = (*r_ptr).weight / 150 as libc::c_int;
            meat =
                (*r_ptr).weight / 30 as libc::c_int +
                    (*r_ptr).weight / 300 as libc::c_int - not_meat
        }
        _ => { }
    }
    if (*o_ptr).weight <= not_meat || meat < 10 as libc::c_int {
        msg_print(b"There is not enough meat.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Hacking 10 pounds off */
    if meat > 100 as libc::c_int { meat = 100 as libc::c_int }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    (*o_ptr).pval -= meat;
    (*o_ptr).weight -= meat;
    msg_print(b"You hack some meat off the corpse.\x00" as *const u8 as
                  *const libc::c_char);
    corpse_effect(o_ptr, 1 as libc::c_int as bool_);
    /* Get local object */
    i_ptr = &mut object_type_body;
    /* Make some meat */
    object_prep(i_ptr,
                lookup_kind(9 as libc::c_int, 5 as libc::c_int) as
                    libc::c_int);
    (*i_ptr).number = (meat / 10 as libc::c_int) as byte_hack;
    (*i_ptr).pval2 = (*o_ptr).pval2;
    /* Length of time before decay */
    (*i_ptr).pval = 1000 as libc::c_int + Rand_div(1000 as libc::c_int);
    if inven_carry_okay(i_ptr) != 0 {
        inven_carry(i_ptr, 1 as libc::c_int as bool_);
    } else {
        drop_near(i_ptr, 0 as libc::c_int, (*p_ptr).py as libc::c_int,
                  (*p_ptr).px as libc::c_int);
    };
}
/*
 * Use a potion to cure some meat
 *
 * Salt water works well.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_cure_meat() {
    let mut item: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut cure: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut i_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Restrict choices to corpses */
    item_tester_tval = 9 as libc::c_int as byte_hack;
    item_tester_hook =
        Some(item_tester_hook_eatable as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get some meat */
    q = b"Cure which meat? \x00" as *const u8 as *const libc::c_char;
    s = b"You have no meat to cure.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Restrict choices to potions */
    item_tester_tval = 71 as libc::c_int as byte_hack;
    /* Get a potion */
    q = b"Use which potion? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have no potions to use.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    i_ptr = get_object(item);
    if (*i_ptr).number as libc::c_int > 1 as libc::c_int {
        /* Get a number */
        get_count(1 as libc::c_int, (*i_ptr).number as libc::c_int);
        /* Save it */
        num = command_arg as libc::c_int
    } else { num = 1 as libc::c_int }
    if num == 0 as libc::c_int { return }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    q = b"You soak the meat.\x00" as *const u8 as *const libc::c_char;
    s = b"You soak the meat.\x00" as *const u8 as *const libc::c_char;
    match (*i_ptr).sval as libc::c_int {
        5 => {
            q = b"You salt the meat.\x00" as *const u8 as *const libc::c_char;
            cure = 200 as libc::c_int * num
        }
        6 => {
            q =
                b"You poison the meat.\x00" as *const u8 as
                    *const libc::c_char;
            cure = 0 as libc::c_int;
            (*o_ptr).pval /= 2 as libc::c_int;
            if (*o_ptr).pval > (*o_ptr).weight {
                (*o_ptr).pval = (*o_ptr).weight
            }
        }
        9 => { cure = 80 as libc::c_int * num }
        26 => { cure = 20 as libc::c_int * num }
        27 => { cure = 45 as libc::c_int * num }
        23 => {
            q = b"You ruin the meat.\x00" as *const u8 as *const libc::c_char;
            cure = 0 as libc::c_int;
            (*o_ptr).pval /= 10 as libc::c_int;
            if (*o_ptr).pval > (*o_ptr).weight {
                (*o_ptr).pval = (*o_ptr).weight / 2 as libc::c_int
            }
        }
        _ => { cure = 0 as libc::c_int }
    }
    /* Message */
    if (*i_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
           (*k_info.offset((*i_ptr).k_idx as isize)).easy_know as libc::c_int
               != 0 &&
               (*k_info.offset((*i_ptr).k_idx as isize)).aware as libc::c_int
                   != 0 {
        msg_print(q);
    } else { msg_print(s); }
    /* The meat is already spoiling */
    if (*o_ptr).sval as libc::c_int == 5 as libc::c_int &&
           (*o_ptr).weight > (*o_ptr).pval ||
           (*o_ptr).weight - (*o_ptr).pval > 10 as libc::c_int {
        cure = cure * (*o_ptr).pval / ((*o_ptr).weight * 20 as libc::c_int)
    }
    /* Cure the meat */
    (*o_ptr).timeout =
        ((*o_ptr).timeout as libc::c_int +
             cure / (*o_ptr).number as libc::c_int) as s16b;
    if (*o_ptr).timeout as libc::c_int > (*o_ptr).pval {
        (*o_ptr).timeout = (*o_ptr).pval as s16b
    }
    /* Use up the potions */
    inc_stack_size(item, -num);
}
/*
 * Hook to determine if an object is quaffable
 */
unsafe extern "C" fn item_tester_hook_quaffable(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int == 71 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 72 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
unsafe extern "C" fn quaff_potion(mut tval: libc::c_int,
                                  mut sval: libc::c_int,
                                  mut pval: libc::c_int,
                                  mut pval2: libc::c_int) -> bool_ {
    let mut ident: libc::c_int = 0 as libc::c_int;
    /* "Traditional" potions */
    if tval == 71 as libc::c_int {
        match sval {
            0 | 1 | 2 => {
                msg_print(b"You feel less thirsty.\x00" as *const u8 as
                              *const libc::c_char);
                ident = 1 as libc::c_int
            }
            4 => {
                if set_slow((*p_ptr).slow as libc::c_int +
                                (Rand_div(25 as libc::c_int) +
                                     1 as libc::c_int) + 15 as libc::c_int) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            5 => {
                msg_print(b"The potion makes you vomit!\x00" as *const u8 as
                              *const libc::c_char);
                set_food(100 as libc::c_int - 1 as libc::c_int);
                set_poisoned(0 as libc::c_int);
                set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                  4 as libc::c_int);
                ident = 1 as libc::c_int
            }
            6 => {
                if !((*p_ptr).resist_pois as libc::c_int != 0 ||
                         (*p_ptr).oppose_pois as libc::c_int != 0) {
                    if set_poisoned((*p_ptr).poisoned as libc::c_int +
                                        Rand_div(15 as libc::c_int) +
                                        10 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                }
            }
            7 => {
                if (*p_ptr).resist_blind == 0 {
                    if set_blind((*p_ptr).blind as libc::c_int +
                                     Rand_div(100 as libc::c_int) +
                                     100 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                }
            }
            9 => {
                /* Booze */
                if !((*p_ptr).resist_conf as libc::c_int != 0 ||
                         (*p_ptr).resist_chaos as libc::c_int != 0) {
                    if set_confused((*p_ptr).confused as libc::c_int +
                                        Rand_div(20 as libc::c_int) +
                                        15 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                    if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        if set_image((*p_ptr).image as libc::c_int +
                                         Rand_div(150 as libc::c_int) +
                                         150 as libc::c_int) != 0 {
                            ident = 1 as libc::c_int
                        }
                    }
                    if Rand_div(13 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        ident = 1 as libc::c_int;
                        if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                               1 as libc::c_int {
                            lose_all_info();
                        } else { wiz_dark(); }
                        teleport_player(100 as libc::c_int);
                        wiz_dark();
                        msg_print(b"You wake up elsewhere with a sore head...\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(b"You can\'t remember a thing, or how you got here!\x00"
                                      as *const u8 as *const libc::c_char);
                    }
                }
            }
            11 => {
                if (*p_ptr).free_act == 0 {
                    if set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                         Rand_div(4 as libc::c_int) +
                                         4 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                }
            }
            13 => {
                if (*p_ptr).hold_life == 0 && (*p_ptr).exp > 0 as libc::c_int
                   {
                    msg_print(b"You feel your memories fade.\x00" as *const u8
                                  as *const libc::c_char);
                    lose_exp((*p_ptr).exp / 4 as libc::c_int);
                    ident = 1 as libc::c_int
                }
            }
            15 => {
                msg_print(b"Your nerves and muscles feel weak and lifeless!\x00"
                              as *const u8 as *const libc::c_char);
                take_hit(damroll(10 as libc::c_int as s16b,
                                 10 as libc::c_int as s16b),
                         b"a potion of Ruination\x00" as *const u8 as
                             *const libc::c_char);
                dec_stat(3 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                dec_stat(2 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                dec_stat(4 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                dec_stat(0 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                dec_stat(5 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                dec_stat(1 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                ident = 1 as libc::c_int
            }
            16 => {
                if do_dec_stat(0 as libc::c_int, 2 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            17 => {
                if do_dec_stat(1 as libc::c_int, 2 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            18 => {
                if do_dec_stat(2 as libc::c_int, 2 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            19 => {
                if do_dec_stat(3 as libc::c_int, 2 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            20 => {
                if do_dec_stat(4 as libc::c_int, 2 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            21 => {
                if do_dec_stat(5 as libc::c_int, 2 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            22 => {
                msg_print(b"Massive explosions rupture your body!\x00" as
                              *const u8 as *const libc::c_char);
                take_hit(damroll(50 as libc::c_int as s16b,
                                 20 as libc::c_int as s16b),
                         b"a potion of Detonation\x00" as *const u8 as
                             *const libc::c_char);
                set_stun((*p_ptr).stun as libc::c_int + 75 as libc::c_int);
                set_cut((*p_ptr).cut as libc::c_int + 5000 as libc::c_int);
                ident = 1 as libc::c_int
            }
            23 => {
                msg_print(b"A feeling of Death flows through your body.\x00"
                              as *const u8 as *const libc::c_char);
                take_hit(5000 as libc::c_int,
                         b"a potion of Death\x00" as *const u8 as
                             *const libc::c_char);
                ident = 1 as libc::c_int
            }
            24 => {
                if set_tim_infra((*p_ptr).tim_infra as libc::c_int +
                                     100 as libc::c_int +
                                     (Rand_div(100 as libc::c_int) +
                                          1 as libc::c_int)) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            25 => {
                if set_tim_invis((*p_ptr).tim_invis as libc::c_int +
                                     12 as libc::c_int +
                                     (Rand_div(12 as libc::c_int) +
                                          1 as libc::c_int)) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            26 => {
                if set_poisoned((*p_ptr).poisoned as libc::c_int /
                                    2 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            27 => {
                if set_poisoned(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            28 => {
                if set_afraid(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            29 => {
                if (*p_ptr).fast == 0 {
                    if set_fast(Rand_div(25 as libc::c_int) + 1 as libc::c_int
                                    + 15 as libc::c_int, 10 as libc::c_int) !=
                           0 {
                        ident = 1 as libc::c_int
                    }
                } else {
                    set_fast((*p_ptr).fast as libc::c_int + 5 as libc::c_int,
                             10 as libc::c_int);
                }
            }
            30 => {
                if set_oppose_fire((*p_ptr).oppose_fire as libc::c_int +
                                       (Rand_div(10 as libc::c_int) +
                                            1 as libc::c_int) +
                                       10 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            31 => {
                if set_oppose_cold((*p_ptr).oppose_cold as libc::c_int +
                                       (Rand_div(10 as libc::c_int) +
                                            1 as libc::c_int) +
                                       10 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            32 => {
                if set_afraid(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_hero((*p_ptr).hero as libc::c_int +
                                (Rand_div(25 as libc::c_int) +
                                     1 as libc::c_int) + 25 as libc::c_int) !=
                       0 {
                    ident = 1 as libc::c_int
                }
                if hp_player(10 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            33 => {
                if set_afraid(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_shero((*p_ptr).shero as libc::c_int +
                                 (Rand_div(25 as libc::c_int) +
                                      1 as libc::c_int) + 25 as libc::c_int)
                       != 0 {
                    ident = 1 as libc::c_int
                }
                if hp_player(30 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            34 => {
                if hp_player(damroll(2 as libc::c_int as s16b,
                                     8 as libc::c_int as s16b)) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_blind(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_cut((*p_ptr).cut as libc::c_int - 10 as libc::c_int) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            35 => {
                if hp_player(damroll(4 as libc::c_int as s16b,
                                     8 as libc::c_int as s16b)) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_blind(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_confused(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_cut((*p_ptr).cut as libc::c_int / 2 as libc::c_int -
                               50 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            36 => {
                if hp_player(damroll(6 as libc::c_int as s16b,
                                     8 as libc::c_int as s16b)) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_blind(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_confused(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_poisoned(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_stun(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_cut(0 as libc::c_int) != 0 { ident = 1 as libc::c_int }
            }
            37 => {
                if hp_player(300 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_blind(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_confused(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_poisoned(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_stun(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_cut(0 as libc::c_int) != 0 { ident = 1 as libc::c_int }
            }
            38 => {
                if hp_player(1200 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_blind(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_confused(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_poisoned(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_stun(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_cut(0 as libc::c_int) != 0 { ident = 1 as libc::c_int }
            }
            39 => {
                msg_print(b"You feel life flow through your body!\x00" as
                              *const u8 as *const libc::c_char);
                restore_level();
                hp_player(5000 as libc::c_int);
                set_poisoned(0 as libc::c_int);
                set_blind(0 as libc::c_int);
                set_confused(0 as libc::c_int);
                set_image(0 as libc::c_int);
                set_stun(0 as libc::c_int);
                set_cut(0 as libc::c_int);
                do_res_stat(0 as libc::c_int, 1 as libc::c_int as bool_);
                do_res_stat(4 as libc::c_int, 1 as libc::c_int as bool_);
                do_res_stat(3 as libc::c_int, 1 as libc::c_int as bool_);
                do_res_stat(2 as libc::c_int, 1 as libc::c_int as bool_);
                do_res_stat(1 as libc::c_int, 1 as libc::c_int as bool_);
                do_res_stat(5 as libc::c_int, 1 as libc::c_int as bool_);
                if (*p_ptr).black_breath != 0 {
                    msg_print(b"The hold of the Black Breath on you is broken!\x00"
                                  as *const u8 as *const libc::c_char);
                }
                (*p_ptr).black_breath = 0 as libc::c_int as bool_;
                ident = 1 as libc::c_int
            }
            40 => {
                if ((*p_ptr).csp as libc::c_int) < (*p_ptr).msp as libc::c_int
                   {
                    (*p_ptr).csp = (*p_ptr).msp;
                    (*p_ptr).csp_frac = 0 as libc::c_int as u16b;
                    msg_print(b"Your feel your head clear.\x00" as *const u8
                                  as *const libc::c_char);
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x80 as libc::c_long) as u32b;
                    (*p_ptr).window =
                        ((*p_ptr).window as libc::c_long |
                             0x8 as libc::c_long) as u32b;
                    ident = 1 as libc::c_int
                }
            }
            41 => { if restore_level() != 0 { ident = 1 as libc::c_int } }
            42 => {
                if do_res_stat(0 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            43 => {
                if do_res_stat(1 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            44 => {
                if do_res_stat(2 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            45 => {
                if do_res_stat(3 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            46 => {
                if do_res_stat(4 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            47 => {
                if do_res_stat(5 as libc::c_int, 1 as libc::c_int as bool_) !=
                       0 {
                    ident = 1 as libc::c_int
                }
            }
            48 => {
                if do_inc_stat(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            49 => {
                if do_inc_stat(1 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            50 => {
                if do_inc_stat(2 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            51 => {
                if do_inc_stat(3 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            52 => {
                if do_inc_stat(4 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            53 => {
                if do_inc_stat(5 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            55 => {
                if do_inc_stat(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if do_inc_stat(1 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if do_inc_stat(2 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if do_inc_stat(3 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if do_inc_stat(4 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if do_inc_stat(5 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            56 => {
                msg_print(b"An image of your surroundings forms in your mind...\x00"
                              as *const u8 as *const libc::c_char);
                wiz_lite();
                ident = 1 as libc::c_int
            }
            57 => {
                msg_print(b"You begin to feel more enlightened...\x00" as
                              *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
                wiz_lite_extra();
                do_inc_stat(1 as libc::c_int);
                do_inc_stat(2 as libc::c_int);
                detect_traps(25 as libc::c_int);
                detect_doors(25 as libc::c_int);
                detect_stairs(25 as libc::c_int);
                detect_treasure(25 as libc::c_int);
                detect_objects_gold(25 as libc::c_int);
                detect_objects_normal(25 as libc::c_int);
                identify_pack();
                self_knowledge(0 as *mut FILE);
                ident = 1 as libc::c_int
            }
            58 => {
                msg_print(b"You begin to know yourself a little better...\x00"
                              as *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
                self_knowledge(0 as *mut FILE);
                ident = 1 as libc::c_int
            }
            59 => {
                if ((*p_ptr).exp as libc::c_long) < 99999999 as libc::c_long {
                    let mut ee: s32b =
                        (*p_ptr).exp / 2 as libc::c_int + 10 as libc::c_int;
                    if ee as libc::c_long > 100000 as libc::c_long {
                        ee = 100000 as libc::c_long as s32b
                    }
                    msg_print(b"You feel more experienced.\x00" as *const u8
                                  as *const libc::c_char);
                    gain_exp(ee);
                    ident = 1 as libc::c_int
                }
            }
            60 => {
                set_oppose_acid((*p_ptr).oppose_acid as libc::c_int +
                                    (Rand_div(20 as libc::c_int) +
                                         1 as libc::c_int) +
                                    20 as libc::c_int);
                set_oppose_elec((*p_ptr).oppose_elec as libc::c_int +
                                    (Rand_div(20 as libc::c_int) +
                                         1 as libc::c_int) +
                                    20 as libc::c_int);
                set_oppose_fire((*p_ptr).oppose_fire as libc::c_int +
                                    (Rand_div(20 as libc::c_int) +
                                         1 as libc::c_int) +
                                    20 as libc::c_int);
                set_oppose_cold((*p_ptr).oppose_cold as libc::c_int +
                                    (Rand_div(20 as libc::c_int) +
                                         1 as libc::c_int) +
                                    20 as libc::c_int);
                set_oppose_pois((*p_ptr).oppose_pois as libc::c_int +
                                    (Rand_div(20 as libc::c_int) +
                                         1 as libc::c_int) +
                                    20 as libc::c_int);
                ident = 1 as libc::c_int
            }
            61 => {
                if hp_player(50 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_blind(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_poisoned(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_confused(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_stun(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if set_cut(0 as libc::c_int) != 0 { ident = 1 as libc::c_int }
                if set_image(0 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if heal_insanity(50 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            62 => {
                set_invuln((*p_ptr).invuln as libc::c_int +
                               (Rand_div(7 as libc::c_int) + 1 as libc::c_int)
                               + 7 as libc::c_int);
                ident = 1 as libc::c_int
            }
            63 => { do_cmd_rerate(); ident = 1 as libc::c_int }
            3 => {
                msg_print(b"You feel the blood of life running through your veins!\x00"
                              as *const u8 as *const libc::c_char);
                ident = 1 as libc::c_int;
                (*p_ptr).allow_one_death =
                    (*p_ptr).allow_one_death.wrapping_add(1)
            }
            10 => {
                msg_print(b"You feel the dark corruptions of Morgoth coming over you!\x00"
                              as *const u8 as *const libc::c_char);
                gain_random_corruption(0 as libc::c_int);
                ident = 1 as libc::c_int
            }
            8 => {
                let mut t: libc::c_int =
                    30 as libc::c_int +
                        (Rand_div(30 as libc::c_int) + 1 as libc::c_int);
                if set_invis((*p_ptr).tim_invis as libc::c_int + t,
                             35 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                set_tim_invis((*p_ptr).tim_invis as libc::c_int + t);
            }
            12 => {
                (*p_ptr).skill_points =
                    ((*p_ptr).skill_points as libc::c_int +
                         (4 as libc::c_int +
                              Rand_div(1 as libc::c_int +
                                           (10 as libc::c_int +
                                                luck(-(4 as libc::c_int),
                                                     4 as libc::c_int)) -
                                           4 as libc::c_int))) as s16b;
                cmsg_format(13 as libc::c_int as byte_hack,
                            b"You can increase %d more skills.\x00" as
                                *const u8 as *const libc::c_char,
                            (*p_ptr).skill_points as libc::c_int);
            }
            _ => { }
        }
    } else {
        /* "Duplicate" potions */
        match sval {
            1 => {
                if (*p_ptr).mimic_form == 0 {
                    let mut time: s32b = 0;
                    call_lua(b"get_mimic_rand_dur\x00" as *const u8 as
                                 *const libc::c_char,
                             b"(d)\x00" as *const u8 as *const libc::c_char,
                             b"d\x00" as *const u8 as *const libc::c_char,
                             pval2, &mut time as *mut s32b);
                    set_mimic(time, pval2,
                              (*p_ptr).lev as libc::c_int * 2 as libc::c_int /
                                  3 as libc::c_int);
                    /* Redraw title */
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x2 as libc::c_long) as u32b;
                    /* Recalculate bonuses */
                    (*p_ptr).update =
                        ((*p_ptr).update as libc::c_long |
                             0x1 as libc::c_long) as u32b;
                    ident = 1 as libc::c_int
                }
            }
            14 => {
                if heal_insanity(damroll(4 as libc::c_int as s16b,
                                         8 as libc::c_int as s16b)) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            15 => {
                if heal_insanity(damroll(8 as libc::c_int as s16b,
                                         8 as libc::c_int as s16b)) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            16 => {
                if heal_insanity(damroll(12 as libc::c_int as s16b,
                                         8 as libc::c_int as s16b)) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            17 => {
                if heal_insanity(damroll(10 as libc::c_int as s16b,
                                         100 as libc::c_int as s16b)) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            _ => { }
        }
    }
    return ident as bool_;
}
/*
 * Quaff a potion (from the pack or the floor)
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_quaff_potion() {
    let mut item: libc::c_int = 0;
    let mut ident: libc::c_int = 0;
    let mut lev: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
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
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Restrict choices to potions */
    item_tester_hook =
        Some(item_tester_hook_quaffable as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Set up the extra finder */
    get_item_hook_find_obj_what =
        b"Potion full name? \x00" as *const u8 as *const libc::c_char;
    get_item_extra_hook =
        Some(get_item_hook_find_obj as
                 unsafe extern "C" fn(_: *mut libc::c_int) -> bool_);
    /* Get an item */
    q = b"Quaff which potion? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have no potions to quaff.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Sound */
    sound(11 as libc::c_int);
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Not identified yet */
    ident = 0 as libc::c_int;
    /* Object level */
    lev = (*k_info.offset((*o_ptr).k_idx as isize)).level as libc::c_int;
    /* Analyze the potion */
    if process_hooks_ret(23 as libc::c_int,
                         b"d\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"(O)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, o_ptr) != 0 {
        ident = process_hooks_return[0 as libc::c_int as usize].num
    } else {
        ident =
            quaff_potion((*o_ptr).tval as libc::c_int,
                         (*o_ptr).sval as libc::c_int, (*o_ptr).pval,
                         (*o_ptr).pval2 as libc::c_int) as libc::c_int
    }
    /* Combine / Reorder the pack (later) */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* The item has been tried */
    object_tried(o_ptr);
    /* An identification was made */
    if ident != 0 && (*k_info.offset((*o_ptr).k_idx as isize)).aware == 0 {
        object_aware(o_ptr);
        gain_exp((lev + ((*p_ptr).lev as libc::c_int >> 1 as libc::c_int)) /
                     (*p_ptr).lev as libc::c_int);
    }
    if get_skill(39 as libc::c_int) != 0 {
        if item >= 0 as libc::c_int {
            q_ptr = &mut forge;
            object_prep(q_ptr,
                        lookup_kind(2 as libc::c_int, 1 as libc::c_int) as
                            libc::c_int);
            (*q_ptr).number = 1 as libc::c_int as byte_hack;
            object_aware(q_ptr);
            object_known(q_ptr);
            (*q_ptr).ident =
                ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as
                    byte_hack;
            inven_carry(q_ptr, 0 as libc::c_int as bool_);
        }
    }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    /* Potions can feed the player */
    set_food((*p_ptr).food as libc::c_int + (*o_ptr).pval);
    /* Destroy potion */
    inc_stack_size(item, -(1 as libc::c_int));
}
/*
 * Drink from a fountain
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_drink_fountain() {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                             isize)).offset((*p_ptr).px as
                                                                isize) as
            *mut cave_type;
    let mut ident: bool_ = 0;
    let mut tval: libc::c_int = 0;
    let mut sval: libc::c_int = 0;
    let mut pval: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    /* Is the fountain empty? */
    if (*c_ptr).special2 as libc::c_int <= 0 as libc::c_int {
        msg_print(b"The fountain is dried out.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* We quaff or we fill ? */
    if get_com(b"Do you want to [Q]uaff or [F]ill from the fountain? \x00" as
                   *const u8 as *const libc::c_char, &mut ch) == 0 {
        return
    }
    if ch as libc::c_int == 'F' as i32 || ch as libc::c_int == 'f' as i32 {
        do_cmd_fill_bottle();
        return
    } else {
        if ch as libc::c_int == 'Q' as i32 || ch as libc::c_int == 'q' as i32
           {
            if (*c_ptr).special as libc::c_int <= 63 as libc::c_int {
                tval = 71 as libc::c_int;
                sval = (*c_ptr).special as libc::c_int
            } else {
                tval = 72 as libc::c_int;
                sval = (*c_ptr).special as libc::c_int - 63 as libc::c_int
            }
            i = 0 as libc::c_int;
            while i < max_k_idx as libc::c_int {
                let mut k_ptr: *mut object_kind =
                    &mut *k_info.offset(i as isize) as *mut object_kind;
                if !((*k_ptr).tval as libc::c_int != tval) {
                    if !((*k_ptr).sval as libc::c_int != sval) {
                        pval = (*k_ptr).pval;
                        break ;
                    }
                }
                i += 1
            }
            ident = quaff_potion(tval, sval, pval, 0 as libc::c_int);
            (*c_ptr).special2 -= 1;
            if (*c_ptr).special2 as libc::c_int <= 0 as libc::c_int {
                cave_set_feat((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int, 0xf as libc::c_int);
            }
            if ident != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x200 as libc::c_int) as
                        u16b
            }
        }
    };
}
/*
 * Fill an empty bottle
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_fill_bottle() {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                             isize)).offset((*p_ptr).px as
                                                                isize) as
            *mut cave_type;
    let mut tval: libc::c_int = 0;
    let mut sval: libc::c_int = 0;
    let mut item: libc::c_int = 0;
    let mut amt: libc::c_int = 1 as libc::c_int;
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
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
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Is the fountain empty? */
	/*
	 * This check is redundant as it is done in do_cmd_drink_fountain()
	 * but I keep this because someone might want to call this directly.
	 * -- Kusunose
	 */
    if (*c_ptr).special2 as libc::c_int <= 0 as libc::c_int {
        msg_print(b"The fountain has dried up.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Determine the tval/sval of the potion */
    if (*c_ptr).special as libc::c_int <= 63 as libc::c_int {
        tval = 71 as libc::c_int;
        sval = (*c_ptr).special as libc::c_int
    } else {
        tval = 72 as libc::c_int;
        sval = (*c_ptr).special as libc::c_int - 63 as libc::c_int
    }
    /* Restrict choices to bottles */
    item_tester_tval = 2 as libc::c_int as byte_hack;
    /* Get an item */
    q = b"Fill which bottle? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have no bottles to fill.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int) == 0 { return }
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    /* Find out how many the player wants */
    if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
        /* Get a quantity */
        amt = get_quantity(0 as cptr, (*o_ptr).number as s32b);
        /* Allow user abort */
        if amt <= 0 as libc::c_int { return }
    }
    if amt > (*c_ptr).special2 as libc::c_int {
        amt = (*c_ptr).special2 as libc::c_int
    }
    /* Destroy bottles */
    inc_stack_size(item, -amt);
    /* Create the potion */
    q_ptr = &mut forge;
    object_prep(q_ptr, lookup_kind(tval, sval) as libc::c_int);
    (*q_ptr).number = amt as byte_hack;
    if (*c_ptr).info as libc::c_int & 0x200 as libc::c_int != 0 {
        object_aware(q_ptr);
        object_known(q_ptr);
    }
    inven_carry(q_ptr, 1 as libc::c_int as bool_);
    (*c_ptr).special2 = ((*c_ptr).special2 as libc::c_int - amt) as s16b;
    if (*c_ptr).special2 as libc::c_int <= 0 as libc::c_int {
        cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                      0xf as libc::c_int);
    };
}
/*
 * Curse the players armor
 */
#[no_mangle]
pub unsafe extern "C" fn curse_armor() -> bool_ {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Curse the body armor */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(37 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Nothing to curse */
    if (*o_ptr).k_idx == 0 { return 0 as libc::c_int as bool_ }
    /* Describe */
    object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                3 as libc::c_int);
    /* Attempt a saving throw for artifacts */
    if ((*o_ptr).art_name as libc::c_int != 0 ||
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
                  } else { 0 as libc::c_int }) != 0)) &&
           Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        /* Cool */
        msg_format(b"A terrible black aura tries to surround your armour, but your %s resists the effects!\x00"
                       as *const u8 as *const libc::c_char,
                   o_name.as_mut_ptr());
    } else {
        /* not artifact or failed save... */
        /* Oops */
        msg_format(b"A terrible black aura blasts your %s!\x00" as *const u8
                       as *const libc::c_char, o_name.as_mut_ptr());
        (*o_ptr).name1 = 0 as libc::c_int as byte_hack;
        (*o_ptr).name2 = 127 as libc::c_int as s16b;
        (*o_ptr).to_a =
            (0 as libc::c_int -
                 (Rand_div(5 as libc::c_int) + 1 as libc::c_int) -
                 (Rand_div(5 as libc::c_int) + 1 as libc::c_int)) as s16b;
        (*o_ptr).to_h = 0 as libc::c_int as s16b;
        (*o_ptr).to_d = 0 as libc::c_int as s16b;
        (*o_ptr).ac = 0 as libc::c_int as s16b;
        (*o_ptr).dd = 0 as libc::c_int as byte_hack;
        (*o_ptr).ds = 0 as libc::c_int as byte_hack;
        (*o_ptr).art_flags1 = 0 as libc::c_int as u32b;
        (*o_ptr).art_flags2 = 0 as libc::c_int as u32b;
        (*o_ptr).art_flags3 = 0 as libc::c_int as u32b;
        (*o_ptr).art_flags4 = 0 as libc::c_int as u32b;
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                byte_hack;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x20 as libc::c_long) as u32b;
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long |
                      0x8 as libc::c_long)) as u32b
    }
    return 1 as libc::c_int as bool_;
}
/* Blast the armor */
/* Curse it */
/* Recalculate bonuses */
/* Recalculate mana */
/* Window stuff */
/*
 * Curse the players weapon
 */
#[no_mangle]
pub unsafe extern "C" fn curse_weapon() -> bool_ {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Curse the weapon */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Nothing to curse */
    if (*o_ptr).k_idx == 0 { return 0 as libc::c_int as bool_ }
    /* Describe */
    object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                3 as libc::c_int);
    /* Attempt a saving throw */
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
             } else { 0 as libc::c_int }) != 0 ||
            (*o_ptr).art_name as libc::c_int != 0) &&
           Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        /* Cool */
        msg_format(b"A terrible black aura tries to surround your weapon, but your %s resists the effects!\x00"
                       as *const u8 as *const libc::c_char,
                   o_name.as_mut_ptr());
    } else {
        /* not artifact or failed save... */
        /* Oops */
        msg_format(b"A terrible black aura blasts your %s!\x00" as *const u8
                       as *const libc::c_char, o_name.as_mut_ptr());
        (*o_ptr).name1 = 0 as libc::c_int as byte_hack;
        (*o_ptr).name2 = 126 as libc::c_int as s16b;
        (*o_ptr).to_h =
            (0 as libc::c_int -
                 (Rand_div(5 as libc::c_int) + 1 as libc::c_int) -
                 (Rand_div(5 as libc::c_int) + 1 as libc::c_int)) as s16b;
        (*o_ptr).to_d =
            (0 as libc::c_int -
                 (Rand_div(5 as libc::c_int) + 1 as libc::c_int) -
                 (Rand_div(5 as libc::c_int) + 1 as libc::c_int)) as s16b;
        (*o_ptr).to_a = 0 as libc::c_int as s16b;
        (*o_ptr).ac = 0 as libc::c_int as s16b;
        (*o_ptr).dd = 0 as libc::c_int as byte_hack;
        (*o_ptr).ds = 0 as libc::c_int as byte_hack;
        (*o_ptr).art_flags1 = 0 as libc::c_int as u32b;
        (*o_ptr).art_flags2 = 0 as libc::c_int as u32b;
        (*o_ptr).art_flags3 = 0 as libc::c_int as u32b;
        (*o_ptr).art_flags4 = 0 as libc::c_int as u32b;
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                byte_hack;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x20 as libc::c_long) as u32b;
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long |
                      0x8 as libc::c_long)) as u32b
    }
    /* Shatter the weapon */
    /* Curse it */
    /* Recalculate bonuses */
    /* Recalculate mana */
    /* Window stuff */
    /* Notice */
    return 1 as libc::c_int as bool_;
}
/*
 * Hook to determine if an object is readable
 */
unsafe extern "C" fn item_tester_hook_readable(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int == 70 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 8 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
/*
 * Read a scroll (from the pack or floor).
 *
 * Certain scrolls can be "aborted" without losing the scroll.  These
 * include scrolls with no effects but recharge or identify, which are
 * cancelled before use.  XXX Reading them still takes a turn, though.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_read_scroll() {
    let mut item: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut used_up: libc::c_int = 0;
    let mut ident: libc::c_int = 0;
    let mut lev: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
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
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Check some conditions */
    if (*p_ptr).blind != 0 {
        msg_print(b"You can\'t see anything.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if no_lite() != 0 {
        msg_print(b"You have no light by which to read.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Restrict choices to scrolls */
    item_tester_hook =
        Some(item_tester_hook_readable as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Set up the extra finder */
    get_item_hook_find_obj_what =
        b"Scroll full name? \x00" as *const u8 as *const libc::c_char;
    get_item_extra_hook =
        Some(get_item_hook_find_obj as
                 unsafe extern "C" fn(_: *mut libc::c_int) -> bool_);
    /* Get an item */
    q = b"Read which scroll? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have no scrolls to read.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Not identified yet */
    ident = 0 as libc::c_int;
    /* Object level */
    lev = (*k_info.offset((*o_ptr).k_idx as isize)).level as libc::c_int;
    /* Assume the scroll will get used up */
    used_up = 1 as libc::c_int;
    /* New scripts, can override the ingame code */
    if process_hooks_ret(28 as libc::c_int,
                         b"dd\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"(O)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, o_ptr) != 0 {
        used_up = process_hooks_return[0 as libc::c_int as usize].num;
        ident = process_hooks_return[1 as libc::c_int as usize].num
    } else if (*o_ptr).tval as libc::c_int == 70 as libc::c_int {
        /* Traditional scrolls */
        /* Analyze the scroll */
        match (*o_ptr).sval as libc::c_int {
            43 => {
                let mut k_0: libc::c_int = 0;
                ident = 1 as libc::c_int;
                msg_print(b"You feel the souls of the dead coming back from the Halls of Mandos.\x00"
                              as *const u8 as *const libc::c_char);
                k_0 = 0 as libc::c_int;
                while k_0 < max_r_idx as libc::c_int {
                    let mut r_ptr: *mut monster_race =
                        &mut *r_info.offset(k_0 as isize) as
                            *mut monster_race;
                    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint !=
                           0 &&
                           (*r_ptr).flags9 &
                               0x2000 as libc::c_int as libc::c_uint == 0 {
                        (*r_ptr).max_num = 1 as libc::c_int as s16b
                    }
                    k_0 += 1
                }
            }
            40 => {
                if get_check(b"Do you really want to leave your body? (beware, it\'ll be destroyed!) \x00"
                                 as *const u8 as *const libc::c_char) == 0 {
                    used_up = 0 as libc::c_int
                } else {
                    do_cmd_leave_body(0 as libc::c_int as bool_);
                    ident = 1 as libc::c_int;
                    used_up = 1 as libc::c_int
                }
            }
            23 => {
                /* original didn't set used_up flag ??? -- pelpel */
                if reset_recall(1 as libc::c_int as bool_) == 0 {
                    used_up = 0 as libc::c_int
                } else {
                    msg_format(b"Recall reset to %s at level %d.\x00" as
                                   *const u8 as *const libc::c_char,
                               d_name.offset((*d_info.offset((*p_ptr).recall_dungeon
                                                                 as
                                                                 isize)).name
                                                 as isize),
                               *max_dlv.offset((*p_ptr).recall_dungeon as
                                                   isize) as libc::c_int);
                    ident = 1 as libc::c_int;
                    used_up = 1 as libc::c_int
                }
            }
            31 => {
                let mut i: libc::c_int = 0;
                let mut count: libc::c_int = 0 as libc::c_int;
                let mut buf: [libc::c_char; 120] = [0; 120];
                while count < 1000 as libc::c_int {
                    count += 1;
                    i = Rand_div(200 as libc::c_int);
                    if fates[i as usize].fate == 0 { continue ; }
                    if fates[i as usize].know != 0 { continue ; }
                    msg_print(b"A message appears on the scroll. It says:\x00"
                                  as *const u8 as *const libc::c_char);
                    msg_print(0 as cptr);
                    fate_desc(buf.as_mut_ptr(), i);
                    msg_format(b"%s\x00" as *const u8 as *const libc::c_char,
                               buf.as_mut_ptr());
                    msg_print(0 as cptr);
                    msg_print(b"The scroll disappears in a puff of smoke!\x00"
                                  as *const u8 as *const libc::c_char);
                    fates[i as usize].know = 1 as libc::c_int as bool_;
                    ident = 1 as libc::c_int;
                    break ;
                }
            }
            0 => {
                if (*p_ptr).resist_blind == 0 && (*p_ptr).resist_dark == 0 {
                    set_blind((*p_ptr).blind as libc::c_int + 3 as libc::c_int
                                  +
                                  (Rand_div(5 as libc::c_int) +
                                       1 as libc::c_int));
                }
                if unlite_area(10 as libc::c_int, 3 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            1 => {
                msg_print(b"There is a high-pitched humming noise.\x00" as
                              *const u8 as *const libc::c_char);
                aggravate_monsters(1 as libc::c_int);
                ident = 1 as libc::c_int
            }
            2 => { if curse_armor() != 0 { ident = 1 as libc::c_int } }
            3 => { if curse_weapon() != 0 { ident = 1 as libc::c_int } }
            4 => {
                k = 0 as libc::c_int;
                while k < Rand_div(3 as libc::c_int) + 1 as libc::c_int {
                    if summon_specific((*p_ptr).py as libc::c_int,
                                       (*p_ptr).px as libc::c_int,
                                       dun_level as libc::c_int,
                                       0 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                    k += 1
                }
            }
            6 => {
                if summon_specific_friendly((*p_ptr).py as libc::c_int,
                                            (*p_ptr).px as libc::c_int,
                                            dun_level as libc::c_int,
                                            53 as libc::c_int,
                                            0 as libc::c_int as bool_) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            5 => {
                k = 0 as libc::c_int;
                while k < Rand_div(3 as libc::c_int) + 1 as libc::c_int {
                    if summon_specific((*p_ptr).py as libc::c_int,
                                       (*p_ptr).px as libc::c_int,
                                       dun_level as libc::c_int,
                                       17 as libc::c_int) != 0 {
                        ident = 1 as libc::c_int
                    }
                    k += 1
                }
            }
            7 => { if trap_creation() != 0 { ident = 1 as libc::c_int } }
            8 => {
                teleport_player(10 as libc::c_int);
                ident = 1 as libc::c_int
            }
            9 => {
                teleport_player(100 as libc::c_int);
                ident = 1 as libc::c_int
            }
            10 => { teleport_player_level(); ident = 1 as libc::c_int }
            11 => {
                if dungeon_flags2 as libc::c_long & 0x10 as libc::c_long != 0
                       &&
                       get_check(b"Leave this unique level forever? \x00" as
                                     *const u8 as *const libc::c_char) == 0 {
                    used_up = 0 as libc::c_int
                } else {
                    recall_player(21 as libc::c_int, 15 as libc::c_int);
                    ident = 1 as libc::c_int
                }
            }
            12 => {
                ident = 1 as libc::c_int;
                if ident_spell() == 0 { used_up = 0 as libc::c_int }
            }
            13 => {
                ident = 1 as libc::c_int;
                if identify_fully() == 0 { used_up = 0 as libc::c_int }
            }
            14 => {
                if remove_curse() != 0 {
                    msg_print(b"You feel as if someone is watching over you.\x00"
                                  as *const u8 as *const libc::c_char);
                    ident = 1 as libc::c_int
                }
            }
            15 => { remove_all_curse(); ident = 1 as libc::c_int }
            16 => {
                ident = 1 as libc::c_int;
                if enchant_spell(0 as libc::c_int, 0 as libc::c_int,
                                 1 as libc::c_int, 0 as libc::c_int) == 0 {
                    used_up = 0 as libc::c_int
                }
            }
            17 => {
                if enchant_spell(1 as libc::c_int, 0 as libc::c_int,
                                 0 as libc::c_int, 0 as libc::c_int) == 0 {
                    used_up = 0 as libc::c_int
                }
                ident = 1 as libc::c_int
            }
            18 => {
                if enchant_spell(0 as libc::c_int, 1 as libc::c_int,
                                 0 as libc::c_int, 0 as libc::c_int) == 0 {
                    used_up = 0 as libc::c_int
                }
                ident = 1 as libc::c_int
            }
            19 => {
                if enchant_spell(0 as libc::c_int, 0 as libc::c_int,
                                 0 as libc::c_int, 1 as libc::c_int) == 0 {
                    used_up = 0 as libc::c_int
                }
                ident = 1 as libc::c_int
            }
            20 => {
                if enchant_spell(0 as libc::c_int, 0 as libc::c_int,
                                 Rand_div(3 as libc::c_int) + 1 as libc::c_int
                                     + 2 as libc::c_int, 0 as libc::c_int) ==
                       0 {
                    used_up = 0 as libc::c_int
                }
                ident = 1 as libc::c_int
            }
            21 => {
                if enchant_spell(Rand_div(3 as libc::c_int) +
                                     1 as libc::c_int,
                                 Rand_div(3 as libc::c_int) +
                                     1 as libc::c_int, 0 as libc::c_int,
                                 0 as libc::c_int) == 0 {
                    used_up = 0 as libc::c_int
                }
                ident = 1 as libc::c_int
            }
            22 => {
                if recharge(60 as libc::c_int) == 0 {
                    used_up = 0 as libc::c_int
                }
                ident = 1 as libc::c_int
            }
            24 => {
                if lite_area(damroll(2 as libc::c_int as s16b,
                                     8 as libc::c_int as s16b),
                             2 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            25 => { map_area(); ident = 1 as libc::c_int }
            26 => {
                if detect_treasure(25 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if detect_objects_gold(25 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            27 => {
                if detect_objects_normal(25 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            28 => {
                if detect_traps(25 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            29 => {
                if detect_doors(25 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
                if detect_stairs(25 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            30 => {
                if detect_monsters_invis(25 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            32 => {
                if set_food(15000 as libc::c_int - 1 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            33 => {
                if set_blessed((*p_ptr).blessed as libc::c_int +
                                   (Rand_div(12 as libc::c_int) +
                                        1 as libc::c_int) + 6 as libc::c_int)
                       != 0 {
                    ident = 1 as libc::c_int
                }
            }
            34 => {
                if set_blessed((*p_ptr).blessed as libc::c_int +
                                   (Rand_div(24 as libc::c_int) +
                                        1 as libc::c_int) + 12 as libc::c_int)
                       != 0 {
                    ident = 1 as libc::c_int
                }
            }
            35 => {
                if set_blessed((*p_ptr).blessed as libc::c_int +
                                   (Rand_div(48 as libc::c_int) +
                                        1 as libc::c_int) + 24 as libc::c_int)
                       != 0 {
                    ident = 1 as libc::c_int
                }
            }
            36 => {
                if (*p_ptr).confusing as libc::c_int == 0 as libc::c_int {
                    msg_print(b"Your hands begin to glow.\x00" as *const u8 as
                                  *const libc::c_char);
                    (*p_ptr).confusing = 1 as libc::c_int as byte_hack;
                    ident = 1 as libc::c_int
                }
            }
            37 => {
                k = 3 as libc::c_int * (*p_ptr).lev as libc::c_int;
                if set_protevil((*p_ptr).protevil as libc::c_int +
                                    (Rand_div(25 as libc::c_int) +
                                         1 as libc::c_int) + k) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            38 => { warding_glyph(); ident = 1 as libc::c_int }
            39 => {
                if destroy_doors_touch() != 0 { ident = 1 as libc::c_int }
            }
            41 => {
                /* Prevent destruction of quest levels and town */
                if is_quest(dun_level as libc::c_int) == 0 &&
                       dun_level as libc::c_int != 0 {
                    destroy_area((*p_ptr).py as libc::c_int,
                                 (*p_ptr).px as libc::c_int,
                                 15 as libc::c_int, 1 as libc::c_int as bool_,
                                 0 as libc::c_int as bool_);
                } else {
                    msg_print(b"The dungeon trembles...\x00" as *const u8 as
                                  *const libc::c_char);
                }
                ident = 1 as libc::c_int
            }
            42 => {
                if dispel_undead(60 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            }
            44 => {
                genocide(1 as libc::c_int as bool_);
                ident = 1 as libc::c_int
            }
            45 => {
                mass_genocide(1 as libc::c_int as bool_);
                ident = 1 as libc::c_int
            }
            46 => {
                acquirement((*p_ptr).py as libc::c_int,
                            (*p_ptr).px as libc::c_int, 1 as libc::c_int,
                            1 as libc::c_int as bool_,
                            0 as libc::c_int as bool_);
                ident = 1 as libc::c_int
            }
            47 => {
                acquirement((*p_ptr).py as libc::c_int,
                            (*p_ptr).px as libc::c_int,
                            Rand_div(2 as libc::c_int) + 1 as libc::c_int +
                                1 as libc::c_int, 1 as libc::c_int as bool_,
                            0 as libc::c_int as bool_);
                ident = 1 as libc::c_int
            }
            48 => {
                /* ZAngband scrolls */
                fire_ball(5 as libc::c_int, 0 as libc::c_int,
                          150 as libc::c_int, 4 as libc::c_int);
                /*
				 * Note: "Double" damage since it is centered on
				 * the player ...
				 */
                if (*p_ptr).oppose_fire == 0 && (*p_ptr).resist_fire == 0 &&
                       (*p_ptr).immune_fire == 0 {
                    take_hit(if 50 as libc::c_int +
                                    (Rand_div(50 as libc::c_int) +
                                         1 as libc::c_int) +
                                    (*p_ptr).sensible_fire as libc::c_int != 0
                                {
                                 20 as libc::c_int
                             } else { 0 as libc::c_int },
                             b"a Scroll of Fire\x00" as *const u8 as
                                 *const libc::c_char);
                }
                ident = 1 as libc::c_int
            }
            49 => {
                fire_ball(28 as libc::c_int, 0 as libc::c_int,
                          175 as libc::c_int, 4 as libc::c_int);
                if (*p_ptr).oppose_cold == 0 && (*p_ptr).resist_cold == 0 &&
                       (*p_ptr).immune_cold == 0 {
                    take_hit(100 as libc::c_int +
                                 (Rand_div(100 as libc::c_int) +
                                      1 as libc::c_int),
                             b"a Scroll of Ice\x00" as *const u8 as
                                 *const libc::c_char);
                }
                ident = 1 as libc::c_int
            }
            50 => {
                fire_ball(30 as libc::c_int, 0 as libc::c_int,
                          222 as libc::c_int, 4 as libc::c_int);
                if (*p_ptr).resist_chaos == 0 {
                    take_hit(111 as libc::c_int +
                                 (Rand_div(111 as libc::c_int) +
                                      1 as libc::c_int),
                             b"a Scroll of Chaos\x00" as *const u8 as
                                 *const libc::c_char);
                }
                ident = 1 as libc::c_int
            }
            51 => {
                let mut rumour: [libc::c_char; 80] = [0; 80];
                msg_print(b"There is message on the scroll. It says:\x00" as
                              *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
                /* Pick random text */
                match Rand_div(20 as libc::c_int) + 1 as libc::c_int {
                    1 => {
                        get_rnd_line(b"chainswd.txt\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     rumour.as_mut_ptr());
                    }
                    2 => {
                        get_rnd_line(b"error.txt\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     rumour.as_mut_ptr());
                    }
                    3 | 4 | 5 => {
                        get_rnd_line(b"death.txt\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     rumour.as_mut_ptr());
                    }
                    _ => {
                        get_rnd_line(b"rumors.txt\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     rumour.as_mut_ptr());
                    }
                }
                msg_format(b"%s\x00" as *const u8 as *const libc::c_char,
                           rumour.as_mut_ptr());
                msg_print(0 as cptr);
                msg_print(b"The scroll disappears in a puff of smoke!\x00" as
                              *const u8 as *const libc::c_char);
                ident = 1 as libc::c_int
            }
            52 => {
                ident = 1 as libc::c_int;
                if artifact_scroll() == 0 { used_up = 0 as libc::c_int }
            }
            _ => { }
        }
    } else if (*o_ptr).sval as libc::c_int >= 200 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        let mut buf_0: [libc::c_char; 80] = [0; 80];
        let mut fil: [libc::c_char; 20] = [0; 20];
        strnfmt(fil.as_mut_ptr(), 20 as libc::c_int as uint_hack,
                b"book-%d.txt\x00" as *const u8 as *const libc::c_char,
                (*o_ptr).sval as libc::c_int);
        n =
            atoi(get_line(fil.as_mut_ptr(), ANGBAND_DIR_FILE,
                          buf_0.as_mut_ptr(), -(1 as libc::c_int)));
        /* Other readable items */
        /* Maps */
        /* Parse all the fields */
        i_0 = 0 as libc::c_int;
        while i_0 < n {
            /* Grab the fields */
            let mut x: libc::c_int =
                atoi(get_line(fil.as_mut_ptr(), ANGBAND_DIR_FILE,
                              buf_0.as_mut_ptr(), i_0 + 0 as libc::c_int));
            let mut y: libc::c_int =
                atoi(get_line(fil.as_mut_ptr(), ANGBAND_DIR_FILE,
                              buf_0.as_mut_ptr(), i_0 + 1 as libc::c_int));
            let mut w: libc::c_int =
                atoi(get_line(fil.as_mut_ptr(), ANGBAND_DIR_FILE,
                              buf_0.as_mut_ptr(), i_0 + 2 as libc::c_int));
            let mut h: libc::c_int =
                atoi(get_line(fil.as_mut_ptr(), ANGBAND_DIR_FILE,
                              buf_0.as_mut_ptr(), i_0 + 3 as libc::c_int));
            reveal_wilderness_around_player(y, x, h, w);
            i_0 += 4 as libc::c_int
        }
    } else {
        /* Normal parchements */
        /* Save screen */
        screen_save();
        q =
            format(b"book-%d.txt\x00" as *const u8 as *const libc::c_char,
                   (*o_ptr).sval as libc::c_int) as cptr;
        show_file(q, 0 as cptr, 0 as libc::c_int, 0 as libc::c_int);
        screen_load();
        if (*o_ptr).sval as libc::c_int >= 100 as libc::c_int {
            inscription_info[((*o_ptr).sval as libc::c_int -
                                  100 as libc::c_int) as usize].know =
                1 as libc::c_int as bool_
        }
        used_up = 0 as libc::c_int
    }
    /* Get the filename */
    /* Peruse the help file */
    /* Load screen */
    /* Combine / Reorder the pack (later) */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* The item was tried */
    object_tried(o_ptr);
    /* An identification was made */
    if ident != 0 && (*k_info.offset((*o_ptr).k_idx as isize)).aware == 0 {
        object_aware(o_ptr);
        gain_exp((lev + ((*p_ptr).lev as libc::c_int >> 1 as libc::c_int)) /
                     (*p_ptr).lev as libc::c_int);
    }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    /* Hack -- allow certain scrolls to be "preserved" */
    if used_up == 0 { return }
    sound(25 as libc::c_int);
    /* Destroy scroll */
    inc_stack_size(item, -(1 as libc::c_int));
    if get_skill(39 as libc::c_int) != 0 {
        if item >= 0 as libc::c_int {
            q_ptr = &mut forge;
            object_prep(q_ptr,
                        lookup_kind(70 as libc::c_int, 53 as libc::c_int) as
                            libc::c_int);
            object_aware(q_ptr);
            object_known(q_ptr);
            (*q_ptr).ident =
                ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as
                    byte_hack;
            inven_carry(q_ptr, 0 as libc::c_int as bool_);
        }
    };
}
/* Set the 'stick mode' on */
#[no_mangle]
pub unsafe extern "C" fn set_stick_mode(mut o_ptr: *mut object_type) {
    let mut bonus: s32b = (*o_ptr).pval3 & 0xffff as libc::c_int;
    let mut max: s32b = (*o_ptr).pval3 >> 16 as libc::c_int;
    exec_lua(format(b"get_level_use_stick = %d; get_level_max_stick = %d\x00"
                        as *const u8 as *const libc::c_char, bonus, max));
}
/* Remove 'stick mode' */
#[no_mangle]
pub unsafe extern "C" fn unset_stick_mode() {
    exec_lua(b"get_level_use_stick = -1; get_level_max_stick = -1\x00" as
                 *const u8 as *const libc::c_char as *mut libc::c_char);
}
/*
 * Use a staff.                        -RAK-
 *
 * One charge of one staff disappears.
 *
 * Hack -- staffs of identify can be "cancelled".
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_use_staff() {
    let mut item: libc::c_int = 0;
    let mut ident: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut obvious: s32b = 0;
    let mut use_charge: s32b = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Restrict choices to wands */
    item_tester_tval = 55 as libc::c_int as byte_hack;
    /* Set up the extra finder */
    get_item_hook_find_obj_what =
        b"Staff full name? \x00" as *const u8 as *const libc::c_char;
    get_item_extra_hook =
        Some(get_item_hook_find_obj as
                 unsafe extern "C" fn(_: *mut libc::c_int) -> bool_);
    /* Get an item */
    q = b"Use which staff? \x00" as *const u8 as *const libc::c_char;
    s = b"You have no staff to use.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s,
                0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Mega-Hack -- refuse to use a pile from the ground */
    if item < 0 as libc::c_int &&
           (*o_ptr).number as libc::c_int > 1 as libc::c_int {
        msg_print(b"You must first pick up the staffs.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Enter device mode  */
    set_stick_mode(o_ptr);
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Not identified yet */
    ident = 0 as libc::c_int;
    /* get the chance */
    chance =
        exec_lua(format(b"return spell_chance(%d)\x00" as *const u8 as
                            *const libc::c_char,
                        (*o_ptr).pval2 as libc::c_int));
    /* Extract object flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Is it simple to use ? */
    if f4 as libc::c_long & 0x200000 as libc::c_long != 0 {
        chance /= 3 as libc::c_int
    }
    /* Give everyone a (slight) chance */
    if chance < 3 as libc::c_int &&
           Rand_div(3 as libc::c_int - chance + 1 as libc::c_int) ==
               0 as libc::c_int {
        chance = 3 as libc::c_int
    }
    /* Roll for usage */
    if Rand_div(100 as libc::c_int) < chance {
        if flush_failure != 0 { flush(); }
        msg_print(b"You failed to use the staff properly.\x00" as *const u8 as
                      *const libc::c_char);
        sound(55 as libc::c_int);
        /* Leave device mode  */
        unset_stick_mode();
        return
    }
    /* Notice empty staffs */
    if (*o_ptr).pval <= 0 as libc::c_int {
        if flush_failure != 0 { flush(); }
        msg_print(b"The staff has no charges left.\x00" as *const u8 as
                      *const libc::c_char);
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x4 as libc::c_int) as byte_hack;
        /* Leave device mode  */
        unset_stick_mode();
        return
    }
    /* Sound */
    sound(12 as libc::c_int);
    /* Analyze the staff */
    call_lua(b"activate_stick\x00" as *const u8 as *const libc::c_char,
             b"(d)\x00" as *const u8 as *const libc::c_char,
             b"dd\x00" as *const u8 as *const libc::c_char,
             (*o_ptr).pval2 as libc::c_int, &mut obvious as *mut s32b,
             &mut use_charge as *mut s32b);
    /* Combine / Reorder the pack (later) */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* Tried the item */
    object_tried(o_ptr);
    /* An identification was made */
	/* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    /* Hack -- some uses are "free" */
    if use_charge == 0 {
        /* Leave device mode  */
        unset_stick_mode();
        return
    }
    /* An identification was made */
    if obvious != 0 { object_aware(o_ptr); }
    /* Use a single charge */
    (*o_ptr).pval -= 1;
    /* XXX Hack -- unstack if necessary */
    if item >= 0 as libc::c_int &&
           (*o_ptr).number as libc::c_int > 1 as libc::c_int {
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
        /* Get local object */
        q_ptr = &mut forge;
        /* Obtain a local object */
        object_copy(q_ptr, o_ptr);
        /* Modify quantity */
        (*q_ptr).number = 1 as libc::c_int as byte_hack;
        /* Restore the charges */
        (*o_ptr).pval += 1;
        /* Unstack the used item */
        (*o_ptr).number = (*o_ptr).number.wrapping_sub(1);
        item = inven_carry(q_ptr, 0 as libc::c_int as bool_) as libc::c_int;
        /* Message */
        msg_print(b"You unstack your staff.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Describe charges in the pack */
    if item >= 0 as libc::c_int {
        inven_item_charges(item);
    } else {
        /* Describe charges on the floor */
        floor_item_charges(0 as libc::c_int - item);
    }
    /* Leave device mode  */
    unset_stick_mode();
}
/*
 * Aim a wand (from the pack or floor).
 *
 * Use a single charge from a single item.
 * Handle "unstacking" in a logical manner.
 *
 * For simplicity, you cannot use a stack of items from the
 * ground.  This would require too much nasty code.
 *
 * There are no wands which can "destroy" themselves, in the p_ptr->inventory
 * or on the ground, so we can ignore this possibility.  Note that this
 * required giving "wand of wonder" the ability to ignore destruction
 * by electric balls.
 *
 * All wands can be "cancelled" at the "Direction?" prompt for free.
 *
 * Note that the basic "bolt" wands do slightly less damage than the
 * basic "bolt" rods, but the basic "ball" wands do the same damage
 * as the basic "ball" rods.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_aim_wand() {
    let mut obvious: s32b = 0;
    let mut use_charge: s32b = 0;
    let mut item: libc::c_int = 0;
    let mut ident: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut sval: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Restrict choices to wands */
    item_tester_tval = 65 as libc::c_int as byte_hack;
    /* Set up the extra finder */
    get_item_hook_find_obj_what =
        b"Wand full name? \x00" as *const u8 as *const libc::c_char;
    get_item_extra_hook =
        Some(get_item_hook_find_obj as
                 unsafe extern "C" fn(_: *mut libc::c_int) -> bool_);
    /* Get an item */
    q = b"Aim which wand? \x00" as *const u8 as *const libc::c_char;
    s = b"You have no wand to aim.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s,
                0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Mega-Hack -- refuse to aim a pile from the ground */
    if item < 0 as libc::c_int &&
           (*o_ptr).number as libc::c_int > 1 as libc::c_int {
        msg_print(b"You must first pick up the wands.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Not identified yet */
    ident = 0 as libc::c_int;
    /* Enter device mode  */
    set_stick_mode(o_ptr);
    /* get the chance */
    chance =
        exec_lua(format(b"return spell_chance(%d)\x00" as *const u8 as
                            *const libc::c_char,
                        (*o_ptr).pval2 as libc::c_int));
    /* Extract object flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Is it simple to use ? */
    if f4 as libc::c_long & 0x200000 as libc::c_long != 0 {
        chance /= 3 as libc::c_int
    }
    /* Roll for usage */
    if Rand_div(100 as libc::c_int) < chance {
        if flush_failure != 0 { flush(); }
        msg_print(b"You failed to use the wand properly.\x00" as *const u8 as
                      *const libc::c_char);
        sound(55 as libc::c_int);
        /* Leave device mode  */
        unset_stick_mode();
        return
    }
    /* The wand is already empty! */
    if (*o_ptr).pval <= 0 as libc::c_int {
        if flush_failure != 0 { flush(); }
        msg_print(b"The wand has no charges left.\x00" as *const u8 as
                      *const libc::c_char);
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x4 as libc::c_int) as byte_hack;
        /* Leave device mode  */
        unset_stick_mode();
        return
    }
    /* Sound */
    sound(12 as libc::c_int);
    /* XXX Hack -- Extract the "sval" effect */
    sval = (*o_ptr).sval as libc::c_int;
    /* Analyze the wand */
    call_lua(b"activate_stick\x00" as *const u8 as *const libc::c_char,
             b"(d)\x00" as *const u8 as *const libc::c_char,
             b"dd\x00" as *const u8 as *const libc::c_char,
             (*o_ptr).pval2 as libc::c_int, &mut obvious as *mut s32b,
             &mut use_charge as *mut s32b);
    /* Combine / Reorder the pack (later) */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* Mark it as tried */
    object_tried(o_ptr);
    /* Hack -- some uses are "free" */
    if use_charge == 0 {
        /* Leave device mode  */
        unset_stick_mode();
        return
    }
    /* An identification was made */
    if obvious != 0 { object_aware(o_ptr); }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    /* Use a single charge */
    (*o_ptr).pval -= 1;
    /* Describe the charges in the pack */
    if item >= 0 as libc::c_int {
        inven_item_charges(item);
    } else {
        /* Describe the charges on the floor */
        floor_item_charges(0 as libc::c_int - item);
    }
    /* Leave device mode  */
    unset_stick_mode();
}
/*
 * Activate (zap) a Rod
 *
 * Unstack fully charged rods as needed.
 *
 * Hack -- rods of perception/genocide can be "cancelled"
 * All rods can be cancelled at the "Direction?" prompt
 */
/*
 * Hook to determine if an object is zapable
 */
unsafe extern "C" fn item_tester_hook_zapable(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int == 66 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
/*
 * Hook to determine if an object is attachable
 */
unsafe extern "C" fn item_tester_hook_attachable(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int == 67 as libc::c_int &&
           (*o_ptr).pval == 0 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
/*
 * Combine a rod and a rod tip
 */
#[no_mangle]
pub unsafe extern "C" fn zap_combine_rod_tip(mut q_ptr: *mut object_type,
                                             mut tip_item: libc::c_int) {
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut cost: s32b = 0;
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Restrict choices to rods */
    item_tester_hook =
        Some(item_tester_hook_attachable as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q =
        b"Attach the rod tip with which rod? \x00" as *const u8 as
            *const libc::c_char;
    s =
        b"You have no rod to attach to.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int) == 0 { return }
    /* Get the item */
    o_ptr = get_object(item);
    /* Examine the rod */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Calculate rod tip's mana cost */
    cost = (*q_ptr).pval;
    if f4 as libc::c_long & 0x8000 as libc::c_long != 0 {
        cost /= 2 as libc::c_int
    }
    /*
	 * The rod must have at least the same mana capacity as the
	 * rod tip spell needs
	 */
    if ((*o_ptr).pval2 as libc::c_int) < cost {
        msg_print(b"This rod doesn\'t have enough mana for the rod tip.\x00"
                      as *const u8 as *const libc::c_char);
        return
    }
    /* Attach the tip to the rod */
    (*o_ptr).pval = (*q_ptr).sval as s32b;
    /* Destroy rod tip */
    inc_stack_size(tip_item, -(1 as libc::c_int));
}
/*
 * Zap a rod, or attack a rod tip to a rod
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_zap_rod() {
    let mut item: libc::c_int = 0;
    let mut ident: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut lev: libc::c_int = 0;
    let mut cost: libc::c_int = 0;
    let mut require_dir: bool_ = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut tip_ptr: *mut object_kind = 0 as *mut object_kind;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Hack -- let perception get aborted */
    let mut use_charge: bool_ = 1 as libc::c_int as bool_;
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Restrict choices to rods */
    item_tester_hook =
        Some(item_tester_hook_zapable as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Set up the extra finder */
    get_item_hook_find_obj_what =
        b"Rod full name? \x00" as *const u8 as *const libc::c_char;
    get_item_extra_hook =
        Some(get_item_hook_find_obj as
                 unsafe extern "C" fn(_: *mut libc::c_int) -> bool_);
    /* Get an item */
    q = b"Zap which rod? \x00" as *const u8 as *const libc::c_char;
    s = b"You have no rod to zap.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s,
                0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* "Zapping" a Rod Tip on rod of nothing will attach it */
    if (*o_ptr).tval as libc::c_int == 66 as libc::c_int {
        if item >= 0 as libc::c_int {
            zap_combine_rod_tip(o_ptr, item);
            return
        } else {
            msg_print(b"You can\'t zap a rod tip that\'s on the floor.\x00" as
                          *const u8 as *const libc::c_char);
            return
        }
    }
    /* Non-directed rods */
    if (*o_ptr).pval < 12 as libc::c_int {
        require_dir = 0 as libc::c_int as bool_
    } else {
        /* Some rods always require direction */
        match (*o_ptr).pval {
            29 | 28 | 30 => { require_dir = 0 as libc::c_int as bool_ }
            _ => { require_dir = 1 as libc::c_int as bool_ }
        }
    }
    /* Get a direction (unless KNOWN not to need it) */
    if (*k_info.offset((*o_ptr).k_idx as isize)).aware == 0 ||
           require_dir as libc::c_int != 0 {
        /* Get a direction, allow cancel */
        if get_aim_dir(&mut dir) == 0 { return }
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Examine the rod */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f4 as libc::c_long & 0x1000 as libc::c_long != 0 {
        energy_use /= 2 as libc::c_int
    }
    /* Not identified yet */
    ident = 0 as libc::c_int;
    /* Extract the item level */
    tip_ptr =
        &mut *k_info.offset((lookup_kind as
                                 unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int)
                                     ->
                                         s16b)(66 as libc::c_int,
                                               (*o_ptr).pval) as isize) as
            *mut object_kind;
    lev =
        (*k_info.offset(lookup_kind(66 as libc::c_int, (*o_ptr).pval) as
                            isize)).level as libc::c_int;
    /* Base chance of success */
    chance = (*p_ptr).skill_dev as libc::c_int;
    /* Confusion hurts skill */
    if (*p_ptr).confused != 0 { chance = chance / 2 as libc::c_int }
    /* High level objects are harder */
    chance =
        chance -
            (if lev > 50 as libc::c_int { 50 as libc::c_int } else { lev });
    if chance <= 0 as libc::c_int { chance = 1 as libc::c_int }
    /* Is it simple to use ? */
    if f4 as libc::c_long & 0x200000 as libc::c_long != 0 {
        chance *= 10 as libc::c_int
    }
    /* Give everyone a (slight) chance */
    if chance < 3 as libc::c_int &&
           Rand_div(3 as libc::c_int - chance + 1 as libc::c_int) ==
               0 as libc::c_int {
        chance = 3 as libc::c_int
    }
    /* Roll for usage */
    if chance < 3 as libc::c_int ||
           (Rand_div(chance) + 1 as libc::c_int) < 3 as libc::c_int {
        /* Flush input if necessary */
        if flush_failure != 0 { flush(); }
        /* Message */
        msg_print(b"You failed to use the rod properly.\x00" as *const u8 as
                      *const libc::c_char);
        sound(55 as libc::c_int);
        return
    }
    /* Extract mana cost */
    cost = (*tip_ptr).pval;
    /* "Cheapness" ego halven the cost */
    if f4 as libc::c_long & 0x8000 as libc::c_long != 0 {
        cost = cost / 2 as libc::c_int
    }
    /* A single rod is still charging */
    if ((*o_ptr).timeout as libc::c_int) < cost {
        /* Flush input if necessary */
        if flush_failure != 0 { flush(); }
        /* Message */
        msg_print(b"The rod does not have enough mana yet.\x00" as *const u8
                      as *const libc::c_char);
        return
    }
    /* Increase the timeout by the rod kind's pval. */
    (*o_ptr).timeout = ((*o_ptr).timeout as libc::c_int - cost) as s16b;
    /* Sound */
    sound(12 as libc::c_int);
    /* Analyze the rod */
    match (*o_ptr).pval {
        30 => { ident = 1 as libc::c_int; do_cmd_home_trump(); }
        29 => {
            if detect_traps(25 as libc::c_int) != 0 {
                ident = 1 as libc::c_int
            }
        }
        1 => {
            if detect_doors(25 as libc::c_int) != 0 {
                ident = 1 as libc::c_int
            }
            if detect_stairs(25 as libc::c_int) != 0 {
                ident = 1 as libc::c_int
            }
        }
        2 => {
            ident = 1 as libc::c_int;
            if ident_spell() == 0 { use_charge = 0 as libc::c_int as bool_ }
        }
        3 => {
            if dungeon_flags2 as libc::c_long & 0x10 as libc::c_long != 0 &&
                   get_check(b"Leave this unique level forever? \x00" as
                                 *const u8 as *const libc::c_char) == 0 {
                use_charge = 0 as libc::c_int as bool_
            } else {
                recall_player(21 as libc::c_int, 15 as libc::c_int);
                ident = 1 as libc::c_int
            }
        }
        4 => {
            if lite_area(damroll(2 as libc::c_int as s16b,
                                 8 as libc::c_int as s16b), 2 as libc::c_int)
                   != 0 {
                ident = 1 as libc::c_int
            }
        }
        5 => { map_area(); ident = 1 as libc::c_int }
        6 => { detect_all(25 as libc::c_int); ident = 1 as libc::c_int }
        7 => { probing(); ident = 1 as libc::c_int }
        8 => {
            if set_blind(0 as libc::c_int) != 0 { ident = 1 as libc::c_int }
            if set_poisoned(0 as libc::c_int) != 0 {
                ident = 1 as libc::c_int
            }
            if set_confused(0 as libc::c_int) != 0 {
                ident = 1 as libc::c_int
            }
            if set_stun(0 as libc::c_int) != 0 { ident = 1 as libc::c_int }
            if set_cut(0 as libc::c_int) != 0 { ident = 1 as libc::c_int }
            if set_image(0 as libc::c_int) != 0 { ident = 1 as libc::c_int }
        }
        9 => {
            if hp_player(500 as libc::c_int) != 0 { ident = 1 as libc::c_int }
            if set_stun(0 as libc::c_int) != 0 { ident = 1 as libc::c_int }
            if set_cut(0 as libc::c_int) != 0 { ident = 1 as libc::c_int }
        }
        10 => {
            if restore_level() != 0 { ident = 1 as libc::c_int }
            if do_res_stat(0 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                ident = 1 as libc::c_int
            }
            if do_res_stat(1 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                ident = 1 as libc::c_int
            }
            if do_res_stat(2 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                ident = 1 as libc::c_int
            }
            if do_res_stat(3 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                ident = 1 as libc::c_int
            }
            if do_res_stat(4 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                ident = 1 as libc::c_int
            }
            if do_res_stat(5 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                ident = 1 as libc::c_int
            }
        }
        11 => {
            if (*p_ptr).fast == 0 {
                if set_fast(Rand_div(30 as libc::c_int) + 1 as libc::c_int +
                                15 as libc::c_int, 10 as libc::c_int) != 0 {
                    ident = 1 as libc::c_int
                }
            } else {
                set_fast((*p_ptr).fast as libc::c_int + 5 as libc::c_int,
                         10 as libc::c_int);
            }
        }
        13 => { if teleport_monster(dir) != 0 { ident = 1 as libc::c_int } }
        14 => { if disarm_trap(dir) != 0 { ident = 1 as libc::c_int } }
        15 => {
            msg_print(b"A line of blue shimmering light appears.\x00" as
                          *const u8 as *const libc::c_char);
            lite_line(dir);
            ident = 1 as libc::c_int
        }
        16 => { if sleep_monster(dir) != 0 { ident = 1 as libc::c_int } }
        17 => { if slow_monster(dir) != 0 { ident = 1 as libc::c_int } }
        18 => {
            if drain_life(dir, 75 as libc::c_int) != 0 {
                ident = 1 as libc::c_int
            }
        }
        19 => { if poly_monster(dir) != 0 { ident = 1 as libc::c_int } }
        20 => {
            fire_bolt_or_beam(10 as libc::c_int, 3 as libc::c_int, dir,
                              damroll(6 as libc::c_int as s16b,
                                      8 as libc::c_int as s16b));
            ident = 1 as libc::c_int
        }
        21 => {
            fire_bolt_or_beam(10 as libc::c_int, 1 as libc::c_int, dir,
                              damroll(3 as libc::c_int as s16b,
                                      8 as libc::c_int as s16b));
            ident = 1 as libc::c_int
        }
        22 => {
            fire_bolt_or_beam(10 as libc::c_int, 5 as libc::c_int, dir,
                              damroll(8 as libc::c_int as s16b,
                                      8 as libc::c_int as s16b));
            ident = 1 as libc::c_int
        }
        23 => {
            fire_bolt_or_beam(10 as libc::c_int, 4 as libc::c_int, dir,
                              damroll(5 as libc::c_int as s16b,
                                      8 as libc::c_int as s16b));
            ident = 1 as libc::c_int
        }
        24 => {
            fire_ball(3 as libc::c_int, dir, 60 as libc::c_int,
                      2 as libc::c_int);
            ident = 1 as libc::c_int
        }
        25 => {
            fire_ball(1 as libc::c_int, dir, 32 as libc::c_int,
                      2 as libc::c_int);
            ident = 1 as libc::c_int
        }
        26 => {
            fire_ball(5 as libc::c_int, dir, 72 as libc::c_int,
                      2 as libc::c_int);
            ident = 1 as libc::c_int
        }
        27 => {
            fire_ball(4 as libc::c_int, dir, 48 as libc::c_int,
                      2 as libc::c_int);
            ident = 1 as libc::c_int
        }
        28 => { call_chaos(); ident = 1 as libc::c_int }
        _ => {
            process_hooks(27 as libc::c_int,
                          b"(d,d)\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, (*o_ptr).tval as libc::c_int,
                          (*o_ptr).sval as libc::c_int);
        }
    }
    /* Combine / Reorder the pack (later) */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* Tried the object */
    object_tried(o_ptr);
    /* Successfully determined the object function */
    if ident != 0 && (*k_info.offset((*o_ptr).k_idx as isize)).aware == 0 {
        object_aware(o_ptr);
        gain_exp((lev + ((*p_ptr).lev as libc::c_int >> 1 as libc::c_int)) /
                     (*p_ptr).lev as libc::c_int);
    }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    /* Hack -- deal with cancelled zap */
    if use_charge == 0 {
        (*o_ptr).timeout = ((*o_ptr).timeout as libc::c_int + cost) as s16b;
        return
    };
}
/*
 * Hook to determine if an object is activable
 */
unsafe extern "C" fn item_tester_hook_activate(mut o_ptr: *mut object_type)
 -> bool_ {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Not known */
    if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
             (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                 libc::c_int != 0 &&
                 (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                     libc::c_int != 0) {
        return 0 as libc::c_int as bool_
    }
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Check activation flag */
    if f3 as libc::c_long & 0x1000000 as libc::c_long != 0 {
        return 1 as libc::c_int as bool_
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
/*
 * Hack -- activate the ring of power
 */
#[no_mangle]
pub unsafe extern "C" fn ring_of_power() -> libc::c_int {
    let mut ch: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut p: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut plev: libc::c_int = (*p_ptr).lev as libc::c_int;
    let mut timeout: libc::c_int = 0 as libc::c_int;
    loop 
         /* Select power to use */
         {
        if get_com(b"[S]ummon a wraith, [R]ule the world or [C]ast a powerful attack? \x00"
                       as *const u8 as *const libc::c_char, &mut ch) == 0 {
            return 0 as libc::c_int
        }
        if ch as libc::c_int == 'S' as i32 || ch as libc::c_int == 's' as i32
           {
            p = 1 as libc::c_int as libc::c_char;
            break ;
        } else if ch as libc::c_int == 'R' as i32 ||
                      ch as libc::c_int == 'r' as i32 {
            p = 2 as libc::c_int as libc::c_char;
            break ;
        } else {
            if !(ch as libc::c_int == 'C' as i32 ||
                     ch as libc::c_int == 'c' as i32) {
                continue ;
            }
            p = 3 as libc::c_int as libc::c_char;
            break ;
        }
    }
    /* Summon a Wraith */
    if p as libc::c_int == 1 as libc::c_int {
        /* Rewrite this -- pelpel */
        if summon_specific_friendly((*p_ptr).py as libc::c_int,
                                    (*p_ptr).px as libc::c_int,
                                    plev * 3 as libc::c_int /
                                        2 as libc::c_int,
                                    if plev > 47 as libc::c_int {
                                        44 as libc::c_int
                                    } else { 17 as libc::c_int },
                                    if plev > 24 as libc::c_int &&
                                           Rand_div(3 as libc::c_int) +
                                               1 as libc::c_int ==
                                               1 as libc::c_int {
                                        1 as libc::c_int
                                    } else { 0 as libc::c_int } as bool_) != 0
           {
            msg_print(b"Cold winds begin to blow around you, carrying with them the stench of decay...\x00"
                          as *const u8 as *const libc::c_char);
            msg_print(b"Ancient, long-dead forms arise from the ground to serve you!\x00"
                          as *const u8 as *const libc::c_char);
        }
        timeout = 200 as libc::c_int + Rand_div(200 as libc::c_int)
    } else if p as libc::c_int == 2 as libc::c_int {
        msg_print(b"The power of the ring destroys the world!\x00" as
                      *const u8 as *const libc::c_char);
        msg_print(b"The world changes!\x00" as *const u8 as
                      *const libc::c_char);
        autosave_checkpoint();
        /* Rule the World -- only if we can really do so */
        /* Leaving */
        (*p_ptr).leaving = 1 as libc::c_int as bool_;
        timeout = 250 as libc::c_int + Rand_div(250 as libc::c_int)
    } else if p as libc::c_int == 3 as libc::c_int {
        let mut dir: libc::c_int = 0;
        if get_aim_dir(&mut dir) == 0 { return 0 as libc::c_int }
        if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
            msg_print(b"You call the fire of Mount Doom!\x00" as *const u8 as
                          *const libc::c_char);
            fire_ball(27 as libc::c_int, dir, 600 as libc::c_int,
                      4 as libc::c_int);
        } else {
            msg_print(b"Your ring tries to take possession of your enemy\'s mind!\x00"
                          as *const u8 as *const libc::c_char);
            fire_bolt(82 as libc::c_int, dir, 600 as libc::c_int);
        }
        timeout = 300 as libc::c_int + Rand_div(300 as libc::c_int)
    }
    return timeout;
}
/* Cast a powerful spell */
/*
 * Enchant some bolts
 */
#[no_mangle]
pub unsafe extern "C" fn brand_bolts() -> bool_ {
    let mut i: libc::c_int = 0;
    /* Use the first acceptable bolts */
    i = 0 as libc::c_int;
    while i < 23 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-bolts */
        if !((*o_ptr).tval as libc::c_int != 18 as libc::c_int) {
            /* Skip artifacts and ego-items */
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
                           } else { 0 as libc::c_int }) != 0) ||
                     (if (*o_ptr).name2 as libc::c_int != 0 ||
                             (*o_ptr).name2b as libc::c_int != 0 {
                          1 as libc::c_int
                      } else { 0 as libc::c_int }) != 0) {
                /* Skip cursed/broken items */
                if !((*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0)
                   {
                    /* Randomize */
                    if !(Rand_div(100 as libc::c_int) < 75 as libc::c_int) {
                        /* Message */
                        msg_print(b"Your bolts are covered in a fiery aura!\x00"
                                      as *const u8 as *const libc::c_char);
                        /* Ego-item */
                        (*o_ptr).name2 = 122 as libc::c_int as s16b;
                        /* Apply the ego */
                        apply_magic(o_ptr, dun_level as libc::c_int,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_);
                        /* Enchant */
                        enchant(o_ptr,
                                Rand_div(3 as libc::c_int) + 4 as libc::c_int,
                                0x1 as libc::c_int | 0x2 as libc::c_int);
                        /* Notice */
                        return 1 as libc::c_int as bool_
                    }
                }
            }
        }
        i += 1
    }
    /* Flush */
    if flush_failure != 0 { flush(); }
    /* Fail */
    msg_print(b"The fiery enchantment failed.\x00" as *const u8 as
                  *const libc::c_char);
    /* Notice */
    return 1 as libc::c_int as bool_;
}
/*
 * Objects in the p_ptr->inventory can now be activated, and
 * SOME of those may be able to stack (ego wands or something)
 * in any case, we can't know that it's impossible. *BUT* we'll
 * ignore it for now, and the timeout will be set on the entire stack
 * of objects. Reduces their utility, but oh well.
 *
 * Note that it always takes a turn to activate an object, even if
 * the user hits "escape" at the "direction" prompt.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_activate() {
    let mut item: libc::c_int = 0;
    let mut lev: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    let mut spell_choice: libc::c_char = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Prepare the hook */
    item_tester_hook =
        Some(item_tester_hook_activate as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    command_wrk = 0x1 as libc::c_int as s16b;
    q = b"Activate which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to activate.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x1 as libc::c_int | 0x2 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Extract object flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Wearable items have to be worn */
    if f5 as libc::c_long & 0x2000 as libc::c_long == 0 {
        if item < 24 as libc::c_int {
            msg_print(b"You must wear it to activate it.\x00" as *const u8 as
                          *const libc::c_char);
            return
        }
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Extract the item level */
    lev = (*k_info.offset((*o_ptr).k_idx as isize)).level as libc::c_int;
    /* Hack -- Use artifact level instead */
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
            lev =
                random_artifacts[(*o_ptr).sval as usize].level as libc::c_int
        } else {
            lev =
                (*a_info.offset((*o_ptr).name1 as isize)).level as libc::c_int
        }
    }
    /* Base chance of success */
    chance = (*p_ptr).skill_dev as libc::c_int;
    /* Confusion hurts skill */
    if (*p_ptr).confused != 0 { chance = chance / 2 as libc::c_int }
    /* Hight level objects are harder */
    chance =
        chance -
            (if lev > 50 as libc::c_int { 50 as libc::c_int } else { lev });
    if chance <= 0 as libc::c_int { chance = 1 as libc::c_int }
    /* Is it simple to use ? */
    if f4 as libc::c_long & 0x200000 as libc::c_long != 0 {
        chance *= 10 as libc::c_int
    }
    /* Give everyone a (slight) chance */
    if chance < 3 as libc::c_int &&
           Rand_div(3 as libc::c_int - chance + 1 as libc::c_int) ==
               0 as libc::c_int {
        chance = 3 as libc::c_int
    }
    /* Roll for usage */
    if chance < 3 as libc::c_int ||
           (Rand_div(chance) + 1 as libc::c_int) < 3 as libc::c_int {
        if flush_failure != 0 { flush(); }
        msg_print(b"You failed to activate it properly.\x00" as *const u8 as
                      *const libc::c_char);
        sound(55 as libc::c_int);
        return
    }
    /* Check the recharge */
    if (*o_ptr).timeout != 0 {
        /* Mage Staff of Spells -- Have another timeout in xtra2 */
        if ((*o_ptr).name2 as libc::c_int == 4 as libc::c_int ||
                (*o_ptr).name2b as libc::c_int == 4 as libc::c_int) &&
               (*o_ptr).xtra2 as libc::c_int != 0 {
            msg_print(b"It whines, glows and fades...\x00" as *const u8 as
                          *const libc::c_char);
            return
        } else if (*o_ptr).tval as libc::c_int == 10 as libc::c_int {
            msg_print(b"You resume the development of the egg.\x00" as
                          *const u8 as *const libc::c_char);
            (*o_ptr).timeout = 0 as libc::c_int as s16b;
            /* Monster eggs */
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long |
                     (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
            /* Success */
            return
        } else {
            /* Normal activatable items */
            msg_print(b"It whines, glows and fades...\x00" as *const u8 as
                          *const libc::c_char);
            return
        }
    }
    /* Activate the item */
    msg_print(b"You activate it...\x00" as *const u8 as *const libc::c_char);
    /* Sound */
    sound(12 as libc::c_int);
    /* Lua hook ? -- go first to allow lua to override */
    if process_hooks(26 as libc::c_int,
                     b"(d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, item) != 0 {
        return
    }
    /* New mostly unified activation code
	   This has to be early to allow artifacts to override normal items -- neil */
    if activation_aux(o_ptr, 1 as libc::c_int as bool_, item).is_null() {
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
        /* Success */
        return
    }
    /* Mage Staff of Spells */
    if (*o_ptr).name2 as libc::c_int == 4 as libc::c_int ||
           (*o_ptr).name2b as libc::c_int == 4 as libc::c_int {
        loop  {
            if get_com(b"Use Spell [1] or [2]?\x00" as *const u8 as
                           *const libc::c_char, &mut ch) == 0 {
                return
            }
            if ch as libc::c_int == '1' as i32 {
                spell_choice = 1 as libc::c_int as libc::c_char;
                break ;
            } else {
                if !(ch as libc::c_int == '2' as i32) { continue ; }
                spell_choice = 2 as libc::c_int as libc::c_char;
                break ;
            }
        }
        if spell_choice as libc::c_int == 1 as libc::c_int {
            /* Still need to check timeouts because there is another counter */
            if (*o_ptr).timeout != 0 {
                msg_print(b"The first spell is still charging!\x00" as
                              *const u8 as *const libc::c_char);
                return
            }
            /* Cast spell 1 */
            activate_spell(o_ptr, spell_choice as byte_hack);
        } else if spell_choice as libc::c_int == 2 as libc::c_int {
            /* Still need to check timeouts because there is another counter */
            if (*o_ptr).xtra2 != 0 {
                msg_print(b"The second spell is still charging!\x00" as
                              *const u8 as *const libc::c_char);
                return
            }
            /* Cast spell 2 */
            activate_spell(o_ptr, spell_choice as byte_hack);
        }
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
        /* Success */
        return
    }
    /* Monster eggs */
    if (*o_ptr).tval as libc::c_int == 10 as libc::c_int {
        msg_print(b"You stop the development of the egg.\x00" as *const u8 as
                      *const libc::c_char);
        (*o_ptr).timeout = -(1 as libc::c_int) as s16b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
        /* Success */
        return
    }
    /* Musical instruments */
    if (*o_ptr).tval as libc::c_int == 14 as libc::c_int {
        /* Horns */
        if (*o_ptr).sval as libc::c_int == 60 as libc::c_int {
            msg_format(b"Your instrument emits a loud sound!\x00" as *const u8
                           as *const libc::c_char);
            aggravate_monsters(1 as libc::c_int);
            (*o_ptr).timeout = 100 as libc::c_int as s16b
        }
        /* Success */
        return
    }
    /* Mistake */
    msg_print(b"Oops.  That object cannot be activated.\x00" as *const u8 as
                  *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn activation_aux(mut o_ptr: *mut object_type,
                                        mut doit: bool_,
                                        mut item: libc::c_int)
 -> *const libc::c_char {
    let mut plev: libc::c_int = get_skill(56 as libc::c_int) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ii: libc::c_int = 0 as libc::c_int;
    let mut ij: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut dummy: libc::c_int = 0 as libc::c_int;
    let mut chance: libc::c_int = 0;
    let mut is_junkart: bool_ =
        ((*o_ptr).tval as libc::c_int == 102 as libc::c_int) as libc::c_int as
            bool_;
    let mut spell: libc::c_int = 0 as libc::c_int;
    /* Junkarts */
    if is_junkart != 0 {
        spell = activation_info[(*o_ptr).pval2 as usize].spell as libc::c_int
    }
    /* True Actifacts */
    if spell == 0 && (*o_ptr).name1 as libc::c_int != 0 {
        spell =
            (*a_info.offset((*o_ptr).name1 as isize)).activate as libc::c_int
    }
    /* Random and Alchemist Artifacts */
    if spell == 0 && (*o_ptr).art_name as libc::c_int != 0 {
        spell = (*o_ptr).xtra2 as libc::c_int
    }
    /* Ego Items */
    if spell == 0 && (*o_ptr).name2 as libc::c_int != 0 {
        spell =
            (*e_info.offset((*o_ptr).name2 as isize)).activate as libc::c_int
    }
    /* Dual egos with the second ego having the activation */
    if spell == 0 && (*o_ptr).name2b as libc::c_int != 0 {
        spell =
            (*e_info.offset((*o_ptr).name2b as isize)).activate as libc::c_int
    }
    /* Intrinsic to item type (rings of Ice, etc) */
    if spell == 0 {
        spell =
            (*k_info.offset((*o_ptr).k_idx as isize)).activate as libc::c_int
    }
    /* Complain about mis-configured .txt files? */
    if spell == 0 {
        return b"Unknown!\x00" as *const u8 as *const libc::c_char
    }
    /* Negative means a unified spell index */
    if spell < 0 as libc::c_int {
        if doit != 0 {
            call_lua(b"activate_activation\x00" as *const u8 as
                         *const libc::c_char,
                     b"(d,d)\x00" as *const u8 as *const libc::c_char,
                     b"\x00" as *const u8 as *const libc::c_char, -spell,
                     item);
            (*o_ptr).timeout =
                exec_lua(format(b"return get_activation_timeout(%d)\x00" as
                                    *const u8 as *const libc::c_char, -spell))
                    as s16b
        } else {
            return string_exec_lua(format(b"return get_activation_desc(%d)\x00"
                                              as *const u8 as
                                              *const libc::c_char, -spell))
        }
    } else {
        /* Activate for attack */
        match spell {
            26 => {
                if doit == 0 {
                    return b"starlight (75) every 75+d75 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                k = 1 as libc::c_int;
                while k < 10 as libc::c_int {
                    if k - 5 as libc::c_int != 0 {
                        fire_beam(15 as libc::c_int, k, 75 as libc::c_int);
                    }
                    k += 1
                }
                (*o_ptr).timeout =
                    (Rand_div(75 as libc::c_int) + 75 as libc::c_int) as s16b
            }
            27 => {
                if doit == 0 {
                    return b"temporary ESP (dur 20+d20) every 20+d50 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                set_tim_esp((*p_ptr).tim_esp as libc::c_int +
                                (Rand_div(20 as libc::c_int) +
                                     1 as libc::c_int) + 20 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(50 as libc::c_int) + 20 as libc::c_int) as s16b
            }
            28 => {
                if doit == 0 {
                    return b"destruction every 200+d200 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                destroy_area((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int, 15 as libc::c_int,
                             1 as libc::c_int as bool_,
                             0 as libc::c_int as bool_);
                (*o_ptr).timeout =
                    (Rand_div(200 as libc::c_int) + 200 as libc::c_int) as
                        s16b
            }
            29 => {
                if doit == 0 {
                    return b"berserk strength every 50+d50 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                set_afraid(0 as libc::c_int);
                set_shero((*p_ptr).shero as libc::c_int +
                              (Rand_div(25 as libc::c_int) + 1 as libc::c_int)
                              + 25 as libc::c_int);
                hp_player(30 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(50 as libc::c_int) + 50 as libc::c_int) as s16b
            }
            30 => {
                if doit == 0 {
                    return b"dispel evil (x4) every 100+d100 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                dispel_evil((*p_ptr).lev as libc::c_int * 4 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(100 as libc::c_int) + 100 as libc::c_int) as
                        s16b
            }
            31 => {
                if doit == 0 {
                    return b"mana bolt (9d8) 7+d7 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_bolt(26 as libc::c_int, dir,
                              damroll(9 as libc::c_int as s16b,
                                      8 as libc::c_int as s16b));
                    (*o_ptr).timeout =
                        (Rand_div(7 as libc::c_int) + 7 as libc::c_int) as
                            s16b
                }
            }
            32 => {
                if doit == 0 {
                    return b"magic arrow (10d10) every 20+d20 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_bolt(10 as libc::c_int, dir,
                              damroll(10 as libc::c_int as s16b,
                                      10 as libc::c_int as s16b));
                    (*o_ptr).timeout =
                        (Rand_div(20 as libc::c_int) + 20 as libc::c_int) as
                            s16b
                }
            }
            33 => {
                /* Give full knowledge */
				/* Hack -- Maximal info */
                let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
                let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
                let mut x: libc::c_int = 0;
                let mut y: libc::c_int = 0;
                let mut m: libc::c_int = 0;
                if doit == 0 {
                    return b"analyze monster every 500+d200 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(tgt_pt(&mut x, &mut y) == 0) {
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset(y as
                                                             isize)).offset(x
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    if !((*c_ptr).m_idx == 0) {
                        r_ptr =
                            &mut *r_info.offset((*c_ptr).m_idx as isize) as
                                *mut monster_race;
                        /* Observe "maximal" attacks */
                        m = 0 as libc::c_int;
                        while m < 4 as libc::c_int {
                            /* Examine "actual" blows */
                            if (*r_ptr).blow[m as usize].effect as libc::c_int
                                   != 0 ||
                                   (*r_ptr).blow[m as usize].method as
                                       libc::c_int != 0 {
                                /* Hack -- maximal observations */
                                (*r_ptr).r_blows[m as usize] =
                                    255 as libc::c_int as byte_hack
                            }
                            m += 1
                        }
                        /* Hack -- maximal drops */
                        (*r_ptr).r_drop_item =
                            ((if (*r_ptr).flags1 &
                                     0x8000000 as libc::c_int as libc::c_uint
                                     != 0 {
                                  8 as libc::c_int
                              } else { 0 as libc::c_int }) +
                                 (if (*r_ptr).flags1 &
                                         0x4000000 as libc::c_int as
                                             libc::c_uint != 0 {
                                      6 as libc::c_int
                                  } else { 0 as libc::c_int }) +
                                 (if (*r_ptr).flags1 &
                                         0x2000000 as libc::c_int as
                                             libc::c_uint != 0 {
                                      4 as libc::c_int
                                  } else { 0 as libc::c_int }) +
                                 (if (*r_ptr).flags1 &
                                         0x1000000 as libc::c_int as
                                             libc::c_uint != 0 {
                                      2 as libc::c_int
                                  } else { 0 as libc::c_int }) +
                                 (if (*r_ptr).flags1 &
                                         0x800000 as libc::c_int as
                                             libc::c_uint != 0 {
                                      1 as libc::c_int
                                  } else { 0 as libc::c_int }) +
                                 (if (*r_ptr).flags1 &
                                         0x400000 as libc::c_int as
                                             libc::c_uint != 0 {
                                      1 as libc::c_int
                                  } else { 0 as libc::c_int })) as byte_hack;
                        (*r_ptr).r_drop_gold = (*r_ptr).r_drop_item;
                        /* Hack -- but only "valid" drops */
                        if (*r_ptr).flags1 &
                               0x100000 as libc::c_int as libc::c_uint != 0 {
                            (*r_ptr).r_drop_item =
                                0 as libc::c_int as byte_hack
                        }
                        if (*r_ptr).flags1 &
                               0x200000 as libc::c_int as libc::c_uint != 0 {
                            (*r_ptr).r_drop_gold =
                                0 as libc::c_int as byte_hack
                        }
                        /* Hack -- observe many spells */
                        (*r_ptr).r_cast_inate =
                            255 as libc::c_int as byte_hack;
                        (*r_ptr).r_cast_spell =
                            255 as libc::c_int as byte_hack;
                        /* Hack -- know all the flags */
                        (*r_ptr).r_flags1 = (*r_ptr).flags1;
                        (*r_ptr).r_flags2 = (*r_ptr).flags2;
                        (*r_ptr).r_flags3 = (*r_ptr).flags3;
                        (*r_ptr).r_flags4 = (*r_ptr).flags4;
                        (*r_ptr).r_flags5 = (*r_ptr).flags5;
                        (*r_ptr).r_flags6 = (*r_ptr).flags6;
                        (*r_ptr).r_flags7 = (*r_ptr).flags7;
                        (*r_ptr).r_flags8 = (*r_ptr).flags8;
                        (*r_ptr).r_flags9 = (*r_ptr).flags9;
                        (*o_ptr).timeout =
                            (Rand_div(200 as libc::c_int) +
                                 500 as libc::c_int) as s16b
                    }
                }
            }
            34 => {
                if doit == 0 {
                    return b"whispers from beyond(sanity drain) 100+d200 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                identify_fully();
                take_sanity_hit(damroll(10 as libc::c_int as s16b,
                                        7 as libc::c_int as s16b),
                                b"the sounds of the dead\x00" as *const u8 as
                                    *const libc::c_char);
                (*o_ptr).timeout =
                    (Rand_div(200 as libc::c_int) + 100 as libc::c_int) as
                        s16b
            }
            35 => {
                if doit == 0 {
                    return b"ruination every 10+d10 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"The phial wells with dark light...\x00" as
                              *const u8 as *const libc::c_char);
                unlite_area(damroll(2 as libc::c_int as s16b,
                                    15 as libc::c_int as s16b),
                            3 as libc::c_int);
                take_hit(damroll(10 as libc::c_int as s16b,
                                 10 as libc::c_int as s16b),
                         b"activating The Phial of Undeath\x00" as *const u8
                             as *const libc::c_char);
                dec_stat(3 as libc::c_int, 25 as libc::c_int,
                         3 as libc::c_int);
                dec_stat(2 as libc::c_int, 25 as libc::c_int,
                         3 as libc::c_int);
                dec_stat(4 as libc::c_int, 25 as libc::c_int,
                         3 as libc::c_int);
                dec_stat(0 as libc::c_int, 25 as libc::c_int,
                         3 as libc::c_int);
                dec_stat(5 as libc::c_int, 25 as libc::c_int,
                         3 as libc::c_int);
                dec_stat(1 as libc::c_int, 25 as libc::c_int,
                         3 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(10 as libc::c_int) + 10 as libc::c_int) as s16b
            }
            36 => {
                if doit == 0 {
                    return b"detection every 30+d30 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"The stone glows a deep green...\x00" as *const u8
                              as *const libc::c_char);
                detect_all(25 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(30 as libc::c_int) + 30 as libc::c_int) as s16b
            }
            37 => {
                if doit == 0 {
                    return b"dispel small life every 55+d55 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"You exterminate small life.\x00" as *const u8 as
                              *const libc::c_char);
                dispel_monsters(4 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(55 as libc::c_int) + 55 as libc::c_int) as s16b
            }
            38 => {
                if doit == 0 {
                    return b"haste self (75+d75 turns) every 150+d150 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"The ring glows brightly...\x00" as *const u8 as
                              *const libc::c_char);
                if (*p_ptr).fast == 0 {
                    set_fast(Rand_div(75 as libc::c_int) + 1 as libc::c_int +
                                 75 as libc::c_int, 10 as libc::c_int);
                } else {
                    set_fast((*p_ptr).fast as libc::c_int + 5 as libc::c_int,
                             10 as libc::c_int);
                }
                (*o_ptr).timeout =
                    (Rand_div(150 as libc::c_int) + 150 as libc::c_int) as
                        s16b
            }
            39 => {
                if doit == 0 {
                    return b"healing (500) every 200+d100 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"The ring glows deep red...\x00" as *const u8 as
                              *const libc::c_char);
                hp_player(500 as libc::c_int);
                set_blind(0 as libc::c_int);
                set_confused(0 as libc::c_int);
                set_poisoned(0 as libc::c_int);
                set_stun(0 as libc::c_int);
                set_cut(0 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(100 as libc::c_int) + 200 as libc::c_int) as
                        s16b
            }
            40 => {
                if doit == 0 {
                    return b"healing (800) every 100+d200 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"The ring glows bright white...\x00" as *const u8
                              as *const libc::c_char);
                hp_player(800 as libc::c_int);
                set_blind(0 as libc::c_int);
                set_confused(0 as libc::c_int);
                set_poisoned(0 as libc::c_int);
                set_stun(0 as libc::c_int);
                set_cut(0 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(200 as libc::c_int) + 100 as libc::c_int) as
                        s16b
            }
            41 => {
                if doit == 0 {
                    return b"greater healing (900) every 200+d200 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"The ring glows deep blue...\x00" as *const u8 as
                              *const libc::c_char);
                hp_player(900 as libc::c_int);
                set_blind(0 as libc::c_int);
                set_confused(0 as libc::c_int);
                set_poisoned(0 as libc::c_int);
                set_stun(0 as libc::c_int);
                set_cut(0 as libc::c_int);
                if (*p_ptr).black_breath != 0 {
                    (*p_ptr).black_breath = 0 as libc::c_int as bool_;
                    msg_print(b"The hold of the Black Breath on you is broken!\x00"
                                  as *const u8 as *const libc::c_char);
                }
                (*o_ptr).timeout =
                    (Rand_div(200 as libc::c_int) + 200 as libc::c_int) as
                        s16b
            }
            42 => {
                if doit == 0 {
                    return b"powerful things\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"The ring glows intensely black...\x00" as
                              *const u8 as *const libc::c_char);
                (*o_ptr).timeout = ring_of_power() as s16b
            }
            43 => {
                /* The Stone of Lore is perilous, for the sake of game balance. */
                if doit == 0 {
                    return b"perilous identify every turn\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"The stone reveals hidden mysteries...\x00" as
                              *const u8 as *const libc::c_char);
                if !(ident_spell() == 0) {
                    if has_ability(2 as libc::c_int) != 0 {
                        /* Sufficient mana */
                        if 20 as libc::c_int <= (*p_ptr).csp as libc::c_int {
                            /* Use some mana */
                            (*p_ptr).csp =
                                ((*p_ptr).csp as libc::c_int -
                                     20 as libc::c_int) as s16b
                        } else {
                            /* Over-exert the player */
                            let mut oops: libc::c_int =
                                20 as libc::c_int -
                                    (*p_ptr).csp as libc::c_int;
                            /* No mana left */
                            (*p_ptr).csp = 0 as libc::c_int as s16b;
                            (*p_ptr).csp_frac = 0 as libc::c_int as u16b;
                            /* Message */
                            msg_print(b"You are too weak to control the stone!\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            /* Hack -- Bypass free action */
                            set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                              (Rand_div(5 as libc::c_int *
                                                            oops +
                                                            1 as libc::c_int)
                                                   + 1 as libc::c_int));
                            /* Confusing. */
                            set_confused((*p_ptr).confused as libc::c_int +
                                             (Rand_div(5 as libc::c_int * oops
                                                           + 1 as libc::c_int)
                                                  + 1 as libc::c_int));
                        }
                        /* Redraw mana */
                        (*p_ptr).redraw =
                            ((*p_ptr).redraw as libc::c_long |
                                 0x80 as libc::c_long) as u32b
                    }
                    take_hit(damroll(1 as libc::c_int as s16b,
                                     12 as libc::c_int as s16b),
                             b"perilous secrets\x00" as *const u8 as
                                 *const libc::c_char);
                    /* Confusing. */
                    if Rand_div(5 as libc::c_int) == 0 as libc::c_int {
                        set_confused((*p_ptr).confused as libc::c_int +
                                         (Rand_div(10 as libc::c_int) +
                                              1 as libc::c_int));
                    }
                    /* Exercise a little care... */
                    if Rand_div(20 as libc::c_int) == 0 as libc::c_int {
                        take_hit(damroll(4 as libc::c_int as s16b,
                                         10 as libc::c_int as s16b),
                                 b"perilous secrets\x00" as *const u8 as
                                     *const libc::c_char);
                    }
                    (*o_ptr).timeout = 1 as libc::c_int as s16b
                }
            }
            44 => {
                if doit == 0 {
                    return b"star ball (150) every 1000 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your armor is surrounded by lightning...\x00" as
                              *const u8 as *const libc::c_char);
                i = 0 as libc::c_int;
                while i < 8 as libc::c_int {
                    fire_ball(1 as libc::c_int,
                              ddd[i as usize] as libc::c_int,
                              150 as libc::c_int, 3 as libc::c_int);
                    i += 1
                }
                (*o_ptr).timeout = 1000 as libc::c_int as s16b
            }
            45 => {
                if doit == 0 {
                    return b"invulnerability (4+d8) every 800 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                set_invuln((*p_ptr).invuln as libc::c_int +
                               (Rand_div(8 as libc::c_int) + 1 as libc::c_int)
                               + 4 as libc::c_int);
                (*o_ptr).timeout = 800 as libc::c_int as s16b
            }
            46 => {
                if doit == 0 {
                    return b"breathe elements (300), berserk rage, bless, and resistance every 400 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe the elements.\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(10 as libc::c_int, dir, 300 as libc::c_int,
                              4 as libc::c_int);
                    msg_print(b"Your armor glows many colours...\x00" as
                                  *const u8 as *const libc::c_char);
                    set_afraid(0 as libc::c_int);
                    set_shero((*p_ptr).shero as libc::c_int +
                                  (Rand_div(50 as libc::c_int) +
                                       1 as libc::c_int) + 50 as libc::c_int);
                    hp_player(30 as libc::c_int);
                    set_blessed((*p_ptr).blessed as libc::c_int +
                                    (Rand_div(50 as libc::c_int) +
                                         1 as libc::c_int) +
                                    50 as libc::c_int);
                    set_oppose_acid((*p_ptr).oppose_acid as libc::c_int +
                                        (Rand_div(50 as libc::c_int) +
                                             1 as libc::c_int) +
                                        50 as libc::c_int);
                    set_oppose_elec((*p_ptr).oppose_elec as libc::c_int +
                                        (Rand_div(50 as libc::c_int) +
                                             1 as libc::c_int) +
                                        50 as libc::c_int);
                    set_oppose_fire((*p_ptr).oppose_fire as libc::c_int +
                                        (Rand_div(50 as libc::c_int) +
                                             1 as libc::c_int) +
                                        50 as libc::c_int);
                    set_oppose_cold((*p_ptr).oppose_cold as libc::c_int +
                                        (Rand_div(50 as libc::c_int) +
                                             1 as libc::c_int) +
                                        50 as libc::c_int);
                    set_oppose_pois((*p_ptr).oppose_pois as libc::c_int +
                                        (Rand_div(50 as libc::c_int) +
                                             1 as libc::c_int) +
                                        50 as libc::c_int);
                    (*o_ptr).timeout = 400 as libc::c_int as s16b
                }
            }
            47 => {
                if doit == 0 {
                    return b"heal (777), curing and heroism every 300 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"A heavenly choir sings...\x00" as *const u8 as
                              *const libc::c_char);
                set_poisoned(0 as libc::c_int);
                set_cut(0 as libc::c_int);
                set_stun(0 as libc::c_int);
                set_confused(0 as libc::c_int);
                set_blind(0 as libc::c_int);
                set_hero((*p_ptr).hero as libc::c_int +
                             (Rand_div(25 as libc::c_int) + 1 as libc::c_int)
                             + 25 as libc::c_int);
                hp_player(777 as libc::c_int);
                (*o_ptr).timeout = 300 as libc::c_int as s16b
            }
            48 => {
                if doit == 0 {
                    return b"rays of fear in every direction\x00" as *const u8
                               as *const libc::c_char
                }
                turn_monsters(40 as libc::c_int +
                                  (*p_ptr).lev as libc::c_int);
                (*o_ptr).timeout =
                    (3 as libc::c_int *
                         ((*p_ptr).lev as libc::c_int + 10 as libc::c_int)) as
                        s16b
            }
            49 => {
                if doit == 0 {
                    return b"resistance (20+d20 turns) every 111 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your cloak glows many colours...\x00" as *const u8
                              as *const libc::c_char);
                set_oppose_acid((*p_ptr).oppose_acid as libc::c_int +
                                    (Rand_div(20 as libc::c_int) +
                                         1 as libc::c_int) +
                                    20 as libc::c_int);
                set_oppose_elec((*p_ptr).oppose_elec as libc::c_int +
                                    (Rand_div(20 as libc::c_int) +
                                         1 as libc::c_int) +
                                    20 as libc::c_int);
                set_oppose_fire((*p_ptr).oppose_fire as libc::c_int +
                                    (Rand_div(20 as libc::c_int) +
                                         1 as libc::c_int) +
                                    20 as libc::c_int);
                set_oppose_cold((*p_ptr).oppose_cold as libc::c_int +
                                    (Rand_div(20 as libc::c_int) +
                                         1 as libc::c_int) +
                                    20 as libc::c_int);
                set_oppose_pois((*p_ptr).oppose_pois as libc::c_int +
                                    (Rand_div(20 as libc::c_int) +
                                         1 as libc::c_int) +
                                    20 as libc::c_int);
                (*o_ptr).timeout = 111 as libc::c_int as s16b
            }
            50 => {
                if doit == 0 {
                    return b"frost ball (48) every 5+d5 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your dagger is covered in frost...\x00" as
                              *const u8 as *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(4 as libc::c_int, dir, 48 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(5 as libc::c_int) + 5 as libc::c_int) as
                            s16b
                }
            }
            59 => {
                if doit == 0 {
                    return b"a getaway every 35 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                match Rand_div(13 as libc::c_int) + 1 as libc::c_int {
                    1 | 2 | 3 | 4 | 5 => {
                        teleport_player(10 as libc::c_int);
                    }
                    6 | 7 | 8 | 9 | 10 => {
                        teleport_player(222 as libc::c_int);
                    }
                    11 | 12 => { stair_creation(); }
                    _ => {
                        if get_check(b"Leave this level? \x00" as *const u8 as
                                         *const libc::c_char) != 0 {
                            autosave_checkpoint();
                            /* Leaving */
                            (*p_ptr).leaving = 1 as libc::c_int as bool_
                        }
                    }
                }
                (*o_ptr).timeout = 35 as libc::c_int as s16b
            }
            60 => {
                if doit == 0 {
                    return b"healing(7000), curing every 500 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your sword glows an intense white...\x00" as
                              *const u8 as *const libc::c_char);
                hp_player(7000 as libc::c_int);
                heal_insanity(50 as libc::c_int);
                set_blind(0 as libc::c_int);
                set_poisoned(0 as libc::c_int);
                set_confused(0 as libc::c_int);
                set_stun(0 as libc::c_int);
                set_cut(0 as libc::c_int);
                set_image(0 as libc::c_int);
                (*o_ptr).timeout = 500 as libc::c_int as s16b
            }
            61 => {
                if doit == 0 {
                    return b"summon the Legion of the Dawn every 500+d500 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"You summon the Legion of the Dawn.\x00" as
                              *const u8 as *const libc::c_char);
                summon_specific_friendly((*p_ptr).py as libc::c_int,
                                         (*p_ptr).px as libc::c_int,
                                         dun_level as libc::c_int,
                                         41 as libc::c_int,
                                         1 as libc::c_int as bool_);
                (*o_ptr).timeout =
                    (500 as libc::c_int +
                         (Rand_div(500 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            62 => {
                if doit == 0 {
                    return b"large fire ball (72) every 100 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your morning star rages in fire...\x00" as
                              *const u8 as *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(5 as libc::c_int, dir, 72 as libc::c_int,
                              3 as libc::c_int);
                    (*o_ptr).timeout = 100 as libc::c_int as s16b
                }
            }
            63 => {
                if doit == 0 {
                    return b"drain life (90) every 70 turns\x00" as *const u8
                               as *const libc::c_char
                }
                msg_print(b"Your hammer glows white...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    drain_life(dir, 90 as libc::c_int);
                    (*o_ptr).timeout = 70 as libc::c_int as s16b
                }
            }
            64 => {
                if doit == 0 {
                    return b"fire branding of bolts every 999 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your crossbow glows deep red...\x00" as *const u8
                              as *const libc::c_char);
                brand_bolts();
                (*o_ptr).timeout = 999 as libc::c_int as s16b
            }
            75 => {
                if doit == 0 {
                    return b"heal and cure black breath every 200 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if (*p_ptr).black_breath != 0 {
                    msg_print(b"The hold of the Black Breath on you is broken!\x00"
                                  as *const u8 as *const libc::c_char);
                }
                (*p_ptr).black_breath = 0 as libc::c_int as bool_;
                hp_player(100 as libc::c_int);
                (*o_ptr).timeout = 200 as libc::c_int as s16b
            }
            76 => {
                if doit == 0 {
                    return b"restore mana every 666 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"Your mage staff glows deep blue...\x00" as
                              *const u8 as *const libc::c_char);
                if ((*p_ptr).csp as libc::c_int) < (*p_ptr).msp as libc::c_int
                   {
                    (*p_ptr).csp = (*p_ptr).msp;
                    (*p_ptr).csp_frac = 0 as libc::c_int as u16b;
                    msg_print(b"Your feel your head clear.\x00" as *const u8
                                  as *const libc::c_char);
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x80 as libc::c_long) as u32b;
                    (*p_ptr).window =
                        ((*p_ptr).window as libc::c_long |
                             0x8 as libc::c_long) as u32b
                }
                (*o_ptr).timeout = 666 as libc::c_int as s16b
            }
            77 => {
                if doit == 0 {
                    return b"summon a thunderlord every 1000 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    if summon_specific((*p_ptr).py as libc::c_int,
                                       (*p_ptr).px as libc::c_int,
                                       plev * 3 as libc::c_int /
                                           2 as libc::c_int,
                                       49 as libc::c_int) != 0 {
                        msg_print(b"A Thunderlord comes from thin air!\x00" as
                                      *const u8 as *const libc::c_char);
                        msg_print(b"\'I will burn you!\'\x00" as *const u8 as
                                      *const libc::c_char);
                    }
                } else if summon_specific_friendly((*p_ptr).py as libc::c_int,
                                                   (*p_ptr).px as libc::c_int,
                                                   plev * 3 as libc::c_int /
                                                       2 as libc::c_int,
                                                   49 as libc::c_int,
                                                   if plev ==
                                                          50 as libc::c_int {
                                                       1 as libc::c_int
                                                   } else { 0 as libc::c_int }
                                                       as bool_) != 0 {
                    msg_print(b"A Thunderlord comes from thin air!\x00" as
                                  *const u8 as *const libc::c_char);
                    msg_print(b"\'I will help you in your difficult task.\'\x00"
                                  as *const u8 as *const libc::c_char);
                }
                (*o_ptr).timeout = 1000 as libc::c_int as s16b
            }
            78 => {
                if doit == 0 {
                    return b"clairvoyance every 100+d100 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"The stone glows a deep green...\x00" as *const u8
                              as *const libc::c_char);
                wiz_lite_extra();
                detect_traps(25 as libc::c_int);
                detect_doors(25 as libc::c_int);
                detect_stairs(25 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(100 as libc::c_int) + 100 as libc::c_int) as
                        s16b
            }
            89 => {
                if doit == 0 {
                    return b"open a secret passage every 75 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your pick twists in your hands.\x00" as *const u8
                              as *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    if passwall(dir, 1 as libc::c_int as bool_) != 0 {
                        msg_print(b"A passage opens, and you step through.\x00"
                                      as *const u8 as *const libc::c_char);
                    } else {
                        msg_print(b"There is no wall there!\x00" as *const u8
                                      as *const libc::c_char);
                    }
                    (*o_ptr).timeout = 75 as libc::c_int as s16b
                }
            }
            90 => {
                if doit == 0 {
                    return b"detection every 99 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"Your drum shows you the world.\x00" as *const u8
                              as *const libc::c_char);
                detect_all(25 as libc::c_int);
                (*o_ptr).timeout = 99 as libc::c_int as s16b
            }
            99 => {
                if doit == 0 {
                    return b"heroism, berserker, and haste every 250 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"Your horn glows deep red.\x00" as *const u8 as
                              *const libc::c_char);
                set_afraid(0 as libc::c_int);
                set_shero((*p_ptr).shero as libc::c_int +
                              damroll(5 as libc::c_int as s16b,
                                      10 as libc::c_int as s16b) +
                              30 as libc::c_int);
                set_afraid(0 as libc::c_int);
                set_hero((*p_ptr).hero as libc::c_int +
                             damroll(5 as libc::c_int as s16b,
                                     10 as libc::c_int as s16b) +
                             30 as libc::c_int);
                set_fast((*p_ptr).fast as libc::c_int +
                             damroll(5 as libc::c_int as s16b,
                                     10 as libc::c_int as s16b) +
                             30 as libc::c_int, 10 as libc::c_int);
                hp_player(30 as libc::c_int);
                (*o_ptr).timeout = 250 as libc::c_int as s16b
            }
            100 => {
                if doit == 0 {
                    return b"sound ball (300) every 300 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your horn emits a loud sound.\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(21 as libc::c_int, dir, 300 as libc::c_int,
                              6 as libc::c_int);
                    (*o_ptr).timeout = 300 as libc::c_int as s16b
                }
            }
            101 => {
                if doit == 0 {
                    return b"mass human summoning every 1000 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your horn calls for help.\x00" as *const u8 as
                              *const libc::c_char);
                i = 0 as libc::c_int;
                while i < 15 as libc::c_int {
                    summon_specific_friendly((*p_ptr).py as libc::c_int,
                                             (*p_ptr).px as libc::c_int,
                                             plev * 3 as libc::c_int /
                                                 2 as libc::c_int,
                                             54 as libc::c_int,
                                             1 as libc::c_int as bool_);
                    i += 1
                }
                (*o_ptr).timeout = 1000 as libc::c_int as s16b
            }
            102 => {
                if doit == 0 {
                    return b"berserker and +10 to speed (50) every 100+d200 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if (*p_ptr).fast == 0 {
                    set_fast(Rand_div(50 as libc::c_int) + 1 as libc::c_int +
                                 50 as libc::c_int, 10 as libc::c_int);
                } else {
                    set_fast((*p_ptr).fast as libc::c_int + 5 as libc::c_int,
                             10 as libc::c_int);
                }
                hp_player(30 as libc::c_int);
                set_afraid(0 as libc::c_int);
                set_shero((*p_ptr).shero as libc::c_int +
                              (Rand_div(50 as libc::c_int) + 1 as libc::c_int)
                              + 50 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(200 as libc::c_int) + 100 as libc::c_int) as
                        s16b
            }
            103 => {
                if doit == 0 {
                    return b"fire ball (300) every 200+d200 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your lochaber axe erupts in fire...\x00" as
                              *const u8 as *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(5 as libc::c_int, dir, 300 as libc::c_int,
                              4 as libc::c_int);
                    (*o_ptr).timeout =
                        (200 as libc::c_int + Rand_div(200 as libc::c_int)) as
                            s16b
                }
            }
            104 => {
                if doit == 0 {
                    return b"darkness ball (150) every 100 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your spear is covered of darkness...\x00" as
                              *const u8 as *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(16 as libc::c_int, dir, 150 as libc::c_int,
                              3 as libc::c_int);
                    (*o_ptr).timeout = 100 as libc::c_int as s16b
                }
            }
            105 => {
                if doit == 0 {
                    return b"alter reality every 100 turns\x00" as *const u8
                               as *const libc::c_char
                }
                msg_print(b"Your hammer hits the floor...\x00" as *const u8 as
                              *const libc::c_char);
                alter_reality();
                (*o_ptr).timeout = 100 as libc::c_int as s16b
            }
            106 => {
                if doit == 0 {
                    return b"dispel monsters (300) every 200+d200 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"Your axe glows blood red...\x00" as *const u8 as
                              *const libc::c_char);
                dispel_monsters(300 as libc::c_int);
                (*o_ptr).timeout =
                    (200 as libc::c_int +
                         (Rand_div(200 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            107 => {
                if doit == 0 {
                    return b"vampiric drain (3*100) every 250 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"Your axe emits a black aura...\x00" as *const u8
                              as *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    i = 0 as libc::c_int;
                    while i < 3 as libc::c_int {
                        if drain_life(dir, 100 as libc::c_int) != 0 {
                            hp_player(100 as libc::c_int);
                        }
                        i += 1
                    }
                    (*o_ptr).timeout = 250 as libc::c_int as s16b
                }
            }
            108 => {
                if doit == 0 {
                    return b"detect orcs every 10 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"Your weapon glows brightly...\x00" as *const u8 as
                              *const libc::c_char);
                detect_monsters_xxx(0x1 as libc::c_int as u32b,
                                    25 as libc::c_int);
                (*o_ptr).timeout = 10 as libc::c_int as s16b
            }
            1 => {
                if doit == 0 {
                    return b"beam of sunlight every 10 turns\x00" as *const u8
                               as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"A line of sunlight appears.\x00" as *const u8
                                  as *const libc::c_char);
                    lite_line(dir);
                    (*o_ptr).timeout = 10 as libc::c_int as s16b
                }
            }
            2 => {
                if doit == 0 {
                    return b"magic missile (2d6) every 2 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows extremely brightly...\x00" as *const u8
                              as *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_bolt(10 as libc::c_int, dir,
                              damroll(2 as libc::c_int as s16b,
                                      6 as libc::c_int as s16b));
                    (*o_ptr).timeout = 2 as libc::c_int as s16b
                }
            }
            3 => {
                if doit == 0 {
                    return b"stinking cloud (12), rad. 3, every 4+d4 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"It throbs deep green...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(2 as libc::c_int, dir, 12 as libc::c_int,
                              3 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(4 as libc::c_int) + 4 as libc::c_int) as
                            s16b
                }
            }
            4 => {
                if doit == 0 {
                    return b"lightning bolt (4d8) every 6+d6 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It is covered in sparks...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_bolt(1 as libc::c_int, dir,
                              damroll(4 as libc::c_int as s16b,
                                      8 as libc::c_int as s16b));
                    (*o_ptr).timeout =
                        (Rand_div(6 as libc::c_int) + 6 as libc::c_int) as
                            s16b
                }
            }
            5 => {
                if doit == 0 {
                    return b"acid bolt (5d8) every 5+d5 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It is covered in acid...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_bolt(3 as libc::c_int, dir,
                              damroll(5 as libc::c_int as s16b,
                                      8 as libc::c_int as s16b));
                    (*o_ptr).timeout =
                        (Rand_div(5 as libc::c_int) + 5 as libc::c_int) as
                            s16b
                }
            }
            6 => {
                if doit == 0 {
                    return b"frost bolt (6d8) every 7+d7 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It is covered in frost...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_bolt(4 as libc::c_int, dir,
                              damroll(6 as libc::c_int as s16b,
                                      8 as libc::c_int as s16b));
                    (*o_ptr).timeout =
                        (Rand_div(7 as libc::c_int) + 7 as libc::c_int) as
                            s16b
                }
            }
            7 => {
                if doit == 0 {
                    return b"fire bolt (9d8) every 8+d8 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It is covered in fire...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_bolt(5 as libc::c_int, dir,
                              damroll(9 as libc::c_int as s16b,
                                      8 as libc::c_int as s16b));
                    (*o_ptr).timeout =
                        (Rand_div(8 as libc::c_int) + 8 as libc::c_int) as
                            s16b
                }
            }
            8 => {
                if doit == 0 {
                    return b"ball of cold (48) every 400 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It is covered in frost...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(4 as libc::c_int, dir, 48 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout = 400 as libc::c_int as s16b
                }
            }
            9 => {
                if doit == 0 {
                    return b"ball of fire (72) every 400 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows an intense red...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(5 as libc::c_int, dir, 72 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout = 400 as libc::c_int as s16b
                }
            }
            10 => {
                if doit == 0 {
                    return b"drain life (100) every 100+d100 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows black...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    if drain_life(dir, 100 as libc::c_int) != 0 {
                        (*o_ptr).timeout =
                            (Rand_div(100 as libc::c_int) +
                                 100 as libc::c_int) as s16b
                    }
                }
            }
            11 => {
                if doit == 0 {
                    return b"ball of cold (100) every 300 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows an intense blue...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(4 as libc::c_int, dir, 100 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout = 300 as libc::c_int as s16b
                }
            }
            12 => {
                if doit == 0 {
                    return b"ball of lightning (100) every 500 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It crackles with electricity...\x00" as *const u8
                              as *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(1 as libc::c_int, dir, 100 as libc::c_int,
                              3 as libc::c_int);
                    (*o_ptr).timeout = 500 as libc::c_int as s16b
                }
            }
            13 => {
                if doit == 0 {
                    return b"drain life (120) every 400 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows black...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    drain_life(dir, 120 as libc::c_int);
                    (*o_ptr).timeout = 400 as libc::c_int as s16b
                }
            }
            14 => {
                if doit == 0 {
                    return b"vampiric drain (3*50) every 400 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    dummy = 0 as libc::c_int;
                    while dummy < 3 as libc::c_int {
                        if drain_life(dir, 50 as libc::c_int) != 0 {
                            hp_player(50 as libc::c_int);
                        }
                        dummy += 1
                    }
                    (*o_ptr).timeout = 400 as libc::c_int as s16b
                }
            }
            15 => {
                if doit == 0 {
                    return b"arrows (150) every 90+d90 turns\x00" as *const u8
                               as *const libc::c_char
                }
                msg_print(b"It grows magical spikes...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_bolt(11 as libc::c_int, dir, 150 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 90 as libc::c_int) as
                            s16b
                }
            }
            16 => {
                if doit == 0 {
                    return b"fire ball (120) every 225+d225 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows deep red...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(5 as libc::c_int, dir, 120 as libc::c_int,
                              3 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(225 as libc::c_int) + 225 as libc::c_int) as
                            s16b
                }
            }
            17 => {
                if doit == 0 {
                    return b"ball of cold (200) every 325+d325 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows bright white...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(4 as libc::c_int, dir, 200 as libc::c_int,
                              3 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(325 as libc::c_int) + 325 as libc::c_int) as
                            s16b
                }
            }
            18 => {
                if doit == 0 {
                    return b"Lightning Ball (250) every 425+d425 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows deep blue...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(1 as libc::c_int, dir, 250 as libc::c_int,
                              3 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(425 as libc::c_int) + 425 as libc::c_int) as
                            s16b
                }
            }
            19 => {
                let mut y_0: libc::c_int = 0 as libc::c_int;
                let mut x_0: libc::c_int = 0 as libc::c_int;
                let mut c_ptr_0: *mut cave_type = 0 as *mut cave_type;
                let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
                if doit == 0 {
                    return b"whirlwind attack every 250 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                dir = 0 as libc::c_int;
                while dir <= 9 as libc::c_int {
                    y_0 =
                        (*p_ptr).py as libc::c_int +
                            ddy[dir as usize] as libc::c_int;
                    x_0 =
                        (*p_ptr).px as libc::c_int +
                            ddx[dir as usize] as libc::c_int;
                    c_ptr_0 =
                        &mut *(*cave.as_mut_ptr().offset(y_0 as
                                                             isize)).offset(x_0
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Get the monster */
                    m_ptr =
                        &mut *m_list.offset((*c_ptr_0).m_idx as isize) as
                            *mut monster_type;
                    /* Hack -- attack monsters */
                    if (*c_ptr_0).m_idx as libc::c_int != 0 &&
                           ((*m_ptr).ml as libc::c_int != 0 ||
                                (*f_info.offset((*cave[y_0 as
                                                           usize].offset(x_0
                                                                             as
                                                                             isize)).feat
                                                    as isize)).flags1 as
                                    libc::c_long & 0x10 as libc::c_long != 0
                                    &&
                                    (*cave[y_0 as
                                               usize].offset(x_0 as
                                                                 isize)).feat
                                        as libc::c_int != 0xaf as libc::c_int)
                       {
                        py_attack(y_0, x_0, -(1 as libc::c_int));
                    }
                    dir += 1
                }
                (*o_ptr).timeout = 250 as libc::c_int as s16b
            }
            20 => {
                if doit == 0 {
                    return b"vampiric drain (3*100) every 400 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    dummy = 0 as libc::c_int;
                    while dummy < 3 as libc::c_int {
                        if drain_life(dir, 100 as libc::c_int) != 0 {
                            hp_player(100 as libc::c_int);
                        }
                        dummy += 1
                    }
                    (*o_ptr).timeout = 400 as libc::c_int as s16b
                }
            }
            21 => {
                if doit == 0 {
                    return b"call chaos every 350 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"It glows in scintillating colours...\x00" as
                              *const u8 as *const libc::c_char);
                call_chaos();
                (*o_ptr).timeout = 350 as libc::c_int as s16b
            }
            22 => {
                if doit == 0 {
                    return b"launch rocket (120+level) every 400 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You launch a rocket!\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(72 as libc::c_int, dir,
                              120 as libc::c_int + plev, 2 as libc::c_int);
                    (*o_ptr).timeout = 400 as libc::c_int as s16b
                }
            }
            23 => {
                if doit == 0 {
                    return b"dispel evil (level*5) every 300+d300 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"It floods the area with goodness...\x00" as
                              *const u8 as *const libc::c_char);
                dispel_evil((*p_ptr).lev as libc::c_int * 5 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(300 as libc::c_int) + 300 as libc::c_int) as
                        s16b
            }
            25 => {
                if doit == 0 {
                    return b"dispel good (level*5) every 300+d300 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"It floods the area with evil...\x00" as *const u8
                              as *const libc::c_char);
                dispel_good((*p_ptr).lev as libc::c_int * 5 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(300 as libc::c_int) + 300 as libc::c_int) as
                        s16b
            }
            24 => {
                if doit == 0 {
                    return b"elemental breath (300) every 500 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe the elements.\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(10 as libc::c_int, dir, 300 as libc::c_int,
                              4 as libc::c_int);
                    (*o_ptr).timeout = 500 as libc::c_int as s16b
                }
            }
            51 => {
                /* Activate for other offensive action */
                if doit == 0 {
                    return b"confuse monster every 15 turns\x00" as *const u8
                               as *const libc::c_char
                }
                msg_print(b"It glows in scintillating colours...\x00" as
                              *const u8 as *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    confuse_monster(dir, 20 as libc::c_int);
                    (*o_ptr).timeout = 15 as libc::c_int as s16b
                }
            }
            52 => {
                if doit == 0 {
                    return b"sleep nearby monsters every 55 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows deep blue...\x00" as *const u8 as
                              *const libc::c_char);
                sleep_monsters_touch();
                (*o_ptr).timeout = 55 as libc::c_int as s16b
            }
            53 => {
                if doit == 0 {
                    return b"earthquake (rad 10) every 50 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                /* Prevent destruction of quest levels and town */
                if is_quest(dun_level as libc::c_int) == 0 &&
                       dun_level as libc::c_int != 0 {
                    earthquake((*p_ptr).py as libc::c_int,
                               (*p_ptr).px as libc::c_int, 10 as libc::c_int);
                    (*o_ptr).timeout = 50 as libc::c_int as s16b
                }
            }
            54 => {
                if doit == 0 {
                    return b"terror every 3 * (level+10) turns\x00" as
                               *const u8 as *const libc::c_char
                }
                turn_monsters(40 as libc::c_int +
                                  (*p_ptr).lev as libc::c_int);
                (*o_ptr).timeout =
                    (3 as libc::c_int *
                         ((*p_ptr).lev as libc::c_int + 10 as libc::c_int)) as
                        s16b
            }
            55 => {
                if doit == 0 {
                    return b"teleport away every 200 turns\x00" as *const u8
                               as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_beam(63 as libc::c_int, dir, plev);
                    (*o_ptr).timeout = 200 as libc::c_int as s16b
                }
            }
            56 => {
                if doit == 0 {
                    return b"banish evil every 250+d250 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if banish_evil(100 as libc::c_int) != 0 {
                    msg_print(b"The power of the artifact banishes evil!\x00"
                                  as *const u8 as *const libc::c_char);
                }
                (*o_ptr).timeout =
                    (250 as libc::c_int +
                         (Rand_div(250 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            57 => {
                if doit == 0 {
                    return b"genocide every 500 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"It glows deep blue...\x00" as *const u8 as
                              *const libc::c_char);
                genocide(1 as libc::c_int as bool_);
                (*o_ptr).timeout = 500 as libc::c_int as s16b
            }
            58 => {
                if doit == 0 {
                    return b"mass genocide every 1000 turns\x00" as *const u8
                               as *const libc::c_char
                }
                msg_print(b"It lets out a long, shrill note...\x00" as
                              *const u8 as *const libc::c_char);
                mass_genocide(1 as libc::c_int as bool_);
                (*o_ptr).timeout = 1000 as libc::c_int as s16b
            }
            65 => {
                /* Activate for summoning / charming */
                if doit == 0 {
                    return b"charm animal every 300 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    charm_animal(dir, plev);
                    (*o_ptr).timeout = 300 as libc::c_int as s16b
                }
            }
            66 => {
                if doit == 0 {
                    return b"enslave undead every 333 turns\x00" as *const u8
                               as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    control_one_undead(dir, plev);
                    (*o_ptr).timeout = 333 as libc::c_int as s16b
                }
            }
            67 => {
                if doit == 0 {
                    return b"charm monster every 400 turns\x00" as *const u8
                               as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    charm_monster(dir, plev);
                    (*o_ptr).timeout = 400 as libc::c_int as s16b
                }
            }
            68 => {
                if doit == 0 {
                    return b"animal friendship every 500 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                charm_animals(plev * 2 as libc::c_int);
                (*o_ptr).timeout = 500 as libc::c_int as s16b
            }
            69 => {
                if doit == 0 {
                    return b"mass charm every 750 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                charm_monsters(plev * 2 as libc::c_int);
                (*o_ptr).timeout = 750 as libc::c_int as s16b
            }
            70 => {
                if doit == 0 {
                    return b"summon animal every 200+d300 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                summon_specific_friendly((*p_ptr).py as libc::c_int,
                                         (*p_ptr).px as libc::c_int, plev,
                                         43 as libc::c_int,
                                         1 as libc::c_int as bool_);
                (*o_ptr).timeout =
                    (200 as libc::c_int +
                         (Rand_div(300 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            71 => {
                if doit == 0 {
                    return b"summon phantasmal servant every 200+d200 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"You summon a phantasmal servant.\x00" as *const u8
                              as *const libc::c_char);
                summon_specific_friendly((*p_ptr).py as libc::c_int,
                                         (*p_ptr).px as libc::c_int,
                                         dun_level as libc::c_int,
                                         47 as libc::c_int,
                                         1 as libc::c_int as bool_);
                (*o_ptr).timeout =
                    (200 as libc::c_int +
                         (Rand_div(200 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            72 => {
                if doit == 0 {
                    return b"summon elemental every 750 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    if summon_specific((*p_ptr).py as libc::c_int,
                                       (*p_ptr).px as libc::c_int,
                                       plev * 3 as libc::c_int /
                                           2 as libc::c_int,
                                       48 as libc::c_int) != 0 {
                        msg_print(b"An elemental materialises...\x00" as
                                      *const u8 as *const libc::c_char);
                        msg_print(b"You fail to control it!\x00" as *const u8
                                      as *const libc::c_char);
                    }
                } else if summon_specific_friendly((*p_ptr).py as libc::c_int,
                                                   (*p_ptr).px as libc::c_int,
                                                   plev * 3 as libc::c_int /
                                                       2 as libc::c_int,
                                                   48 as libc::c_int,
                                                   if plev ==
                                                          50 as libc::c_int {
                                                       1 as libc::c_int
                                                   } else { 0 as libc::c_int }
                                                       as bool_) != 0 {
                    msg_print(b"An elemental materialises...\x00" as *const u8
                                  as *const libc::c_char);
                    msg_print(b"It seems obedient to you.\x00" as *const u8 as
                                  *const libc::c_char);
                }
                (*o_ptr).timeout = 750 as libc::c_int as s16b
            }
            73 => {
                if doit == 0 {
                    return b"summon demon every 666+d333 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    if summon_specific((*p_ptr).py as libc::c_int,
                                       (*p_ptr).px as libc::c_int,
                                       plev * 3 as libc::c_int /
                                           2 as libc::c_int,
                                       16 as libc::c_int) != 0 {
                        msg_print(b"The area fills with a stench of sulphur and brimstone.\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(b"\'NON SERVIAM! Wretch! I shall feast on thy mortal soul!\'\x00"
                                      as *const u8 as *const libc::c_char);
                    }
                } else if summon_specific_friendly((*p_ptr).py as libc::c_int,
                                                   (*p_ptr).px as libc::c_int,
                                                   plev * 3 as libc::c_int /
                                                       2 as libc::c_int,
                                                   16 as libc::c_int,
                                                   if plev ==
                                                          50 as libc::c_int {
                                                       1 as libc::c_int
                                                   } else { 0 as libc::c_int }
                                                       as bool_) != 0 {
                    msg_print(b"The area fills with a stench of sulphur and brimstone.\x00"
                                  as *const u8 as *const libc::c_char);
                    msg_print(b"\'What is thy bidding... Master?\'\x00" as
                                  *const u8 as *const libc::c_char);
                }
                (*o_ptr).timeout =
                    (666 as libc::c_int +
                         (Rand_div(333 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            74 => {
                if doit == 0 {
                    return b"summon undead every 666+d333 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    if summon_specific((*p_ptr).py as libc::c_int,
                                       (*p_ptr).px as libc::c_int,
                                       plev * 3 as libc::c_int /
                                           2 as libc::c_int,
                                       if plev > 47 as libc::c_int {
                                           21 as libc::c_int
                                       } else { 17 as libc::c_int }) != 0 {
                        msg_print(b"Cold winds begin to blow around you, carrying with them the stench of decay...\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(b"\'The dead arise... to punish you for disturbing them!\'\x00"
                                      as *const u8 as *const libc::c_char);
                    }
                } else if summon_specific_friendly((*p_ptr).py as libc::c_int,
                                                   (*p_ptr).px as libc::c_int,
                                                   plev * 3 as libc::c_int /
                                                       2 as libc::c_int,
                                                   if plev > 47 as libc::c_int
                                                      {
                                                       44 as libc::c_int
                                                   } else {
                                                       17 as libc::c_int
                                                   },
                                                   if plev > 24 as libc::c_int
                                                          &&
                                                          Rand_div(3 as
                                                                       libc::c_int)
                                                              +
                                                              1 as libc::c_int
                                                              ==
                                                              1 as libc::c_int
                                                      {
                                                       1 as libc::c_int
                                                   } else { 0 as libc::c_int }
                                                       as bool_) != 0 {
                    msg_print(b"Cold winds begin to blow around you, carrying with them the stench of decay...\x00"
                                  as *const u8 as *const libc::c_char);
                    msg_print(b"Ancient, long-dead forms arise from the ground to serve you!\x00"
                                  as *const u8 as *const libc::c_char);
                }
                (*o_ptr).timeout =
                    (666 as libc::c_int +
                         (Rand_div(333 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            81 => {
                /* Activate for healing */
                if doit == 0 {
                    return format(b"cure light wounds every %d turns\x00" as
                                      *const u8 as *const libc::c_char,
                                  if is_junkart as libc::c_int != 0 {
                                      50 as libc::c_int
                                  } else { 10 as libc::c_int })
                }
                set_afraid(0 as libc::c_int);
                hp_player(30 as libc::c_int);
                (*o_ptr).timeout = 10 as libc::c_int as s16b
            }
            82 => {
                if doit == 0 {
                    return format(b"cure serious wounds every %s turns\x00" as
                                      *const u8 as *const libc::c_char,
                                  if is_junkart as libc::c_int != 0 {
                                      b"75\x00" as *const u8 as
                                          *const libc::c_char
                                  } else {
                                      b"3+d3\x00" as *const u8 as
                                          *const libc::c_char
                                  })
                }
                msg_print(b"It radiates deep purple...\x00" as *const u8 as
                              *const libc::c_char);
                hp_player(damroll(4 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b));
                set_cut((*p_ptr).cut as libc::c_int / 2 as libc::c_int -
                            50 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(3 as libc::c_int) + 3 as libc::c_int) as s16b
            }
            83 => {
                if doit == 0 {
                    return b"remove fear and cure poison every 5 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows deep blue...\x00" as *const u8 as
                              *const libc::c_char);
                set_afraid(0 as libc::c_int);
                set_poisoned(0 as libc::c_int);
                (*o_ptr).timeout = 5 as libc::c_int as s16b
            }
            84 => {
                if doit == 0 {
                    return b"restore life levels every 450 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows a deep red...\x00" as *const u8 as
                              *const libc::c_char);
                restore_level();
                (*o_ptr).timeout = 450 as libc::c_int as s16b
            }
            85 => {
                if doit == 0 {
                    return format(b"restore stats and life levels every %d turns\x00"
                                      as *const u8 as *const libc::c_char,
                                  if is_junkart as libc::c_int != 0 {
                                      200 as libc::c_int
                                  } else { 750 as libc::c_int })
                }
                msg_print(b"It glows a deep green...\x00" as *const u8 as
                              *const libc::c_char);
                do_res_stat(0 as libc::c_int, 1 as libc::c_int as bool_);
                do_res_stat(1 as libc::c_int, 1 as libc::c_int as bool_);
                do_res_stat(2 as libc::c_int, 1 as libc::c_int as bool_);
                do_res_stat(3 as libc::c_int, 1 as libc::c_int as bool_);
                do_res_stat(4 as libc::c_int, 1 as libc::c_int as bool_);
                do_res_stat(5 as libc::c_int, 1 as libc::c_int as bool_);
                restore_level();
                (*o_ptr).timeout = 750 as libc::c_int as s16b
            }
            86 => {
                if doit == 0 {
                    return format(b"heal 700 hit points every %d turns\x00" as
                                      *const u8 as *const libc::c_char,
                                  if is_junkart as libc::c_int != 0 {
                                      100 as libc::c_int
                                  } else { 250 as libc::c_int })
                }
                msg_print(b"It glows deep blue...\x00" as *const u8 as
                              *const libc::c_char);
                msg_print(b"You feel a warm tingling inside...\x00" as
                              *const u8 as *const libc::c_char);
                hp_player(700 as libc::c_int);
                set_cut(0 as libc::c_int);
                (*o_ptr).timeout = 250 as libc::c_int as s16b
            }
            87 => {
                if doit == 0 {
                    return b"heal 1000 hit points every 888 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows a bright white...\x00" as *const u8 as
                              *const libc::c_char);
                msg_print(b"You feel much better...\x00" as *const u8 as
                              *const libc::c_char);
                hp_player(1000 as libc::c_int);
                set_cut(0 as libc::c_int);
                (*o_ptr).timeout = 888 as libc::c_int as s16b
            }
            91 => {
                if doit == 0 {
                    return b"temporary ESP (dur 25+d30) every 200 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                set_tim_esp((*p_ptr).tim_esp as libc::c_int +
                                (Rand_div(30 as libc::c_int) +
                                     1 as libc::c_int) + 25 as libc::c_int);
                (*o_ptr).timeout = 200 as libc::c_int as s16b
            }
            92 => {
                if doit == 0 {
                    return b"heroism and berserk (dur 50+d50) every 100+d100 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                set_shero((*p_ptr).shero as libc::c_int +
                              (Rand_div(50 as libc::c_int) + 1 as libc::c_int)
                              + 50 as libc::c_int);
                set_blessed((*p_ptr).blessed as libc::c_int +
                                (Rand_div(50 as libc::c_int) +
                                     1 as libc::c_int) + 50 as libc::c_int);
                (*o_ptr).timeout =
                    (100 as libc::c_int +
                         (Rand_div(100 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            93 => {
                if doit == 0 {
                    return b"protection from evil (dur level*3 + d25) every 225+d225 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"It lets out a shrill wail...\x00" as *const u8 as
                              *const libc::c_char);
                k = 3 as libc::c_int * (*p_ptr).lev as libc::c_int;
                set_protevil((*p_ptr).protevil as libc::c_int +
                                 (Rand_div(25 as libc::c_int) +
                                      1 as libc::c_int) + k);
                (*o_ptr).timeout =
                    (Rand_div(225 as libc::c_int) + 225 as libc::c_int) as
                        s16b
            }
            94 => {
                if doit == 0 {
                    return b"resist elements (dur 40+d40) every 200 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"It glows many colours...\x00" as *const u8 as
                              *const libc::c_char);
                set_oppose_acid((*p_ptr).oppose_acid as libc::c_int +
                                    (Rand_div(40 as libc::c_int) +
                                         1 as libc::c_int) +
                                    40 as libc::c_int);
                set_oppose_elec((*p_ptr).oppose_elec as libc::c_int +
                                    (Rand_div(40 as libc::c_int) +
                                         1 as libc::c_int) +
                                    40 as libc::c_int);
                set_oppose_fire((*p_ptr).oppose_fire as libc::c_int +
                                    (Rand_div(40 as libc::c_int) +
                                         1 as libc::c_int) +
                                    40 as libc::c_int);
                set_oppose_cold((*p_ptr).oppose_cold as libc::c_int +
                                    (Rand_div(40 as libc::c_int) +
                                         1 as libc::c_int) +
                                    40 as libc::c_int);
                set_oppose_pois((*p_ptr).oppose_pois as libc::c_int +
                                    (Rand_div(40 as libc::c_int) +
                                         1 as libc::c_int) +
                                    40 as libc::c_int);
                (*o_ptr).timeout = 200 as libc::c_int as s16b
            }
            95 => {
                if doit == 0 {
                    return b"speed (dur 20+d20) every 250 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows bright green...\x00" as *const u8 as
                              *const libc::c_char);
                if (*p_ptr).fast == 0 {
                    set_fast(Rand_div(20 as libc::c_int) + 1 as libc::c_int +
                                 20 as libc::c_int, 10 as libc::c_int);
                } else {
                    set_fast((*p_ptr).fast as libc::c_int + 5 as libc::c_int,
                             10 as libc::c_int);
                }
                (*o_ptr).timeout = 250 as libc::c_int as s16b
            }
            96 => {
                if doit == 0 {
                    return b"speed (dur 75+d75) every 200+d200 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows brightly...\x00" as *const u8 as
                              *const libc::c_char);
                if (*p_ptr).fast == 0 {
                    set_fast(Rand_div(75 as libc::c_int) + 1 as libc::c_int +
                                 75 as libc::c_int, 10 as libc::c_int);
                } else {
                    set_fast((*p_ptr).fast as libc::c_int + 5 as libc::c_int,
                             10 as libc::c_int);
                }
                (*o_ptr).timeout =
                    (Rand_div(200 as libc::c_int) + 200 as libc::c_int) as
                        s16b
            }
            97 => {
                if doit == 0 {
                    return b"wraith form (level/2 + d(level/2)) every 1000 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                set_shadow((*p_ptr).tim_wraith as libc::c_int +
                               (Rand_div(plev / 2 as libc::c_int) +
                                    1 as libc::c_int) +
                               plev / 2 as libc::c_int);
                (*o_ptr).timeout = 1000 as libc::c_int as s16b
            }
            98 => {
                if doit == 0 {
                    return b"invulnerability (dur 8+d8) every 1000 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                set_invuln((*p_ptr).invuln as libc::c_int +
                               (Rand_div(8 as libc::c_int) + 1 as libc::c_int)
                               + 8 as libc::c_int);
                (*o_ptr).timeout = 1000 as libc::c_int as s16b
            }
            111 => {
                /* Activate for general purpose effect (detection etc.) */
                if doit == 0 {
                    return format(b"light area (dam 2d15) every %s turns\x00"
                                      as *const u8 as *const libc::c_char,
                                  if is_junkart as libc::c_int != 0 {
                                      b"100\x00" as *const u8 as
                                          *const libc::c_char
                                  } else {
                                      b"10+d10\x00" as *const u8 as
                                          *const libc::c_char
                                  })
                }
                msg_print(b"It wells with clear light...\x00" as *const u8 as
                              *const libc::c_char);
                lite_area(damroll(2 as libc::c_int as s16b,
                                  15 as libc::c_int as s16b),
                          3 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(10 as libc::c_int) + 10 as libc::c_int) as s16b
            }
            112 => {
                if doit == 0 {
                    return b"light (dam 2d15) & map area every 50+d50 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"It shines brightly...\x00" as *const u8 as
                              *const libc::c_char);
                map_area();
                lite_area(damroll(2 as libc::c_int as s16b,
                                  15 as libc::c_int as s16b),
                          3 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(50 as libc::c_int) + 50 as libc::c_int) as s16b
            }
            113 => {
                if doit == 0 {
                    return b"detection every 55+d55 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"It glows bright white...\x00" as *const u8 as
                              *const libc::c_char);
                msg_print(b"An image forms in your mind...\x00" as *const u8
                              as *const libc::c_char);
                detect_all(25 as libc::c_int);
                (*o_ptr).timeout =
                    (Rand_div(55 as libc::c_int) + 55 as libc::c_int) as s16b
            }
            114 => {
                if doit == 0 {
                    return b"detection, probing and identify true every 1000 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                msg_print(b"It glows brightly...\x00" as *const u8 as
                              *const libc::c_char);
                detect_all(25 as libc::c_int);
                probing();
                identify_fully();
                (*o_ptr).timeout = 1000 as libc::c_int as s16b
            }
            115 => {
                if doit == 0 {
                    return b"identify true every 750 turns\x00" as *const u8
                               as *const libc::c_char
                }
                msg_print(b"It glows yellow...\x00" as *const u8 as
                              *const libc::c_char);
                identify_fully();
                (*o_ptr).timeout = 750 as libc::c_int as s16b
            }
            116 => {
                if doit == 0 {
                    return b"identify spell every 10 turns\x00" as *const u8
                               as *const libc::c_char
                }
                if !(ident_spell() == 0) {
                    (*o_ptr).timeout = 10 as libc::c_int as s16b
                }
            }
            117 => {
                if doit == 0 {
                    return b"explosive rune every 200 turns\x00" as *const u8
                               as *const libc::c_char
                }
                msg_print(b"It glows bright red...\x00" as *const u8 as
                              *const libc::c_char);
                explosive_rune();
                (*o_ptr).timeout = 200 as libc::c_int as s16b
            }
            118 => {
                if doit == 0 {
                    return b"rune of protection every 400 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows light blue...\x00" as *const u8 as
                              *const libc::c_char);
                warding_glyph();
                (*o_ptr).timeout = 400 as libc::c_int as s16b
            }
            119 => {
                if doit == 0 {
                    return b"satisfy hunger every 200 turns\x00" as *const u8
                               as *const libc::c_char
                }
                set_food(15000 as libc::c_int - 1 as libc::c_int);
                (*o_ptr).timeout = 200 as libc::c_int as s16b
            }
            120 => {
                if doit == 0 {
                    return b"destroy doors and traps every 10 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"It glows bright red...\x00" as *const u8 as
                              *const libc::c_char);
                destroy_doors_touch();
                (*o_ptr).timeout = 10 as libc::c_int as s16b
            }
            121 => {
                if doit == 0 {
                    return b"stone to mud every 5 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"It pulsates...\x00" as *const u8 as
                              *const libc::c_char);
                if !(get_aim_dir(&mut dir) == 0) {
                    wall_to_mud(dir);
                    (*o_ptr).timeout = 5 as libc::c_int as s16b
                }
            }
            122 => {
                if doit == 0 {
                    return b"recharging every 70 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                recharge(60 as libc::c_int);
                (*o_ptr).timeout = 70 as libc::c_int as s16b
            }
            123 => {
                if doit == 0 {
                    return b"alchemy every 500 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"It glows bright yellow...\x00" as *const u8 as
                              *const libc::c_char);
                alchemy();
                (*o_ptr).timeout = 500 as libc::c_int as s16b
            }
            124 => {
                if doit == 0 {
                    return b"dimension door every 100 turns\x00" as *const u8
                               as *const libc::c_char
                }
                if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 {
                    msg_print(b"Not on special levels!\x00" as *const u8 as
                                  *const libc::c_char);
                } else {
                    msg_print(b"You open a Void Jumpgate. Choose a destination.\x00"
                                  as *const u8 as *const libc::c_char);
                    if !(tgt_pt(&mut ii, &mut ij) == 0) {
                        (*p_ptr).energy -= 60 as libc::c_int - plev;
                        if !((*f_info.offset((*cave[ij as
                                                        usize].offset(ii as
                                                                          isize)).feat
                                                 as isize)).flags1 as
                                 libc::c_long & 0x10 as libc::c_long != 0 &&
                                 (*cave[ij as usize].offset(ii as isize)).feat
                                     as libc::c_int != 0xaf as libc::c_int &&
                                 (*cave[ij as
                                            usize].offset(ii as isize)).m_idx
                                     == 0 &&
                                 !(ij == (*p_ptr).py as libc::c_int &&
                                       ii == (*p_ptr).px as libc::c_int)) ||
                               (*cave[ij as usize].offset(ii as isize)).info
                                   as libc::c_int & 0x4 as libc::c_int != 0 ||
                               distance(ij, ii, (*p_ptr).py as libc::c_int,
                                        (*p_ptr).px as libc::c_int) >
                                   plev + 2 as libc::c_int ||
                               Rand_div(plev * plev / 2 as libc::c_int) == 0 {
                            msg_print(b"You fail to exit the void correctly!\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            (*p_ptr).energy -= 100 as libc::c_int;
                            get_pos_player(10 as libc::c_int, &mut ij,
                                           &mut ii);
                        }
                        cave_set_feat((*p_ptr).py as libc::c_int,
                                      (*p_ptr).px as libc::c_int,
                                      0xa0 as libc::c_int);
                        cave_set_feat(ij, ii, 0xa0 as libc::c_int);
                        (*cave[(*p_ptr).py as
                                   usize].offset((*p_ptr).px as
                                                     isize)).special =
                            (ii + (ij << 8 as libc::c_int)) as s16b;
                        (*cave[ij as usize].offset(ii as isize)).special =
                            ((*p_ptr).px as libc::c_int +
                                 (((*p_ptr).py as libc::c_int) <<
                                      8 as libc::c_int)) as s16b;
                        (*o_ptr).timeout = 100 as libc::c_int as s16b
                    }
                }
            }
            125 => {
                if doit == 0 {
                    return format(b"teleport (range 100) every %d turns\x00"
                                      as *const u8 as *const libc::c_char,
                                  if is_junkart as libc::c_int != 0 {
                                      100 as libc::c_int
                                  } else { 45 as libc::c_int })
                }
                msg_print(b"It twists space around you...\x00" as *const u8 as
                              *const libc::c_char);
                teleport_player(100 as libc::c_int);
                (*o_ptr).timeout = 45 as libc::c_int as s16b
            }
            126 => {
                if dungeon_flags2 as libc::c_long & 0x10 as libc::c_long == 0
                       ||
                       dungeon_flags2 as libc::c_long & 0x10 as libc::c_long
                           != 0 &&
                           get_check(b"Leave this unique level forever? \x00"
                                         as *const u8 as *const libc::c_char)
                               == 0 {
                    if doit == 0 {
                        return b"word of recall every 200 turns\x00" as
                                   *const u8 as *const libc::c_char
                    }
                    msg_print(b"It glows soft white...\x00" as *const u8 as
                                  *const libc::c_char);
                    recall_player(20 as libc::c_int, 15 as libc::c_int);
                    (*o_ptr).timeout = 200 as libc::c_int as s16b
                }
            }
            127 => {
                if doit == 0 {
                    return b"death\x00" as *const u8 as *const libc::c_char
                }
                take_hit(5000 as libc::c_int,
                         b"activating a death spell\x00" as *const u8 as
                             *const libc::c_char);
            }
            128 => {
                if doit == 0 {
                    return b"Ruination\x00" as *const u8 as
                               *const libc::c_char
                }
                msg_print(b"Your nerves and muscles feel weak and lifeless!\x00"
                              as *const u8 as *const libc::c_char);
                take_hit(damroll(10 as libc::c_int as s16b,
                                 10 as libc::c_int as s16b),
                         b"activating Ruination\x00" as *const u8 as
                             *const libc::c_char);
                dec_stat(3 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                dec_stat(2 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                dec_stat(4 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                dec_stat(0 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                dec_stat(5 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
                dec_stat(1 as libc::c_int, 25 as libc::c_int,
                         1 as libc::c_int);
            }
            129 => {
                if doit == 0 {
                    return b"Destruction every 100 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                earthquake((*p_ptr).py as libc::c_int,
                           (*p_ptr).px as libc::c_int, 12 as libc::c_int);
            }
            130 => {
                if doit == 0 {
                    return b"decreasing Intelligence\x00" as *const u8 as
                               *const libc::c_char
                }
                dec_stat(1 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
            }
            131 => {
                if doit == 0 {
                    return b"decreasing Strength\x00" as *const u8 as
                               *const libc::c_char
                }
                dec_stat(0 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
            }
            132 => {
                if doit == 0 {
                    return b"decreasing Constitution\x00" as *const u8 as
                               *const libc::c_char
                }
                dec_stat(4 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
            }
            133 => {
                if doit == 0 {
                    return b"decreasing Charisma\x00" as *const u8 as
                               *const libc::c_char
                }
                dec_stat(5 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
            }
            134 => {
                if doit == 0 {
                    return b"decreasing Dexterity\x00" as *const u8 as
                               *const libc::c_char
                }
                dec_stat(3 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
            }
            135 => {
                if doit == 0 {
                    return b"decreasing Wisdom\x00" as *const u8 as
                               *const libc::c_char
                }
                dec_stat(2 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
            }
            136 => {
                if doit == 0 {
                    return b"stat loss\x00" as *const u8 as
                               *const libc::c_char
                }
                dec_stat(0 as libc::c_int, 15 as libc::c_int,
                         0 as libc::c_int);
                dec_stat(1 as libc::c_int, 15 as libc::c_int,
                         0 as libc::c_int);
                dec_stat(2 as libc::c_int, 15 as libc::c_int,
                         0 as libc::c_int);
                dec_stat(3 as libc::c_int, 15 as libc::c_int,
                         0 as libc::c_int);
                dec_stat(4 as libc::c_int, 15 as libc::c_int,
                         0 as libc::c_int);
                dec_stat(5 as libc::c_int, 15 as libc::c_int,
                         0 as libc::c_int);
            }
            137 => {
                if doit == 0 {
                    return b"high stat loss\x00" as *const u8 as
                               *const libc::c_char
                }
                dec_stat(0 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
                dec_stat(1 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
                dec_stat(2 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
                dec_stat(3 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
                dec_stat(4 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
                dec_stat(5 as libc::c_int, 25 as libc::c_int,
                         0 as libc::c_int);
            }
            138 => {
                if doit == 0 {
                    return b"experience loss\x00" as *const u8 as
                               *const libc::c_char
                }
                lose_exp((*p_ptr).exp / 20 as libc::c_int);
            }
            139 => {
                if doit == 0 {
                    return b"high experience loss\x00" as *const u8 as
                               *const libc::c_char
                }
                lose_exp((*p_ptr).exp / 10 as libc::c_int);
            }
            140 => {
                if doit == 0 {
                    return b"summon monster\x00" as *const u8 as
                               *const libc::c_char
                }
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                *max_dlv.offset(dungeon_type as isize) as
                                    libc::c_int, 0 as libc::c_int);
            }
            141 => {
                if doit == 0 {
                    return b"paralyze\x00" as *const u8 as *const libc::c_char
                }
                set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                  20 as libc::c_int +
                                  (Rand_div(10 as libc::c_int) +
                                       1 as libc::c_int));
            }
            142 => {
                if doit == 0 {
                    return b"hallucination every 10 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                set_image((*p_ptr).image as libc::c_int + 20 as libc::c_int +
                              (Rand_div(10 as libc::c_int) +
                                   1 as libc::c_int));
            }
            143 => {
                if doit == 0 {
                    return b"poison\x00" as *const u8 as *const libc::c_char
                }
                set_poisoned((*p_ptr).poisoned as libc::c_int +
                                 20 as libc::c_int +
                                 (Rand_div(10 as libc::c_int) +
                                      1 as libc::c_int));
            }
            144 => {
                if doit == 0 {
                    return b"create hunger\x00" as *const u8 as
                               *const libc::c_char
                }
                set_food(1000 as libc::c_int);
            }
            145 => {
                if doit == 0 {
                    return b"stun\x00" as *const u8 as *const libc::c_char
                }
                set_stun((*p_ptr).stun as libc::c_int + 20 as libc::c_int +
                             (Rand_div(10 as libc::c_int) +
                                  1 as libc::c_int));
            }
            146 => {
                if doit == 0 {
                    return b"cuts\x00" as *const u8 as *const libc::c_char
                }
                set_cut((*p_ptr).cut as libc::c_int + 20 as libc::c_int +
                            (Rand_div(10 as libc::c_int) + 1 as libc::c_int));
            }
            147 => {
                if doit == 0 {
                    return b"confusion\x00" as *const u8 as
                               *const libc::c_char
                }
                set_confused((*p_ptr).confused as libc::c_int +
                                 30 as libc::c_int +
                                 (Rand_div(10 as libc::c_int) +
                                      1 as libc::c_int));
            }
            148 => {
                if doit == 0 {
                    return b"confusion\x00" as *const u8 as
                               *const libc::c_char
                }
                set_confused((*p_ptr).confused as libc::c_int +
                                 20 as libc::c_int +
                                 (Rand_div(10 as libc::c_int) +
                                      1 as libc::c_int));
            }
            149 => {
                if doit == 0 {
                    return b"blindness\x00" as *const u8 as
                               *const libc::c_char
                }
                set_blind((*p_ptr).blind as libc::c_int + 20 as libc::c_int +
                              (Rand_div(10 as libc::c_int) +
                                   1 as libc::c_int));
            }
            150 => {
                if doit == 0 {
                    return b"summon pet every 101 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                summon_specific_friendly((*p_ptr).py as libc::c_int,
                                         (*p_ptr).px as libc::c_int,
                                         *max_dlv.offset(dungeon_type as
                                                             isize) as
                                             libc::c_int, 0 as libc::c_int,
                                         0 as libc::c_int as bool_);
            }
            151 => {
                if doit == 0 {
                    return b"cure confusion every 500 turns\x00" as *const u8
                               as *const libc::c_char
                }
                set_confused(0 as libc::c_int);
            }
            152 => {
                if doit == 0 {
                    return b"cure hallucination every 100 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                set_image(0 as libc::c_int);
            }
            153 => {
                if doit == 0 {
                    return b"cure poison every 100 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                set_poisoned(0 as libc::c_int);
            }
            154 => {
                if doit == 0 {
                    return b"satisfy hunger every 100 turns\x00" as *const u8
                               as *const libc::c_char
                }
                set_food(15000 as libc::c_int - 1 as libc::c_int);
            }
            155 => {
                if doit == 0 {
                    return b"cure stun every 100 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                set_stun(0 as libc::c_int);
            }
            156 => {
                if doit == 0 {
                    return b"cure cuts every 100 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                set_cut(0 as libc::c_int);
            }
            157 => {
                if doit == 0 {
                    return b"cure fear every 100 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                set_afraid(0 as libc::c_int);
            }
            158 => {
                if doit == 0 {
                    return b"cure confusion every 100 turns\x00" as *const u8
                               as *const libc::c_char
                }
                set_confused(0 as libc::c_int);
            }
            159 => {
                if doit == 0 {
                    return b"cure blindness every 100 turns\x00" as *const u8
                               as *const libc::c_char
                }
                set_blind(0 as libc::c_int);
            }
            160 => {
                if doit == 0 {
                    return b"curing every 110 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                set_blind(0 as libc::c_int);
                set_poisoned(0 as libc::c_int);
                set_confused(0 as libc::c_int);
                set_stun(0 as libc::c_int);
                set_cut(0 as libc::c_int);
                set_image(0 as libc::c_int);
            }
            161 => {
                if doit == 0 {
                    return b"darkness\x00" as *const u8 as *const libc::c_char
                }
                unlite_area(damroll(2 as libc::c_int as s16b,
                                    10 as libc::c_int as s16b),
                            10 as libc::c_int);
            }
            162 => {
                if doit == 0 {
                    return b"teleport level every 50 turns\x00" as *const u8
                               as *const libc::c_char
                }
                teleport_player_level();
            }
            163 => {
                if doit == 0 {
                    return b"acquirement every 3000 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                acquirement((*p_ptr).py as libc::c_int,
                            (*p_ptr).px as libc::c_int, 1 as libc::c_int,
                            0 as libc::c_int as bool_,
                            0 as libc::c_int as bool_);
            }
            164 => {
                if doit == 0 {
                    return b"something weird every 5 turns\x00" as *const u8
                               as *const libc::c_char
                }
            }
            165 => {
                if doit == 0 {
                    return b"aggravate\x00" as *const u8 as
                               *const libc::c_char
                }
                aggravate_monsters(1 as libc::c_int);
            }
            166 => {
                if doit == 0 {
                    return b"gain corruption every 10 turns\x00" as *const u8
                               as *const libc::c_char
                }
                gain_random_corruption(0 as libc::c_int);
            }
            167 => {
                if doit == 0 {
                    return b"cure insanity every 200 turns\x00" as *const u8
                               as *const libc::c_char
                }
                heal_insanity(damroll(10 as libc::c_int as s16b,
                                      10 as libc::c_int as s16b));
            }
            168 => {
                msg_print(b"Ahah, you wish.\x00" as *const u8 as
                              *const libc::c_char);
            }
            169 => {
                let mut y_1: libc::c_int = 0;
                let mut x_1: libc::c_int = 0;
                let mut light: libc::c_int = 0 as libc::c_int;
                let mut dir_0: libc::c_int = 0;
                let mut c_ptr_1: *mut cave_type = 0 as *mut cave_type;
                if doit == 0 {
                    return b"light absorption every 80 turns\x00" as *const u8
                               as *const libc::c_char
                }
                y_1 = (*p_ptr).py as libc::c_int - 6 as libc::c_int;
                while y_1 <= (*p_ptr).py as libc::c_int + 6 as libc::c_int {
                    x_1 = (*p_ptr).px as libc::c_int - 6 as libc::c_int;
                    while x_1 <= (*p_ptr).px as libc::c_int + 6 as libc::c_int
                          {
                        if y_1 > 0 as libc::c_int && x_1 > 0 as libc::c_int &&
                               y_1 < cur_hgt as libc::c_int - 1 as libc::c_int
                               &&
                               x_1 < cur_wid as libc::c_int - 1 as libc::c_int
                           {
                            c_ptr_1 =
                                &mut *(*cave.as_mut_ptr().offset(y_1 as
                                                                     isize)).offset(x_1
                                                                                        as
                                                                                        isize)
                                    as *mut cave_type;
                            if !(distance(y_1, x_1,
                                          (*p_ptr).py as libc::c_int,
                                          (*p_ptr).px as libc::c_int) >
                                     6 as libc::c_int) {
                                if (*c_ptr_1).info as libc::c_int &
                                       0x2 as libc::c_int != 0 {
                                    light += 1;
                                    /* No longer in the array */
                                    (*c_ptr_1).info =
                                        ((*c_ptr_1).info as libc::c_int &
                                             !(0x40 as libc::c_int)) as u16b;
                                    /* Darken the grid */
                                    (*c_ptr_1).info =
                                        ((*c_ptr_1).info as libc::c_int &
                                             !(0x2 as libc::c_int)) as u16b;
                                    /* Hack -- Forget "boring" grids */
                                    if (*f_info.offset((*c_ptr_1).feat as
                                                           isize)).flags1 as
                                           libc::c_long & 0x10 as libc::c_long
                                           != 0 &&
                                           (*f_info.offset((*c_ptr_1).feat as
                                                               isize)).flags1
                                               as libc::c_long &
                                               0x100 as libc::c_long == 0 &&
                                           (*c_ptr_1).info as libc::c_int &
                                               0x100 as libc::c_int == 0 {
                                        /* Forget the grid */
                                        (*c_ptr_1).info =
                                            ((*c_ptr_1).info as libc::c_int &
                                                 !(0x1 as libc::c_int)) as
                                                u16b;
                                        /* Notice */
                                        note_spot(y_1, x_1);
                                    }
                                    /* Process affected monsters */
                                    if (*c_ptr_1).m_idx != 0 {
                                        /* Update the monster */
                                        update_mon((*c_ptr_1).m_idx as
                                                       libc::c_int,
                                                   0 as libc::c_int as bool_);
                                    }
                                    /* Redraw */
                                    lite_spot(y_1, x_1);
                                }
                            }
                        }
                        x_1 += 1
                    }
                    y_1 += 1
                }
                if get_aim_dir(&mut dir_0) == 0 {
                    return 0 as *const libc::c_char
                }
                msg_print(b"The light around you is absorbed... and released in a powerful bolt!\x00"
                              as *const u8 as *const libc::c_char);
                fire_bolt(15 as libc::c_int, dir_0,
                          damroll(light as s16b, (*p_ptr).lev));
            }
            170 => {
                /* Horns of DragonKind (Note that these are new egos)*/
                if doit == 0 {
                    return b"large fire ball (300) every 100 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                fire_ball(5 as libc::c_int, 5 as libc::c_int,
                          300 as libc::c_int, 7 as libc::c_int);
                (*o_ptr).timeout = 100 as libc::c_int as s16b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
            }
            171 => {
                if doit == 0 {
                    return b"large cold ball (300) every 100 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                fire_ball(4 as libc::c_int, 5 as libc::c_int,
                          300 as libc::c_int, 7 as libc::c_int);
                (*o_ptr).timeout = 100 as libc::c_int as s16b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
            }
            172 => {
                if doit == 0 {
                    return b"large lightning ball (300) every 100 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                fire_ball(1 as libc::c_int, 5 as libc::c_int,
                          300 as libc::c_int, 7 as libc::c_int);
                (*o_ptr).timeout = 100 as libc::c_int as s16b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
            }
            173 => {
                if doit == 0 {
                    return b"large acid ball (300) every 100 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                fire_ball(3 as libc::c_int, 5 as libc::c_int,
                          300 as libc::c_int, 7 as libc::c_int);
                (*o_ptr).timeout = 100 as libc::c_int as s16b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
            }
            174 => {
                if doit == 0 {
                    return b"spinning around every 50+d25 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                do_spin();
                (*o_ptr).timeout =
                    (50 as libc::c_int +
                         (Rand_div(25 as libc::c_int) + 1 as libc::c_int)) as
                        s16b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
            }
            175 => {
                if doit == 0 {
                    return b"detect treasure every 10+d20 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                detect_treasure(25 as libc::c_int);
                (*o_ptr).timeout =
                    (10 as libc::c_int +
                         (Rand_div(20 as libc::c_int) + 1 as libc::c_int)) as
                        s16b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
            }
            176 => {
                if doit == 0 {
                    return b"wraith-form every 50+d50 turns\x00" as *const u8
                               as *const libc::c_char
                }
                if (*p_ptr).wraith_form == 0 {
                    set_shadow(20 as libc::c_int +
                                   (Rand_div(20 as libc::c_int) +
                                        1 as libc::c_int));
                } else {
                    set_shadow((*p_ptr).tim_wraith as libc::c_int +
                                   (Rand_div(20 as libc::c_int) +
                                        1 as libc::c_int));
                }
                (*o_ptr).timeout =
                    (50 as libc::c_int +
                         (Rand_div(50 as libc::c_int) + 1 as libc::c_int)) as
                        s16b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
            }
            177 => {
                if doit == 0 {
                    return b"phasing every 10+d10 turns\x00" as *const u8 as
                               *const libc::c_char
                }
                teleport_player(10 as libc::c_int);
                (*o_ptr).timeout =
                    (10 as libc::c_int +
                         (Rand_div(10 as libc::c_int) + 1 as libc::c_int)) as
                        s16b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
            }
            178 => {
                if doit == 0 {
                    return b"teleportation and destruction of the ring\x00" as
                               *const u8 as *const libc::c_char
                }
                if item == 0 {
                    msg_print(b"You can\'t activate this when it\'s there!\x00"
                                  as *const u8 as *const libc::c_char);
                }
                if get_check(b"This will destroy the ring. Do you wish to continue? \x00"
                                 as *const u8 as *const libc::c_char) != 0 {
                    msg_print(b"The ring explodes into a space distortion.\x00"
                                  as *const u8 as *const libc::c_char);
                    teleport_player(200 as libc::c_int);
                    /* It explodes, doesn't it ? */
                    take_hit(damroll(2 as libc::c_int as s16b,
                                     10 as libc::c_int as s16b),
                             b"an exploding ring\x00" as *const u8 as
                                 *const libc::c_char);
                    inc_stack_size_ex(item, -(255 as libc::c_int), OPTIMIZE,
                                      NO_DESCRIBE);
                }
            }
            179 => {
                /*amulet of serpents dam 100, rad 2 timeout 40+d60 */
                if doit == 0 {
                    return b"venom breathing every 40+d60 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                /* Get a direction for breathing (or abort) */
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe venom...\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(2 as libc::c_int, dir, 100 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(60 as libc::c_int) + 40 as libc::c_int) as
                            s16b;
                    /* Window stuff */
                    (*p_ptr).window =
                        ((*p_ptr).window as libc::c_long |
                             (0x1 as libc::c_long | 0x2 as libc::c_long)) as
                            u32b
                }
            }
            180 => {
                /*rings of X 50,50+d50 dur 20+d20 */
                if doit == 0 {
                    return b"ball of cold and resist cold every 50+d50 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                /* Get a direction for breathing (or abort) */
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(4 as libc::c_int, dir, 50 as libc::c_int,
                              2 as libc::c_int);
                    set_oppose_cold((*p_ptr).oppose_cold as libc::c_int +
                                        (Rand_div(20 as libc::c_int) +
                                             1 as libc::c_int) +
                                        20 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(50 as libc::c_int) + 50 as libc::c_int) as
                            s16b
                }
            }
            181 => {
                if doit == 0 {
                    return b"ball of fire and resist fire every 50+d50 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                /* Get a direction for breathing (or abort) */
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(5 as libc::c_int, dir, 50 as libc::c_int,
                              2 as libc::c_int);
                    set_oppose_fire((*p_ptr).oppose_fire as libc::c_int +
                                        (Rand_div(20 as libc::c_int) +
                                             1 as libc::c_int) +
                                        20 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(50 as libc::c_int) + 50 as libc::c_int) as
                            s16b
                }
            }
            182 => {
                if doit == 0 {
                    return b"ball of acid and resist acid every 50+d50 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                /* Get a direction for breathing (or abort) */
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(3 as libc::c_int, dir, 50 as libc::c_int,
                              2 as libc::c_int);
                    set_oppose_acid((*p_ptr).oppose_acid as libc::c_int +
                                        (Rand_div(20 as libc::c_int) +
                                             1 as libc::c_int) +
                                        20 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(50 as libc::c_int) + 50 as libc::c_int) as
                            s16b
                }
            }
            183 => {
                if doit == 0 {
                    return b"ball of lightning and resist lightning every 50+d50 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                /* Get a direction for breathing (or abort) */
                if !(get_aim_dir(&mut dir) == 0) {
                    fire_ball(1 as libc::c_int, dir, 50 as libc::c_int,
                              2 as libc::c_int);
                    set_oppose_elec((*p_ptr).oppose_elec as libc::c_int +
                                        (Rand_div(20 as libc::c_int) +
                                             1 as libc::c_int) +
                                        20 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(50 as libc::c_int) + 50 as libc::c_int) as
                            s16b
                }
            }
            184 => {
                if doit == 0 {
                    return b"breathe lightning (100) every 90+d90 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe lightning.\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(1 as libc::c_int, dir, 100 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 90 as libc::c_int) as
                            s16b
                }
            }
            185 => {
                if doit == 0 {
                    return b"breathe frost (110) every 90+d90 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe frost.\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(4 as libc::c_int, dir, 110 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 90 as libc::c_int) as
                            s16b
                }
            }
            186 => {
                if doit == 0 {
                    return b"breathe fire (200) every 90+d90 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe fire.\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(5 as libc::c_int, dir, 200 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 90 as libc::c_int) as
                            s16b
                }
            }
            187 => {
                if doit == 0 {
                    return b"breathe acid (130) every 90+d90 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe acid.\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(3 as libc::c_int, dir, 130 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 90 as libc::c_int) as
                            s16b
                }
            }
            188 => {
                if doit == 0 {
                    return b"breathe poison gas (150) every 90+d90 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe poison gas.\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(2 as libc::c_int, dir, 150 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 90 as libc::c_int) as
                            s16b
                }
            }
            189 => {
                if doit == 0 {
                    return b"breathe multi-hued (250) every 60+d60 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    chance = Rand_div(5 as libc::c_int);
                    msg_format(b"You breathe %s.\x00" as *const u8 as
                                   *const libc::c_char,
                               if chance == 1 as libc::c_int {
                                   b"lightning\x00" as *const u8 as
                                       *const libc::c_char
                               } else if chance == 2 as libc::c_int {
                                   b"frost\x00" as *const u8 as
                                       *const libc::c_char
                               } else if chance == 3 as libc::c_int {
                                   b"acid\x00" as *const u8 as
                                       *const libc::c_char
                               } else if chance == 4 as libc::c_int {
                                   b"poison gas\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"fire\x00" as *const u8 as
                                       *const libc::c_char
                               });
                    fire_ball(if chance == 1 as libc::c_int {
                                  1 as libc::c_int
                              } else if chance == 2 as libc::c_int {
                                  4 as libc::c_int
                              } else if chance == 3 as libc::c_int {
                                  3 as libc::c_int
                              } else if chance == 4 as libc::c_int {
                                  2 as libc::c_int
                              } else { 5 as libc::c_int }, dir,
                              250 as libc::c_int, 2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(60 as libc::c_int) + 60 as libc::c_int) as
                            s16b
                }
            }
            190 => {
                if doit == 0 {
                    return b"breathe confusion (120) every 90+d90 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe confusion.\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(22 as libc::c_int, dir, 120 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 90 as libc::c_int) as
                            s16b
                }
            }
            191 => {
                if doit == 0 {
                    return b"breathe sound (130) every 90+d90 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe sound.\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(21 as libc::c_int, dir, 130 as libc::c_int,
                              2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 90 as libc::c_int) as
                            s16b
                }
            }
            192 => {
                if doit == 0 {
                    return b"breathe chaos/disenchant (220) every 60+d90 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    chance = Rand_div(2 as libc::c_int);
                    msg_format(b"You breathe %s.\x00" as *const u8 as
                                   *const libc::c_char,
                               if chance == 1 as libc::c_int {
                                   b"chaos\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"disenchantment\x00" as *const u8 as
                                       *const libc::c_char
                               });
                    fire_ball(if chance == 1 as libc::c_int {
                                  30 as libc::c_int
                              } else { 32 as libc::c_int }, dir,
                              220 as libc::c_int, 2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 60 as libc::c_int) as
                            s16b
                }
            }
            193 => {
                if doit == 0 {
                    return b"breathe sound/shards (230) every 60+d90 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    chance = Rand_div(2 as libc::c_int);
                    msg_format(b"You breathe %s.\x00" as *const u8 as
                                   *const libc::c_char,
                               if chance == 1 as libc::c_int {
                                   b"sound\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"shards\x00" as *const u8 as
                                       *const libc::c_char
                               });
                    fire_ball(if chance == 1 as libc::c_int {
                                  21 as libc::c_int
                              } else { 20 as libc::c_int }, dir,
                              230 as libc::c_int, 2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 60 as libc::c_int) as
                            s16b
                }
            }
            194 => {
                if doit == 0 {
                    return b"breathe balance (250) every 60+d90 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    chance = Rand_div(4 as libc::c_int);
                    msg_format(b"You breathe %s.\x00" as *const u8 as
                                   *const libc::c_char,
                               if chance == 1 as libc::c_int {
                                   b"chaos\x00" as *const u8 as
                                       *const libc::c_char
                               } else if chance == 2 as libc::c_int {
                                   b"disenchantment\x00" as *const u8 as
                                       *const libc::c_char
                               } else if chance == 3 as libc::c_int {
                                   b"sound\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"shards\x00" as *const u8 as
                                       *const libc::c_char
                               });
                    fire_ball(if chance == 1 as libc::c_int {
                                  30 as libc::c_int
                              } else if chance == 2 as libc::c_int {
                                  32 as libc::c_int
                              } else if chance == 3 as libc::c_int {
                                  21 as libc::c_int
                              } else { 20 as libc::c_int }, dir,
                              250 as libc::c_int, 2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 60 as libc::c_int) as
                            s16b
                }
            }
            195 => {
                if doit == 0 {
                    return b"breathe light/darkness (200) every 60+d90 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    chance = Rand_div(2 as libc::c_int);
                    msg_format(b"You breathe %s.\x00" as *const u8 as
                                   *const libc::c_char,
                               if chance == 0 as libc::c_int {
                                   b"light\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"darkness\x00" as *const u8 as
                                       *const libc::c_char
                               });
                    fire_ball(if chance == 0 as libc::c_int {
                                  15 as libc::c_int
                              } else { 16 as libc::c_int }, dir,
                              200 as libc::c_int, 2 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 60 as libc::c_int) as
                            s16b
                }
            }
            196 => {
                if doit == 0 {
                    return b"breathe the elements (300) every 60+d90 turns\x00"
                               as *const u8 as *const libc::c_char
                }
                if !(get_aim_dir(&mut dir) == 0) {
                    msg_print(b"You breathe the elements.\x00" as *const u8 as
                                  *const libc::c_char);
                    fire_ball(10 as libc::c_int, dir, 300 as libc::c_int,
                              3 as libc::c_int);
                    (*o_ptr).timeout =
                        (Rand_div(90 as libc::c_int) + 60 as libc::c_int) as
                            s16b
                }
            }
            197 => {
                if doit == 0 {
                    return b"grow mushrooms every 50+d50 turns\x00" as
                               *const u8 as *const libc::c_char
                }
                msg_print(b"You twirl and spores fly everywhere!\x00" as
                              *const u8 as *const libc::c_char);
                i = 0 as libc::c_int;
                while i < 8 as libc::c_int {
                    summon_specific_friendly((*p_ptr).py as libc::c_int,
                                             (*p_ptr).px as libc::c_int,
                                             (*p_ptr).lev as libc::c_int,
                                             33 as libc::c_int,
                                             0 as libc::c_int as bool_);
                    i += 1
                }
                (*o_ptr).timeout =
                    (Rand_div(50 as libc::c_int) + 1 as libc::c_int +
                         50 as libc::c_int) as s16b
            }
            200 | _ => {
                /*
			 fall through to unknown, as music should be
			 handled by calling procedure.
			 */
                msg_format(b"Unknown activation effect: %d.\x00" as *const u8
                               as *const libc::c_char, spell);
                if doit == 0 {
                    return b"Unknown Activation\x00" as *const u8 as
                               *const libc::c_char
                }
                return 0 as *const libc::c_char
            }
        }
    }
    /* Set timeout for junkarts
	 * Note that I still need to set the timeouts for other
	 * (non-random) artifacts above 
	 */
    if is_junkart as libc::c_int != 0 && doit as libc::c_int != 0 {
        (*o_ptr).timeout =
            activation_info[(*o_ptr).pval2 as
                                usize].cost.wrapping_div(10 as libc::c_int as
                                                             libc::c_uint) as
                s16b
    }
    return 0 as *const libc::c_char;
}
/* File: cmd6.c */
/* Purpose: Object commands */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Forward declare
 */
unsafe extern "C" fn activate_spell(mut o_ptr: *mut object_type,
                                    mut choice: byte_hack) -> bool_ {
    let mut mana: libc::c_int = 0 as libc::c_int;
    let mut gf: libc::c_int = 0 as libc::c_int;
    let mut mod_0: libc::c_int = 0 as libc::c_int;
    let mut s_ptr: rune_spell =
        rune_spell{name: [0; 30], type_0: 0, rune2: 0, mana: 0,};
    if choice as libc::c_int == 1 as libc::c_int {
        gf = (*o_ptr).pval & 0xffff as libc::c_int;
        mod_0 = (*o_ptr).pval3 & 0xffff as libc::c_int;
        mana = (*o_ptr).pval2 as libc::c_int & 0xff as libc::c_int
    } else if choice as libc::c_int == 2 as libc::c_int {
        gf = (*o_ptr).pval >> 16 as libc::c_int;
        mod_0 = (*o_ptr).pval3 >> 16 as libc::c_int;
        mana = (*o_ptr).pval2 as libc::c_int >> 8 as libc::c_int
    }
    s_ptr.type_0 = gf as s16b;
    s_ptr.rune2 = ((1 as libc::c_int) << mod_0) as s16b;
    s_ptr.mana = mana as s16b;
    /* Execute */
    rune_exec(&mut s_ptr, 0 as libc::c_int);
    if choice as libc::c_int == 1 as libc::c_int {
        (*o_ptr).timeout = (mana * 5 as libc::c_int) as s16b
    }
    if choice as libc::c_int == 2 as libc::c_int {
        (*o_ptr).xtra2 = (mana * 5 as libc::c_int) as s16b
    }
    return 1 as libc::c_int as bool_;
}
