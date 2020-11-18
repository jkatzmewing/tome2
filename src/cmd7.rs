use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut adj_mag_fail: [byte_hack; 0];
    #[no_mangle]
    static mut adj_mag_stat: [byte_hack; 0];
    #[no_mangle]
    static mut mindcraft_powers: [magic_power; 12];
    #[no_mangle]
    static mut necro_powers: [magic_power; 6];
    #[no_mangle]
    static mut mimic_powers: [magic_power; 5];
    #[no_mangle]
    static mut symbiotic_powers: [magic_power; 9];
    #[no_mangle]
    static mut character_icky: bool_;
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
    static mut flush_failure: bool_;
    #[no_mangle]
    static mut target_col: s16b;
    #[no_mangle]
    static mut target_row: s16b;
    #[no_mangle]
    static mut health_who: s16b;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut alchemist_known_egos: [u32b; 32];
    #[no_mangle]
    static mut alchemist_recipes: *mut alchemist_recipe;
    #[no_mangle]
    static mut alchemist_known_artifacts: [u32b; 6];
    #[no_mangle]
    static mut alchemist_gained: u32b;
    #[no_mangle]
    static mut al_name: *mut libc::c_char;
    #[no_mangle]
    static mut a_select_flags: *mut artifact_select_flag;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut k_name: *mut libc::c_char;
    #[no_mangle]
    static mut e_info: *mut ego_item_type;
    #[no_mangle]
    static mut e_name: *mut libc::c_char;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut item_tester_tval: byte_hack;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut max_al_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_e_idx: u16b;
    #[no_mangle]
    static mut random_spells: [random_spell; 100];
    #[no_mangle]
    static mut spell_num: s16b;
    #[no_mangle]
    static mut rune_spells: [rune_spell; 100];
    #[no_mangle]
    static mut rune_num: s16b;
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut ironman_rooms: bool_;
    #[no_mangle]
    static mut k_allow_special: *mut bool_;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut deity_info: *mut deity_type;
    #[no_mangle]
    fn add_hook(h_idx: libc::c_int, hook: hook_type, name: cptr)
     -> *mut hooks_chain;
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn no_lite() -> bool_;
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
    fn do_cmd_integrate_body() -> bool_;
    #[no_mangle]
    fn do_cmd_leave_body(drop_body: bool_) -> bool_;
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
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn c_prt(attr: byte_hack, str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn inven_item_optimize(item: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn inc_stack_size(item: libc::c_int, delta: libc::c_int);
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn quark_add(str: cptr) -> s16b;
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn object_out_desc(o_ptr: *mut object_type, fff: *mut FILE,
                       trim_down: bool_, wait_for_it: bool_) -> bool_;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn inc_stack_size_ex(item: libc::c_int, delta: libc::c_int,
                         opt: optimize_flag, desc: describe_flag);
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn activation_aux(o_ptr: *mut object_type, desc: bool_, item: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn display_list(y: libc::c_int, x: libc::c_int, h: libc::c_int,
                    w: libc::c_int, title: cptr, list: *mut cptr,
                    max: libc::c_int, begin: libc::c_int, sel: libc::c_int,
                    sel_color: byte_hack);
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn is_magestaff() -> bool_;
    #[no_mangle]
    fn symbiote_name(capitalize: bool_) -> cptr;
    #[no_mangle]
    fn use_symbiotic_power(r_idx: libc::c_int, great: bool_,
                           only_number: bool_, no_cost: bool_) -> libc::c_int;
    #[no_mangle]
    fn inven_carry(o_ptr: *mut object_type, final_0: bool_) -> s16b;
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn wall_to_mud(dir: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_rep_dir(dp: *mut libc::c_int) -> bool_;
    #[link_name = "unsafe"]
    static mut unsafe_0: bool_;
    #[no_mangle]
    fn project(who: libc::c_int, rad: libc::c_int, y: libc::c_int,
               x: libc::c_int, dam: libc::c_int, typ: libc::c_int,
               flg: libc::c_int) -> bool_;
    #[no_mangle]
    fn project_meteor(radius: libc::c_int, typ: libc::c_int, dam: libc::c_int,
                      flg: u32b);
    #[no_mangle]
    fn project_hack(typ: libc::c_int, dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn target_okay() -> bool_;
    #[no_mangle]
    fn get_aim_dir(dp: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn is_a_vowel(ch: libc::c_int) -> bool_;
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn get_skill_scale(skill: libc::c_int, scale: u32b) -> s16b;
    #[no_mangle]
    static mut bypass_r_ptr_max_num: bool_;
    #[no_mangle]
    fn place_monster_one(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         ego: libc::c_int, slp: bool_, status: libc::c_int)
     -> s16b;
    #[no_mangle]
    static mut place_monster_one_no_drop: bool_;
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn dec_stat(stat: libc::c_int, amount: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn fire_ball(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int,
                 rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_fast(v: libc::c_int, p: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_shero(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_hero(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn hp_player(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_stun(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_afraid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn mindblast_monsters(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn ident_spell() -> bool_;
    #[no_mangle]
    fn set_oppose_pois(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_elec(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_cold(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_fire(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_acid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_shield(v: libc::c_int, p: libc::c_int, o: s16b, d1: s16b, d2: s16b)
     -> bool_;
    #[no_mangle]
    fn charm_monsters(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn teleport_player(dis: libc::c_int);
    #[no_mangle]
    fn banish_monsters(dist: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_pos_player(dis: libc::c_int, ny: *mut libc::c_int,
                      nx: *mut libc::c_int);
    #[no_mangle]
    fn tgt_pt(x: *mut libc::c_int, y: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn fire_beam(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn set_tim_esp(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_all(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_traps(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_monsters_invis(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_monsters_normal(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_image(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn lose_all_info() -> bool_;
    #[no_mangle]
    fn repeat_push(what: libc::c_int);
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn repeat_pull(what: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_invis(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_invis(v: libc::c_int, p: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_mimic(v: libc::c_int, p: libc::c_int, level: libc::c_int) -> bool_;
    #[no_mangle]
    fn resolve_mimic_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn set_rush(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn inven_item_describe(item: libc::c_int);
    #[no_mangle]
    fn inven_carry_okay(o_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn apply_magic(o_ptr: *mut object_type, lev: libc::c_int, okay: bool_,
                   good: bool_, great: bool_);
    #[no_mangle]
    static mut hack_apply_magic_power: libc::c_int;
    #[no_mangle]
    static mut tvals: [tval_desc2; 0];
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn item_tester_hook_recharge(o_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn object_value_real(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn has_ability(ab: libc::c_int) -> bool_;
    #[no_mangle]
    fn summon_specific_friendly(y1: libc::c_int, x1: libc::c_int,
                                lev: libc::c_int, type_0: libc::c_int,
                                Group_ok: bool_) -> bool_;
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn m_bonus(max: libc::c_int, level: libc::c_int) -> s16b;
    #[no_mangle]
    fn calc_hitpoints();
    #[no_mangle]
    fn fire_bolt(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn drain_life(dir: libc::c_int, dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_absorb_soul(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn test_item_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn summon_specific(y1: libc::c_int, x1: libc::c_int, lev: libc::c_int,
                       type_0: libc::c_int) -> bool_;
    #[no_mangle]
    fn destroy_doors_touch() -> bool_;
    #[no_mangle]
    fn get_quantity(prompt: cptr, max: s32b) -> s32b;
    #[no_mangle]
    fn quark_str(num: s16b) -> cptr;
    #[no_mangle]
    fn floor_item_optimize(item: libc::c_int);
    #[no_mangle]
    fn floor_item_describe(item: libc::c_int);
    #[no_mangle]
    fn floor_item_increase(item: libc::c_int, num: libc::c_int);
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn monster_check_experience(m_idx: libc::c_int, silent: bool_);
    #[no_mangle]
    fn delete_monster(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn enchant(o_ptr: *mut object_type, n: libc::c_int, eflag: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
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
pub struct magic_power {
    pub min_lev: libc::c_int,
    pub mana_cost: libc::c_int,
    pub fail: libc::c_int,
    pub name: cptr,
    pub desc: cptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tval_desc2 {
    pub tval: libc::c_int,
    pub desc: cptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alchemist_recipe {
    pub sval_essence: libc::c_int,
    pub tval: byte_hack,
    pub sval: byte_hack,
    pub qty: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct artifact_select_flag {
    pub group: byte_hack,
    pub flag: libc::c_int,
    pub level: byte_hack,
    pub desc: libc::c_int,
    pub xp: u32b,
    pub pval: bool_,
    pub item_desc: libc::c_int,
    pub item_descp: libc::c_int,
    pub rtval: byte_hack,
    pub rsval: byte_hack,
    pub rpval: libc::c_int,
    pub rflag: [libc::c_int; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deity_type {
    pub name: cptr,
    pub desc: [[libc::c_char; 80]; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random_spell {
    pub desc: [libc::c_char; 30],
    pub name: [libc::c_char; 30],
    pub mana: s16b,
    pub fail: s16b,
    pub proj_flags: u32b,
    pub GF: byte_hack,
    pub radius: byte_hack,
    pub dam_sides: byte_hack,
    pub dam_dice: byte_hack,
    pub level: byte_hack,
    pub untried: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rune_spell {
    pub name: [libc::c_char; 30],
    pub type_0: s16b,
    pub rune2: s16b,
    pub mana: s16b,
}
pub type hook_type
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_char) -> bool_>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hooks_chain {
    pub hook: hook_type,
    pub name: [libc::c_char; 40],
    pub script: [libc::c_char; 40],
    pub type_0: byte_hack,
    pub next: *mut hooks_chain,
}
pub type optimize_flag = libc::c_uint;
pub const NO_OPTIMIZE: optimize_flag = 1;
pub const OPTIMIZE: optimize_flag = 0;
pub type describe_flag = libc::c_uint;
pub const NO_DESCRIBE: describe_flag = 1;
pub const DESCRIBE: describe_flag = 0;
/* File: cmd7.c */
/* Purpose: More Class commands */
/*
 * Copyright (c) 1999 Dark God
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Describe class powers of Mindcrafters
 *
 * 'p' points to a 80 byte long buffer
 */
#[no_mangle]
pub unsafe extern "C" fn mindcraft_info(mut p: *mut libc::c_char,
                                        mut power: libc::c_int) {
    let mut plev: libc::c_int = get_skill(29 as libc::c_int) as libc::c_int;
    /* Clear buffer */
    strcpy(p, b"\x00" as *const u8 as *const libc::c_char);
    /* Fill the buffer with requested power description */
    match power {
        0 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" rad %d\x00" as *const u8 as *const libc::c_char,
                    25 as libc::c_int);
        }
        1 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dam %dd%d\x00" as *const u8 as *const libc::c_char,
                    3 as libc::c_int +
                        (plev - 1 as libc::c_int) / 4 as libc::c_int,
                    3 as libc::c_int + plev / 15 as libc::c_int);
        }
        2 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" range %d\x00" as *const u8 as *const libc::c_char,
                    if plev < 25 as libc::c_int {
                        10 as libc::c_int
                    } else {
                        (plev + 2 as libc::c_int) +
                            (*p_ptr).to_s as libc::c_int * 3 as libc::c_int
                    });
        }
        3 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" range %d\x00" as *const u8 as *const libc::c_char,
                    plev * 5 as libc::c_int);
        }
        4 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" power %d\x00" as *const u8 as *const libc::c_char,
                    plev *
                        (if plev < 30 as libc::c_int {
                             1 as libc::c_int
                         } else { 2 as libc::c_int }));
        }
        5 => {
            if plev > 20 as libc::c_int {
                strnfmt(p, 80 as libc::c_int as uint_hack,
                        b" dam %dd8 rad %d\x00" as *const u8 as
                            *const libc::c_char,
                        8 as libc::c_int +
                            (plev - 5 as libc::c_int) / 4 as libc::c_int,
                        (plev - 20 as libc::c_int) / 8 as libc::c_int +
                            1 as libc::c_int);
            } else {
                strnfmt(p, 80 as libc::c_int as uint_hack,
                        b" dam %dd8\x00" as *const u8 as *const libc::c_char,
                        8 as libc::c_int +
                            (plev - 5 as libc::c_int) / 4 as libc::c_int);
            }
        }
        6 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dur %d\x00" as *const u8 as *const libc::c_char, plev);
        }
        8 => {
            if plev < 25 as libc::c_int {
                strnfmt(p, 80 as libc::c_int as uint_hack,
                        b" dam %d rad %d\x00" as *const u8 as
                            *const libc::c_char,
                        3 as libc::c_int * plev / 2 as libc::c_int,
                        2 as libc::c_int + plev / 10 as libc::c_int);
            } else {
                strnfmt(p, 80 as libc::c_int as uint_hack,
                        b" dam %d\x00" as *const u8 as *const libc::c_char,
                        plev *
                            ((plev - 5 as libc::c_int) / 10 as libc::c_int +
                                 1 as libc::c_int));
            }
        }
        9 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dur 11-%d\x00" as *const u8 as *const libc::c_char,
                    10 as libc::c_int + plev + plev / 2 as libc::c_int);
        }
        10 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dam %dd6 rad %d\x00" as *const u8 as
                        *const libc::c_char, plev / 2 as libc::c_int,
                    0 as libc::c_int +
                        (plev - 25 as libc::c_int) / 10 as libc::c_int);
        }
        11 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dam %d rad %d\x00" as *const u8 as *const libc::c_char,
                    plev *
                        (if plev > 39 as libc::c_int {
                             4 as libc::c_int
                         } else { 3 as libc::c_int }),
                    3 as libc::c_int + plev / 10 as libc::c_int);
        }
        7 | _ => { }
    };
}
/*
 * Describe class powers of Mimics
 *
 * 'p' points to a 80 byte long buffer
 */
#[no_mangle]
pub unsafe extern "C" fn mimic_info(mut p: *mut libc::c_char,
                                    mut power: libc::c_int) {
    let mut plev: libc::c_int = get_skill(32 as libc::c_int) as libc::c_int;
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(38 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Clear the buffer */
    strcpy(p, b"\x00" as *const u8 as *const libc::c_char);
    /* Fill the buffer with requested power description */
    match power {
        0 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dur %d\x00" as *const u8 as *const libc::c_char,
                    (*k_info.offset((*o_ptr).k_idx as isize)).pval2 +
                        get_skill_scale(32 as libc::c_int,
                                        1000 as libc::c_int as u32b) as
                            libc::c_int);
        }
        1 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dur %d+d20\x00" as *const u8 as *const libc::c_char,
                    10 as libc::c_int + plev);
        }
        2 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dur 50+d%d\x00" as *const u8 as *const libc::c_char,
                    50 as libc::c_int + 2 as libc::c_int * plev);
        }
        3 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dur 50+d%d\x00" as *const u8 as *const libc::c_char,
                    50 as libc::c_int + 2 as libc::c_int * plev);
        }
        4 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dur 50+d%d\x00" as *const u8 as *const libc::c_char,
                    50 as libc::c_int + 2 as libc::c_int * plev);
        }
        _ => { }
    };
}
/*
 * Allow user to choose a magic power.
 *
 * If a valid spell is chosen, saves it in '*sn' and returns TRUE
 * If the user hits escape, returns FALSE, and set '*sn' to -1
 * If there are no legal choices, returns FALSE, and sets '*sn' to -2
 *
 * The "prompt" should be "cast", "recite", or "study"
 * The "known" should be TRUE for cast/pray, FALSE for study
 *
 * nb: This function has a (trivial) display bug which will be obvious
 * when you run it. It's probably easy to fix but I haven't tried,
 * sorry.
 */
#[no_mangle]
pub unsafe extern "C" fn get_magic_power(mut sn: *mut libc::c_int,
                                         mut powers: *mut magic_power,
                                         mut max_powers: libc::c_int,
                                         mut power_info:
                                             Option<unsafe extern "C" fn(_:
                                                                             *mut libc::c_char,
                                                                         _:
                                                                             libc::c_int)
                                                        -> ()>,
                                         mut plev: libc::c_int,
                                         mut cast_stat: libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 2 as libc::c_int;
    let mut x: libc::c_int = 18 as libc::c_int;
    let mut minfail: libc::c_int = 0 as libc::c_int;
    let mut chance: libc::c_int = 0 as libc::c_int;
    let mut info: libc::c_int = 0;
    let mut choice: libc::c_char = 0;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut comment: [libc::c_char; 80] = [0; 80];
    let mut p: cptr = b"power\x00" as *const u8 as *const libc::c_char;
    let mut spell: magic_power =
        magic_power{min_lev: 0,
                    mana_cost: 0,
                    fail: 0,
                    name: 0 as *const libc::c_char,
                    desc: 0 as *const libc::c_char,};
    let mut flag: bool_ = 0;
    let mut redraw: bool_ = 0;
    /* Assume cancelled */
    *sn = -(1 as libc::c_int);
    /* Get the spell, if available */
    if repeat_pull(sn) != 0 {
        /* Verify the spell */
        if (*powers.offset(*sn as isize)).min_lev <= plev {
            /* Success */
            return 1 as libc::c_int as bool_
        }
    }
    /* Nothing chosen yet */
    flag = 0 as libc::c_int as bool_;
    /* No redraw yet */
    redraw = 0 as libc::c_int as bool_;
    /* Count number of powers that satisfies minimum plev requirement */
    i = 0 as libc::c_int;
    while i < max_powers {
        if (*powers.offset(i as isize)).min_lev <= plev { num += 1 }
        i += 1
    }
    /* Build a prompt (accept all spells) */
    strnfmt(out_val.as_mut_ptr(), 78 as libc::c_int as uint_hack,
            b"(%^ss %c-%c, *=List, ESC=exit, %c-%c=Info) Use which %s? \x00"
                as *const u8 as *const libc::c_char, p,
            0 as libc::c_int + 'a' as i32,
            num - 1 as libc::c_int + 'a' as i32,
            toupper(0 as libc::c_int + 'a' as i32),
            toupper(num - 1 as libc::c_int + 'a' as i32), p);
    /* Save the screen */
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    /* Get a spell from the user */
    while flag == 0 &&
              get_com(out_val.as_mut_ptr() as cptr, &mut choice) as
                  libc::c_int != 0 {
        /* Request redraw */
        if choice as libc::c_int == ' ' as i32 ||
               choice as libc::c_int == '*' as i32 ||
               choice as libc::c_int == '?' as i32 {
            /* Show the list */
            if redraw == 0 {
                let mut psi_desc: [libc::c_char; 80] = [0; 80];
                /* Show list */
                redraw = 1 as libc::c_int as bool_;
                /* Display a list of spells */
                prt(b"\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int, x);
                prt(b"\x00" as *const u8 as *const libc::c_char, y, x);
                put_str(b"Name\x00" as *const u8 as *const libc::c_char, y,
                        x + 5 as libc::c_int);
                put_str(b"Lv Mana Fail Info\x00" as *const u8 as
                            *const libc::c_char, y, x + 35 as libc::c_int);
                /* Dump the spells */
                i = 0 as libc::c_int;
                while i < max_powers {
                    /* Access the spell */
                    spell = *powers.offset(i as isize);
                    if spell.min_lev > plev { break ; }
                    chance = spell.fail;
                    /* Reduce failure rate by "effective" level adjustment */
                    chance -= 3 as libc::c_int * (plev - spell.min_lev);
                    /* Reduce failure rate by INT/WIS adjustment */
                    chance -=
                        3 as libc::c_int *
                            (*adj_mag_stat.as_mut_ptr().offset((*p_ptr).stat_ind[cast_stat
                                                                                     as
                                                                                     usize]
                                                                   as isize)
                                 as libc::c_int - 1 as libc::c_int);
                    /* Not enough mana to cast */
                    if spell.mana_cost > (*p_ptr).csp as libc::c_int {
                        chance +=
                            5 as libc::c_int *
                                (spell.mana_cost -
                                     (*p_ptr).csp as libc::c_int)
                    }
                    /* Extract the minimum failure rate */
                    minfail =
                        *adj_mag_fail.as_mut_ptr().offset((*p_ptr).stat_ind[cast_stat
                                                                                as
                                                                                usize]
                                                              as isize) as
                            libc::c_int;
                    /* Failure rate */
                    chance = clamp_failure_chance(chance, minfail);
                    /* Get info */
                    power_info.expect("non-null function pointer")(comment.as_mut_ptr(),
                                                                   i);
                    /* Dump the spell --(-- */
                    strnfmt(psi_desc.as_mut_ptr(),
                            80 as libc::c_int as uint_hack,
                            b"  %c) %-30s%2d %4d %3d%%%s\x00" as *const u8 as
                                *const libc::c_char, i + 'a' as i32,
                            spell.name, spell.min_lev, spell.mana_cost,
                            chance, comment.as_mut_ptr());
                    prt(psi_desc.as_mut_ptr() as cptr,
                        y + i + 1 as libc::c_int, x);
                    i += 1
                }
                /* Clear the bottom line */
                prt(b"\x00" as *const u8 as *const libc::c_char,
                    y + i + 1 as libc::c_int, x);
            } else {
                /* Hide the list */
                /* Hide list */
                redraw = 0 as libc::c_int as bool_;
                Term_load();
                character_icky = 0 as libc::c_int as bool_
            }
        } else {
            /* Restore the screen */
            /* Note verify */
            info =
                *(*__ctype_b_loc()).offset(choice as libc::c_int as isize) as
                    libc::c_int &
                    _ISupper as libc::c_int as libc::c_ushort as libc::c_int;
            /* Lowercase */
            if info != 0 {
                choice = tolower(choice as libc::c_int) as libc::c_char
            }
            /* Extract request */
            i =
                if *(*__ctype_b_loc()).offset(choice as libc::c_int as isize)
                       as libc::c_int &
                       _ISlower as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    (choice as libc::c_int) - 'a' as i32
                } else { -(1 as libc::c_int) };
            /* Totally Illegal */
            if i < 0 as libc::c_int || i >= num {
                bell();
            } else {
                /* Save the spell index */
                spell = *powers.offset(i as isize);
                /* Provides info */
                if info != 0 {
                    c_prt(14 as libc::c_int as byte_hack, spell.desc,
                          1 as libc::c_int, 0 as libc::c_int);
                    /* Restore the screen */
                    inkey();
                    Term_load();
                    character_icky = 0 as libc::c_int as bool_
                } else {
                    /* Stop the loop */
                    flag = 1 as libc::c_int as bool_
                }
            }
        }
    }
    /* Restore the screen */
    if redraw != 0 { Term_load(); }
    character_icky = 0 as libc::c_int as bool_;
    /* Abort if needed */
    if flag == 0 { return 0 as libc::c_int as bool_ }
    /* Save the choice */
    *sn = i;
    repeat_push(*sn);
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * do_cmd_cast calls this function if the player's class
 * is 'mindcrafter'.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_mindcraft() {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 0 as libc::c_int;
    let mut chance: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut minfail: libc::c_int = 0 as libc::c_int;
    let mut plev: libc::c_int = get_skill(29 as libc::c_int) as libc::c_int;
    let mut spell: magic_power =
        magic_power{min_lev: 0,
                    mana_cost: 0,
                    fail: 0,
                    name: 0 as *const libc::c_char,
                    desc: 0 as *const libc::c_char,};
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).anti_magic != 0 {
        msg_print(b"Your anti-magic shell disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* not if confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* get power */
    if get_magic_power(&mut n, mindcraft_powers.as_mut_ptr(),
                       12 as libc::c_int,
                       Some(mindcraft_info as
                                unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: libc::c_int) -> ()),
                       plev, 2 as libc::c_int) == 0 {
        return
    }
    spell = mindcraft_powers[n as usize];
    /* Verify "dangerous" spells */
    if spell.mana_cost > (*p_ptr).csp as libc::c_int {
        /* Warning */
        msg_print(b"You do not have enough mana to use this power.\x00" as
                      *const u8 as *const libc::c_char);
        /* Verify */
        if get_check(b"Attempt it anyway? \x00" as *const u8 as
                         *const libc::c_char) == 0 {
            return
        }
    }
    /* Spell failure chance */
    chance = spell.fail;
    /* Reduce failure rate by "effective" level adjustment */
    chance -=
        3 as libc::c_int *
            (get_skill(29 as libc::c_int) as libc::c_int - spell.min_lev);
    /* Reduce failure rate by INT/WIS adjustment */
    chance -=
        3 as libc::c_int *
            (*adj_mag_stat.as_mut_ptr().offset((*p_ptr).stat_ind[2 as
                                                                     libc::c_int
                                                                     as usize]
                                                   as isize) as libc::c_int -
                 1 as libc::c_int);
    /* Not enough mana to cast */
    if spell.mana_cost > (*p_ptr).csp as libc::c_int {
        chance +=
            5 as libc::c_int * (spell.mana_cost - (*p_ptr).csp as libc::c_int)
    }
    /* Extract the minimum failure rate */
    minfail =
        *adj_mag_fail.as_mut_ptr().offset((*p_ptr).stat_ind[2 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int;
    /* Failure rate */
    chance = clamp_failure_chance(chance, minfail);
    /* Failed spell */
    if Rand_div(100 as libc::c_int) < chance {
        if flush_failure != 0 { flush(); }
        msg_format(b"You failed to concentrate hard enough!\x00" as *const u8
                       as *const libc::c_char);
        sound(55 as libc::c_int);
        if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
               chance / 2 as libc::c_int {
            /* Backfire */
            b = Rand_div(100 as libc::c_int) + 1 as libc::c_int;
            if b < 5 as libc::c_int {
                msg_print(b"Oh, no! Your mind has gone blank!\x00" as
                              *const u8 as *const libc::c_char);
                lose_all_info();
            } else if b < 15 as libc::c_int {
                msg_print(b"Weird visions seem to dance before your eyes...\x00"
                              as *const u8 as *const libc::c_char);
                set_image((*p_ptr).image as libc::c_int + 5 as libc::c_int +
                              (Rand_div(10 as libc::c_int) +
                                   1 as libc::c_int));
            } else if b < 45 as libc::c_int {
                msg_print(b"Your brain is addled!\x00" as *const u8 as
                              *const libc::c_char);
                set_confused((*p_ptr).confused as libc::c_int +
                                 (Rand_div(8 as libc::c_int) +
                                      1 as libc::c_int));
            } else if b < 90 as libc::c_int {
                set_stun((*p_ptr).stun as libc::c_int +
                             (Rand_div(8 as libc::c_int) + 1 as libc::c_int));
            } else {
                /* Mana storm */
                msg_print(b"Your mind unleashes its power in an uncontrollable storm!\x00"
                              as *const u8 as *const libc::c_char);
                project(1 as libc::c_int,
                        2 as libc::c_int + plev / 10 as libc::c_int,
                        (*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, plev * 2 as libc::c_int,
                        26 as libc::c_int,
                        0x1 as libc::c_int | 0x40 as libc::c_int |
                            0x10 as libc::c_int | 0x20 as libc::c_int);
                (*p_ptr).csp =
                    if (0 as libc::c_int) <
                           (*p_ptr).csp as libc::c_int -
                               plev *
                                   (if (1 as libc::c_int) <
                                           plev / 10 as libc::c_int {
                                        (plev) / 10 as libc::c_int
                                    } else { 1 as libc::c_int }) {
                        ((*p_ptr).csp as libc::c_int) -
                            plev *
                                (if (1 as libc::c_int) <
                                        plev / 10 as libc::c_int {
                                     (plev) / 10 as libc::c_int
                                 } else { 1 as libc::c_int })
                    } else { 0 as libc::c_int } as s16b
            }
        }
    } else {
        /* Successful spells */
        sound(12 as libc::c_int);
        /* spell code */
        match n {
            0 => {
                /* Precog */
                /* Magic mapping */
                if plev > 44 as libc::c_int {
                    wiz_lite();
                } else if plev > 19 as libc::c_int { map_area(); }
                /* Detection */
                if plev < 30 as libc::c_int {
                    b =
                        detect_monsters_normal(25 as libc::c_int) as
                            libc::c_int;
                    if plev > 14 as libc::c_int {
                        b |=
                            detect_monsters_invis(25 as libc::c_int) as
                                libc::c_int
                    }
                    if plev > 4 as libc::c_int {
                        b |= detect_traps(25 as libc::c_int) as libc::c_int
                    }
                } else { b = detect_all(25 as libc::c_int) as libc::c_int }
                /* Telepathy */
                if plev > 24 as libc::c_int {
                    set_tim_esp((*p_ptr).tim_esp as libc::c_int + plev);
                    /* If plvl >= 40, we should have permanent ESP */
                }
                if b == 0 {
                    msg_print(b"You feel safe.\x00" as *const u8 as
                                  *const libc::c_char);
                }
            }
            1 => {
                /* Mindblast */
                if get_aim_dir(&mut dir) == 0 { return }
                if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                       plev * 2 as libc::c_int {
                    fire_beam(85 as libc::c_int, dir,
                              damroll((3 as libc::c_int +
                                           (plev - 1 as libc::c_int) /
                                               4 as libc::c_int) as s16b,
                                      (3 as libc::c_int +
                                           plev / 15 as libc::c_int) as
                                          s16b));
                } else {
                    fire_ball(85 as libc::c_int, dir,
                              damroll((3 as libc::c_int +
                                           (plev - 1 as libc::c_int) /
                                               4 as libc::c_int) as s16b,
                                      (3 as libc::c_int +
                                           plev / 15 as libc::c_int) as s16b),
                              0 as libc::c_int);
                }
            }
            2 => {
                /* Minor displace */
                if plev < 25 as libc::c_int {
                    teleport_player(10 as libc::c_int);
                } else {
                    let mut ii: libc::c_int = 0;
                    let mut ij: libc::c_int = 0;
                    if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long !=
                           0 {
                        msg_print(b"Not on special levels!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        msg_print(b"You open a Void Jumpgate. Choose a destination.\x00"
                                      as *const u8 as *const libc::c_char);
                        if tgt_pt(&mut ii, &mut ij) == 0 { return }
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
                                   plev + 2 as libc::c_int +
                                       (*p_ptr).to_s as libc::c_int *
                                           3 as libc::c_int ||
                               Rand_div(plev * plev / 2 as libc::c_int) ==
                                   0 as libc::c_int {
                            msg_print(b"You fail to exit the void correctly!\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            (*p_ptr).energy -= 100 as libc::c_int;
                            get_pos_player(10 as libc::c_int +
                                               (*p_ptr).to_s as libc::c_int /
                                                   2 as libc::c_int, &mut ij,
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
                                      8 as libc::c_int)) as s16b
                    }
                }
            }
            3 => {
                /* Major displace */
                if plev > 29 as libc::c_int { banish_monsters(plev); }
                teleport_player(plev * 5 as libc::c_int);
            }
            4 => {
                /* Domination */
                if plev < 30 as libc::c_int {
                    if get_aim_dir(&mut dir) == 0 { return }
                    fire_ball(89 as libc::c_int, dir, plev, 0 as libc::c_int);
                } else { charm_monsters(plev * 2 as libc::c_int); }
            }
            5 => {
                /* Fist of Force  ---  not 'true' TK  */
                if get_aim_dir(&mut dir) == 0 { return }
                fire_ball(21 as libc::c_int, dir,
                          damroll((8 as libc::c_int +
                                       (plev - 5 as libc::c_int) /
                                           4 as libc::c_int) as s16b,
                                  8 as libc::c_int as s16b),
                          if plev > 20 as libc::c_int {
                              ((plev - 20 as libc::c_int) / 8 as libc::c_int)
                                  + 1 as libc::c_int
                          } else { 0 as libc::c_int });
            }
            6 => {
                /* Character Armour */
                set_shield((*p_ptr).shield as libc::c_int + plev, plev,
                           0 as libc::c_int as s16b, 0 as libc::c_int as s16b,
                           0 as libc::c_int as s16b);
                if plev > 14 as libc::c_int {
                    set_oppose_acid((*p_ptr).oppose_acid as libc::c_int +
                                        plev);
                }
                if plev > 19 as libc::c_int {
                    set_oppose_fire((*p_ptr).oppose_fire as libc::c_int +
                                        plev);
                }
                if plev > 24 as libc::c_int {
                    set_oppose_cold((*p_ptr).oppose_cold as libc::c_int +
                                        plev);
                }
                if plev > 29 as libc::c_int {
                    set_oppose_elec((*p_ptr).oppose_elec as libc::c_int +
                                        plev);
                }
                if plev > 34 as libc::c_int {
                    set_oppose_pois((*p_ptr).oppose_pois as libc::c_int +
                                        plev);
                }
            }
            7 => {
                /* Psychometry */
                ident_spell();
            }
            8 => {
                /* Mindwave */
                msg_print(b"Mind-warping forces emanate from your brain!\x00"
                              as *const u8 as *const libc::c_char);
                if plev < 25 as libc::c_int {
                    project(0 as libc::c_int,
                            2 as libc::c_int + plev / 10 as libc::c_int,
                            (*p_ptr).py as libc::c_int,
                            (*p_ptr).px as libc::c_int,
                            plev * 3 as libc::c_int / 2 as libc::c_int,
                            85 as libc::c_int, 0x40 as libc::c_int);
                } else {
                    mindblast_monsters(plev *
                                           ((plev - 5 as libc::c_int) /
                                                10 as libc::c_int +
                                                1 as libc::c_int));
                }
            }
            9 => {
                /* Adrenaline */
                set_afraid(0 as libc::c_int);
                set_stun(0 as libc::c_int);
                hp_player(plev);
                b =
                    10 as libc::c_int +
                        (Rand_div(plev * 3 as libc::c_int / 2 as libc::c_int)
                             + 1 as libc::c_int);
                if plev < 35 as libc::c_int {
                    set_hero((*p_ptr).hero as libc::c_int + b);
                } else { set_shero((*p_ptr).shero as libc::c_int + b); }
                if (*p_ptr).fast == 0 {
                    /* Haste */
                    set_fast(b, plev / 5 as libc::c_int);
                } else {
                    set_fast((*p_ptr).fast as libc::c_int + b,
                             plev / 5 as libc::c_int);
                }
            }
            10 => {
                /* Psychic Drain */
                if get_aim_dir(&mut dir) == 0 { return }
                b =
                    damroll((plev / 2 as libc::c_int) as s16b,
                            6 as libc::c_int as s16b);
                if fire_ball(86 as libc::c_int, dir, b,
                             0 as libc::c_int +
                                 (plev - 25 as libc::c_int) /
                                     10 as libc::c_int) != 0 {
                    (*p_ptr).energy -=
                        Rand_div(150 as libc::c_int) + 1 as libc::c_int
                }
            }
            11 => {
                /* Telekinesis */
                msg_print(b"A wave of pure physical force radiates out from your body!\x00"
                              as *const u8 as *const libc::c_char);
                project(0 as libc::c_int,
                        3 as libc::c_int + plev / 10 as libc::c_int,
                        (*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int,
                        plev *
                            (if plev > 39 as libc::c_int {
                                 4 as libc::c_int
                             } else { 3 as libc::c_int }), 87 as libc::c_int,
                        0x40 as libc::c_int | 0x20 as libc::c_int |
                            0x10 as libc::c_int);
            }
            _ => {
                msg_print(b"Zap?\x00" as *const u8 as *const libc::c_char);
            }
        }
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Sufficient mana */
    if spell.mana_cost <= (*p_ptr).csp as libc::c_int {
        /* Use some mana */
        (*p_ptr).csp = ((*p_ptr).csp as libc::c_int - spell.mana_cost) as s16b
    } else {
        /* Over-exert the player */
        let mut oops: libc::c_int =
            spell.mana_cost - (*p_ptr).csp as libc::c_int;
        /* No mana left */
        (*p_ptr).csp = 0 as libc::c_int as s16b;
        (*p_ptr).csp_frac = 0 as libc::c_int as u16b;
        /* Message */
        msg_print(b"You faint from the effort!\x00" as *const u8 as
                      *const libc::c_char);
        /* Hack -- Bypass free action */
        set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                          (Rand_div(5 as libc::c_int * oops +
                                        1 as libc::c_int) +
                               1 as libc::c_int));
        /* Damage WIS (possibly permanently) */
        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
            let mut perm: bool_ =
                (Rand_div(100 as libc::c_int) < 25 as libc::c_int) as
                    libc::c_int as bool_;
            /* Message */
            msg_print(b"You have damaged your mind!\x00" as *const u8 as
                          *const libc::c_char);
            /* Reduce constitution */
            dec_stat(2 as libc::c_int,
                     15 as libc::c_int +
                         (Rand_div(10 as libc::c_int) + 1 as libc::c_int),
                     perm as libc::c_int);
        }
    }
    /* Redraw mana */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
}
unsafe extern "C" fn get_mimic_chance(mut mimic: libc::c_int) -> libc::c_int {
    let mut chance: s32b = 0;
    call_lua(b"get_mimic_info\x00" as *const u8 as *const libc::c_char,
             b"(d,s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char, mimic,
             b"level\x00" as *const u8 as *const libc::c_char,
             &mut chance as *mut s32b);
    chance *= 3 as libc::c_int;
    chance -=
        get_skill_scale(32 as libc::c_int, 150 as libc::c_int as u32b) as
            libc::c_int;
    chance -=
        3 as libc::c_int *
            *adj_mag_stat.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int;
    /* Return the chance */
    return clamp_failure_chance(chance, 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_mimic_lore() {
    let mut fail: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Player has to be able to see */
    if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0 {
        msg_print(b"You cannot see!\x00" as *const u8 as *const libc::c_char);
        return
    }
    /* No transformations when confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Already in a mimic form -- Allow cancelling */
    if (*p_ptr).mimic_form != 0 {
        msg_print(b"You morph back to your natural form!\x00" as *const u8 as
                      *const libc::c_char);
        set_mimic(0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    } else {
        /* Not in mimic forms -- Allow transformations */
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(38 as libc::c_int as
                                                             isize) as
                *mut object_type;
        if (*o_ptr).tval as libc::c_int != 35 as libc::c_int ||
               (*o_ptr).sval as libc::c_int != 100 as libc::c_int {
            msg_print(b"You are not wearing any cloaks of mimicry.\x00" as
                          *const u8 as *const libc::c_char);
            return
        }
        /* Calculate failure rate */
        fail = get_mimic_chance((*o_ptr).pval2 as libc::c_int);
        if fail > 75 as libc::c_int {
            msg_print(b"You feel uneasy with this shape-change.\x00" as
                          *const u8 as *const libc::c_char);
            if get_check(b"Try it anyway? \x00" as *const u8 as
                             *const libc::c_char) == 0 {
                return
            }
        }
        /* Fumble */
        if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) < fail {
            msg_print(b"Your shape-change goes horribly wrong!\x00" as
                          *const u8 as *const libc::c_char);
            if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                   (*p_ptr).skill_sav as libc::c_int {
                msg_print(b"You manage to wrest your body back under control.\x00"
                              as *const u8 as *const libc::c_char);
                return
            }
            set_mimic(30 as libc::c_int,
                      resolve_mimic_name(b"Abomination\x00" as *const u8 as
                                             *const libc::c_char),
                      get_skill(32 as libc::c_int) as libc::c_int);
        } else {
            /* Success */
            set_mimic((*k_info.offset((*o_ptr).k_idx as isize)).pval2 +
                          get_skill_scale(32 as libc::c_int,
                                          1000 as libc::c_int as u32b) as
                              libc::c_int, (*o_ptr).pval2 as libc::c_int,
                      get_skill(32 as libc::c_int) as libc::c_int);
        }
    }
    /* Redraw title */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x2 as libc::c_long) as u32b;
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
}
unsafe extern "C" fn mimic_forbid_travel(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut value: u32b = (*p_ptr).mimic_extra >> 16 as libc::c_int;
    let mut att: u32b =
        (*p_ptr).mimic_extra & 0xffff as libc::c_int as libc::c_uint;
    if value > 0 as libc::c_int as libc::c_uint &&
           (att & 0x40 as libc::c_int as libc::c_uint != 0 ||
                att & 0x20 as libc::c_int as libc::c_uint != 0) {
        msg_print(b"You had best not travel with your extra limbs.\x00" as
                      *const u8 as *const libc::c_char);
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
/*
 * do_cmd_cast calls this function if the player's class
 * is 'mimic'.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_mimic() {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 0 as libc::c_int;
    let mut fail: libc::c_int = 0;
    let mut minfail: libc::c_int = 0 as libc::c_int;
    let mut plev: libc::c_int = get_skill(32 as libc::c_int) as libc::c_int;
    let mut spell: magic_power =
        magic_power{min_lev: 0,
                    mana_cost: 0,
                    fail: 0,
                    name: 0 as *const libc::c_char,
                    desc: 0 as *const libc::c_char,};
    static mut added_hooks: bool_ = 0 as libc::c_int as bool_;
    if added_hooks == 0 {
        add_hook(75 as libc::c_int,
                 Some(mimic_forbid_travel as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"mimic_forbid_travel\x00" as *const u8 as
                     *const libc::c_char);
        added_hooks = 1 as libc::c_int as bool_
    }
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).anti_magic != 0 {
        msg_print(b"Your anti-magic shell disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* not if confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* get power */
    if get_magic_power(&mut n, mimic_powers.as_mut_ptr(), 5 as libc::c_int,
                       Some(mimic_info as
                                unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: libc::c_int) -> ()),
                       plev, 3 as libc::c_int) == 0 {
        return
    }
    spell = mimic_powers[n as usize];
    /* Verify "dangerous" spells */
    if spell.mana_cost > (*p_ptr).csp as libc::c_int {
        /* Warning */
        msg_print(b"You do not have enough mana to use this power.\x00" as
                      *const u8 as *const libc::c_char);
        /* Verify */
        if get_check(b"Attempt it anyway? \x00" as *const u8 as
                         *const libc::c_char) == 0 {
            return
        }
    }
    /* Spell failure chance */
    fail = spell.fail;
    /* Reduce failure rate by "effective" level adjustment */
    fail -= 3 as libc::c_int * (plev - spell.min_lev);
    /* Reduce failure rate by INT/WIS adjustment */
    fail -=
        3 as libc::c_int *
            (*adj_mag_stat.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                     libc::c_int
                                                                     as usize]
                                                   as isize) as libc::c_int -
                 1 as libc::c_int);
    /* Not enough mana to cast */
    if spell.mana_cost > (*p_ptr).csp as libc::c_int {
        fail +=
            5 as libc::c_int * (spell.mana_cost - (*p_ptr).csp as libc::c_int)
    }
    /* Extract the minimum failure rate */
    minfail =
        *adj_mag_fail.as_mut_ptr().offset((*p_ptr).stat_ind[3 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int;
    /* Minimum failure rate */
    if fail < minfail { fail = minfail }
    /* Stunning makes spells harder */
    if (*p_ptr).stun as libc::c_int > 50 as libc::c_int {
        fail += 25 as libc::c_int
    } else if (*p_ptr).stun != 0 { fail += 15 as libc::c_int }
    /* Always a 5 percent chance of working */
    if fail > 95 as libc::c_int { fail = 95 as libc::c_int }
    /* Failed spell */
    if Rand_div(100 as libc::c_int) < fail {
        if flush_failure != 0 { flush(); }
        msg_format(b"You failed to concentrate hard enough!\x00" as *const u8
                       as *const libc::c_char);
        sound(55 as libc::c_int);
        if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
               fail / 2 as libc::c_int {
            /* Backfire */
            b = Rand_div(100 as libc::c_int) + 1 as libc::c_int;
            if b < 5 as libc::c_int {
                msg_print(b"Oh, no! Your mind has gone blank!\x00" as
                              *const u8 as *const libc::c_char);
                lose_all_info();
            } else if b < 15 as libc::c_int {
                msg_print(b"Weird visions seem to dance before your eyes...\x00"
                              as *const u8 as *const libc::c_char);
                set_image((*p_ptr).image as libc::c_int + 5 as libc::c_int +
                              (Rand_div(10 as libc::c_int) +
                                   1 as libc::c_int));
            } else if b < 45 as libc::c_int {
                msg_print(b"Your brain is addled!\x00" as *const u8 as
                              *const libc::c_char);
                set_confused((*p_ptr).confused as libc::c_int +
                                 (Rand_div(8 as libc::c_int) +
                                      1 as libc::c_int));
            } else {
                set_stun((*p_ptr).stun as libc::c_int +
                             (Rand_div(8 as libc::c_int) + 1 as libc::c_int));
            }
        }
    } else {
        /* Successful spells */
        sound(12 as libc::c_int);
        /* spell code */
        match n {
            0 => {
                /* Mimic */
                do_cmd_mimic_lore();
            }
            1 => {
                /* Invisibility */
                let mut ii: libc::c_int =
                    10 as libc::c_int + plev +
                        (Rand_div(20 as libc::c_int) + 1 as libc::c_int) +
                        (*p_ptr).to_s as libc::c_int;
                set_invis((*p_ptr).tim_invisible as libc::c_int + ii,
                          50 as libc::c_int);
                set_tim_invis((*p_ptr).tim_invisible as libc::c_int + ii);
            }
            2 => {
                /* Extract the value and the flags */
                let mut value: u32b =
                    (*p_ptr).mimic_extra >> 16 as libc::c_int;
                let mut att: u32b =
                    (*p_ptr).mimic_extra &
                        0xffff as libc::c_int as libc::c_uint;
                /* Clear useless things */
                att &= !(0x40 as libc::c_int) as libc::c_uint;
                att &= !(0x80 as libc::c_int) as libc::c_uint;
                if att & 0x20 as libc::c_int as libc::c_uint != 0 {
                    value =
                        (value as
                             libc::c_uint).wrapping_add((50 as libc::c_int +
                                                             (Rand_div(50 as
                                                                           libc::c_int
                                                                           +
                                                                           2
                                                                               as
                                                                               libc::c_int
                                                                               *
                                                                               plev)
                                                                  +
                                                                  1 as
                                                                      libc::c_int))
                                                            as libc::c_uint)
                            as u32b as u32b
                } else {
                    msg_print(b"You mimic a new pair of legs.\x00" as
                                  *const u8 as *const libc::c_char);
                    value =
                        (50 as libc::c_int +
                             (Rand_div(50 as libc::c_int +
                                           2 as libc::c_int * plev) +
                                  1 as libc::c_int)) as u32b;
                    att |= 0x20 as libc::c_int as libc::c_uint
                }
                if value > 10000 as libc::c_int as libc::c_uint {
                    value = 10000 as libc::c_int as u32b
                }
                (*p_ptr).mimic_extra =
                    att.wrapping_add(value << 16 as libc::c_int);
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long | 0x4 as libc::c_long) as
                        u32b
            }
            3 => {
                /* Extract the value and the flags */
                let mut value_0: u32b =
                    (*p_ptr).mimic_extra >> 16 as libc::c_int;
                let mut att_0: u32b =
                    (*p_ptr).mimic_extra &
                        0xffff as libc::c_int as libc::c_uint;
                /* Clear useless things */
                att_0 &= !(0x40 as libc::c_int) as libc::c_uint;
                att_0 &= !(0x20 as libc::c_int) as libc::c_uint;
                if att_0 & 0x80 as libc::c_int as libc::c_uint != 0 {
                    value_0 =
                        (value_0 as
                             libc::c_uint).wrapping_add((50 as libc::c_int +
                                                             (Rand_div(50 as
                                                                           libc::c_int
                                                                           +
                                                                           2
                                                                               as
                                                                               libc::c_int
                                                                               *
                                                                               plev)
                                                                  +
                                                                  1 as
                                                                      libc::c_int))
                                                            as libc::c_uint)
                            as u32b as u32b
                } else {
                    msg_print(b"You grow an affinity for walls.\x00" as
                                  *const u8 as *const libc::c_char);
                    value_0 =
                        (50 as libc::c_int +
                             (Rand_div(50 as libc::c_int +
                                           2 as libc::c_int * plev) +
                                  1 as libc::c_int)) as u32b;
                    att_0 |= 0x80 as libc::c_int as libc::c_uint
                }
                if value_0 > 10000 as libc::c_int as libc::c_uint {
                    value_0 = 10000 as libc::c_int as u32b
                }
                (*p_ptr).mimic_extra =
                    att_0.wrapping_add(value_0 << 16 as libc::c_int);
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long | 0x4 as libc::c_long) as
                        u32b
            }
            4 => {
                /* Arms Mimicry */
                /* Extract the value and the flags */
                let mut value_1: u32b =
                    (*p_ptr).mimic_extra >> 16 as libc::c_int;
                let mut att_1: u32b =
                    (*p_ptr).mimic_extra &
                        0xffff as libc::c_int as libc::c_uint;
                /* Clear useless things */
                att_1 &= !(0x20 as libc::c_int) as libc::c_uint;
                att_1 &= !(0x80 as libc::c_int) as libc::c_uint;
                if att_1 & 0x40 as libc::c_int as libc::c_uint != 0 {
                    value_1 =
                        (value_1 as
                             libc::c_uint).wrapping_add((50 as libc::c_int +
                                                             (Rand_div(50 as
                                                                           libc::c_int
                                                                           +
                                                                           2
                                                                               as
                                                                               libc::c_int
                                                                               *
                                                                               plev)
                                                                  +
                                                                  1 as
                                                                      libc::c_int))
                                                            as libc::c_uint)
                            as u32b as u32b
                } else {
                    msg_print(b"You mimic a new pair of arms.\x00" as
                                  *const u8 as *const libc::c_char);
                    value_1 =
                        (50 as libc::c_int +
                             (Rand_div(50 as libc::c_int +
                                           2 as libc::c_int * plev) +
                                  1 as libc::c_int)) as u32b;
                    att_1 |= 0x40 as libc::c_int as libc::c_uint
                }
                if value_1 > 10000 as libc::c_int as libc::c_uint {
                    value_1 = 10000 as libc::c_int as u32b
                }
                (*p_ptr).mimic_extra =
                    att_1.wrapping_add(value_1 << 16 as libc::c_int);
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long | 0x4 as libc::c_long) as
                        u32b
            }
            _ => {
                msg_print(b"Zap?\x00" as *const u8 as *const libc::c_char);
            }
        }
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Sufficient mana */
    if spell.mana_cost <= (*p_ptr).csp as libc::c_int {
        /* Use some mana */
        (*p_ptr).csp = ((*p_ptr).csp as libc::c_int - spell.mana_cost) as s16b
    } else {
        /* Over-exert the player */
        let mut oops: libc::c_int =
            spell.mana_cost - (*p_ptr).csp as libc::c_int;
        /* No mana left */
        (*p_ptr).csp = 0 as libc::c_int as s16b;
        (*p_ptr).csp_frac = 0 as libc::c_int as u16b;
        /* Message */
        msg_print(b"You faint from the effort!\x00" as *const u8 as
                      *const libc::c_char);
        /* Hack -- Bypass free action */
        set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                          (Rand_div(5 as libc::c_int * oops +
                                        1 as libc::c_int) +
                               1 as libc::c_int));
        /* Damage WIS (possibly permanently) */
        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
            let mut perm: bool_ =
                (Rand_div(100 as libc::c_int) < 25 as libc::c_int) as
                    libc::c_int as bool_;
            /* Message */
            msg_print(b"You have damaged your mind!\x00" as *const u8 as
                          *const libc::c_char);
            /* Reduce constitution */
            dec_stat(3 as libc::c_int,
                     15 as libc::c_int +
                         (Rand_div(10 as libc::c_int) + 1 as libc::c_int),
                     perm as libc::c_int);
        }
    }
    /* Redraw mana */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
}
/*
 * do_cmd_cast calls this function if the player's class
 * is 'beastmaster'.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_beastmaster() {
    let mut plev: libc::c_int = (*p_ptr).lev as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    /* Process the monsters (backwards) */
    num = 0 as libc::c_int;
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        /* Access the monster */
        m_ptr = &mut *m_list.offset(i as isize) as *mut monster_type;
        if (*m_ptr).status as libc::c_int == 3 as libc::c_int { num += 1 }
        i -= 1
    }
    if num < plev * 2 as libc::c_int {
        /* XXX XXX */
        if Rand_div(80 as libc::c_int - plev -
                        (*p_ptr).stat_use[5 as libc::c_int as usize] as
                            libc::c_int - (*p_ptr).to_s as libc::c_int) <
               20 as libc::c_int {
            summon_specific_friendly((*p_ptr).py as libc::c_int,
                                     (*p_ptr).px as libc::c_int, plev,
                                     Rand_div(plev / 2 as libc::c_int),
                                     0 as libc::c_int as bool_);
        }
    } else {
        msg_print(b"You can\'t summon more pets\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Take a turn */
    if is_magestaff() != 0 {
        energy_use = 80 as libc::c_int
    } else { energy_use = 100 as libc::c_int }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
}
/*
 * Set of variables and functions to create an artifact
 */
/* LOG2 is a constant (compile-time) method of converting a single
 * set bit into a number. Works well, but for variable (runtime)
 * expressions, use a loop instead.. much smaller code*/
#[no_mangle]
pub static mut flags_select: [libc::c_int; 160] = [0; 160];
#[no_mangle]
pub static mut activation_select: libc::c_int = 0;
/* Return true if the player is wielding the philosopher's stone
 */
#[no_mangle]
pub unsafe extern "C" fn alchemist_has_stone() -> bool_ {
    if (*p_ptr).inventory[36 as libc::c_int as usize].name1 as libc::c_int ==
           209 as libc::c_int {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/*
 Display a group of flags from a_select flags, and return
 the number of flags displayed (even invisible ones)
 */
#[no_mangle]
pub unsafe extern "C" fn show_flags(mut group: byte_hack,
                                    mut pval: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0; /* Adjust - no zero group */
    let mut x: libc::c_int = 0;
    let mut color: libc::c_int = 1 as libc::c_int;
    let mut items: libc::c_int = 0 as libc::c_int;
    let mut ttt: [libc::c_char; 80] = [0; 80];
    Term_clear();
    group = group.wrapping_add(1);
    i = 0 as libc::c_int;
    while (*a_select_flags.offset(i as isize)).group != 0 {
        if !((*a_select_flags.offset(i as isize)).group as libc::c_int !=
                 group as libc::c_int) {
            if (*a_select_flags.offset(i as isize)).xp ==
                   0 as libc::c_int as libc::c_uint {
                break ;
            }
            sprintf(ttt.as_mut_ptr(),
                    b"%c) %s\x00" as *const u8 as *const libc::c_char,
                    if items < 26 as libc::c_int {
                        (items) + 'a' as i32
                    } else { ('0' as i32 + items) - 26 as libc::c_int },
                    al_name.offset((*a_select_flags.offset(i as isize)).desc
                                       as isize));
            if wizard as libc::c_int != 0 ||
                   alchemist_has_stone() as libc::c_int != 0 {
                sprintf(ttt.as_mut_ptr(),
                        b"%c) %s (exp %ld)\x00" as *const u8 as
                            *const libc::c_char,
                        if items < 26 as libc::c_int {
                            (items) + 'a' as i32
                        } else { ('0' as i32 + items) - 26 as libc::c_int },
                        al_name.offset((*a_select_flags.offset(i as
                                                                   isize)).desc
                                           as isize),
                        (*a_select_flags.offset(i as isize)).xp as
                            libc::c_long);
            }
            /* Note: Somebody is VERY clever, and it wasn't me. Text printed as
			 * TERM_DARK is actually printed as TERM_BLUE *SPACES* to prevent the
			 * player from using a 'cut-and-paste' enabled terminal to see
			 * what he shouldn't.  Thus, simply setting the color to TERM_DARK
			 * will entirely prevent the unspoiled player from knowing that it's
			 * even possible. */
            match flags_select[i as usize] {
                1 => {
                    color = 11 as libc::c_int
                    /* Just in Case*/
                }
                0 => { color = 1 as libc::c_int }
                -1 => { color = 13 as libc::c_int }
                -2 => { color = 0 as libc::c_int }
                -3 => { color = 4 as libc::c_int }
                -4 => { color = 8 as libc::c_int }
                _ => { color = 0 as libc::c_int }
            }
            /* For alchemists who have the stone, at least show all the flags... */
            if (alchemist_has_stone() as libc::c_int != 0 ||
                    wizard as libc::c_int != 0) && color == 0 as libc::c_int {
                color = 6 as libc::c_int
            }
            if items < 16 as libc::c_int {
                x = 5 as libc::c_int
            } else { x = 45 as libc::c_int }
            c_prt(color as byte_hack, ttt.as_mut_ptr() as cptr,
                  (if items < 16 as libc::c_int {
                       items
                   } else { (items) - 16 as libc::c_int }) + 5 as libc::c_int,
                  x);
            items += 1
        }
        i += 1
    }
    return items;
}
#[no_mangle]
pub unsafe extern "C" fn show_levels() {
    Term_clear();
    c_prt(1 as libc::c_int as byte_hack,
          b"[a] Stats, sustains, luck, speed, vision, etc.          \x00" as
              *const u8 as *const libc::c_char, 3 as libc::c_int,
          10 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          b"[b] Misc. (Auras, light, see invis, etc)                \x00" as
              *const u8 as *const libc::c_char, 4 as libc::c_int,
          10 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          b"[c] Weapon Branding                                     \x00" as
              *const u8 as *const libc::c_char, 5 as libc::c_int,
          10 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          b"[d] Resistances and Immunities                          \x00" as
              *const u8 as *const libc::c_char, 6 as libc::c_int,
          10 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          b"[e] ESP and Curses                                      \x00" as
              *const u8 as *const libc::c_char, 7 as libc::c_int,
          10 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          b"[f] Activation                                          \x00" as
              *const u8 as *const libc::c_char, 8 as libc::c_int,
          10 as libc::c_int);
    c_prt(0 as libc::c_int as byte_hack,
          b"[g] Abilities Gained                                    \x00" as
              *const u8 as *const libc::c_char, 9 as libc::c_int,
          10 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          b"[h] Display Required Essences and items                 \x00" as
              *const u8 as *const libc::c_char, 10 as libc::c_int,
          10 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          b"[i] Done! Finalize and commit changes.                  \x00" as
              *const u8 as *const libc::c_char, 11 as libc::c_int,
          10 as libc::c_int);
    /*No need to return anything - if the valid selections change, it'll be a code level change.*/
}
#[no_mangle]
pub unsafe extern "C" fn get_flags_exp(mut pval: libc::c_int,
                                       mut oldpval: libc::c_int) -> s32b {
    let mut i: libc::c_int = 0;
    let mut exp: s32b = 0 as libc::c_int;
    let mut current_block_12: u64;
    i = 0 as libc::c_int;
    while (*a_select_flags.offset(i as isize)).group != 0 {
        if (*a_select_flags.offset(i as isize)).xp ==
               0 as libc::c_int as libc::c_uint {
            break ;
        }
        if (*a_select_flags.offset(i as isize)).group as libc::c_int <=
               5 as libc::c_int && flags_select[i as usize] != 0 {
            let mut xp: s32b =
                (*a_select_flags.offset(i as isize)).xp as s32b;
            let mut factor: libc::c_int = 1 as libc::c_int;
            let mut oldfactor: libc::c_int = 0 as libc::c_int;
            /* don't even look at flags which the user can't set
				 * because they also can't change the pval when a pval-
				 * dependant flag is set, flags which they can't set
				 * cannot effect the exp in any way, whether their set or not
				 */
            if flags_select[i as usize] < -(1 as libc::c_int) {
                current_block_12 = 16559507199688588974;
            } else {
                if flags_select[i as usize] == -(1 as libc::c_int) {
                    oldfactor = 1 as libc::c_int
                }
                if (*a_select_flags.offset(i as isize)).pval != 0 {
                    /* (1/4)x^2 + x
					 * I wanted something smaller than x^2 or x^x
					 * this is because although a ring of speed +10 is
					 * more than 10 times better than a ring of speed +1,
					 * I don't think it's 100 times better. More like 30.
					 * this function yields:
					 * 1=1 * 2=3 * 3=5 * 4=8 * 5=11 * 6=15 * 7=21
					 * 8=24 * 9=29 * 10=35 * 11=41 * 12=48 * 13=55
					 * 14=63 * 15=71 * 20=120 * 25=181 * 30=255
					 * which I think is acceptable.
					 * briefly, to get a +30 speed ring, it would be:
					 * 255*50000 or over 12 million experience
					 * points. For reference, a level 50 human requires
					 * 5 million xp. I'm sure it's doable, but it'd be
					 * *HARD*
					 * a speed+10 artifact would require 1.75 million.
					 * much more doable, but not too easily.
					 */
                    factor = pval * pval / 4 as libc::c_int + pval;
                    if flags_select[i as usize] == -(1 as libc::c_int) {
                        oldfactor =
                            oldpval * oldpval / 4 as libc::c_int + oldpval
                    }
                }
                exp += xp * factor - xp * oldfactor;
                current_block_12 = 12039483399334584727;
            }
        } else { current_block_12 = 12039483399334584727; }
        match current_block_12 {
            12039483399334584727 => {
                if (*a_select_flags.offset(i as isize)).group as libc::c_int
                       == 88 as libc::c_int &&
                       (*a_select_flags.offset(i as isize)).flag ==
                           -activation_select {
                    exp =
                        (exp as
                             libc::c_uint).wrapping_add((*a_select_flags.offset(i
                                                                                    as
                                                                                    isize)).xp)
                            as s32b as s32b
                }
            }
            _ => { }
        }
        i += 1
    }
    if alchemist_has_stone() != 0 { exp = exp / 4 as libc::c_int }
    return exp;
}
/* returns the 'real quantity' of items needed to empower
 * a particular flag to a particular pval.
 * Note that this routine returns zero for any flag that
 * doesn't require some sort of action.
 */
#[no_mangle]
pub unsafe extern "C" fn calc_rqty(mut i: libc::c_int, mut pval: libc::c_int,
                                   mut oldpval: libc::c_int) -> libc::c_int {
    /* return 0 if flag is greater than size of flags_select && ! activation */
    if (*a_select_flags.offset(i as isize)).group as libc::c_int >
           5 as libc::c_int {
        if activation_select == (*a_select_flags.offset(i as isize)).flag {
            return 1 as libc::c_int
        } else { return 0 as libc::c_int }
    }
    /* return 0 if the flag wasn't set */
    if flags_select[i as usize] < -(1 as libc::c_int) ||
           flags_select[i as usize] == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    /* Return change in pval if the flag was already set */
    if flags_select[i as usize] == -(1 as libc::c_int) &&
           (*a_select_flags.offset(i as isize)).pval as libc::c_int != 0 {
        return pval - oldpval
    } else {
        /* Return pval if the flag will be set this time */
        if (*a_select_flags.offset(i as isize)).pval != 0 {
            return pval
        } else {
            /* Return 0 if the flag is unknown */
            if flags_select[i as usize] == -(1 as libc::c_int) {
                return 0 as libc::c_int
            }
        }
    }
    return 1 as libc::c_int;
}
/* Handle the various items that creating artifacts requires.
 * Mode = 0 to print a description,
 * 1 to use up the items
 * -1 to check to see if the items exist
 * Note that this function is called ONLY from the
 * other artifact item helper function.
 */
#[no_mangle]
pub unsafe extern "C" fn check_artifact_items(mut pval: libc::c_int,
                                              mut oldpval: libc::c_int,
                                              mut mode: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut row: libc::c_int = 1 as libc::c_int;
    let mut col: libc::c_int = 15 as libc::c_int;
    let mut rqty: libc::c_int = 0;
    let mut orqty: libc::c_int = 0;
    let mut trqty: libc::c_int = 0;
    let mut good: bool_ = 1 as libc::c_int as bool_;
    let mut temporary: libc::c_int = -(1 as libc::c_int);
    let mut ch: libc::c_char = 0;
    /* For temporary items, waive the item requirements,
	 * except for the corpse... */
    j = 0 as libc::c_int;
    while (*a_select_flags.offset(j as isize)).group != 0 {
        if (*a_select_flags.offset(j as isize)).flag ==
               4 as libc::c_int * 32 as libc::c_int &&
               flags_select[j as usize] == 1 as libc::c_int {
            temporary = j
        }
        j += 1
    }
    /* Check for enough items */
    i =
        0 as
            libc::c_int; /* End of group associated with this a_select_flags entry */
    while (*a_select_flags.offset(i as isize)).group != 0 {
        /* For temporary items, ignore
		 everything except the one item
		 */
        if !(temporary != -(1 as libc::c_int) && i != temporary) {
            /* Calc quantity is done per flag, because
		 some have a pval, some don't, some where already
		 set at pval=2, etc
		 */
            orqty = calc_rqty(i, pval, oldpval);
            rqty = orqty;
            /* If no item is associated with this flag,
		 or this flag wasn't set or didn't change */
            if !((*a_select_flags.offset(i as isize)).rtval == 0 || rqty == 0)
               {
                let mut current_block_34:
                        u64; /*end of looping through the p_ptr->inventory*/
                k = 0 as libc::c_int;
                while k < 24 as libc::c_int {
                    let mut o_ptr: *mut object_type =
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset(k as
                                                                         isize)
                            as *mut object_type;
                    /* if p_ptr->inventory item is acceptable */
                    /* Note here that an rsval of -1 (which is read is 0xff
			 for a byte..) matches anything. */
                    if (*o_ptr).tval as libc::c_int ==
                           (*a_select_flags.offset(i as isize)).rtval as
                               libc::c_int &&
                           ((*o_ptr).sval as libc::c_int ==
                                (*a_select_flags.offset(i as isize)).rsval as
                                    libc::c_int ||
                                (*a_select_flags.offset(i as isize)).rsval as
                                    libc::c_int ==
                                    -(1 as libc::c_int) as byte_hack as
                                        libc::c_int) {
                        /* Corpse validation is COMPLICATED!
				 * But at least we don't have to do this twice.
				 */
                        if (*a_select_flags.offset(i as isize)).rtval as
                               libc::c_int == 9 as libc::c_int {
                            let mut itemgood: bool_ =
                                1 as libc::c_int as bool_;
                            /*Specified race not this one */
                            if (*o_ptr).pval2 as libc::c_int !=
                                   (*a_select_flags.offset(i as isize)).rpval
                                   &&
                                   (*a_select_flags.offset(i as isize)).rpval
                                       != 0 {
                                current_block_34 = 1841672684692190573;
                            } else {
                                /* Race flag (any monster who...)*/
                                j = 0 as libc::c_int;
                                while (*a_select_flags.offset(i as
                                                                  isize)).rpval
                                          == 0 &&
                                          (*a_select_flags.offset(i as
                                                                      isize)).rflag[j
                                                                                        as
                                                                                        usize]
                                              != 0 && j < 6 as libc::c_int &&
                                          itemgood as libc::c_int != 0 {
                                    let mut flag: libc::c_int =
                                        (*a_select_flags.offset(i as
                                                                    isize)).rflag[j
                                                                                      as
                                                                                      usize]
                                            / 32 as libc::c_int;
                                    let mut mask: u32b =
                                        ((1 as libc::c_int) <<
                                             (*a_select_flags.offset(i as
                                                                         isize)).rflag[j
                                                                                           as
                                                                                           usize]
                                                 % 32 as libc::c_int) as u32b;
                                    match flag {
                                        0 => {
                                            if (*r_info.offset((*o_ptr).pval2
                                                                   as
                                                                   isize)).flags1
                                                   & mask == 0 {
                                                itemgood =
                                                    0 as libc::c_int as bool_
                                            }
                                        }
                                        1 => {
                                            if (*r_info.offset((*o_ptr).pval2
                                                                   as
                                                                   isize)).flags2
                                                   & mask == 0 {
                                                itemgood =
                                                    0 as libc::c_int as bool_
                                            }
                                        }
                                        2 => {
                                            if (*r_info.offset((*o_ptr).pval2
                                                                   as
                                                                   isize)).flags3
                                                   & mask == 0 {
                                                itemgood =
                                                    0 as libc::c_int as bool_
                                            }
                                        }
                                        3 => {
                                            if (*r_info.offset((*o_ptr).pval2
                                                                   as
                                                                   isize)).flags4
                                                   & mask == 0 {
                                                itemgood =
                                                    0 as libc::c_int as bool_
                                            }
                                        }
                                        4 => {
                                            if (*r_info.offset((*o_ptr).pval2
                                                                   as
                                                                   isize)).flags5
                                                   & mask == 0 {
                                                itemgood =
                                                    0 as libc::c_int as bool_
                                            }
                                        }
                                        5 => {
                                            if (*r_info.offset((*o_ptr).pval2
                                                                   as
                                                                   isize)).flags6
                                                   & mask == 0 {
                                                itemgood =
                                                    0 as libc::c_int as bool_
                                            }
                                        }
                                        6 => {
                                            if (*r_info.offset((*o_ptr).pval2
                                                                   as
                                                                   isize)).flags7
                                                   & mask == 0 {
                                                itemgood =
                                                    0 as libc::c_int as bool_
                                            }
                                        }
                                        7 => {
                                            if (*r_info.offset((*o_ptr).pval2
                                                                   as
                                                                   isize)).flags8
                                                   & mask == 0 {
                                                itemgood =
                                                    0 as libc::c_int as bool_
                                            }
                                        }
                                        8 => {
                                            if (*r_info.offset((*o_ptr).pval2
                                                                   as
                                                                   isize)).flags9
                                                   & mask == 0 {
                                                itemgood =
                                                    0 as libc::c_int as bool_
                                            }
                                        }
                                        _ => {
                                            msg_print(b"This code should never be hit!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                        }
                                    }
                                    j += 1
                                }
                                if itemgood == 0 {
                                    current_block_34 = 1841672684692190573;
                                } else {
                                    current_block_34 = 7018308795614528254;
                                }
                            }
                        } else if (*a_select_flags.offset(i as isize)).rpval
                                      != 0 {
                            /* Validate pval of good item */
                            /* Must have matching signs */
                            if ((*o_ptr).pval < 0 as libc::c_int) as
                                   libc::c_int !=
                                   ((*a_select_flags.offset(i as isize)).rpval
                                        < 0 as libc::c_int) as libc::c_int {
                                current_block_34 = 1841672684692190573;
                            } else if abs((*o_ptr).pval) <
                                          abs((*a_select_flags.offset(i as
                                                                          isize)).rpval)
                             {
                                current_block_34 = 1841672684692190573;
                            } else { current_block_34 = 7018308795614528254; }
                        } else { current_block_34 = 7018308795614528254; }
                        match current_block_34 {
                            1841672684692190573 => { }
                            _ => {
                                trqty =
                                    if (*o_ptr).number as libc::c_int > rqty {
                                        rqty
                                    } else { (*o_ptr).number as libc::c_int };
                                rqty -= trqty;
                                if mode == 1 as libc::c_int {
                                    inc_stack_size_ex(k, -trqty, NO_OPTIMIZE,
                                                      DESCRIBE);
                                }
                            }
                        }
                    }
                    k += 1
                }
                if rqty != 0 {
                    good = 0 as libc::c_int as bool_;
                    /* Must be greater than */
                    /* Oops, we didn't have enough of this object
			 when actually creating the artifact.
			 unset this flag
			 */
                    if mode == 1 as libc::c_int {
                        flags_select[i as usize] = -(4 as libc::c_int)
                    }
                    /* we only return false for mode -1,
			 * for mode 0 we display stuff, and for
			 * mode 1 we want to continue destroying things
			 * even if the player is missing one small item,
			 * because there's no way to change things now.
			 * We may have already destroyed a unique corpse,
			 * or some other hard-to-find item.
			 */
                    if mode == -(1 as libc::c_int) { return 0 as libc::c_int }
                }
                /* Display a description of the required object, if needed */
		/* Note that the tests for good items HAVE to be in a different
		 place, because otherwise we don't know how many the player
		 has, as opposed to how many they need.
		 */
                if mode == 0 as libc::c_int {
                    let mut o_name: *mut libc::c_char =
                        al_name.offset((*a_select_flags.offset(i as
                                                                   isize)).item_desc
                                           as isize);
                    if orqty > 1 as libc::c_int &&
                           (*a_select_flags.offset(i as isize)).pval as
                               libc::c_int != 0 &&
                           (*a_select_flags.offset(i as isize)).item_descp !=
                               0 {
                        o_name =
                            al_name.offset((*a_select_flags.offset(i as
                                                                       isize)).item_descp
                                               as isize)
                    }
                    if rqty != 0 {
                        if orqty > 1 as libc::c_int {
                            let fresh0 = row;
                            row = row + 1;
                            c_prt(4 as libc::c_int as byte_hack,
                                  format(b" you are missing %d of the %d %s\x00"
                                             as *const u8 as
                                             *const libc::c_char, rqty, orqty,
                                         o_name) as cptr, fresh0, col);
                        } else if is_a_vowel(*o_name.offset(0 as libc::c_int
                                                                as isize) as
                                                 libc::c_int) != 0 {
                            let fresh1 = row;
                            row = row + 1;
                            c_prt(4 as libc::c_int as byte_hack,
                                  format(b" you are missing an %s\x00" as
                                             *const u8 as *const libc::c_char,
                                         o_name) as cptr, fresh1, col);
                        } else {
                            let fresh2 = row;
                            row = row + 1;
                            c_prt(4 as libc::c_int as byte_hack,
                                  format(b" you are missing a %s\x00" as
                                             *const u8 as *const libc::c_char,
                                         o_name) as cptr, fresh2, col);
                        }
                    } else if orqty > 1 as libc::c_int {
                        let fresh3 = row;
                        row = row + 1;
                        c_prt(5 as libc::c_int as byte_hack,
                              format(b" you have the %d %s\x00" as *const u8
                                         as *const libc::c_char, orqty,
                                     o_name) as cptr, fresh3, col);
                    } else if is_a_vowel(*o_name.offset(0 as libc::c_int as
                                                            isize) as
                                             libc::c_int) != 0 {
                        let fresh4 = row;
                        row = row + 1;
                        c_prt(5 as libc::c_int as byte_hack,
                              format(b" you have an %s\x00" as *const u8 as
                                         *const libc::c_char, o_name) as cptr,
                              fresh4, col);
                    } else {
                        let fresh5 = row;
                        row = row + 1;
                        c_prt(5 as libc::c_int as byte_hack,
                              format(b" you have a %s\x00" as *const u8 as
                                         *const libc::c_char, o_name) as cptr,
                              fresh5, col);
                    }
                    if row > 21 as libc::c_int {
                        row = 1 as libc::c_int;
                        if good == 0 {
                            get_com(b"You are missing some items:\x00" as
                                        *const u8 as *const libc::c_char,
                                    &mut ch);
                        } else {
                            get_com(b"You have these needed items on hand:\x00"
                                        as *const u8 as *const libc::c_char,
                                    &mut ch);
                        }
                    }
                }
            }
        }
        i += 1
    }
    if mode == 0 as libc::c_int {
        while row < 22 as libc::c_int {
            let fresh6 = row;
            row = row + 1;
            c_prt(5 as libc::c_int as byte_hack,
                  b"                            \x00" as *const u8 as
                      *const libc::c_char, fresh6, col);
        }
        if good == 0 {
            get_com(b"You are missing some items:\x00" as *const u8 as
                        *const libc::c_char, &mut ch);
        } else {
            get_com(b"You have these needed items on hand:\x00" as *const u8
                        as *const libc::c_char, &mut ch);
        }
    }
    return good as libc::c_int;
}
/* Display a list of required essences,
 * and/or use up the essences. */
#[no_mangle]
pub unsafe extern "C" fn artifact_display_or_use(mut pval: libc::c_int,
                                                 mut oldpval: libc::c_int,
                                                 mut use_0: bool_) -> bool_ {
    let mut essence: [libc::c_int; 18] = [0; 18];
    let mut essenceh: [libc::c_int; 18] = [0; 18];
    let mut al_idx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut enough: bool_ = 0;
    /* Temporary Items require only one item, and no essences. */
    i = 0 as libc::c_int;
    while (*a_select_flags.offset(i as isize)).group != 0 {
        if (*a_select_flags.offset(i as isize)).flag ==
               32 as libc::c_int * 4 as libc::c_int {
            if use_0 != 0 {
                return check_artifact_items(pval, oldpval, 1 as libc::c_int)
                           as bool_
            } else {
                return check_artifact_items(pval, oldpval, 0 as libc::c_int)
                           as bool_
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 18 as libc::c_int {
        essenceh[i as usize] = 0 as libc::c_int;
        essence[i as usize] = essenceh[i as usize];
        i += 1
    }
    /* Accumulate a list of required essences */
    al_idx = 0 as libc::c_int;
    while al_idx < max_al_idx as libc::c_int {
        if (*alchemist_recipes.offset(al_idx as isize)).tval as libc::c_int ==
               0 as libc::c_int {
            i = 0 as libc::c_int;
            while (*a_select_flags.offset(i as isize)).group != 0 {
                let mut rqty: libc::c_int = calc_rqty(i, pval, oldpval);
                /* If the flag isn't being set, rqty will be zero */
                if !(rqty == 0) {
                    if (*alchemist_recipes.offset(al_idx as isize)).sval as
                           libc::c_int ==
                           (*a_select_flags.offset(i as isize)).flag {
                        essence[(*alchemist_recipes.offset(al_idx as
                                                               isize)).sval_essence
                                    as usize] +=
                            (*alchemist_recipes.offset(al_idx as isize)).qty
                                as libc::c_int * rqty
                    }
                }
                i += 1
            }
        }
        al_idx += 1
    }
    /* The essence array now contains a list of all essences
	 * that will be consumed in the creation of this artifact */
    /* Check for existence of required quatities of essences. */
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 18 as libc::c_int {
            if (*p_ptr).inventory[i as usize].tval as libc::c_int ==
                   4 as libc::c_int &&
                   (*p_ptr).inventory[i as usize].sval as libc::c_int ==
                       j + 1 as libc::c_int {
                essenceh[j as usize] +=
                    (*p_ptr).inventory[i as usize].number as libc::c_int
            }
            j += 1
        }
        i += 1
    }
    /* Check for enough essences */
    enough = 1 as libc::c_int as bool_;
    i = 0 as libc::c_int;
    while i < 18 as libc::c_int {
        if essenceh[i as usize] < essence[i as usize] {
            enough = 0 as libc::c_int as bool_;
            break ;
        } else { i += 1 }
    }
    /* Check for items */
    if enough != 0 {
        enough =
            check_artifact_items(pval, oldpval, -(1 as libc::c_int)) as bool_
    }
    /* Display recipe list if they don't have enough, or not enough exp */
    if enough == 0 || use_0 == 0 {
        let mut row: libc::c_int = 1 as libc::c_int;
        let mut col: libc::c_int = 15 as libc::c_int;
        let mut good: bool_ = 0 as libc::c_int as bool_;
        let mut ch: libc::c_char = 0;
        /* display of list of required essences */
		/* Note: there are only 12 or so essences, so this list
		 * will ALWAYS fit on the screen */
        i = 0 as libc::c_int;
        while i < 18 as libc::c_int {
            if essence[i as usize] != 0 {
                let mut missing: libc::c_int =
                    -if essenceh[i as usize] - essence[i as usize] >
                            0 as libc::c_int {
                         0 as libc::c_int
                     } else { (essenceh[i as usize]) - essence[i as usize] };
                good = 1 as libc::c_int as bool_;
                if missing != 0 {
                    let fresh7 = row;
                    row = row + 1;
                    c_prt(4 as libc::c_int as byte_hack,
                          format(b"%d of the required %d essences of %s\x00"
                                     as *const u8 as *const libc::c_char,
                                 missing, essence[i as usize],
                                 k_name.offset((*k_info.offset(lookup_kind(4
                                                                               as
                                                                               libc::c_int,
                                                                           i +
                                                                               1
                                                                                   as
                                                                                   libc::c_int)
                                                                   as
                                                                   isize)).name
                                                   as isize)) as cptr, fresh7,
                          col);
                } else {
                    let fresh8 = row;
                    row = row + 1;
                    c_prt(5 as libc::c_int as byte_hack,
                          format(b"you have the needed %d essences of %s\x00"
                                     as *const u8 as *const libc::c_char,
                                 essence[i as usize],
                                 k_name.offset((*k_info.offset(lookup_kind(4
                                                                               as
                                                                               libc::c_int,
                                                                           i +
                                                                               1
                                                                                   as
                                                                                   libc::c_int)
                                                                   as
                                                                   isize)).name
                                                   as isize)) as cptr, fresh8,
                          col);
                }
            }
            i += 1
        }
        if good != 0 {
            /* blank the bottom row */
            let fresh9 = row;
            row = row + 1;
            c_prt(1 as libc::c_int as byte_hack,
                  b"                              \x00" as *const u8 as
                      *const libc::c_char, fresh9, col);
            /* and wait for a key */
            get_com(b"You are currently missing:\x00" as *const u8 as
                        *const libc::c_char, &mut ch);
        }
        /* Display a list of needed items as well */
        check_artifact_items(pval, oldpval, 0 as libc::c_int);
        return 0 as libc::c_int as bool_
    }
    /* If we get to this point in the code, then the player
	 * has the required essences and items in their p_ptr->inventory */
    /* If they do have enough, and they have enough exp, consume them */
    i = 0 as libc::c_int;
    while i < 18 as libc::c_int {
        k = 0 as libc::c_int;
        while k < 24 as libc::c_int && essence[i as usize] > 0 as libc::c_int
              {
            if (*p_ptr).inventory[k as usize].tval as libc::c_int ==
                   4 as libc::c_int &&
                   (*p_ptr).inventory[k as usize].sval as libc::c_int ==
                       i + 1 as libc::c_int && essence[i as usize] != 0 {
                let mut num: libc::c_int =
                    (*p_ptr).inventory[k as usize].number as libc::c_int;
                inc_stack_size_ex(k,
                                  if -essence[i as usize] < -num {
                                      -num
                                  } else { -essence[i as usize] },
                                  NO_OPTIMIZE, DESCRIBE);
                essence[i as usize] -=
                    if num > essence[i as usize] {
                        essence[i as usize]
                    } else { num }
            }
            k += 1
        }
        i += 1
    }
    /* Destroy the items needed */
    check_artifact_items(pval, oldpval, 1 as libc::c_int);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn display_activation_info(mut num: libc::c_int) {
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
    let mut i: libc::c_int = 0;
    /* find the a_select_flags number of this activation type... */
    i = 0 as libc::c_int;
    while (*a_select_flags.offset(i as isize)).group != 0 {
        if (*a_select_flags.offset(i as isize)).group as libc::c_int ==
               88 as libc::c_int &&
               (*a_select_flags.offset(i as isize)).flag == -num {
            break ;
        }
        i += 1
    }
    object_wipe(&mut forge);
    forge.xtra2 = num as s16b;
    /* Print out various information about this activation... */
	/* min level, experience, required items (and essences)
	   full description (from activation_aux) */
    if wizard != 0 {
        c_prt(1 as libc::c_int as byte_hack,
              format(b"  number:%d          \x00" as *const u8 as
                         *const libc::c_char, num) as cptr, 5 as libc::c_int,
              5 as libc::c_int); /* currently, ~127 hardcoded activations */
    } else {
        c_prt(1 as libc::c_int as byte_hack,
              b"                                    \x00" as *const u8 as
                  *const libc::c_char, 5 as libc::c_int, 5 as libc::c_int);
    }
    c_prt(1 as libc::c_int as byte_hack,
          format(b"  Level:%d                              \x00" as *const u8
                     as *const libc::c_char,
                 (*a_select_flags.offset(i as isize)).level as libc::c_int) as
              cptr, 6 as libc::c_int, 5 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          format(b"  Exp  :%d                              \x00" as *const u8
                     as *const libc::c_char,
                 (*a_select_flags.offset(i as isize)).xp) as cptr,
          7 as libc::c_int, 5 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          format(b"  Item :%s                              \x00" as *const u8
                     as *const libc::c_char,
                 al_name.offset((*a_select_flags.offset(i as isize)).item_desc
                                    as isize)) as cptr, 8 as libc::c_int,
          5 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          b"                                                                  \x00"
              as *const u8 as *const libc::c_char, 9 as libc::c_int,
          5 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          format(b"  %s  \x00" as *const u8 as *const libc::c_char,
                 activation_aux(&mut forge, 0 as libc::c_int as bool_,
                                0 as libc::c_int)) as cptr, 9 as libc::c_int,
          5 as libc::c_int);
    c_prt(1 as libc::c_int as byte_hack,
          b"                                    \x00" as *const u8 as
              *const libc::c_char, 10 as libc::c_int, 5 as libc::c_int);
    inkey();
}
#[no_mangle]
pub unsafe extern "C" fn select_an_activation() {
    let mut i: libc::c_int = 0;
    let mut lev: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut begin: libc::c_int = 0 as libc::c_int;
    let mut sel: libc::c_int = 0 as libc::c_int;
    let mut max: u32b = 0;
    let mut act_list: [cptr; 150] = [0 as *const libc::c_char; 150];
    let mut act_ref: [libc::c_int; 150] = [0; 150];
    let mut c: libc::c_char = 0;
    /* How do we want to do this? */
	/* Ideally, we let them select from a list, which includes all the activations that they've ecountered in any form.
	Problems with this idea include mainly the lack of any (current) place to store which activations they've seen, and
	that they'll not get credit for any seen before we start tracking it.

	So - list is everything. If they select one which they're to low-level for
	or if the explicitly request it, we'll display info about this item.
	We'll also get our descriptions from the activation_aux(ACT_CONSTANT) 
	function, because they are more complete, and include even lua-scripted ones.
	msg_print("Since the code to actually let you select one isn't here");
	msg_print("You will automatically get the activation 'Dawn'");
	activation_select = ACT_DAWN;
	*/
    /* Build a list of available activations at the player's level */
    lev = get_skill(39 as libc::c_int) as libc::c_int; /* Activation number */
    max = 0 as libc::c_int as u32b;
    i = max as libc::c_int;
    while (max as libc::c_ulong) <
              (::std::mem::size_of::<[cptr; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<cptr>()
                                                   as libc::c_ulong) &&
              (*a_select_flags.offset(i as isize)).group as libc::c_int != 0 {
        if (*a_select_flags.offset(i as isize)).group as libc::c_int ==
               88 as libc::c_int &&
               (*a_select_flags.offset(i as isize)).level as libc::c_int <=
                   lev {
            act_ref[max as usize] =
                -(*a_select_flags.offset(i as isize)).flag;
            let fresh10 = max;
            max = max.wrapping_add(1);
            act_list[fresh10 as usize] =
                al_name.offset((*a_select_flags.offset(i as isize)).desc as
                                   isize) as cptr
            /* Description */
        }
        i += 1
    }
    loop 
         /* Select from that list, using the util.c function display_list to display the scrolled list */
	/* Note: I think that there is only one other place that uses this function. Should be more! */
         {
        Term_clear();
        Term_get_size(&mut wid, &mut hgt);
        c_prt(1 as libc::c_int as byte_hack,
              b"Enter to select, ? for more information, 2 and 8 to scroll         \x00"
                  as *const u8 as *const libc::c_char, 0 as libc::c_int,
              0 as libc::c_int);
        display_list(1 as libc::c_int, 0 as libc::c_int,
                     hgt - 2 as libc::c_int, wid - 2 as libc::c_int,
                     b"Select an Activation\x00" as *const u8 as
                         *const libc::c_char, act_list.as_mut_ptr(),
                     max as libc::c_int, begin, sel,
                     13 as libc::c_int as byte_hack);
        c = inkey();
        if c as libc::c_int == '\u{1b}' as i32 { break ; }
        if c as libc::c_int == '8' as i32 {
            sel -= 1;
            if sel < 0 as libc::c_int {
                sel =
                    max.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                        libc::c_int;
                begin = max.wrapping_sub(hgt as libc::c_uint) as libc::c_int;
                if begin < 0 as libc::c_int { begin = 0 as libc::c_int }
            }
            if sel < begin { begin = sel }
        } else if c as libc::c_int == '2' as i32 {
            sel += 1;
            if sel >= max as s32b {
                sel = 0 as libc::c_int;
                begin = 0 as libc::c_int
            }
            if sel >= begin + hgt - 1 as libc::c_int { begin += 1 }
        } else if c as libc::c_int == '?' as i32 {
            display_activation_info(act_ref[sel as usize]);
        } else if c as libc::c_int == '\r' as i32 {
            display_activation_info(act_ref[sel as usize]);
            activation_select = act_ref[sel as usize];
            return
        }
    }
    activation_select = 0 as libc::c_int;
}
/* Consume 'num' magic essences and return true.
 * If there aren't enough essences, return false */
#[no_mangle]
pub unsafe extern "C" fn magic_essence(mut num: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Count the magic essences */
        if (*o_ptr).k_idx as libc::c_int != 0 &&
               (*o_ptr).tval as libc::c_int == 4 as libc::c_int &&
               (*o_ptr).sval as libc::c_int == 12 as libc::c_int {
            j += (*o_ptr).number as libc::c_int
        }
        i += 1
    }
    /* Abort if not enough essences. */
    if j < num { return 0 as libc::c_int as bool_ }
    /* Consume them */
    i = 0 as libc::c_int;
    j = num;
    while i < 24 as libc::c_int {
        let mut o_ptr_0: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if (*o_ptr_0).k_idx as libc::c_int != 0 &&
               (*o_ptr_0).tval as libc::c_int == 4 as libc::c_int &&
               (*o_ptr_0).sval as libc::c_int == 12 as libc::c_int {
            /* This can lead to invalid object pointer for objects
			 * that come after the magic essences. Therefore, every
			 * artifactable object should come before the essences.
			 */
            j -= (*o_ptr_0).number as libc::c_int;
            inc_stack_size(i, -num);
            num = j;
            if num <= 0 as libc::c_int { break ; }
            /* Stay on this slot; do not increment i. */
        } else {
            /* Move on to the next slot. */
            i += 1
        }
    }
    /* Sanity check. */
    if num > 0 as libc::c_int {
        msg_format(b"ERROR: Couldn\'t destroy %d essences!\x00" as *const u8
                       as *const libc::c_char, num);
        return 0 as libc::c_int as bool_
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_create_artifact(mut q_ptr: *mut object_type) {
    let mut max: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut cur_set: libc::c_int = 0 as libc::c_int;
    let mut abord: libc::c_int = 0 as libc::c_int;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut skill: libc::c_int = 0;
    let mut exp: s32b = 0 as libc::c_int;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut choice: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut lockpval: bool_ = 0 as libc::c_int as bool_;
    let mut pval: libc::c_int = 0;
    let mut oldpval: libc::c_int = 0;
    energy_use = 100 as libc::c_int;
    pval = (*q_ptr).pval;
    oldpval = pval;
    skill = get_skill(39 as libc::c_int) as libc::c_int;
    if pval == 0 { pval = 1 as libc::c_int }
    /* No activation added on this round */
    activation_select = 0 as libc::c_int;
    /* Save the current flags */
    i = 0 as libc::c_int;
    while (*a_select_flags.offset(i as isize)).group != 0 {
        if !((*a_select_flags.offset(i as isize)).flag < 0 as libc::c_int ||
                 (*a_select_flags.offset(i as isize)).group as libc::c_int >
                     5 as libc::c_int) {
            flags_select[i as usize] = 0 as libc::c_int;
            match (*a_select_flags.offset(i as isize)).flag /
                      32 as libc::c_int {
                0 => {
                    if (*q_ptr).art_flags1 &
                           ((1 as libc::c_int) <<
                                (*a_select_flags.offset(i as isize)).flag %
                                    32 as libc::c_int) as libc::c_uint != 0 {
                        flags_select[i as usize] = -(1 as libc::c_int)
                    }
                }
                1 => {
                    if (*q_ptr).art_flags2 &
                           ((1 as libc::c_int) <<
                                (*a_select_flags.offset(i as isize)).flag %
                                    32 as libc::c_int) as libc::c_uint != 0 {
                        flags_select[i as usize] = -(1 as libc::c_int)
                    }
                }
                2 => {
                    if (*q_ptr).art_flags3 &
                           ((1 as libc::c_int) <<
                                (*a_select_flags.offset(i as isize)).flag %
                                    32 as libc::c_int) as libc::c_uint != 0 {
                        flags_select[i as usize] = -(1 as libc::c_int)
                    }
                }
                3 => {
                    if (*q_ptr).art_flags4 &
                           ((1 as libc::c_int) <<
                                (*a_select_flags.offset(i as isize)).flag %
                                    32 as libc::c_int) as libc::c_uint != 0 {
                        flags_select[i as usize] = -(1 as libc::c_int)
                    }
                }
                4 => {
                    if (*q_ptr).art_flags5 &
                           ((1 as libc::c_int) <<
                                (*a_select_flags.offset(i as isize)).flag %
                                    32 as libc::c_int) as libc::c_uint != 0 {
                        flags_select[i as usize] = -(1 as libc::c_int)
                    }
                }
                5 => {
                    if (*q_ptr).art_esp &
                           ((1 as libc::c_int) <<
                                (*a_select_flags.offset(i as isize)).flag %
                                    32 as libc::c_int) as libc::c_uint != 0 {
                        flags_select[i as usize] = -(1 as libc::c_int)
                    }
                }
                _ => { }
            }
            /*
		 this would learn about ALL flags....
		 if(wizard)
		 alchemist_known_artifacts[a_select_flags[i].flag/32] = 0xffffffffL;
		 */
            /* Set various flags if they haven't *ID*'d an artifact with this flag set.*/
            if alchemist_known_artifacts[((*a_select_flags.offset(i as
                                                                      isize)).flag
                                              / 32 as libc::c_int) as usize] &
                   ((1 as libc::c_int) <<
                        (*a_select_flags.offset(i as isize)).flag %
                            32 as libc::c_int) as libc::c_uint == 0 {
                /* If this item has an ability that depends on pval which the player
			 * cannot set, don't allow them to change the pval either. */
                if (*a_select_flags.offset(i as isize)).pval as libc::c_int !=
                       0 && flags_select[i as usize] != 0 {
                    lockpval = 1 as libc::c_int as bool_
                }
                /* Set the color and set-ablitity of this flag */
                if flags_select[i as usize] != 0 {
                    flags_select[i as usize] = -(3 as libc::c_int)
                } else { flags_select[i as usize] = -(2 as libc::c_int) }
            } else if skill <
                          (*a_select_flags.offset(i as isize)).level as
                              libc::c_int {
                /* If the alchemist has not passed the skill level for this flag,
			 Set this flag as unsettable.
			 */
                if flags_select[i as usize] != 0 {
                    lockpval = 1 as libc::c_int as bool_
                } else { flags_select[i as usize] = -(4 as libc::c_int) }
            }
        }
        i += 1
    }
    /* Save the screen */
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    Term_clear();
    /* Everlasting love ... ... nevermind :) */
    while done == 0 && abord == 0 {
        c_prt(if (*q_ptr).exp - exp > 0 as libc::c_int {
                  13 as libc::c_int
              } else { 12 as libc::c_int } as byte_hack,
              format(b"Experience left: %ld\x00" as *const u8 as
                         *const libc::c_char, (*q_ptr).exp - exp) as cptr,
              2 as libc::c_int,
              0 as
                  libc::c_int); /* main screen (flag select screen) select and redraw loop*/
        /* Display the menu, but don't display it if we just
		 * displayed a message (it erases the screen, creating a blink message */
        if cur_set < 6 as libc::c_int || cur_set == 7 as libc::c_int {
            show_levels(); /*or break, same diff */
        } /*sub-screen select and redraw loop*/
        c_prt(if (*q_ptr).exp - exp > 0 as libc::c_int {
                  13 as libc::c_int
              } else { 12 as libc::c_int } as byte_hack,
              format(b"Experience left: %ld\x00" as *const u8 as
                         *const libc::c_char, (*q_ptr).exp - exp) as cptr,
              2 as libc::c_int, 0 as libc::c_int);
        prt(b"Enter to accept, Escape to abort\x00" as *const u8 as
                *const libc::c_char, 1 as libc::c_int, 0 as libc::c_int);
        abord =
            (get_com(b"Play around with which group of powers?[a-g]\x00" as
                         *const u8 as *const libc::c_char, &mut choice) == 0)
                as libc::c_int;
        if choice as libc::c_int == '\u{1b}' as i32 {
            abord = 1 as libc::c_int
        }
        if abord != 0 { continue ; }
        if *(*__ctype_b_loc()).offset(choice as libc::c_int as isize) as
               libc::c_int &
               _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            if *(*__ctype_b_loc()).offset(choice as libc::c_int as isize) as
                   libc::c_int &
                   _ISupper as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                choice = tolower(choice as libc::c_int) as libc::c_char
            }
            cur_set = choice as libc::c_int - 'a' as i32;
            if cur_set == 5 as libc::c_int {
                if (*q_ptr).xtra2 as libc::c_int != 0 &&
                       activation_select == 0 &&
                       get_check(b"This item already activates! Choose a different activation?\x00"
                                     as *const u8 as *const libc::c_char) == 0
                   {
                    continue ;
                }
                select_an_activation();
                exp = get_flags_exp(pval, oldpval)
            } else if cur_set == 6 as libc::c_int {
                msg_print(b"This option is not available\x00" as *const u8 as
                              *const libc::c_char);
            } else if cur_set == 7 as libc::c_int {
                artifact_display_or_use(pval, oldpval,
                                        0 as libc::c_int as bool_);
            } else if cur_set == 8 as libc::c_int {
                if (*q_ptr).exp - exp < 0 as libc::c_int {
                    msg_print(b"Not enough experience for the flags you\'ve selected.\x00"
                                  as *const u8 as *const libc::c_char);
                } else { done = 1 as libc::c_int }
            } else if cur_set < 0 as libc::c_int || cur_set > 4 as libc::c_int
             {
                bell();
            } else {
                while done == 0 && abord == 0 {
                    /* Chose the flags */
                    exp = 0 as libc::c_int;
                    max = show_flags(cur_set as byte_hack, pval);
                    exp = get_flags_exp(pval, oldpval);
                    c_prt(if (*q_ptr).exp - exp > 0 as libc::c_int {
                              13 as libc::c_int
                          } else { 12 as libc::c_int } as byte_hack,
                          format(b"Experience left: %ld\x00" as *const u8 as
                                     *const libc::c_char, (*q_ptr).exp - exp)
                              as cptr, 2 as libc::c_int, 0 as libc::c_int);
                    /* Build a prompt (accept all flags) */
                    if max <= 26 as libc::c_int {
                        /* Build a prompt (accept all flags) */
                        strnfmt(out_val.as_mut_ptr(),
                                78 as libc::c_int as uint_hack,
                                b"(Flags %c-%c, I,D to change power level) Add/Remove which flag? \x00"
                                    as *const u8 as *const libc::c_char,
                                0 as libc::c_int + 'a' as i32,
                                max - 1 as libc::c_int + 'a' as i32);
                    } else {
                        strnfmt(out_val.as_mut_ptr(),
                                78 as libc::c_int as uint_hack,
                                b"(Flags %c-%c, I,D to change power level) Add/Remove which flag? \x00"
                                    as *const u8 as *const libc::c_char,
                                0 as libc::c_int + 'a' as i32,
                                '0' as i32 + max - 27 as libc::c_int);
                    }
                    c_prt(14 as libc::c_int as byte_hack,
                          format(b"Power(I/D to increase/decrease): %d\x00" as
                                     *const u8 as *const libc::c_char, pval)
                              as cptr, 3 as libc::c_int, 0 as libc::c_int);
                    loop 
                         /* Get a spell from the user */
                         {
                        done =
                            (get_com(out_val.as_mut_ptr() as cptr,
                                     &mut choice) == 0) as libc::c_int;
                        if !(done == 0) { break ; }
                        if choice as libc::c_int == 'I' as i32 {
                            if lockpval != 0 {
                                msg_print(b"You cannot do that - you don\'t know how!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                            } else if (*q_ptr).exp - exp < 0 as libc::c_int {
                                msg_print(b"Not enough experience.  Decrease power or deselect flags.\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                            } else { pval += 1; break ; }
                        } else if choice as libc::c_int == 'D' as i32 {
                            if lockpval != 0 {
                                msg_print(b"You cannot do that - you don\'t know how!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                            } else {
                                pval -= 1;
                                if pval < oldpval { pval = oldpval }
                                break ;
                            }
                        } else if choice as libc::c_int == '\r' as i32 ||
                                      choice as libc::c_int == '\u{1b}' as i32
                                      || choice as libc::c_int == ' ' as i32 {
                            done = 1 as libc::c_int;
                            break ;
                        } else {
                            if *(*__ctype_b_loc()).offset(choice as
                                                              libc::c_int as
                                                              isize) as
                                   libc::c_int &
                                   _ISalpha as libc::c_int as libc::c_ushort
                                       as libc::c_int != 0 {
                                /* Lowercase */
                                if *(*__ctype_b_loc()).offset(choice as
                                                                  libc::c_int
                                                                  as isize) as
                                       libc::c_int &
                                       _ISupper as libc::c_int as
                                           libc::c_ushort as libc::c_int != 0
                                   {
                                    choice =
                                        tolower(choice as libc::c_int) as
                                            libc::c_char
                                }
                                /* Extract request */
                                i =
                                    if *(*__ctype_b_loc()).offset(choice as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                           as libc::c_int &
                                           _ISlower as libc::c_int as
                                               libc::c_ushort as libc::c_int
                                           != 0 {
                                        (choice as libc::c_int) - 'a' as i32
                                    } else { -(1 as libc::c_int) }
                            } else {
                                i =
                                    choice as libc::c_int - '0' as i32 +
                                        26 as libc::c_int;
                                /* Illegal */
                                if i < 26 as libc::c_int {
                                    i = -(1 as libc::c_int)
                                }
                            }
                            /* Totally Illegal */
                            if i < 0 as libc::c_int || i >= max {
                                bell();
                            } else {
                                /*Find the i'th flag in group cur_set...*/
                                j = 0 as libc::c_int;
                                while (*a_select_flags.offset(j as
                                                                  isize)).group
                                          != 0 {
                                    if (*a_select_flags.offset(j as
                                                                   isize)).group
                                           as libc::c_int ==
                                           cur_set + 1 as libc::c_int {
                                        let fresh11 = i;
                                        i = i - 1;
                                        if fresh11 == 0 { break ; }
                                    }
                                    j += 1
                                }
                                if flags_select[j as usize] ==
                                       -(4 as libc::c_int) {
                                    msg_format(b"You need at least %d skill in alchemy.\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               (*a_select_flags.offset(j as
                                                                           isize)).level
                                                   as libc::c_int);
                                } else if flags_select[j as usize] !=
                                              0 as libc::c_int &&
                                              flags_select[j as usize] !=
                                                  1 as libc::c_int {
                                    bell();
                                } else if flags_select[j as usize] != 0 {
                                    flags_select[j as usize] =
                                        0 as libc::c_int;
                                    break ;
                                } else {
                                    if !(flags_select[j as usize] == 0) {
                                        break ;
                                    }
                                    if (*q_ptr).exp - exp < 0 as libc::c_int {
                                        msg_print(b"Not enough experience.  Decrease power or deselect flags.\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                    } else {
                                        flags_select[j as usize] =
                                            1 as libc::c_int;
                                        break ;
                                    }
                                }
                            }
                        }
                    }
                }
                done = 0 as libc::c_int;
                Term_clear();
            }
        } else { bell(); }
    }
    /* Abort if not enough experience, or no flags added */
    if (*q_ptr).exp - exp < 0 as libc::c_int || exp == 0 as libc::c_int {
        abord = 1 as libc::c_int
    }
    /* Display the recipe, or use up the essences.
	 * Note that this has to be done before the screen
	 * is restored. This is because it's also called from
	 * within the loop to display the required items. */
    if abord == 0 {
        if artifact_display_or_use(pval, oldpval, 1 as libc::c_int as bool_)
               == 0 {
            abord = 1 as libc::c_int
        }
    }
    /* Restore the screen */
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    /* Return if abort, or missing ingredients */
    if abord != 0 { return }
    /* Actually create the artifact */
    (*q_ptr).exp -= exp;
    (*q_ptr).art_flags4 =
        ((*q_ptr).art_flags4 as libc::c_long & !(0x20000000 as libc::c_long))
            as u32b;
    (*q_ptr).pval = pval;
    /* Just to be sure */
    (*q_ptr).art_flags3 =
        ((*q_ptr).art_flags3 as libc::c_long |
             (0x100000 as libc::c_long | 0x200000 as libc::c_long |
                  0x400000 as libc::c_long | 0x800000 as libc::c_long)) as
            u32b;
    let mut now: libc::c_int = 0 as libc::c_int;
    let mut before: libc::c_int = 0 as libc::c_int;
    let mut dummy_name: [libc::c_char; 80] = [0; 80];
    let mut new_name: [libc::c_char; 80] = [0; 80];
    /* Apply the flags */
    i = 0 as libc::c_int;
    while (*a_select_flags.offset(i as isize)).group != 0 {
        if flags_select[i as usize] < 0 as libc::c_int {
            before += 1
        } else if flags_select[i as usize] == 1 as libc::c_int {
            now += 1;
            match (*a_select_flags.offset(i as isize)).flag /
                      32 as libc::c_int {
                0 => {
                    (*q_ptr).art_flags1 |=
                        ((1 as libc::c_int) <<
                             (*a_select_flags.offset(i as isize)).flag %
                                 32 as libc::c_int) as libc::c_uint
                }
                1 => {
                    (*q_ptr).art_flags2 |=
                        ((1 as libc::c_int) <<
                             (*a_select_flags.offset(i as isize)).flag %
                                 32 as libc::c_int) as libc::c_uint
                }
                2 => {
                    (*q_ptr).art_flags3 |=
                        ((1 as libc::c_int) <<
                             (*a_select_flags.offset(i as isize)).flag %
                                 32 as libc::c_int) as libc::c_uint
                }
                3 => {
                    (*q_ptr).art_flags4 |=
                        ((1 as libc::c_int) <<
                             (*a_select_flags.offset(i as isize)).flag %
                                 32 as libc::c_int) as libc::c_uint
                }
                4 => {
                    (*q_ptr).art_flags5 |=
                        ((1 as libc::c_int) <<
                             (*a_select_flags.offset(i as isize)).flag %
                                 32 as libc::c_int) as libc::c_uint
                }
                5 => {
                    (*q_ptr).art_esp |=
                        ((1 as libc::c_int) <<
                             (*a_select_flags.offset(i as isize)).flag %
                                 32 as libc::c_int) as libc::c_uint
                }
                _ => {
                    msg_print(b"error: this code can\'t ever be hit!\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
        }
        i += 1
    }
    if activation_select != 0 {
        (*q_ptr).art_flags3 =
            ((*q_ptr).art_flags3 as libc::c_long | 0x1000000 as libc::c_long)
                as u32b;
        (*q_ptr).xtra2 = activation_select as s16b
    }
    /* Set the 'show modifier' flag */
    (*q_ptr).art_flags3 =
        ((*q_ptr).art_flags3 as libc::c_long | 0x400 as libc::c_long) as u32b;
    /* For temporary items, set a timeout.
		 * alchemist_skill^2 for now */
    if (*q_ptr).art_flags5 as libc::c_long & 0x1 as libc::c_long != 0 {
        let mut lev: libc::c_int =
            get_skill(39 as libc::c_int) as libc::c_int;
        (*q_ptr).timeout = (lev * lev * 3 as libc::c_int) as s16b
    }
    /* Describe the new artifact */
    object_out_desc(q_ptr, 0 as *mut FILE, 0 as libc::c_int as bool_,
                    1 as libc::c_int as bool_);
    /* Name the new artifact */
    strcpy(dummy_name.as_mut_ptr(),
           b"of an Alchemist\x00" as *const u8 as *const libc::c_char);
    if get_string(b"What do you want to call the artifact? \x00" as *const u8
                      as *const libc::c_char, dummy_name.as_mut_ptr(),
                  80 as libc::c_int) == 0 {
        strcpy(new_name.as_mut_ptr(),
               b"of an Alchemist\x00" as *const u8 as *const libc::c_char);
    } else if strncmp(dummy_name.as_mut_ptr(),
                      b"of \x00" as *const u8 as *const libc::c_char,
                      3 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
                  ||
                  strncmp(dummy_name.as_mut_ptr(),
                          b"Of \x00" as *const u8 as *const libc::c_char,
                          3 as libc::c_int as libc::c_ulong) ==
                      0 as libc::c_int ||
                  dummy_name[0 as libc::c_int as usize] as libc::c_int ==
                      '\'' as i32 &&
                      dummy_name[strlen(dummy_name.as_mut_ptr()).wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong)
                                     as usize] as libc::c_int == '\'' as i32 {
        strcpy(new_name.as_mut_ptr(), dummy_name.as_mut_ptr());
    } else {
        strcpy(new_name.as_mut_ptr(),
               b"called \'\x00" as *const u8 as *const libc::c_char);
        strcat(new_name.as_mut_ptr(), dummy_name.as_mut_ptr());
        strcat(new_name.as_mut_ptr(),
               b"\'\x00" as *const u8 as *const libc::c_char);
    }
    /* Identify it fully */
    object_aware(q_ptr);
    object_known(q_ptr);
    /* Mark the item as fully known */
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int) as
            byte_hack; /* This will be used later on... */
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as byte_hack;
    /* Save the inscription */
    (*q_ptr).art_name = quark_add(new_name.as_mut_ptr() as cptr) as u16b;
    (*q_ptr).found = 9 as libc::c_int as byte_hack;
    done = 0 as libc::c_int;
    while done == 0 &&
              get_com(b"Do you want to let this item continue to gain experience?\x00"
                          as *const u8 as *const libc::c_char, &mut choice) as
                  libc::c_int != 0 {
        match choice as libc::c_int {
            121 | 89 => {
                if magic_essence(get_skill(39 as libc::c_int) as libc::c_int)
                       != 0 {
                    (*q_ptr).art_flags4 =
                        ((*q_ptr).art_flags4 as libc::c_long |
                             0x20000000 as libc::c_long) as u32b
                } else {
                    msg_format(b"Oh, NO! You don\'t have enough magic essences. You needed %d.\x00"
                                   as *const u8 as *const libc::c_char,
                               get_skill(39 as libc::c_int) as libc::c_int);
                }
                done = 1 as libc::c_int
            }
            110 | 78 => {
                (*q_ptr).exp = 0 as libc::c_int;
                done = 1 as libc::c_int
            }
            _ => { }
        }
    }
    /* Cycle through the p_ptr->inventory, and optimize everything.
		 * This wasn't done earlier, because if we had, then
		 * things in the p_ptr->inventory would shift around, and q_ptr
		 * wouldn't point to the right thing. BUT, at this point
		 * we don't need q_ptr anymore, so optimizing the p_ptr->inventory
		 * becomes sane. Sticky bug to figure out, let me tell you.
		 * Note also that this is cycling backwards - this is so
		 * that the same effect doesn't cause us to skip items. */
    i = 24 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int { inven_item_optimize(i); i -= 1 }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
}
/*
 * Test to see if this tval/sval combo is in the alchemists'
 * recipes as a createable item. Used to determine if we
 * should extract from it.
 */
#[no_mangle]
pub unsafe extern "C" fn alchemist_exists(mut tval: libc::c_int,
                                          mut sval: libc::c_int,
                                          mut ego: libc::c_int,
                                          mut artifact: libc::c_int)
 -> bool_ {
    let mut al_idx: libc::c_int = 0;
    /* To prevent conflicts with recipes for ego-items.
	 * artifact not used, simplifies the loop below. */
    if tval == 1 as libc::c_int || artifact != 0 {
        return 0 as libc::c_int as bool_
    }
    /*Search for recipes with this tval/sval combo as the final result*/
    al_idx = 0 as libc::c_int;
    while al_idx < max_al_idx as libc::c_int {
        let mut rtval: libc::c_int =
            (*alchemist_recipes.offset(al_idx as isize)).tval as libc::c_int;
        let mut rsval: libc::c_int =
            (*alchemist_recipes.offset(al_idx as isize)).sval as libc::c_int;
        /* Accept ego wands and staves since ego is extracted last */
        if (ego == 0 || tval == 65 as libc::c_int ||
                tval == 55 as libc::c_int) && rtval == tval && rsval == sval
               || ego != 0 && rtval == 1 as libc::c_int && rsval == ego {
            return 1 as libc::c_int as bool_
        }
        al_idx += 1
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Hook to determine if an object can have things extracted from it.
 */
#[no_mangle]
pub unsafe extern "C" fn item_tester_hook_extractable(mut o_ptr:
                                                          *mut object_type)
 -> bool_ {
    /* No artifacts */
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
        return 0 as libc::c_int as bool_
    }
    /* No cursed things */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    /* If we REALLY wanted to rebalance alchemists,
	 * we'd test for 'fully identified this object kind' here.
	 */
    return ((*o_ptr).tval as libc::c_int == 67 as libc::c_int &&
                (*o_ptr).pval != 0 as libc::c_int ||
                alchemist_exists((*o_ptr).tval as libc::c_int,
                                 (*o_ptr).sval as libc::c_int,
                                 (*o_ptr).name2 as libc::c_int,
                                 (*o_ptr).name1 as libc::c_int) as libc::c_int
                    != 0) as libc::c_int as bool_;
}
/*
 * Hook to determine if an object is empowerable (NOT rechargeable)
 */
#[no_mangle]
pub unsafe extern "C" fn item_tester_hook_empower(mut o_ptr: *mut object_type)
 -> bool_ {
    let mut sval: libc::c_int = -(1 as libc::c_int);
    let mut lev: libc::c_int = get_skill(39 as libc::c_int) as libc::c_int;
    /* after level 25, can empower ego items to create artifacts
	 * and double ego items.
	 * after level 50, can empower artifacts to create powerful artifacts
	 */
    /* Never Empower a cursed item */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Allow finalizing a self created artifact */
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
           (*o_ptr).art_flags4 as libc::c_long & 0x20000000 as libc::c_long !=
               0 &&
           (*o_ptr).art_flags4 as libc::c_long & 0x1000000 as libc::c_long ==
               0 {
        return 1 as libc::c_int as bool_
    }
    's_190:
        {
            let mut current_block_23: u64;
            match (*o_ptr).tval as libc::c_int {
                65 => {
                    /* Empowerable objects: Traditional alchemist stuff */
                    sval = 2 as libc::c_int;
                    current_block_23 = 15597372965620363352;
                }
                45 => {
                    sval = 50 as libc::c_int;
                    current_block_23 = 15597372965620363352;
                }
                55 => {
                    sval = 2 as libc::c_int;
                    current_block_23 = 15597372965620363352;
                }
                2 => {
                    sval = 1 as libc::c_int;
                    current_block_23 = 15597372965620363352;
                }
                40 => {
                    sval = 16 as libc::c_int;
                    current_block_23 = 15597372965620363352;
                }
                70 => {
                    sval = 53 as libc::c_int;
                    current_block_23 = 15597372965620363352;
                }
                66 => {
                    sval = 0 as libc::c_int;
                    current_block_23 = 15597372965620363352;
                }
                67 => {
                    sval = -(1 as libc::c_int);
                    current_block_23 = 15597372965620363352;
                }
                111 => {
                    sval = -(1 as libc::c_int);
                    current_block_23 = 15597372965620363352;
                }
                38 => {
                    /* Ego item stuff */
		/* Disallow ego dragon armour before you can create artifacts.*/
                    if lev < 25 as libc::c_int {
                        return 0 as libc::c_int as bool_
                    }
                    /* FALL THROUGH! no break here. */
                    /* weapons */
                    current_block_23 = 16458813237836223358;
                }
                115 | 23 | 21 | 22 | 24 | 6 => {
                    current_block_23 = 16458813237836223358;
                }
                19 | 15 | 14 | 20 | 39 => {
                    current_block_23 = 14862200832326069016;
                }
                16 | 17 | 18 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 => {
                    current_block_23 = 11251498038296059222;
                }
                _ => { return 0 as libc::c_int as bool_ }
            }
            match current_block_23 {
                16458813237836223358 =>
                /* misc other items */
                {
                    current_block_23 = 14862200832326069016;
                }
                15597372965620363352 => { break 's_190 ; }
                _ => { }
            }
            match current_block_23 {
                14862200832326069016 =>
                /* Ammo */
                {
                }
                _ => { }
            }
            /* Armor of various sorts */
            /* Disallow ANY creation of ego items below level 5*/
            if lev < 5 as libc::c_int { return 0 as libc::c_int as bool_ }
            /* empowering an ego item creates an artifact or a
		 * double ego item, disallow below level 25 */
            if lev < 25 as libc::c_int && (*o_ptr).name2 as libc::c_int != 0 {
                return 0 as libc::c_int as bool_
            }
            /* Disallow double-ego and artifact unless the character has
		 * the artifact creation ability. */
            if has_ability(7 as libc::c_int) == 0 &&
                   ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                        (if (*o_ptr).name1 as libc::c_int != 0 {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0 ||
                        (if (*o_ptr).art_name as libc::c_int != 0 {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0 ||
                        (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3
                                as libc::c_long & 0x8000 as libc::c_long != 0
                            {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0 ||
                        (*o_ptr).name2 as libc::c_int != 0 &&
                            (*o_ptr).name2b as libc::c_int != 0) {
                return 0 as libc::c_int as bool_
            }
            /* Otherwise... */
            return 1 as libc::c_int as bool_
        }
    /* Return to the traditional alchemist objects.
	 * All ego items and artifacts returning TRUE are accepted as artifactable
	 * at level 25. If we want double ego non wieldable items (Fireproof Staff
	 * of Plenty) the artifactable test in do_cmd_alchemist() must be changed,
	 * e.g. checking if the item is wearable.
	 * For now, we disallow non-wearable ego-items and artifacts here.
	 */
    if ((*o_ptr).name2 as libc::c_int != 0 ||
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
           (*o_ptr).tval as libc::c_int != 45 as libc::c_int &&
           (*o_ptr).tval as libc::c_int != 40 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* return true if it's a 'of nothing' item;
	 * does nothing for TV_ROD_MAIN and TV_BOOK
	 */
    return (sval == (*o_ptr).sval as libc::c_int ||
                (lev >= 50 as libc::c_int ||
                     lev >= 25 as libc::c_int &&
                         !((*o_ptr).tval as libc::c_int == 102 as libc::c_int
                               ||
                               (if (*o_ptr).name1 as libc::c_int != 0 {
                                    1 as libc::c_int
                                } else { 0 as libc::c_int }) != 0 ||
                               (if (*o_ptr).art_name as libc::c_int != 0 {
                                    1 as libc::c_int
                                } else { 0 as libc::c_int }) != 0 ||
                               (if (*k_info.offset((*o_ptr).k_idx as
                                                       isize)).flags3 as
                                       libc::c_long & 0x8000 as libc::c_long
                                       != 0 {
                                    1 as libc::c_int
                                } else { 0 as libc::c_int }) != 0)) &&
                    ((*o_ptr).tval as libc::c_int == 45 as libc::c_int ||
                         (*o_ptr).tval as libc::c_int == 40 as libc::c_int) ||
                (*o_ptr).name2 == 0 && lev >= 15 as libc::c_int) as
               libc::c_int as bool_;
}
/* Extract a rod tip from a rod */
#[no_mangle]
pub unsafe extern "C" fn rod_tip_extract(mut o_ptr: *mut object_type) {
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
    /* Get local object */
    q_ptr = &mut forge;
    /* Paranoia, return if it's a rod of nothing */
    if (*o_ptr).pval == 0 as libc::c_int { return }
    /* Extract the rod tip */
    object_prep(q_ptr,
                lookup_kind(66 as libc::c_int, (*o_ptr).pval) as libc::c_int);
    (*q_ptr).number = (*o_ptr).number;
    object_aware(q_ptr);
    object_known(q_ptr);
    inven_carry(q_ptr, 0 as libc::c_int as bool_);
    /* Remove it from the rod */
    (*o_ptr).pval = 0 as libc::c_int;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x1 as libc::c_long) as u32b;
}
/* Begin & finish an art */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_toggle_artifact(mut o_ptr: *mut object_type) {
    let mut o_name: [libc::c_char; 80] = [0; 80];
    if (*o_ptr).art_flags4 as libc::c_long & 0x20000000 as libc::c_long == 0 {
        let mut okay: bool_ = 1 as libc::c_int as bool_;
        if alchemist_has_stone() == 0 {
            msg_print(b"Creating an artifact will result into a permanent loss of 10 hp.\x00"
                          as *const u8 as *const libc::c_char);
            if get_check(b"Are you sure you want to do that?\x00" as *const u8
                             as *const libc::c_char) == 0 {
                return
            }
        }
        if magic_essence(get_skill(39 as libc::c_int) as libc::c_int) == 0 {
            msg_format(b"You need %d magic essences.\x00" as *const u8 as
                           *const libc::c_char,
                       get_skill(39 as libc::c_int) as libc::c_int);
            return
        }
        /* Description */
        object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
            msg_print(b"Not enough energy to enchant more than one object!\x00"
                          as *const u8 as *const libc::c_char);
            msg_format(b"%d of your %s %s destroyed!\x00" as *const u8 as
                           *const libc::c_char,
                       (*o_ptr).number as libc::c_int - 1 as libc::c_int,
                       o_name.as_mut_ptr(),
                       if (*o_ptr).number as libc::c_int > 2 as libc::c_int {
                           b"were\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"was\x00" as *const u8 as *const libc::c_char
                       });
            (*o_ptr).number = 1 as libc::c_int as byte_hack
        }
        okay = 1 as libc::c_int as bool_;
        if okay == 0 { return }
        /* he/she got warned */
        (*p_ptr).hp_mod =
            ((*p_ptr).hp_mod as libc::c_int - 10 as libc::c_int) as s16b;
        /* Ok toggle it */
        (*o_ptr).art_flags4 =
            ((*o_ptr).art_flags4 as libc::c_long | 0x20000000 as libc::c_long)
                as u32b;
        (*o_ptr).name2 = 0 as libc::c_int as s16b;
        (*o_ptr).name2b = 0 as libc::c_int as s16b;
        (*o_ptr).art_name =
            quark_add(b"Becoming\x00" as *const u8 as *const libc::c_char) as
                u16b;
        /* Copy the object_kind flags to the artifact flags.
		 * Note that this is only needed so that flags set in the
		 * 'kind' area are visible when finalizing the artifact.
		 */
        let mut f1: u32b = 0;
        let mut f2: u32b = 0;
        let mut f3: u32b = 0;
        let mut f4: u32b = 0;
        let mut f5: u32b = 0;
        let mut esp: u32b = 0;
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        (*o_ptr).art_flags1 |= f1;
        (*o_ptr).art_flags2 |= f2;
        (*o_ptr).art_flags3 |= f3;
        (*o_ptr).art_flags4 |= f4;
        (*o_ptr).art_flags5 |= f5;
        (*o_ptr).art_esp |= esp;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x10 as libc::c_long) as u32b;
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long |
                      0x8 as libc::c_long)) as u32b
    } else { do_cmd_create_artifact(o_ptr); };
}
/*
 * Test to see if they have all the ingredients to create an item.
 * (doesn't count base item)
 * creates 'tocreate' items (may be -1, but no more than that!)
 * if tocreate=0, will return true if the player has enough
 * in their p_ptr->inventory to empower that item.
 */
#[no_mangle]
pub unsafe extern "C" fn alchemist_items_check(mut tval: libc::c_int,
                                               mut sval: libc::c_int,
                                               mut ego: libc::c_int,
                                               mut tocreate: libc::c_int,
                                               mut message: bool_) -> bool_ {
    let mut al_idx: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut exists: bool_ = 0 as libc::c_int as bool_;
    al_idx = 0 as libc::c_int;
    while al_idx < max_al_idx as libc::c_int {
        if ego != 0 &&
               (*alchemist_recipes.offset(al_idx as isize)).sval as
                   libc::c_int == ego &&
               (*alchemist_recipes.offset(al_idx as isize)).tval as
                   libc::c_int == 1 as libc::c_int ||
               ego == 0 &&
                   (*alchemist_recipes.offset(al_idx as isize)).sval as
                       libc::c_int == sval &&
                   (*alchemist_recipes.offset(al_idx as isize)).tval as
                       libc::c_int == tval {
            exists = 1 as libc::c_int as bool_;
            /*destroying items, or just checking for existence */
            if tocreate > 0 as libc::c_int {
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
                let mut o_ptr: *mut object_type = &mut forge;
                object_wipe(o_ptr);
                object_prep(o_ptr,
                            lookup_kind(4 as libc::c_int,
                                        (*alchemist_recipes.offset(al_idx as
                                                                       isize)).sval_essence)
                                as libc::c_int);
                (*o_ptr).number =
                    ((*alchemist_recipes.offset(al_idx as isize)).qty as
                         libc::c_int * tocreate) as byte_hack;
                /* Create the essences */
                /* Don't bother with apply_magic */
                /* Randomly decrease the number of essences created */
                if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int &&
                       Rand_div(52 as libc::c_int) + 1 as libc::c_int >
                           get_skill(39 as libc::c_int) as libc::c_int &&
                       alchemist_has_stone() == 0 {
                    (*o_ptr).number =
                        ((*o_ptr).number as libc::c_int /
                             (Rand_div(2 as libc::c_int) + 1 as libc::c_int +
                                  1 as libc::c_int)) as byte_hack
                }
                if !((*o_ptr).number as libc::c_int == 0 as libc::c_int) {
                    object_aware(o_ptr);
                    object_known(o_ptr);
                    if inven_carry_okay(o_ptr) != 0 {
                        let mut i: libc::c_int = 0;
                        inven_carry(o_ptr, 0 as libc::c_int as bool_);
                        i = 0 as libc::c_int;
                        while i < 24 as libc::c_int {
                            if (*p_ptr).inventory[i as usize].tval as
                                   libc::c_int == (*o_ptr).tval as libc::c_int
                                   &&
                                   (*p_ptr).inventory[i as usize].sval as
                                       libc::c_int ==
                                       (*o_ptr).sval as libc::c_int {
                                if message != 0 { inven_item_describe(i); }
                                break ;
                            } else { i += 1 }
                        }
                    } else {
                        drop_near(o_ptr, 0 as libc::c_int,
                                  (*p_ptr).py as libc::c_int,
                                  (*p_ptr).px as libc::c_int);
                    }
                    (*o_ptr).ident =
                        ((*o_ptr).ident as libc::c_int | 0x10 as libc::c_int)
                            as byte_hack
                }
            } else if tocreate < -(1 as libc::c_int) {
                /*It's not valid to create more than one
				 * thing at a time, so if it's less than -1,
				 * it must be time to display a recipe
				 */
                msg_format(b"%d essences of %d\x00" as *const u8 as
                               *const libc::c_char,
                           (*alchemist_recipes.offset(al_idx as isize)).qty as
                               libc::c_int, al_idx);
            } else {
                /* Destroy the essences (tocreate == -1)
				                              * or check for existence(tocreate == 0)*/
                let mut rqty: libc::c_int =
                    (*alchemist_recipes.offset(al_idx as isize)).qty as
                        libc::c_int;
                j = 0 as libc::c_int;
                while j < 24 as libc::c_int {
                    let mut o_ptr_0: *mut object_type =
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset(j as
                                                                         isize)
                            as *mut object_type;
                    if (*o_ptr_0).k_idx as libc::c_int != 0 &&
                           (*o_ptr_0).tval as libc::c_int == 4 as libc::c_int
                           &&
                           (*o_ptr_0).sval as libc::c_int ==
                               (*alchemist_recipes.offset(al_idx as
                                                              isize)).sval_essence
                           && (*o_ptr_0).number as libc::c_int >= rqty {
                        /* At this point, the item is required, destroy it. */
                        if tocreate != 0 {
                            inc_stack_size_ex(j, 0 as libc::c_int - rqty,
                                              OPTIMIZE,
                                              if message as libc::c_int != 0 {
                                                  DESCRIBE as libc::c_int
                                              } else {
                                                  NO_DESCRIBE as libc::c_int
                                              } as describe_flag);
                        }
                        break ;
                    } else { j += 1 }
                }
                if j == 24 as libc::c_int {
                    /* This ingredient was not found, cannot do recipe */
                    return 0 as libc::c_int as bool_
                }
            }
        }
        al_idx += 1
    }
    return exists;
}
/* This function lists all the ingredients
 * needed to create something.
 */
#[no_mangle]
pub unsafe extern "C" fn alchemist_display_recipe(mut tval: libc::c_int,
                                                  mut sval: libc::c_int,
                                                  mut ego: libc::c_int) {
    let mut al_idx: libc::c_int = 0;
    let mut row: libc::c_int = 1 as libc::c_int;
    let mut col: libc::c_int = 15 as libc::c_int;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut ch: libc::c_char = 0;
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
    /* Display the ingredients for a recipe */
    al_idx = 0 as libc::c_int;
    while al_idx < max_al_idx as libc::c_int {
        if ego != 0 &&
               (*alchemist_recipes.offset(al_idx as isize)).sval as
                   libc::c_int == ego &&
               (*alchemist_recipes.offset(al_idx as isize)).tval as
                   libc::c_int == 1 as libc::c_int ||
               ego == 0 &&
                   (*alchemist_recipes.offset(al_idx as isize)).sval as
                       libc::c_int == sval &&
                   (*alchemist_recipes.offset(al_idx as isize)).tval as
                       libc::c_int == tval {
            let mut qty: libc::c_int =
                (*alchemist_recipes.offset(al_idx as isize)).qty as
                    libc::c_int;
            let fresh12 = row;
            row = row + 1;
            c_prt(5 as libc::c_int as byte_hack,
                  format(b"     %d essence%s %s         \x00" as *const u8 as
                             *const libc::c_char, qty,
                         if qty > 1 as libc::c_int {
                             b"s\x00" as *const u8 as *const libc::c_char
                         } else {
                             b"\x00" as *const u8 as *const libc::c_char
                         },
                         k_name.offset((*k_info.offset(lookup_kind(4 as
                                                                       libc::c_int,
                                                                   (*alchemist_recipes.offset(al_idx
                                                                                                  as
                                                                                                  isize)).sval_essence)
                                                           as isize)).name as
                                           isize)) as cptr, fresh12, col);
        }
        al_idx += 1
    }
    let fresh13 = row;
    row = row + 1;
    c_prt(1 as libc::c_int as byte_hack,
          b"                                                 \x00" as
              *const u8 as *const libc::c_char, fresh13, col);
    if ego == 0 {
        /* Find the name of that object */
        o_ptr = &mut forge;
        object_prep(o_ptr, lookup_kind(tval, sval) as libc::c_int);
        (*o_ptr).name2 = ego as s16b;
        hack_apply_magic_power = -(99 as libc::c_int);
        apply_magic(o_ptr,
                    get_skill(39 as libc::c_int) as libc::c_int *
                        2 as libc::c_int, 0 as libc::c_int as bool_,
                    0 as libc::c_int as bool_, 0 as libc::c_int as bool_);
        object_aware(o_ptr);
        object_known(o_ptr);
        /* the 0 mode means only the text, leaving off any numbers */
        object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
    } else {
        /* Display the ego item name */
        strcpy(o_name.as_mut_ptr(),
               e_name.offset((*e_info.offset(ego as isize)).name as isize));
    }
    /* Display a short message about it, and wait for a key. */
    get_com(format(b"ingredients needed to create a %s\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr()) as cptr,
            &mut ch);
}
/*
 *
 * The alchemist_recipe_select was copied from
 * wiz_create_itemtype
 * and then changed quite a bit.
 *
 */
/*
 The select array is a simple array of 'use this char to select item x'
 It has 88 items (three columns of 20 each)
 selectitem is initilized with the reverse mappings:
 selectitem[selectchar[x]] == x is always true.
 */
#[no_mangle]
pub static mut selectchar: [libc::c_char; 90] =
    unsafe {
        *::std::mem::transmute::<&[u8; 90],
                                 &mut [libc::c_char; 90]>(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*():;,.<=>[]{}/=?+\'~\x00")
    };
#[no_mangle]
pub static mut selectitem: [byte_hack; 256] = [0; 256];
#[no_mangle]
pub unsafe extern "C" fn strip_and_print(mut str: *mut libc::c_char,
                                         mut color: libc::c_int,
                                         mut num: libc::c_int) {
    let mut row: libc::c_int = 2 as libc::c_int + num % 20 as libc::c_int;
    let mut col: libc::c_int = 40 as libc::c_int * (num / 20 as libc::c_int);
    let mut ch: libc::c_int = 0;
    let mut max_len: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    if num > 60 as libc::c_int {
        msg_print(b"Attempting to display too many items!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    ch = selectchar[num as usize] as libc::c_int;
    if selectitem[ch as usize] as libc::c_int != num {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            selectitem[i as usize] = 0xff as libc::c_int as byte_hack;
            i += 1
        }
        i = 0 as libc::c_int;
        while selectchar[i as usize] != 0 {
            selectitem[selectchar[i as usize] as byte_hack as usize] =
                i as byte_hack;
            i += 1
        }
    }
    /* Skip past leading characters */
    while *str as libc::c_int == ' ' as i32 ||
              *str as libc::c_int == '&' as i32 {
        str = str.offset(1)
    }
    /* Copy useful chars */
    string = buf.as_mut_ptr();
    while *str != 0 {
        if *str as libc::c_int != '~' as i32 {
            let fresh14 = string;
            string = string.offset(1);
            *fresh14 = *str
        }
        str = str.offset(1)
    }
    /* Terminate the new name */
    *string = '\u{0}' as i32 as libc::c_char;
    /* strip the name down to size
	 if (76-col < (signed)max_len)
	 max_len = 76-col;
	 else
	 max_len = 30-6;*/
    max_len = 39 as libc::c_int;
    string = buf.as_mut_ptr();
    if strlen(string) > max_len as libc::c_uint as libc::c_ulong {
        string =
            string.offset(strlen(string).wrapping_sub(max_len as
                                                          libc::c_ulong) as
                              isize)
    }
    /* Print it */
    c_prt(color as byte_hack,
          format(b"[%c] %s\x00" as *const u8 as *const libc::c_char, ch,
                 string) as cptr, row, col);
}
/* Display a list of recipes that need a particular essence.
 * Note that we display a list of essences first,
 * so in effect, this is the alchemist's recipe book.
 */
#[no_mangle]
pub unsafe extern "C" fn alchemist_recipe_book() {
    let mut num: libc::c_int = 0;
    let mut max_num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut al_idx: libc::c_int = 0;
    let mut bat: libc::c_int = 0;
    let mut kidx: libc::c_int = 0;
    let mut choice: [libc::c_int; 61] = [0; 61];
    let mut choice2: [libc::c_int; 61] = [0; 61];
    let mut mod40: libc::c_int = 0;
    let mut essence: [bool_; 19] = [0; 19];
    let mut ch: libc::c_char = 0;
    /* Save and clear the screen */
    character_icky = 1 as libc::c_int as bool_; /*show recipes*/
    Term_save();
    loop  {
        Term_clear();
        num = 0 as libc::c_int;
        /* Display bateries */
        /* start with assumption that the alchemist knows about no recipes */
        i = 0 as libc::c_int;
        while i < 18 as libc::c_int + 1 as libc::c_int {
            essence[i as usize] = 0 as libc::c_int as bool_;
            i += 1
        }
        /* cycle through all alchemist recipes */
        al_idx = 0 as libc::c_int;
        while al_idx < max_al_idx as libc::c_int {
            /* if we aren't already going to display this essence */
            if essence[(*alchemist_recipes.offset(al_idx as
                                                      isize)).sval_essence as
                           usize] == 0 {
                /*Note that we don't display artifact recipes here...*/
				/*This is partially because artifacts often require exotic
				 ingredients as well */
                if !((*alchemist_recipes.offset(al_idx as isize)).tval == 0) {
                    if (*alchemist_recipes.offset(al_idx as isize)).tval as
                           libc::c_int == 1 as libc::c_int {
                        if alchemist_known_egos[((*alchemist_recipes.offset(al_idx
                                                                                as
                                                                                isize)).sval
                                                     as libc::c_int /
                                                     32 as libc::c_int) as
                                                    usize] &
                               ((1 as libc::c_int) <<
                                    (*alchemist_recipes.offset(al_idx as
                                                                   isize)).sval
                                        as libc::c_int % 32 as libc::c_int) as
                                   libc::c_uint != 0 {
                            essence[(*alchemist_recipes.offset(al_idx as
                                                                   isize)).sval_essence
                                        as usize] = 1 as libc::c_int as bool_
                        }
                    } else {
                        kidx =
                            lookup_kind((*alchemist_recipes.offset(al_idx as
                                                                       isize)).tval
                                            as libc::c_int,
                                        (*alchemist_recipes.offset(al_idx as
                                                                       isize)).sval
                                            as libc::c_int) as libc::c_int;
                        if (*alchemist_recipes.offset(al_idx as isize)).tval
                               as libc::c_int != 1 as libc::c_int &&
                               (*k_info.offset(kidx as isize)).know as
                                   libc::c_int != 0 {
                            essence[(*alchemist_recipes.offset(al_idx as
                                                                   isize)).sval_essence
                                        as usize] = 1 as libc::c_int as bool_
                        }
                    }
                }
            }
            al_idx += 1
        }
        let mut current_block_22: u64;
        num = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 18 as libc::c_int + 7 as libc::c_int {
            if essence[i as usize] as libc::c_int != 0 ||
                   i > 18 as libc::c_int {
                let mut kidx_0: libc::c_int =
                    lookup_kind(4 as libc::c_int, i) as libc::c_int;
                if i > 18 as libc::c_int {
                    match i {
                        19 => {
                            current_block_22 = 10561433309897793409;
                            match current_block_22 {
                                7561792199208311900 => {
                                    strip_and_print(b"Amulets\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15972130623657383834 => {
                                    strip_and_print(b"Potions\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                16423844523754167959 => {
                                    strip_and_print(b"Wands\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15269270707378459463 => {
                                    strip_and_print(b"Rings\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                12191487335605195799 => {
                                    strip_and_print(b"Staves\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                _ => {
                                    strip_and_print(b"Scrolls\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                            }
                            current_block_22 = 5529461102203738653;
                        }
                        20 => {
                            current_block_22 = 15972130623657383834;
                            match current_block_22 {
                                7561792199208311900 => {
                                    strip_and_print(b"Amulets\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15972130623657383834 => {
                                    strip_and_print(b"Potions\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                16423844523754167959 => {
                                    strip_and_print(b"Wands\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15269270707378459463 => {
                                    strip_and_print(b"Rings\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                12191487335605195799 => {
                                    strip_and_print(b"Staves\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                _ => {
                                    strip_and_print(b"Scrolls\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                            }
                            current_block_22 = 5529461102203738653;
                        }
                        21 => {
                            current_block_22 = 16423844523754167959;
                            match current_block_22 {
                                7561792199208311900 => {
                                    strip_and_print(b"Amulets\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15972130623657383834 => {
                                    strip_and_print(b"Potions\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                16423844523754167959 => {
                                    strip_and_print(b"Wands\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15269270707378459463 => {
                                    strip_and_print(b"Rings\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                12191487335605195799 => {
                                    strip_and_print(b"Staves\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                _ => {
                                    strip_and_print(b"Scrolls\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                            }
                            current_block_22 = 5529461102203738653;
                        }
                        22 => {
                            current_block_22 = 15269270707378459463;
                            match current_block_22 {
                                7561792199208311900 => {
                                    strip_and_print(b"Amulets\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15972130623657383834 => {
                                    strip_and_print(b"Potions\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                16423844523754167959 => {
                                    strip_and_print(b"Wands\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15269270707378459463 => {
                                    strip_and_print(b"Rings\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                12191487335605195799 => {
                                    strip_and_print(b"Staves\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                _ => {
                                    strip_and_print(b"Scrolls\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                            }
                            current_block_22 = 5529461102203738653;
                        }
                        23 => {
                            current_block_22 = 12191487335605195799;
                            match current_block_22 {
                                7561792199208311900 => {
                                    strip_and_print(b"Amulets\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15972130623657383834 => {
                                    strip_and_print(b"Potions\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                16423844523754167959 => {
                                    strip_and_print(b"Wands\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15269270707378459463 => {
                                    strip_and_print(b"Rings\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                12191487335605195799 => {
                                    strip_and_print(b"Staves\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                _ => {
                                    strip_and_print(b"Scrolls\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                            }
                            current_block_22 = 5529461102203738653;
                        }
                        24 => {
                            current_block_22 = 7561792199208311900;
                            match current_block_22 {
                                7561792199208311900 => {
                                    strip_and_print(b"Amulets\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15972130623657383834 => {
                                    strip_and_print(b"Potions\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                16423844523754167959 => {
                                    strip_and_print(b"Wands\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                15269270707378459463 => {
                                    strip_and_print(b"Rings\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                12191487335605195799 => {
                                    strip_and_print(b"Staves\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                                _ => {
                                    strip_and_print(b"Scrolls\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    1 as libc::c_int, num);
                                }
                            }
                            current_block_22 = 5529461102203738653;
                        }
                        _ => { current_block_22 = 11042950489265723346; }
                    }
                } else {
                    /* add this essence to the list*/
                    strip_and_print(k_name.offset((*k_info.offset(kidx_0 as
                                                                      isize)).name
                                                      as isize),
                                    1 as libc::c_int, num);
                    current_block_22 = 5529461102203738653;
                }
                match current_block_22 {
                    11042950489265723346 => { }
                    _ => {
                        let fresh15 = num;
                        num = num + 1;
                        choice[fresh15 as usize] = i
                    }
                }
            }
            i += 1
        }
        max_num = num;
        if max_num == 0 as libc::c_int {
            /*Note that this should never actually happen, as any skill
			 at alchemy automatically gets you some recipes, and this
			 procedure shouldn't be called for players without alchemist skill
			 */
            msg_print(b"You don\'t know any recipes!\x00" as *const u8 as
                          *const libc::c_char);
            msg_print(b"You can\'t be an alchemist without recipes!\x00" as
                          *const u8 as *const libc::c_char);
            break ;
        } else {
            while num == 0xff as libc::c_int || num >= max_num {
                ch = selectchar[(max_num - 1 as libc::c_int) as usize];
                /* Choose! */
                if max_num == 0 as libc::c_int ||
                       get_com(format(b"Which Type of Recipe?[a-%c]\x00" as
                                          *const u8 as *const libc::c_char,
                                      selectchar[(max_num - 1 as libc::c_int)
                                                     as usize] as libc::c_int)
                                   as cptr, &mut ch) == 0 {
                    break ;
                }
                /* Analyze choice - note that the cast to byte prevents overflow*/
                num = selectitem[ch as byte_hack as usize] as libc::c_int
            }
            /* This break, and the break for no recipes above,
		 are the only exits from this procedure.
		 */
            if num == 0xff as libc::c_int || num >= max_num { break ; }
            /* Save the baterie index */
            bat = choice[num as usize];
            num = 0 as libc::c_int;
            /*Display the 'type of object' recipe screen*/
            if bat > 18 as libc::c_int {
                let mut tval: libc::c_int = 0;
                match bat {
                    19 => { tval = 70 as libc::c_int }
                    20 => { tval = 71 as libc::c_int }
                    21 => { tval = 65 as libc::c_int }
                    22 => { tval = 45 as libc::c_int }
                    23 => { tval = 55 as libc::c_int }
                    24 => { tval = 40 as libc::c_int }
                    _ => { }
                }
                Term_load();
                alchemist_recipe_select(&mut tval, 0 as libc::c_int,
                                        0 as libc::c_int,
                                        1 as libc::c_int as bool_);
                Term_save();
            } else {
                mod40 = 0 as libc::c_int;
                loop  {
                    let mut skipped: libc::c_int = 0;
                    Term_clear();
                    num = 0 as libc::c_int;
                    if mod40 != 0 {
                        strip_and_print(b"--MORE--\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        1 as libc::c_int, num);
                        choice[num as usize] = -(2 as libc::c_int);
                        let fresh16 = num;
                        num = num + 1;
                        choice2[fresh16 as usize] = 0 as libc::c_int
                    }
                    let mut current_block_68: u64;
                    /* Display all items made with this essence */
                    al_idx = 0 as libc::c_int; /*Loop through tidx/sidx*/
                    skipped = 0 as libc::c_int;
                    while al_idx < max_al_idx as libc::c_int {
                        if (*alchemist_recipes.offset(al_idx as
                                                          isize)).sval_essence
                               == bat {
                            let mut sval: libc::c_int =
                                (*alchemist_recipes.offset(al_idx as
                                                               isize)).sval as
                                    libc::c_int;
                            let mut tval_0: libc::c_int =
                                (*alchemist_recipes.offset(al_idx as
                                                               isize)).tval as
                                    libc::c_int;
                            let mut names: [libc::c_char; 200] =
                                *::std::mem::transmute::<&[u8; 200],
                                                         &mut [libc::c_char; 200]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
                            if (*alchemist_recipes.offset(al_idx as
                                                              isize)).tval as
                                   libc::c_int == 1 as libc::c_int {
                                /* Ego items */
                                let mut e_ptr: *mut ego_item_type =
                                    &mut *e_info.offset(sval as isize) as
                                        *mut ego_item_type;
                                let mut j: libc::c_int = 0;
                                let mut k: libc::c_int = 0;
                                if alchemist_known_egos[(sval /
                                                             32 as
                                                                 libc::c_int)
                                                            as usize] &
                                       ((1 as libc::c_int) <<
                                            sval % 32 as libc::c_int) as
                                           libc::c_uint == 0 {
                                    current_block_68 = 5793491756164225964;
                                } else {
                                    j = 0 as libc::c_int;
                                    while j < 6 as libc::c_int &&
                                              (*e_ptr).tval[j as usize] as
                                                  libc::c_int != 0 {
                                        if !(j > 0 as libc::c_int &&
                                                 (*e_ptr).tval[j as usize] as
                                                     libc::c_int ==
                                                     (*e_ptr).tval[(j -
                                                                        1 as
                                                                            libc::c_int)
                                                                       as
                                                                       usize]
                                                         as libc::c_int) {
                                            k = 0 as libc::c_int;
                                            while (*tvals.as_mut_ptr().offset(k
                                                                                  as
                                                                                  isize)).tval
                                                      != 0 {
                                                if (*tvals.as_mut_ptr().offset(k
                                                                                   as
                                                                                   isize)).tval
                                                       ==
                                                       (*e_ptr).tval[j as
                                                                         usize]
                                                           as libc::c_int {
                                                    strcat(names.as_mut_ptr(),
                                                           (*tvals.as_mut_ptr().offset(k
                                                                                           as
                                                                                           isize)).desc);
                                                    strcat(names.as_mut_ptr(),
                                                           b", \x00" as
                                                               *const u8 as
                                                               *const libc::c_char);
                                                    break ;
                                                } else { k += 1 }
                                            }
                                        }
                                        j += 1
                                    }
                                    strcat(names.as_mut_ptr(),
                                           e_name.offset((*e_ptr).name as
                                                             isize));
                                    current_block_68 = 9521147444787763968;
                                }
                            } else {
                                /* Normal Items */
                                let mut kidx_1: libc::c_int =
                                    lookup_kind(tval_0, sval) as libc::c_int;
                                let mut k_0: libc::c_int = 0;
                                if (*k_info.offset(kidx_1 as isize)).know == 0
                                   {
                                    current_block_68 = 5793491756164225964;
                                } else {
                                    k_0 = 0 as libc::c_int;
                                    while (*tvals.as_mut_ptr().offset(k_0 as
                                                                          isize)).tval
                                              != 0 {
                                        if (*tvals.as_mut_ptr().offset(k_0 as
                                                                           isize)).tval
                                               == tval_0 {
                                            strcat(names.as_mut_ptr(),
                                                   (*tvals.as_mut_ptr().offset(k_0
                                                                                   as
                                                                                   isize)).desc);
                                            break ;
                                        } else { k_0 += 1 }
                                    }
                                    strcat(names.as_mut_ptr(),
                                           b" of \x00" as *const u8 as
                                               *const libc::c_char);
                                    strcat(names.as_mut_ptr(),
                                           k_name.offset((*k_info.offset(kidx_1
                                                                             as
                                                                             isize)).name
                                                             as isize));
                                    current_block_68 = 9521147444787763968;
                                }
                            }
                            match current_block_68 {
                                5793491756164225964 => { }
                                _ =>
                                /*Skip the first mod40 pages of recipes*/
                                {
                                    let fresh17 = skipped;
                                    skipped = skipped + 1;
                                    if !(fresh17 < mod40 * 38 as libc::c_int)
                                       {
                                        /* add this object kind to the list*/
                                        strip_and_print(names.as_mut_ptr(),
                                                        1 as libc::c_int,
                                                        num);
                                        choice[num as usize] = tval_0;
                                        let fresh18 = num;
                                        num = num + 1;
                                        choice2[fresh18 as usize] = sval;
                                        if num > 38 as libc::c_int {
                                            strip_and_print(b"--MORE--\x00" as
                                                                *const u8 as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            1 as libc::c_int,
                                                            num);
                                            choice[num as usize] =
                                                -(1 as libc::c_int);
                                            let fresh19 = num;
                                            num = num + 1;
                                            choice2[fresh19 as usize] =
                                                0 as libc::c_int;
                                            break ;
                                        }
                                    }
                                }
                            }
                        }
                        al_idx += 1
                    }
                    max_num = num;
                    while num == 0xff as libc::c_int || num >= max_num {
                        ch =
                            selectchar[(max_num - 1 as libc::c_int) as usize];
                        /* Choose! */
                        if max_num == 0 as libc::c_int ||
                               get_com(format(b"Examine which recipe?[%c-%c]\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              selectchar[0 as libc::c_int as
                                                             usize] as
                                                  libc::c_int,
                                              ch as libc::c_int) as cptr,
                                       &mut ch) == 0 {
                            break ;
                        }
                        /* Analyze choice */
                        num =
                            selectitem[ch as byte_hack as usize] as
                                libc::c_int
                    }
                    if choice[num as usize] < 0 as libc::c_int {
                        if choice[num as usize] < -(1 as libc::c_int) {
                            mod40 -= 1
                        } else { mod40 += 1 }
                    } else {
                        if num == 0xff as libc::c_int || num >= max_num {
                            break ;
                        }
                        /* Display the recipe */
                        if choice[num as usize] == 1 as libc::c_int {
                            alchemist_display_recipe(0 as libc::c_int,
                                                     0 as libc::c_int,
                                                     choice2[num as usize]);
                        } else {
                            alchemist_display_recipe(choice[num as usize],
                                                     choice2[num as usize],
                                                     0 as libc::c_int);
                        }
                    }
                }
            }
        }
    }
    /* Restore screen contents */
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
}
/* Display a list of known recipies that can be made with
 * materials on hand (including the passed tval). Also
 * calls the recipe_display function, if requested by the
 * player or there aren't enough essences to make the
 * requested object.
 *
 * Note: sval is ignored if !ego, tval is the only determinant
 * of what recipies are available otherwise.
 *
 * This function needs to be able to scroll a list, because
 * there are SO MANY potions. :)
 */
#[no_mangle]
pub unsafe extern "C" fn alchemist_recipe_select(mut tval: *mut libc::c_int,
                                                 mut sval: libc::c_int,
                                                 mut ego: libc::c_int,
                                                 mut recipe: bool_)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut mod40: libc::c_int = 0 as libc::c_int;
    let mut num: libc::c_int = 0;
    let mut max_num: libc::c_int = 0 as libc::c_int;
    let mut tval_desc2: cptr = b"\x00" as *const u8 as *const libc::c_char;
    let mut ch: libc::c_char = 0;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    let mut choice: [libc::c_int; 60] = [0; 60];
    let mut validc: [libc::c_int; 60] = [0; 60];
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Save and clear the screen */
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    Term_clear();
    /* Base object type chosen, fill in tval */
    num = 0 as libc::c_int; /*while(!done)*/
    while num < 40 as libc::c_int {
        if (*tvals.as_mut_ptr().offset(num as isize)).tval == *tval {
            tval_desc2 = (*tvals.as_mut_ptr().offset(num as isize)).desc
        }
        num += 1
    }
    while done == 0 {
        Term_clear();
        if ego != 0 {
            /* Find matching ego items */
            num = 0 as libc::c_int;
            i = 1 as libc::c_int;
            while num < 40 as libc::c_int && i < max_e_idx as libc::c_int {
                let mut j: libc::c_int = 0;
                let mut e_ptr: *mut ego_item_type =
                    &mut *e_info.offset(i as isize) as *mut ego_item_type;
                /* Skip if unknown ego type */
                if !(alchemist_known_egos[(i / 32 as libc::c_int) as usize] &
                         ((1 as libc::c_int) << i % 32 as libc::c_int) as
                             libc::c_uint == 0) {
                    /* search in permitted tvals/svals for allowed egos */
                    j = 0 as libc::c_int;
                    while j < 6 as libc::c_int {
                        if (*e_ptr).tval[j as usize] as libc::c_int == *tval
                               &&
                               sval >=
                                   (*e_ptr).min_sval[j as usize] as
                                       libc::c_int &&
                               sval <=
                                   (*e_ptr).max_sval[j as usize] as
                                       libc::c_int {
                            let mut color: libc::c_int = 5 as libc::c_int;
                            /*Reject if not opposite end of name
						 prefixes only on postfix egos,
						 postfixes only on prefix egos.
						 */
                            if !(ego != -(1 as libc::c_int) &&
                                     (*e_ptr).before as libc::c_int ==
                                         (*e_info.offset(ego as isize)).before
                                             as libc::c_int) {
                                /*Color it red of the alchemist doesn't have the essences to create it*/
                                if alchemist_items_check(*tval,
                                                         0 as libc::c_int, i,
                                                         0 as libc::c_int,
                                                         1 as libc::c_int as
                                                             bool_) == 0 {
                                    color = 4 as libc::c_int
                                }
                                /* add this ego to the list*/
                                strip_and_print(e_name.offset((*e_info.offset(i
                                                                                  as
                                                                                  isize)).name
                                                                  as isize),
                                                color, num);
                                validc[num as usize] = color;
                                let fresh20 = num;
                                num = num + 1;
                                choice[fresh20 as usize] = i;
                                break ;
                            }
                        }
                        j += 1
                    }
                }
                i += 1
            }
        } else {
            let mut skipped: libc::c_char = 0 as libc::c_int as libc::c_char;
            num = 0 as libc::c_int;
            if mod40 != 0 as libc::c_int {
                strip_and_print(b"--MORE--\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                1 as libc::c_int, num);
                validc[num as usize] = 1 as libc::c_int;
                let fresh21 = num;
                num = num + 1;
                choice[fresh21 as usize] = -(1 as libc::c_int)
            }
            i = 1 as libc::c_int;
            while num < 39 as libc::c_int && i < max_k_idx as libc::c_int {
                let mut k_ptr: *mut object_kind =
                    &mut *k_info.offset(i as isize) as *mut object_kind;
                /* Analyze matching items */
                if (*k_ptr).tval as libc::c_int == *tval ||
                       (*k_ptr).tval as libc::c_int == 72 as libc::c_int &&
                           *tval == 71 as libc::c_int {
                    let mut color_0: libc::c_char =
                        5 as libc::c_int as libc::c_char;
                    /* Hack -- Skip instant artifacts */
                    if !((*k_ptr).flags3 as libc::c_long &
                             0x800 as libc::c_long != 0) {
                        /*Don't display recipes that the alchemist doesn't know about*/
                        if !((*k_ptr).know == 0 && wizard == 0) {
                            /*Skip recipes that are somehow known, but don't exist*/
                            if !(alchemist_exists((*k_ptr).tval as
                                                      libc::c_int,
                                                  (*k_ptr).sval as
                                                      libc::c_int,
                                                  0 as libc::c_int,
                                                  0 as libc::c_int) == 0) {
                                /* Skip the first 39 if they hit 'more' */
                                let fresh22 = skipped;
                                skipped = skipped + 1;
                                if !((fresh22 as libc::c_int) <
                                         mod40 * 39 as libc::c_int) {
                                    /* Color 'unable to create' items different */
                                    if alchemist_items_check((*k_ptr).tval as
                                                                 libc::c_int,
                                                             (*k_ptr).sval as
                                                                 libc::c_int,
                                                             0 as libc::c_int,
                                                             0 as libc::c_int,
                                                             1 as libc::c_int
                                                                 as bool_) ==
                                           0 {
                                        color_0 =
                                            4 as libc::c_int as libc::c_char
                                    }
                                    /* Acquire the "name" of object "i" */
					/* and print it in it's place */
                                    strip_and_print(k_name.offset((*k_ptr).name
                                                                      as
                                                                      isize),
                                                    color_0 as libc::c_int,
                                                    num);
                                    /* Remember the object index */
                                    validc[num as usize] =
                                        color_0 as libc::c_int;
                                    let fresh23 = num;
                                    num = num + 1;
                                    choice[fresh23 as usize] = i
                                }
                            }
                        }
                    }
                }
                i += 1
            }
            if num == 39 as libc::c_int {
                strip_and_print(b"--MORE--\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                1 as libc::c_int, num);
                validc[num as usize] = 1 as libc::c_int;
                let fresh24 = num;
                num = num + 1;
                choice[fresh24 as usize] = -(1 as libc::c_int)
            }
        }
        /* We need to know the maximal possible remembered object_index */
        max_num = num;
        string =
            b"What Kind of %s? (* to see recipe) [%c-%c,*]\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        num = 0xff as libc::c_int;
        /* Pretend they're all undoable if we where called to display recipes */
        if recipe != 0 {
            num = 0 as libc::c_int;
            while num < max_num {
                if validc[num as usize] != 1 as libc::c_int {
                    validc[num as usize] = 4 as libc::c_int
                }
                num += 1
            }
            string =
                b"show which %s recipe? [%c-%c]\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        while num == 0xff as libc::c_int || num >= max_num {
            ch = selectchar[(max_num - 1 as libc::c_int) as usize];
            /* Choose! */
            if max_num == 0 as libc::c_int ||
                   get_com(format(string as cptr, tval_desc2,
                                  selectchar[0 as libc::c_int as usize] as
                                      libc::c_int, ch as libc::c_int) as cptr,
                           &mut ch) == 0 {
                break ;
            }
            /* Extra breaks for recipe */
            if recipe as libc::c_int != 0 &&
                   (ch as libc::c_int == '\r' as i32 ||
                        ch as libc::c_int == ' ' as i32 ||
                        ch as libc::c_int == '\u{1b}' as i32) {
                break ;
            }
            /* Analyze choice */
            num = selectitem[ch as byte_hack as usize] as libc::c_int;
            /* Pretend that we don't have enough essences for anything */
            if ch as libc::c_int == '*' as i32 {
                num = 0 as libc::c_int;
                while num < max_num {
                    if validc[num as usize] != 1 as libc::c_int {
                        validc[num as usize] = 4 as libc::c_int
                    }
                    num += 1
                }
                string =
                    b"Show which %s recipe? [%c-%c]\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char
            }
        }
        if num == 0xff as libc::c_int || max_num == 0 as libc::c_int ||
               num >= max_num {
            break ;
        }
        if validc[num as usize] == 1 as libc::c_int {
            if num == 0 as libc::c_int { mod40 -= 1 } else { mod40 += 1 }
            if mod40 < 0 as libc::c_int { mod40 = 0 as libc::c_int }
        } else if validc[num as usize] != 5 as libc::c_int {
            /* If we don't have enough essences, or user asked for recipes */
            /* Display the recipe */
            if ego != 0 {
                alchemist_display_recipe(*tval, sval, choice[num as usize]);
            } else {
                alchemist_display_recipe((*k_info.offset(choice[num as usize]
                                                             as isize)).tval
                                             as libc::c_int,
                                         (*k_info.offset(choice[num as usize]
                                                             as isize)).sval
                                             as libc::c_int,
                                         0 as libc::c_int);
            }
        } else { done = 1 as libc::c_int as bool_ }
    }
    /* Restore screen contents */
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    /* User abort, or no choices */
    if max_num == 0 as libc::c_int || num == 0xff as libc::c_int ||
           num >= max_num {
        if max_num == 0 as libc::c_int {
            msg_print(b"You don\'t know of anything you can make using that.\x00"
                          as *const u8 as *const libc::c_char);
        }
        return -(1 as libc::c_int)
    }
    if validc[num as usize] != 5 as libc::c_int { return -(1 as libc::c_int) }
    /* And return successful */
    if ego != 0 { return choice[num as usize] }
    /* Set the tval, should be the same unless they selected a potion2 */
    if *tval !=
           (*k_info.offset(choice[num as usize] as isize)).tval as libc::c_int
           && *tval != 71 as libc::c_int {
        msg_print(b"Coding error: tval != TV_POTION\x00" as *const u8 as
                      *const libc::c_char);
    }
    *tval =
        (*k_info.offset(choice[num as usize] as isize)).tval as libc::c_int;
    return (*k_info.offset(choice[num as usize] as isize)).sval as
               libc::c_int;
}
/* Set the 'known' flags for all objects with a level <= lev
 * This lets the budding alchemist create basic items.
 */
#[no_mangle]
pub unsafe extern "C" fn alchemist_learn_all(mut lev: libc::c_int) {
    let mut i: libc::c_int = 0;
    if get_skill(39 as libc::c_int) == 0 { return }
    /* msg_format("You learn about level %d items",lev); */
    i = 0 as libc::c_int;
    while i < max_k_idx as libc::c_int {
        if (*k_info.offset(i as isize)).level as libc::c_int <= lev {
            if alchemist_exists((*k_info.offset(i as isize)).tval as
                                    libc::c_int,
                                (*k_info.offset(i as isize)).sval as
                                    libc::c_int, 0 as libc::c_int,
                                0 as libc::c_int) != 0 {
                (*k_info.offset(i as isize)).know = 1 as libc::c_int as bool_
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn alchemist_learn_ego(mut ego: libc::c_int) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    /* some Paranoia*/
    if ego == 0 || ego >= max_e_idx as libc::c_int { return }
    /* Get the ego items name */
    name = e_name.offset((*e_info.offset(ego as isize)).name as isize);
    while !strchr(name, ' ' as i32).is_null() {
        name = strchr(name, ' ' as i32).offset(1 as libc::c_int as isize)
    }
    /* Don't learn about egos without recipes, and
	 * always learn about the passed ego item. */
    if alchemist_exists(0 as libc::c_int, 0 as libc::c_int, ego,
                        0 as libc::c_int) != 0 {
        alchemist_known_egos[(ego / 32 as libc::c_int) as usize] |=
            ((1 as libc::c_int) << ego % 32 as libc::c_int) as libc::c_uint
        /* msg_format("You learn about '%s' ego items.",e_name+e_info[ego].name); */
    } else { return }
    /* Don't mass learn about egos that have no name. */
    if *name.offset(0 as libc::c_int as isize) as libc::c_int ==
           0 as libc::c_int {
        return
    }
    /* Look through all ego's for matching name */
	/* Note that the original ego is marked here too */
    i = 0 as libc::c_int;
    while i < max_e_idx as libc::c_int {
        if !strstr(e_name.offset((*e_info.offset(i as isize)).name as isize),
                   name).is_null() &&
               alchemist_exists(0 as libc::c_int, 0 as libc::c_int, i,
                                0 as libc::c_int) as libc::c_int != 0 &&
               alchemist_known_egos[(i / 32 as libc::c_int) as usize] &
                   ((1 as libc::c_int) << i % 32 as libc::c_int) as
                       libc::c_uint == 0 {
            /*Not already known*/
            /*&& (e_name+e_info[i].name)[0])non-blank name*/
            alchemist_known_egos[(i / 32 as libc::c_int) as usize] |=
                ((1 as libc::c_int) << i % 32 as libc::c_int) as libc::c_uint
            /* msg_format("You learn about '%s' ego items.",e_name+e_info[i].name); */
        }
        i += 1
    };
}
/* Alchemist has learned about a new item.
 * Learn about not only it, but ALL egos with the
 * same name.
 */
#[no_mangle]
pub unsafe extern "C" fn alchemist_learn_object(mut o_ptr: *mut object_type)
 -> libc::c_int {
    /* Allow alchemist to create this item,
	 and.. learn about it even if the player
	 doesn't currently have the alchemy skill
	 */
    (*k_info.offset((*o_ptr).k_idx as isize)).know =
        1 as libc::c_int as bool_;
    /* Not Paranoia, identify_fully calls this always */
    if get_skill(39 as libc::c_int) == 0 { return 0 as libc::c_int }
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
        let mut o_name: [libc::c_char; 80] = [0; 80];
        let mut f1: u32b = 0;
        let mut f2: u32b = 0;
        let mut f3: u32b = 0;
        let mut f4: u32b = 0;
        let mut f5: u32b = 0;
        let mut esp: u32b = 0;
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        /* Randarts and normal artifacts both*/
        alchemist_known_artifacts[0 as libc::c_int as usize] |= f1;
        alchemist_known_artifacts[1 as libc::c_int as usize] |= f2;
        alchemist_known_artifacts[2 as libc::c_int as usize] |= f3;
        alchemist_known_artifacts[3 as libc::c_int as usize] |= f4;
        alchemist_known_artifacts[4 as libc::c_int as usize] |= f5;
        alchemist_known_artifacts[5 as libc::c_int as usize] |= esp;
        object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                    0 as libc::c_int);
        msg_format(b"You learn all about the abilities of %s!\x00" as
                       *const u8 as *const libc::c_char, o_name.as_mut_ptr());
    }
    if (*o_ptr).name2 != 0 {
        alchemist_learn_ego((*o_ptr).name2 as libc::c_int);
    }
    if (*o_ptr).name2b != 0 {
        alchemist_learn_ego((*o_ptr).name2b as libc::c_int);
    }
    return 1 as libc::c_int;
}
/* Alchemist has gained a level - set the ego flags
 * for all egos <= lev/4.
 */
#[no_mangle]
pub unsafe extern "C" fn alchemist_gain_level(mut lev: libc::c_int) {
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
    let mut o_ptr: *mut object_type = &mut forge;
    if lev == 0 as libc::c_int {
        /* Learn about potions of Detonation */
        (*k_info.offset(417 as libc::c_int as isize)).know =
            1 as libc::c_int as bool_
    }
    if lev == 5 as libc::c_int {
        let mut ego: libc::c_int = 0;
        let mut egos: [libc::c_int; 13] =
            [7 as libc::c_int, 18 as libc::c_int, 74 as libc::c_int,
             75 as libc::c_int, 76 as libc::c_int, 77 as libc::c_int,
             78 as libc::c_int, 115 as libc::c_int, 116 as libc::c_int,
             122 as libc::c_int, 123 as libc::c_int, 137 as libc::c_int,
             0 as libc::c_int];
        object_wipe(o_ptr);
        /* learn about some basic ego items */
		/* Note that this is just to get you started. */
        ego = 0 as libc::c_int;
        while egos[ego as usize] != 0 {
            (*o_ptr).name2 = egos[ego as usize] as s16b;
            alchemist_learn_object(o_ptr);
            ego += 1
        }
        msg_print(b"You recall your old master teaching you about elemental item infusing.\x00"
                      as *const u8 as *const libc::c_char);
    }
    if lev == 10 as libc::c_int {
        /*For 'hard rooms' Players only, learn about diggers.*/
        if ironman_rooms != 0 {
            msg_print(b"There\'s gotta be an easier way to get into all these vaults!\x00"
                          as *const u8 as
                          *const libc::c_char); /* Ego item, 'of digging' */
            object_wipe(o_ptr);
            (*o_ptr).name2 = 101 as libc::c_int as s16b;
            alchemist_learn_object(o_ptr);
        }
    }
    if lev == 25 as libc::c_int {
        msg_print(b"You recall your old master reminiscing about legendary infusings\x00"
                      as *const u8 as *const libc::c_char);
        msg_print(b"and the Philosophers\' stone.\x00" as *const u8 as
                      *const libc::c_char);
        /* No auto-learn on artifacts - by this level, you'll have *ID*'d several */
    }
    if lev == 25 as libc::c_int {
        msg_print(b"You wonder about shocking daggers of slay evil.\x00" as
                      *const u8 as *const libc::c_char);
    }
    if lev == 50 as libc::c_int {
        /* learn about Temporary item creation */
		/* Note that this is the ONLY way to learn this,
		 because spells which create a temporary item
		 also fully ID it. */
        alchemist_known_artifacts[4 as libc::c_int as usize] =
            (alchemist_known_artifacts[4 as libc::c_int as usize] as
                 libc::c_long | 0x1 as libc::c_long) as u32b;
        msg_print(b"It suddenly occurs to you that artifacts don\'t *HAVE* to be permanent...\x00"
                      as *const u8 as *const libc::c_char);
    }
    /* Every Four Levels, learn about items that are
	 * less than that.
	 * Note that this isn't a significant effect after the
	 * first few levels, as the level at which you are learning
	 * things here quickly drops behind the level at which you
	 * are finding items.
	 */
    if lev & 0x3 as libc::c_int != 0 as libc::c_int { return }
    lev = (lev >> 2 as libc::c_int) + 1 as libc::c_int;
    alchemist_learn_all(lev);
}
/* This, in combination with some code in loadsave.c,
 insures that alchemist_gain_level is called EXACTLY
 once with each possible value during the characters
 lifetime.
 */
#[no_mangle]
pub unsafe extern "C" fn alchemist_check_level() {
    let mut lev: u32b = get_skill(39 as libc::c_int) as u32b;
    if alchemist_gained > lev { return }
    /*Paranoia*/
    if lev == 0 { return }
    while alchemist_gained <= lev {
        let fresh25 = alchemist_gained;
        alchemist_gained = alchemist_gained.wrapping_add(1);
        alchemist_gain_level(fresh25 as libc::c_int);
    };
}
/*
 * do_cmd_cast calls this function if the player's class
 * is 'alchemist'.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_alchemist() {
    let mut item: libc::c_int = 0;
    let mut ext: libc::c_int = 0 as libc::c_int;
    let mut value: libc::c_int = 0;
    let mut basechance: libc::c_int = 0;
    let mut askill: libc::c_int = 0;
    let mut repeat: bool_ = 0 as libc::c_int as bool_;
    let mut ch: libc::c_char = 0;
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
    let mut forge2: object_type =
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
    let mut carry_o_ptr: byte_hack = 0 as libc::c_int as byte_hack;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* With the new skill system, we can no longer depend on
	 * check_exp to handle the changes and learning involved in
	 * gaining levels.
	 * So we'll have to check for it here.
	 */
    alchemist_check_level();
    askill = get_skill(39 as libc::c_int) as libc::c_int;
    q_ptr = &mut forge;
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(44 as libc::c_int as
                                                         isize) as
            *mut object_type;
    if (*o_ptr).tval as libc::c_int != 31 as libc::c_int ||
           (*o_ptr).sval as libc::c_int != 1 as libc::c_int {
        msg_print(b"You must wear gloves in order to do alchemy.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    loop  {
        if get_com(b"[P]ower, [R]echarge or [L]eech an item, [E]xtract essences, or recipe [B]ook?\x00"
                       as *const u8 as *const libc::c_char, &mut ch) == 0 {
            ext = 0 as libc::c_int;
            break ;
        } else if ch as libc::c_int == ' ' as i32 {
            ext = 0 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'P' as i32 ||
                      ch as libc::c_int == 'p' as i32 {
            ext = 1 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'E' as i32 ||
                      ch as libc::c_int == 'e' as i32 {
            ext = 2 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'R' as i32 ||
                      ch as libc::c_int == 'r' as i32 {
            ext = 3 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'L' as i32 ||
                      ch as libc::c_int == 'l' as i32 {
            ext = 2 as libc::c_int;
            repeat = 1 as libc::c_int as bool_;
            break ;
        } else {
            if !(ch as libc::c_int == 'B' as i32 ||
                     ch as libc::c_int == 'b' as i32) {
                continue ;
            }
            ext = 4 as libc::c_int;
            break ;
        }
    }
    /* *********Add a power*********/
    if ext == 1 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut qty: libc::c_int = 0;
        let mut tval: libc::c_int = 0;
        let mut sval: libc::c_int = 0 as libc::c_int;
        let mut ego: libc::c_int = 0 as libc::c_int;
        let mut o_name: [libc::c_char; 200] = [0; 200];
        /* *********Extract a power*********/
        q = b"Empower which item? \x00" as *const u8 as *const libc::c_char;
        s =
            b"You have no empowerable items.\x00" as *const u8 as
                *const libc::c_char;
        item_tester_hook =
            Some(item_tester_hook_empower as
                     unsafe extern "C" fn(_: *mut object_type) -> bool_);
        if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int)
               == 0 {
            return
        }
        o_ptr = get_object(item);
        if askill >= 25 as libc::c_int &&
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
                     } else { 0 as libc::c_int }) != 0 ||
                    (*o_ptr).name2 as libc::c_int != 0) &&
               has_ability(7 as libc::c_int) as libc::c_int != 0 {
            if get_check(b"Create an artifact?\x00" as *const u8 as
                             *const libc::c_char) != 0 {
                do_cmd_toggle_artifact(o_ptr);
                return
            } else {
                /* Get an item */
                /* Get the item */
                /* Create an artifact from an ego or double ego item,
		 * from a previous artifact, or finish an artifact
		 */
                /* Don't change artifacts or double ego items further */
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
                       (*o_ptr).name2 as libc::c_int != 0 &&
                           (*o_ptr).name2b as libc::c_int != 0 {
                    return
                }
            }
        }
        if (*o_ptr).name2 != 0 {
            /*Ok, now we have the item, so we can now pick recipes.
		 Note: No recipe is known unless we have 'extracted' from
		 that object type. I.E. the 'know' flag (also greater identify)
		 is set.
		 */
            /* Here we're not setting what kind of ego item it IS,
		 * where' just deciding that it CAN be an ego item */
            /* creating a DUAL ego */
            ego = 1 as libc::c_int
        }
        if ((*o_ptr).tval as libc::c_int) < 40 as libc::c_int &&
               (*o_ptr).tval as libc::c_int != 2 as libc::c_int {
            ego = 1 as libc::c_int
        }
        if (*o_ptr).tval as libc::c_int == 67 as libc::c_int ||
               (*o_ptr).tval as libc::c_int == 115 as libc::c_int ||
               (*o_ptr).tval as libc::c_int == 111 as libc::c_int {
            ego = 1 as libc::c_int
        }
        sval = (*o_ptr).sval as libc::c_int;
        if ego == 0 {
            match (*o_ptr).tval as libc::c_int {
                65 => { sval = 2 as libc::c_int }
                45 => { sval = 50 as libc::c_int }
                55 => { sval = 2 as libc::c_int }
                2 => { sval = 1 as libc::c_int }
                40 => { sval = 16 as libc::c_int }
                70 => { sval = 53 as libc::c_int }
                66 => { sval = 0 as libc::c_int }
                _ => { }
            }
        }
        if (*o_ptr).sval as libc::c_int != sval { ego = 1 as libc::c_int }
        tval = (*o_ptr).tval as libc::c_int;
        sval = (*o_ptr).sval as libc::c_int;
        if tval == 2 as libc::c_int { tval = 71 as libc::c_int }
        if ego != 0 {
            if (*o_ptr).name2 != 0 {
                ego =
                    alchemist_recipe_select(&mut tval, sval,
                                            (*o_ptr).name2 as libc::c_int,
                                            0 as libc::c_int as bool_)
            } else {
                ego =
                    alchemist_recipe_select(&mut tval, sval,
                                            -(1 as libc::c_int),
                                            0 as libc::c_int as bool_)
            }
        } else {
            sval =
                alchemist_recipe_select(&mut tval, 0 as libc::c_int,
                                        0 as libc::c_int,
                                        0 as libc::c_int as bool_)
        }
        if sval < 0 as libc::c_int || ego < 0 as libc::c_int { return }
        if alchemist_items_check(tval, sval, ego, 0 as libc::c_int,
                                 1 as libc::c_int as bool_) == 0 {
            msg_print(b"You do not have enough essences.\x00" as *const u8 as
                          *const libc::c_char);
            return
        }
        energy_use = 100 as libc::c_int;
        alchemist_items_check(tval, sval, ego, -(1 as libc::c_int),
                              1 as libc::c_int as bool_);
        if (*o_ptr).tval as libc::c_int == 16 as libc::c_int ||
               (*o_ptr).tval as libc::c_int == 17 as libc::c_int ||
               (*o_ptr).tval as libc::c_int == 18 as libc::c_int {
            qty = 1 as libc::c_int;
            while qty < (*o_ptr).number as libc::c_int &&
                      alchemist_items_check(tval, sval, ego,
                                            -(1 as libc::c_int),
                                            0 as libc::c_int as bool_) as
                          libc::c_int != 0 {
                qty += 1
            }
        } else { qty = 1 as libc::c_int }
        q_ptr = &mut forge;
        object_copy(q_ptr, o_ptr);
        if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
            /*HACK - bottles don't have the same tval as potions*/
		/*Everything else will have the same tval after empowering*/
            /* Check to make sure we have enough essences */
		/* theoretically this is taken care of by recipe_select*/
		/* but we'll double check just for paranoia. */
            /* Take a turn */
            /* Use up the essences */
            /* Enchant stacks of ammunition at a time */
            /* Copy the object */
            /* distribute charges on wands */
            (*q_ptr).pval = (*o_ptr).pval / (*o_ptr).number as libc::c_int;
            (*o_ptr).pval -= (*q_ptr).pval
        }
        o_ptr = q_ptr;
        (*o_ptr).number = qty as byte_hack;
        carry_o_ptr = 1 as libc::c_int as byte_hack;
        inc_stack_size(item, -qty);
        if ego != 0 {
            let mut pval: libc::c_int = 0;
            let mut pval2: libc::c_int = 0;
            let mut pval3: s32b = 0;
            pval = (*o_ptr).pval;
            pval2 = (*o_ptr).pval2 as libc::c_int;
            pval3 = (*o_ptr).pval3;
            if (*o_ptr).name2 != 0 {
                (*o_ptr).name2b = ego as s16b
            } else { (*o_ptr).name2 = ego as s16b }
            (*o_ptr).pval =
                Rand_div((*e_info.offset(ego as isize)).max_pval -
                             1 as libc::c_int) + 1 as libc::c_int +
                    1 as libc::c_int;
            /* Destroy the initial object */
            /* dilemma - how to prevent creation of cursed items,
			 * without allowing the creation of artifacts?
			 * We can't, unless we want to finalize the ego flags ourselves.
			 */
            apply_magic(o_ptr, askill * 2 as libc::c_int,
                        0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                        0 as libc::c_int as bool_);
            /* Remember what the old pval was, so that we can re-apply it. */
            if (*o_ptr).tval as libc::c_int == 65 as libc::c_int ||
                   (*o_ptr).tval as libc::c_int == 45 as libc::c_int ||
                   (*o_ptr).tval as libc::c_int == 40 as libc::c_int ||
                   (*o_ptr).tval as libc::c_int == 55 as libc::c_int {
                (*o_ptr).pval = pval;
                (*o_ptr).pval2 = pval2 as s16b;
                (*o_ptr).pval3 = pval3
            } else if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
                (*o_ptr).pval = pval
            } else if (*o_ptr).tval as libc::c_int == 111 as libc::c_int &&
                          (*o_ptr).sval as libc::c_int == 255 as libc::c_int {
                (*o_ptr).pval = pval
            } else if (*o_ptr).tval as libc::c_int == 16 as libc::c_int ||
                          (*o_ptr).tval as libc::c_int == 17 as libc::c_int ||
                          (*o_ptr).tval as libc::c_int == 18 as libc::c_int {
                (*o_ptr).pval2 = pval2 as s16b
            } else if (*o_ptr).tval as libc::c_int == 14 as libc::c_int {
                (*o_ptr).pval2 = pval2 as s16b
            }
            /* Calculate failure rate, lev=val/2500+5 */
            value =
                if (*e_info.offset((*o_ptr).name2 as isize)).cost >
                       50000 as libc::c_int {
                    50000 as libc::c_int
                } else { (*e_info.offset((*o_ptr).name2 as isize)).cost };
            if (*o_ptr).name2b != 0 {
                value +=
                    if (*e_info.offset((*o_ptr).name2b as isize)).cost >
                           50000 as libc::c_int {
                        50000 as libc::c_int
                    } else { (*e_info.offset((*o_ptr).name2b as isize)).cost }
            }
            basechance =
                (value / 1000 as libc::c_int + 5 as libc::c_int -
                     get_skill_scale(39 as libc::c_int,
                                     100 as libc::c_int as u32b) as
                         libc::c_int) * 10 as libc::c_int;
            if basechance < 0 as libc::c_int { basechance = 0 as libc::c_int }
            if basechance > 100 as libc::c_int {
                basechance = 100 as libc::c_int
            }
            value = object_value_real(o_ptr)
        } else {
            /* not an ego item */
            o_ptr = &mut forge;
            object_wipe(o_ptr);
            object_prep(o_ptr, lookup_kind(tval, sval) as libc::c_int);
            hack_apply_magic_power = -(99 as libc::c_int);
            apply_magic(o_ptr, askill * 2 as libc::c_int,
                        0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                        0 as libc::c_int as bool_);
            if (*o_ptr).tval as libc::c_int == 65 as libc::c_int ||
                   (*o_ptr).tval as libc::c_int == 55 as libc::c_int {
                (*o_ptr).pval = 0 as libc::c_int
            }
            value = object_value_real(o_ptr);
            basechance =
                (*k_info.offset((*o_ptr).k_idx as isize)).level as libc::c_int
                    - askill * 2 as libc::c_int;
            basechance *= 10 as libc::c_int;
            /* Can't fail more that 100% of the time... */
            if basechance > 100 as libc::c_int {
                basechance = 100 as libc::c_int
            }
            /* Always success in creation of potion of detonations */
            if (*o_ptr).tval as libc::c_int == 71 as libc::c_int &&
                   (*o_ptr).sval as libc::c_int == 22 as libc::c_int {
                basechance /= 10 as libc::c_int
            }
        }
        if alchemist_has_stone() != 0 { basechance = 0 as libc::c_int }
        if basechance > 0 as libc::c_int && value != 0 {
            let mut string: [libc::c_char; 80] = [0; 80];
            string[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            string[1 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_char;
            msg_format(b"The chance of success is only %d%%!\x00" as *const u8
                           as *const libc::c_char,
                       100 as libc::c_int - basechance);
            get_string(b"How much gold do you want to add?\x00" as *const u8
                           as *const libc::c_char, string.as_mut_ptr(),
                       50 as libc::c_int);
            i = atoi(string.as_mut_ptr());
            /* Use up gold to create items */
		/* this has the effect of making the alchemist
		 chronically short of funds, unless he finds the
		 philosopher's stone. It also means the easiest
		 things to make are 'bad', like a potion of
		 detonations...
		 */
		/* Problem - to restrictive. We need something
		 which requires less money. But at the same time,
		 we don't want an 'easy cash' situation. Maybe something
		 like '10% * level difference', meaning at skill level 5,
		 level one items are free? But egos are frequently level
		 zero! Maybe egos are forced to level 25? with a cost ceiling?
		 I mean, Potions and scrolls are really the problem causing the
		 'easy cash' situation, it's ego items. Ego items require
		 relatively few essences, and the rewards are HUGE. Most powerful
		 potions and scrolls require rare essences. Maybe force all egos
		 to require a magic essence? But then you'd get lots of magic
		 from distilling them. Maybe consumed in the creation? then when
		 you got a powerful item, you could make one ego item...
		 But if making things doesn't take gold, what about the cash
		 does the Philosopher's stone do?
		 Time*/
            /* 0% failure if you have the stone */
            /* Note: don't trust the user to enter a positive number... */
            if i < 0 as libc::c_int { i = 0 as libc::c_int }
            if i > (*p_ptr).au { i = (*p_ptr).au }
            if i != 0 {
                basechance = basechance - i * 20 as libc::c_int / value;
                msg_format(b"The chance of success improved to %d%%.\x00" as
                               *const u8 as *const libc::c_char,
                           100 as libc::c_int - basechance);
            }
            if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) < basechance
               {
                /*creation failed, even with the extra gold...*/
                carry_o_ptr = 0 as libc::c_int as byte_hack
            }
            /* Redraw gold */
            (*p_ptr).au -= i;
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x100 as libc::c_long) as
                    u32b
        }
        object_aware(o_ptr);
        object_known(o_ptr);
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x20 as libc::c_int) as
                byte_hack;
        (*o_ptr).found = 9 as libc::c_int as byte_hack;
        object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        if carry_o_ptr != 0 {
            msg_format(b"You have successfully created %s %s\x00" as *const u8
                           as *const libc::c_char,
                       if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                           b"some\x00" as *const u8 as *const libc::c_char
                       } else if is_a_vowel(o_name[0 as libc::c_int as usize]
                                                as libc::c_int) as libc::c_int
                                     != 0 {
                           b"an\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"a\x00" as *const u8 as *const libc::c_char
                       }, o_name.as_mut_ptr());
            if inven_carry_okay(o_ptr) != 0 {
                inven_carry(o_ptr, 0 as libc::c_int as bool_);
            } else {
                drop_near(o_ptr, 0 as libc::c_int, (*p_ptr).py as libc::c_int,
                          (*p_ptr).px as libc::c_int);
                msg_format(b"You drop the %s\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr());
            }
            carry_o_ptr = 0 as libc::c_int as byte_hack
        } else {
            /* Set fully identified
		 * After all, the player just made it...
		 */
            /* don't carry, or in other words... */
            let mut level: libc::c_int =
                (*k_info.offset((*o_ptr).k_idx as isize)).level as
                    libc::c_int;
            if (*o_ptr).name1 != 0 {
                /* created ego item */
                level +=
                    (*e_info.offset((*o_ptr).name2 as isize)).level as
                        libc::c_int
            }
            msg_format(b"Your attempt backfires! Your %s explodes!\x00" as
                           *const u8 as *const libc::c_char,
                       o_name.as_mut_ptr());
            take_hit(damroll(3 as libc::c_int as s16b,
                             (level - askill) as s16b),
                     b"Alchemical Explosion\x00" as *const u8 as
                         *const libc::c_char);
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as
                    u32b
        }
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long |
                      0x8 as libc::c_long)) as u32b;
        item = 0 as libc::c_int;
        while item < 52 as libc::c_int {
            inven_item_optimize(item);
            item += 1
        }
    } else if ext == 2 as libc::c_int {
        let mut ego_0: libc::c_int = 0;
        let mut discharge_stick: bool_ = 0 as libc::c_int as bool_;
        /* Combine / Reorder the pack (later) */
        /* Window stuff */
        /* Optimize the entire p_ptr->inventory - needed because we
		 don't know how many essences where used, and we may
		 have 'used up' a wielded item as well.
		 */
        /* ****** Recharge an item *******/
        let mut s_ptr: *mut object_type = 0 as *mut object_type;
        let mut carry_s_ptr: bool_ = 0 as libc::c_int as bool_;
        item_tester_hook =
            Some(item_tester_hook_extractable as
                     unsafe extern "C" fn(_: *mut object_type) -> bool_);
        q =
            b"Extract from which item? \x00" as *const u8 as
                *const libc::c_char;
        s =
            b"You have no item to extract power from.\x00" as *const u8 as
                *const libc::c_char;
        if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int)
               == 0 {
            return
        }
        o_ptr = get_object(item);
        if ((*o_ptr).tval as libc::c_int == 65 as libc::c_int ||
                (*o_ptr).tval as libc::c_int == 55 as libc::c_int) &&
               (*o_ptr).art_flags4 as libc::c_long & 0x800000 as libc::c_long
                   != 0 {
            msg_print(b"You cannot extract essences after it\'s been magically recharged.\x00"
                          as *const u8 as *const libc::c_char);
            return
        }
        energy_use = 100 as libc::c_int;
        if (*o_ptr).tval as libc::c_int == 67 as libc::c_int &&
               (*o_ptr).pval != 0 as libc::c_int {
            rod_tip_extract(o_ptr);
            return
        }
        loop  {
            /* s_ptr holds the empty items */
            /* Get an item */
            /* Get the item */
            /* This is to prevent creating magic essences by extracting
		 * from a recharged wand of dragon breath or something.
		 */
            /* Take a turn */
            /* Handle Rods before the loop, since they don't stack */
            /* Repeat (for leech command) */
            /* Create the items.
			 * we don't care if they drop to the ground,
			 * and if no action was taken, return
			 */
            ego_0 = 0 as libc::c_int;
            if (*o_ptr).name2 != 0 { ego_0 = (*o_ptr).name2 as libc::c_int }
            /* For ego staves and wands (not of nothing), discharge before extracting the ego */
            discharge_stick =
                ((*o_ptr).pval > 0 as libc::c_int &&
                     ((*o_ptr).tval as libc::c_int == 55 as libc::c_int &&
                          (*o_ptr).sval as libc::c_int != 2 as libc::c_int ||
                          (*o_ptr).tval as libc::c_int == 65 as libc::c_int &&
                              (*o_ptr).sval as libc::c_int !=
                                  2 as libc::c_int)) as libc::c_int as bool_;
            if discharge_stick != 0 { ego_0 = 0 as libc::c_int }
            if alchemist_items_check((*o_ptr).tval as libc::c_int,
                                     (*o_ptr).sval as libc::c_int, ego_0,
                                     1 as libc::c_int,
                                     1 as libc::c_int as bool_) == 0 {
                msg_print(b"You cannot extract anything from that item.\x00"
                              as *const u8 as *const libc::c_char);
                return
            }
            ((*o_ptr).name2b as libc::c_int != 0) &&
                alchemist_items_check((*o_ptr).tval as libc::c_int,
                                      (*o_ptr).sval as libc::c_int,
                                      (*o_ptr).name2b as libc::c_int,
                                      1 as libc::c_int,
                                      1 as libc::c_int as bool_) == 0;
            /* Once in three times, learn how to make the item */
			/* Sorry for the complicated if! Basically, if it's an
			 * unknown regular item or an unknown ego item, there's
			 * a one in 3 chance that it'll be id'd */
            if (ego_0 == 0 &&
                    (*k_info.offset((*o_ptr).k_idx as isize)).know == 0 ||
                    ego_0 != 0 &&
                        alchemist_known_egos[(ego_0 / 32 as libc::c_int) as
                                                 usize] &
                            ((1 as libc::c_int) << ego_0 % 32 as libc::c_int)
                                as libc::c_uint == 0) &&
                   Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                msg_print(b"While destroying it, you gain insight into this item.\x00"
                              as *const u8 as *const libc::c_char);
                /* If over level 10, the player has a chance of 'greater ID'
				 * on extracted items
				 */
                if askill > 9 as libc::c_int {
                    object_out_desc(o_ptr, 0 as *mut FILE,
                                    0 as libc::c_int as bool_,
                                    1 as libc::c_int as bool_);
                }
                alchemist_learn_object(o_ptr);
            }
            /* Always learn what kind of thing it is */
            object_known(o_ptr);
            object_aware(o_ptr);
            /* If it's a wand or staff with charges (but not of nothing),
			 * decrease number of charges, unstacking if needed.
			 * Otherwise, create the 'of nothing' item and destroy the old one.
			 */
            if discharge_stick != 0 {
                /* Unstack staves */
                if (*o_ptr).tval as libc::c_int == 55 as libc::c_int &&
                       (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                    /* Create one local copy of the staff */
                    q_ptr = &mut forge2;
                    object_copy(q_ptr, o_ptr);
                    /* Modify quantity */
                    (*q_ptr).number = 1 as libc::c_int as byte_hack;
                    /* Unstack the copied staff */
                    (*o_ptr).number = (*o_ptr).number.wrapping_sub(1);
                    /* Use the local copy of the staff */
                    o_ptr = q_ptr;
                    carry_o_ptr = 1 as libc::c_int as byte_hack
                }
                /* remove one charge */
                (*o_ptr).pval -= 1
            } else {
                /* Create the empty, plain item */
				/* If the item was already created, increase the number */
                if carry_s_ptr != 0 {
                    (*s_ptr).number = (*s_ptr).number.wrapping_add(1)
                } else {
                    /* Otherwise we must create a local copy of the empty item */
                    let mut tval_0: libc::c_int = 0;
                    let mut sval_0: libc::c_int = 0;
                    let mut create_item: bool_ = 1 as libc::c_int as bool_;
                    tval_0 = (*o_ptr).tval as libc::c_int;
                    if ego_0 == 0 &&
                           (tval_0 == 71 as libc::c_int ||
                                tval_0 == 72 as libc::c_int) {
                        tval_0 = 2 as libc::c_int
                    }
                    sval_0 = (*o_ptr).sval as libc::c_int;
                    if ego_0 == 0 {
                        match tval_0 {
                            65 => { sval_0 = 2 as libc::c_int }
                            45 => { sval_0 = 50 as libc::c_int }
                            55 => { sval_0 = 2 as libc::c_int }
                            2 => { sval_0 = 1 as libc::c_int }
                            40 => { sval_0 = 16 as libc::c_int }
                            70 => { sval_0 = 53 as libc::c_int }
                            66 => { sval_0 = 0 as libc::c_int }
                            _ => { create_item = 0 as libc::c_int as bool_ }
                        }
                    }
                    if create_item != 0 {
                        /* Create the empty item */
                        s_ptr = &mut forge;
                        object_wipe(s_ptr);
                        object_prep(s_ptr,
                                    lookup_kind(tval_0, sval_0) as
                                        libc::c_int);
                        (*s_ptr).number = 1 as libc::c_int as byte_hack;
                        /* Force creation of non ego non cursed */
                        hack_apply_magic_power = -(99 as libc::c_int);
                        apply_magic(s_ptr, 0 as libc::c_int,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_);
                        /* Hack -- remove possible curse */
                        if (*s_ptr).ident as libc::c_int & 0x40 as libc::c_int
                               != 0 {
                            (*s_ptr).art_flags3 =
                                ((*s_ptr).art_flags3 as libc::c_long &
                                     !(0x20000000 as libc::c_long |
                                           0x40000000 as libc::c_long)) as
                                    u32b;
                            (*s_ptr).ident =
                                ((*s_ptr).ident as libc::c_int &
                                     !(0x40 as libc::c_int)) as byte_hack
                        }
                        /* Restore pvals (e.g. charges ==0) of the item */
                        if ego_0 != 0 &&
                               (tval_0 == 65 as libc::c_int ||
                                    tval_0 == 55 as libc::c_int ||
                                    tval_0 == 45 as libc::c_int ||
                                    tval_0 == 40 as libc::c_int) {
                            (*s_ptr).pval = (*o_ptr).pval;
                            (*s_ptr).pval2 = (*o_ptr).pval2;
                            (*s_ptr).pval3 = (*o_ptr).pval3
                        } else if (*o_ptr).tval as libc::c_int ==
                                      111 as libc::c_int &&
                                      (*o_ptr).sval as libc::c_int ==
                                          255 as libc::c_int {
                            (*s_ptr).pval = (*o_ptr).pval
                        } else if (*o_ptr).tval as libc::c_int ==
                                      16 as libc::c_int ||
                                      (*o_ptr).tval as libc::c_int ==
                                          17 as libc::c_int ||
                                      (*o_ptr).tval as libc::c_int ==
                                          18 as libc::c_int {
                            (*s_ptr).pval2 = (*o_ptr).pval2
                        } else if (*o_ptr).tval as libc::c_int ==
                                      14 as libc::c_int {
                            (*s_ptr).pval2 = (*o_ptr).pval2
                        }
                        object_aware(s_ptr);
                        object_known(s_ptr);
                        (*s_ptr).ident =
                            ((*s_ptr).ident as libc::c_int |
                                 0x10 as libc::c_int) as byte_hack;
                        /* Restore the spell stored in a random book */
                        /* Restore the type of explosive ammo */
                        /* Restore the music stored in an instrument */
                        /* The empty item will be added to the p_ptr->inventory later */
                        carry_s_ptr = 1 as libc::c_int as bool_
                    }
                }
                /* Now, we can delete the original (leeched) object.
				 * Is o_ptr an p_ptr->inventory / floor item or a local copy?
				 */
                if carry_o_ptr == 0 {
                    /* Break the leech-loop if it was the last item */
                    if (*o_ptr).number as libc::c_int == 1 as libc::c_int {
                        repeat = 0 as libc::c_int as bool_
                    }
                    inc_stack_size(item, -(1 as libc::c_int));
                } else {
                    /* Forget the local object */
                    carry_o_ptr = 0 as libc::c_int as byte_hack;
                    /* reset o_ptr to the original stack,
					 * which contains at least another item */
                    o_ptr = get_object(item)
                }
            }
            if !(repeat as libc::c_int == 1 as libc::c_int) { break ; }
        }
        if carry_s_ptr != 0 { inven_carry(s_ptr, 1 as libc::c_int as bool_); }
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long |
                      0x8 as libc::c_long)) as u32b
    } else if ext == 3 as libc::c_int {
        let mut item_0: libc::c_int = 0;
        let mut q_0: cptr = 0 as *const libc::c_char;
        let mut s_0: cptr = 0 as *const libc::c_char;
        item_tester_hook =
            Some(item_tester_hook_recharge as
                     unsafe extern "C" fn(_: *mut object_type) -> bool_);
        /* If we carry empty items, add them to the p_ptr->inventory */
        /* Combine / Reorder the pack (later) */
        /* Window stuff */
        /* Get an item */
        q_0 =
            b"Recharge which item? \x00" as *const u8 as *const libc::c_char;
        s_0 =
            b"You have no rechargable items.\x00" as *const u8 as
                *const libc::c_char;
        if get_item(&mut item_0, q_0, s_0,
                    0x2 as libc::c_int | 0x4 as libc::c_int) == 0 {
            return
        }
        /* Get the item */
        o_ptr = get_object(item_0);
        /* Make sure we have enough essences to recharge this */
        if alchemist_items_check((*o_ptr).tval as libc::c_int,
                                 (*o_ptr).sval as libc::c_int,
                                 0 as libc::c_int, 0 as libc::c_int,
                                 1 as libc::c_int as bool_) == 0 {
            msg_print(b"You don\'t have the essences to recharge this item.\x00"
                          as *const u8 as *const libc::c_char);
            return
        }
        /* Take a turn */
        energy_use = 100 as libc::c_int;
        /* Destroy the essences */
        alchemist_items_check((*o_ptr).tval as libc::c_int,
                              (*o_ptr).sval as libc::c_int, 0 as libc::c_int,
                              -(1 as libc::c_int), 1 as libc::c_int as bool_);
        if (*o_ptr).tval as libc::c_int == 55 as libc::c_int &&
               (*o_ptr).number as libc::c_int > 1 as libc::c_int {
            /* Unstack staves */
			/* Get local object */
            q_ptr = &mut forge2;
            /* Obtain a local object */
            object_copy(q_ptr, o_ptr);
            /* Modify quantity */
            (*q_ptr).number = 1 as libc::c_int as byte_hack;
            /* Unstack the used item */
            (*o_ptr).number = (*o_ptr).number.wrapping_sub(1);
            o_ptr = q_ptr;
            carry_o_ptr = 1 as libc::c_int as byte_hack
        }
        (*o_ptr).pval += 1
    } else if ext == 4 as libc::c_int { alchemist_recipe_book(); }
    /* Just in case - */
    if carry_o_ptr != 0 {
        /* the o_ptr item was probably an unstacked staff
		 * Anyway, we need to add it to the p_ptr->inventory */
        if inven_carry_okay(o_ptr) != 0 {
            inven_carry(o_ptr, 1 as libc::c_int as bool_);
        } else {
            drop_near(o_ptr, 0 as libc::c_int, (*p_ptr).py as libc::c_int,
                      (*p_ptr).px as libc::c_int);
        }
    };
}
/*
 * Command to ask favors from your god.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_pray() {
    if (*p_ptr).pgod as libc::c_int == 0 as libc::c_int {
        msg_print(b"Pray hard enough and your prayers might be answered.\x00"
                      as *const u8 as *const libc::c_char);
        return
    } else {
        if (*p_ptr).praying == 0 {
            msg_format(b"You start praying to %s.\x00" as *const u8 as
                           *const libc::c_char,
                       (*deity_info.offset((*p_ptr).pgod as isize)).name);
        } else {
            msg_format(b"You stop praying to %s.\x00" as *const u8 as
                           *const libc::c_char,
                       (*deity_info.offset((*p_ptr).pgod as isize)).name);
        }
        (*p_ptr).praying = ((*p_ptr).praying == 0) as libc::c_int as bool_;
        /* Update stuffs */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x1 as libc::c_long | 0x10 as libc::c_long |
                      0x20 as libc::c_long | 0x40 as libc::c_long |
                      0x80 as libc::c_long | 0x8 as libc::c_long |
                      0x4 as libc::c_long)) as u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long |
                 (0x8000 as libc::c_long | 0x8000000 as libc::c_long |
                      0x2000000 as libc::c_long | 0x1000000 as libc::c_long |
                      0x4000000 as libc::c_long)) as u32b;
        energy_use = 100 as libc::c_int
    };
}
/*
 * Return percentage chance of spell failure.
 */
#[no_mangle]
pub unsafe extern "C" fn spell_chance_random(mut rspell: *mut random_spell)
 -> libc::c_int {
    let mut chance: libc::c_int = 0;
    let mut minfail: libc::c_int = 0;
    /* Extract the base spell failure rate */
    chance = (*rspell).level as libc::c_int + 10 as libc::c_int;
    /* Reduce failure rate by "effective" level adjustment */
    chance -=
        3 as libc::c_int *
            (get_skill(43 as libc::c_int) as libc::c_int -
                 (*rspell).level as libc::c_int);
    /* Reduce failure rate by INT/WIS adjustment */
    chance -=
        3 as libc::c_int *
            (*adj_mag_stat.as_mut_ptr().offset((*p_ptr).stat_ind[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                   as isize) as libc::c_int -
                 1 as libc::c_int);
    /* Not enough mana to cast */
    if (*rspell).mana as libc::c_int > (*p_ptr).csp as libc::c_int {
        chance +=
            5 as libc::c_int *
                ((*rspell).mana as libc::c_int - (*p_ptr).csp as libc::c_int)
    }
    /* Extract the minimum failure rate */
    minfail =
        *adj_mag_fail.as_mut_ptr().offset((*p_ptr).stat_ind[1 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int;
    /* Failure rate */
    return clamp_failure_chance(chance, minfail);
}
/*
 * Print a batch of spells.
 */
unsafe extern "C" fn print_spell_batch(mut batch: libc::c_int,
                                       mut max: libc::c_int) {
    let mut buff: [libc::c_char; 80] = [0; 80];
    let mut rspell: *mut random_spell = 0 as *mut random_spell;
    let mut i: libc::c_int = 0;
    prt(format(b"      %-30s Lev Fail Mana Damage \x00" as *const u8 as
                   *const libc::c_char,
               b"Name\x00" as *const u8 as *const libc::c_char) as cptr,
        1 as libc::c_int, 20 as libc::c_int);
    i = 0 as libc::c_int;
    while i < max {
        rspell =
            &mut *random_spells.as_mut_ptr().offset((batch * 10 as libc::c_int
                                                         + i) as isize) as
                *mut random_spell;
        if (*rspell).untried != 0 {
            strnfmt(buff.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"  %c) %-30s  (Spell untried)  \x00" as *const u8 as
                        *const libc::c_char, i + 'a' as i32,
                    (*rspell).name.as_mut_ptr());
        } else {
            strnfmt(buff.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"  %c) %-30s %3d %4d%% %3d %3dd%d \x00" as *const u8 as
                        *const libc::c_char, i + 'a' as i32,
                    (*rspell).name.as_mut_ptr(),
                    (*rspell).level as libc::c_int,
                    spell_chance_random(rspell),
                    (*rspell).mana as libc::c_int,
                    (*rspell).dam_dice as libc::c_int,
                    (*rspell).dam_sides as libc::c_int);
        }
        prt(buff.as_mut_ptr() as cptr, 2 as libc::c_int + i,
            20 as libc::c_int);
        i += 1
    }
    prt(b"\x00" as *const u8 as *const libc::c_char, 2 as libc::c_int + i,
        20 as libc::c_int);
}
/*
 * List ten random spells and ask to pick one.
 */
unsafe extern "C" fn select_spell_from_batch(mut batch: libc::c_int,
                                             mut quick: bool_)
 -> *mut random_spell {
    let mut tmp: [libc::c_char; 160] = [0; 160];
    let mut out_val: [libc::c_char; 30] = [0; 30];
    let mut which: libc::c_char = 0;
    let mut mut_max: libc::c_int = 10 as libc::c_int;
    let mut ret: *mut random_spell = 0 as *mut random_spell;
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    if (spell_num as libc::c_int) <
           (batch + 1 as libc::c_int) * 10 as libc::c_int {
        mut_max = spell_num as libc::c_int - batch * 10 as libc::c_int
    }
    strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
            b"(a-%c, * to list, A-%cto browse, / to rename, - to comment) Select a power: \x00"
                as *const u8 as *const libc::c_char,
            mut_max - 1 as libc::c_int + 'a' as i32,
            mut_max - 1 as libc::c_int + 'a' as i32 - 'a' as i32 +
                'A' as i32);
    prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
    if quick != 0 { print_spell_batch(batch, mut_max); }
    loop 
         /* Wait for next command */
         /* Get a command */
         {
        which = inkey();
        /* Abort */
        if which as libc::c_int == '\u{1b}' as i32 {
            /* No selection */
            ret = 0 as *mut random_spell;
            /* Leave the command loop */
            break ;
        } else if which as libc::c_int == '*' as i32 ||
                      which as libc::c_int == '?' as i32 ||
                      which as libc::c_int == ' ' as i32 {
            /* List */
            /* Print power list */
            print_spell_batch(batch, mut_max);
        } else if which as libc::c_int == '\r' as i32 {
            /* Accept default */
            /* There are no other choices */
            if mut_max == 1 as libc::c_int {
                ret =
                    &mut *random_spells.as_mut_ptr().offset((batch *
                                                                 10 as
                                                                     libc::c_int)
                                                                as isize) as
                        *mut random_spell;
                break ;
            }
        } else if which as libc::c_int == '/' as i32 {
            prt(b"Rename which power: \x00" as *const u8 as
                    *const libc::c_char, 0 as libc::c_int, 0 as libc::c_int);
            which = tolower(inkey() as libc::c_int) as libc::c_char;
            if *(*__ctype_b_loc()).offset(which as libc::c_int as isize) as
                   libc::c_int &
                   _ISalpha as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 && which as libc::c_int - 'a' as i32 <= mut_max {
                strcpy(out_val.as_mut_ptr(),
                       random_spells[(batch * 10 as libc::c_int +
                                          (which as libc::c_int - 'a' as i32))
                                         as usize].name.as_mut_ptr());
                if get_string(b"Name this power: \x00" as *const u8 as
                                  *const libc::c_char, out_val.as_mut_ptr(),
                              29 as libc::c_int) != 0 {
                    strcpy(random_spells[(batch * 10 as libc::c_int +
                                              (which as libc::c_int -
                                                   'a' as i32)) as
                                             usize].name.as_mut_ptr(),
                           out_val.as_mut_ptr());
                }
                prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int,
                    0 as libc::c_int);
            } else {
                bell();
                prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int,
                    0 as libc::c_int);
            }
        } else if which as libc::c_int == '-' as i32 {
            prt(b"Comment which power: \x00" as *const u8 as
                    *const libc::c_char, 0 as libc::c_int, 0 as libc::c_int);
            which = tolower(inkey() as libc::c_int) as libc::c_char;
            if *(*__ctype_b_loc()).offset(which as libc::c_int as isize) as
                   libc::c_int &
                   _ISalpha as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 && which as libc::c_int - 'a' as i32 <= mut_max {
                strcpy(out_val.as_mut_ptr(),
                       random_spells[(batch * 10 as libc::c_int +
                                          (which as libc::c_int - 'a' as i32))
                                         as usize].desc.as_mut_ptr());
                if get_string(b"Comment this power: \x00" as *const u8 as
                                  *const libc::c_char, out_val.as_mut_ptr(),
                              29 as libc::c_int) != 0 {
                    strcpy(random_spells[(batch * 10 as libc::c_int +
                                              (which as libc::c_int -
                                                   'a' as i32)) as
                                             usize].desc.as_mut_ptr(),
                           out_val.as_mut_ptr());
                }
                prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int,
                    0 as libc::c_int);
            } else {
                bell();
                prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int,
                    0 as libc::c_int);
            }
        } else if *(*__ctype_b_loc()).offset(which as libc::c_int as isize) as
                      libc::c_int &
                      _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 &&
                      *(*__ctype_b_loc()).offset(which as libc::c_int as
                                                     isize) as libc::c_int &
                          _ISupper as libc::c_int as libc::c_ushort as
                              libc::c_int != 0 {
            which = tolower(which as libc::c_int) as libc::c_char;
            c_prt(14 as libc::c_int as byte_hack,
                  format(b"%s : %s\x00" as *const u8 as *const libc::c_char,
                         random_spells[(batch * 10 as libc::c_int +
                                            (which as libc::c_int -
                                                 'a' as i32)) as
                                           usize].name.as_mut_ptr(),
                         random_spells[(batch * 10 as libc::c_int +
                                            (which as libc::c_int -
                                                 'a' as i32)) as
                                           usize].desc.as_mut_ptr()) as cptr,
                  0 as libc::c_int, 0 as libc::c_int);
            inkey();
            prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
        } else if *(*__ctype_b_loc()).offset(which as libc::c_int as isize) as
                      libc::c_int &
                      _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 && (which as libc::c_int - 'a' as i32) < mut_max {
            /* Rename */
            /* Comment */
            /* Pick the power */
            ret =
                &mut *random_spells.as_mut_ptr().offset((batch *
                                                             10 as libc::c_int
                                                             +
                                                             (which as
                                                                  libc::c_int
                                                                  -
                                                                  'a' as i32))
                                                            as isize) as
                    *mut random_spell;
            /* Leave the command loop */
            break ;
        } else { bell(); }
    }
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
    /* Return selection */
    return ret;
}
/*
 * Pick a random spell from a menu
 */
#[no_mangle]
pub unsafe extern "C" fn select_spell(mut quick: bool_) -> *mut random_spell {
    let mut tmp: [libc::c_char; 160] = [0; 160];
    let mut which: libc::c_char = 0;
    let mut batch_max: libc::c_int =
        (spell_num as libc::c_int - 1 as libc::c_int) / 10 as libc::c_int;
    let mut ret: *mut random_spell = 0 as *mut random_spell;
    /* Too confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You can\'t use your powers while confused!\x00" as
                      *const u8 as *const libc::c_char);
        return 0 as *mut random_spell
    }
    /* No spells available */
    if spell_num as libc::c_int == 0 as libc::c_int {
        msg_print(b"There are no spells you can cast.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as *mut random_spell
    }
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
            b"(a-%c) Select batch of powers: \x00" as *const u8 as
                *const libc::c_char, batch_max + 'a' as i32);
    prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
    loop  {
        which = inkey();
        if which as libc::c_int == '\u{1b}' as i32 {
            ret = 0 as *mut random_spell;
            break ;
        } else if which as libc::c_int == '\r' as i32 {
            if !(batch_max == 0 as libc::c_int) { continue ; }
            ret = select_spell_from_batch(0 as libc::c_int, quick);
            break ;
        } else {
            which = tolower(which as libc::c_int) as libc::c_char;
            if *(*__ctype_b_loc()).offset(which as libc::c_int as isize) as
                   libc::c_int &
                   _ISalpha as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 && which as libc::c_int - 'a' as i32 <= batch_max {
                ret =
                    select_spell_from_batch(which as libc::c_int - 'a' as i32,
                                            quick);
                break ;
            } else { bell(); }
        }
    }
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_powermage() {
    let mut s_ptr: *mut random_spell = 0 as *mut random_spell;
    let mut proj_flags: u32b = 0;
    let mut dir: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut ty: libc::c_int = 0 as libc::c_int;
    let mut tx: libc::c_int = 0 as libc::c_int;
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).anti_magic != 0 {
        msg_print(b"Your anti-magic shell disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    s_ptr = select_spell(0 as libc::c_int as bool_);
    if s_ptr.is_null() { return }
    if ((*p_ptr).csp as libc::c_int) < (*s_ptr).mana as libc::c_int {
        msg_print(b"You do not have enough mana.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Spell failure chance */
    chance = spell_chance_random(s_ptr);
    /* Failed spell */
    if Rand_div(100 as libc::c_int) < chance {
        let mut insanity: libc::c_int =
            ((*p_ptr).msane as libc::c_int - (*p_ptr).csane as libc::c_int) *
                100 as libc::c_int / (*p_ptr).msane as libc::c_int;
        let mut sfail: [libc::c_char; 80] = [0; 80];
        /* Flush input if told so */
        if flush_failure != 0 { flush(); }
        /* Insane players can see something strange */
        if Rand_div(100 as libc::c_int) < insanity {
            get_rnd_line(b"sfail.txt\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char, sfail.as_mut_ptr());
            msg_format(b"A cloud of %s appears above you.\x00" as *const u8 as
                           *const libc::c_char, sfail.as_mut_ptr());
        } else {
            /* Normal failure messages */
            msg_print(b"You failed to get the spell off!\x00" as *const u8 as
                          *const libc::c_char);
        }
        sound(55 as libc::c_int);
        /* Let time pass */
        if is_magestaff() != 0 {
            energy_use = 80 as libc::c_int
        } else { energy_use = 100 as libc::c_int }
        /* Mana is spent anyway */
        (*p_ptr).csp =
            ((*p_ptr).csp as libc::c_int - (*s_ptr).mana as libc::c_int) as
                s16b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
        return
    }
    (*p_ptr).csp =
        ((*p_ptr).csp as libc::c_int - (*s_ptr).mana as libc::c_int) as s16b;
    (*s_ptr).untried = 0 as libc::c_int as bool_;
    proj_flags = (*s_ptr).proj_flags;
    /* Hack -- Spell needs a target */
    if (*s_ptr).proj_flags & 0x2 as libc::c_int as libc::c_uint != 0 ||
           (*s_ptr).proj_flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        if get_aim_dir(&mut dir) == 0 { return }
        /* Hack -- Use an actual "target" */
        if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
            tx = target_col as libc::c_int;
            ty = target_row as libc::c_int;
            /* Mega-Hack -- Beam spells should continue through
				 * the target; bolt spells should stop at the
				 * target. --dsb */
            if (*s_ptr).proj_flags & 0x2 as libc::c_int as libc::c_uint != 0 {
                proj_flags |= 0x4 as libc::c_int as libc::c_uint
            }
        } else {
            /* Use the given direction */
            ty =
                (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
            tx =
                (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
            /* Mega-Hack -- Both beam and bolt spells should
				 * continue through this fake target. --dsb */
            proj_flags |= 0x4 as libc::c_int as libc::c_uint
        }
    }
    if (*s_ptr).proj_flags & 0x400 as libc::c_int as libc::c_uint != 0 {
        ty = (*p_ptr).py as libc::c_int;
        tx = (*p_ptr).px as libc::c_int
    }
    if (*s_ptr).proj_flags & 0x100 as libc::c_int as libc::c_uint != 0 {
        project_hack((*s_ptr).GF as libc::c_int,
                     damroll((*s_ptr).dam_dice as s16b,
                             (*s_ptr).dam_sides as s16b));
    } else if (*s_ptr).proj_flags & 0x200 as libc::c_int as libc::c_uint != 0
     {
        project_meteor((*s_ptr).radius as libc::c_int,
                       (*s_ptr).GF as libc::c_int,
                       damroll((*s_ptr).dam_dice as s16b,
                               (*s_ptr).dam_sides as s16b),
                       (*s_ptr).proj_flags);
    } else {
        project(0 as libc::c_int, (*s_ptr).radius as libc::c_int, ty, tx,
                damroll((*s_ptr).dam_dice as s16b,
                        (*s_ptr).dam_sides as s16b),
                (*s_ptr).GF as libc::c_int, proj_flags as libc::c_int);
    }
    /* Take a turn */
    if is_magestaff() != 0 {
        energy_use = 80 as libc::c_int
    } else { energy_use = 100 as libc::c_int }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Brand some ammunition.  Used by Cubragol and a mage spell.  The spell was
 * moved here from cmd6.c where it used to be for Cubragol only.  I've also
 * expanded it to do either frost, fire or venom, at random. -GJW	-KMW-
 */
#[no_mangle]
pub unsafe extern "C" fn brand_ammo(mut brand_type: libc::c_int,
                                    mut bolts_only: libc::c_int) {
    let mut a: libc::c_int = 0;
    a = 0 as libc::c_int;
    while a < 23 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(a as isize) as
                *mut object_type;
        if !(bolts_only != 0 &&
                 (*o_ptr).tval as libc::c_int != 18 as libc::c_int) {
            if !(bolts_only == 0 &&
                     (*o_ptr).tval as libc::c_int != 18 as libc::c_int &&
                     (*o_ptr).tval as libc::c_int != 17 as libc::c_int &&
                     (*o_ptr).tval as libc::c_int != 16 as libc::c_int) {
                if !((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                         (if (*o_ptr).name1 as libc::c_int != 0 {
                              1 as libc::c_int
                          } else { 0 as libc::c_int }) != 0 ||
                         (if (*o_ptr).art_name as libc::c_int != 0 {
                              1 as libc::c_int
                          } else { 0 as libc::c_int }) != 0 ||
                         (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3
                                 as libc::c_long & 0x8000 as libc::c_long != 0
                             {
                              1 as libc::c_int
                          } else { 0 as libc::c_int }) != 0) &&
                       (if (*o_ptr).name2 as libc::c_int != 0 ||
                               (*o_ptr).name2b as libc::c_int != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) == 0 &&
                       (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int ==
                           0 {
                    break ;
                }
            }
        }
        a += 1
    }
    /* Enchant the ammo (or fail) */
    if a < 23 as libc::c_int &&
           Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        let mut o_ptr_0: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(a as isize) as
                *mut object_type;
        let mut ammo_name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut aura_name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut msg: [libc::c_char; 48] = [0; 48];
        let mut aura_type: libc::c_int = 0;
        let mut r: libc::c_int = 0;
        /* fire only */
        if brand_type == 1 as libc::c_int {
            r = 0 as libc::c_int
        } else if brand_type == 2 as libc::c_int {
            r = 99 as libc::c_int
        } else {
            /* cold only */
            /* No bias */
            r = Rand_div(100 as libc::c_int)
        }
        if r < 50 as libc::c_int {
            aura_name =
                b"fiery\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            aura_type = 122 as libc::c_int
        } else {
            aura_name =
                b"frosty\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            aura_type = 123 as libc::c_int
        }
        if (*o_ptr_0).tval as libc::c_int == 18 as libc::c_int {
            ammo_name =
                b"bolts\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if (*o_ptr_0).tval as libc::c_int == 17 as libc::c_int {
            ammo_name =
                b"arrows\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else {
            ammo_name =
                b"shots\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        strnfmt(msg.as_mut_ptr(), 48 as libc::c_int as uint_hack,
                b"Your %s are covered in a %s aura!\x00" as *const u8 as
                    *const libc::c_char, ammo_name, aura_name);
        msg_print(msg.as_mut_ptr() as cptr);
        (*o_ptr_0).name2 = aura_type as s16b;
        /* Apply the ego */
        apply_magic(o_ptr_0, dun_level as libc::c_int,
                    0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                    0 as libc::c_int as bool_);
        (*o_ptr_0).discount = 100 as libc::c_int as byte_hack;
        enchant(o_ptr_0, Rand_div(3 as libc::c_int) + 4 as libc::c_int,
                0x1 as libc::c_int | 0x2 as libc::c_int);
    } else {
        if flush_failure != 0 { flush(); }
        msg_print(b"The enchantment failed.\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/*
 * From Kamband by Ivan Tkatchev
 */
#[no_mangle]
pub unsafe extern "C" fn summon_monster(mut sumtype: libc::c_int) {
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    if (*p_ptr).inside_arena != 0 {
        msg_print(b"This place seems devoid of life.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        return
    }
    if summon_specific_friendly((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int +
                                    (Rand_div(5 as libc::c_int) +
                                         1 as libc::c_int), sumtype,
                                1 as libc::c_int as bool_) != 0 {
        msg_print(b"You summon some help.\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        msg_print(b"You called, but no help came.\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/*
 * Use a class power of Possessor
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_possessor() {
    let mut ch: libc::c_char = 0;
    let mut ext: libc::c_char = 0;
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).anti_magic != 0 {
        msg_print(b"Your anti-magic shell disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    loop  {
        if get_com(b"Use your [R]ace powers or your [I]ncarnating powers?\x00"
                       as *const u8 as *const libc::c_char, &mut ch) == 0 {
            ext = 0 as libc::c_int as libc::c_char;
            break ;
        } else if ch as libc::c_int == 'R' as i32 ||
                      ch as libc::c_int == 'r' as i32 {
            ext = 1 as libc::c_int as libc::c_char;
            break ;
        } else {
            if !(ch as libc::c_int == 'I' as i32 ||
                     ch as libc::c_int == 'i' as i32) {
                continue ;
            }
            ext = 2 as libc::c_int as libc::c_char;
            break ;
        }
    }
    if ext as libc::c_int == 1 as libc::c_int {
        let mut use_great: bool_ = 0 as libc::c_int as bool_;
        if (*p_ptr).disembodied != 0 {
            msg_print(b"You don\'t currently own a body to use.\x00" as
                          *const u8 as *const libc::c_char);
            return
        }
        /* Do we have access to all the powers ? */
        if get_skill_scale(50 as libc::c_int, 100 as libc::c_int as u32b) as
               libc::c_int >=
               (*r_info.offset((*p_ptr).body_monster as isize)).level as
                   libc::c_int {
            use_great = 1 as libc::c_int as bool_
        }
        use_symbiotic_power((*p_ptr).body_monster as libc::c_int, use_great,
                            0 as libc::c_int as bool_,
                            0 as libc::c_int as bool_);
        if ((*p_ptr).csp as libc::c_int) < 0 as libc::c_int {
            msg_print(b"You lose control of your body!\x00" as *const u8 as
                          *const libc::c_char);
            if do_cmd_leave_body(0 as libc::c_int as bool_) == 0 {
                cmsg_print(10 as libc::c_int as byte_hack,
                           b"You are forced back into your body by your cursed items, you suffer a system shock!\x00"
                               as *const u8 as *const libc::c_char);
                (*p_ptr).chp = 1 as libc::c_int as s16b;
                /* Display the hitpoints */
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long)
                        as u32b
            }
        }
    } else if ext as libc::c_int == 2 as libc::c_int {
        if (*p_ptr).disembodied != 0 {
            do_cmd_integrate_body();
        } else { do_cmd_leave_body(1 as libc::c_int as bool_); }
    } else { return }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
}
/*
 * Hook to determine if an object is contertible in an arrow/bolt
 */
unsafe extern "C" fn item_tester_hook_convertible(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int == 11 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 1 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
/*
 * do_cmd_cast calls this function if the player's class
 * is 'archer'.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_archer() {
    let mut ext: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_char = 0;
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
    let mut com: [libc::c_char; 80] = [0; 80];
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if (*p_ptr).blind != 0 {
        msg_print(b"You are blind!\x00" as *const u8 as *const libc::c_char);
        return
    }
    if get_skill(23 as libc::c_int) as libc::c_int >= 20 as libc::c_int {
        strnfmt(com.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"Create [S]hots, [A]rrows or [B]olts? \x00" as *const u8 as
                    *const libc::c_char);
    } else if get_skill(23 as libc::c_int) as libc::c_int >= 10 as libc::c_int
     {
        strnfmt(com.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"Create [S]hots or [A]rrows? \x00" as *const u8 as
                    *const libc::c_char);
    } else {
        strnfmt(com.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"Create [S]hots? \x00" as *const u8 as *const libc::c_char);
    }
    loop  {
        if get_com(com.as_mut_ptr() as cptr, &mut ch) == 0 {
            ext = 0 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'S' as i32 ||
                      ch as libc::c_int == 's' as i32 {
            ext = 1 as libc::c_int;
            break ;
        } else if (ch as libc::c_int == 'A' as i32 ||
                       ch as libc::c_int == 'a' as i32) &&
                      get_skill(23 as libc::c_int) as libc::c_int >=
                          10 as libc::c_int {
            ext = 2 as libc::c_int;
            break ;
        } else {
            if !((ch as libc::c_int == 'B' as i32 ||
                      ch as libc::c_int == 'b' as i32) &&
                     get_skill(23 as libc::c_int) as libc::c_int >=
                         20 as libc::c_int) {
                continue ;
            }
            ext = 3 as libc::c_int;
            break ;
        }
    }
    /* Prepare for object creation */
    q_ptr = &mut forge;
    /* *********Create shots*********/
    if ext == 1 as libc::c_int {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut dir: libc::c_int = 0;
        let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
        if get_rep_dir(&mut dir) == 0 { return }
        y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
        x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        if (*c_ptr).feat as libc::c_int == 0x31 as libc::c_int {
            /* Get local object */
            q_ptr = &mut forge;
            /* Hack -- Give the player some shots */
            object_prep(q_ptr,
                        lookup_kind(16 as libc::c_int,
                                    m_bonus(2 as libc::c_int,
                                            dun_level as libc::c_int) as
                                        libc::c_int) as libc::c_int);
            if !((*q_ptr).tval as libc::c_int == 102 as libc::c_int ||
                     (if (*q_ptr).name1 as libc::c_int != 0 {
                          1 as libc::c_int
                      } else { 0 as libc::c_int }) != 0 ||
                     (if (*q_ptr).art_name as libc::c_int != 0 {
                          1 as libc::c_int
                      } else { 0 as libc::c_int }) != 0 ||
                     (if (*k_info.offset((*q_ptr).k_idx as isize)).flags3 as
                             libc::c_long & 0x8000 as libc::c_long != 0 {
                          1 as libc::c_int
                      } else { 0 as libc::c_int }) != 0) {
                (*q_ptr).number =
                    (15 as libc::c_int +
                         Rand_div(1 as libc::c_int + 30 as libc::c_int -
                                      15 as libc::c_int)) as byte_hack
            } else { (*q_ptr).number = 1 as libc::c_int as byte_hack }
            object_aware(q_ptr);
            object_known(q_ptr);
            (*q_ptr).ident =
                ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int) as
                    byte_hack;
            apply_magic(q_ptr, dun_level as libc::c_int,
                        1 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                        if Rand_div(100 as libc::c_int) < 20 as libc::c_int {
                            1 as libc::c_int
                        } else { 0 as libc::c_int } as bool_);
            (*q_ptr).discount = 90 as libc::c_int as byte_hack;
            (*q_ptr).found = 9 as libc::c_int as byte_hack;
            inven_carry(q_ptr, 0 as libc::c_int as bool_);
            msg_print(b"You make some ammo.\x00" as *const u8 as
                          *const libc::c_char);
            wall_to_mud(dir);
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long |
                     (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                          0x200000 as libc::c_long)) as u32b;
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as
                    u32b
        }
    } else if ext == 2 as libc::c_int {
        let mut item: libc::c_int = 0;
        let mut q: cptr = 0 as *const libc::c_char;
        let mut s: cptr = 0 as *const libc::c_char;
        item_tester_hook =
            Some(item_tester_hook_convertible as
                     unsafe extern "C" fn(_: *mut object_type) -> bool_);
        /* *********Create arrows*********/
        /* Get an item */
        q = b"Convert which item? \x00" as *const u8 as *const libc::c_char;
        s =
            b"You have no item to convert.\x00" as *const u8 as
                *const libc::c_char;
        if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int)
               == 0 {
            return
        }
        /* Get local object */
        q_ptr = &mut forge;
        /* Hack -- Give the player some arrows */
        object_prep(q_ptr,
                    lookup_kind(17 as libc::c_int,
                                m_bonus(1 as libc::c_int,
                                        dun_level as libc::c_int) as
                                    libc::c_int + 1 as libc::c_int) as
                        libc::c_int);
        (*q_ptr).number =
            (15 as libc::c_int +
                 Rand_div(1 as libc::c_int + 25 as libc::c_int -
                              15 as libc::c_int)) as byte_hack;
        if !((*q_ptr).tval as libc::c_int == 102 as libc::c_int ||
                 (if (*q_ptr).name1 as libc::c_int != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0 ||
                 (if (*q_ptr).art_name as libc::c_int != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0 ||
                 (if (*k_info.offset((*q_ptr).k_idx as isize)).flags3 as
                         libc::c_long & 0x8000 as libc::c_long != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0) {
            (*q_ptr).number =
                (15 as libc::c_int +
                     Rand_div(1 as libc::c_int + 30 as libc::c_int -
                                  15 as libc::c_int)) as byte_hack
        } else { (*q_ptr).number = 1 as libc::c_int as byte_hack }
        object_aware(q_ptr);
        object_known(q_ptr);
        (*q_ptr).ident =
            ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int) as
                byte_hack;
        apply_magic(q_ptr, dun_level as libc::c_int,
                    1 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                    if Rand_div(100 as libc::c_int) < 20 as libc::c_int {
                        1 as libc::c_int
                    } else { 0 as libc::c_int } as bool_);
        (*q_ptr).discount = 90 as libc::c_int as byte_hack;
        (*q_ptr).found = 9 as libc::c_int as byte_hack;
        msg_print(b"You make some ammo.\x00" as *const u8 as
                      *const libc::c_char);
        inc_stack_size(item, -(1 as libc::c_int));
        inven_carry(q_ptr, 0 as libc::c_int as bool_);
    } else if ext == 3 as libc::c_int {
        let mut item_0: libc::c_int = 0;
        let mut q_0: cptr = 0 as *const libc::c_char;
        let mut s_0: cptr = 0 as *const libc::c_char;
        item_tester_hook =
            Some(item_tester_hook_convertible as
                     unsafe extern "C" fn(_: *mut object_type) -> bool_);
        /* *********Create bolts*********/
        /* Get an item */
        q_0 = b"Convert which item? \x00" as *const u8 as *const libc::c_char;
        s_0 =
            b"You have no item to convert.\x00" as *const u8 as
                *const libc::c_char;
        if get_item(&mut item_0, q_0, s_0,
                    0x2 as libc::c_int | 0x4 as libc::c_int) == 0 {
            return
        }
        /* Get local object */
        q_ptr = &mut forge;
        /* Hack -- Give the player some bolts */
        object_prep(q_ptr,
                    lookup_kind(18 as libc::c_int,
                                m_bonus(1 as libc::c_int,
                                        dun_level as libc::c_int) as
                                    libc::c_int + 1 as libc::c_int) as
                        libc::c_int);
        (*q_ptr).number =
            (15 as libc::c_int +
                 Rand_div(1 as libc::c_int + 25 as libc::c_int -
                              15 as libc::c_int)) as byte_hack;
        if !((*q_ptr).tval as libc::c_int == 102 as libc::c_int ||
                 (if (*q_ptr).name1 as libc::c_int != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0 ||
                 (if (*q_ptr).art_name as libc::c_int != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0 ||
                 (if (*k_info.offset((*q_ptr).k_idx as isize)).flags3 as
                         libc::c_long & 0x8000 as libc::c_long != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0) {
            (*q_ptr).number =
                (15 as libc::c_int +
                     Rand_div(1 as libc::c_int + 30 as libc::c_int -
                                  15 as libc::c_int)) as byte_hack
        } else { (*q_ptr).number = 1 as libc::c_int as byte_hack }
        object_aware(q_ptr);
        object_known(q_ptr);
        (*q_ptr).ident =
            ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int) as
                byte_hack;
        apply_magic(q_ptr, dun_level as libc::c_int,
                    1 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                    if Rand_div(100 as libc::c_int) < 20 as libc::c_int {
                        1 as libc::c_int
                    } else { 0 as libc::c_int } as bool_);
        (*q_ptr).discount = 90 as libc::c_int as byte_hack;
        (*q_ptr).found = 9 as libc::c_int as byte_hack;
        msg_print(b"You make some ammo.\x00" as *const u8 as
                      *const libc::c_char);
        inc_stack_size(item_0, -(1 as libc::c_int));
        inven_carry(q_ptr, 0 as libc::c_int as bool_);
    };
}
/*
 * Control whether shots are allowed to pierce
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_set_piercing() {
    let mut ch: libc::c_char = 0;
    let mut com: [libc::c_char; 80] = [0; 80];
    if get_skill(25 as libc::c_int) as libc::c_int <= 25 as libc::c_int &&
           get_skill(26 as libc::c_int) as libc::c_int <= 25 as libc::c_int &&
           get_skill(24 as libc::c_int) as libc::c_int <= 25 as libc::c_int {
        msg_print(b"You can\'t fire piercing shots yet.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    strnfmt(com.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"Allow shots to pierce? \x00" as *const u8 as
                *const libc::c_char);
    while !(get_com(com.as_mut_ptr() as cptr, &mut ch) == 0) {
        if ch as libc::c_int == 'Y' as i32 || ch as libc::c_int == 'y' as i32
           {
            (*p_ptr).use_piercing_shots = 1 as libc::c_int as s16b;
            msg_print(b"Piercing shots activated.\x00" as *const u8 as
                          *const libc::c_char);
            break ;
        } else {
            if !(ch as libc::c_int == 'N' as i32 ||
                     ch as libc::c_int == 'n' as i32) {
                continue ;
            }
            (*p_ptr).use_piercing_shots = 0 as libc::c_int as s16b;
            msg_print(b"Piercing shots deactivated.\x00" as *const u8 as
                          *const libc::c_char);
            break ;
        }
    };
}
/*
 * Helper function to describe necro powers
 */
#[no_mangle]
pub unsafe extern "C" fn necro_info(mut p: *mut libc::c_char,
                                    mut power: libc::c_int) {
    let mut plev: libc::c_int = get_skill(31 as libc::c_int) as libc::c_int;
    strcpy(p, b"\x00" as *const u8 as *const libc::c_char);
    match power {
        0 => {
            if (*p_ptr).to_s != 0 {
                strnfmt(p, 80 as libc::c_int as uint_hack,
                        b" power %dd%d+%d\x00" as *const u8 as
                            *const libc::c_char,
                        2 as libc::c_int +
                            plev * 2 as libc::c_int / 3 as libc::c_int,
                        4 as libc::c_int,
                        (*p_ptr).to_s as libc::c_int * 2 as libc::c_int);
            } else {
                strnfmt(p, 80 as libc::c_int as uint_hack,
                        b" power %dd%d\x00" as *const u8 as
                            *const libc::c_char,
                        2 as libc::c_int +
                            plev * 2 as libc::c_int / 3 as libc::c_int,
                        4 as libc::c_int);
            }
        }
        2 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dur d%d+%d\x00" as *const u8 as *const libc::c_char,
                    100 as libc::c_int + plev * 4 as libc::c_int,
                    200 as libc::c_int + plev * 3 as libc::c_int);
        }
        3 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" dur d%d+%d\x00" as *const u8 as *const libc::c_char,
                    30 as libc::c_int + plev * 2 as libc::c_int,
                    50 as libc::c_int + plev);
        }
        _ => { }
    };
}
/*
 * Cast a Necromancy spell
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_necromancer() {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 0 as libc::c_int;
    let mut chance: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut minfail: libc::c_int = 0 as libc::c_int;
    let mut plev: libc::c_int = get_skill(31 as libc::c_int) as libc::c_int;
    let mut spell: magic_power =
        magic_power{min_lev: 0,
                    mana_cost: 0,
                    fail: 0,
                    name: 0 as *const libc::c_char,
                    desc: 0 as *const libc::c_char,};
    let mut to_s2: libc::c_int =
        (*p_ptr).to_s as libc::c_int / 2 as libc::c_int;
    let mut mto_s2: libc::c_int =
        (*p_ptr).to_s as libc::c_int / 2 as libc::c_int;
    if mto_s2 == 0 as libc::c_int { mto_s2 = 1 as libc::c_int }
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).anti_magic != 0 {
        msg_print(b"Your anti-magic shell disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* not if confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* get power */
    if get_magic_power(&mut n, necro_powers.as_mut_ptr(), 6 as libc::c_int,
                       Some(necro_info as
                                unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: libc::c_int) -> ()),
                       get_skill(31 as libc::c_int) as libc::c_int,
                       4 as libc::c_int) == 0 {
        return
    }
    spell = necro_powers[n as usize];
    /* Verify "dangerous" spells */
    if spell.mana_cost > (*p_ptr).csp as libc::c_int {
        /* Warning */
        msg_print(b"You do not have enough mana to use this power.\x00" as
                      *const u8 as *const libc::c_char);
        /* Verify */
        if get_check(b"Attempt it anyway? \x00" as *const u8 as
                         *const libc::c_char) == 0 {
            return
        }
    }
    /* Spell failure chance */
    chance = spell.fail;
    /* Reduce failure rate by "effective" level adjustment */
    chance -= 3 as libc::c_int * (plev - spell.min_lev);
    /* Reduce failure rate by INT/WIS adjustment */
    chance -=
        3 as libc::c_int *
            (*adj_mag_stat.as_mut_ptr().offset((*p_ptr).stat_ind[4 as
                                                                     libc::c_int
                                                                     as usize]
                                                   as isize) as libc::c_int -
                 1 as libc::c_int);
    /* Not enough mana to cast */
    if spell.mana_cost > (*p_ptr).csp as libc::c_int {
        chance +=
            5 as libc::c_int * (spell.mana_cost - (*p_ptr).csp as libc::c_int)
    }
    /* Extract the minimum failure rate */
    minfail =
        *adj_mag_fail.as_mut_ptr().offset((*p_ptr).stat_ind[4 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int;
    /* Failure rate */
    chance = clamp_failure_chance(chance, minfail);
    /* Failed spell */
    if Rand_div(100 as libc::c_int) < chance {
        if flush_failure != 0 { flush(); }
        msg_format(b"You failed to concentrate hard enough!\x00" as *const u8
                       as *const libc::c_char);
        sound(55 as libc::c_int);
        if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
               chance / 2 as libc::c_int {
            /* Backfire */
            b = Rand_div(100 as libc::c_int) + 1 as libc::c_int;
            if b < 10 as libc::c_int {
                msg_print(b"Oh, no! You become undead!\x00" as *const u8 as
                              *const libc::c_char);
                (*p_ptr).necro_extra |= 0x8 as libc::c_int as libc::c_uint;
                (*p_ptr).necro_extra2 = (2 as libc::c_int * plev) as u32b;
                msg_format(b"You have to kill %d monster%s to be brought back to life.\x00"
                               as *const u8 as *const libc::c_char,
                           (*p_ptr).necro_extra2,
                           if (*p_ptr).necro_extra2 ==
                                  1 as libc::c_int as libc::c_uint {
                               b"\x00" as *const u8 as *const libc::c_char
                           } else {
                               b"s\x00" as *const u8 as *const libc::c_char
                           });
                /* MEGA-HACK !!! */
                calc_hitpoints();
                /* Enforce maximum */
                (*p_ptr).chp = (*p_ptr).mhp;
                (*p_ptr).chp_frac = 0 as libc::c_int as u16b;
                /* Display the hitpoints */
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long)
                        as u32b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as
                        u32b
            } else if b < 40 as libc::c_int {
                msg_print(b"Suddenly you feel that you\'re in a bad situation...\x00"
                              as *const u8 as *const libc::c_char);
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                *max_dlv.offset(dungeon_type as isize) as
                                    libc::c_int,
                                if plev >= 30 as libc::c_int {
                                    21 as libc::c_int
                                } else { 17 as libc::c_int });
            } else {
                msg_print(b"Your body is damaged by the horrible forces of the spell!\x00"
                              as *const u8 as *const libc::c_char);
                take_hit(damroll(5 as libc::c_int as s16b, plev as s16b),
                         b"using necromancy unwisely\x00" as *const u8 as
                             *const libc::c_char);
            }
        }
    } else {
        sound(12 as libc::c_int);
        /* spell code */
        match n {
            0 => {
                /* Horrify */
                let mut dam: libc::c_int =
                    damroll((2 as libc::c_int +
                                 plev * 2 as libc::c_int / 3 as libc::c_int)
                                as s16b, 4 as libc::c_int as s16b) +
                        (*p_ptr).to_s as libc::c_int * 2 as libc::c_int;
                if plev > 45 as libc::c_int {
                    project_hack(78 as libc::c_int, dam);
                    project_hack(66 as libc::c_int, dam);
                } else if plev > 35 as libc::c_int {
                    if get_aim_dir(&mut dir) == 0 { return }
                    fire_ball(78 as libc::c_int, dir, dam,
                              3 as libc::c_int + plev / 10 as libc::c_int);
                    fire_ball(66 as libc::c_int, dir, dam,
                              3 as libc::c_int + plev / 10 as libc::c_int);
                } else if plev > 20 as libc::c_int {
                    if get_aim_dir(&mut dir) == 0 { return }
                    fire_beam(78 as libc::c_int, dir, dam);
                    fire_beam(66 as libc::c_int, dir, dam);
                } else {
                    if get_aim_dir(&mut dir) == 0 { return }
                    fire_bolt(78 as libc::c_int, dir, dam);
                    fire_bolt(66 as libc::c_int, dir, dam);
                }
            }
            1 => {
                /* Raise Death */
                fire_ball(92 as libc::c_int, 0 as libc::c_int,
                          plev * 3 as libc::c_int,
                          1 as libc::c_int + to_s2 +
                              plev / 10 as libc::c_int);
            }
            2 => {
                /* Conjures temporary weapon */
                let mut dur: libc::c_int =
                    Rand_div(100 as libc::c_int + plev * 4 as libc::c_int) +
                        1 as libc::c_int + 200 as libc::c_int +
                        plev * 3 as libc::c_int;
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
                let mut o_ptr: *mut object_type = &mut forge;
                let mut k_idx: libc::c_int =
                    test_item_name(b"& Necromantic Teeth~\x00" as *const u8 as
                                       *const libc::c_char);
                *k_allow_special.offset(k_idx as isize) =
                    1 as libc::c_int as bool_;
                object_prep(o_ptr, k_idx);
                apply_magic(o_ptr, plev * 2 as libc::c_int,
                            1 as libc::c_int as bool_,
                            1 as libc::c_int as bool_,
                            1 as libc::c_int as bool_);
                (*o_ptr).art_flags5 =
                    ((*o_ptr).art_flags5 as libc::c_long |
                         0x1 as libc::c_long) as u32b;
                (*o_ptr).timeout = dur as s16b;
                /* These objects are "storebought" */
                (*o_ptr).ident =
                    ((*o_ptr).ident as libc::c_int | 0x20 as libc::c_int) as
                        byte_hack;
                (*o_ptr).number = 1 as libc::c_int as byte_hack;
                object_aware(o_ptr);
                object_known(o_ptr);
                inven_carry(o_ptr, 0 as libc::c_int as bool_);
                *k_allow_special.offset(k_idx as isize) =
                    0 as libc::c_int as bool_
            }
            3 => {
                /* Absorb souls */
                set_absorb_soul(Rand_div(30 as libc::c_int +
                                             plev * 2 as libc::c_int) +
                                    1 as libc::c_int + 50 as libc::c_int +
                                    plev);
            }
            4 => {
                /* Vampirism */
                let mut i: libc::c_int = 0;
                if get_aim_dir(&mut dir) == 0 { return }
                i = 0 as libc::c_int;
                while i < 1 as libc::c_int + to_s2 + plev / 15 as libc::c_int
                      {
                    if drain_life(dir, 100 as libc::c_int) != 0 {
                        hp_player(100 as libc::c_int);
                    }
                    i += 1
                }
            }
            5 => {
                /* Death */
                if get_check(b"Using the Death word will leave you undead, with 1 DP. Do you *really* want to use it? \x00"
                                 as *const u8 as *const libc::c_char) != 0 {
                    if get_aim_dir(&mut dir) == 0 { return }
                    fire_bolt(105 as libc::c_int, dir, 1 as libc::c_int);
                    (*p_ptr).necro_extra |=
                        0x8 as libc::c_int as libc::c_uint;
                    (*p_ptr).necro_extra2 =
                        (plev +
                             (Rand_div(plev / 2 as libc::c_int) -
                                  plev / 4 as libc::c_int)) as u32b;
                    msg_format(b"You have to kill %d monster%s to be brought back to life.\x00"
                                   as *const u8 as *const libc::c_char,
                               (*p_ptr).necro_extra2,
                               if (*p_ptr).necro_extra2 ==
                                      1 as libc::c_int as libc::c_uint {
                                   b"\x00" as *const u8 as *const libc::c_char
                               } else {
                                   b"s\x00" as *const u8 as
                                       *const libc::c_char
                               });
                    /* MEGA-HACK !!! */
                    calc_hitpoints();
                    /* Enforce 1 DP */
                    (*p_ptr).chp = (*p_ptr).mhp;
                    (*p_ptr).chp_frac = 0 as libc::c_int as u16b;
                    /* Display the hitpoints */
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x40 as libc::c_long) as u32b;
                    /* Window stuff */
                    (*p_ptr).window =
                        ((*p_ptr).window as libc::c_long |
                             0x8 as libc::c_long) as u32b
                }
            }
            _ => {
                msg_print(b"Zap?\x00" as *const u8 as *const libc::c_char);
            }
        }
    }
    /* Take a turn */
    if is_magestaff() != 0 {
        energy_use = 80 as libc::c_int
    } else { energy_use = 100 as libc::c_int }
    /* Sufficient mana */
    if spell.mana_cost <= (*p_ptr).csp as libc::c_int {
        /* Use some mana */
        (*p_ptr).csp = ((*p_ptr).csp as libc::c_int - spell.mana_cost) as s16b
    } else {
        /* Over-exert the player */
        let mut oops: libc::c_int =
            spell.mana_cost - (*p_ptr).csp as libc::c_int;
        /* No mana left */
        (*p_ptr).csp = 0 as libc::c_int as s16b;
        (*p_ptr).csp_frac = 0 as libc::c_int as u16b;
        /* Message */
        msg_print(b"You faint from the effort!\x00" as *const u8 as
                      *const libc::c_char);
        /* Hack -- Bypass free action */
        set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                          (Rand_div(5 as libc::c_int * oops +
                                        1 as libc::c_int) +
                               1 as libc::c_int));
        /* Damage CON (possibly permanently) */
        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
            let mut perm: bool_ =
                (Rand_div(100 as libc::c_int) < 25 as libc::c_int) as
                    libc::c_int as bool_;
            /* Message */
            msg_print(b"You have damaged your body!\x00" as *const u8 as
                          *const libc::c_char);
            /* Reduce constitution */
            dec_stat(4 as libc::c_int,
                     15 as libc::c_int +
                         (Rand_div(10 as libc::c_int) + 1 as libc::c_int),
                     perm as libc::c_int);
        }
    }
    /* Redraw mana */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
}
/* Runecrafters -- Move this into variable.c XXX XXX XXX */
static mut rune_combine: s32b = 0 as libc::c_int;
/*
 * Hook to determine if an object is "runestone"
 */
unsafe extern "C" fn item_tester_hook_runestone(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int != 105 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).sval as libc::c_int != 0xff as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).pval != 0 as libc::c_int { return 0 as libc::c_int as bool_ }
    /* Assume yes */
    return 1 as libc::c_int as bool_;
}
unsafe extern "C" fn item_tester_hook_runestone_full(mut o_ptr:
                                                         *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int != 105 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).sval as libc::c_int != 0xff as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).pval == 0 as libc::c_int { return 0 as libc::c_int as bool_ }
    /* Assume yes */
    return 1 as libc::c_int as bool_;
}
/*
 * Hook to determine if an object is "rune-able"
 */
unsafe extern "C" fn item_tester_hook_runeable1(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int != 104 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Assume yes */
    return 1 as libc::c_int as bool_;
}
/*
 * Hook to determine if an object is "rune-able"
 */
unsafe extern "C" fn item_tester_hook_runeable2(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int != 105 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).sval as libc::c_int == 0xff as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if rune_combine as libc::c_long &
           (1 as libc::c_long) << (*o_ptr).sval as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Assume yes */
    return 1 as libc::c_int as bool_;
}
/*
 * math.h(sqrt) is banned of angband so ... :)
 */
#[no_mangle]
pub unsafe extern "C" fn sroot(mut n: s32b) -> s32b {
    let mut i: s32b = n / 2 as libc::c_int;
    if n < 2 as libc::c_int { return n }
    loop  {
        let mut err: s32b =
            (i - n / (i + 1 as libc::c_int)) / 2 as libc::c_int;
        if err == 0 { break ; }
        i -= err
    }
    return if n / i < i { (i) - 1 as libc::c_int } else { i };
}
/*
 * Damage formula, for runes
 */
#[no_mangle]
pub unsafe extern "C" fn rune_calc_power(mut power: *mut s32b,
                                         mut powerdiv: *mut s32b) {
    /* Not too weak power(paranoia) */
    *power =
        if *power < 1 as libc::c_int { 1 as libc::c_int } else { *power };
    *power += 3 as libc::c_int;
    *power = 37 as libc::c_int * sroot(*power) / 10 as libc::c_int;
    /* To reduce the high level power, while increasing the low levels */
    *powerdiv = *power / 3 as libc::c_int;
    if *powerdiv < 1 as libc::c_int { *powerdiv = 1 as libc::c_int }
    /* Use the spell multiplicator */
    *power *=
        if (*p_ptr).to_s as libc::c_int / 2 as libc::c_int != 0 {
            ((*p_ptr).to_s as libc::c_int) / 2 as libc::c_int
        } else { 1 as libc::c_int };
}
/*
 * Return percentage chance of runespell failure.
 */
#[no_mangle]
pub unsafe extern "C" fn spell_chance_rune(mut spell: *mut rune_spell)
 -> libc::c_int {
    let mut chance: libc::c_int = 0;
    let mut minfail: libc::c_int = 0;
    let mut power: s32b = (*spell).mana as s32b;
    let mut power_rune: s32b = 0 as libc::c_int;
    let mut powerdiv: s32b = 0 as libc::c_int;
    if (*spell).rune2 as libc::c_int & 0x10 as libc::c_int != 0 {
        power_rune += 4 as libc::c_int
    }
    if (*spell).rune2 as libc::c_int & 0x20 as libc::c_int != 0 {
        power_rune += 3 as libc::c_int
    }
    if (*spell).rune2 as libc::c_int & 0x8 as libc::c_int != 0 {
        power_rune += 2 as libc::c_int
    }
    if (*spell).rune2 as libc::c_int & 0x4 as libc::c_int != 0 {
        power_rune += 1 as libc::c_int
    }
    rune_calc_power(&mut power, &mut powerdiv);
    chance = 5 as libc::c_int * power_rune + power;
    /* Reduce failure rate by INT/WIS adjustment */
    chance -=
        3 as libc::c_int *
            (*adj_mag_stat.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                     libc::c_int
                                                                     as usize]
                                                   as isize) as libc::c_int -
                 1 as libc::c_int);
    /* Extract the minimum failure rate */
    minfail =
        *adj_mag_fail.as_mut_ptr().offset((*p_ptr).stat_ind[3 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int;
    /* Return the chance */
    return clamp_failure_chance(chance, minfail);
}
/*
 * Combine the Runes
 */
#[no_mangle]
pub unsafe extern "C" fn rune_exec(mut spell: *mut rune_spell,
                                   mut cost: libc::c_int) -> libc::c_int {
    let mut dir: libc::c_int = 0;
    let mut power_rune: libc::c_int = 0 as libc::c_int;
    let mut mana_used: libc::c_int = 0;
    let mut plev: libc::c_int = get_skill(34 as libc::c_int) as libc::c_int;
    let mut chance: libc::c_int = 0;
    let mut power: s32b = 0;
    let mut powerdiv: s32b = 0;
    let mut rad: libc::c_int = 0 as libc::c_int;
    let mut ty: libc::c_int = -(1 as libc::c_int);
    let mut tx: libc::c_int = -(1 as libc::c_int);
    let mut dam: libc::c_int = 0 as libc::c_int;
    let mut flg: libc::c_int = 0 as libc::c_int;
    if (*spell).rune2 as libc::c_int & 0x10 as libc::c_int != 0 {
        power_rune += 4 as libc::c_int
    }
    if (*spell).rune2 as libc::c_int & 0x20 as libc::c_int != 0 {
        power_rune += 3 as libc::c_int
    }
    if (*spell).rune2 as libc::c_int & 0x8 as libc::c_int != 0 {
        power_rune += 2 as libc::c_int
    }
    if (*spell).rune2 as libc::c_int & 0x4 as libc::c_int != 0 {
        power_rune += 1 as libc::c_int
    }
    power = (*spell).mana as s32b;
    if cost != 0 &&
           power * cost / 100 as libc::c_int >
               (*p_ptr).csp as libc::c_int -
                   power_rune * (plev / 5 as libc::c_int) {
        power =
            (*p_ptr).csp as libc::c_int -
                power_rune * (plev / 5 as libc::c_int);
        mana_used = power + power_rune * (plev / 5 as libc::c_int)
    } else {
        mana_used =
            power * cost / 100 as libc::c_int +
                power_rune * (plev / 5 as libc::c_int)
    }
    rune_calc_power(&mut power, &mut powerdiv);
    dam = damroll(powerdiv as s16b, power as s16b);
    if wizard != 0 {
        msg_format(b"Rune %dd%d = dam %d\x00" as *const u8 as
                       *const libc::c_char, powerdiv, power, dam);
    }
    /* Extract the base spell failure rate */
    chance = spell_chance_rune(spell);
    /* Failure ? */
    if Rand_div(100 as libc::c_int) < chance {
        let mut insanity: libc::c_int =
            ((*p_ptr).msane as libc::c_int - (*p_ptr).csane as libc::c_int) *
                100 as libc::c_int / (*p_ptr).msane as libc::c_int;
        let mut sfail: [libc::c_char; 80] = [0; 80];
        /* Flush input if told so */
        if flush_failure != 0 { flush(); }
        /* Insane players can see something strange */
        if Rand_div(100 as libc::c_int) < insanity {
            get_rnd_line(b"sfail.txt\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char, sfail.as_mut_ptr());
            msg_format(b"A cloud of %s appears above you.\x00" as *const u8 as
                           *const libc::c_char, sfail.as_mut_ptr());
        } else {
            /* Normal failure messages */
            msg_print(b"You failed to get the spell off!\x00" as *const u8 as
                          *const libc::c_char);
        }
        sound(55 as libc::c_int);
        if is_magestaff() != 0 {
            energy_use = 80 as libc::c_int
        } else { energy_use = 100 as libc::c_int }
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
        return mana_used
    }
    if (*spell).rune2 as libc::c_int & 0x10 as libc::c_int != 0 {
        flg |= 0x100 as libc::c_int;
        ty = (*p_ptr).py as libc::c_int;
        tx = (*p_ptr).px as libc::c_int
    }
    if (*spell).rune2 as libc::c_int & 0x20 as libc::c_int != 0 {
        flg |= 0x4 as libc::c_int;
        flg |= 0x40 as libc::c_int;
        flg |= 0x20 as libc::c_int;
        flg |= 0x10 as libc::c_int;
        flg |= 0x200 as libc::c_int;
        rad =
            if power / 8 as libc::c_int == 0 as libc::c_int {
                1 as libc::c_int
            } else { (power) / 8 as libc::c_int };
        rad = if rad > 10 as libc::c_int { 10 as libc::c_int } else { rad };
        ty = (*p_ptr).py as libc::c_int;
        tx = (*p_ptr).px as libc::c_int
    }
    if (*spell).rune2 as libc::c_int & 0x8 as libc::c_int != 0 {
        flg |= 0x4 as libc::c_int;
        flg |= 0x40 as libc::c_int;
        flg |= 0x20 as libc::c_int;
        flg |= 0x10 as libc::c_int;
        rad =
            if power / 8 as libc::c_int == 0 as libc::c_int {
                1 as libc::c_int
            } else { (power) / 8 as libc::c_int };
        rad = if rad > 10 as libc::c_int { 10 as libc::c_int } else { rad };
        ty = (*p_ptr).py as libc::c_int;
        tx = (*p_ptr).px as libc::c_int
    }
    if (*spell).rune2 as libc::c_int & 0x4 as libc::c_int != 0 {
        flg |= 0x4 as libc::c_int;
        flg |= 0x40 as libc::c_int;
        flg |= 0x2 as libc::c_int;
        ty = -(1 as libc::c_int);
        tx = -(1 as libc::c_int)
    }
    if (*spell).rune2 as libc::c_int & 0x2 as libc::c_int != 0 {
        flg |= 0x4 as libc::c_int;
        flg |= 0x8 as libc::c_int;
        flg |= 0x40 as libc::c_int;
        ty = -(1 as libc::c_int);
        tx = -(1 as libc::c_int)
    }
    if (*spell).rune2 as libc::c_int & 0x1 as libc::c_int != 0 {
        flg |= 0x4 as libc::c_int;
        flg |= 0x8 as libc::c_int;
        flg |= 0x40 as libc::c_int;
        ty = (*p_ptr).py as libc::c_int;
        tx = (*p_ptr).px as libc::c_int;
        unsafe_0 = 1 as libc::c_int as bool_
    }
    if ty == -(1 as libc::c_int) && tx == -(1 as libc::c_int) {
        if get_aim_dir(&mut dir) == 0 { return mana_used }
        /* Use the given direction */
        tx = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
        ty = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
        /* Hack -- Use an actual "target" */
        if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
            tx = target_col as libc::c_int;
            ty = target_row as libc::c_int
        }
    }
    if flg & 0x100 as libc::c_int != 0 {
        project_hack((*spell).type_0 as libc::c_int, dam);
    } else if flg & 0x200 as libc::c_int != 0 {
        project_meteor(rad, (*spell).type_0 as libc::c_int, dam, flg as u32b);
    } else {
        project(0 as libc::c_int, rad, ty, tx, dam,
                (*spell).type_0 as libc::c_int, flg);
    }
    if unsafe_0 != 0 { unsafe_0 = 0 as libc::c_int as bool_ }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
    return mana_used;
}
/*
 * Test if all runes needed at in the player p_ptr->inventory
 */
#[no_mangle]
pub unsafe extern "C" fn test_runespell(mut spell: *mut rune_spell) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut typeok: bool_ = 0 as libc::c_int as bool_;
    let mut rune2: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if !((*o_ptr).k_idx == 0) {
            /* Does the rune1(type) match ? */
            if (*o_ptr).tval as libc::c_int == 104 as libc::c_int &&
                   (*o_ptr).sval as libc::c_int ==
                       (*spell).type_0 as libc::c_int {
                typeok = 1 as libc::c_int as bool_
            }
            if (*o_ptr).tval as libc::c_int == 105 as libc::c_int &&
                   (*o_ptr).sval as libc::c_int != 0xff as libc::c_int {
                /* Add it to the list */
                rune2 |= (1 as libc::c_int) << (*o_ptr).sval as libc::c_int
            }
        }
        i += 1
    }
    /* Need all runes to be present */
    return (typeok as libc::c_int != 0 &&
                rune2 & (*spell).rune2 as libc::c_int ==
                    (*spell).rune2 as libc::c_int) as libc::c_int as bool_;
}
/*
 * Ask for rune, rune2 and mana
 */
#[no_mangle]
pub unsafe extern "C" fn get_runespell(mut spell: *mut rune_spell) -> bool_ {
    let mut item: libc::c_int = 0;
    let mut power_rune: libc::c_int = 0 as libc::c_int;
    let mut rune2: libc::c_int = 0 as libc::c_int;
    let mut plev: libc::c_int = get_skill(34 as libc::c_int) as libc::c_int;
    let mut power: s32b = 0;
    let mut type_0: libc::c_int = 0 as libc::c_int;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut OK: bool_ = 0 as libc::c_int as bool_;
    rune_combine = 0 as libc::c_int;
    /* Restrict choices to unused runes */
    item_tester_hook =
        Some(item_tester_hook_runeable1 as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q = b"Use which rune? \x00" as *const u8 as *const libc::c_char;
    s = b"You have no rune to use.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return 0 as libc::c_int as bool_
    }
    /* Get the item */
    o_ptr = get_object(item);
    type_0 = (*o_ptr).sval as libc::c_int;
    loop  {
        /* Restrict choices to unused secondary runes */
        item_tester_hook =
            Some(item_tester_hook_runeable2 as
                     unsafe extern "C" fn(_: *mut object_type) -> bool_);
        OK =
            (get_item(&mut item, q, s,
                      0x2 as libc::c_int | 0x4 as libc::c_int) == 0) as
                libc::c_int as bool_;
        if OK != 0 { break ; }
        /* Get the item */
        o_ptr = get_object(item);
        rune_combine |= (1 as libc::c_int) << (*o_ptr).sval as libc::c_int;
        rune2 |= (1 as libc::c_int) << (*o_ptr).sval as libc::c_int
    }
    if rune2 == 0 {
        msg_print(b"You have not selected a second rune!\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    power =
        get_quantity(b"Which amount of Mana?\x00" as *const u8 as
                         *const libc::c_char,
                     (*p_ptr).csp as libc::c_int -
                         power_rune * (plev / 5 as libc::c_int));
    if power < 1 as libc::c_int { power = 1 as libc::c_int }
    (*spell).mana = power as s16b;
    (*spell).type_0 = type_0 as s16b;
    (*spell).rune2 = rune2 as s16b;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_rune() {
    let mut spell: rune_spell =
        rune_spell{name: [0; 30], type_0: 0, rune2: 0, mana: 0,};
    /* Require some mana */
    if (*p_ptr).csp as libc::c_int <= 0 as libc::c_int {
        msg_print(b"You have no mana!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Require lite */
    if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0 {
        msg_print(b"You cannot see!\x00" as *const u8 as *const libc::c_char);
        return
    }
    /* Not when confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if get_runespell(&mut spell) == 0 { return }
    /* Execute at normal mana cost */
    (*p_ptr).csp =
        ((*p_ptr).csp as libc::c_int -
             rune_exec(&mut spell, 100 as libc::c_int)) as s16b;
    /* Safety :) */
    if ((*p_ptr).csp as libc::c_int) < 0 as libc::c_int {
        (*p_ptr).csp = 0 as libc::c_int as s16b
    }
    /* Take a turn */
    if is_magestaff() != 0 {
        energy_use = 80 as libc::c_int
    } else { energy_use = 100 as libc::c_int }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Print a batch of runespells.
 */
unsafe extern "C" fn print_runespell_batch(mut batch: libc::c_int,
                                           mut max: libc::c_int) {
    let mut buff: [libc::c_char; 80] = [0; 80];
    let mut spell: *mut rune_spell = 0 as *mut rune_spell;
    let mut i: libc::c_int = 0;
    let mut power: s32b = 0;
    let mut powerdiv: s32b = 0;
    let mut p: libc::c_int = 0;
    let mut dp: libc::c_int = 0;
    prt(format(b"      %-30s Fail Mana Power\x00" as *const u8 as
                   *const libc::c_char,
               b"Name\x00" as *const u8 as *const libc::c_char) as cptr,
        1 as libc::c_int, 20 as libc::c_int);
    i = 0 as libc::c_int;
    while i < max {
        spell =
            &mut *rune_spells.as_mut_ptr().offset((batch * 10 as libc::c_int +
                                                       i) as isize) as
                *mut rune_spell;
        power = (*spell).mana as s32b;
        rune_calc_power(&mut power, &mut powerdiv);
        p = power;
        dp = powerdiv;
        strnfmt(buff.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"  %c) %-30s %4d%% %4d %dd%d \x00" as *const u8 as
                    *const libc::c_char, i + 'a' as i32,
                (*spell).name.as_mut_ptr(), spell_chance_rune(spell),
                (*spell).mana as libc::c_int, dp, p);
        prt(buff.as_mut_ptr() as cptr, 2 as libc::c_int + i,
            20 as libc::c_int);
        i += 1
    }
    prt(b"\x00" as *const u8 as *const libc::c_char, 2 as libc::c_int + i,
        20 as libc::c_int);
}
/*
 * List ten random spells and ask to pick one.
 */
unsafe extern "C" fn select_runespell_from_batch(mut batch: libc::c_int,
                                                 mut quick: bool_,
                                                 mut s_idx: *mut libc::c_int)
 -> *mut rune_spell {
    let mut tmp: [libc::c_char; 160] = [0; 160];
    let mut out_val: [libc::c_char; 30] = [0; 30];
    let mut which: libc::c_char = 0;
    let mut mut_max: libc::c_int = 10 as libc::c_int;
    let mut ret: *mut rune_spell = 0 as *mut rune_spell;
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    if (rune_num as libc::c_int) <
           (batch + 1 as libc::c_int) * 10 as libc::c_int {
        mut_max = rune_num as libc::c_int - batch * 10 as libc::c_int
    }
    strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
            b"(a-%c, * to list, / to rename, - to comment) Select a power: \x00"
                as *const u8 as *const libc::c_char,
            mut_max - 1 as libc::c_int + 'a' as i32);
    prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
    if quick != 0 { print_runespell_batch(batch, mut_max); }
    loop  {
        which = inkey();
        if which as libc::c_int == '\u{1b}' as i32 {
            *s_idx = -(1 as libc::c_int);
            ret = 0 as *mut rune_spell;
            break ;
        } else if which as libc::c_int == '*' as i32 ||
                      which as libc::c_int == '?' as i32 ||
                      which as libc::c_int == ' ' as i32 {
            print_runespell_batch(batch, mut_max);
        } else if which as libc::c_int == '\r' as i32 &&
                      mut_max == 1 as libc::c_int {
            *s_idx = batch * 10 as libc::c_int;
            ret =
                &mut *rune_spells.as_mut_ptr().offset((batch *
                                                           10 as libc::c_int)
                                                          as isize) as
                    *mut rune_spell;
            break ;
        } else if which as libc::c_int == '/' as i32 {
            prt(b"Rename which power: \x00" as *const u8 as
                    *const libc::c_char, 0 as libc::c_int, 0 as libc::c_int);
            which = tolower(inkey() as libc::c_int) as libc::c_char;
            if *(*__ctype_b_loc()).offset(which as libc::c_int as isize) as
                   libc::c_int &
                   _ISalpha as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 && which as libc::c_int - 'a' as i32 <= mut_max {
                strcpy(out_val.as_mut_ptr(),
                       rune_spells[(batch * 10 as libc::c_int +
                                        (which as libc::c_int - 'a' as i32))
                                       as usize].name.as_mut_ptr());
                if get_string(b"Name this power: \x00" as *const u8 as
                                  *const libc::c_char, out_val.as_mut_ptr(),
                              29 as libc::c_int) != 0 {
                    strcpy(rune_spells[(batch * 10 as libc::c_int +
                                            (which as libc::c_int -
                                                 'a' as i32)) as
                                           usize].name.as_mut_ptr(),
                           out_val.as_mut_ptr());
                }
                prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int,
                    0 as libc::c_int);
            } else {
                bell();
                prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int,
                    0 as libc::c_int);
            }
        } else {
            which = tolower(which as libc::c_int) as libc::c_char;
            if *(*__ctype_b_loc()).offset(which as libc::c_int as isize) as
                   libc::c_int &
                   _ISalpha as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 && (which as libc::c_int - 'a' as i32) < mut_max {
                *s_idx =
                    batch * 10 as libc::c_int +
                        (which as libc::c_int - 'a' as i32);
                ret =
                    &mut *rune_spells.as_mut_ptr().offset((batch *
                                                               10 as
                                                                   libc::c_int
                                                               +
                                                               (which as
                                                                    libc::c_int
                                                                    -
                                                                    'a' as
                                                                        i32))
                                                              as isize) as
                        *mut rune_spell;
                break ;
            } else { bell(); }
        }
    }
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    return ret;
}
/*
 * Pick a random spell from a menu
 */
#[no_mangle]
pub unsafe extern "C" fn select_runespell(mut quick: bool_,
                                          mut s_idx: *mut libc::c_int)
 -> *mut rune_spell {
    let mut tmp: [libc::c_char; 160] = [0; 160];
    let mut which: libc::c_char = 0;
    let mut batch_max: libc::c_int =
        (rune_num as libc::c_int - 1 as libc::c_int) / 10 as libc::c_int;
    if rune_num as libc::c_int == 0 as libc::c_int {
        msg_print(b"There are no runespells you can cast.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as *mut rune_spell
    }
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
            b"(a-%c) Select batch of powers: \x00" as *const u8 as
                *const libc::c_char, batch_max + 'a' as i32);
    prt(tmp.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
    loop  {
        which = inkey();
        if which as libc::c_int == '\u{1b}' as i32 {
            Term_load();
            character_icky = 0 as libc::c_int as bool_;
            return 0 as *mut rune_spell
        } else {
            if which as libc::c_int == '\r' as i32 &&
                   batch_max == 0 as libc::c_int {
                Term_load();
                character_icky = 0 as libc::c_int as bool_;
                return select_runespell_from_batch(0 as libc::c_int, quick,
                                                   s_idx)
            } else {
                which = tolower(which as libc::c_int) as libc::c_char;
                if *(*__ctype_b_loc()).offset(which as libc::c_int as isize)
                       as libc::c_int &
                       _ISalpha as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 &&
                       which as libc::c_int - 'a' as i32 <= batch_max {
                    Term_load();
                    character_icky = 0 as libc::c_int as bool_;
                    return select_runespell_from_batch(which as libc::c_int -
                                                           'a' as i32, quick,
                                                       s_idx)
                } else { bell(); }
            }
        }
    };
}
/*
 * Cast a memorized runespell
 * Note that the only limits are antimagic & conf, NOT blind
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_rune_cast() {
    let mut s_ptr: *mut rune_spell = 0 as *mut rune_spell;
    let mut s_idx: libc::c_int = 0;
    /* Require some mana */
    if (*p_ptr).csp as libc::c_int <= 0 as libc::c_int {
        msg_print(b"You have no mana!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).anti_magic != 0 {
        msg_print(b"Your anti-magic shell disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Not when confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    s_ptr = select_runespell(0 as libc::c_int as bool_, &mut s_idx);
    if s_ptr.is_null() { return }
    /* Need the runes */
    if test_runespell(s_ptr) == 0 {
        msg_print(b"You lack some essential rune(s) for this runespell!\x00"
                      as *const u8 as *const libc::c_char);
        return
    }
    /* Execute at normal mana cost */
    (*p_ptr).csp =
        ((*p_ptr).csp as libc::c_int - rune_exec(s_ptr, 100 as libc::c_int))
            as s16b;
    /* Safety :) */
    if ((*p_ptr).csp as libc::c_int) < 0 as libc::c_int {
        (*p_ptr).csp = 0 as libc::c_int as s16b
    }
    /* Take a turn */
    if is_magestaff() != 0 {
        energy_use = 80 as libc::c_int
    } else { energy_use = 100 as libc::c_int }
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Cast a runespell from a carved runestone
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_runestone() {
    let mut s_ptr: rune_spell =
        rune_spell{name: [0; 30], type_0: 0, rune2: 0, mana: 0,};
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut item: libc::c_int = 0;
    /* Require some mana */
    if (*p_ptr).csp as libc::c_int <= 0 as libc::c_int {
        msg_print(b"You have no mana!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Require lite */
    if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0 {
        msg_print(b"You cannot see!\x00" as *const u8 as *const libc::c_char);
        return
    }
    /* Not when confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).anti_magic != 0 {
        msg_print(b"Your anti-magic shell disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Restrict choices to unused runes */
    item_tester_hook =
        Some(item_tester_hook_runestone_full as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q =
        b"Cast from which runestone? \x00" as *const u8 as
            *const libc::c_char;
    s =
        b"You have no runestone to cast from.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    s_ptr.type_0 = (*o_ptr).pval as s16b;
    s_ptr.rune2 = (*o_ptr).pval2;
    s_ptr.mana = (*o_ptr).pval3 as s16b;
    /* Execute less mana */
    (*p_ptr).csp =
        ((*p_ptr).csp as libc::c_int -
             rune_exec(&mut s_ptr, 75 as libc::c_int)) as s16b;
    /* Safety :) */
    if ((*p_ptr).csp as libc::c_int) < 0 as libc::c_int {
        (*p_ptr).csp = 0 as libc::c_int as s16b
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Add a runespell to the list
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_rune_add_mem() {
    let mut s_ptr: rune_spell =
        rune_spell{name: [0; 30], type_0: 0, rune2: 0, mana: 0,};
    let mut ds_ptr: *mut rune_spell =
        &mut *rune_spells.as_mut_ptr().offset(rune_num as isize) as
            *mut rune_spell;
    /* Not when confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if rune_num as libc::c_int >= 100 as libc::c_int {
        msg_print(b"You have already learn the maximun number of runespells!\x00"
                      as *const u8 as *const libc::c_char);
        return
    }
    if get_runespell(&mut s_ptr) == 0 { return }
    (*ds_ptr).type_0 = s_ptr.type_0;
    (*ds_ptr).rune2 = s_ptr.rune2;
    (*ds_ptr).mana = s_ptr.mana;
    strcpy((*ds_ptr).name.as_mut_ptr(),
           b"Unnamed Runespell\x00" as *const u8 as *const libc::c_char);
    get_string(b"Name this runespell: \x00" as *const u8 as
                   *const libc::c_char, (*ds_ptr).name.as_mut_ptr(),
               29 as libc::c_int);
    rune_num += 1;
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Carve a runespell onto a Runestone
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_rune_carve() {
    let mut s_ptr: rune_spell =
        rune_spell{name: [0; 30], type_0: 0, rune2: 0, mana: 0,};
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut item: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut out_val: [libc::c_char; 80] = [0; 80];
    /* Not when confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Require lite */
    if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0 {
        msg_print(b"You cannot see!\x00" as *const u8 as *const libc::c_char);
        return
    }
    if get_check(b"Beware, this will destroy the involved runes, continue?\x00"
                     as *const u8 as *const libc::c_char) == 0 {
        return
    }
    if get_runespell(&mut s_ptr) == 0 { return }
    /* Restrict choices to unused runes */
    item_tester_hook =
        Some(item_tester_hook_runestone as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q = b"Use which runestone? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have no runestone to use.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    (*o_ptr).pval = s_ptr.type_0 as s32b;
    (*o_ptr).pval2 = s_ptr.rune2;
    (*o_ptr).pval3 = s_ptr.mana as s32b;
    /* Start with nothing */
    strcpy(out_val.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
    /* Use old inscription */
    if (*o_ptr).note != 0 {
        /* Start with the old inscription */
        strcpy(out_val.as_mut_ptr(), quark_str((*o_ptr).note as s16b));
    }
    /* Get a new inscription (possibly empty) */
    if get_string(b"Name this runestone: \x00" as *const u8 as
                      *const libc::c_char, out_val.as_mut_ptr(),
                  80 as libc::c_int) != 0 {
        /* Save the inscription */
        (*o_ptr).note = quark_add(out_val.as_mut_ptr() as cptr) as u16b;
        /* Combine the pack */
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
    }
    /* Delete the runes */
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if (*o_ptr).k_idx != 0 {
            let mut do_del: bool_ = 0 as libc::c_int as bool_;
            if (*o_ptr).tval as libc::c_int == 104 as libc::c_int &&
                   (*o_ptr).sval as libc::c_int == s_ptr.type_0 as libc::c_int
               {
                do_del = 1 as libc::c_int as bool_
            }
            if (*o_ptr).tval as libc::c_int == 105 as libc::c_int &&
                   (1 as libc::c_long) << (*o_ptr).sval as libc::c_int &
                       s_ptr.rune2 as libc::c_long != 0 {
                do_del = 1 as libc::c_int as bool_
            }
            if do_del != 0 {
                inc_stack_size_ex(i, -(1 as libc::c_int), OPTIMIZE,
                                  NO_DESCRIBE);
            }
        }
        i += 1
    }
    /* Take a turn -- Carving takes a LONG time */
    energy_use = 400 as libc::c_int;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Remove a runespell
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_rune_del() {
    let mut s_ptr: *mut rune_spell = 0 as *mut rune_spell;
    let mut s_idx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* Not when confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    s_ptr = select_runespell(0 as libc::c_int as bool_, &mut s_idx);
    if s_ptr.is_null() { return }
    /* Delete and move */
    i = s_idx + 1 as libc::c_int;
    while i < rune_num as libc::c_int {
        rune_spells[(i - 1 as libc::c_int) as usize].type_0 =
            rune_spells[i as usize].type_0;
        rune_spells[(i - 1 as libc::c_int) as usize].rune2 =
            rune_spells[i as usize].rune2;
        rune_spells[(i - 1 as libc::c_int) as usize].mana =
            rune_spells[i as usize].mana;
        strcpy(rune_spells[(i - 1 as libc::c_int) as usize].name.as_mut_ptr(),
               rune_spells[i as usize].name.as_mut_ptr());
        i += 1
    }
    rune_num -= 1;
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_rune_add() {
    let mut ext: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_char = 0;
    loop 
         /* Select what to do */
         {
        if get_com(b"Add to [M]emory(need runes to cast) or Carve a [R]unestone(less mana to cast)\x00"
                       as *const u8 as *const libc::c_char, &mut ch) == 0 {
            ext = 0 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'M' as i32 ||
                      ch as libc::c_int == 'm' as i32 {
            ext = 1 as libc::c_int;
            break ;
        } else {
            if !(ch as libc::c_int == 'R' as i32 ||
                     ch as libc::c_int == 'r' as i32) {
                continue ;
            }
            ext = 2 as libc::c_int;
            break ;
        }
    }
    match ext {
        1 => {
            /* Create a Spell in memory */
            do_cmd_rune_add_mem();
        }
        2 => {
            /* Carve a Runestone */
            do_cmd_rune_carve();
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_runecrafter() {
    let mut ext: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_char = 0;
    loop 
         /* Select what to do */
         {
        if get_com(b"Rune Spell:[C]reate, [D]elete, C[a]st, D[i]rectly Cast or Use [R]unestone\x00"
                       as *const u8 as *const libc::c_char, &mut ch) == 0 {
            ext = 0 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'C' as i32 ||
                      ch as libc::c_int == 'c' as i32 {
            ext = 1 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'D' as i32 ||
                      ch as libc::c_int == 'd' as i32 {
            ext = 2 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'A' as i32 ||
                      ch as libc::c_int == 'a' as i32 {
            ext = 3 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'I' as i32 ||
                      ch as libc::c_int == 'i' as i32 {
            ext = 4 as libc::c_int;
            break ;
        } else {
            if !(ch as libc::c_int == 'R' as i32 ||
                     ch as libc::c_int == 'r' as i32) {
                continue ;
            }
            ext = 5 as libc::c_int;
            break ;
        }
    }
    match ext {
        1 => {
            /* Create a Spell */
            do_cmd_rune_add();
        }
        2 => {
            /* Delete a Spell */
            do_cmd_rune_del();
        }
        3 => {
            /* Cast a Spell */
            do_cmd_rune_cast();
        }
        4 => {
            /* Directly Cast a Spell */
            do_cmd_rune();
        }
        5 => {
            /* Cast a Runestone */
            do_cmd_runestone();
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_unbeliever_antimagic() {
    if (get_skill(33 as libc::c_int) as libc::c_int) < 20 as libc::c_int {
        msg_print(b"You must have at least a level 20 antimagic skill to be able to disrupt the magic continuum.\x00"
                      as *const u8 as *const libc::c_char);
        return
    }
    if (*p_ptr).antimagic_extra & 0x10 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).antimagic_extra &= !(0x10 as libc::c_int) as libc::c_uint;
        msg_print(b"You stop disrupting the magic continuum.\x00" as *const u8
                      as *const libc::c_char);
    } else {
        (*p_ptr).antimagic_extra |= 0x10 as libc::c_int as libc::c_uint;
        msg_print(b"You start disrupting the magic continuum.\x00" as
                      *const u8 as *const libc::c_char);
    }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
}
/*
 * Detect traps + kill traps
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_unbeliever() {
    let mut ext: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_char = 0;
    loop 
         /* Select what to do */
         {
        if get_com(b"Disrupt [C]ontinuum or [D]etect Traps\x00" as *const u8
                       as *const libc::c_char, &mut ch) == 0 {
            ext = 0 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'C' as i32 ||
                      ch as libc::c_int == 'c' as i32 {
            ext = 1 as libc::c_int;
            break ;
        } else {
            if !(ch as libc::c_int == 'D' as i32 ||
                     ch as libc::c_int == 'd' as i32) {
                continue ;
            }
            ext = 2 as libc::c_int;
            break ;
        }
    }
    match ext {
        1 => {
            /* Disrupt Continuum */
            do_cmd_unbeliever_antimagic();
        }
        2 => {
            /* Detect Traps */
            let mut skill: s16b = get_skill(33 as libc::c_int);
            if (skill as libc::c_int) < 25 as libc::c_int {
                msg_print(b"You cannot use your detection abilities yet.\x00"
                              as *const u8 as *const libc::c_char);
            } else {
                detect_traps(25 as libc::c_int);
                if skill as libc::c_int >= 35 as libc::c_int {
                    destroy_doors_touch();
                }
            }
        }
        _ => { }
    };
}
/*
 * Hook to determine if an object is totemable
 */
unsafe extern "C" fn item_tester_hook_totemable(mut o_ptr: *mut object_type)
 -> bool_ {
    /* Only full corpse */
    if (*o_ptr).tval as libc::c_int == 9 as libc::c_int &&
           ((*o_ptr).sval as libc::c_int == 1 as libc::c_int ||
                (*o_ptr).sval as libc::c_int == 2 as libc::c_int) {
        return 1 as libc::c_int as bool_
    }
    /* Assume not */
    return 0 as libc::c_int as bool_;
}
/*
 * Summoners
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_summoner_extract() {
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
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut item: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut partial: bool_ = 0;
    /* Not when confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Require lite */
    if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0 {
        msg_print(b"You cannot see!\x00" as *const u8 as *const libc::c_char);
        return
    }
    item_tester_hook =
        Some(item_tester_hook_totemable as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q = b"Use which corpse? \x00" as *const u8 as *const libc::c_char;
    s = b"You have no corpse to use.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    if (*r_info.offset((*o_ptr).pval2 as isize)).flags1 &
           0x1 as libc::c_int as libc::c_uint != 0 {
        partial = 0 as libc::c_int as bool_
    } else {
        partial =
            get_check(b"Do you want to create a partial totem?\x00" as
                          *const u8 as *const libc::c_char)
    }
    r = (*o_ptr).pval2 as libc::c_int;
    inc_stack_size(item, -(1 as libc::c_int));
    if Rand_div(100 as libc::c_int) <
           (*r_info.offset((*o_ptr).pval2 as isize)).level as libc::c_int -
               get_skill(44 as libc::c_int) as libc::c_int {
        msg_print(b"You failed to extract a totem.\x00" as *const u8 as
                      *const libc::c_char);
        energy_use += 100 as libc::c_int;
        return
    }
    /* Prepare for object creation */
    q_ptr = &mut forge;
    /* Create the object */
    object_prep(q_ptr,
                lookup_kind(54 as libc::c_int,
                            if partial as libc::c_int != 0 {
                                1 as libc::c_int
                            } else { 2 as libc::c_int }) as libc::c_int);
    (*q_ptr).pval = r;
    (*q_ptr).pval2 = 0 as libc::c_int as s16b;
    (*q_ptr).number = 1 as libc::c_int as byte_hack;
    (*q_ptr).found = 9 as libc::c_int as byte_hack;
    object_aware(q_ptr);
    object_known(q_ptr);
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int) as byte_hack;
    inven_carry(q_ptr, 0 as libc::c_int as bool_);
    msg_print(b"You extract a totem from the dead corpse.\x00" as *const u8 as
                  *const libc::c_char);
    energy_use += 100 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn summon_true(mut r_idx: libc::c_int,
                                     mut item: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut x: libc::c_int = 1 as libc::c_int;
    let mut y: libc::c_int = 1 as libc::c_int;
    let mut rx: libc::c_int = 0;
    let mut ry: libc::c_int = 0 as libc::c_int;
    let mut chance: libc::c_int = 0;
    let mut used: bool_ = 0;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Uniques are less likely to be nice */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        /* Because it's unique, it will always be destroyed */
        used = 1 as libc::c_int as bool_;
        /* About twice as hard as non-uniques */
        chance =
            get_skill(44 as libc::c_int) as libc::c_int * 70 as libc::c_int /
                ((*r_ptr).level as libc::c_int + 1 as libc::c_int);
        if Rand_div(100 as libc::c_int) < chance {
            status = 3 as libc::c_int
        } else { status = -(2 as libc::c_int) }
    } else {
        /* Non-uniques are easier to handle */
        if get_skill(44 as libc::c_int) as libc::c_int == 0 as libc::c_int {
            used = 1 as libc::c_int as bool_
        } else {
            /* It can be used multiple times */
            used = 0 as libc::c_int as bool_;
            /* But it is not 100% sure (note: skill > 0) */
            chance =
                (*r_ptr).level as libc::c_int * 25 as libc::c_int /
                    get_skill(44 as libc::c_int) as libc::c_int;
            if Rand_div(100 as libc::c_int) < chance {
                used = 1 as libc::c_int as bool_
            }
        }
        chance =
            get_skill(44 as libc::c_int) as libc::c_int * 130 as libc::c_int /
                ((*r_ptr).level as libc::c_int + 1 as libc::c_int);
        if Rand_div(100 as libc::c_int) < chance {
            status = 3 as libc::c_int
        } else { status = -(2 as libc::c_int) }
    }
    /* Find a grid where the monster is summoned */
    i = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        rx =
            Rand_div(8 as libc::c_int) - 4 as libc::c_int +
                (*p_ptr).px as libc::c_int;
        ry =
            Rand_div(8 as libc::c_int) - 4 as libc::c_int +
                (*p_ptr).py as libc::c_int;
        if ry > 0 as libc::c_int && rx > 0 as libc::c_int &&
               ry < cur_hgt as libc::c_int - 1 as libc::c_int &&
               rx < cur_wid as libc::c_int - 1 as libc::c_int &&
               ((*f_info.offset((*cave[ry as usize].offset(rx as isize)).feat
                                    as isize)).flags1 as libc::c_long &
                    0x10 as libc::c_long != 0 &&
                    (*cave[ry as usize].offset(rx as isize)).feat as
                        libc::c_int != 0xaf as libc::c_int &&
                    (*cave[ry as usize].offset(rx as isize)).m_idx == 0 &&
                    !(ry == (*p_ptr).py as libc::c_int &&
                          rx == (*p_ptr).px as libc::c_int)) {
            x = rx;
            y = ry;
            break ;
        } else { i += 1 }
    }
    /* No room found */
    if i == 40 as libc::c_int {
        msg_print(b"The summoning fails due to lack of room.\x00" as *const u8
                      as *const libc::c_char);
        return
    }
    /* Summon the monster */
    bypass_r_ptr_max_num = 1 as libc::c_int as bool_;
    i =
        place_monster_one(y, x, r_idx, 0 as libc::c_int,
                          0 as libc::c_int as bool_, status) as libc::c_int;
    if i == 0 {
        msg_print(b"The summoning fails.\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        (*m_list.offset(i as isize)).status = status as s16b;
        let ref mut fresh26 = (*m_list.offset(i as isize)).mflag;
        *fresh26 |= 0x100 as libc::c_int
    }
    bypass_r_ptr_max_num = 0 as libc::c_int as bool_;
    /* Destroy the totem if the used flag is set */
    if used != 0 {
        /* Eliminate the totem */
        inc_stack_size(item, -(1 as libc::c_int));
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_summoner_summon() {
    let mut item: libc::c_int = 0;
    let mut x: libc::c_int = 1 as libc::c_int;
    let mut y: libc::c_int = 1 as libc::c_int;
    let mut rx: libc::c_int = 0;
    let mut ry: libc::c_int = 0;
    let mut m_idx: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    /* Which Totem? */
    item_tester_tval = 54 as libc::c_int as byte_hack;
    q = b"Summon from which Totem?\x00" as *const u8 as *const libc::c_char;
    s =
        b"There are no totems to summon from!\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Access the item */
    o_ptr = get_object(item);
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* True Totems have their own function. */
    if (*o_ptr).sval as libc::c_int == 2 as libc::c_int {
        summon_true((*o_ptr).pval, item);
        return
    }
    /* Handle partial totems */
    /* Find a grid where the monster is summoned */
    i = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        rx =
            Rand_div(8 as libc::c_int) - 4 as libc::c_int +
                (*p_ptr).px as libc::c_int;
        ry =
            Rand_div(8 as libc::c_int) - 4 as libc::c_int +
                (*p_ptr).py as libc::c_int;
        if ry > 0 as libc::c_int && rx > 0 as libc::c_int &&
               ry < cur_hgt as libc::c_int - 1 as libc::c_int &&
               rx < cur_wid as libc::c_int - 1 as libc::c_int &&
               ((*f_info.offset((*cave[ry as usize].offset(rx as isize)).feat
                                    as isize)).flags1 as libc::c_long &
                    0x10 as libc::c_long != 0 &&
                    (*cave[ry as usize].offset(rx as isize)).feat as
                        libc::c_int != 0xaf as libc::c_int &&
                    (*cave[ry as usize].offset(rx as isize)).m_idx == 0 &&
                    !(ry == (*p_ptr).py as libc::c_int &&
                          rx == (*p_ptr).px as libc::c_int)) {
            x = rx;
            y = ry;
            break ;
        } else { i += 1 }
    }
    /* No room found */
    if i == 40 as libc::c_int {
        msg_print(b"The summoning fails due to lack of room.\x00" as *const u8
                      as *const libc::c_char);
        return
    }
    /* Summon the monster */
    bypass_r_ptr_max_num = 1 as libc::c_int as bool_;
    place_monster_one_no_drop = 1 as libc::c_int as bool_;
    m_idx =
        place_monster_one(y, x, (*o_ptr).pval, 0 as libc::c_int,
                          0 as libc::c_int as bool_, 3 as libc::c_int) as
            libc::c_int;
    bypass_r_ptr_max_num = 0 as libc::c_int as bool_;
    /* Failure. */
    if m_idx == 0 {
        msg_print(b"The summoning fails.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Mark the monster as a "partial" ally */
    m_ptr = &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    (*m_ptr).mflag |= 0x4 as libc::c_int | 0x100 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_summoner() {
    let mut ext: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_char = 0;
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).anti_magic != 0 {
        msg_print(b"Your anti-magic shell disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* not if confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* not if blind */
    if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0 {
        msg_print(b"You cannot see!\x00" as *const u8 as *const libc::c_char);
        return
    }
    loop 
         /* Select what to do */
         {
        if get_com(b"[E]xtract a totem, [S]ummon\x00" as *const u8 as
                       *const libc::c_char, &mut ch) == 0 {
            ext = 0 as libc::c_int;
            break ;
        } else if ch as libc::c_int == 'E' as i32 ||
                      ch as libc::c_int == 'e' as i32 {
            ext = 1 as libc::c_int;
            break ;
        } else {
            if !(ch as libc::c_int == 's' as i32 ||
                     ch as libc::c_int == 'S' as i32) {
                continue ;
            }
            ext = 2 as libc::c_int;
            break ;
        }
    }
    match ext {
        1 => { do_cmd_summoner_extract(); }
        2 => { do_cmd_summoner_summon(); }
        _ => { }
    };
}
/*
 * Fighters may invoke The Rush.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_blade() {
    /* Are we already Rushed? */
    if (*p_ptr).rush != 0 {
        msg_format(b"You have %d turns of The Rush remaining\x00" as *const u8
                       as *const libc::c_char, (*p_ptr).rush as libc::c_int);
        return
    }
    /* Are you sure? */
    if get_check(b"Are you sure you want to invoke The Rush?\x00" as *const u8
                     as *const libc::c_char) == 0 {
        return
    }
    /* Let's Rush! */
    set_rush(2 as libc::c_int + (*p_ptr).lev as libc::c_int / 2 as libc::c_int
                 +
                 (Rand_div((*p_ptr).lev as libc::c_int / 2 as libc::c_int) +
                      1 as libc::c_int));
}
/*
 * Dodge Chance Feedback.
 */
#[no_mangle]
pub unsafe extern "C" fn use_ability_blade() {
    let mut chance: libc::c_int =
        (*p_ptr).dodge_chance as libc::c_int -
            dun_level as libc::c_int * 5 as libc::c_int / 6 as libc::c_int;
    if chance < 0 as libc::c_int { chance = 0 as libc::c_int }
    if wizard != 0 {
        msg_format(b"You have exactly %d chances of dodging a level %d monster.\x00"
                       as *const u8 as *const libc::c_char, chance,
                   dun_level as libc::c_int);
    }
    if chance < 5 as libc::c_int {
        msg_format(b"You have almost no chance of dodging a level %d monster.\x00"
                       as *const u8 as *const libc::c_char,
                   dun_level as libc::c_int);
    } else if chance < 10 as libc::c_int {
        msg_format(b"You have a slight chance of dodging a level %d monster.\x00"
                       as *const u8 as *const libc::c_char,
                   dun_level as libc::c_int);
    } else if chance < 20 as libc::c_int {
        msg_format(b"You have a significant chance of dodging a level %d monster.\x00"
                       as *const u8 as *const libc::c_char,
                   dun_level as libc::c_int);
    } else if chance < 40 as libc::c_int {
        msg_format(b"You have a large chance of dodging a level %d monster.\x00"
                       as *const u8 as *const libc::c_char,
                   dun_level as libc::c_int);
    } else if chance < 70 as libc::c_int {
        msg_format(b"You have a high chance of dodging a level %d monster.\x00"
                       as *const u8 as *const libc::c_char,
                   dun_level as libc::c_int);
    } else {
        msg_format(b"You will usually dodge successfully a level %d monster.\x00"
                       as *const u8 as *const libc::c_char,
                   dun_level as libc::c_int);
    };
}
/*
 * Helper function to describe symbiotic powers
 */
#[no_mangle]
pub unsafe extern "C" fn symbiotic_info(mut p: *mut libc::c_char,
                                        mut power: libc::c_int) {
    let mut plev: libc::c_int = get_skill(8 as libc::c_int) as libc::c_int;
    strcpy(p, b"\x00" as *const u8 as *const libc::c_char);
    match power {
        2 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" power %d\x00" as *const u8 as *const libc::c_char,
                    plev * 3 as libc::c_int);
        }
        5 => {
            strnfmt(p, 80 as libc::c_int as uint_hack,
                    b" heal %d%%\x00" as *const u8 as *const libc::c_char,
                    15 as libc::c_int +
                        get_skill_scale(8 as libc::c_int,
                                        35 as libc::c_int as u32b) as
                            libc::c_int);
        }
        _ => { }
    };
}
/*
 * Cast a symbiotic spell
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_symbiotic() {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut chance: libc::c_int = 0;
    let mut minfail: libc::c_int = 0 as libc::c_int;
    let mut plev: libc::c_int = get_skill(8 as libc::c_int) as libc::c_int;
    let mut spell: magic_power =
        magic_power{min_lev: 0,
                    mana_cost: 0,
                    fail: 0,
                    name: 0 as *const libc::c_char,
                    desc: 0 as *const libc::c_char,};
    /* Get the carried monster */
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).anti_magic != 0 {
        msg_print(b"Your anti-magic shell disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* not if confused */
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* get power */
    if get_magic_power(&mut n, symbiotic_powers.as_mut_ptr(),
                       9 as libc::c_int,
                       Some(symbiotic_info as
                                unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: libc::c_int) -> ()),
                       get_skill(8 as libc::c_int) as libc::c_int,
                       1 as libc::c_int) == 0 {
        return
    }
    spell = symbiotic_powers[n as usize];
    /* Verify "dangerous" spells */
    if spell.mana_cost > (*p_ptr).csp as libc::c_int {
        /* Warning */
        msg_print(b"You do not have enough mana to use this power.\x00" as
                      *const u8 as *const libc::c_char);
        /* Verify */
        if get_check(b"Attempt it anyway? \x00" as *const u8 as
                         *const libc::c_char) == 0 {
            return
        }
    }
    /* Spell failure chance */
    chance = spell.fail;
    /* Reduce failure rate by "effective" level adjustment */
    chance -= 3 as libc::c_int * (plev - spell.min_lev);
    /* Reduce failure rate by INT/WIS adjustment */
    chance -=
        3 as libc::c_int *
            (*adj_mag_stat.as_mut_ptr().offset((*p_ptr).stat_ind[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                   as isize) as libc::c_int -
                 1 as libc::c_int);
    /* Not enough mana to cast */
    if spell.mana_cost > (*p_ptr).csp as libc::c_int {
        chance +=
            5 as libc::c_int * (spell.mana_cost - (*p_ptr).csp as libc::c_int)
    }
    /* Extract the minimum failure rate */
    minfail =
        *adj_mag_fail.as_mut_ptr().offset((*p_ptr).stat_ind[1 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int;
    /* Failure rate */
    chance = clamp_failure_chance(chance, minfail);
    /* Failed spell */
    if Rand_div(100 as libc::c_int) < chance {
        if flush_failure != 0 { flush(); }
        msg_format(b"You failed to concentrate hard enough!\x00" as *const u8
                       as *const libc::c_char);
        sound(55 as libc::c_int);
    } else {
        sound(12 as libc::c_int);
        /* spell code */
        match n {
            0 => {
                let mut dir: libc::c_int = 0;
                let mut x: libc::c_int = 0;
                let mut y: libc::c_int = 0;
                let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
                let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
                let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
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
                msg_print(b"Hypnotise which pet?\x00" as *const u8 as
                              *const libc::c_char);
                if get_rep_dir(&mut dir) == 0 { return }
                y =
                    (*p_ptr).py as libc::c_int +
                        ddy[dir as usize] as libc::c_int;
                x =
                    (*p_ptr).px as libc::c_int +
                        ddx[dir as usize] as libc::c_int;
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(y as
                                                         isize)).offset(x as
                                                                            isize)
                        as *mut cave_type;
                if (*c_ptr).m_idx != 0 {
                    m_ptr =
                        &mut *m_list.offset((*c_ptr).m_idx as isize) as
                            *mut monster_type;
                    r_ptr =
                        if !(*m_ptr).sr_ptr.is_null() {
                            (*m_ptr).sr_ptr
                        } else {
                            race_info_idx((*m_ptr).r_idx as libc::c_int,
                                          (*m_ptr).ego as libc::c_int)
                        };
                    if (*r_ptr).flags1 &
                           0x20000 as libc::c_int as libc::c_uint == 0 {
                        msg_print(b"You can only hypnotise monsters that cannot move.\x00"
                                      as *const u8 as *const libc::c_char);
                    } else if ((*m_ptr).status as libc::c_int) <
                                  3 as libc::c_int {
                        msg_print(b"You can only hypnotise pets and companions.\x00"
                                      as *const u8 as *const libc::c_char);
                    } else if (*r_ptr).flags9 &
                                  0x2000 as libc::c_int as libc::c_uint != 0 {
                        msg_print(b"You cannot hypnotise this monster.\x00" as
                                      *const u8 as *const libc::c_char);
                    } else {
                        /* TODO fix this hack hack hack hackity hack with ToME 3 flags */
                        q_ptr = &mut forge;
                        object_prep(q_ptr,
                                    lookup_kind(99 as libc::c_int,
                                                1 as libc::c_int) as
                                        libc::c_int);
                        (*q_ptr).number = 1 as libc::c_int as byte_hack;
                        (*q_ptr).pval = (*m_ptr).r_idx as s32b;
                        (*q_ptr).pval2 = (*m_ptr).hp as s16b;
                        (*q_ptr).pval3 = (*m_ptr).maxhp;
                        /* overflow alert */
                        (*q_ptr).exp = (*m_ptr).exp as s32b;
                        (*q_ptr).elevel = (*m_ptr).level;
                        object_aware(q_ptr);
                        object_known(q_ptr);
                        (*q_ptr).ident =
                            ((*q_ptr).ident as libc::c_int |
                                 0x10 as libc::c_int) as byte_hack;
                        drop_near(q_ptr, 0 as libc::c_int, y, x);
                        delete_monster(y, x);
                        health_who = 0 as libc::c_int as s16b
                    }
                } else {
                    msg_print(b"There is no pet here !\x00" as *const u8 as
                                  *const libc::c_char);
                }
            }
            1 => {
                let mut m_ptr_0: *mut monster_type = 0 as *mut monster_type;
                let mut m_idx: libc::c_int = 0;
                let mut item: libc::c_int = 0;
                let mut x_0: libc::c_int = 0;
                let mut y_0: libc::c_int = 0;
                let mut d: libc::c_int = 0;
                let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
                let mut q: cptr = 0 as *const libc::c_char;
                let mut s: cptr = 0 as *const libc::c_char;
                /* Restrict choices to monsters */
                item_tester_tval = 99 as libc::c_int as byte_hack;
                /* Get an item */
                q =
                    b"Awaken which monster? \x00" as *const u8 as
                        *const libc::c_char;
                s =
                    b"You have no monster to awaken.\x00" as *const u8 as
                        *const libc::c_char;
                if get_item(&mut item, q, s, 0x4 as libc::c_int) == 0 {
                    return
                }
                o_ptr_0 =
                    &mut *o_list.offset((0 as libc::c_int - item) as isize) as
                        *mut object_type;
                d = 2 as libc::c_int;
                while d < 100 as libc::c_int {
                    scatter(&mut y_0, &mut x_0, (*p_ptr).py as libc::c_int,
                            (*p_ptr).px as libc::c_int, d);
                    if (*f_info.offset((*cave[y_0 as
                                                  usize].offset(x_0 as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x10 as libc::c_long != 0 &&
                           (*cave[y_0 as usize].offset(x_0 as isize)).feat as
                               libc::c_int != 0xaf as libc::c_int &&
                           (*cave[y_0 as usize].offset(x_0 as isize)).m_idx ==
                               0 {
                        break ;
                    }
                    d += 1
                }
                if d >= 100 as libc::c_int { return }
                m_idx =
                    place_monster_one(y_0, x_0, (*o_ptr_0).pval,
                                      0 as libc::c_int,
                                      0 as libc::c_int as bool_,
                                      3 as libc::c_int) as libc::c_int;
                if m_idx == 0 as libc::c_int { return }
                /* TODO fix this hack hack hack hackity hack with ToME 3 flags */
				/* Have to be careful here; releasing the symbiote into a
                 * dungeon with leveled monsters will level the symbiote
                 * before we can get hold of it. We'll be nice and use the
                 * larger of the saved exp and the exp that the newly-generated
                 * monster starts with. */
                m_ptr_0 =
                    &mut *m_list.offset(m_idx as isize) as *mut monster_type;
                if (*m_ptr_0).exp < (*o_ptr_0).exp as libc::c_uint {
                    (*m_ptr_0).exp = (*o_ptr_0).exp as u32b;
                    monster_check_experience(m_idx,
                                             1 as libc::c_int as bool_);
                    if (*m_ptr_0).level as libc::c_int !=
                           (*o_ptr_0).elevel as libc::c_int {
                        cmsg_format(10 as libc::c_int as byte_hack,
                                    b"ERROR: level-%d HYPNOS becomes level-%d symbiote\x00"
                                        as *const u8 as *const libc::c_char,
                                    (*o_ptr_0).elevel as libc::c_int,
                                    (*m_ptr_0).level as libc::c_int);
                    }
                }
                (*m_ptr_0).hp = (*o_ptr_0).pval2 as s32b;
                (*m_ptr_0).maxhp = (*o_ptr_0).pval3;
                floor_item_increase(0 as libc::c_int - item,
                                    -(1 as libc::c_int));
                floor_item_describe(0 as libc::c_int - item);
                floor_item_optimize(0 as libc::c_int - item);
            }
            2 => {
                /* Charm */
                let mut dir_0: libc::c_int = 0;
                if get_aim_dir(&mut dir_0) == 0 { return }
                fire_bolt(110 as libc::c_int, dir_0, plev * 3 as libc::c_int);
            }
            3 => {
                /* Life Share */
                let mut percent1: s32b = 0;
                let mut percent2: s32b = 0;
                if (*o_ptr).k_idx == 0 {
                    msg_print(b"You are not in symbiosis.\x00" as *const u8 as
                                  *const libc::c_char);
                } else {
                    percent1 = (*p_ptr).chp as s32b;
                    percent1 =
                        percent1 * 100 as libc::c_int /
                            (*p_ptr).mhp as libc::c_int;
                    percent2 = (*o_ptr).pval2 as s32b;
                    percent2 = percent2 * 100 as libc::c_int / (*o_ptr).pval3;
                    /* Now get the average */
                    percent1 = (percent1 + percent2) / 2 as libc::c_int;
                    /* And set the hp of monster & player to it */
                    (*p_ptr).chp =
                        (percent1 * (*p_ptr).mhp as libc::c_int /
                             100 as libc::c_int) as s16b;
                    (*o_ptr).pval2 =
                        (percent1 * (*o_ptr).pval3 / 100 as libc::c_int) as
                            s16b;
                    /* Redraw */
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x40 as libc::c_long) as u32b;
                    /* Window stuff */
                    (*p_ptr).window =
                        ((*p_ptr).window as libc::c_long |
                             0x8 as libc::c_long) as u32b;
                    /* Display the monster hitpoints */
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x10000000 as libc::c_long) as u32b
                }
            }
            4 => {
                /* Minor Symbiotic Powers */
                if (*o_ptr).k_idx == 0 {
                    msg_print(b"You are not in symbiosis.\x00" as *const u8 as
                                  *const libc::c_char);
                } else if 0 as libc::c_int >
                              use_symbiotic_power((*o_ptr).pval,
                                                  0 as libc::c_int as bool_,
                                                  0 as libc::c_int as bool_,
                                                  1 as libc::c_int as bool_) {
                    return
                }
            }
            5 => {
                /* Heal Symbiote */
                let mut hp: libc::c_int = 0;
                if (*o_ptr).k_idx == 0 {
                    msg_print(b"You are not in symbiosis.\x00" as *const u8 as
                                  *const libc::c_char);
                } else {
                    hp =
                        (*o_ptr).pval3 *
                            (15 as libc::c_int +
                                 get_skill_scale(8 as libc::c_int,
                                                 35 as libc::c_int as u32b) as
                                     libc::c_int) / 100 as libc::c_int;
                    (*o_ptr).pval2 =
                        ((*o_ptr).pval2 as libc::c_int + hp) as s16b;
                    if (*o_ptr).pval2 as libc::c_int > (*o_ptr).pval3 {
                        (*o_ptr).pval2 = (*o_ptr).pval3 as s16b
                    }
                    msg_format(b"%s is healed.\x00" as *const u8 as
                                   *const libc::c_char,
                               symbiote_name(1 as libc::c_int as bool_));
                    /* Display the monster hitpoints */
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x10000000 as libc::c_long) as u32b
                }
            }
            6 => {
                /* Major Symbiotic Powers */
                if (*o_ptr).k_idx == 0 {
                    msg_print(b"You are not in symbiosis.\x00" as *const u8 as
                                  *const libc::c_char);
                } else if 0 as libc::c_int >
                              use_symbiotic_power((*o_ptr).pval,
                                                  1 as libc::c_int as bool_,
                                                  0 as libc::c_int as bool_,
                                                  1 as libc::c_int as bool_) {
                    return
                }
            }
            7 => {
                /* Summon never-moving pet */
                summon_specific_friendly((*p_ptr).py as libc::c_int,
                                         (*p_ptr).px as libc::c_int,
                                         dun_level as libc::c_int,
                                         53 as libc::c_int,
                                         0 as libc::c_int as bool_);
            }
            8 => {
                /* Force Symbiosis */
                let mut y_1: libc::c_int = 0;
                let mut x_1: libc::c_int = 0;
                let mut c_ptr_0: *mut cave_type = 0 as *mut cave_type;
                let mut m_ptr_1: *mut monster_type = 0 as *mut monster_type;
                if tgt_pt(&mut x_1, &mut y_1) == 0 { return }
                c_ptr_0 =
                    &mut *(*cave.as_mut_ptr().offset(y_1 as
                                                         isize)).offset(x_1 as
                                                                            isize)
                        as *mut cave_type;
                if !((*c_ptr_0).m_idx == 0) {
                    m_ptr_1 =
                        &mut *m_list.offset((*c_ptr_0).m_idx as isize) as
                            *mut monster_type;
                    use_symbiotic_power((*m_ptr_1).r_idx as libc::c_int,
                                        1 as libc::c_int as bool_,
                                        0 as libc::c_int as bool_,
                                        1 as libc::c_int as bool_);
                }
            }
            _ => {
                msg_print(b"Zap?\x00" as *const u8 as *const libc::c_char);
            }
        }
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Sufficient mana */
    if spell.mana_cost <= (*p_ptr).csp as libc::c_int {
        /* Use some mana */
        (*p_ptr).csp = ((*p_ptr).csp as libc::c_int - spell.mana_cost) as s16b
    } else {
        /* Over-exert the player */
        let mut oops: libc::c_int =
            spell.mana_cost - (*p_ptr).csp as libc::c_int;
        /* No mana left */
        (*p_ptr).csp = 0 as libc::c_int as s16b;
        (*p_ptr).csp_frac = 0 as libc::c_int as u16b;
        /* Message */
        msg_print(b"You faint from the effort!\x00" as *const u8 as
                      *const libc::c_char);
        /* Hack -- Bypass free action */
        set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                          (Rand_div(5 as libc::c_int * oops +
                                        1 as libc::c_int) +
                               1 as libc::c_int));
        /* Damage CON (possibly permanently) */
        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
            let mut perm: bool_ =
                (Rand_div(100 as libc::c_int) < 25 as libc::c_int) as
                    libc::c_int as bool_;
            /* Message */
            msg_print(b"You have damaged your body!\x00" as *const u8 as
                          *const libc::c_char);
            /* Reduce constitution */
            dec_stat(5 as libc::c_int,
                     15 as libc::c_int +
                         (Rand_div(10 as libc::c_int) + 1 as libc::c_int),
                     perm as libc::c_int);
        }
    }
    /* Redraw mana */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
}
/*
 * Boulder creation .. sorry :)
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_create_boulder() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    if get_rep_dir(&mut dir) == 0 { return }
    y = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
    x = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Granite -- How about other wall types? */
    if (*c_ptr).feat as libc::c_int >= 0x38 as libc::c_int &&
           (*c_ptr).feat as libc::c_int <= 0x3b as libc::c_int ||
           (*c_ptr).feat as libc::c_int >= 0x34 as libc::c_int &&
               (*c_ptr).feat as libc::c_int <= 0x37 as libc::c_int ||
           ((*c_ptr).feat as libc::c_int == 0x32 as libc::c_int ||
                (*c_ptr).feat as libc::c_int == 0x33 as libc::c_int) {
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
        wall_to_mud(dir);
        /* Get local object */
        q_ptr = &mut forge;
        /* Hack -- Give the player some shots */
        object_prep(q_ptr,
                    lookup_kind(11 as libc::c_int, 1 as libc::c_int) as
                        libc::c_int);
        (*q_ptr).number =
            (2 as libc::c_int +
                 Rand_div(1 as libc::c_int + 5 as libc::c_int -
                              2 as libc::c_int)) as byte_hack;
        object_aware(q_ptr);
        object_known(q_ptr);
        (*q_ptr).ident =
            ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int) as
                byte_hack;
        (*q_ptr).discount = 90 as libc::c_int as byte_hack;
        (*q_ptr).found = 9 as libc::c_int as byte_hack;
        inven_carry(q_ptr, 0 as libc::c_int as bool_);
        msg_print(b"You make some boulders.\x00" as *const u8 as
                      *const libc::c_char);
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                      0x200000 as libc::c_long)) as u32b;
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Take a turn */
        energy_use = 100 as libc::c_int
    };
}
/*
 * Clamp failure chance
 */
#[no_mangle]
pub unsafe extern "C" fn clamp_failure_chance(mut chance: libc::c_int,
                                              mut minfail: libc::c_int)
 -> libc::c_int {
    if minfail < 0 as libc::c_int { minfail = 0 as libc::c_int }
    /* Minimum failure rate */
    if chance < minfail { chance = minfail }
    /* Stunning makes spells harder */
    if (*p_ptr).stun as libc::c_int > 50 as libc::c_int {
        chance += 25 as libc::c_int
    } else if (*p_ptr).stun != 0 { chance += 15 as libc::c_int }
    /* Always a 5 percent chance of working */
    if chance > 95 as libc::c_int { chance = 95 as libc::c_int }
    return chance;
}
