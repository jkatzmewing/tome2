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
    static mut inscription_info: [inscription_info_type; 8];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut death: bool_;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut repair_monsters: bool_;
    #[no_mangle]
    static mut o_max: s16b;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut artifact_bias: libc::c_int;
    #[no_mangle]
    static mut flush_failure: bool_;
    #[no_mangle]
    static mut auto_destroy: bool_;
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
    static mut monster_race_idx: s16b;
    #[no_mangle]
    static mut died_from: [libc::c_char; 80];
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
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut d_name: *mut libc::c_char;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut max_d_idx: u16b;
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut take_notes: bool_;
    #[no_mangle]
    static mut auto_notes: bool_;
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut powers_type: *mut power_type;
    #[no_mangle]
    static mut power_max: s16b;
    #[no_mangle]
    static mut project_time: libc::c_int;
    #[no_mangle]
    static mut project_time_effect: s32b;
    #[no_mangle]
    static mut DUNGEON_DEATH: s32b;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cave_valid_bold(y: libc::c_int, x: libc::c_int) -> bool_;
    #[no_mangle]
    fn move_cursor_relative(row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn wiz_dark();
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn place_floor_convert_glass(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn scatter(yp: *mut libc::c_int, xp: *mut libc::c_int, y: libc::c_int,
               x: libc::c_int, d: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execute_inscription(i: byte_hack, y: byte_hack, x: byte_hack) -> bool_;
    #[no_mangle]
    fn alchemist_learn_object(o_ptr: *mut object_type) -> libc::c_int;
    #[no_mangle]
    fn describe_player_location() -> cptr;
    #[no_mangle]
    fn autosave_checkpoint();
    #[no_mangle]
    fn curse_equipment(chance: libc::c_int, heavy_chance: libc::c_int);
    #[no_mangle]
    fn curse_equipment_dg(chance: libc::c_int, heavy_chance: libc::c_int);
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
    fn monster_race_desc(desc: *mut libc::c_char, r_idx: libc::c_int,
                         ego: libc::c_int);
    #[no_mangle]
    fn lore_do_probe(m_idx: libc::c_int);
    #[no_mangle]
    fn update_mon(m_idx: libc::c_int, full: bool_);
    #[no_mangle]
    fn place_monster_aux(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         slp: bool_, grp: bool_, status: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn summon_specific(y1: libc::c_int, x1: libc::c_int, lev: libc::c_int,
                       type_0: libc::c_int) -> bool_;
    #[no_mangle]
    fn change_side(m_ptr: *mut monster_type) -> bool_;
    #[no_mangle]
    fn is_friend(m_ptr: *mut monster_type) -> libc::c_int;
    #[no_mangle]
    fn inc_stack_size(item: libc::c_int, delta: libc::c_int);
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn object_out_desc(o_ptr: *mut object_type, fff: *mut FILE,
                       trim_down: bool_, wait_for_it: bool_) -> bool_;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn Rand_mod(m: s32b) -> s32b;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn index_to_label(i: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn describe_use(i: libc::c_int) -> cptr;
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn delete_object(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_value(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn object_value_real(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn pick_trap(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn get_pos_player(dis: libc::c_int, ny: *mut libc::c_int,
                      nx: *mut libc::c_int);
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn inc_stat(stat: libc::c_int) -> bool_;
    #[no_mangle]
    fn dec_stat(stat: libc::c_int, amount: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn res_stat(stat: libc::c_int, full: bool_) -> bool_;
    #[no_mangle]
    fn project_m(who: libc::c_int, r: libc::c_int, y: libc::c_int,
                 x: libc::c_int, dam: libc::c_int, typ: libc::c_int) -> bool_;
    #[no_mangle]
    fn project(who: libc::c_int, rad: libc::c_int, y: libc::c_int,
               x: libc::c_int, dam: libc::c_int, typ: libc::c_int,
               flg: libc::c_int) -> bool_;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn check_experience();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn luck(min: libc::c_int, max: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn add_note(note: *mut libc::c_char, code: libc::c_char);
    #[no_mangle]
    fn has_ability(ab: libc::c_int) -> bool_;
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn tgt_pt(x: *mut libc::c_int, y: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn set_blind(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn verify_panel();
    #[no_mangle]
    fn set_stun(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn update_stuff();
    #[no_mangle]
    fn target_okay() -> bool_;
    #[no_mangle]
    fn get_aim_dir(dp: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn get_quantity(prompt: cptr, max: s32b) -> s32b;
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn lose_exp(amount: s32b);
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn strlower(buf: *mut libc::c_char);
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn c_prt(attr: byte_hack, str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
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
pub struct power_type {
    pub name: *mut libc::c_char,
    pub desc_text: *mut libc::c_char,
    pub gain_text: *mut libc::c_char,
    pub lose_text: *mut libc::c_char,
    pub level: byte_hack,
    pub cost: byte_hack,
    pub stat: byte_hack,
    pub diff: byte_hack,
}
/*
 * Grow things
 */
#[no_mangle]
pub unsafe extern "C" fn grow_things(mut type_0: s16b, mut rad: libc::c_int) {
    let mut a: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    a = 0 as libc::c_int;
    while a < rad * rad + 11 as libc::c_int {
        i =
            (Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad +
                 Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad) /
                2 as libc::c_int;
        j =
            (Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad +
                 Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad) /
                2 as libc::c_int;
        if (*p_ptr).py as libc::c_int + j > 0 as libc::c_int &&
               (*p_ptr).px as libc::c_int + i > 0 as libc::c_int &&
               (*p_ptr).py as libc::c_int + j <
                   cur_hgt as libc::c_int - 1 as libc::c_int &&
               (*p_ptr).px as libc::c_int + i <
                   cur_wid as libc::c_int - 1 as libc::c_int {
            if !(distance((*p_ptr).py as libc::c_int,
                          (*p_ptr).px as libc::c_int,
                          (*p_ptr).py as libc::c_int + j,
                          (*p_ptr).px as libc::c_int + i) > rad) {
                if (*f_info.offset((*cave[((*p_ptr).py as libc::c_int + j) as
                                              usize].offset(((*p_ptr).px as
                                                                 libc::c_int +
                                                                 i) as
                                                                isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[((*p_ptr).py as libc::c_int + j) as
                                  usize].offset(((*p_ptr).px as libc::c_int +
                                                     i) as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int &&
                       (*cave[((*p_ptr).py as libc::c_int + j) as
                                  usize].offset(((*p_ptr).px as libc::c_int +
                                                     i) as isize)).o_idx as
                           libc::c_int == 0 as libc::c_int &&
                       (*f_info.offset((*cave[((*p_ptr).py as libc::c_int + j)
                                                  as
                                                  usize].offset(((*p_ptr).px
                                                                     as
                                                                     libc::c_int
                                                                     + i) as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x40 as libc::c_long == 0 {
                    cave_set_feat((*p_ptr).py as libc::c_int + j,
                                  (*p_ptr).px as libc::c_int + i,
                                  type_0 as libc::c_int);
                }
            }
        }
        a += 1
    };
}
/*
 * Grow trees
 */
#[no_mangle]
pub unsafe extern "C" fn grow_trees(mut rad: libc::c_int) {
    let mut a: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    a = 0 as libc::c_int;
    while a < rad * rad + 11 as libc::c_int {
        i =
            (Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad +
                 Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad) /
                2 as libc::c_int;
        j =
            (Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad +
                 Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad) /
                2 as libc::c_int;
        if (*p_ptr).py as libc::c_int + j > 0 as libc::c_int &&
               (*p_ptr).px as libc::c_int + i > 0 as libc::c_int &&
               (*p_ptr).py as libc::c_int + j <
                   cur_hgt as libc::c_int - 1 as libc::c_int &&
               (*p_ptr).px as libc::c_int + i <
                   cur_wid as libc::c_int - 1 as libc::c_int {
            if !(distance((*p_ptr).py as libc::c_int,
                          (*p_ptr).px as libc::c_int,
                          (*p_ptr).py as libc::c_int + j,
                          (*p_ptr).px as libc::c_int + i) > rad) {
                if (*f_info.offset((*cave[((*p_ptr).py as libc::c_int + j) as
                                              usize].offset(((*p_ptr).px as
                                                                 libc::c_int +
                                                                 i) as
                                                                isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[((*p_ptr).py as libc::c_int + j) as
                                  usize].offset(((*p_ptr).px as libc::c_int +
                                                     i) as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int &&
                       (*cave[((*p_ptr).py as libc::c_int + j) as
                                  usize].offset(((*p_ptr).px as libc::c_int +
                                                     i) as isize)).o_idx as
                           libc::c_int == 0 as libc::c_int &&
                       (*f_info.offset((*cave[((*p_ptr).py as libc::c_int + j)
                                                  as
                                                  usize].offset(((*p_ptr).px
                                                                     as
                                                                     libc::c_int
                                                                     + i) as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x40 as libc::c_long == 0 &&
                       (*f_info.offset((*cave[(*p_ptr).py as
                                                  usize].offset((*p_ptr).px as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x40000 as libc::c_long != 0 {
                    cave_set_feat((*p_ptr).py as libc::c_int + j,
                                  (*p_ptr).px as libc::c_int + i,
                                  0x60 as libc::c_int);
                }
            }
        }
        a += 1
    };
}
/*
 * Grow grass
 */
#[no_mangle]
pub unsafe extern "C" fn grow_grass(mut rad: libc::c_int) {
    let mut a: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    a = 0 as libc::c_int;
    while a < rad * rad + 11 as libc::c_int {
        i =
            (Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad +
                 Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad) /
                2 as libc::c_int;
        j =
            (Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad +
                 Rand_mod(rad * 2 as libc::c_int + 1 as libc::c_int) - rad) /
                2 as libc::c_int;
        if (*p_ptr).py as libc::c_int + j > 0 as libc::c_int &&
               (*p_ptr).px as libc::c_int + i > 0 as libc::c_int &&
               (*p_ptr).py as libc::c_int + j <
                   cur_hgt as libc::c_int - 1 as libc::c_int &&
               (*p_ptr).px as libc::c_int + i <
                   cur_wid as libc::c_int - 1 as libc::c_int {
            if !(distance((*p_ptr).py as libc::c_int,
                          (*p_ptr).px as libc::c_int,
                          (*p_ptr).py as libc::c_int + j,
                          (*p_ptr).px as libc::c_int + i) > rad) {
                if (*f_info.offset((*cave[((*p_ptr).py as libc::c_int + j) as
                                              usize].offset(((*p_ptr).px as
                                                                 libc::c_int +
                                                                 i) as
                                                                isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[((*p_ptr).py as libc::c_int + j) as
                                  usize].offset(((*p_ptr).px as libc::c_int +
                                                     i) as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int &&
                       (*cave[((*p_ptr).py as libc::c_int + j) as
                                  usize].offset(((*p_ptr).px as libc::c_int +
                                                     i) as isize)).o_idx as
                           libc::c_int == 0 as libc::c_int &&
                       (*f_info.offset((*cave[((*p_ptr).py as libc::c_int + j)
                                                  as
                                                  usize].offset(((*p_ptr).px
                                                                     as
                                                                     libc::c_int
                                                                     + i) as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x40 as libc::c_long == 0 &&
                       (*f_info.offset((*cave[(*p_ptr).py as
                                                  usize].offset((*p_ptr).px as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x40000 as libc::c_long != 0 {
                    cave_set_feat((*p_ptr).py as libc::c_int + j,
                                  (*p_ptr).px as libc::c_int + i,
                                  0x59 as libc::c_int);
                }
            }
        }
        a += 1
    };
}
/*
 * Increase players hit points, notice effects
 */
#[no_mangle]
pub unsafe extern "C" fn hp_player(mut num: libc::c_int) -> bool_ {
    let mut dead: bool_ =
        (((*p_ptr).chp as libc::c_int) < 0 as libc::c_int) as libc::c_int as
            bool_;
    /* Healing needed */
    if ((*p_ptr).chp as libc::c_int) < (*p_ptr).mhp as libc::c_int {
        /* Gain hitpoints */
        (*p_ptr).chp = ((*p_ptr).chp as libc::c_int + num) as s16b;
        /* Enforce maximum */
        if (*p_ptr).chp as libc::c_int >= (*p_ptr).mhp as libc::c_int ||
               dead == 0 && ((*p_ptr).chp as libc::c_int) < 0 as libc::c_int {
            (*p_ptr).chp = (*p_ptr).mhp;
            (*p_ptr).chp_frac = 0 as libc::c_int as u16b
        }
        /* Redraw */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
        /* Heal 0-4 */
        if num < 5 as libc::c_int {
            msg_print(b"You feel a little better.\x00" as *const u8 as
                          *const libc::c_char);
        } else if num < 15 as libc::c_int {
            msg_print(b"You feel better.\x00" as *const u8 as
                          *const libc::c_char);
        } else if num < 35 as libc::c_int {
            msg_print(b"You feel much better.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            /* Heal 5-14 */
            /* Heal 15-34 */
            /* Heal 35+ */
            msg_print(b"You feel very good.\x00" as *const u8 as
                          *const libc::c_char);
        }
        /* Notice */
        return 1 as libc::c_int as bool_
    }
    /* Ignore */
    return 0 as libc::c_int as bool_;
}
/*
 * Leave a "glyph of warding" which prevents monster movement
 */
#[no_mangle]
pub unsafe extern "C" fn warding_glyph() {
    /* XXX XXX XXX */
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
        msg_print(b"The object resists the spell.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Create a glyph */
    cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                  0x3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn explosive_rune() {
    /* XXX XXX XXX */
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
        msg_print(b"The object resists the spell.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Create a glyph */
    cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                  0x40 as libc::c_int);
}
/*
 * Array of stat "descriptions"
 */
static mut desc_stat_pos: [cptr; 6] =
    [b"strong\x00" as *const u8 as *const libc::c_char,
     b"smart\x00" as *const u8 as *const libc::c_char,
     b"wise\x00" as *const u8 as *const libc::c_char,
     b"dextrous\x00" as *const u8 as *const libc::c_char,
     b"healthy\x00" as *const u8 as *const libc::c_char,
     b"cute\x00" as *const u8 as *const libc::c_char];
/*
 * Array of long descriptions of stat
 */
static mut long_desc_stat: [cptr; 6] =
    [b"strength\x00" as *const u8 as *const libc::c_char,
     b"intelligence\x00" as *const u8 as *const libc::c_char,
     b"wisdom\x00" as *const u8 as *const libc::c_char,
     b"dexterity\x00" as *const u8 as *const libc::c_char,
     b"constitution\x00" as *const u8 as *const libc::c_char,
     b"charisma\x00" as *const u8 as *const libc::c_char];
/*
 * Array of stat "descriptions"
 */
static mut desc_stat_neg: [cptr; 6] =
    [b"weak\x00" as *const u8 as *const libc::c_char,
     b"stupid\x00" as *const u8 as *const libc::c_char,
     b"naive\x00" as *const u8 as *const libc::c_char,
     b"clumsy\x00" as *const u8 as *const libc::c_char,
     b"sickly\x00" as *const u8 as *const libc::c_char,
     b"ugly\x00" as *const u8 as *const libc::c_char];
/*
 * Lose a "point"
 */
#[no_mangle]
pub unsafe extern "C" fn do_dec_stat(mut stat: libc::c_int,
                                     mut mode: libc::c_int) -> bool_ {
    let mut sust: bool_ = 0 as libc::c_int as bool_;
    /* Access the "sustain" */
    match stat {
        0 => {
            if (*p_ptr).sustain_str != 0 { sust = 1 as libc::c_int as bool_ }
        }
        1 => {
            if (*p_ptr).sustain_int != 0 { sust = 1 as libc::c_int as bool_ }
        }
        2 => {
            if (*p_ptr).sustain_wis != 0 { sust = 1 as libc::c_int as bool_ }
        }
        3 => {
            if (*p_ptr).sustain_dex != 0 { sust = 1 as libc::c_int as bool_ }
        }
        4 => {
            if (*p_ptr).sustain_con != 0 { sust = 1 as libc::c_int as bool_ }
        }
        5 => {
            if (*p_ptr).sustain_chr != 0 { sust = 1 as libc::c_int as bool_ }
        }
        _ => { }
    }
    /* Sustain */
    if sust != 0 {
        /* Message */
        msg_format(b"You feel %s for a moment, but the feeling passes.\x00" as
                       *const u8 as *const libc::c_char,
                   desc_stat_neg[stat as usize]);
        /* Notice effect */
        return 1 as libc::c_int as bool_
    }
    /* Attempt to reduce the stat */
    if dec_stat(stat, 10 as libc::c_int, mode) != 0 {
        /* Message */
        msg_format(b"You feel very %s.\x00" as *const u8 as
                       *const libc::c_char, desc_stat_neg[stat as usize]);
        /* Notice effect */
        return 1 as libc::c_int as bool_
    }
    /* Nothing obvious */
    return 0 as libc::c_int as bool_;
}
/*
 * Restore lost "points" in a stat
 */
#[no_mangle]
pub unsafe extern "C" fn do_res_stat(mut stat: libc::c_int, mut full: bool_)
 -> bool_ {
    /* Keep a copy of the current stat, so we can evaluate it if necessary */
    let mut cur_stat: libc::c_int =
        (*p_ptr).stat_cur[stat as usize] as libc::c_int;
    /* Attempt to increase */
    if res_stat(stat, full) != 0 {
        /* Message, depending on whether we got stronger or weaker */
        if cur_stat > (*p_ptr).stat_max[stat as usize] as libc::c_int {
            msg_format(b"You feel your %s boost drain away.\x00" as *const u8
                           as *const libc::c_char,
                       long_desc_stat[stat as usize]);
        } else {
            msg_format(b"You feel less %s.\x00" as *const u8 as
                           *const libc::c_char, desc_stat_neg[stat as usize]);
        }
        /* Notice */
        return 1 as libc::c_int as bool_
    }
    /* Nothing obvious */
    return 0 as libc::c_int as bool_;
}
/*
 * Gain a "point" in a stat
 */
#[no_mangle]
pub unsafe extern "C" fn do_inc_stat(mut stat: libc::c_int) -> bool_ {
    let mut res: bool_ = 0;
    /* Restore strength */
    res = res_stat(stat, 1 as libc::c_int as bool_);
    /* Attempt to increase */
    if inc_stat(stat) != 0 {
        /* Message */
        msg_format(b"Wow!  You feel very %s!\x00" as *const u8 as
                       *const libc::c_char, desc_stat_pos[stat as usize]);
        /* Notice */
        return 1 as libc::c_int as bool_
    }
    /* Restoration worked */
    if res != 0 {
        /* Message */
        msg_format(b"You feel less %s.\x00" as *const u8 as
                       *const libc::c_char, desc_stat_neg[stat as usize]);
        /* Notice */
        return 1 as libc::c_int as bool_
    }
    /* Nothing obvious */
    return 0 as libc::c_int as bool_;
}
/*
 * Identify everything being carried.
 * Done by a potion of "self knowledge".
 */
#[no_mangle]
pub unsafe extern "C" fn identify_pack() {
    let mut i: libc::c_int = 0;
    /* Simply identify and know every item */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Aware and Known */
            object_aware(o_ptr);
            object_known(o_ptr);
            /* Process the appropriate hooks */
            process_hooks(16 as libc::c_int,
                          b"(d,s)\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, i,
                          b"normal\x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    }
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
}
/*
 * common portions of identify_fully and identify_pack_fully
 */
unsafe extern "C" fn make_item_fully_identified(mut o_ptr: *mut object_type) {
    /* Identify it fully */
    object_aware(o_ptr);
    object_known(o_ptr);
    /* Mark the item as fully known */
    (*o_ptr).ident =
        ((*o_ptr).ident as libc::c_int | 0x20 as libc::c_int) as byte_hack;
    /* For those with alchemist skills, learn how to create it */
    alchemist_learn_object(o_ptr);
}
/*
 * Identify everything being carried.
 * Done by a potion of "self knowledge".
 */
#[no_mangle]
pub unsafe extern "C" fn identify_pack_fully() {
    let mut i: libc::c_int = 0;
    /* Simply identify and know every item */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            make_item_fully_identified(o_ptr);
            /* Process the appropriate hooks */
            process_hooks(16 as libc::c_int,
                          b"(d,s)\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, i,
                          b"full\x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    }
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
}
/*
 * Used by the "enchant" function (chance of failure)
 * (modified for Zangband, we need better stuff there...) -- TY
 */
static mut enchant_table: [libc::c_int; 16] =
    [0 as libc::c_int, 10 as libc::c_int, 50 as libc::c_int,
     100 as libc::c_int, 200 as libc::c_int, 300 as libc::c_int,
     400 as libc::c_int, 500 as libc::c_int, 650 as libc::c_int,
     800 as libc::c_int, 950 as libc::c_int, 987 as libc::c_int,
     993 as libc::c_int, 995 as libc::c_int, 998 as libc::c_int,
     1000 as libc::c_int];
#[no_mangle]
pub unsafe extern "C" fn remove_curse_object(mut o_ptr: *mut object_type,
                                             mut all: bool_) -> bool_ {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Skip non-objects */
    if (*o_ptr).k_idx == 0 { return 0 as libc::c_int as bool_ }
    /* Uncursed already */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Heavily Cursed Items need a special spell */
    if all == 0 && f3 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Perma-Cursed Items can NEVER be uncursed */
    if f3 as libc::c_long & 0x80000000 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Uncurse it */
    (*o_ptr).ident =
        ((*o_ptr).ident as libc::c_int & !(0x40 as libc::c_int)) as byte_hack;
    /* Hack -- Assume felt */
    (*o_ptr).ident =
        ((*o_ptr).ident as libc::c_int | 0x1 as libc::c_int) as byte_hack;
    if (*o_ptr).art_flags3 as libc::c_long & 0x20000000 as libc::c_long != 0 {
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long &
                 !(0x20000000 as libc::c_long)) as u32b
    }
    if (*o_ptr).art_flags3 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long &
                 !(0x40000000 as libc::c_long)) as u32b
    }
    /* Take note */
    (*o_ptr).sense = 10 as libc::c_int as byte_hack;
    /* Reverse the curse effect */
	/* jk - scrolls of *remove curse* have a 1 in (55-level chance to */
	/* reverse the curse effects - a ring of damage(-15) {cursed} then */
	/* becomes a ring of damage (+15) */
	/* this does not go for artifacts - a Sword of Mormegil +40,+60 would */
	/* be somewhat unbalancing */
	/* due to the nature of this procedure, it only works on cursed items */
	/* ie you get only one chance! */
    if Rand_div(55 as libc::c_int - (*p_ptr).lev as libc::c_int) +
           1 as libc::c_int == 1 as libc::c_int &&
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
                  } else { 0 as libc::c_int }) != 0) {
        if ((*o_ptr).to_a as libc::c_int) < 0 as libc::c_int {
            (*o_ptr).to_a = -((*o_ptr).to_a as libc::c_int) as s16b
        }
        if ((*o_ptr).to_h as libc::c_int) < 0 as libc::c_int {
            (*o_ptr).to_h = -((*o_ptr).to_h as libc::c_int) as s16b
        }
        if ((*o_ptr).to_d as libc::c_int) < 0 as libc::c_int {
            (*o_ptr).to_d = -((*o_ptr).to_d as libc::c_int) as s16b
        }
        if (*o_ptr).pval < 0 as libc::c_int { (*o_ptr).pval = -(*o_ptr).pval }
    }
    /* Recalculate the bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x2 as libc::c_long) as u32b;
    return 1 as libc::c_int as bool_;
}
/*
 * Removes curses from items in inventory
 *
 * Note that Items which are "Perma-Cursed" (The One Ring,
 * The Crown of Morgoth) can NEVER be uncursed.
 *
 * Note that if "all" is FALSE, then Items which are
 * "Heavy-Cursed" (Mormegil, Calris, and Weapons of Morgul)
 * will not be uncursed.
 */
unsafe extern "C" fn remove_curse_aux(mut all: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    /* Attempt to uncurse items being worn */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if !(remove_curse_object(o_ptr, all as bool_) == 0) {
            /* Count the uncursings */
            cnt += 1
        }
        i += 1
    }
    /* Return "something uncursed" */
    return cnt;
}
/*
 * Remove most curses
 */
#[no_mangle]
pub unsafe extern "C" fn remove_curse() -> bool_ {
    return if remove_curse_aux(0 as libc::c_int) != 0 {
               1 as libc::c_int
           } else { 0 as libc::c_int } as bool_;
}
/*
 * Remove all curses
 */
#[no_mangle]
pub unsafe extern "C" fn remove_all_curse() -> bool_ {
    return if remove_curse_aux(1 as libc::c_int) != 0 {
               1 as libc::c_int
           } else { 0 as libc::c_int } as bool_;
}
/*
 * Restores any drained experience
 */
#[no_mangle]
pub unsafe extern "C" fn restore_level() -> bool_ {
    /* Restore experience */
    if (*p_ptr).exp < (*p_ptr).max_exp {
        /* Message */
        msg_print(b"You feel your life energies returning.\x00" as *const u8
                      as *const libc::c_char);
        /* Restore the experience */
        (*p_ptr).exp = (*p_ptr).max_exp;
        /* Check the experience */
        check_experience();
        /* Did something */
        return 1 as libc::c_int as bool_
    }
    /* No effect */
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn alchemy() -> bool_ 
 /* Turns an object into gold, gain some of its value in a shop */
 {
    let mut item: libc::c_int = 0;
    let mut amt: libc::c_int = 1 as libc::c_int;
    let mut old_number: libc::c_int = 0;
    let mut price: libc::c_long = 0;
    let mut force: bool_ = 0 as libc::c_int as bool_;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Hack -- force destruction */
    if command_arg as libc::c_int > 0 as libc::c_int {
        force = 1 as libc::c_int as bool_
    }
    /* Get an item */
    q = b"Turn which item to gold? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to turn to gold.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return 0 as libc::c_int as bool_
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* See how many items */
    if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
        /* Get a quantity */
        amt = get_quantity(0 as cptr, (*o_ptr).number as s32b);
        /* Allow user abort */
        if amt <= 0 as libc::c_int { return 0 as libc::c_int as bool_ }
    }
    /* Describe the object */
    old_number = (*o_ptr).number as libc::c_int;
    (*o_ptr).number = amt as byte_hack;
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    (*o_ptr).number = old_number as byte_hack;
    /* Verify unless quantity given */
    if force == 0 {
        if !(auto_destroy as libc::c_int != 0 &&
                 object_value(o_ptr) < 1 as libc::c_int) {
            /* Make a verification */
            sprintf(out_val.as_mut_ptr(),
                    b"Really turn %s to gold? \x00" as *const u8 as
                        *const libc::c_char, o_name.as_mut_ptr());
            if get_check(out_val.as_mut_ptr() as cptr) == 0 {
                return 0 as libc::c_int as bool_
            }
        }
    }
    /* Artifacts cannot be destroyed */
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
        let mut feel: byte_hack = 8 as libc::c_int as byte_hack;
        /* Message */
        msg_format(b"You fail to turn %s to gold!\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr());
        /* Hack -- Handle icky artifacts */
        if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
            feel = 7 as libc::c_int as byte_hack
        }
        /* Hack -- inscribe the artifact */
        (*o_ptr).sense = feel;
        /* We have "felt" it (again) */
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x1 as libc::c_int) as byte_hack;
        /* Combine the pack */
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
        /* Done */
        return 0 as libc::c_int as bool_
    }
    price = object_value_real(o_ptr) as libc::c_long;
    if price <= 0 as libc::c_int as libc::c_long {
        /* Message */
        msg_format(b"You turn %s to fool\'s gold.\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr());
    } else {
        price /= 3 as libc::c_int as libc::c_long;
        if amt > 1 as libc::c_int { price *= amt as libc::c_long }
        msg_format(b"You turn %s to %ld coins worth of gold.\x00" as *const u8
                       as *const libc::c_char, o_name.as_mut_ptr(), price);
        (*p_ptr).au = ((*p_ptr).au as libc::c_long + price) as s32b;
        /* Redraw gold */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    }
    /* Eliminate the item */
    inc_stack_size(item, -amt);
    return 1 as libc::c_int as bool_;
}
/*
 * self-knowledge... idea from nethack.  Useful for determining powers and
 * resistances of items.  It saves the screen, clears it, then starts listing
 * attributes, a screenful at a time.  (There are a LOT of attributes to
 * list.  It will probably take 2 or 3 screens for a powerful character whose
 * using several artifacts...) -CFT
 *
 * It is now a lot more efficient. -BEN-
 *
 * See also "identify_fully()".
 *
 * XXX XXX XXX Use the "show_file()" method, perhaps.
 */
#[no_mangle]
pub unsafe extern "C" fn self_knowledge(mut fff: *mut FILE) {
    let mut i: libc::c_int = 0 as libc::c_int; /* Iterator for a loop */
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut f1: u32b = 0 as libc::c_long as u32b;
    let mut f2: u32b = 0 as libc::c_long as u32b;
    let mut f3: u32b = 0 as libc::c_long as u32b;
    let mut f4: u32b = 0 as libc::c_long as u32b;
    let mut f5: u32b = 0 as libc::c_long as u32b;
    let mut esp: u32b = 0 as libc::c_long as u32b;
    let mut iter: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut Dummy: [libc::c_char; 80] = [0; 80];
    let mut info: [cptr; 200] = [0 as *const libc::c_char; 200];
    strcpy(Dummy.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
    /* Acquire item flags from equipment */
    k = 24 as libc::c_int;
    while k < 52 as libc::c_int {
        let mut t1: u32b = 0;
        let mut t2: u32b = 0;
        let mut t3: u32b = 0;
        let mut t4: u32b = 0;
        let mut t5: u32b = 0;
        let mut esp_0: u32b = 0;
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(k as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Extract the flags */
            object_flags(o_ptr, &mut t1, &mut t2, &mut t3, &mut t4, &mut t5,
                         &mut esp_0);
            /* Extract flags */
            f1 |= t1;
            f2 |= t2;
            f3 |= t3
        }
        k += 1
    }
    if death != 0 {
        static mut buf: [libc::c_char; 250] = [0; 250];
        sprintf(buf.as_mut_ptr(),
                b"You are dead, killed by %s %s.\x00" as *const u8 as
                    *const libc::c_char, died_from.as_mut_ptr(),
                describe_player_location());
        let fresh0 = i;
        i = i + 1;
        info[fresh0 as usize] = buf.as_mut_ptr() as cptr
    }
    /* Racial powers... */
    if (*p_ptr).body_monster as libc::c_int != 0 as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset((*p_ptr).body_monster as isize) as
                *mut monster_race;
        if (*r_ptr).flags1 & 0x10 as libc::c_int as libc::c_uint != 0 ||
               (*r_ptr).flags1 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh1 = i;
            i = i + 1;
            info[fresh1 as usize] =
                b"You are transparent.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags1 & 0x20 as libc::c_int as libc::c_uint != 0 ||
               (*r_ptr).flags2 & 0x400 as libc::c_int as libc::c_uint != 0 {
            let fresh2 = i;
            i = i + 1;
            info[fresh2 as usize] =
                b"Your form constantly changes.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags1 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh3 = i;
            i = i + 1;
            info[fresh3 as usize] =
                b"Your color constantly changes.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags1 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let fresh4 = i;
            i = i + 1;
            info[fresh4 as usize] =
                b"You do not have a physical weapon.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags1 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let fresh5 = i;
            i = i + 1;
            info[fresh5 as usize] =
                b"You cannot move.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags1 & 0x40000 as libc::c_int as libc::c_uint != 0 &&
               (*r_ptr).flags1 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let fresh6 = i;
            i = i + 1;
            info[fresh6 as usize] =
                b"You move extremely erratically.\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).flags1 & 0x80000 as libc::c_int as libc::c_uint !=
                      0 {
            let fresh7 = i;
            i = i + 1;
            info[fresh7 as usize] =
                b"You move somewhat erratically.\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).flags1 & 0x40000 as libc::c_int as libc::c_uint !=
                      0 {
            let fresh8 = i;
            i = i + 1;
            info[fresh8 as usize] =
                b"You move a bit erratically.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let fresh9 = i;
            i = i + 1;
            info[fresh9 as usize] =
                b"You are very stupid (INT -4).\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint != 0 {
            let fresh10 = i;
            i = i + 1;
            info[fresh10 as usize] =
                b"You are very smart (INT +4).\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags2 & 0x4 as libc::c_int as libc::c_uint != 0 {
            let fresh11 = i;
            i = i + 1;
            info[fresh11 as usize] =
                b"You can speak.\x00" as *const u8 as *const libc::c_char
        } else {
            let fresh12 = i;
            i = i + 1;
            info[fresh12 as usize] =
                b"You cannot speak.\x00" as *const u8 as *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags2 & 0x20 as libc::c_int as libc::c_uint != 0 {
            let fresh13 = i;
            i = i + 1;
            info[fresh13 as usize] =
                b"You are cold blooded.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags2 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh14 = i;
            i = i + 1;
            info[fresh14 as usize] =
                b"You have an empty mind.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags2 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh15 = i;
            i = i + 1;
            info[fresh15 as usize] =
                b"You have a weird mind.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
            let fresh16 = i;
            i = i + 1;
            info[fresh16 as usize] =
                b"You can multiply.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags2 & 0x1000 as libc::c_int as libc::c_uint != 0 {
            let fresh17 = i;
            i = i + 1;
            info[fresh17 as usize] =
                b"You have strong breath.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags2 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            let fresh18 = i;
            i = i + 1;
            info[fresh18 as usize] =
                b"You are an eldritch horror.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags2 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let fresh19 = i;
            i = i + 1;
            info[fresh19 as usize] =
                b"You can open doors.\x00" as *const u8 as *const libc::c_char
        } else {
            let fresh20 = i;
            i = i + 1;
            info[fresh20 as usize] =
                b"You cannot open doors.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags2 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let fresh21 = i;
            i = i + 1;
            info[fresh21 as usize] =
                b"You can bash doors.\x00" as *const u8 as *const libc::c_char
        } else {
            let fresh22 = i;
            i = i + 1;
            info[fresh22 as usize] =
                b"You cannot bash doors.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags2 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            let fresh23 = i;
            i = i + 1;
            info[fresh23 as usize] =
                b"You can pass walls.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags2 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let fresh24 = i;
            i = i + 1;
            info[fresh24 as usize] =
                b"You destroy walls.\x00" as *const u8 as *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags2 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            let fresh25 = i;
            i = i + 1;
            info[fresh25 as usize] =
                b"You can move monsters.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags3 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let fresh26 = i;
            i = i + 1;
            info[fresh26 as usize] =
                b"You have orc blood in your veins.\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).flags3 & 0x2 as libc::c_int as libc::c_uint != 0 {
            let fresh27 = i;
            i = i + 1;
            info[fresh27 as usize] =
                b"You have troll blood in your veins.\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).flags3 & 0x4 as libc::c_int as libc::c_uint != 0 {
            let fresh28 = i;
            i = i + 1;
            info[fresh28 as usize] =
                b"You have giant blood in your veins.\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).flags3 & 0x8 as libc::c_int as libc::c_uint != 0 {
            let fresh29 = i;
            i = i + 1;
            info[fresh29 as usize] =
                b"You have dragon blood in your veins.\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 {
            let fresh30 = i;
            i = i + 1;
            info[fresh30 as usize] =
                b"You have demon blood in your veins.\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
            let fresh31 = i;
            i = i + 1;
            info[fresh31 as usize] =
                b"You are an undead.\x00" as *const u8 as *const libc::c_char
        } else if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh32 = i;
            i = i + 1;
            info[fresh32 as usize] =
                b"You are an animal.\x00" as *const u8 as *const libc::c_char
        } else if (*r_ptr).flags3 & 0x100 as libc::c_int as libc::c_uint != 0
         {
            let fresh33 = i;
            i = i + 1;
            info[fresh33 as usize] =
                b"You have thunderlord blood in your veins.\x00" as *const u8
                    as *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh34 = i;
            i = i + 1;
            info[fresh34 as usize] =
                b"You are inherently evil.\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).flags3 & 0x200 as libc::c_int as libc::c_uint != 0
         {
            let fresh35 = i;
            i = i + 1;
            info[fresh35 as usize] =
                b"You are inherently good.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x400 as libc::c_int as libc::c_uint != 0 {
            let fresh36 = i;
            i = i + 1;
            info[fresh36 as usize] =
                b"You are surrounded by a chilly aura.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Not implemented */
        /* Not implemented */
        /* Not implemented */
        /* Not implemented */
        /* Not implemented */
        /* Not implemented */
        /* Not implemented */
        /* Not implemented */
        if (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0 {
            let fresh37 = i;
            i = i + 1;
            info[fresh37 as usize] =
                b"You are not living.\x00" as *const u8 as *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags3 & 0x1000 as libc::c_int as libc::c_uint != 0 {
            let fresh38 = i;
            i = i + 1;
            info[fresh38 as usize] =
                b"Your eyes are vulnerable to bright light.\x00" as *const u8
                    as *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags3 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            let fresh39 = i;
            i = i + 1;
            info[fresh39 as usize] =
                b"You can be hurt by rock remover.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            let fresh40 = i;
            i = i + 1;
            info[fresh40 as usize] =
                b"You are vulnerable to fire.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            let fresh41 = i;
            i = i + 1;
            info[fresh41 as usize] =
                b"You are vulnerable to cold.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            let fresh42 = i;
            i = i + 1;
            info[fresh42 as usize] =
                b"You are resistant to teleportation.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            let fresh43 = i;
            i = i + 1;
            info[fresh43 as usize] =
                b"You are resistant to nether.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            let fresh44 = i;
            i = i + 1;
            info[fresh44 as usize] =
                b"You are resistant to water.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            let fresh45 = i;
            i = i + 1;
            info[fresh45 as usize] =
                b"You are resistant to plasma.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            let fresh46 = i;
            i = i + 1;
            info[fresh46 as usize] =
                b"You are resistant to nexus.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            let fresh47 = i;
            i = i + 1;
            info[fresh47 as usize] =
                b"You are resistant to disease.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags3 & 0x80000000 as libc::c_uint != 0 {
            let fresh48 = i;
            i = i + 1;
            info[fresh48 as usize] =
                b"You cannot be slept.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags3 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            let fresh49 = i;
            i = i + 1;
            info[fresh49 as usize] =
                b"You are a Nazgul.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            let fresh50 = i;
            i = i + 1;
            info[fresh50 as usize] =
                b"You are immune to fear.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            let fresh51 = i;
            i = i + 1;
            info[fresh51 as usize] =
                b"You are immune to stun.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            let fresh52 = i;
            i = i + 1;
            info[fresh52 as usize] =
                b"You are immune to confusion.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags3 & 0x80000000 as libc::c_uint != 0 {
            let fresh53 = i;
            i = i + 1;
            info[fresh53 as usize] =
                b"You are immune to sleep.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let fresh54 = i;
            i = i + 1;
            info[fresh54 as usize] =
                b"You can aggravate monsters.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x8 as libc::c_int as libc::c_uint != 0 {
            let fresh55 = i;
            i = i + 1;
            info[fresh55 as usize] =
                b"You can fire a rocket.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x10 as libc::c_int as libc::c_uint != 0 {
            let fresh56 = i;
            i = i + 1;
            info[fresh56 as usize] =
                b"You can fire a light arrow.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x20 as libc::c_int as libc::c_uint != 0 {
            let fresh57 = i;
            i = i + 1;
            info[fresh57 as usize] =
                b"You can fire a heavy arrow.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh58 = i;
            i = i + 1;
            info[fresh58 as usize] =
                b"You can fire a light missile.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh59 = i;
            i = i + 1;
            info[fresh59 as usize] =
                b"You can fire a heavy missile.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x100 as libc::c_int as libc::c_uint != 0 {
            let fresh60 = i;
            i = i + 1;
            info[fresh60 as usize] =
                b"You can breathe acid.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x200 as libc::c_int as libc::c_uint != 0 {
            let fresh61 = i;
            i = i + 1;
            info[fresh61 as usize] =
                b"You can breathe electricity.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x400 as libc::c_int as libc::c_uint != 0 {
            let fresh62 = i;
            i = i + 1;
            info[fresh62 as usize] =
                b"You can breathe fire.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x800 as libc::c_int as libc::c_uint != 0 {
            let fresh63 = i;
            i = i + 1;
            info[fresh63 as usize] =
                b"You can breathe cold.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x1000 as libc::c_int as libc::c_uint != 0 {
            let fresh64 = i;
            i = i + 1;
            info[fresh64 as usize] =
                b"You can breathe poison.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            let fresh65 = i;
            i = i + 1;
            info[fresh65 as usize] =
                b"You can breathe nether.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            let fresh66 = i;
            i = i + 1;
            info[fresh66 as usize] =
                b"You can breathe light.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            let fresh67 = i;
            i = i + 1;
            info[fresh67 as usize] =
                b"You can breathe darkness.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let fresh68 = i;
            i = i + 1;
            info[fresh68 as usize] =
                b"You can breathe confusion.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let fresh69 = i;
            i = i + 1;
            info[fresh69 as usize] =
                b"You can breathe sound.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            let fresh70 = i;
            i = i + 1;
            info[fresh70 as usize] =
                b"You can breathe chaos.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let fresh71 = i;
            i = i + 1;
            info[fresh71 as usize] =
                b"You can breathe disenchantment.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            let fresh72 = i;
            i = i + 1;
            info[fresh72 as usize] =
                b"You can breathe nexus.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            let fresh73 = i;
            i = i + 1;
            info[fresh73 as usize] =
                b"You can breathe time.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            let fresh74 = i;
            i = i + 1;
            info[fresh74 as usize] =
                b"You can breathe inertia.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            let fresh75 = i;
            i = i + 1;
            info[fresh75 as usize] =
                b"You can breathe gravity.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            let fresh76 = i;
            i = i + 1;
            info[fresh76 as usize] =
                b"You can breathe shards.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            let fresh77 = i;
            i = i + 1;
            info[fresh77 as usize] =
                b"You can breathe plasma.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            let fresh78 = i;
            i = i + 1;
            info[fresh78 as usize] =
                b"You can breathe force.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            let fresh79 = i;
            i = i + 1;
            info[fresh79 as usize] =
                b"You can breathe mana.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            let fresh80 = i;
            i = i + 1;
            info[fresh80 as usize] =
                b"You can breathe nuke.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags4 & 0x80000000 as libc::c_uint != 0 {
            let fresh81 = i;
            i = i + 1;
            info[fresh81 as usize] =
                b"You can breathe disintegration.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let fresh82 = i;
            i = i + 1;
            info[fresh82 as usize] =
                b"You can cast a ball of acid.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x2 as libc::c_int as libc::c_uint != 0 {
            let fresh83 = i;
            i = i + 1;
            info[fresh83 as usize] =
                b"You can cast a ball of electricity.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x4 as libc::c_int as libc::c_uint != 0 {
            let fresh84 = i;
            i = i + 1;
            info[fresh84 as usize] =
                b"You can cast a ball of fire.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x8 as libc::c_int as libc::c_uint != 0 {
            let fresh85 = i;
            i = i + 1;
            info[fresh85 as usize] =
                b"You can cast a ball of cold.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x10 as libc::c_int as libc::c_uint != 0 {
            let fresh86 = i;
            i = i + 1;
            info[fresh86 as usize] =
                b"You can cast a ball of poison.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x20 as libc::c_int as libc::c_uint != 0 {
            let fresh87 = i;
            i = i + 1;
            info[fresh87 as usize] =
                b"You can cast a ball of nether.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh88 = i;
            i = i + 1;
            info[fresh88 as usize] =
                b"You can cast a ball of water.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags5 & 0x200 as libc::c_int as libc::c_uint != 0 {
            let fresh89 = i;
            i = i + 1;
            info[fresh89 as usize] =
                b"You can drain mana.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x400 as libc::c_int as libc::c_uint != 0 {
            let fresh90 = i;
            i = i + 1;
            info[fresh90 as usize] =
                b"You can cause mind blasting.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x800 as libc::c_int as libc::c_uint != 0 {
            let fresh91 = i;
            i = i + 1;
            info[fresh91 as usize] =
                b"You can cause brain smashing.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x1000 as libc::c_int as libc::c_uint != 0 {
            let fresh92 = i;
            i = i + 1;
            info[fresh92 as usize] =
                b"You can cause light wounds.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            let fresh93 = i;
            i = i + 1;
            info[fresh93 as usize] =
                b"You can cause serious wounds.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            let fresh94 = i;
            i = i + 1;
            info[fresh94 as usize] =
                b"You can cause critical wounds.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            let fresh95 = i;
            i = i + 1;
            info[fresh95 as usize] =
                b"You can cause mortal wounds.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let fresh96 = i;
            i = i + 1;
            info[fresh96 as usize] =
                b"You can cast a bolt of acid.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let fresh97 = i;
            i = i + 1;
            info[fresh97 as usize] =
                b"You can cast a bolt of electricity.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            let fresh98 = i;
            i = i + 1;
            info[fresh98 as usize] =
                b"You can cast a bolt of fire.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let fresh99 = i;
            i = i + 1;
            info[fresh99 as usize] =
                b"You can cast a bolt of cold.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            let fresh100 = i;
            i = i + 1;
            info[fresh100 as usize] =
                b"You can cast a bolt of poison.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            let fresh101 = i;
            i = i + 1;
            info[fresh101 as usize] =
                b"You can cast a bolt of nether.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            let fresh102 = i;
            i = i + 1;
            info[fresh102 as usize] =
                b"You can cast a bolt of water.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            let fresh103 = i;
            i = i + 1;
            info[fresh103 as usize] =
                b"You can cast a bolt of mana.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            let fresh104 = i;
            i = i + 1;
            info[fresh104 as usize] =
                b"You can cast a bolt of plasma.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            let fresh105 = i;
            i = i + 1;
            info[fresh105 as usize] =
                b"You can cast a bolt of ice.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            let fresh106 = i;
            i = i + 1;
            info[fresh106 as usize] =
                b"You can cast magic missile.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            let fresh107 = i;
            i = i + 1;
            info[fresh107 as usize] =
                b"You can terrify.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            let fresh108 = i;
            i = i + 1;
            info[fresh108 as usize] =
                b"You can blind.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            let fresh109 = i;
            i = i + 1;
            info[fresh109 as usize] =
                b"You can use confusion.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            let fresh110 = i;
            i = i + 1;
            info[fresh110 as usize] =
                b"You can cast slow.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags5 & 0x80000000 as libc::c_uint != 0 {
            let fresh111 = i;
            i = i + 1;
            info[fresh111 as usize] =
                b"You can touch to paralyze.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let fresh112 = i;
            i = i + 1;
            info[fresh112 as usize] =
                b"You can haste yourself.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x2 as libc::c_int as libc::c_uint != 0 {
            let fresh113 = i;
            i = i + 1;
            info[fresh113 as usize] =
                b"You can invoke Hand of Doom.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x4 as libc::c_int as libc::c_uint != 0 {
            let fresh114 = i;
            i = i + 1;
            info[fresh114 as usize] =
                b"You can heal yourself.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x10 as libc::c_int as libc::c_uint != 0 {
            let fresh115 = i;
            i = i + 1;
            info[fresh115 as usize] =
                b"You can blink.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x20 as libc::c_int as libc::c_uint != 0 {
            let fresh116 = i;
            i = i + 1;
            info[fresh116 as usize] =
                b"You can teleport.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh117 = i;
            i = i + 1;
            info[fresh117 as usize] =
                b"You can go between places.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh118 = i;
            i = i + 1;
            info[fresh118 as usize] =
                b"You can teleport away.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x100 as libc::c_int as libc::c_uint != 0 {
            let fresh119 = i;
            i = i + 1;
            info[fresh119 as usize] =
                b"You can teleport level.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x200 as libc::c_int as libc::c_uint != 0 {
            let fresh120 = i;
            i = i + 1;
            info[fresh120 as usize] =
                b"You can create darkness.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x400 as libc::c_int as libc::c_uint != 0 {
            let fresh121 = i;
            i = i + 1;
            info[fresh121 as usize] =
                b"You can create traps.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags6 & 0x800 as libc::c_int as libc::c_uint != 0 {
            let fresh122 = i;
            i = i + 1;
            info[fresh122 as usize] =
                b"You can fade memories.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x1000 as libc::c_int as libc::c_uint != 0 {
            let fresh123 = i;
            i = i + 1;
            info[fresh123 as usize] =
                b"You can Raise the Dead.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            let fresh124 = i;
            i = i + 1;
            info[fresh124 as usize] =
                b"You can magically summon a Software Bugs.\x00" as *const u8
                    as *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            let fresh125 = i;
            i = i + 1;
            info[fresh125 as usize] =
                b"You can magically summon the RNG.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            let fresh126 = i;
            i = i + 1;
            info[fresh126 as usize] =
                b"You can magically summon some Thunderlords.\x00" as
                    *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let fresh127 = i;
            i = i + 1;
            info[fresh127 as usize] =
                b"You can magically summon some Kins.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let fresh128 = i;
            i = i + 1;
            info[fresh128 as usize] =
                b"You can magically summon greater demons.\x00" as *const u8
                    as *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            let fresh129 = i;
            i = i + 1;
            info[fresh129 as usize] =
                b"You can magically summon a monster.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let fresh130 = i;
            i = i + 1;
            info[fresh130 as usize] =
                b"You can magically summon monsters.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            let fresh131 = i;
            i = i + 1;
            info[fresh131 as usize] =
                b"You can magically summon ants.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            let fresh132 = i;
            i = i + 1;
            info[fresh132 as usize] =
                b"You can magically summon spiders.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            let fresh133 = i;
            i = i + 1;
            info[fresh133 as usize] =
                b"You can magically summon hounds.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            let fresh134 = i;
            i = i + 1;
            info[fresh134 as usize] =
                b"You can magically summon hydras.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            let fresh135 = i;
            i = i + 1;
            info[fresh135 as usize] =
                b"You can magically summon an angel.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            let fresh136 = i;
            i = i + 1;
            info[fresh136 as usize] =
                b"You can magically summon a demon.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            let fresh137 = i;
            i = i + 1;
            info[fresh137 as usize] =
                b"You can magically summon an undead.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            let fresh138 = i;
            i = i + 1;
            info[fresh138 as usize] =
                b"You can magically summon a dragon.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            let fresh139 = i;
            i = i + 1;
            info[fresh139 as usize] =
                b"You can magically summon greater undead.\x00" as *const u8
                    as *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            let fresh140 = i;
            i = i + 1;
            info[fresh140 as usize] =
                b"You can magically summon greater dragons.\x00" as *const u8
                    as *const libc::c_char
        }
        if (*r_ptr).flags6 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            let fresh141 = i;
            i = i + 1;
            info[fresh141 as usize] =
                b"You can magically summon a wraith.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags6 & 0x80000000 as libc::c_uint != 0 {
            let fresh142 = i;
            i = i + 1;
            info[fresh142 as usize] =
                b"You can magically summon an unique monster.\x00" as
                    *const u8 as *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags7 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let fresh143 = i;
            i = i + 1;
            info[fresh143 as usize] =
                b"You are aquatic.\x00" as *const u8 as *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags7 & 0x2 as libc::c_int as libc::c_uint != 0 {
            let fresh144 = i;
            i = i + 1;
            info[fresh144 as usize] =
                b"You can swim.\x00" as *const u8 as *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint != 0 {
            let fresh145 = i;
            i = i + 1;
            info[fresh145 as usize] =
                b"You can fly.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags7 & 0x20 as libc::c_int as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            let fresh146 = i;
            i = i + 1;
            info[fresh146 as usize] =
                b"You are immortal.\x00" as *const u8 as *const libc::c_char
        }
        /* Not implemented */
        if (*r_ptr).flags7 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh147 = i;
            i = i + 1;
            info[fresh147 as usize] =
                b"You are a Nazgul.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags7 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh148 = i;
            i = i + 1;
            info[fresh148 as usize] =
                b"You are a spider.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags8 & 0x2 as libc::c_int as libc::c_uint != 0 {
            let fresh149 = i;
            i = i + 1;
            info[fresh149 as usize] =
                b"You appear in towns.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags8 & 0x8 as libc::c_int as libc::c_uint != 0 {
            let fresh150 = i;
            i = i + 1;
            info[fresh150 as usize] =
                b"You appear on the shore.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags8 & 0x10 as libc::c_int as libc::c_uint != 0 {
            let fresh151 = i;
            i = i + 1;
            info[fresh151 as usize] =
                b"You appear in the ocean.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags8 & 0x20 as libc::c_int as libc::c_uint != 0 {
            let fresh152 = i;
            i = i + 1;
            info[fresh152 as usize] =
                b"You appear in the waste.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags8 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh153 = i;
            i = i + 1;
            info[fresh153 as usize] =
                b"You appear in woods.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags8 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh154 = i;
            i = i + 1;
            info[fresh154 as usize] =
                b"You appear in volcanos.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags8 & 0x200 as libc::c_int as libc::c_uint != 0 {
            let fresh155 = i;
            i = i + 1;
            info[fresh155 as usize] =
                b"You appear in the mountains.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags8 & 0x400 as libc::c_int as libc::c_uint != 0 {
            let fresh156 = i;
            i = i + 1;
            info[fresh156 as usize] =
                b"You appear in grassy areas.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags9 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh157 = i;
            i = i + 1;
            info[fresh157 as usize] =
                b"You are vulnerable to acid.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags9 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh158 = i;
            i = i + 1;
            info[fresh158 as usize] =
                b"You are vulnerable to electricity.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags9 & 0x100 as libc::c_int as libc::c_uint != 0 {
            let fresh159 = i;
            i = i + 1;
            info[fresh159 as usize] =
                b"You are vulnerable to poison.\x00" as *const u8 as
                    *const libc::c_char
        }
        if (*r_ptr).flags9 & 0x200 as libc::c_int as libc::c_uint != 0 {
            let fresh160 = i;
            i = i + 1;
            info[fresh160 as usize] =
                b"You can eat trees.\x00" as *const u8 as *const libc::c_char
        }
        if (*r_ptr).flags9 & 0x400 as libc::c_int as libc::c_uint != 0 {
            let fresh161 = i;
            i = i + 1;
            info[fresh161 as usize] =
                b"You are protected by great wyrms of power.\x00" as *const u8
                    as *const libc::c_char
        }
    }
    /* List powers */
    iter = 0 as libc::c_int;
    while iter < power_max as libc::c_int {
        if *(*p_ptr).powers.offset(iter as isize) != 0 {
            let fresh162 = i;
            i = i + 1;
            info[fresh162 as usize] =
                (*powers_type.offset(iter as isize)).desc_text as cptr
        }
        iter += 1
    }
    if (*p_ptr).allow_one_death != 0 {
        let fresh163 = i;
        i = i + 1;
        info[fresh163 as usize] =
            b"The Blood of Life flows through your veins.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).blind != 0 {
        let fresh164 = i;
        i = i + 1;
        info[fresh164 as usize] =
            b"You cannot see.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).confused != 0 {
        let fresh165 = i;
        i = i + 1;
        info[fresh165 as usize] =
            b"You are confused.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).afraid != 0 {
        let fresh166 = i;
        i = i + 1;
        info[fresh166 as usize] =
            b"You are terrified.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).cut != 0 {
        let fresh167 = i;
        i = i + 1;
        info[fresh167 as usize] =
            b"You are bleeding.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).stun != 0 {
        let fresh168 = i;
        i = i + 1;
        info[fresh168 as usize] =
            b"You are stunned.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).poisoned != 0 {
        let fresh169 = i;
        i = i + 1;
        info[fresh169 as usize] =
            b"You are poisoned.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).image != 0 {
        let fresh170 = i;
        i = i + 1;
        info[fresh170 as usize] =
            b"You are hallucinating.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).aggravate != 0 {
        let fresh171 = i;
        i = i + 1;
        info[fresh171 as usize] =
            b"You aggravate monsters.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).teleport != 0 {
        let fresh172 = i;
        i = i + 1;
        info[fresh172 as usize] =
            b"Your position is very uncertain.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).blessed != 0 {
        let fresh173 = i;
        i = i + 1;
        info[fresh173 as usize] =
            b"You feel righteous.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).hero != 0 {
        let fresh174 = i;
        i = i + 1;
        info[fresh174 as usize] =
            b"You feel heroic.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).shero != 0 {
        let fresh175 = i;
        i = i + 1;
        info[fresh175 as usize] =
            b"You are in a battle rage.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).protevil != 0 {
        let fresh176 = i;
        i = i + 1;
        info[fresh176 as usize] =
            b"You are protected from evil.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).protgood != 0 {
        let fresh177 = i;
        i = i + 1;
        info[fresh177 as usize] =
            b"You are protected from good.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).shield != 0 {
        let fresh178 = i;
        i = i + 1;
        info[fresh178 as usize] =
            b"You are protected by a mystic shield.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).invuln != 0 {
        let fresh179 = i;
        i = i + 1;
        info[fresh179 as usize] =
            b"You are temporarily invulnerable.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).confusing != 0 {
        let fresh180 = i;
        i = i + 1;
        info[fresh180 as usize] =
            b"Your hands are glowing dull red.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).searching != 0 {
        let fresh181 = i;
        i = i + 1;
        info[fresh181 as usize] =
            b"You are looking around very carefully.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).new_spells != 0 {
        let fresh182 = i;
        i = i + 1;
        info[fresh182 as usize] =
            b"You can learn some spells/prayers.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).word_recall != 0 {
        let fresh183 = i;
        i = i + 1;
        info[fresh183 as usize] =
            b"You will soon be recalled.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).see_infra != 0 {
        let fresh184 = i;
        i = i + 1;
        info[fresh184 as usize] =
            b"Your eyes are sensitive to infrared light.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).see_inv != 0 {
        let fresh185 = i;
        i = i + 1;
        info[fresh185 as usize] =
            b"You can see invisible creatures.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).magical_breath != 0 {
        let fresh186 = i;
        i = i + 1;
        info[fresh186 as usize] =
            b"You can breathe without air.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).water_breath != 0 {
        let fresh187 = i;
        i = i + 1;
        info[fresh187 as usize] =
            b"You can breathe underwater.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).ffall != 0 {
        let fresh188 = i;
        i = i + 1;
        info[fresh188 as usize] =
            b"You levitate just over the ground.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).climb != 0 {
        let fresh189 = i;
        i = i + 1;
        info[fresh189 as usize] =
            b"You can climb high mountains.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).free_act != 0 {
        let fresh190 = i;
        i = i + 1;
        info[fresh190 as usize] =
            b"You have free action.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).regenerate != 0 {
        let fresh191 = i;
        i = i + 1;
        info[fresh191 as usize] =
            b"You regenerate quickly.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).slow_digest != 0 {
        let fresh192 = i;
        i = i + 1;
        info[fresh192 as usize] =
            b"Your appetite is small.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).telepathy != 0 {
        if (*p_ptr).telepathy as libc::c_long & 0x80000000 as libc::c_long !=
               0 {
            let fresh193 = i;
            i = i + 1;
            info[fresh193 as usize] =
                b"You have ESP.\x00" as *const u8 as *const libc::c_char
        } else {
            if (*p_ptr).telepathy as libc::c_long & 0x1 as libc::c_long != 0 {
                let fresh194 = i;
                i = i + 1;
                info[fresh194 as usize] =
                    b"You can sense the presence of orcs.\x00" as *const u8 as
                        *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x2 as libc::c_long != 0 {
                let fresh195 = i;
                i = i + 1;
                info[fresh195 as usize] =
                    b"You can sense the presence of trolls.\x00" as *const u8
                        as *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x4 as libc::c_long != 0 {
                let fresh196 = i;
                i = i + 1;
                info[fresh196 as usize] =
                    b"You can sense the presence of dragons.\x00" as *const u8
                        as *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x1000 as libc::c_long !=
                   0 {
                let fresh197 = i;
                i = i + 1;
                info[fresh197 as usize] =
                    b"You can sense the presence of spiders.\x00" as *const u8
                        as *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x8 as libc::c_long != 0 {
                let fresh198 = i;
                i = i + 1;
                info[fresh198 as usize] =
                    b"You can sense the presence of giants.\x00" as *const u8
                        as *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x10 as libc::c_long != 0
               {
                let fresh199 = i;
                i = i + 1;
                info[fresh199 as usize] =
                    b"You can sense the presence of demons.\x00" as *const u8
                        as *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x20 as libc::c_long != 0
               {
                let fresh200 = i;
                i = i + 1;
                info[fresh200 as usize] =
                    b"You can sense presence of undead.\x00" as *const u8 as
                        *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x40 as libc::c_long != 0
               {
                let fresh201 = i;
                i = i + 1;
                info[fresh201 as usize] =
                    b"You can sense the presence of evil beings.\x00" as
                        *const u8 as *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x80 as libc::c_long != 0
               {
                let fresh202 = i;
                i = i + 1;
                info[fresh202 as usize] =
                    b"You can sense the presence of animals.\x00" as *const u8
                        as *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x100 as libc::c_long != 0
               {
                let fresh203 = i;
                i = i + 1;
                info[fresh203 as usize] =
                    b"You can sense the presence of thunderlords.\x00" as
                        *const u8 as *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x200 as libc::c_long != 0
               {
                let fresh204 = i;
                i = i + 1;
                info[fresh204 as usize] =
                    b"You can sense the presence of good beings.\x00" as
                        *const u8 as *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x400 as libc::c_long != 0
               {
                let fresh205 = i;
                i = i + 1;
                info[fresh205 as usize] =
                    b"You can sense the presence of non-living things.\x00" as
                        *const u8 as *const libc::c_char
            }
            if (*p_ptr).telepathy as libc::c_long & 0x800 as libc::c_long != 0
               {
                let fresh206 = i;
                i = i + 1;
                info[fresh206 as usize] =
                    b"You can sense the presence of unique beings.\x00" as
                        *const u8 as *const libc::c_char
            }
        }
    }
    if luck(-(100 as libc::c_int), 100 as libc::c_int) == 0 {
        let fresh207 = i;
        i = i + 1;
        info[fresh207 as usize] =
            b"You have normal luck.\x00" as *const u8 as *const libc::c_char
    } else if luck(-(100 as libc::c_int), 100 as libc::c_int) <
                  0 as libc::c_int {
        if luck(-(100 as libc::c_int), 100 as libc::c_int) <
               -(90 as libc::c_int) {
            let fresh208 = i;
            i = i + 1;
            info[fresh208 as usize] =
                b"You are incredibly unlucky.\x00" as *const u8 as
                    *const libc::c_char
        } else if luck(-(100 as libc::c_int), 100 as libc::c_int) <
                      -(60 as libc::c_int) {
            let fresh209 = i;
            i = i + 1;
            info[fresh209 as usize] =
                b"You are extremely unlucky.\x00" as *const u8 as
                    *const libc::c_char
        } else if luck(-(100 as libc::c_int), 100 as libc::c_int) <
                      -(30 as libc::c_int) {
            let fresh210 = i;
            i = i + 1;
            info[fresh210 as usize] =
                b"You are very unlucky.\x00" as *const u8 as
                    *const libc::c_char
        } else {
            let fresh211 = i;
            i = i + 1;
            info[fresh211 as usize] =
                b"You are unlucky.\x00" as *const u8 as *const libc::c_char
        }
    } else if luck(-(100 as libc::c_int), 100 as libc::c_int) >
                  0 as libc::c_int {
        if luck(-(100 as libc::c_int), 100 as libc::c_int) > 90 as libc::c_int
           {
            let fresh212 = i;
            i = i + 1;
            info[fresh212 as usize] =
                b"You are incredibly lucky.\x00" as *const u8 as
                    *const libc::c_char
        } else if luck(-(100 as libc::c_int), 100 as libc::c_int) >
                      60 as libc::c_int {
            let fresh213 = i;
            i = i + 1;
            info[fresh213 as usize] =
                b"You are extremely lucky.\x00" as *const u8 as
                    *const libc::c_char
        } else if luck(-(100 as libc::c_int), 100 as libc::c_int) >
                      30 as libc::c_int {
            let fresh214 = i;
            i = i + 1;
            info[fresh214 as usize] =
                b"You are very lucky.\x00" as *const u8 as *const libc::c_char
        } else {
            let fresh215 = i;
            i = i + 1;
            info[fresh215 as usize] =
                b"You are lucky.\x00" as *const u8 as *const libc::c_char
        }
    }
    if (*p_ptr).auto_id != 0 {
        let fresh216 = i;
        i = i + 1;
        info[fresh216 as usize] =
            b"You know everything.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).hold_life != 0 {
        let fresh217 = i;
        i = i + 1;
        info[fresh217 as usize] =
            b"You have a firm hold on your life force.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).reflect != 0 {
        let fresh218 = i;
        i = i + 1;
        info[fresh218 as usize] =
            b"You reflect arrows and bolts.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).sh_fire != 0 {
        let fresh219 = i;
        i = i + 1;
        info[fresh219 as usize] =
            b"You are surrounded with a fiery aura.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).sh_elec != 0 {
        let fresh220 = i;
        i = i + 1;
        info[fresh220 as usize] =
            b"You are surrounded with electricity.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).antimagic != 0 {
        let fresh221 = i;
        i = i + 1;
        info[fresh221 as usize] =
            b"You are surrounded by an anti-magic field.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).anti_magic != 0 {
        let fresh222 = i;
        i = i + 1;
        info[fresh222 as usize] =
            b"You are surrounded by an anti-magic shell.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).wraith_form != 0 {
        let fresh223 = i;
        i = i + 1;
        info[fresh223 as usize] =
            b"You are incorporeal.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).anti_tele != 0 {
        let fresh224 = i;
        i = i + 1;
        info[fresh224 as usize] =
            b"You cannot teleport.\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).lite != 0 {
        let fresh225 = i;
        i = i + 1;
        info[fresh225 as usize] =
            b"You are carrying a permanent light.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).immune_acid != 0 {
        let fresh226 = i;
        i = i + 1;
        info[fresh226 as usize] =
            b"You are completely immune to acid.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).resist_acid as libc::c_int != 0 &&
                  (*p_ptr).oppose_acid as libc::c_int != 0 {
        let fresh227 = i;
        i = i + 1;
        info[fresh227 as usize] =
            b"You resist acid exceptionally well.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).resist_acid as libc::c_int != 0 ||
                  (*p_ptr).oppose_acid as libc::c_int != 0 {
        let fresh228 = i;
        i = i + 1;
        info[fresh228 as usize] =
            b"You are resistant to acid.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).immune_elec != 0 {
        let fresh229 = i;
        i = i + 1;
        info[fresh229 as usize] =
            b"You are completely immune to lightning.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).resist_elec as libc::c_int != 0 &&
                  (*p_ptr).oppose_elec as libc::c_int != 0 {
        let fresh230 = i;
        i = i + 1;
        info[fresh230 as usize] =
            b"You resist lightning exceptionally well.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).resist_elec as libc::c_int != 0 ||
                  (*p_ptr).oppose_elec as libc::c_int != 0 {
        let fresh231 = i;
        i = i + 1;
        info[fresh231 as usize] =
            b"You are resistant to lightning.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).immune_fire != 0 {
        let fresh232 = i;
        i = i + 1;
        info[fresh232 as usize] =
            b"You are completely immune to fire.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).resist_fire as libc::c_int != 0 &&
                  (*p_ptr).oppose_fire as libc::c_int != 0 {
        let fresh233 = i;
        i = i + 1;
        info[fresh233 as usize] =
            b"You resist fire exceptionally well.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).resist_fire as libc::c_int != 0 ||
                  (*p_ptr).oppose_fire as libc::c_int != 0 {
        let fresh234 = i;
        i = i + 1;
        info[fresh234 as usize] =
            b"You are resistant to fire.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).sensible_fire != 0 {
        let fresh235 = i;
        i = i + 1;
        info[fresh235 as usize] =
            b"You are very vulnerable to fire.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).immune_cold != 0 {
        let fresh236 = i;
        i = i + 1;
        info[fresh236 as usize] =
            b"You are completely immune to cold.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).resist_cold as libc::c_int != 0 &&
                  (*p_ptr).oppose_cold as libc::c_int != 0 {
        let fresh237 = i;
        i = i + 1;
        info[fresh237 as usize] =
            b"You resist cold exceptionally well.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).resist_cold as libc::c_int != 0 ||
                  (*p_ptr).oppose_cold as libc::c_int != 0 {
        let fresh238 = i;
        i = i + 1;
        info[fresh238 as usize] =
            b"You are resistant to cold.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_pois as libc::c_int != 0 &&
           (*p_ptr).oppose_pois as libc::c_int != 0 {
        let fresh239 = i;
        i = i + 1;
        info[fresh239 as usize] =
            b"You resist poison exceptionally well.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).resist_pois as libc::c_int != 0 ||
                  (*p_ptr).oppose_pois as libc::c_int != 0 {
        let fresh240 = i;
        i = i + 1;
        info[fresh240 as usize] =
            b"You are resistant to poison.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_lite != 0 {
        let fresh241 = i;
        i = i + 1;
        info[fresh241 as usize] =
            b"You are resistant to bright light.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_dark != 0 {
        let fresh242 = i;
        i = i + 1;
        info[fresh242 as usize] =
            b"You are resistant to darkness.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_conf != 0 {
        let fresh243 = i;
        i = i + 1;
        info[fresh243 as usize] =
            b"You are resistant to confusion.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_sound != 0 {
        let fresh244 = i;
        i = i + 1;
        info[fresh244 as usize] =
            b"You are resistant to sonic attacks.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_disen != 0 {
        let fresh245 = i;
        i = i + 1;
        info[fresh245 as usize] =
            b"You are resistant to disenchantment.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_chaos != 0 {
        let fresh246 = i;
        i = i + 1;
        info[fresh246 as usize] =
            b"You are resistant to chaos.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_shard != 0 {
        let fresh247 = i;
        i = i + 1;
        info[fresh247 as usize] =
            b"You are resistant to blasts of shards.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_nexus != 0 {
        let fresh248 = i;
        i = i + 1;
        info[fresh248 as usize] =
            b"You are resistant to nexus attacks.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).immune_neth != 0 {
        let fresh249 = i;
        i = i + 1;
        info[fresh249 as usize] =
            b"You are immune to nether forces.\x00" as *const u8 as
                *const libc::c_char
    } else if (*p_ptr).resist_neth != 0 {
        let fresh250 = i;
        i = i + 1;
        info[fresh250 as usize] =
            b"You are resistant to nether forces.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_fear != 0 {
        let fresh251 = i;
        i = i + 1;
        info[fresh251 as usize] =
            b"You are completely fearless.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_blind != 0 {
        let fresh252 = i;
        i = i + 1;
        info[fresh252 as usize] =
            b"Your eyes are resistant to blindness.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).resist_continuum != 0 {
        let fresh253 = i;
        i = i + 1;
        info[fresh253 as usize] =
            b"The space-time continuum cannot be disrupted near you.\x00" as
                *const u8 as *const libc::c_char
    }
    if (*p_ptr).sustain_str != 0 {
        let fresh254 = i;
        i = i + 1;
        info[fresh254 as usize] =
            b"Your strength is sustained.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).sustain_int != 0 {
        let fresh255 = i;
        i = i + 1;
        info[fresh255 as usize] =
            b"Your intelligence is sustained.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).sustain_wis != 0 {
        let fresh256 = i;
        i = i + 1;
        info[fresh256 as usize] =
            b"Your wisdom is sustained.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).sustain_con != 0 {
        let fresh257 = i;
        i = i + 1;
        info[fresh257 as usize] =
            b"Your constitution is sustained.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).sustain_dex != 0 {
        let fresh258 = i;
        i = i + 1;
        info[fresh258 as usize] =
            b"Your dexterity is sustained.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).sustain_chr != 0 {
        let fresh259 = i;
        i = i + 1;
        info[fresh259 as usize] =
            b"Your charisma is sustained.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).black_breath != 0 {
        let fresh260 = i;
        i = i + 1;
        info[fresh260 as usize] =
            b"You suffer from Black Breath.\x00" as *const u8 as
                *const libc::c_char
    }
    if f1 as libc::c_long & 0x1 as libc::c_long != 0 {
        let fresh261 = i;
        i = i + 1;
        info[fresh261 as usize] =
            b"Your strength is affected by your equipment.\x00" as *const u8
                as *const libc::c_char
    }
    if f1 as libc::c_long & 0x2 as libc::c_long != 0 {
        let fresh262 = i;
        i = i + 1;
        info[fresh262 as usize] =
            b"Your intelligence is affected by your equipment.\x00" as
                *const u8 as *const libc::c_char
    }
    if f1 as libc::c_long & 0x4 as libc::c_long != 0 {
        let fresh263 = i;
        i = i + 1;
        info[fresh263 as usize] =
            b"Your wisdom is affected by your equipment.\x00" as *const u8 as
                *const libc::c_char
    }
    if f1 as libc::c_long & 0x8 as libc::c_long != 0 {
        let fresh264 = i;
        i = i + 1;
        info[fresh264 as usize] =
            b"Your dexterity is affected by your equipment.\x00" as *const u8
                as *const libc::c_char
    }
    if f1 as libc::c_long & 0x10 as libc::c_long != 0 {
        let fresh265 = i;
        i = i + 1;
        info[fresh265 as usize] =
            b"Your constitution is affected by your equipment.\x00" as
                *const u8 as *const libc::c_char
    }
    if f1 as libc::c_long & 0x20 as libc::c_long != 0 {
        let fresh266 = i;
        i = i + 1;
        info[fresh266 as usize] =
            b"Your charisma is affected by your equipment.\x00" as *const u8
                as *const libc::c_char
    }
    if f1 as libc::c_long & 0x100 as libc::c_long != 0 {
        let fresh267 = i;
        i = i + 1;
        info[fresh267 as usize] =
            b"Your stealth is affected by your equipment.\x00" as *const u8 as
                *const libc::c_char
    }
    if f1 as libc::c_long & 0x200 as libc::c_long != 0 {
        let fresh268 = i;
        i = i + 1;
        info[fresh268 as usize] =
            b"Your searching ability is affected by your equipment.\x00" as
                *const u8 as *const libc::c_char
    }
    if f1 as libc::c_long & 0x400 as libc::c_long != 0 {
        let fresh269 = i;
        i = i + 1;
        info[fresh269 as usize] =
            b"Your infravision is affected by your equipment.\x00" as
                *const u8 as *const libc::c_char
    }
    if f1 as libc::c_long & 0x800 as libc::c_long != 0 {
        let fresh270 = i;
        i = i + 1;
        info[fresh270 as usize] =
            b"Your digging ability is affected by your equipment.\x00" as
                *const u8 as *const libc::c_char
    }
    if f1 as libc::c_long & 0x1000 as libc::c_long != 0 {
        let fresh271 = i;
        i = i + 1;
        info[fresh271 as usize] =
            b"Your speed is affected by your equipment.\x00" as *const u8 as
                *const libc::c_char
    }
    if f1 as libc::c_long & 0x2000 as libc::c_long != 0 {
        let fresh272 = i;
        i = i + 1;
        info[fresh272 as usize] =
            b"Your attack speed is affected by your equipment.\x00" as
                *const u8 as *const libc::c_char
    }
    if f5 as libc::c_long & 0x20 as libc::c_long != 0 {
        let fresh273 = i;
        i = i + 1;
        info[fresh273 as usize] =
            b"Your ability to score critical hits is affected by your equipment.\x00"
                as *const u8 as *const libc::c_char
    }
    /* Access the current weapon */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Analyze the weapon */
    if (*o_ptr).k_idx != 0 {
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        /* Indicate Blessing */
        if f3 as libc::c_long & 0x10000000 as libc::c_long != 0 {
            let fresh274 = i;
            i = i + 1;
            info[fresh274 as usize] =
                b"Your weapon has been blessed by the gods.\x00" as *const u8
                    as *const libc::c_char
        }
        if f1 as libc::c_long & 0x4000 as libc::c_long != 0 {
            let fresh275 = i;
            i = i + 1;
            info[fresh275 as usize] =
                b"Your weapon is branded with the Sign of Chaos.\x00" as
                    *const u8 as *const libc::c_char
        }
        /* Hack */
        if f1 as libc::c_long & 0x4000000 as libc::c_long != 0 {
            let fresh276 = i;
            i = i + 1;
            info[fresh276 as usize] =
                b"The impact of your weapon can cause earthquakes.\x00" as
                    *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x2000000 as libc::c_long != 0 {
            let fresh277 = i;
            i = i + 1;
            info[fresh277 as usize] =
                b"Your weapon is very sharp.\x00" as *const u8 as
                    *const libc::c_char
        }
        if f1 as libc::c_long & 0x8000 as libc::c_long != 0 {
            let fresh278 = i;
            i = i + 1;
            info[fresh278 as usize] =
                b"Your weapon drains life from your foes.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Special "Attack Bonuses" */
        if f1 as libc::c_long & 0x10000000 as libc::c_long != 0 {
            let fresh279 = i;
            i = i + 1;
            info[fresh279 as usize] =
                b"Your weapon melts your foes.\x00" as *const u8 as
                    *const libc::c_char
        }
        if f1 as libc::c_long & 0x20000000 as libc::c_long != 0 {
            let fresh280 = i;
            i = i + 1;
            info[fresh280 as usize] =
                b"Your weapon shocks your foes.\x00" as *const u8 as
                    *const libc::c_char
        }
        if f1 as libc::c_long & 0x40000000 as libc::c_long != 0 {
            let fresh281 = i;
            i = i + 1;
            info[fresh281 as usize] =
                b"Your weapon burns your foes.\x00" as *const u8 as
                    *const libc::c_char
        }
        if f1 as libc::c_long & 0x80000000 as libc::c_long != 0 {
            let fresh282 = i;
            i = i + 1;
            info[fresh282 as usize] =
                b"Your weapon freezes your foes.\x00" as *const u8 as
                    *const libc::c_char
        }
        if f1 as libc::c_long & 0x8000000 as libc::c_long != 0 {
            let fresh283 = i;
            i = i + 1;
            info[fresh283 as usize] =
                b"Your weapon poisons your foes.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Special "slay" flags */
        if f1 as libc::c_long & 0x10000 as libc::c_long != 0 {
            let fresh284 = i;
            i = i + 1;
            info[fresh284 as usize] =
                b"Your weapon strikes at animals with extra force.\x00" as
                    *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x20000 as libc::c_long != 0 {
            let fresh285 = i;
            i = i + 1;
            info[fresh285 as usize] =
                b"Your weapon strikes at evil with extra force.\x00" as
                    *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x40000 as libc::c_long != 0 {
            let fresh286 = i;
            i = i + 1;
            info[fresh286 as usize] =
                b"Your weapon strikes at undead with holy wrath.\x00" as
                    *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x80000 as libc::c_long != 0 {
            let fresh287 = i;
            i = i + 1;
            info[fresh287 as usize] =
                b"Your weapon strikes at demons with holy wrath.\x00" as
                    *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x100000 as libc::c_long != 0 {
            let fresh288 = i;
            i = i + 1;
            info[fresh288 as usize] =
                b"Your weapon is especially deadly against orcs.\x00" as
                    *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x200000 as libc::c_long != 0 {
            let fresh289 = i;
            i = i + 1;
            info[fresh289 as usize] =
                b"Your weapon is especially deadly against trolls.\x00" as
                    *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x400000 as libc::c_long != 0 {
            let fresh290 = i;
            i = i + 1;
            info[fresh290 as usize] =
                b"Your weapon is especially deadly against giants.\x00" as
                    *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x800000 as libc::c_long != 0 {
            let fresh291 = i;
            i = i + 1;
            info[fresh291 as usize] =
                b"Your weapon is especially deadly against dragons.\x00" as
                    *const u8 as *const libc::c_char
        }
        /* Special "kill" flags */
        if f1 as libc::c_long & 0x1000000 as libc::c_long != 0 {
            let fresh292 = i;
            i = i + 1;
            info[fresh292 as usize] =
                b"Your weapon is a great bane of dragons.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Special "kill" flags */
        if f5 as libc::c_long & 0x8 as libc::c_long != 0 {
            let fresh293 = i;
            i = i + 1;
            info[fresh293 as usize] =
                b"Your weapon is a great bane of demons.\x00" as *const u8 as
                    *const libc::c_char
        }
        /* Special "kill" flags */
        if f5 as libc::c_long & 0x10 as libc::c_long != 0 {
            let fresh294 = i;
            i = i + 1;
            info[fresh294 as usize] =
                b"Your weapon is a great bane of undeads.\x00" as *const u8 as
                    *const libc::c_char
        }
    }
    /* Print on screen or in a file ? */
    if fff.is_null() {
        /* Save the screen */
        character_icky = 1 as libc::c_int as bool_;
        Term_save();
        /* Erase the screen */
        k = 1 as libc::c_int;
        while k < 24 as libc::c_int {
            prt(b"\x00" as *const u8 as *const libc::c_char, k,
                13 as libc::c_int);
            k += 1
        }
        /* Label the information */
        prt(b"     Your Attributes:\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int, 15 as libc::c_int);
        /* We will print on top of the map (column 13) */
        k = 2 as libc::c_int;
        j = 0 as libc::c_int;
        while j < i {
            /* Show the info */
            let fresh295 = k;
            k = k + 1;
            prt(info[j as usize], fresh295, 15 as libc::c_int);
            /* Every 20 entries (lines 2 to 21), start over */
            if k == 22 as libc::c_int && (j + 1 as libc::c_int) < i {
                prt(b"-- more --\x00" as *const u8 as *const libc::c_char, k,
                    15 as libc::c_int);
                inkey();
                while k > 2 as libc::c_int {
                    prt(b"\x00" as *const u8 as *const libc::c_char, k,
                        15 as libc::c_int);
                    k -= 1
                }
            }
            j += 1
        }
        /* Pause */
        prt(b"[Press any key to continue]\x00" as *const u8 as
                *const libc::c_char, k, 13 as libc::c_int);
        inkey();
        /* Restore the screen */
        Term_load();
        character_icky = 0 as libc::c_int as bool_
    } else {
        /* Label the information */
        fprintf(fff,
                b"     Your Attributes:\n\x00" as *const u8 as
                    *const libc::c_char);
        /* We will print on top of the map (column 13) */
        j = 0 as libc::c_int;
        while j < i {
            /* Show the info */
            fprintf(fff, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    info[j as usize]);
            j += 1
        }
    };
}
unsafe extern "C" fn report_magics_aux(mut dur: libc::c_int) -> libc::c_int {
    if dur <= 5 as libc::c_int {
        return 0 as libc::c_int
    } else if dur <= 10 as libc::c_int {
        return 1 as libc::c_int
    } else if dur <= 20 as libc::c_int {
        return 2 as libc::c_int
    } else if dur <= 50 as libc::c_int {
        return 3 as libc::c_int
    } else if dur <= 100 as libc::c_int {
        return 4 as libc::c_int
    } else if dur <= 200 as libc::c_int {
        return 5 as libc::c_int
    } else { return 6 as libc::c_int };
}
static mut report_magic_durations: [cptr; 8] =
    [b"for a short time\x00" as *const u8 as *const libc::c_char,
     b"for a little while\x00" as *const u8 as *const libc::c_char,
     b"for a while\x00" as *const u8 as *const libc::c_char,
     b"for a long while\x00" as *const u8 as *const libc::c_char,
     b"for a long time\x00" as *const u8 as *const libc::c_char,
     b"for a very long time\x00" as *const u8 as *const libc::c_char,
     b"for an incredibly long time\x00" as *const u8 as *const libc::c_char,
     b"until you hit a monster\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn report_magics() {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut Dummy: [libc::c_char; 80] = [0; 80];
    let mut info: [cptr; 128] = [0 as *const libc::c_char; 128];
    let mut info2: [libc::c_int; 128] = [0; 128];
    if (*p_ptr).blind != 0 {
        info2[i as usize] = report_magics_aux((*p_ptr).blind as libc::c_int);
        let fresh296 = i;
        i = i + 1;
        info[fresh296 as usize] =
            b"You cannot see\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).confused != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).confused as libc::c_int);
        let fresh297 = i;
        i = i + 1;
        info[fresh297 as usize] =
            b"You are confused\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).afraid != 0 {
        info2[i as usize] = report_magics_aux((*p_ptr).afraid as libc::c_int);
        let fresh298 = i;
        i = i + 1;
        info[fresh298 as usize] =
            b"You are terrified\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).poisoned != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).poisoned as libc::c_int);
        let fresh299 = i;
        i = i + 1;
        info[fresh299 as usize] =
            b"You are poisoned\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).image != 0 {
        info2[i as usize] = report_magics_aux((*p_ptr).image as libc::c_int);
        let fresh300 = i;
        i = i + 1;
        info[fresh300 as usize] =
            b"You are hallucinating\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).blessed != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).blessed as libc::c_int);
        let fresh301 = i;
        i = i + 1;
        info[fresh301 as usize] =
            b"You feel righteous\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).hero != 0 {
        info2[i as usize] = report_magics_aux((*p_ptr).hero as libc::c_int);
        let fresh302 = i;
        i = i + 1;
        info[fresh302 as usize] =
            b"You feel heroic\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).shero != 0 {
        info2[i as usize] = report_magics_aux((*p_ptr).shero as libc::c_int);
        let fresh303 = i;
        i = i + 1;
        info[fresh303 as usize] =
            b"You are in a battle rage\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).protevil != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).protevil as libc::c_int);
        let fresh304 = i;
        i = i + 1;
        info[fresh304 as usize] =
            b"You are protected from evil\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).protgood != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).protgood as libc::c_int);
        let fresh305 = i;
        i = i + 1;
        info[fresh305 as usize] =
            b"You are protected from good\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).shield != 0 {
        info2[i as usize] = report_magics_aux((*p_ptr).shield as libc::c_int);
        let fresh306 = i;
        i = i + 1;
        info[fresh306 as usize] =
            b"You are protected by a mystic shield\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).invuln != 0 {
        info2[i as usize] = report_magics_aux((*p_ptr).invuln as libc::c_int);
        let fresh307 = i;
        i = i + 1;
        info[fresh307 as usize] =
            b"You are invulnerable\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).tim_wraith != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).tim_wraith as libc::c_int);
        let fresh308 = i;
        i = i + 1;
        info[fresh308 as usize] =
            b"You are incorporeal\x00" as *const u8 as *const libc::c_char
    }
    if (*p_ptr).confusing != 0 {
        info2[i as usize] = 7 as libc::c_int;
        let fresh309 = i;
        i = i + 1;
        info[fresh309 as usize] =
            b"Your hands are glowing dull red.\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).word_recall != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).word_recall as libc::c_int);
        let fresh310 = i;
        i = i + 1;
        info[fresh310 as usize] =
            b"You waiting to be recalled\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).oppose_acid != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).oppose_acid as libc::c_int);
        let fresh311 = i;
        i = i + 1;
        info[fresh311 as usize] =
            b"You are resistant to acid\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).oppose_elec != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).oppose_elec as libc::c_int);
        let fresh312 = i;
        i = i + 1;
        info[fresh312 as usize] =
            b"You are resistant to lightning\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).oppose_fire != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).oppose_fire as libc::c_int);
        let fresh313 = i;
        i = i + 1;
        info[fresh313 as usize] =
            b"You are resistant to fire\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).oppose_cold != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).oppose_cold as libc::c_int);
        let fresh314 = i;
        i = i + 1;
        info[fresh314 as usize] =
            b"You are resistant to cold\x00" as *const u8 as
                *const libc::c_char
    }
    if (*p_ptr).oppose_pois != 0 {
        info2[i as usize] =
            report_magics_aux((*p_ptr).oppose_pois as libc::c_int);
        let fresh315 = i;
        i = i + 1;
        info[fresh315 as usize] =
            b"You are resistant to poison\x00" as *const u8 as
                *const libc::c_char
    }
    /* Save the screen */
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    /* Erase the screen */
    k = 1 as libc::c_int;
    while k < 24 as libc::c_int {
        prt(b"\x00" as *const u8 as *const libc::c_char, k,
            13 as libc::c_int);
        k += 1
    }
    /* Label the information */
    prt(b"     Your Current Magic:\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int, 15 as libc::c_int);
    /* We will print on top of the map (column 13) */
    k = 2 as libc::c_int;
    j = 0 as libc::c_int;
    while j < i {
        /* Show the info */
        sprintf(Dummy.as_mut_ptr(),
                b"%s %s.\x00" as *const u8 as *const libc::c_char,
                info[j as usize],
                report_magic_durations[info2[j as usize] as usize]);
        let fresh316 = k;
        k = k + 1;
        prt(Dummy.as_mut_ptr() as cptr, fresh316, 15 as libc::c_int);
        /* Every 20 entries (lines 2 to 21), start over */
        if k == 22 as libc::c_int && (j + 1 as libc::c_int) < i {
            prt(b"-- more --\x00" as *const u8 as *const libc::c_char, k,
                15 as libc::c_int);
            inkey();
            while k > 2 as libc::c_int {
                prt(b"\x00" as *const u8 as *const libc::c_char, k,
                    15 as libc::c_int);
                k -= 1
            }
        }
        j += 1
    }
    /* Pause */
    prt(b"[Press any key to continue]\x00" as *const u8 as
            *const libc::c_char, k, 13 as libc::c_int);
    inkey();
    /* Restore the screen */
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
}
/*
 * Forget everything
 */
#[no_mangle]
pub unsafe extern "C" fn lose_all_info() -> bool_ {
    let mut i: libc::c_int = 0;
    /* Forget info about objects */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Allow "protection" by the MENTAL flag */
            if !((*o_ptr).ident as libc::c_int & 0x20 as libc::c_int != 0) {
                /* Remove sensing */
                (*o_ptr).sense = 0 as libc::c_int as byte_hack;
                /* Hack -- Clear the "empty" flag */
                (*o_ptr).ident =
                    ((*o_ptr).ident as libc::c_int & !(0x4 as libc::c_int)) as
                        byte_hack;
                /* Hack -- Clear the "known" flag */
                (*o_ptr).ident =
                    ((*o_ptr).ident as libc::c_int & !(0x8 as libc::c_int)) as
                        byte_hack;
                /* Hack -- Clear the "felt" flag */
                (*o_ptr).ident =
                    ((*o_ptr).ident as libc::c_int & !(0x1 as libc::c_int)) as
                        byte_hack
            }
        }
        i += 1
    }
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
                  0x8 as libc::c_long)) as u32b;
    /* Mega-Hack -- Forget the map */
    wiz_dark();
    /* It worked */
    return 1 as libc::c_int as bool_;
}
/*
 * Detect all traps on current panel
 */
#[no_mangle]
pub unsafe extern "C" fn detect_traps(mut rad: libc::c_int) -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut detect: bool_ = 0 as libc::c_int as bool_;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Scan the current panel */
    y = (*p_ptr).py as libc::c_int - rad;
    while y <= (*p_ptr).py as libc::c_int + rad {
        x = (*p_ptr).px as libc::c_int - rad;
        while x <= (*p_ptr).px as libc::c_int + rad {
            /* Reject locations outside of dungeon */
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Reject those out of radius */
                if !(distance((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int, y, x) > rad) {
                    /* Access the grid */
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset(y as
                                                             isize)).offset(x
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* mark as detected */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x1000 as libc::c_int)
                            as u16b;
                    /* Detect invisible traps */
                    if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
                        /* Hack -- Remember detected traps */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int |
                                 0x1 as libc::c_int) as u16b;
                        /* Pick a trap */
                        pick_trap(y, x);
                        /* Obvious */
                        detect = 1 as libc::c_int as bool_
                    }
                }
            }
            x += 1
        }
        y += 1
    }
    /* Describe */
    if detect != 0 {
        msg_print(b"You sense the presence of traps!\x00" as *const u8 as
                      *const libc::c_char);
    }
    /*
	 * This reveals un-identified trap detection items,
	 * but so does leaving/entering trap-detected areas...
	 * There are a couple of possible solutions:
	 * (1) Immediately self-id such items (i.e. always returns TRUE)
	 * (2) add another parameter to function which tells if unaware
	 * item is used for trap detection, and if it is the case,
	 * do two-pass scanning, first scanning for traps if an unaware
	 * item is used and return FALSE there are none,
	 * followed by current implementation --pelpel
	 */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x20000000 as libc::c_long) as
            u32b;
    /* Result -- see my comment above -- pelpel */
	/* return (detect); */
    return 1 as libc::c_int as bool_;
}
/*
 * Detect all doors on current panel
 */
#[no_mangle]
pub unsafe extern "C" fn detect_doors(mut rad: libc::c_int) -> bool_ {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut detect: bool_ = 0 as libc::c_int as bool_;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Scan the panel */
    y = (*p_ptr).py as libc::c_int - rad;
    while y <= (*p_ptr).py as libc::c_int + rad {
        x = (*p_ptr).px as libc::c_int - rad;
        while x <= (*p_ptr).px as libc::c_int + rad {
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                if !(distance((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int, y, x) > rad) {
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset(y as
                                                             isize)).offset(x
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Detect secret doors */
                    if (*c_ptr).feat as libc::c_int == 0x30 as libc::c_int {
                        /* Remove feature mimics */
                        (*cave[y as usize].offset(x as isize)).mimic =
                            0 as libc::c_int as byte_hack;
                        /* Pick a door XXX XXX XXX */
                        cave_set_feat(y, x,
                                      0x20 as libc::c_int + 0 as libc::c_int);
                    }
                    /* Detect doors */
                    if (*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
                           (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int
                           ||
                           ((*c_ptr).feat as libc::c_int == 0x4 as libc::c_int
                                ||
                                (*c_ptr).feat as libc::c_int ==
                                    0x5 as libc::c_int) {
                        /* Hack -- Memorize */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int |
                                 0x1 as libc::c_int) as u16b;
                        /* Reveal it */
				/* c_ptr->mimic = 0; */
                        /* Redraw */
                        lite_spot(y, x);
                        /* Obvious */
                        detect = 1 as libc::c_int as bool_
                    }
                }
            }
            x += 1
        }
        y += 1
    }
    /* Describe */
    if detect != 0 {
        msg_print(b"You sense the presence of doors!\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Result */
    return detect;
}
/*
 * Detect all stairs on current panel
 */
#[no_mangle]
pub unsafe extern "C" fn detect_stairs(mut rad: libc::c_int) -> bool_ {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut detect: bool_ = 0 as libc::c_int as bool_;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Scan the panel */
    y = (*p_ptr).py as libc::c_int - rad;
    while y <= (*p_ptr).py as libc::c_int + rad {
        x = (*p_ptr).px as libc::c_int - rad;
        while x <= (*p_ptr).px as libc::c_int + rad {
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                if !(distance((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int, y, x) > rad) {
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset(y as
                                                             isize)).offset(x
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Detect stairs */
                    if (*c_ptr).feat as libc::c_int == 0x6 as libc::c_int ||
                           (*c_ptr).feat as libc::c_int == 0x7 as libc::c_int
                           ||
                           (*c_ptr).feat as libc::c_int == 0xd as libc::c_int
                           ||
                           (*c_ptr).feat as libc::c_int == 0xe as libc::c_int
                           ||
                           (*c_ptr).feat as libc::c_int == 0xb4 as libc::c_int
                           ||
                           (*c_ptr).feat as libc::c_int == 0xb3 as libc::c_int
                       {
                        /* Hack -- Memorize */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int |
                                 0x1 as libc::c_int) as u16b;
                        /* Redraw */
                        lite_spot(y, x);
                        /* Obvious */
                        detect = 1 as libc::c_int as bool_
                    }
                }
            }
            x += 1
        }
        y += 1
    }
    /* Describe */
    if detect != 0 {
        msg_print(b"You sense the presence of ways out of this area!\x00" as
                      *const u8 as *const libc::c_char);
    }
    /* Result */
    return detect;
}
/*
 * Detect any treasure on the current panel
 */
#[no_mangle]
pub unsafe extern "C" fn detect_treasure(mut rad: libc::c_int) -> bool_ {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut detect: bool_ = 0 as libc::c_int as bool_;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Scan the current panel */
    y = (*p_ptr).py as libc::c_int - rad;
    while y <= (*p_ptr).py as libc::c_int + rad {
        x = (*p_ptr).px as libc::c_int - rad;
        while x <= (*p_ptr).px as libc::c_int + rad {
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                if !(distance((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int, y, x) > rad) {
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset(y as
                                                             isize)).offset(x
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Notice embedded gold */
                    if (*c_ptr).feat as libc::c_int == 0x34 as libc::c_int ||
                           (*c_ptr).feat as libc::c_int == 0x35 as libc::c_int
                       {
                        /* Expose the gold */
                        cave_set_feat(y, x,
                                      (*c_ptr).feat as libc::c_int +
                                          0x2 as libc::c_int);
                    } else if (*c_ptr).feat as libc::c_int ==
                                  0x63 as libc::c_int {
                        /* Expose the gold */
                        cave_set_feat(y, x, 0x64 as libc::c_int);
                    }
                    /* Magma/Quartz + Known Gold */
                    if (*c_ptr).feat as libc::c_int == 0x36 as libc::c_int ||
                           (*c_ptr).feat as libc::c_int == 0x37 as libc::c_int
                           ||
                           (*c_ptr).feat as libc::c_int == 0x64 as libc::c_int
                       {
                        /* Hack -- Memorize */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int |
                                 0x1 as libc::c_int) as u16b;
                        /* Redraw */
                        lite_spot(y, x);
                        /* Detect */
                        detect = 1 as libc::c_int as bool_
                    }
                }
            }
            x += 1
        }
        y += 1
    }
    /* Describe */
    if detect != 0 {
        msg_print(b"You sense the presence of buried treasure!\x00" as
                      *const u8 as *const libc::c_char);
    }
    /* Result */
    return detect;
}
/*
 * Detect all "gold" objects on the current panel
 */
#[no_mangle]
pub static mut hack_no_detect_message: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub unsafe extern "C" fn detect_objects_gold(mut rad: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut detect: bool_ = 0 as libc::c_int as bool_;
    let mut current_block_12: u64;
    /* Scan objects */
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *o_list.offset(i as isize) as *mut object_type;
        /* Skip dead objects */
        if !((*o_ptr).k_idx == 0) {
            /* Skip held objects */
            if (*o_ptr).held_m_idx != 0 {
                /* Access the monster */
                let mut m_ptr: *mut monster_type =
                    &mut *m_list.offset((*o_ptr).held_m_idx as isize) as
                        *mut monster_type;
                let mut r_ptr: *mut monster_race =
                    if !(*m_ptr).sr_ptr.is_null() {
                        (*m_ptr).sr_ptr
                    } else {
                        race_info_idx((*m_ptr).r_idx as libc::c_int,
                                      (*m_ptr).ego as libc::c_int)
                    };
                if (*r_ptr).flags9 & 0x8 as libc::c_int as libc::c_uint == 0 {
                    current_block_12 = 16559507199688588974;
                } else {
                    /* Location */
                    y = (*m_ptr).fy as libc::c_int;
                    x = (*m_ptr).fx as libc::c_int;
                    current_block_12 = 3512920355445576850;
                }
            } else {
                /* Location */
                y = (*o_ptr).iy as libc::c_int;
                x = (*o_ptr).ix as libc::c_int;
                current_block_12 = 3512920355445576850;
            }
            match current_block_12 {
                16559507199688588974 => { }
                _ =>
                /* Only detect nearby objects */
                {
                    if !(distance((*p_ptr).py as libc::c_int,
                                  (*p_ptr).px as libc::c_int, y, x) > rad) {
                        /* Detect "gold" objects */
                        if (*o_ptr).tval as libc::c_int == 100 as libc::c_int
                           {
                            /* Hack -- memorize it */
                            (*o_ptr).marked = 1 as libc::c_int as byte_hack;
                            /* Redraw */
                            if y >= panel_row_min as libc::c_int &&
                                   y <= panel_row_max as libc::c_int &&
                                   x >= panel_col_min as libc::c_int &&
                                   x <= panel_col_max as libc::c_int {
                                lite_spot(y, x);
                            }
                            /* Detect */
                            detect = 1 as libc::c_int as bool_
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Describe */
    if detect as libc::c_int != 0 && hack_no_detect_message == 0 {
        msg_print(b"You sense the presence of treasure!\x00" as *const u8 as
                      *const libc::c_char);
    }
    if detect_monsters_string(b"$\x00" as *const u8 as *const libc::c_char,
                              rad) != 0 {
        detect = 1 as libc::c_int as bool_
    }
    /* Result */
    return detect;
}
/*
 * Detect all "normal" objects on the current panel
 */
#[no_mangle]
pub unsafe extern "C" fn detect_objects_normal(mut rad: libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut detect: bool_ = 0 as libc::c_int as bool_;
    let mut current_block_12: u64;
    /* Scan objects */
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *o_list.offset(i as isize) as *mut object_type;
        /* Skip dead objects */
        if !((*o_ptr).k_idx == 0) {
            /* Skip held objects */
            if (*o_ptr).held_m_idx != 0 {
                /* Access the monster */
                let mut m_ptr: *mut monster_type =
                    &mut *m_list.offset((*o_ptr).held_m_idx as isize) as
                        *mut monster_type;
                let mut r_ptr: *mut monster_race =
                    if !(*m_ptr).sr_ptr.is_null() {
                        (*m_ptr).sr_ptr
                    } else {
                        race_info_idx((*m_ptr).r_idx as libc::c_int,
                                      (*m_ptr).ego as libc::c_int)
                    };
                if (*r_ptr).flags9 & 0x8 as libc::c_int as libc::c_uint == 0 {
                    current_block_12 = 16559507199688588974;
                } else {
                    /* Location */
                    y = (*m_ptr).fy as libc::c_int;
                    x = (*m_ptr).fx as libc::c_int;
                    current_block_12 = 3512920355445576850;
                }
            } else {
                /* Location */
                y = (*o_ptr).iy as libc::c_int;
                x = (*o_ptr).ix as libc::c_int;
                current_block_12 = 3512920355445576850;
            }
            match current_block_12 {
                16559507199688588974 => { }
                _ =>
                /* Only detect nearby objects */
                {
                    if !(distance((*p_ptr).py as libc::c_int,
                                  (*p_ptr).px as libc::c_int, y, x) > rad) {
                        /* Detect "real" objects */
                        if (*o_ptr).tval as libc::c_int != 100 as libc::c_int
                           {
                            /* Hack -- memorize it */
                            (*o_ptr).marked = 1 as libc::c_int as byte_hack;
                            /* Redraw */
                            if y >= panel_row_min as libc::c_int &&
                                   y <= panel_row_max as libc::c_int &&
                                   x >= panel_col_min as libc::c_int &&
                                   x <= panel_col_max as libc::c_int {
                                lite_spot(y, x);
                            }
                            /* Detect */
                            detect = 1 as libc::c_int as bool_
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Describe */
    if detect as libc::c_int != 0 && hack_no_detect_message == 0 {
        msg_print(b"You sense the presence of objects!\x00" as *const u8 as
                      *const libc::c_char);
    }
    if detect_monsters_string(b"!=?|\x00" as *const u8 as *const libc::c_char,
                              rad) != 0 {
        detect = 1 as libc::c_int as bool_
    }
    /* Result */
    return detect;
}
/*
 * Detect all "magic" objects on the current panel.
 *
 * This will light up all spaces with "magic" items, including artifacts,
 * ego-items, potions, scrolls, books, rods, wands, staves, amulets, rings,
 * and "enchanted" items of the "good" variety.
 *
 * It can probably be argued that this function is now too powerful.
 */
#[no_mangle]
pub unsafe extern "C" fn detect_objects_magic(mut rad: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut tv: libc::c_int = 0;
    let mut detect: bool_ = 0 as libc::c_int as bool_;
    let mut current_block_13: u64;
    /* Scan all objects */
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *o_list.offset(i as isize) as *mut object_type;
        /* Skip dead objects */
        if !((*o_ptr).k_idx == 0) {
            /* Skip held objects */
            if (*o_ptr).held_m_idx != 0 {
                /* Access the monster */
                let mut m_ptr: *mut monster_type =
                    &mut *m_list.offset((*o_ptr).held_m_idx as isize) as
                        *mut monster_type;
                let mut r_ptr: *mut monster_race =
                    if !(*m_ptr).sr_ptr.is_null() {
                        (*m_ptr).sr_ptr
                    } else {
                        race_info_idx((*m_ptr).r_idx as libc::c_int,
                                      (*m_ptr).ego as libc::c_int)
                    };
                if (*r_ptr).flags9 & 0x8 as libc::c_int as libc::c_uint == 0 {
                    current_block_13 = 16559507199688588974;
                } else {
                    /* Location */
                    y = (*m_ptr).fy as libc::c_int;
                    x = (*m_ptr).fx as libc::c_int;
                    current_block_13 = 3512920355445576850;
                }
            } else {
                /* Location */
                y = (*o_ptr).iy as libc::c_int;
                x = (*o_ptr).ix as libc::c_int;
                current_block_13 = 3512920355445576850;
            }
            match current_block_13 {
                16559507199688588974 => { }
                _ =>
                /* Only detect nearby objects */
                {
                    if !(distance((*p_ptr).py as libc::c_int,
                                  (*p_ptr).px as libc::c_int, y, x) > rad) {
                        /* Examine the tval */
                        tv = (*o_ptr).tval as libc::c_int;
                        /* Artifacts, misc magic items, or enchanted wearables */
                        if (*o_ptr).tval as libc::c_int == 102 as libc::c_int
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
                                } else { 0 as libc::c_int }) != 0 ||
                               (if (*o_ptr).name2 as libc::c_int != 0 ||
                                       (*o_ptr).name2b as libc::c_int != 0 {
                                    1 as libc::c_int
                                } else { 0 as libc::c_int }) != 0 ||
                               (*o_ptr).art_name as libc::c_int != 0 ||
                               tv == 40 as libc::c_int ||
                               tv == 45 as libc::c_int ||
                               tv == 4 as libc::c_int ||
                               tv == 55 as libc::c_int ||
                               tv == 65 as libc::c_int ||
                               tv == 66 as libc::c_int ||
                               tv == 67 as libc::c_int ||
                               tv == 70 as libc::c_int ||
                               tv == 71 as libc::c_int ||
                               tv == 72 as libc::c_int ||
                               tv == 115 as libc::c_int ||
                               tv == 112 as libc::c_int ||
                               tv == 113 as libc::c_int ||
                               ((*o_ptr).to_a as libc::c_int >
                                    0 as libc::c_int ||
                                    (*o_ptr).to_h as libc::c_int +
                                        (*o_ptr).to_d as libc::c_int >
                                        0 as libc::c_int) {
                            /* Memorize the item */
                            (*o_ptr).marked = 1 as libc::c_int as byte_hack;
                            /* Redraw */
                            if y >= panel_row_min as libc::c_int &&
                                   y <= panel_row_max as libc::c_int &&
                                   x >= panel_col_min as libc::c_int &&
                                   x <= panel_col_max as libc::c_int {
                                lite_spot(y, x);
                            }
                            /* Detect */
                            detect = 1 as libc::c_int as bool_
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Describe */
    if detect != 0 {
        msg_print(b"You sense the presence of magic objects!\x00" as *const u8
                      as *const libc::c_char);
    }
    /* Return result */
    return detect;
}
/*
 * Detect all "normal" monsters on the current panel
 */
#[no_mangle]
pub unsafe extern "C" fn detect_monsters_normal(mut rad: libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    /* Scan monsters */
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
        /* Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Location */
            y = (*m_ptr).fy as libc::c_int;
            x = (*m_ptr).fx as libc::c_int;
            /* Only detect nearby monsters */
            if !((*m_ptr).cdis as libc::c_int > rad) {
                /* Detect all non-invisible monsters */
                if (*r_ptr).flags2 & 0x10 as libc::c_int as libc::c_uint == 0
                       || (*p_ptr).see_inv as libc::c_int != 0 ||
                       (*p_ptr).tim_invis as libc::c_int != 0 {
                    /* Repair visibility later */
                    repair_monsters = 1 as libc::c_int as bool_;
                    /* Hack -- Detect monster */
                    (*m_ptr).mflag |=
                        0x80 as libc::c_int | 0x40 as libc::c_int;
                    /* Hack -- See monster */
                    (*m_ptr).ml = 1 as libc::c_int as bool_;
                    /* Redraw */
                    if y >= panel_row_min as libc::c_int &&
                           y <= panel_row_max as libc::c_int &&
                           x >= panel_col_min as libc::c_int &&
                           x <= panel_col_max as libc::c_int {
                        lite_spot(y, x);
                    }
                    /* Detect */
                    flag = 1 as libc::c_int as bool_
                }
            }
        }
        i += 1
    }
    /* Describe */
    if flag != 0 {
        /* Describe result */
        msg_print(b"You sense the presence of monsters!\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Result */
    return flag;
}
/*
 * Detect all "invisible" monsters on current panel
 */
#[no_mangle]
pub unsafe extern "C" fn detect_monsters_invis(mut rad: libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    /* Scan monsters */
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
        /* Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Location */
            y = (*m_ptr).fy as libc::c_int;
            x = (*m_ptr).fx as libc::c_int;
            /* Only detect nearby monsters */
            if !((*m_ptr).cdis as libc::c_int > rad) {
                /* Detect invisible monsters */
                if (*r_ptr).flags2 & 0x10 as libc::c_int as libc::c_uint != 0
                   {
                    /* Take note that they are invisible */
                    (*r_ptr).r_flags2 |= 0x10 as libc::c_int as libc::c_uint;
                    /* Update monster recall window */
                    if monster_race_idx as libc::c_int ==
                           (*m_ptr).r_idx as libc::c_int {
                        /* Window stuff */
                        (*p_ptr).window =
                            ((*p_ptr).window as libc::c_long |
                                 0x100 as libc::c_long) as u32b
                    }
                    /* Repair visibility later */
                    repair_monsters = 1 as libc::c_int as bool_;
                    /* Hack -- Detect monster */
                    (*m_ptr).mflag |=
                        0x80 as libc::c_int | 0x40 as libc::c_int;
                    /* Hack -- See monster */
                    (*m_ptr).ml = 1 as libc::c_int as bool_;
                    /* Redraw */
                    if y >= panel_row_min as libc::c_int &&
                           y <= panel_row_max as libc::c_int &&
                           x >= panel_col_min as libc::c_int &&
                           x <= panel_col_max as libc::c_int {
                        lite_spot(y, x);
                    }
                    /* Detect */
                    flag = 1 as libc::c_int as bool_
                }
            }
        }
        i += 1
    }
    /* Describe */
    if flag != 0 {
        /* Describe result */
        msg_print(b"You sense the presence of invisible creatures!\x00" as
                      *const u8 as *const libc::c_char);
    }
    /* Result */
    return flag;
}
/*
 * Detect all "evil" monsters on current panel
 */
#[no_mangle]
pub unsafe extern "C" fn detect_monsters_evil(mut rad: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    /* Scan monsters */
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
        /* Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Location */
            y = (*m_ptr).fy as libc::c_int;
            x = (*m_ptr).fx as libc::c_int;
            /* Only detect nearby monsters */
            if !((*m_ptr).cdis as libc::c_int > rad) {
                /* Detect evil monsters */
                if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0
                   {
                    /* Take note that they are evil */
                    (*r_ptr).r_flags3 |= 0x40 as libc::c_int as libc::c_uint;
                    /* Update monster recall window */
                    if monster_race_idx as libc::c_int ==
                           (*m_ptr).r_idx as libc::c_int {
                        /* Window stuff */
                        (*p_ptr).window =
                            ((*p_ptr).window as libc::c_long |
                                 0x100 as libc::c_long) as u32b
                    }
                    /* Repair visibility later */
                    repair_monsters = 1 as libc::c_int as bool_;
                    /* Hack -- Detect monster */
                    (*m_ptr).mflag |=
                        0x80 as libc::c_int | 0x40 as libc::c_int;
                    /* Hack -- See monster */
                    (*m_ptr).ml = 1 as libc::c_int as bool_;
                    /* Redraw */
                    if y >= panel_row_min as libc::c_int &&
                           y <= panel_row_max as libc::c_int &&
                           x >= panel_col_min as libc::c_int &&
                           x <= panel_col_max as libc::c_int {
                        lite_spot(y, x);
                    }
                    /* Detect */
                    flag = 1 as libc::c_int as bool_
                }
            }
        }
        i += 1
    }
    /* Describe */
    if flag != 0 {
        /* Describe result */
        msg_print(b"You sense the presence of evil creatures!\x00" as
                      *const u8 as *const libc::c_char);
    }
    /* Result */
    return flag;
}
/*
 * Detect all (string) monsters on current panel
 */
#[no_mangle]
pub unsafe extern "C" fn detect_monsters_string(mut chars: cptr,
                                                mut rad: libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    /* Scan monsters */
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
        /* Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Location */
            y = (*m_ptr).fy as libc::c_int;
            x = (*m_ptr).fx as libc::c_int;
            /* Only detect nearby monsters */
            if !((*m_ptr).cdis as libc::c_int > rad) {
                /* Detect evil monsters */
                if !strchr(chars, (*r_ptr).d_char as libc::c_int).is_null() {
                    /* Update monster recall window */
                    if monster_race_idx as libc::c_int ==
                           (*m_ptr).r_idx as libc::c_int {
                        /* Window stuff */
                        (*p_ptr).window =
                            ((*p_ptr).window as libc::c_long |
                                 0x100 as libc::c_long) as u32b
                    }
                    /* Repair visibility later */
                    repair_monsters = 1 as libc::c_int as bool_;
                    /* Hack -- Detect monster */
                    (*m_ptr).mflag |=
                        0x80 as libc::c_int | 0x40 as libc::c_int;
                    /* Hack -- See monster */
                    (*m_ptr).ml = 1 as libc::c_int as bool_;
                    /* Redraw */
                    if y >= panel_row_min as libc::c_int &&
                           y <= panel_row_max as libc::c_int &&
                           x >= panel_col_min as libc::c_int &&
                           x <= panel_col_max as libc::c_int {
                        lite_spot(y, x);
                    }
                    /* Detect */
                    flag = 1 as libc::c_int as bool_
                }
            }
        }
        i += 1
    }
    /* Describe */
    if flag != 0 {
        /* Describe result */
        msg_print(b"You sense the presence of monsters!\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Result */
    return flag;
}
/*
 * A "generic" detect monsters routine, tagged to flags3
 */
#[no_mangle]
pub unsafe extern "C" fn detect_monsters_xxx(mut match_flag: u32b,
                                             mut rad: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    let mut desc_monsters: cptr =
        b"weird monsters\x00" as *const u8 as *const libc::c_char;
    /* Scan monsters */
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
        /* Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Location */
            y = (*m_ptr).fy as libc::c_int;
            x = (*m_ptr).fx as libc::c_int;
            /* Only detect nearby monsters */
            if !((*m_ptr).cdis as libc::c_int > rad) {
                /* Detect evil monsters */
                if (*r_ptr).flags3 & match_flag != 0 {
                    /* Take note that they are something */
                    (*r_ptr).r_flags3 |= match_flag;
                    /* Update monster recall window */
                    if monster_race_idx as libc::c_int ==
                           (*m_ptr).r_idx as libc::c_int {
                        /* Window stuff */
                        (*p_ptr).window =
                            ((*p_ptr).window as libc::c_long |
                                 0x100 as libc::c_long) as u32b
                    }
                    /* Repair visibility later */
                    repair_monsters = 1 as libc::c_int as bool_;
                    /* Hack -- Detect monster */
                    (*m_ptr).mflag |=
                        0x80 as libc::c_int | 0x40 as libc::c_int;
                    /* Hack -- See monster */
                    (*m_ptr).ml = 1 as libc::c_int as bool_;
                    /* Redraw */
                    if y >= panel_row_min as libc::c_int &&
                           y <= panel_row_max as libc::c_int &&
                           x >= panel_col_min as libc::c_int &&
                           x <= panel_col_max as libc::c_int {
                        lite_spot(y, x);
                    }
                    /* Detect */
                    flag = 1 as libc::c_int as bool_
                }
            }
        }
        i += 1
    }
    /* Describe */
    if flag != 0 {
        match match_flag {
            16 => {
                desc_monsters =
                    b"demons\x00" as *const u8 as *const libc::c_char
            }
            32 => {
                desc_monsters =
                    b"the undead\x00" as *const u8 as *const libc::c_char
            }
            512 => {
                desc_monsters =
                    b"good\x00" as *const u8 as *const libc::c_char
            }
            _ => { }
        }
        /* Describe result */
        msg_format(b"You sense the presence of %s!\x00" as *const u8 as
                       *const libc::c_char, desc_monsters);
        msg_print(0 as cptr);
    }
    /* Result */
    return flag;
}
/* Detect good monsters */
#[no_mangle]
pub unsafe extern "C" fn detect_monsters_good(mut rad: libc::c_int) -> bool_ {
    return detect_monsters_xxx(0x200 as libc::c_int as u32b, rad);
}
/*
 * Detect everything
 */
#[no_mangle]
pub unsafe extern "C" fn detect_all(mut rad: libc::c_int) -> bool_ {
    let mut detect: bool_ = 0 as libc::c_int as bool_;
    /* Detect everything */
    if detect_traps(rad) != 0 { detect = 1 as libc::c_int as bool_ }
    if detect_doors(rad) != 0 { detect = 1 as libc::c_int as bool_ }
    if detect_stairs(rad) != 0 { detect = 1 as libc::c_int as bool_ }
    if detect_treasure(rad) != 0 { detect = 1 as libc::c_int as bool_ }
    if detect_objects_gold(rad) != 0 { detect = 1 as libc::c_int as bool_ }
    if detect_objects_normal(rad) != 0 { detect = 1 as libc::c_int as bool_ }
    if detect_monsters_invis(rad) != 0 { detect = 1 as libc::c_int as bool_ }
    if detect_monsters_normal(rad) != 0 { detect = 1 as libc::c_int as bool_ }
    /* Result */
    return detect;
}
/*
 * Create stairs at the player location
 */
#[no_mangle]
pub unsafe extern "C" fn stair_creation() {
    /* XXX XXX XXX */
    if cave_valid_bold((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int)
           == 0 {
        msg_print(b"The object resists the spell.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long != 0 {
        msg_print(b"No stair creation in non dungeons...\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if dungeon_flags2 as libc::c_long & 0x40 as libc::c_long != 0 {
        msg_print(b"No stair creation on special levels...\x00" as *const u8
                      as *const libc::c_char);
        return
    }
    /* XXX XXX XXX */
    delete_object((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    /* Create a staircase */
    if (*p_ptr).inside_arena as libc::c_int != 0 ||
           (*p_ptr).inside_quest as libc::c_int != 0 {
        /* arena or quest */
        msg_print(b"There is no effect!\x00" as *const u8 as
                      *const libc::c_char);
    } else if dun_level == 0 {
        /* Town/wilderness */
        cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                      0x7 as libc::c_int);
    } else if is_quest(dun_level as libc::c_int) != 0 ||
                  dun_level as libc::c_int >=
                      128 as libc::c_int - 1 as libc::c_int {
        /* Quest level */
        cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                      0x6 as libc::c_int);
    } else if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                      0x7 as libc::c_int);
    } else {
        cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                      0x6 as libc::c_int);
    };
}
/*
 * Hook to specify "weapon"
 */
unsafe extern "C" fn item_tester_hook_weapon(mut o_ptr: *mut object_type)
 -> bool_ {
    match (*o_ptr).tval as libc::c_int {
        6 | 15 | 23 | 24 | 21 | 22 | 19 | 18 | 17 | 16 => {
            return 1 as libc::c_int as bool_
        }
        115 => {
            match (*o_ptr).sval as libc::c_int {
                55 => { return 1 as libc::c_int as bool_ }
                _ => { }
            }
        }
        _ => { }
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Hook to specify "armour"
 */
#[no_mangle]
pub unsafe extern "C" fn item_tester_hook_armour(mut o_ptr: *mut object_type)
 -> bool_ {
    match (*o_ptr).tval as libc::c_int {
        38 | 37 | 36 | 34 | 35 | 33 | 32 | 30 | 31 => {
            return 1 as libc::c_int as bool_
        }
        115 => {
            match (*o_ptr).sval as libc::c_int {
                57 | 56 => { return 1 as libc::c_int as bool_ }
                _ => { }
            }
        }
        _ => { }
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Check if an object is weapon or armour (but not arrow, bolt, or shot)
 */
#[no_mangle]
pub unsafe extern "C" fn item_tester_hook_weapon_armour(mut o_ptr:
                                                            *mut object_type)
 -> bool_ {
    return (item_tester_hook_weapon(o_ptr) as libc::c_int != 0 ||
                item_tester_hook_armour(o_ptr) as libc::c_int != 0) as
               libc::c_int as bool_;
}
/*
 * Check if an object is artifactable
 */
#[no_mangle]
pub unsafe extern "C" fn item_tester_hook_artifactable(mut o_ptr:
                                                           *mut object_type)
 -> bool_ {
    return ((item_tester_hook_weapon(o_ptr) as libc::c_int != 0 ||
                 item_tester_hook_armour(o_ptr) as libc::c_int != 0 ||
                 (*o_ptr).tval as libc::c_int == 20 as libc::c_int ||
                 (*o_ptr).tval as libc::c_int == 45 as libc::c_int ||
                 (*o_ptr).tval as libc::c_int == 40 as libc::c_int) &&
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
                (if (*o_ptr).name2 as libc::c_int != 0 ||
                        (*o_ptr).name2b as libc::c_int != 0 {
                     1 as libc::c_int
                 } else { 0 as libc::c_int }) == 0) as libc::c_int as bool_;
}
/*
 * Enchants a plus onto an item. -RAK-
 *
 * Revamped!  Now takes item pointer, number of times to try enchanting,
 * and a flag of what to try enchanting.  Artifacts resist enchantment
 * some of the time, and successful enchantment to at least +0 might
 * break a curse on the item. -CFT-
 *
 * Note that an item can technically be enchanted all the way to +15 if
 * you wait a very, very, long time.  Going from +9 to +10 only works
 * about 5% of the time, and from +10 to +11 only about 1% of the time.
 *
 * Note that this function can now be used on "piles" of items, and
 * the larger the pile, the lower the chance of success.
 */
#[no_mangle]
pub unsafe extern "C" fn enchant(mut o_ptr: *mut object_type,
                                 mut n: libc::c_int, mut eflag: libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut prob: libc::c_int = 0;
    let mut res: bool_ = 0 as libc::c_int as bool_;
    let mut a: bool_ =
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
             (*o_ptr).art_name as libc::c_int != 0) as libc::c_int as bool_;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Large piles resist enchantment */
    prob = (*o_ptr).number as libc::c_int * 100 as libc::c_int;
    /* Missiles are easy to enchant */
    if (*o_ptr).tval as libc::c_int == 18 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 17 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 16 as libc::c_int {
        prob = prob / 20 as libc::c_int
    }
    /* Try "n" times */
    i = 0 as libc::c_int;
    while i < n {
        /* Hack -- Roll for pile resistance */
        if !(Rand_div(prob) >= 100 as libc::c_int) {
            /* Enchant to hit */
            if eflag & 0x1 as libc::c_int != 0 {
                if ((*o_ptr).to_h as libc::c_int) < 0 as libc::c_int {
                    chance = 0 as libc::c_int
                } else if (*o_ptr).to_h as libc::c_int > 15 as libc::c_int {
                    chance = 1000 as libc::c_int
                } else { chance = enchant_table[(*o_ptr).to_h as usize] }
                if Rand_div(1000 as libc::c_int) + 1 as libc::c_int > chance
                       &&
                       (a == 0 ||
                            Rand_div(100 as libc::c_int) < 50 as libc::c_int)
                   {
                    (*o_ptr).to_h += 1;
                    res = 1 as libc::c_int as bool_;
                    /* only when you get it above -1 -CFT */
                    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int !=
                           0 &&
                           f3 as libc::c_long & 0x80000000 as libc::c_long ==
                               0 &&
                           (*o_ptr).to_h as libc::c_int >= 0 as libc::c_int &&
                           Rand_div(100 as libc::c_int) < 25 as libc::c_int {
                        msg_print(b"The curse is broken!\x00" as *const u8 as
                                      *const libc::c_char);
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int &
                                 !(0x40 as libc::c_int)) as byte_hack;
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x1 as libc::c_int) as byte_hack;
                        if (*o_ptr).art_flags3 as libc::c_long &
                               0x20000000 as libc::c_long != 0 {
                            (*o_ptr).art_flags3 =
                                ((*o_ptr).art_flags3 as libc::c_long &
                                     !(0x20000000 as libc::c_long)) as u32b
                        }
                        if (*o_ptr).art_flags3 as libc::c_long &
                               0x40000000 as libc::c_long != 0 {
                            (*o_ptr).art_flags3 =
                                ((*o_ptr).art_flags3 as libc::c_long &
                                     !(0x40000000 as libc::c_long)) as u32b
                        }
                        (*o_ptr).sense = 10 as libc::c_int as byte_hack
                    }
                }
            }
            /* Enchant to damage */
            if eflag & 0x2 as libc::c_int != 0 {
                if ((*o_ptr).to_d as libc::c_int) < 0 as libc::c_int {
                    chance = 0 as libc::c_int
                } else if (*o_ptr).to_d as libc::c_int > 15 as libc::c_int {
                    chance = 1000 as libc::c_int
                } else { chance = enchant_table[(*o_ptr).to_d as usize] }
                if Rand_div(1000 as libc::c_int) + 1 as libc::c_int > chance
                       &&
                       (a == 0 ||
                            Rand_div(100 as libc::c_int) < 50 as libc::c_int)
                   {
                    (*o_ptr).to_d += 1;
                    res = 1 as libc::c_int as bool_;
                    /* only when you get it above -1 -CFT */
                    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int !=
                           0 &&
                           f3 as libc::c_long & 0x80000000 as libc::c_long ==
                               0 &&
                           (*o_ptr).to_d as libc::c_int >= 0 as libc::c_int &&
                           Rand_div(100 as libc::c_int) < 25 as libc::c_int {
                        msg_print(b"The curse is broken!\x00" as *const u8 as
                                      *const libc::c_char);
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int &
                                 !(0x40 as libc::c_int)) as byte_hack;
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x1 as libc::c_int) as byte_hack;
                        if (*o_ptr).art_flags3 as libc::c_long &
                               0x20000000 as libc::c_long != 0 {
                            (*o_ptr).art_flags3 =
                                ((*o_ptr).art_flags3 as libc::c_long &
                                     !(0x20000000 as libc::c_long)) as u32b
                        }
                        if (*o_ptr).art_flags3 as libc::c_long &
                               0x40000000 as libc::c_long != 0 {
                            (*o_ptr).art_flags3 =
                                ((*o_ptr).art_flags3 as libc::c_long &
                                     !(0x40000000 as libc::c_long)) as u32b
                        }
                        (*o_ptr).sense = 10 as libc::c_int as byte_hack
                    }
                }
            }
            /* Enchant to damage */
            if eflag & 0x8 as libc::c_int != 0 {
                if (*o_ptr).pval < 0 as libc::c_int {
                    chance = 0 as libc::c_int
                } else if (*o_ptr).pval > 6 as libc::c_int {
                    chance = 1000 as libc::c_int
                } else {
                    chance =
                        enchant_table[((*o_ptr).pval * 2 as libc::c_int) as
                                          usize]
                }
                if Rand_div(1000 as libc::c_int) + 1 as libc::c_int > chance
                       &&
                       (a == 0 ||
                            Rand_div(100 as libc::c_int) < 50 as libc::c_int)
                   {
                    (*o_ptr).pval += 1;
                    res = 1 as libc::c_int as bool_;
                    /* only when you get it above -1 -CFT */
                    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int !=
                           0 &&
                           f3 as libc::c_long & 0x80000000 as libc::c_long ==
                               0 && (*o_ptr).pval >= 0 as libc::c_int &&
                           Rand_div(100 as libc::c_int) < 25 as libc::c_int {
                        msg_print(b"The curse is broken!\x00" as *const u8 as
                                      *const libc::c_char);
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int &
                                 !(0x40 as libc::c_int)) as byte_hack;
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x1 as libc::c_int) as byte_hack;
                        if (*o_ptr).art_flags3 as libc::c_long &
                               0x20000000 as libc::c_long != 0 {
                            (*o_ptr).art_flags3 =
                                ((*o_ptr).art_flags3 as libc::c_long &
                                     !(0x20000000 as libc::c_long)) as u32b
                        }
                        if (*o_ptr).art_flags3 as libc::c_long &
                               0x40000000 as libc::c_long != 0 {
                            (*o_ptr).art_flags3 =
                                ((*o_ptr).art_flags3 as libc::c_long &
                                     !(0x40000000 as libc::c_long)) as u32b
                        }
                        (*o_ptr).sense = 10 as libc::c_int as byte_hack
                    }
                }
            }
            /* Enchant to armor class */
            if eflag & 0x4 as libc::c_int != 0 {
                if ((*o_ptr).to_a as libc::c_int) < 0 as libc::c_int {
                    chance = 0 as libc::c_int
                } else if (*o_ptr).to_a as libc::c_int > 15 as libc::c_int {
                    chance = 1000 as libc::c_int
                } else { chance = enchant_table[(*o_ptr).to_a as usize] }
                if Rand_div(1000 as libc::c_int) + 1 as libc::c_int > chance
                       &&
                       (a == 0 ||
                            Rand_div(100 as libc::c_int) < 50 as libc::c_int)
                   {
                    (*o_ptr).to_a += 1;
                    res = 1 as libc::c_int as bool_;
                    /* only when you get it above -1 -CFT */
                    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int !=
                           0 &&
                           f3 as libc::c_long & 0x80000000 as libc::c_long ==
                               0 &&
                           (*o_ptr).to_a as libc::c_int >= 0 as libc::c_int &&
                           Rand_div(100 as libc::c_int) < 25 as libc::c_int {
                        msg_print(b"The curse is broken!\x00" as *const u8 as
                                      *const libc::c_char);
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int &
                                 !(0x40 as libc::c_int)) as byte_hack;
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x1 as libc::c_int) as byte_hack;
                        if (*o_ptr).art_flags3 as libc::c_long &
                               0x20000000 as libc::c_long != 0 {
                            (*o_ptr).art_flags3 =
                                ((*o_ptr).art_flags3 as libc::c_long &
                                     !(0x20000000 as libc::c_long)) as u32b
                        }
                        if (*o_ptr).art_flags3 as libc::c_long &
                               0x40000000 as libc::c_long != 0 {
                            (*o_ptr).art_flags3 =
                                ((*o_ptr).art_flags3 as libc::c_long &
                                     !(0x40000000 as libc::c_long)) as u32b
                        }
                        (*o_ptr).sense = 10 as libc::c_int as byte_hack
                    }
                }
            }
        }
        i += 1
    }
    /* Failure */
    if res == 0 { return 0 as libc::c_int as bool_ }
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
                  0x8 as libc::c_long)) as u32b;
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Enchant an item (in the inventory or on the floor)
 * Note that "num_ac" requires armour, else weapon
 * Returns TRUE if attempted, FALSE if cancelled
 */
#[no_mangle]
pub unsafe extern "C" fn enchant_spell(mut num_hit: libc::c_int,
                                       mut num_dam: libc::c_int,
                                       mut num_ac: libc::c_int,
                                       mut num_pval: libc::c_int) -> bool_ {
    let mut item: libc::c_int = 0;
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Assume enchant weapon */
    item_tester_hook =
        Some(item_tester_hook_weapon as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Enchant armor if requested */
    if num_ac != 0 {
        item_tester_hook =
            Some(item_tester_hook_armour as
                     unsafe extern "C" fn(_: *mut object_type) -> bool_)
    }
    /* Get an item */
    q = b"Enchant which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to enchant.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
           == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Description */
    object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                0 as libc::c_int);
    /* Describe */
    msg_format(b"%s %s glow%s brightly!\x00" as *const u8 as
                   *const libc::c_char,
               if item >= 0 as libc::c_int {
                   b"Your\x00" as *const u8 as *const libc::c_char
               } else { b"The\x00" as *const u8 as *const libc::c_char },
               o_name.as_mut_ptr(),
               if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                   b"\x00" as *const u8 as *const libc::c_char
               } else { b"s\x00" as *const u8 as *const libc::c_char });
    /* Enchant */
    if enchant(o_ptr, num_hit, 0x1 as libc::c_int) != 0 {
        okay = 1 as libc::c_int as bool_
    }
    if enchant(o_ptr, num_dam, 0x2 as libc::c_int) != 0 {
        okay = 1 as libc::c_int as bool_
    }
    if enchant(o_ptr, num_ac, 0x4 as libc::c_int) != 0 {
        okay = 1 as libc::c_int as bool_
    }
    if enchant(o_ptr, num_pval, 0x8 as libc::c_int) != 0 {
        okay = 1 as libc::c_int as bool_
    }
    /* Failure */
    if okay == 0 {
        /* Flush */
        if flush_failure != 0 { flush(); }
        /* Message */
        msg_print(b"The enchantment failed.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Something happened */
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn curse_artifact(mut o_ptr: *mut object_type) {
    if (*o_ptr).pval != 0 {
        (*o_ptr).pval =
            0 as libc::c_int -
                ((*o_ptr).pval +
                     (Rand_div(4 as libc::c_int) + 1 as libc::c_int))
    }
    if (*o_ptr).to_a != 0 {
        (*o_ptr).to_a =
            (0 as libc::c_int -
                 ((*o_ptr).to_a as libc::c_int +
                      (Rand_div(4 as libc::c_int) + 1 as libc::c_int))) as
                s16b
    }
    if (*o_ptr).to_h != 0 {
        (*o_ptr).to_h =
            (0 as libc::c_int -
                 ((*o_ptr).to_h as libc::c_int +
                      (Rand_div(4 as libc::c_int) + 1 as libc::c_int))) as
                s16b
    }
    if (*o_ptr).to_d != 0 {
        (*o_ptr).to_d =
            (0 as libc::c_int -
                 ((*o_ptr).to_d as libc::c_int +
                      (Rand_div(4 as libc::c_int) + 1 as libc::c_int))) as
                s16b
    }
    (*o_ptr).art_flags3 =
        ((*o_ptr).art_flags3 as libc::c_long |
             (0x40000000 as libc::c_long | 0x20000000 as libc::c_long)) as
            u32b;
    if Rand_div(3 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x80 as libc::c_long) as
                u32b
    }
    if Rand_div(2 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x8000000 as libc::c_long)
                as u32b
    }
    if Rand_div(3 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x2000000 as libc::c_long)
                as u32b
    }
    if Rand_div(3 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
        (*o_ptr).art_flags4 =
            ((*o_ptr).art_flags4 as libc::c_long | 0x4 as libc::c_long) as
                u32b
    }
    if Rand_div(2 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x4000000 as libc::c_long)
                as u32b
    } else if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                  1 as libc::c_int {
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x10 as libc::c_long) as
                u32b
    }
    (*o_ptr).ident =
        ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as byte_hack;
}
/*
 * Should be merged with randart code.
 * looks like BASIC coder's work...
 */
#[no_mangle]
pub unsafe extern "C" fn random_plus(mut o_ptr: *mut object_type,
                                     mut is_scroll: bool_) {
    let mut this_type: libc::c_int =
        if ((*o_ptr).tval as libc::c_int) < 30 as libc::c_int {
            23 as libc::c_int
        } else { 19 as libc::c_int };
    if artifact_bias == 18 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x1 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x1 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
            /* 50% chance of being a "free" power */
        }
        if (*o_ptr).art_flags1 as libc::c_long & 0x10 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x10 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
        if (*o_ptr).art_flags1 as libc::c_long & 0x8 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x8 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 17 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x2 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x2 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 13 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x4 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x4 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 19 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x10 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x10 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
            /* 50% chance of being a "free" power */
        }
        if (*o_ptr).art_flags1 as libc::c_long & 0x8 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x8 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
        if (*o_ptr).art_flags1 as libc::c_long & 0x1 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x1 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 16 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x100 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x100 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
        if (*o_ptr).art_flags1 as libc::c_long & 0x200 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x200 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 6 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x1 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x1 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 8 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x4 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x4 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 7 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x2 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x2 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 9 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x8 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x8 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 10 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x10 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x10 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 11 as libc::c_int {
        if (*o_ptr).art_flags1 as libc::c_long & 0x20 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x20 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    }
    match Rand_div(this_type) + 1 as libc::c_int {
        1 | 2 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x1 as libc::c_long) as
                    u32b;
            /*  if (is_scroll) msg_print("It makes you feel strong!"); */
            if artifact_bias == 0 &&
                   Rand_div(13 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                artifact_bias = 6 as libc::c_int
            } else if artifact_bias == 0 &&
                          Rand_div(7 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                artifact_bias = 18 as libc::c_int
            }
        }
        3 | 4 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x2 as libc::c_long) as
                    u32b;
            /*  if (is_scroll) msg_print("It makes you feel smart!"); */
            if artifact_bias == 0 &&
                   Rand_div(13 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                artifact_bias = 7 as libc::c_int
            } else if artifact_bias == 0 &&
                          Rand_div(7 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                artifact_bias = 17 as libc::c_int
            }
        }
        5 | 6 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x4 as libc::c_long) as
                    u32b;
            /*  if (is_scroll) msg_print("It makes you feel wise!"); */
            if artifact_bias == 0 &&
                   Rand_div(13 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                artifact_bias = 8 as libc::c_int
            } else if artifact_bias == 0 &&
                          Rand_div(7 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                artifact_bias = 13 as libc::c_int
            }
        }
        7 | 8 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x8 as libc::c_long) as
                    u32b;
            /*  if (is_scroll) msg_print("It makes you feel nimble!"); */
            if artifact_bias == 0 &&
                   Rand_div(13 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                artifact_bias = 9 as libc::c_int
            } else if artifact_bias == 0 &&
                          Rand_div(7 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                artifact_bias = 16 as libc::c_int
            }
        }
        9 | 10 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x10 as libc::c_long)
                    as u32b;
            /*  if (is_scroll) msg_print("It makes you feel healthy!"); */
            if artifact_bias == 0 &&
                   Rand_div(13 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                artifact_bias = 10 as libc::c_int
            } else if artifact_bias == 0 &&
                          Rand_div(9 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                artifact_bias = 19 as libc::c_int
            }
        }
        11 | 12 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x20 as libc::c_long)
                    as u32b;
            /*  if (is_scroll) msg_print("It makes you look great!"); */
            if artifact_bias == 0 &&
                   Rand_div(13 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                artifact_bias = 11 as libc::c_int
            }
        }
        13 | 14 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x100 as libc::c_long)
                    as u32b;
            /*  if (is_scroll) msg_print("It looks muffled."); */
            if artifact_bias == 0 &&
                   Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                artifact_bias = 16 as libc::c_int
            }
        }
        15 | 16 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x200 as libc::c_long)
                    as u32b;
            /*  if (is_scroll) msg_print("It makes you see better."); */
            if artifact_bias == 0 &&
                   Rand_div(9 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                artifact_bias = 19 as libc::c_int
            }
        }
        17 | 18 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x400 as libc::c_long)
                    as u32b
        }
        19 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x1000 as libc::c_long)
                    as u32b;
            /*  if (is_scroll) msg_print("It makes you move faster!"); */
            if artifact_bias == 0 &&
                   Rand_div(11 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                artifact_bias = 16 as libc::c_int
            }
        }
        20 | 21 => {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x800 as libc::c_long)
                    as u32b
        }
        22 | 23 => {
            if (*o_ptr).tval as libc::c_int == 19 as libc::c_int {
                random_plus(o_ptr, is_scroll);
            } else {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x2000 as libc::c_long) as u32b;
                /*  if (is_scroll) msg_print("It seems faster!"); */
                if artifact_bias == 0 &&
                       Rand_div(11 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                    artifact_bias = 18 as libc::c_int
                }
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn random_resistance(mut o_ptr: *mut object_type,
                                           mut is_scroll: bool_,
                                           mut specific: libc::c_int) {
    /* To avoid a number of possible bugs */
    if specific == 0 {
        if artifact_bias == 5 as libc::c_int {
            if (*o_ptr).art_flags2 as libc::c_long & 0x10000 as libc::c_long
                   == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x10000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if Rand_div(20 as libc::c_int) == 0 as libc::c_int &&
                   (*o_ptr).art_flags2 as libc::c_long & 0x100 as libc::c_long
                       == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x100 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
        } else if artifact_bias == 1 as libc::c_int {
            if (*o_ptr).art_flags2 as libc::c_long & 0x20000 as libc::c_long
                   == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x20000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if (*o_ptr).tval as libc::c_int >= 35 as libc::c_int &&
                   (*o_ptr).tval as libc::c_int <= 37 as libc::c_int &&
                   (*o_ptr).art_flags3 as libc::c_long & 0x2 as libc::c_long
                       == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x2 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if Rand_div(20 as libc::c_int) == 0 as libc::c_int &&
                   (*o_ptr).art_flags2 as libc::c_long & 0x200 as libc::c_long
                       == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x200 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 1 as libc::c_int { return }
            }
        } else if artifact_bias == 3 as libc::c_int {
            if (*o_ptr).art_flags2 as libc::c_long & 0x40000 as libc::c_long
                   == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x40000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if (*o_ptr).tval as libc::c_int >= 35 as libc::c_int &&
                   (*o_ptr).tval as libc::c_int <= 37 as libc::c_int &&
                   (*o_ptr).art_flags3 as libc::c_long & 0x1 as libc::c_long
                       == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x1 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if Rand_div(20 as libc::c_int) == 0 as libc::c_int &&
                   (*o_ptr).art_flags2 as libc::c_long & 0x400 as libc::c_long
                       == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x400 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
        } else if artifact_bias == 4 as libc::c_int {
            if (*o_ptr).art_flags2 as libc::c_long & 0x80000 as libc::c_long
                   == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x80000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if Rand_div(20 as libc::c_int) == 0 as libc::c_int &&
                   (*o_ptr).art_flags2 as libc::c_long & 0x800 as libc::c_long
                       == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x800 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
        } else if artifact_bias == 2 as libc::c_int {
            if (*o_ptr).art_flags2 as libc::c_long & 0x100000 as libc::c_long
                   == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x100000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
        } else if artifact_bias == 18 as libc::c_int {
            if Rand_div(3 as libc::c_int) != 0 &&
                   (*o_ptr).art_flags2 as libc::c_long &
                       0x200000 as libc::c_long == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x200000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if Rand_div(3 as libc::c_int) == 0 as libc::c_int &&
                   (*o_ptr).art_flags3 as libc::c_long & 0x20 as libc::c_long
                       == 0 {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x20 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
        } else if artifact_bias == 14 as libc::c_int {
            if (*o_ptr).art_flags2 as libc::c_long &
                   0x10000000 as libc::c_long == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x10000000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if (*o_ptr).art_flags2 as libc::c_long & 0x100000 as libc::c_long
                   == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x100000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if (*o_ptr).art_flags2 as libc::c_long & 0x800000 as libc::c_long
                   == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x800000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
        } else if artifact_bias == 12 as libc::c_int {
            if (*o_ptr).art_flags2 as libc::c_long &
                   0x40000000 as libc::c_long == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x40000000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if (*o_ptr).art_flags2 as libc::c_long & 0x2000000 as libc::c_long
                   == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x2000000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
            if (*o_ptr).art_flags2 as libc::c_long &
                   0x80000000 as libc::c_long == 0 {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x80000000 as libc::c_long) as u32b;
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int { return }
            }
        }
    }
    match if specific != 0 {
              specific
          } else { (Rand_div(41 as libc::c_int)) + 1 as libc::c_int } {
        1 => {
            if Rand_div(12 as libc::c_int) + 1 as libc::c_int !=
                   1 as libc::c_int {
                random_resistance(o_ptr, is_scroll, specific);
            } else {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x100 as libc::c_long) as u32b;
                /*  if (is_scroll) msg_print("It looks totally incorruptible."); */
                if artifact_bias == 0 { artifact_bias = 5 as libc::c_int }
            }
        }
        2 => {
            if Rand_div(12 as libc::c_int) + 1 as libc::c_int !=
                   1 as libc::c_int {
                random_resistance(o_ptr, is_scroll, specific);
            } else {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x200 as libc::c_long) as u32b;
                /*  if (is_scroll) msg_print("It looks completely grounded."); */
                if artifact_bias == 0 { artifact_bias = 1 as libc::c_int }
            }
        }
        3 => {
            if Rand_div(12 as libc::c_int) + 1 as libc::c_int !=
                   1 as libc::c_int {
                random_resistance(o_ptr, is_scroll, specific);
            } else {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x800 as libc::c_long) as u32b;
                /*  if (is_scroll) msg_print("It feels very warm."); */
                if artifact_bias == 0 { artifact_bias = 4 as libc::c_int }
            }
        }
        4 => {
            if Rand_div(12 as libc::c_int) + 1 as libc::c_int !=
                   1 as libc::c_int {
                random_resistance(o_ptr, is_scroll, specific);
            } else {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x400 as libc::c_long) as u32b;
                /*  if (is_scroll) msg_print("It feels very cool."); */
                if artifact_bias == 0 { artifact_bias = 3 as libc::c_int }
            }
        }
        5 | 6 | 13 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x10000 as libc::c_long) as u32b;
            /*  if (is_scroll) msg_print("It makes your stomach rumble."); */
            if artifact_bias == 0 { artifact_bias = 5 as libc::c_int }
        }
        7 | 8 | 14 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x20000 as libc::c_long) as u32b;
            /*  if (is_scroll) msg_print("It makes you feel grounded."); */
            if artifact_bias == 0 { artifact_bias = 1 as libc::c_int }
        }
        9 | 10 | 15 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x40000 as libc::c_long) as u32b;
            /*  if (is_scroll) msg_print("It makes you feel cool!");*/
            if artifact_bias == 0 { artifact_bias = 3 as libc::c_int }
        }
        11 | 12 | 16 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x80000 as libc::c_long) as u32b;
            /*  if (is_scroll) msg_print("It makes you feel full of hot air!");*/
            if artifact_bias == 0 { artifact_bias = 4 as libc::c_int }
        }
        17 | 18 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x100000 as libc::c_long) as u32b;
            /*  if (is_scroll) msg_print("It makes breathing easier for you."); */
            if artifact_bias == 0 &&
                   Rand_div(4 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                artifact_bias = 2 as libc::c_int
            } else if artifact_bias == 0 &&
                          Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                artifact_bias = 14 as libc::c_int
            } else if artifact_bias == 0 &&
                          Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                artifact_bias = 16 as libc::c_int
            }
        }
        19 | 20 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x200000 as libc::c_long) as u32b;
            /*  if (is_scroll) msg_print("It makes you feel brave!"); */
            if artifact_bias == 0 &&
                   Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                artifact_bias = 18 as libc::c_int
            }
        }
        21 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x400000 as libc::c_long) as u32b
        }
        22 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x800000 as libc::c_long) as u32b
        }
        23 | 24 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x1000000 as libc::c_long) as u32b
        }
        25 | 26 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x2000000 as libc::c_long) as u32b;
            /*  if (is_scroll) msg_print("It makes you feel very determined.");*/
            if artifact_bias == 0 &&
                   Rand_div(6 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                artifact_bias = 12 as libc::c_int
            }
        }
        27 | 28 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x4000000 as libc::c_long) as u32b
        }
        29 | 30 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x8000000 as libc::c_long) as u32b
        }
        31 | 32 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x10000000 as libc::c_long) as u32b;
            /*  if (is_scroll) msg_print("It makes you feel like visiting a graveyard!");*/
            if artifact_bias == 0 &&
                   Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                artifact_bias = 14 as libc::c_int
            }
        }
        33 | 34 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x20000000 as libc::c_long) as u32b
        }
        35 | 36 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x40000000 as libc::c_long) as u32b;
            /*  if (is_scroll) msg_print("It makes you feel very firm.");*/
            if artifact_bias == 0 &&
                   Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                artifact_bias = 12 as libc::c_int
            }
        }
        37 | 38 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long |
                     0x80000000 as libc::c_long) as u32b
        }
        39 => {
            if (*o_ptr).tval as libc::c_int >= 35 as libc::c_int &&
                   (*o_ptr).tval as libc::c_int <= 37 as libc::c_int {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x2 as libc::c_long) as u32b
            } else { random_resistance(o_ptr, is_scroll, specific); }
            if artifact_bias == 0 { artifact_bias = 1 as libc::c_int }
        }
        40 => {
            if (*o_ptr).tval as libc::c_int >= 35 as libc::c_int &&
                   (*o_ptr).tval as libc::c_int <= 37 as libc::c_int {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x1 as libc::c_long) as u32b
            } else { random_resistance(o_ptr, is_scroll, specific); }
            if artifact_bias == 0 { artifact_bias = 3 as libc::c_int }
        }
        41 => {
            if (*o_ptr).tval as libc::c_int == 34 as libc::c_int ||
                   (*o_ptr).tval as libc::c_int == 35 as libc::c_int ||
                   (*o_ptr).tval as libc::c_int == 32 as libc::c_int ||
                   (*o_ptr).tval as libc::c_int == 37 as libc::c_int {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x2000 as libc::c_long) as u32b
            } else { random_resistance(o_ptr, is_scroll, specific); }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn random_misc(mut o_ptr: *mut object_type,
                                     mut is_scroll: bool_) {
    if artifact_bias == 19 as libc::c_int {
        if (*o_ptr).art_flags2 as libc::c_long & 0x10 as libc::c_long == 0 {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x10 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 6 as libc::c_int {
        if (*o_ptr).art_flags2 as libc::c_long & 0x1 as libc::c_long == 0 {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x1 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 8 as libc::c_int {
        if (*o_ptr).art_flags2 as libc::c_long & 0x4 as libc::c_long == 0 {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x4 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 7 as libc::c_int {
        if (*o_ptr).art_flags2 as libc::c_long & 0x2 as libc::c_long == 0 {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x2 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 9 as libc::c_int {
        if (*o_ptr).art_flags2 as libc::c_long & 0x8 as libc::c_long == 0 {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x8 as libc::c_long) as
                    u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 10 as libc::c_int {
        if (*o_ptr).art_flags2 as libc::c_long & 0x10 as libc::c_long == 0 {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x10 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 11 as libc::c_int {
        if (*o_ptr).art_flags2 as libc::c_long & 0x20 as libc::c_long == 0 {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x20 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 12 as libc::c_int {
        if (*o_ptr).art_flags3 as libc::c_long & 0x4000000 as libc::c_long ==
               0 {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long |
                     0x4000000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 3 as libc::c_int {
        if (*o_ptr).art_flags3 as libc::c_long & 0x2000 as libc::c_long == 0 {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long | 0x2000 as libc::c_long)
                    as u32b
            /* Freebie */
        }
    }
    match Rand_div(31 as libc::c_int) + 1 as libc::c_int {
        1 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x1 as libc::c_long) as
                    u32b;
            /*			if (is_scroll) msg_print("It makes you feel you cannot become weaker."); */
            if artifact_bias == 0 { artifact_bias = 6 as libc::c_int }
        }
        2 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x2 as libc::c_long) as
                    u32b;
            /*			if (is_scroll) msg_print("It makes you feel you cannot become more stupid.");*/
            if artifact_bias == 0 { artifact_bias = 7 as libc::c_int }
        }
        3 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x4 as libc::c_long) as
                    u32b;
            /*			if (is_scroll) msg_print("It makes you feel you cannot become simpler.");*/
            if artifact_bias == 0 { artifact_bias = 8 as libc::c_int }
        }
        4 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x8 as libc::c_long) as
                    u32b;
            /*			if (is_scroll) msg_print("It makes you feel you cannot become clumsier.");*/
            if artifact_bias == 0 { artifact_bias = 9 as libc::c_int }
        }
        5 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x10 as libc::c_long)
                    as u32b;
            /*			if (is_scroll) msg_print("It makes you feel you cannot become less healthy.");*/
            if artifact_bias == 0 { artifact_bias = 10 as libc::c_int }
        }
        6 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x20 as libc::c_long)
                    as u32b;
            /*			if (is_scroll) msg_print("It makes you feel you cannot become uglier.");*/
            if artifact_bias == 0 { artifact_bias = 11 as libc::c_int }
        }
        7 | 8 | 14 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x4000 as libc::c_long)
                    as u32b
        }
        9 => {
            (*o_ptr).art_flags2 =
                ((*o_ptr).art_flags2 as libc::c_long | 0x8000 as libc::c_long)
                    as u32b;
            /*			if (is_scroll) msg_print("It makes you feel immortal.");*/
            if artifact_bias == 0 &&
                   Rand_div(5 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                artifact_bias = 13 as libc::c_int
            } else if artifact_bias == 0 &&
                          Rand_div(6 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                artifact_bias = 14 as libc::c_int
            }
        }
        10 | 11 => {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long | 0x2000 as libc::c_long)
                    as u32b
        }
        12 | 13 => {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long | 0x1000 as libc::c_long)
                    as u32b
        }
        15 | 16 | 17 => {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long | 0x4000 as libc::c_long)
                    as u32b
        }
        18 => {
            (*o_ptr).art_esp |=
                ((1 as libc::c_int) << Rand_div(32 as libc::c_int)) as
                    libc::c_uint;
            /*			if (is_scroll) msg_print("It makes you hear voices inside your head!");*/
            if artifact_bias == 0 &&
                   Rand_div(9 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                artifact_bias = 17 as libc::c_int
            }
        }
        19 | 20 => {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long |
                     0x10000 as libc::c_long) as u32b
        }
        21 | 22 => {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long |
                     0x20000 as libc::c_long) as u32b
        }
        23 => {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long |
                     0x4000000 as libc::c_long) as u32b
        }
        24 | 25 | 26 => {
            if (*o_ptr).tval as libc::c_int >= 30 as libc::c_int {
                random_misc(o_ptr, is_scroll);
            } else {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x400 as libc::c_long) as u32b;
                (*o_ptr).to_a =
                    (4 as libc::c_int +
                         (Rand_div(11 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
        }
        27 | 28 | 29 => {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long | 0x400 as libc::c_long)
                    as u32b;
            (*o_ptr).to_h =
                ((*o_ptr).to_h as libc::c_int +
                     (4 as libc::c_int +
                          (Rand_div(11 as libc::c_int) + 1 as libc::c_int)))
                    as s16b;
            (*o_ptr).to_d =
                ((*o_ptr).to_d as libc::c_int +
                     (4 as libc::c_int +
                          (Rand_div(11 as libc::c_int) + 1 as libc::c_int)))
                    as s16b
        }
        30 => {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long | 0x20 as libc::c_long)
                    as u32b
        }
        31 => {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long | 0x10 as libc::c_long)
                    as u32b
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn random_slay(mut o_ptr: *mut object_type,
                                     mut is_scroll: bool_) {
    if artifact_bias == 12 as libc::c_int &&
           !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        if (*o_ptr).art_flags1 as libc::c_long & 0x4000 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x4000 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 13 as libc::c_int &&
                  ((*o_ptr).tval as libc::c_int == 23 as libc::c_int ||
                       (*o_ptr).tval as libc::c_int == 22 as libc::c_int ||
                       (*o_ptr).tval as libc::c_int == 24 as libc::c_int) &&
                  (*o_ptr).art_flags3 as libc::c_long &
                      0x10000000 as libc::c_long == 0 {
        /* A free power for "priestly" random artifacts */
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x10000000 as libc::c_long)
                as u32b
    } else if artifact_bias == 14 as libc::c_int &&
                  !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        if (*o_ptr).art_flags1 as libc::c_long & 0x8000 as libc::c_long == 0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long | 0x8000 as libc::c_long)
                    as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
        if (*o_ptr).art_flags1 as libc::c_long & 0x8000000 as libc::c_long ==
               0 &&
               Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x8000000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 19 as libc::c_int &&
                  !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        if (*o_ptr).art_flags1 as libc::c_long & 0x10000 as libc::c_long == 0
           {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x10000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 16 as libc::c_int &&
                  !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        if (*o_ptr).art_flags1 as libc::c_long & 0x8000000 as libc::c_long ==
               0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x8000000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 2 as libc::c_int &&
                  !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        if (*o_ptr).art_flags1 as libc::c_long & 0x8000000 as libc::c_long ==
               0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x8000000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 3 as libc::c_int &&
                  !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        if (*o_ptr).art_flags1 as libc::c_long & 0x40000000 as libc::c_long ==
               0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x40000000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 4 as libc::c_int &&
                  !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        if (*o_ptr).art_flags1 as libc::c_long & 0x80000000 as libc::c_long ==
               0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x80000000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 1 as libc::c_int &&
                  !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        if (*o_ptr).art_flags1 as libc::c_long & 0x20000000 as libc::c_long ==
               0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x20000000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 5 as libc::c_int &&
                  !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        if (*o_ptr).art_flags1 as libc::c_long & 0x10000000 as libc::c_long ==
               0 {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x10000000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    } else if artifact_bias == 15 as libc::c_int &&
                  !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        if (*o_ptr).art_flags1 as libc::c_long & 0x20000 as libc::c_long == 0
           {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x20000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
        if (*o_ptr).art_flags1 as libc::c_long & 0x40000 as libc::c_long == 0
           {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x40000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
        if (*o_ptr).art_flags1 as libc::c_long & 0x80000 as libc::c_long == 0
           {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x80000 as libc::c_long) as u32b;
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                return
            }
        }
    }
    if !((*o_ptr).tval as libc::c_int == 19 as libc::c_int) {
        match Rand_div(34 as libc::c_int) + 1 as libc::c_int {
            1 | 2 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x10000 as libc::c_long) as u32b
            }
            3 | 4 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x20000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("You hate evil creatures.");*/
                if artifact_bias == 0 &&
                       Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                    artifact_bias = 15 as libc::c_int
                } else if artifact_bias == 0 &&
                              Rand_div(9 as libc::c_int) + 1 as libc::c_int ==
                                  1 as libc::c_int {
                    artifact_bias = 13 as libc::c_int
                }
            }
            5 | 6 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x40000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("You hate undead creatures.");*/
                if artifact_bias == 0 &&
                       Rand_div(9 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                    artifact_bias = 13 as libc::c_int
                }
            }
            7 | 8 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x80000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("You hate demons.");*/
                if artifact_bias == 0 &&
                       Rand_div(9 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                    artifact_bias = 13 as libc::c_int
                }
            }
            9 | 10 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x100000 as libc::c_long) as u32b
            }
            11 | 12 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x200000 as libc::c_long) as u32b
            }
            13 | 14 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x400000 as libc::c_long) as u32b
            }
            15 | 16 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x800000 as libc::c_long) as u32b
            }
            17 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x1000000 as libc::c_long) as u32b
            }
            18 | 19 => {
                if (*o_ptr).tval as libc::c_int == 23 as libc::c_int {
                    (*o_ptr).art_flags1 =
                        ((*o_ptr).art_flags1 as libc::c_long |
                             0x2000000 as libc::c_long) as u32b;
                    /*					if (is_scroll) msg_print("It looks extremely sharp!");*/
                    if artifact_bias == 0 &&
                           Rand_div(9 as libc::c_int) + 1 as libc::c_int ==
                               1 as libc::c_int {
                        artifact_bias = 18 as libc::c_int
                    }
                } else { random_slay(o_ptr, is_scroll); }
            }
            20 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x4000000 as libc::c_long) as u32b
            }
            21 | 22 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x40000000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("It feels hot!");*/
                if artifact_bias == 0 { artifact_bias = 3 as libc::c_int }
            }
            23 | 24 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x80000000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("It feels cold!");*/
                if artifact_bias == 0 { artifact_bias = 4 as libc::c_int }
            }
            25 | 26 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x20000000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("Ouch! You get zapped!");*/
                if artifact_bias == 0 { artifact_bias = 1 as libc::c_int }
            }
            27 | 28 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x10000000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("Its smell makes you feel dizzy.");*/
                if artifact_bias == 0 { artifact_bias = 5 as libc::c_int }
            }
            29 | 30 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x8000000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("It smells rotten.");*/
                if artifact_bias == 0 &&
                       Rand_div(3 as libc::c_int) + 1 as libc::c_int !=
                           1 as libc::c_int {
                    artifact_bias = 2 as libc::c_int
                } else if artifact_bias == 0 &&
                              Rand_div(6 as libc::c_int) + 1 as libc::c_int ==
                                  1 as libc::c_int {
                    artifact_bias = 14 as libc::c_int
                } else if artifact_bias == 0 {
                    artifact_bias = 16 as libc::c_int
                }
            }
            31 | 32 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x8000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("You think it bit you!");*/
                if artifact_bias == 0 { artifact_bias = 14 as libc::c_int }
            }
            _ => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x4000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("It looks very confusing.");*/
                if artifact_bias == 0 { artifact_bias = 12 as libc::c_int }
            }
        }
    } else {
        match Rand_div(6 as libc::c_int) + 1 as libc::c_int {
            1 | 2 | 3 => {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x40000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("It looks mightier than before."); */
                if artifact_bias == 0 &&
                       Rand_div(9 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                    artifact_bias = 19 as libc::c_int
                }
            }
            _ => {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x80000 as libc::c_long) as u32b;
                /*				if (is_scroll) msg_print("It seems faster!"); */
                if artifact_bias == 0 &&
                       Rand_div(9 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                    artifact_bias = 19 as libc::c_int
                }
            }
        }
    };
}
/*
 * Determines if an item is not identified
 */
unsafe extern "C" fn item_tester_hook_unknown(mut o_ptr: *mut object_type)
 -> bool_ {
    return if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                  (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                      libc::c_int != 0 &&
                      (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                          libc::c_int != 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } as bool_;
}
/*
 * Identify an object in the inventory (or on the floor)
 * This routine does *not* automatically combine objects.
 * Returns TRUE if something was identified, else FALSE.
 */
#[no_mangle]
pub unsafe extern "C" fn ident_spell() -> bool_ {
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Get an item */
    item_tester_hook =
        Some(item_tester_hook_unknown as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    q = b"Identify which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to identify.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
           == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Identify it fully */
    object_aware(o_ptr);
    object_known(o_ptr);
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
                  0x8 as libc::c_long)) as u32b;
    /* Description */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Describe */
    if item >= 24 as libc::c_int {
        msg_format(b"%^s: %s (%c).\x00" as *const u8 as *const libc::c_char,
                   describe_use(item), o_name.as_mut_ptr(),
                   index_to_label(item) as libc::c_int);
    } else if item >= 0 as libc::c_int {
        msg_format(b"In your pack: %s (%c).\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   index_to_label(item) as libc::c_int);
    } else {
        msg_format(b"On the ground: %s.\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr());
    }
    /* If the item was an artifact, and if the auto-note is selected, write a message. */
    if take_notes as libc::c_int != 0 && auto_notes as libc::c_int != 0 &&
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
                (*o_ptr).name1 as libc::c_int != 0) {
        let mut note: [libc::c_char; 150] = [0; 150];
        let mut item_name: [libc::c_char; 80] = [0; 80];
        object_desc(item_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        /* Build note and write */
        sprintf(note.as_mut_ptr(),
                b"Found The %s\x00" as *const u8 as *const libc::c_char,
                item_name.as_mut_ptr());
        add_note(note.as_mut_ptr(), 'A' as i32 as libc::c_char);
    }
    /* Process the appropriate hooks */
    process_hooks(16 as libc::c_int,
                  b"(d,s)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, item,
                  b"normal\x00" as *const u8 as *const libc::c_char);
    /* Something happened */
    return 1 as libc::c_int as bool_;
}
/*
 * Identify all objects in the level
 */
#[no_mangle]
pub unsafe extern "C" fn ident_all() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        /* Acquire object */
        o_ptr = &mut *o_list.offset(i as isize) as *mut object_type;
        /* Identify it fully */
        object_aware(o_ptr);
        object_known(o_ptr);
        /* If the item was an artifact, and if the auto-note is selected, write a message. */
        if take_notes as libc::c_int != 0 && auto_notes as libc::c_int != 0 &&
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
                    (*o_ptr).name1 as libc::c_int != 0) {
            let mut note: [libc::c_char; 150] = [0; 150];
            let mut item_name: [libc::c_char; 80] = [0; 80];
            object_desc(item_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                        0 as libc::c_int);
            /* Build note and write */
            sprintf(note.as_mut_ptr(),
                    b"Found The %s\x00" as *const u8 as *const libc::c_char,
                    item_name.as_mut_ptr());
            add_note(note.as_mut_ptr(), 'A' as i32 as libc::c_char);
        }
        /* Process the appropriate hooks */
        process_hooks(16 as libc::c_int,
                      b"(d,s)\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, -i,
                      b"normal\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
    /* Something happened */
    return 1 as libc::c_int as bool_;
}
/*
 * Determine if an object is not fully identified
 */
unsafe extern "C" fn item_tester_hook_no_mental(mut o_ptr: *mut object_type)
 -> bool_ {
    return if (*o_ptr).ident as libc::c_int & 0x20 as libc::c_int != 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } as bool_;
}
/*
 * Fully "identify" an object in the inventory  -BEN-
 * This routine returns TRUE if an item was identified.
 */
#[no_mangle]
pub unsafe extern "C" fn identify_fully() -> bool_ {
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Get an item */
    item_tester_hook =
        Some(item_tester_hook_no_mental as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    q = b"Identify which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to identify.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
           == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Do the identification */
    make_item_fully_identified(o_ptr);
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
                  0x8 as libc::c_long)) as u32b;
    /* Description */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Describe */
    if item >= 24 as libc::c_int {
        msg_format(b"%^s: %s (%c).\x00" as *const u8 as *const libc::c_char,
                   describe_use(item), o_name.as_mut_ptr(),
                   index_to_label(item) as libc::c_int);
    } else if item >= 0 as libc::c_int {
        msg_format(b"In your pack: %s (%c).\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   index_to_label(item) as libc::c_int);
    } else {
        msg_format(b"On the ground: %s.\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr());
    }
    /* If the item was an artifact, and if the auto-note is selected, write a message. */
    if take_notes as libc::c_int != 0 && auto_notes as libc::c_int != 0 &&
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
                (*o_ptr).name1 as libc::c_int != 0) {
        let mut note: [libc::c_char; 150] = [0; 150];
        let mut item_name: [libc::c_char; 80] = [0; 80];
        object_desc(item_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        /* Build note and write */
        sprintf(note.as_mut_ptr(),
                b"Found The %s\x00" as *const u8 as *const libc::c_char,
                item_name.as_mut_ptr());
        add_note(note.as_mut_ptr(), 'A' as i32 as libc::c_char);
    }
    /* Describe it fully */
    object_out_desc(o_ptr, 0 as *mut FILE, 0 as libc::c_int as bool_,
                    1 as libc::c_int as bool_);
    /* Process the appropriate hooks */
    process_hooks(16 as libc::c_int,
                  b"(d,s)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, item,
                  b"full\x00" as *const u8 as *const libc::c_char);
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Hook for "get_item()".  Determine if something is rechargable.
 */
#[no_mangle]
pub unsafe extern "C" fn item_tester_hook_recharge(mut o_ptr:
                                                       *mut object_type)
 -> bool_ {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Some objects cannot be recharged */
    if f4 as libc::c_long & 0x80000000 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Recharge staffs */
    if (*o_ptr).tval as libc::c_int == 55 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Recharge wands */
    if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Hack -- Recharge rods */
    if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Nope */
    return 0 as libc::c_int as bool_;
}
/*
 * Recharge a wand/staff/rod from the pack or on the floor.
 * This function has been rewritten in Oangband. -LM-
 *
 * Mage -- Recharge I --> recharge(90)
 * Mage -- Recharge II --> recharge(150)
 * Mage -- Recharge III --> recharge(220)
 *
 * Priest or Necromancer -- Recharge --> recharge(140)
 *
 * Scroll of recharging --> recharge(130)
 * Scroll of *recharging* --> recharge(200)
 *
 * It is harder to recharge high level, and highly charged wands, 
 * staffs, and rods.  The more wands in a stack, the more easily and 
 * strongly they recharge.  Staffs, however, each get fewer charges if 
 * stacked.
 *
 * XXX XXX XXX Beware of "sliding index errors".
 */
#[no_mangle]
pub unsafe extern "C" fn recharge(mut power: libc::c_int) -> bool_ {
    let mut recharge_strength: libc::c_int = 0;
    let mut recharge_amount: libc::c_int = 0;
    let mut item: libc::c_int = 0;
    let mut lev: libc::c_int = 0;
    let mut fail: bool_ = 0 as libc::c_int as bool_;
    let mut fail_type: byte_hack = 1 as libc::c_int as byte_hack;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Only accept legal items */
    item_tester_hook =
        Some(item_tester_hook_recharge as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q = b"Recharge which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to recharge.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return 0 as libc::c_int as bool_
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Extract the object "level" */
    lev = (*k_info.offset((*o_ptr).k_idx as isize)).level as libc::c_int;
    /* Recharge a rod */
    if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
        /* Extract a recharge strength by comparing object level to power. */
        recharge_strength =
            (if power > lev { (power) - lev } else { 0 as libc::c_int }) /
                5 as libc::c_int;
        /* Paranoia */
        if recharge_strength < 0 as libc::c_int {
            recharge_strength = 0 as libc::c_int
        }
        /* Back-fire */
        if Rand_div(recharge_strength) == 0 as libc::c_int &&
               f4 as libc::c_long & 0x8 as libc::c_long == 0 {
            /* Activate the failure code. */
            fail = 1 as libc::c_int as bool_
        } else {
            /* Recharge */
            /* Recharge amount */
            recharge_amount =
                power *
                    damroll(3 as libc::c_int as s16b,
                            2 as libc::c_int as s16b);
            if (*o_ptr).timeout as libc::c_int + recharge_amount <
                   (*o_ptr).pval2 as libc::c_int {
                (*o_ptr).timeout =
                    ((*o_ptr).timeout as libc::c_int + recharge_amount) as
                        s16b
            } else { (*o_ptr).timeout = (*o_ptr).pval2 }
        }
    } else {
        /* Recharge by that amount */
        /* Recharge wand/staff */
        /* Extract a recharge strength by comparing object level to power.
		 * Divide up a stack of wands' charges to calculate charge penalty.
		 */
        if (*o_ptr).tval as libc::c_int == 65 as libc::c_int &&
               (*o_ptr).number as libc::c_int > 1 as libc::c_int {
            recharge_strength =
                (100 as libc::c_int + power - lev -
                     8 as libc::c_int * (*o_ptr).pval /
                         (*o_ptr).number as libc::c_int) / 15 as libc::c_int
        } else {
            /* All staffs, unstacked wands. */
            recharge_strength =
                (100 as libc::c_int + power - lev -
                     8 as libc::c_int * (*o_ptr).pval) / 15 as libc::c_int
        }
        if Rand_div(recharge_strength) == 0 as libc::c_int &&
               f4 as libc::c_long & 0x8 as libc::c_long == 0 ||
               f4 as libc::c_long & 0x80000000 as libc::c_long != 0 {
            /* Activate the failure code. */
            fail = 1 as libc::c_int as bool_
        } else {
            /* If the spell didn't backfire, recharge the wand or staff. */
            /* Recharge based on the standard number of charges. */
            recharge_amount =
                Rand_div(power / (lev + 2 as libc::c_int) + 1 as libc::c_int)
                    + 1 as libc::c_int;
            if (*o_ptr).tval as libc::c_int == 65 as libc::c_int &&
                   (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                recharge_amount +=
                    (Rand_div(recharge_amount *
                                  ((*o_ptr).number as libc::c_int -
                                       1 as libc::c_int)) + 1 as libc::c_int)
                        / 2 as libc::c_int;
                if recharge_amount < 1 as libc::c_int {
                    recharge_amount = 1 as libc::c_int
                }
                if recharge_amount > 12 as libc::c_int {
                    recharge_amount = 12 as libc::c_int
                }
            }
            if (*o_ptr).tval as libc::c_int == 55 as libc::c_int &&
                   (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                recharge_amount /= (*o_ptr).number as libc::c_int;
                if recharge_amount < 1 as libc::c_int {
                    recharge_amount = 1 as libc::c_int
                }
            }
            (*o_ptr).pval += recharge_amount;
            if f4 as libc::c_long & 0x8 as libc::c_long == 0 {
                /* Multiple wands in a stack increase recharging somewhat. */
                /* But each staff in a stack gets fewer additional charges,
			 * although always at least one.
			 */
                /* Recharge the wand or staff. */
                /* Hack -- we no longer "know" the item */
                (*o_ptr).ident =
                    ((*o_ptr).ident as libc::c_int & !(0x8 as libc::c_int)) as
                        byte_hack
            }
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int & !(0x4 as libc::c_int)) as
                    byte_hack
        }
    }
    /* Hack -- we no longer think the item is empty */
    /* Back-fire XXX XXX XXX */
    /* Mark as recharged -- For alchemists */
    (*o_ptr).art_flags4 =
        ((*o_ptr).art_flags4 as libc::c_long | 0x800000 as libc::c_long) as
            u32b;
    /* Inflict the penalties for failing a recharge. */
    if fail != 0 {
        /* Artifacts are never destroyed. */
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
            object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                        0 as libc::c_int);
            msg_format(b"The recharging backfires - %s is completely drained!\x00"
                           as *const u8 as *const libc::c_char,
                       o_name.as_mut_ptr());
            /* Artifact rods. */
            if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
                (*o_ptr).timeout = 0 as libc::c_int as s16b
            } else if (*o_ptr).tval as libc::c_int == 65 as libc::c_int ||
                          (*o_ptr).tval as libc::c_int == 55 as libc::c_int {
                (*o_ptr).pval = 0 as libc::c_int
            }
        } else {
            /* Artifact wands and staffs. */
            /* Get the object description */
            object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                        0 as libc::c_int);
            /* ** Determine Seriousness of Failure ***/
            /* Mages recharge objects more safely. */
            if has_ability(2 as libc::c_int) != 0 {
                /* 10% chance to blow up one rod, otherwise draining. */
                if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
                    if Rand_div(10 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        fail_type = 2 as libc::c_int as byte_hack
                    } else { fail_type = 1 as libc::c_int as byte_hack }
                } else if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
                    if Rand_div(3 as libc::c_int) + 1 as libc::c_int !=
                           1 as libc::c_int {
                        fail_type = 2 as libc::c_int as byte_hack
                    } else { fail_type = 1 as libc::c_int as byte_hack }
                } else if (*o_ptr).tval as libc::c_int == 55 as libc::c_int {
                    if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        fail_type = 2 as libc::c_int as byte_hack
                    } else { fail_type = 0 as libc::c_int as byte_hack }
                }
            } else if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
                if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    fail_type = 2 as libc::c_int as byte_hack
                } else { fail_type = 1 as libc::c_int as byte_hack }
            } else if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
                if Rand_div(5 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    fail_type = 3 as libc::c_int as byte_hack
                } else { fail_type = 2 as libc::c_int as byte_hack }
            } else if (*o_ptr).tval as libc::c_int == 55 as libc::c_int {
                fail_type = 2 as libc::c_int as byte_hack
            }
            /* 75% chance to blow up one wand, otherwise draining. */
            /* 50% chance to blow up one staff, otherwise no effect. */
            /* All other classes get no special favors. */
            /* 33% chance to blow up one rod, otherwise draining. */
            /* 20% chance of the entire stack, else destroy one wand. */
            /* Blow up one staff. */
            /* ** Apply draining and destruction. ***/
            /* Drain object or stack of objects. */
            if fail_type as libc::c_int == 1 as libc::c_int {
                if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
                    msg_print(b"The recharge backfires, draining the rod further!\x00"
                                  as *const u8 as *const libc::c_char);
                    if ((*o_ptr).timeout as libc::c_int) <
                           10000 as libc::c_int {
                        (*o_ptr).timeout = 0 as libc::c_int as s16b
                    }
                } else if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
                    msg_format(b"You save your %s from destruction, but all charges are lost.\x00"
                                   as *const u8 as *const libc::c_char,
                               o_name.as_mut_ptr());
                    (*o_ptr).pval = 0 as libc::c_int
                }
                /* Staffs aren't drained. */
            }
            /* Destroy an object or one in a stack of objects. */
            if fail_type as libc::c_int == 2 as libc::c_int {
                if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                    msg_format(b"Wild magic consumes one of your %s!\x00" as
                                   *const u8 as *const libc::c_char,
                               o_name.as_mut_ptr());
                } else {
                    msg_format(b"Wild magic consumes your %s!\x00" as
                                   *const u8 as *const libc::c_char,
                               o_name.as_mut_ptr());
                }
                /* Reduce rod stack maximum timeout, drain wands. */
                if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
                    (*o_ptr).pval = 0 as libc::c_int
                }
                /* Reduce and describe */
                inc_stack_size(item, -(1 as libc::c_int));
            }
            /* Destroy all memebers of a stack of objects. */
            if fail_type as libc::c_int == 3 as libc::c_int {
                if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                    msg_format(b"Wild magic consumes all your %s!\x00" as
                                   *const u8 as *const libc::c_char,
                               o_name.as_mut_ptr());
                } else {
                    msg_format(b"Wild magic consumes your %s!\x00" as
                                   *const u8 as *const libc::c_char,
                               o_name.as_mut_ptr());
                }
                /* Reduce and describe inventory */
                inc_stack_size(item, -(999 as libc::c_int));
            }
        }
    }
    /* Combine / Reorder the pack (later) */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Something was done */
    return 1 as libc::c_int as bool_;
}
/*
 * Apply a "project()" directly to all viewable monsters
 *
 * Note that affected monsters are NOT auto-tracked by this usage.
 */
#[no_mangle]
pub unsafe extern "C" fn project_hack(mut typ: libc::c_int,
                                      mut dam: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut flg: libc::c_int =
        0x1 as libc::c_int | 0x40 as libc::c_int | 0x80 as libc::c_int;
    let mut obvious: bool_ = 0 as libc::c_int as bool_;
    /* Affect all (nearby) monsters */
    i = 1 as libc::c_int;
    while i < m_max as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Paranoia -- Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Location */
            y = (*m_ptr).fy as libc::c_int;
            x = (*m_ptr).fx as libc::c_int;
            /* Require line of sight */
            if (*cave[y as usize].offset(x as isize)).info as libc::c_int &
                   0x20 as libc::c_int != 0 as libc::c_int {
                /* Jump directly to the target monster */
                if project(0 as libc::c_int, 0 as libc::c_int, y, x, dam, typ,
                           flg) != 0 {
                    obvious = 1 as libc::c_int as bool_
                }
            }
        }
        i += 1
    }
    /* Result */
    return obvious;
}
/*
 * Apply a "project()" a la meteor shower
 */
#[no_mangle]
pub unsafe extern "C" fn project_meteor(mut radius: libc::c_int,
                                        mut typ: libc::c_int,
                                        mut dam: libc::c_int, mut flg: u32b) {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = radius + (Rand_div(radius) + 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < b {
        count = 0 as libc::c_int;
        while count < 1000 as libc::c_int {
            x =
                (*p_ptr).px as libc::c_int - 5 as libc::c_int +
                    (Rand_div(10 as libc::c_int) + 1 as libc::c_int);
            y =
                (*p_ptr).py as libc::c_int - 5 as libc::c_int +
                    (Rand_div(10 as libc::c_int) + 1 as libc::c_int);
            if !(x < 0 as libc::c_int || x >= cur_wid as libc::c_int ||
                     y < 0 as libc::c_int || y >= cur_hgt as libc::c_int) {
                dx =
                    if (*p_ptr).px as libc::c_int > x {
                        ((*p_ptr).px as libc::c_int) - x
                    } else { (x) - (*p_ptr).px as libc::c_int };
                dy =
                    if (*p_ptr).py as libc::c_int > y {
                        ((*p_ptr).py as libc::c_int) - y
                    } else { (y) - (*p_ptr).py as libc::c_int };
                /* Approximate distance */
                d =
                    if dy > dx {
                        (dy) + (dx >> 1 as libc::c_int)
                    } else { (dx) + (dy >> 1 as libc::c_int) };
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(y as
                                                         isize)).offset(x as
                                                                            isize)
                        as *mut cave_type;
                /* Check distance */
                if d <= 5 as libc::c_int &&
                       (*cave[y as usize].offset(x as isize)).info as
                           libc::c_int & 0x20 as libc::c_int !=
                           0 as libc::c_int &&
                       (*c_ptr).info as libc::c_int & 0x80 as libc::c_int == 0
                   {
                    break ;
                }
            }
            count += 1
        }
        if count >= 1000 as libc::c_int { break ; }
        project(0 as libc::c_int, 2 as libc::c_int, y, x, dam, typ,
                (0x1 as libc::c_int as libc::c_uint | flg) as libc::c_int);
        i += 1
    };
}
/*
 * Speed monsters
 */
#[no_mangle]
pub unsafe extern "C" fn speed_monsters() -> bool_ {
    return project_hack(54 as libc::c_int, (*p_ptr).lev as libc::c_int);
}
/*
 * Slow monsters
 */
#[no_mangle]
pub unsafe extern "C" fn slow_monsters() -> bool_ {
    return project_hack(55 as libc::c_int, (*p_ptr).lev as libc::c_int);
}
/*
 * Paralyzation monsters
 */
#[no_mangle]
pub unsafe extern "C" fn conf_monsters() -> bool_ {
    return project_hack(56 as libc::c_int, (*p_ptr).lev as libc::c_int);
}
/*
 * Sleep monsters
 */
#[no_mangle]
pub unsafe extern "C" fn sleep_monsters() -> bool_ {
    return project_hack(57 as libc::c_int, (*p_ptr).lev as libc::c_int);
}
/*
 * Scare monsters
 */
#[no_mangle]
pub unsafe extern "C" fn scare_monsters() -> bool_ {
    return project_hack(102 as libc::c_int, (*p_ptr).lev as libc::c_int);
}
/*
 * Banish evil monsters
 */
#[no_mangle]
pub unsafe extern "C" fn banish_evil(mut dist: libc::c_int) -> bool_ {
    return project_hack(62 as libc::c_int, dist);
}
/*
 * Turn undead
 */
#[no_mangle]
pub unsafe extern "C" fn turn_undead() -> bool_ {
    return project_hack(64 as libc::c_int, (*p_ptr).lev as libc::c_int);
}
/*
 * Dispel undead monsters
 */
#[no_mangle]
pub unsafe extern "C" fn dispel_undead(mut dam: libc::c_int) -> bool_ {
    return project_hack(67 as libc::c_int, dam);
}
/*
 * Dispel evil monsters
 */
#[no_mangle]
pub unsafe extern "C" fn dispel_evil(mut dam: libc::c_int) -> bool_ {
    return project_hack(68 as libc::c_int, dam);
}
/*
 * Dispel good monsters
 */
#[no_mangle]
pub unsafe extern "C" fn dispel_good(mut dam: libc::c_int) -> bool_ {
    return project_hack(90 as libc::c_int, dam);
}
/*
 * Dispel all monsters
 */
#[no_mangle]
pub unsafe extern "C" fn dispel_monsters(mut dam: libc::c_int) -> bool_ {
    return project_hack(69 as libc::c_int, dam);
}
/*
 * Dispel 'living' monsters
 */
#[no_mangle]
pub unsafe extern "C" fn dispel_living(mut dam: libc::c_int) -> bool_ {
    return project_hack(71 as libc::c_int, dam);
}
/*
 * Dispel demons
 */
#[no_mangle]
pub unsafe extern "C" fn dispel_demons(mut dam: libc::c_int) -> bool_ {
    return project_hack(70 as libc::c_int, dam);
}
/*
 * Wake up all monsters, and speed up "los" monsters.
 */
#[no_mangle]
pub unsafe extern "C" fn aggravate_monsters(mut who: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut sleep: bool_ = 0 as libc::c_int as bool_;
    let mut speed: bool_ = 0 as libc::c_int as bool_;
    /* Aggravate everyone nearby */
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
        /* Paranoia -- Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Skip aggravating monster (or player) */
            if !(i == who) {
                /* Wake up nearby sleeping monsters */
                if ((*m_ptr).cdis as libc::c_int) <
                       20 as libc::c_int * 2 as libc::c_int {
                    /* Wake up */
                    if (*m_ptr).csleep != 0 {
                        /* Wake up */
                        (*m_ptr).csleep = 0 as libc::c_int as s16b;
                        sleep = 1 as libc::c_int as bool_
                    }
                }
                /* Speed up monsters in line of sight */
                if (*cave[(*m_ptr).fy as
                              usize].offset((*m_ptr).fx as isize)).info as
                       libc::c_int & 0x20 as libc::c_int != 0 as libc::c_int {
                    /* Speed up (instantly) to racial base + 10 */
                    if ((*m_ptr).mspeed as libc::c_int) <
                           (*r_ptr).speed as libc::c_int + 10 as libc::c_int {
                        /* Speed up */
                        (*m_ptr).mspeed =
                            ((*r_ptr).speed as libc::c_int +
                                 10 as libc::c_int) as byte_hack;
                        speed = 1 as libc::c_int as bool_
                    }
                    /* Pets may get angry (50% chance) */
                    if is_friend(m_ptr) != 0 {
                        if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                               1 as libc::c_int {
                            change_side(m_ptr);
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Messages */
    if speed != 0 {
        msg_print(b"You feel a sudden stirring nearby!\x00" as *const u8 as
                      *const libc::c_char);
    } else if sleep != 0 {
        msg_print(b"You hear a sudden stirring in the distance!\x00" as
                      *const u8 as *const libc::c_char);
    };
}
/*
 * Generic genocide race selection
 */
#[no_mangle]
pub unsafe extern "C" fn get_genocide_race(mut msg: cptr,
                                           mut typ: *mut libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    msg_print(msg);
    if tgt_pt(&mut i, &mut j) == 0 { return 0 as libc::c_int as bool_ }
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(j as isize)).offset(i as isize) as
            *mut cave_type;
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
        *typ = (*r_ptr).d_char;
        return 1 as libc::c_int as bool_
    }
    msg_print(b"You must select a monster.\x00" as *const u8 as
                  *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
/*
 * Inflict dam damage of type typee to all monster of the given race
 */
#[no_mangle]
pub unsafe extern "C" fn invoke(mut dam: libc::c_int, mut typee: libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut typ: libc::c_char = 0;
    let mut result: bool_ = 0 as libc::c_int as bool_;
    let mut msec: libc::c_int =
        delay_factor as libc::c_int * delay_factor as libc::c_int *
            delay_factor as libc::c_int;
    if dungeon_flags2 as libc::c_long & 0x200 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- when you are fated to die, you cant cheat :) */
    if dungeon_type as libc::c_int == DUNGEON_DEATH {
        msg_print(b"A mysterious force stops the genocide.\x00" as *const u8
                      as *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Get a monster symbol */
    if get_genocide_race(b"Target a monster to select the race to invoke on.\x00"
                             as *const u8 as *const libc::c_char, &mut typ) ==
           0 {
        return 0 as libc::c_int as bool_
    }
    /* Delete the monsters of that "type" */
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
        /* Paranoia -- Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Hack -- Skip Unique Monsters */
            if !((*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0) {
                /* Hack -- Skip Quest Monsters */
                if !((*m_ptr).mflag & 0x2 as libc::c_int != 0) {
                    /* Skip "wrong" monsters */
                    if !((*r_ptr).d_char as libc::c_int != typ as libc::c_int)
                       {
                        project_m(0 as libc::c_int, 0 as libc::c_int,
                                  (*m_ptr).fy as libc::c_int,
                                  (*m_ptr).fx as libc::c_int, dam, typee);
                        /* Visual feedback */
                        move_cursor_relative((*p_ptr).py as libc::c_int,
                                             (*p_ptr).px as libc::c_int);
                        /* Window stuff */
                        (*p_ptr).window =
                            ((*p_ptr).window as libc::c_long |
                                 0x8 as libc::c_long) as u32b;
                        /* Handle */
                        handle_stuff();
                        /* Fresh */
                        Term_fresh();
                        /* Delay */
                        Term_xtra(13 as libc::c_int, msec);
                        /* Take note */
                        result = 1 as libc::c_int as bool_
                    }
                }
            }
        }
        i += 1
    }
    return result;
}
/*
 * Delete all non-unique/non-quest monsters of a given "type" from the level
 */
#[no_mangle]
pub unsafe extern "C" fn genocide_aux(mut player_cast: bool_,
                                      mut typ: libc::c_char) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut result: bool_ = 0 as libc::c_int as bool_;
    let mut msec: libc::c_int =
        delay_factor as libc::c_int * delay_factor as libc::c_int *
            delay_factor as libc::c_int;
    let mut dam: libc::c_int = 0 as libc::c_int;
    /* Delete the monsters of that "type" */
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
        /* Paranoia -- Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Hack -- Skip Unique Monsters */
            if !((*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0) {
                /* Hack -- Skip Quest Monsters */
                if !((*m_ptr).mflag & 0x2 as libc::c_int != 0) {
                    /* Skip "wrong" monsters */
                    if !((*r_ptr).d_char as libc::c_int != typ as libc::c_int)
                       {
                        /* Oups */
                        if (*r_ptr).flags2 &
                               0x100 as libc::c_int as libc::c_uint != 0 {
                            let mut wx: libc::c_int = 0;
                            let mut wy: libc::c_int = 0;
                            let mut attempts: libc::c_int =
                                500 as libc::c_int;
                            monster_race_desc(r_name,
                                              (*m_ptr).r_idx as libc::c_int,
                                              0 as libc::c_int);
                            loop  {
                                scatter(&mut wy, &mut wx,
                                        (*m_ptr).fy as libc::c_int,
                                        (*m_ptr).fx as libc::c_int,
                                        10 as libc::c_int);
                                if !(!(wy > 0 as libc::c_int &&
                                           wx > 0 as libc::c_int &&
                                           wy <
                                               cur_hgt as libc::c_int -
                                                   1 as libc::c_int &&
                                           wx <
                                               cur_wid as libc::c_int -
                                                   1 as libc::c_int &&
                                           ((*f_info.offset((*cave[wy as
                                                                       usize].offset(wx
                                                                                         as
                                                                                         isize)).feat
                                                                as
                                                                isize)).flags1
                                                as libc::c_long &
                                                0x10 as libc::c_long != 0 &&
                                                (*cave[wy as
                                                           usize].offset(wx as
                                                                             isize)).feat
                                                    as libc::c_int !=
                                                    0xaf as libc::c_int)) &&
                                         { attempts -= 1; (attempts) != 0 }) {
                                    break ;
                                }
                            }
                            if place_monster_aux(wy, wx,
                                                 (*m_ptr).r_idx as
                                                     libc::c_int,
                                                 0 as libc::c_int as bool_,
                                                 1 as libc::c_int as bool_,
                                                 -(2 as libc::c_int)) != 0 {
                                cmsg_format(14 as libc::c_int as byte_hack,
                                            b"The spell seems to produce an ... interesting effect on the %s.\x00"
                                                as *const u8 as
                                                *const libc::c_char, r_name);
                            }
                            return 1 as libc::c_int as bool_
                        }
                        /* Delete the monster */
                        delete_monster_idx(i);
                        if player_cast != 0 {
                            /* Keep track of damage */
                            dam +=
                                Rand_div(4 as libc::c_int) + 1 as libc::c_int
                        }
                        /* Handle */
                        handle_stuff();
                        /* Fresh */
                        Term_fresh();
                        /* Delay */
                        Term_xtra(13 as libc::c_int, msec);
                        /* Take note */
                        result = 1 as libc::c_int as bool_
                    }
                }
            }
        }
        i += 1
    }
    if player_cast != 0 {
        /* Take damage */
        take_hit(dam,
                 b"the strain of casting Genocide\x00" as *const u8 as
                     *const libc::c_char);
        /* Visual feedback */
        move_cursor_relative((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int);
        /* Redraw */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
        /* Handle */
        handle_stuff();
        /* Fresh */
        Term_fresh();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn genocide(mut player_cast: bool_) -> bool_ {
    let mut typ: libc::c_char = 0;
    if dungeon_flags2 as libc::c_long & 0x200 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- when you are fated to die, you cant cheat :) */
    if dungeon_type as libc::c_int == DUNGEON_DEATH {
        msg_print(b"A mysterious force stops the genocide.\x00" as *const u8
                      as *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Mega-Hack -- Get a monster symbol */
    if get_genocide_race(b"Target a monster to select the race to genocide.\x00"
                             as *const u8 as *const libc::c_char, &mut typ) ==
           0 {
        return 0 as libc::c_int as bool_
    }
    return genocide_aux(player_cast, typ);
}
/*
 * Delete all nearby (non-unique) monsters
 */
#[no_mangle]
pub unsafe extern "C" fn mass_genocide(mut player_cast: bool_) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut result: bool_ = 0 as libc::c_int as bool_;
    let mut msec: libc::c_int =
        delay_factor as libc::c_int * delay_factor as libc::c_int *
            delay_factor as libc::c_int;
    let mut dam: libc::c_int = 0 as libc::c_int;
    if dungeon_flags2 as libc::c_long & 0x200 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- when you are fated to die, you cant cheat :) */
    if dungeon_type as libc::c_int == DUNGEON_DEATH {
        msg_print(b"A mysterious force stops the genocide.\x00" as *const u8
                      as *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Delete the (nearby) monsters */
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
        /* Paranoia -- Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Hack -- Skip unique monsters */
            if !((*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0) {
                /* Hack -- Skip Quest Monsters */
                if !((*m_ptr).mflag & 0x2 as libc::c_int != 0) {
                    /* Skip distant monsters */
                    if !((*m_ptr).cdis as libc::c_int > 20 as libc::c_int) {
                        /* Oups */
                        if (*r_ptr).flags2 &
                               0x100 as libc::c_int as libc::c_uint != 0 {
                            let mut wx: libc::c_int = 0;
                            let mut wy: libc::c_int = 0;
                            let mut attempts: libc::c_int =
                                500 as libc::c_int;
                            monster_race_desc(r_name,
                                              (*m_ptr).r_idx as libc::c_int,
                                              0 as libc::c_int);
                            loop  {
                                scatter(&mut wy, &mut wx,
                                        (*m_ptr).fy as libc::c_int,
                                        (*m_ptr).fx as libc::c_int,
                                        10 as libc::c_int);
                                if !(!(wy > 0 as libc::c_int &&
                                           wx > 0 as libc::c_int &&
                                           wy <
                                               cur_hgt as libc::c_int -
                                                   1 as libc::c_int &&
                                           wx <
                                               cur_wid as libc::c_int -
                                                   1 as libc::c_int &&
                                           ((*f_info.offset((*cave[wy as
                                                                       usize].offset(wx
                                                                                         as
                                                                                         isize)).feat
                                                                as
                                                                isize)).flags1
                                                as libc::c_long &
                                                0x10 as libc::c_long != 0 &&
                                                (*cave[wy as
                                                           usize].offset(wx as
                                                                             isize)).feat
                                                    as libc::c_int !=
                                                    0xaf as libc::c_int)) &&
                                         { attempts -= 1; (attempts) != 0 }) {
                                    break ;
                                }
                            }
                            if place_monster_aux(wy, wx,
                                                 (*m_ptr).r_idx as
                                                     libc::c_int,
                                                 0 as libc::c_int as bool_,
                                                 1 as libc::c_int as bool_,
                                                 -(2 as libc::c_int)) != 0 {
                                cmsg_format(14 as libc::c_int as byte_hack,
                                            b"The spell seems to produce an ... interesting effect on the %s.\x00"
                                                as *const u8 as
                                                *const libc::c_char, r_name);
                            }
                            return 1 as libc::c_int as bool_
                        }
                        /* Delete the monster */
                        delete_monster_idx(i);
                        if player_cast != 0 {
                            /* Keep track of damage. */
                            dam +=
                                Rand_div(3 as libc::c_int) + 1 as libc::c_int
                        }
                        /* Handle */
                        handle_stuff();
                        /* Fresh */
                        Term_fresh();
                        /* Delay */
                        Term_xtra(13 as libc::c_int, msec);
                        /* Note effect */
                        result = 1 as libc::c_int as bool_
                    }
                }
            }
        }
        i += 1
    }
    if player_cast != 0 {
        /* Take damage */
        take_hit(dam,
                 b"the strain of casting Mass Genocide\x00" as *const u8 as
                     *const libc::c_char);
        /* Visual feedback */
        move_cursor_relative((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int);
        /* Redraw */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
        /* Handle */
        handle_stuff();
        /* Fresh */
        Term_fresh();
    }
    return result;
}
/* Probe a monster */
#[no_mangle]
pub unsafe extern "C" fn do_probe(mut m_idx: libc::c_int) {
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    /* Get "the monster" or "something" */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0x4 as libc::c_int);
    /* Describe the monster */
    if wizard == 0 && (*m_ptr).status as libc::c_int != 4 as libc::c_int {
        msg_format(b"%^s has %d hit points.\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr(), (*m_ptr).hp);
    } else {
        let mut i: libc::c_int = 0;
        let mut t_name: [libc::c_char; 80] = [0; 80];
        msg_format(b"%^s has %d(%d) hit points, %d ac, %d speed.\x00" as
                       *const u8 as *const libc::c_char, m_name.as_mut_ptr(),
                   (*m_ptr).hp, (*m_ptr).maxhp, (*m_ptr).ac as libc::c_int,
                   (*m_ptr).mspeed as libc::c_int - 110 as libc::c_int);
        msg_format(b"%^s attacks with:\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr());
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            msg_format(b"    Blow %d: %dd%d\x00" as *const u8 as
                           *const libc::c_char, i,
                       (*m_ptr).blow[i as usize].d_dice as libc::c_int,
                       (*m_ptr).blow[i as usize].d_side as libc::c_int);
            i += 1
        }
        if (*m_ptr).target as libc::c_int > 0 as libc::c_int {
            monster_desc(t_name.as_mut_ptr(),
                         &mut *m_list.offset((*m_ptr).target as isize),
                         0x4 as libc::c_int);
        } else if (*m_ptr).target == 0 {
            sprintf(t_name.as_mut_ptr(),
                    b"you\x00" as *const u8 as *const libc::c_char);
        } else {
            sprintf(t_name.as_mut_ptr(),
                    b"nothing\x00" as *const u8 as *const libc::c_char);
        }
        msg_format(b"%^s target is %s.\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr(),
                   t_name.as_mut_ptr());
        msg_format(b"%^s has %ld exp and needs %ld.\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr(), (*m_ptr).exp,
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
                        }) *
                       (if (*m_ptr).level as libc::c_int + 1 as libc::c_int >
                               150 as libc::c_int {
                            150 as libc::c_int
                        } else {
                            ((*m_ptr).level as libc::c_int) + 1 as libc::c_int
                        }) * 6 as libc::c_int);
    }
    /* Learn all of the non-spell, non-treasure flags */
    lore_do_probe(m_idx);
}
/*
 * Probe nearby monsters
 */
#[no_mangle]
pub unsafe extern "C" fn probing() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut probe: bool_ = 0 as libc::c_int as bool_;
    /* Probe all (nearby) monsters */
    i = 1 as libc::c_int;
    while i < m_max as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Paranoia -- Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Require line of sight */
            if (*cave[(*m_ptr).fy as usize].offset((*m_ptr).fx as isize)).info
                   as libc::c_int & 0x20 as libc::c_int != 0 as libc::c_int {
                /* Probe visible monsters */
                if (*m_ptr).ml != 0 {
                    /* Start the message */
                    if probe == 0 {
                        msg_print(b"Probing...\x00" as *const u8 as
                                      *const libc::c_char);
                    }
                    /* Actualy probe */
                    do_probe(i);
                    /* Probe worked */
                    probe = 1 as libc::c_int as bool_
                }
            }
        }
        i += 1
    }
    /* Done */
    if probe != 0 {
        msg_print(b"That\'s all.\x00" as *const u8 as *const libc::c_char);
    }
    /* Result */
    return probe;
}
/*
 * Wipe -- Empties a part of the dungeon
 */
#[no_mangle]
pub unsafe extern "C" fn wipe(mut y1: libc::c_int, mut x1: libc::c_int,
                              mut r: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    if dungeon_flags2 as libc::c_long & 0x200 as libc::c_long != 0 {
        msg_print(b"Not on special levels!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if (*p_ptr).inside_quest != 0 { return }
    /* Big area of affect */
    y = y1 - r;
    while y <= y1 + r {
        x = x1 - r;
        while x <= x1 + r {
            /* Skip illegal grids */
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Extract the distance */
                k = distance(y1, x1, y, x);
                /* Stay in the circle of death */
                if !(k > r) {
                    /* Access the grid */
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset(y as
                                                             isize)).offset(x
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Lose room and vault */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int &
                             !(0x8 as libc::c_int | 0x4 as libc::c_int)) as
                            u16b;
                    /* Lose light and knowledge */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int &
                             !(0x1 as libc::c_int | 0x2 as libc::c_int)) as
                            u16b;
                    if (*m_list.offset((*c_ptr).m_idx as isize)).status as
                           libc::c_int != 4 as libc::c_int {
                        delete_monster(y, x);
                    }
                    delete_object(y, x);
                    place_floor_convert_glass(y, x);
                }
            }
            x += 1
        }
        y += 1
    }
    /* Hack -- Affect player */
    if flag != 0 {
        /* Message */
        msg_print(b"There is a searing blast of light!\x00" as *const u8 as
                      *const libc::c_char);
        /* Blind the player */
        if (*p_ptr).resist_blind == 0 && (*p_ptr).resist_lite == 0 {
            /* Become blind */
            set_blind((*p_ptr).blind as libc::c_int + 10 as libc::c_int +
                          (Rand_div(10 as libc::c_int) + 1 as libc::c_int));
        }
    }
    /* Mega-Hack -- Forget the view */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x10000 as libc::c_long) as u32b;
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                  0x200000 as libc::c_long)) as u32b;
    /* Update the monsters */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * The spell of destruction
 *
 * This spell "deletes" monsters (instead of "killing" them).
 *
 * Later we may use one function for both "destruction" and
 * "earthquake" by using the "full" to select "destruction".
 */
#[no_mangle]
pub unsafe extern "C" fn destroy_area(mut y1: libc::c_int,
                                      mut x1: libc::c_int, mut r: libc::c_int,
                                      mut full: bool_, mut bypass: bool_) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    /* XXX XXX */
    full =
        if full as libc::c_int != 0 {
            full as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    if dungeon_flags2 as libc::c_long & 0x200 as libc::c_long != 0 {
        msg_print(b"Not on special levels!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if (*p_ptr).inside_quest as libc::c_int != 0 && bypass == 0 { return }
    /* Big area of affect */
    y = y1 - r;
    while y <= y1 + r {
        x = x1 - r;
        while x <= x1 + r {
            /* Skip illegal grids */
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Extract the distance */
                k = distance(y1, x1, y, x);
                /* Stay in the circle of death */
                if !(k > r) {
                    /* Access the grid */
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset(y as
                                                             isize)).offset(x
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Lose room and vault */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int &
                             !(0x8 as libc::c_int | 0x4 as libc::c_int)) as
                            u16b;
                    /* Lose light and knowledge */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int &
                             !(0x1 as libc::c_int | 0x2 as libc::c_int)) as
                            u16b;
                    /* Hack -- Notice player affect */
                    if x == (*p_ptr).px as libc::c_int &&
                           y == (*p_ptr).py as libc::c_int {
                        /* Hurt the player later */
                        flag = 1 as libc::c_int as bool_
                    } else if !(y == y1 && x == x1) {
                        /* Hack -- Skip the epicenter */
                        /* Delete the monster (if any) */
                        if (*m_list.offset((*c_ptr).m_idx as isize)).status as
                               libc::c_int != 4 as libc::c_int ||
                               (*m_list.offset((*c_ptr).m_idx as isize)).mflag
                                   & 0x2 as libc::c_int == 0 ||
                               (*m_list.offset((*c_ptr).m_idx as isize)).mflag
                                   & 0x200 as libc::c_int == 0 {
                            delete_monster(y, x);
                        }
                        /* Destroy "valid" grids */
                        if cave_valid_bold(y, x) != 0 {
                            /* Delete objects */
                            delete_object(y, x);
                            /* Wall (or floor) type */
                            t = Rand_div(200 as libc::c_int);
                            /* Granite */
                            if t < 20 as libc::c_int {
                                /* Create granite wall */
                                cave_set_feat(y, x, 0x38 as libc::c_int);
                            } else if t < 70 as libc::c_int {
                                /* Quartz */
                                /* Create quartz vein */
                                cave_set_feat(y, x, 0x33 as libc::c_int);
                            } else if t < 100 as libc::c_int {
                                /* Magma */
                                /* Create magma vein */
                                cave_set_feat(y, x, 0x32 as libc::c_int);
                            } else {
                                /* Floor */
                                /* Create floor */
                                cave_set_feat(y, x, 0x1 as libc::c_int);
                            }
                        }
                    }
                }
            }
            /* Do not hurt this grid */
            x += 1
        }
        y += 1
    }
    /* Hack -- Affect player */
    if flag != 0 {
        /* Message */
        msg_print(b"There is a searing blast of light!\x00" as *const u8 as
                      *const libc::c_char);
        /* Blind the player */
        if (*p_ptr).resist_blind == 0 && (*p_ptr).resist_lite == 0 {
            /* Become blind */
            set_blind((*p_ptr).blind as libc::c_int + 10 as libc::c_int +
                          (Rand_div(10 as libc::c_int) + 1 as libc::c_int));
        }
    }
    /* Mega-Hack -- Forget the view */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x10000 as libc::c_long) as u32b;
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                  0x200000 as libc::c_long)) as u32b;
    /* Update the monsters */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Induce an "earthquake" of the given radius at the given location.
 *
 * This will turn some walls into floors and some floors into walls.
 *
 * The player will take damage and "jump" into a safe grid if possible,
 * otherwise, he will "tunnel" through the rubble instantaneously.
 *
 * Monsters will take damage, and "jump" into a safe grid if possible,
 * otherwise they will be "buried" in the rubble, disappearing from
 * the level in the same way that they do when genocided.
 *
 * Note that thus the player and monsters (except eaters of walls and
 * passers through walls) will never occupy the same grid as a wall.
 * Note that as of now (2.7.8) no monster may occupy a "wall" grid, even
 * for a single turn, unless that monster can pass_walls or kill_walls.
 * This has allowed massive simplification of the "monster" code.
 */
#[no_mangle]
pub unsafe extern "C" fn earthquake(mut cy: libc::c_int, mut cx: libc::c_int,
                                    mut r: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut xx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut oy: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    let mut damage: libc::c_int = 0 as libc::c_int;
    let mut sn: libc::c_int = 0 as libc::c_int;
    let mut sy: libc::c_int = 0 as libc::c_int;
    let mut sx: libc::c_int = 0 as libc::c_int;
    let mut hurt: bool_ = 0 as libc::c_int as bool_;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut map: [[bool_; 32]; 32] = [[0; 32]; 32];
    if (*p_ptr).inside_quest != 0 { return }
    /* Paranoia -- Enforce maximum range */
    if r > 12 as libc::c_int { r = 12 as libc::c_int }
    /* Clear the "maximal blast" area */
    y = 0 as libc::c_int;
    while y < 32 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 32 as libc::c_int {
            map[y as usize][x as usize] = 0 as libc::c_int as bool_;
            x += 1
        }
        y += 1
    }
    /* Check around the epicenter */
    dy = -r;
    while dy <= r {
        dx = -r;
        while dx <= r {
            /* Extract the location */
            yy = cy + dy;
            xx = cx + dx;
            /* Skip illegal grids */
            if yy > 0 as libc::c_int && xx > 0 as libc::c_int &&
                   yy < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   xx < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Skip distant grids */
                if !(distance(cy, cx, yy, xx) > r) {
                    /* Access the grid */
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset(yy as
                                                             isize)).offset(xx
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Lose room and vault */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int &
                             !(0x8 as libc::c_int | 0x4 as libc::c_int)) as
                            u16b;
                    /* Lose light and knowledge */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int &
                             !(0x2 as libc::c_int | 0x1 as libc::c_int)) as
                            u16b;
                    /* Skip the epicenter */
                    if !(dx == 0 && dy == 0) {
                        /* Skip most grids */
                        if !(Rand_div(100 as libc::c_int) < 85 as libc::c_int)
                           {
                            /* Damage this grid */
                            map[(16 as libc::c_int + yy - cy) as
                                    usize][(16 as libc::c_int + xx - cx) as
                                               usize] =
                                1 as libc::c_int as bool_;
                            /* Hack -- Take note of player damage */
                            if yy == (*p_ptr).py as libc::c_int &&
                                   xx == (*p_ptr).px as libc::c_int {
                                hurt = 1 as libc::c_int as bool_
                            }
                        }
                    }
                }
            }
            dx += 1
        }
        dy += 1
    }
    /* First, affect the player (if necessary) */
    if hurt as libc::c_int != 0 && (*p_ptr).wraith_form == 0 {
        /* Check around the player */
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            /* Access the location */
            y = (*p_ptr).py as libc::c_int + ddy[i as usize] as libc::c_int;
            x = (*p_ptr).px as libc::c_int + ddx[i as usize] as libc::c_int;
            /* Skip non-empty grids */
            if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                       != 0xaf as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
                   !(y == (*p_ptr).py as libc::c_int &&
                         x == (*p_ptr).px as libc::c_int) {
                /* Important -- Skip "quake" grids */
                if !(map[(16 as libc::c_int + y - cy) as
                             usize][(16 as libc::c_int + x - cx) as usize] !=
                         0) {
                    /* Count "safe" grids */
                    sn += 1;
                    /* Randomize choice */
                    if !(Rand_div(sn) > 0 as libc::c_int) {
                        /* Save the safe location */
                        sy = y;
                        sx = x
                    }
                }
            }
            i += 1
        }
        /* Random message */
        match Rand_div(3 as libc::c_int) + 1 as libc::c_int {
            1 => {
                msg_print(b"The cave ceiling collapses!\x00" as *const u8 as
                              *const libc::c_char);
            }
            2 => {
                msg_print(b"The cave floor twists in an unnatural way!\x00" as
                              *const u8 as *const libc::c_char);
            }
            _ => {
                msg_print(b"The cave quakes!  You are pummeled with debris!\x00"
                              as *const u8 as *const libc::c_char);
            }
        }
        /* Hurt the player a lot */
        if sn == 0 {
            /* Message and damage */
            msg_print(b"You are severely crushed!\x00" as *const u8 as
                          *const libc::c_char);
            damage = 300 as libc::c_int
        } else {
            /* Destroy the grid, and push the player to safety */
            /* Calculate results */
            match Rand_div(3 as libc::c_int) + 1 as libc::c_int {
                1 => {
                    msg_print(b"You nimbly dodge the blast!\x00" as *const u8
                                  as *const libc::c_char);
                    damage = 0 as libc::c_int
                }
                2 => {
                    msg_print(b"You are bashed by rubble!\x00" as *const u8 as
                                  *const libc::c_char);
                    damage =
                        damroll(10 as libc::c_int as s16b,
                                4 as libc::c_int as s16b);
                    set_stun((*p_ptr).stun as libc::c_int +
                                 (Rand_div(50 as libc::c_int) +
                                      1 as libc::c_int));
                }
                3 => {
                    msg_print(b"You are crushed between the floor and ceiling!\x00"
                                  as *const u8 as *const libc::c_char);
                    damage =
                        damroll(10 as libc::c_int as s16b,
                                4 as libc::c_int as s16b);
                    set_stun((*p_ptr).stun as libc::c_int +
                                 (Rand_div(50 as libc::c_int) +
                                      1 as libc::c_int));
                }
                _ => { }
            }
            oy = (*p_ptr).py as libc::c_int;
            ox = (*p_ptr).px as libc::c_int;
            (*p_ptr).py = sy as s16b;
            (*p_ptr).px = sx as s16b;
            lite_spot(oy, ox);
            lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
            verify_panel();
        }
        /* Save the old location */
        /* Move the player to the safe location */
        /* Redraw the old spot */
        /* Redraw the new spot */
        /* Check for new panel */
        /* Important -- no wall on player */
        map[(16 as libc::c_int + (*p_ptr).py as libc::c_int - cy) as
                usize][(16 as libc::c_int + (*p_ptr).px as libc::c_int - cx)
                           as usize] = 0 as libc::c_int as bool_;
        /* Semi-wraiths have to be hurt *some*, says DG */
        if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                (*spp_ptr).flags1) as libc::c_long & 0x20000 as libc::c_long
               != 0 {
            damage /= 4 as libc::c_int
        }
        /* Take some damage */
        if damage != 0 {
            take_hit(damage,
                     b"an earthquake\x00" as *const u8 as
                         *const libc::c_char);
        }
    }
    /* Examine the quaked region */
    dy = -r;
    while dy <= r {
        dx = -r;
        while dx <= r {
            /* Extract the location */
            yy = cy + dy;
            xx = cx + dx;
            /* Skip unaffected grids */
            if !(map[(16 as libc::c_int + yy - cy) as
                         usize][(16 as libc::c_int + xx - cx) as usize] == 0)
               {
                /* Access the grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(yy as
                                                         isize)).offset(xx as
                                                                            isize)
                        as *mut cave_type;
                /* Process monsters */
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
                    /* Most monsters cannot co-exist with rock */
                    if (*r_ptr).flags2 &
                           0x80000 as libc::c_int as libc::c_uint == 0 &&
                           (*r_ptr).flags2 &
                               0x40000 as libc::c_int as libc::c_uint == 0 {
                        let mut m_name: [libc::c_char; 80] = [0; 80];
                        /* Assume not safe */
                        sn = 0 as libc::c_int;
                        /* Monster can move to escape the wall */
                        if (*r_ptr).flags1 &
                               0x20000 as libc::c_int as libc::c_uint == 0 {
                            /* Look for safety */
                            i = 0 as libc::c_int;
                            while i < 8 as libc::c_int {
                                /* Access the grid */
                                y = yy + ddy[i as usize] as libc::c_int;
                                x = xx + ddx[i as usize] as libc::c_int;
                                /* Skip non-empty grids */
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
                                   {
                                    /* Hack -- no safety on glyph of warding */
                                    if !((*cave[y as
                                                    usize].offset(x as
                                                                      isize)).feat
                                             as libc::c_int ==
                                             0x3 as libc::c_int) {
                                        if !((*cave[y as
                                                        usize].offset(x as
                                                                          isize)).feat
                                                 as libc::c_int ==
                                                 0x40 as libc::c_int) {
                                            /* ... nor on the Pattern */
                                            if !((*cave[y as
                                                            usize].offset(x as
                                                                              isize)).feat
                                                     as libc::c_int <=
                                                     0x49 as libc::c_int &&
                                                     (*cave[y as
                                                                usize].offset(x
                                                                                  as
                                                                                  isize)).feat
                                                         as libc::c_int >=
                                                         0x41 as libc::c_int)
                                               {
                                                /* Important -- Skip "quake" grids */
                                                if !(map[(16 as libc::c_int +
                                                              y - cy) as
                                                             usize][(16 as
                                                                         libc::c_int
                                                                         + x -
                                                                         cx)
                                                                        as
                                                                        usize]
                                                         != 0) {
                                                    /* Count "safe" grids */
                                                    sn += 1;
                                                    /* Randomize choice */
                                                    if !(Rand_div(sn) >
                                                             0 as libc::c_int)
                                                       {
                                                        /* Save the safe grid */
                                                        sy = y;
                                                        sx = x
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                i += 1
                            }
                        }
                        /* Describe the monster */
                        monster_desc(m_name.as_mut_ptr(), m_ptr,
                                     0 as libc::c_int);
                        /* Scream in pain */
                        msg_format(b"%^s wails out in pain!\x00" as *const u8
                                       as *const libc::c_char,
                                   m_name.as_mut_ptr());
                        /* Take damage from the quake */
                        damage =
                            if sn != 0 {
                                damroll(4 as libc::c_int as s16b,
                                        8 as libc::c_int as s16b)
                            } else { 200 as libc::c_int };
                        /* Monster is certainly awake */
                        (*m_ptr).csleep = 0 as libc::c_int as s16b;
                        /* Apply damage directly */
                        (*m_ptr).hp -= damage;
                        /* Delete (not kill) "dead" monsters */
                        if (*m_ptr).hp < 0 as libc::c_int {
                            /* Message */
                            msg_format(b"%^s is embedded in the rock!\x00" as
                                           *const u8 as *const libc::c_char,
                                       m_name.as_mut_ptr());
                            /* Delete the monster */
                            delete_monster(yy, xx);
                            /* No longer safe */
                            sn = 0 as libc::c_int
                        }
                        /* Hack -- Escape from the rock */
                        if sn != 0 {
                            let mut m_idx: libc::c_int =
                                (*cave[yy as usize].offset(xx as isize)).m_idx
                                    as libc::c_int;
                            /* Update the new location */
                            (*cave[sy as usize].offset(sx as isize)).m_idx =
                                m_idx as s16b;
                            /* Update the old location */
                            (*cave[yy as usize].offset(xx as isize)).m_idx =
                                0 as libc::c_int as s16b;
                            /* Move the monster */
                            (*m_ptr).fy = sy as byte_hack;
                            (*m_ptr).fx = sx as byte_hack;
                            /* Update the monster (new location) */
                            update_mon(m_idx, 1 as libc::c_int as bool_);
                            /* Redraw the old grid */
                            lite_spot(yy, xx);
                            /* Redraw the new grid */
                            lite_spot(sy, sx);
                        }
                    }
                }
            }
            dx += 1
        }
        dy += 1
    }
    /* Examine the quaked region */
    dy = -r;
    while dy <= r {
        dx = -r;
        while dx <= r {
            /* Extract the location */
            yy = cy + dy;
            xx = cx + dx;
            /* Skip unaffected grids */
            if !(map[(16 as libc::c_int + yy - cy) as
                         usize][(16 as libc::c_int + xx - cx) as usize] == 0)
               {
                /* Access the cave grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(yy as
                                                         isize)).offset(xx as
                                                                            isize)
                        as *mut cave_type;
                /* Paranoia -- never affect player */
                if !(yy == (*p_ptr).py as libc::c_int &&
                         xx == (*p_ptr).px as libc::c_int) {
                    /* Destroy location (if valid) */
                    if cave_valid_bold(yy, xx) != 0 {
                        let mut floor: bool_ =
                            ((*f_info.offset((*cave[yy as
                                                        usize].offset(xx as
                                                                          isize)).feat
                                                 as isize)).flags1 as
                                 libc::c_long & 0x10 as libc::c_long != 0 &&
                                 (*cave[yy as usize].offset(xx as isize)).feat
                                     as libc::c_int != 0xaf as libc::c_int) as
                                libc::c_int as bool_;
                        /* Delete objects */
                        delete_object(yy, xx);
                        /* Wall (or floor) type */
                        t =
                            if floor as libc::c_int != 0 {
                                Rand_div(100 as libc::c_int)
                            } else { 200 as libc::c_int };
                        /* Granite */
                        if t < 20 as libc::c_int {
                            /* Create granite wall */
                            cave_set_feat(yy, xx, 0x38 as libc::c_int);
                        } else if t < 70 as libc::c_int {
                            /* Quartz */
                            /* Create quartz vein */
                            cave_set_feat(yy, xx, 0x33 as libc::c_int);
                        } else if t < 100 as libc::c_int {
                            /* Magma */
                            /* Create magma vein */
                            cave_set_feat(yy, xx, 0x32 as libc::c_int);
                        } else {
                            /* Floor */
                            /* Create floor */
                            cave_set_feat(yy, xx, 0x1 as libc::c_int);
                        }
                    }
                }
            }
            dx += 1
        }
        dy += 1
    }
    /* Mega-Hack -- Forget the view */
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
}
/*
 * This routine clears the entire "temp" set.
 *
 * This routine will Perma-Lite all "temp" grids.
 *
 * This routine is used (only) by "lite_room()"
 *
 * Dark grids are illuminated.
 *
 * Also, process all affected monsters.
 *
 * SMART monsters always wake up when illuminated
 * NORMAL monsters wake up 1/4 the time when illuminated
 * STUPID monsters wake up 1/10 the time when illuminated
 */
unsafe extern "C" fn cave_temp_room_lite() {
    let mut i: libc::c_int = 0;
    /* Apply flag changes */
    i = 0 as libc::c_int;
    while i < temp_n as libc::c_int {
        let mut y: libc::c_int = temp_y[i as usize] as libc::c_int;
        let mut x: libc::c_int = temp_x[i as usize] as libc::c_int;
        let mut c_ptr: *mut cave_type =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* No longer in the array */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int & !(0x40 as libc::c_int)) as u16b;
        /* Update only non-CAVE_GLOW grids */
		/* if (c_ptr->info & (CAVE_GLOW)) continue; */
        /* Perma-Lite */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as u16b;
        i += 1
    }
    /* Fully update the visuals */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x10000 as libc::c_long | 0x100000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x200000 as libc::c_long)) as
            u32b;
    /* Update stuff */
    update_stuff();
    /* Process the grids */
    i = 0 as libc::c_int;
    while i < temp_n as libc::c_int {
        let mut y_0: libc::c_int = temp_y[i as usize] as libc::c_int;
        let mut x_0: libc::c_int = temp_x[i as usize] as libc::c_int;
        let mut c_ptr_0: *mut cave_type =
            &mut *(*cave.as_mut_ptr().offset(y_0 as
                                                 isize)).offset(x_0 as isize)
                as *mut cave_type;
        /* Redraw the grid */
        lite_spot(y_0, x_0);
        /* Process affected monsters */
        if (*c_ptr_0).m_idx != 0 {
            let mut chance: libc::c_int = 25 as libc::c_int;
            let mut m_ptr: *mut monster_type =
                &mut *m_list.offset((*c_ptr_0).m_idx as isize) as
                    *mut monster_type;
            let mut r_ptr: *mut monster_race =
                if !(*m_ptr).sr_ptr.is_null() {
                    (*m_ptr).sr_ptr
                } else {
                    race_info_idx((*m_ptr).r_idx as libc::c_int,
                                  (*m_ptr).ego as libc::c_int)
                };
            /* Update the monster */
            update_mon((*c_ptr_0).m_idx as libc::c_int,
                       0 as libc::c_int as bool_);
            /* Stupid monsters rarely wake up */
            if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 {
                chance = 10 as libc::c_int
            }
            /* Smart monsters always wake up */
            if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint != 0 {
                chance = 100 as libc::c_int
            }
            /* Sometimes monsters wake up */
            if (*m_ptr).csleep as libc::c_int != 0 &&
                   Rand_div(100 as libc::c_int) < chance {
                /* Wake up! */
                (*m_ptr).csleep = 0 as libc::c_int as s16b;
                /* Notice the "waking up" */
                if (*m_ptr).ml != 0 {
                    let mut m_name: [libc::c_char; 80] = [0; 80];
                    /* Acquire the monster name */
                    monster_desc(m_name.as_mut_ptr(), m_ptr,
                                 0 as libc::c_int);
                    /* Dump a message */
                    msg_format(b"%^s wakes up.\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                }
            }
        }
        i += 1
    }
    /* None left */
    temp_n = 0 as libc::c_int as s16b;
}
/*
 * This routine clears the entire "temp" set.
 *
 * This routine will "darken" all "temp" grids.
 *
 * In addition, some of these grids will be "unmarked".
 *
 * This routine is used (only) by "unlite_room()"
 *
 * Also, process all affected monsters
 */
unsafe extern "C" fn cave_temp_room_unlite() {
    let mut i: libc::c_int = 0;
    /* Apply flag changes */
    i = 0 as libc::c_int;
    while i < temp_n as libc::c_int {
        let mut y: libc::c_int = temp_y[i as usize] as libc::c_int;
        let mut x: libc::c_int = temp_x[i as usize] as libc::c_int;
        let mut c_ptr: *mut cave_type =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* No longer in the array */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int & !(0x40 as libc::c_int)) as u16b;
        /* Darken the grid */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int & !(0x2 as libc::c_int)) as u16b;
        /* Hack -- Forget "boring" grids */
        if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long
                   & 0x100 as libc::c_long == 0 {
            /* Forget the grid */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int & !(0x1 as libc::c_int)) as u16b
            /* Notice */
			/* note_spot(y, x); */
        }
        i += 1
    }
    /* Fully update the visuals */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x10000 as libc::c_long | 0x100000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x200000 as libc::c_long)) as
            u32b;
    /* Update stuff */
    update_stuff();
    /* Process the grids */
    i = 0 as libc::c_int;
    while i < temp_n as libc::c_int {
        let mut y_0: libc::c_int = temp_y[i as usize] as libc::c_int;
        let mut x_0: libc::c_int = temp_x[i as usize] as libc::c_int;
        /* Redraw the grid */
        lite_spot(y_0, x_0);
        i += 1
    }
    /* None left */
    temp_n = 0 as libc::c_int as s16b;
}
/*
 * Aux function -- see below
 */
unsafe extern "C" fn cave_temp_room_aux(mut y: libc::c_int,
                                        mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Avoid infinite recursion */
    if (*c_ptr).info as libc::c_int & 0x40 as libc::c_int != 0 { return }
    /* Do not "leave" the current room */
    if (*c_ptr).info as libc::c_int & 0x8 as libc::c_int == 0 { return }
    /* Paranoia -- verify space */
    if temp_n as libc::c_int == 16384 as libc::c_int { return }
    /* Mark the grid as "seen" */
    (*c_ptr).info =
        ((*c_ptr).info as libc::c_int | 0x40 as libc::c_int) as u16b;
    /* Add it to the "seen" set */
    temp_y[temp_n as usize] = y as byte_hack;
    temp_x[temp_n as usize] = x as byte_hack;
    temp_n += 1;
}
/*
 * Illuminate any room containing the given location.
 */
#[no_mangle]
pub unsafe extern "C" fn lite_room(mut y1: libc::c_int, mut x1: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    /* Add the initial grid */
    cave_temp_room_aux(y1, x1);
    /* While grids are in the queue, add their neighbors */
    i = 0 as libc::c_int;
    while i < temp_n as libc::c_int {
        x = temp_x[i as usize] as libc::c_int;
        y = temp_y[i as usize] as libc::c_int;
        /* Walls get lit, but stop light */
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int {
            /* Spread adjacent */
            cave_temp_room_aux(y + 1 as libc::c_int, x);
            cave_temp_room_aux(y - 1 as libc::c_int, x);
            cave_temp_room_aux(y, x + 1 as libc::c_int);
            cave_temp_room_aux(y, x - 1 as libc::c_int);
            /* Spread diagonal */
            cave_temp_room_aux(y + 1 as libc::c_int, x + 1 as libc::c_int);
            cave_temp_room_aux(y - 1 as libc::c_int, x - 1 as libc::c_int);
            cave_temp_room_aux(y - 1 as libc::c_int, x + 1 as libc::c_int);
            cave_temp_room_aux(y + 1 as libc::c_int, x - 1 as libc::c_int);
        }
        i += 1
    }
    /* Now, lite them all up at once */
    cave_temp_room_lite();
}
/*
 * Darken all rooms containing the given location
 */
#[no_mangle]
pub unsafe extern "C" fn unlite_room(mut y1: libc::c_int,
                                     mut x1: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    /* Add the initial grid */
    cave_temp_room_aux(y1, x1);
    /* Spread, breadth first */
    i = 0 as libc::c_int;
    while i < temp_n as libc::c_int {
        x = temp_x[i as usize] as libc::c_int;
        y = temp_y[i as usize] as libc::c_int;
        /* Walls get dark, but stop darkness */
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int {
            /* Spread adjacent */
            cave_temp_room_aux(y + 1 as libc::c_int, x);
            cave_temp_room_aux(y - 1 as libc::c_int, x);
            cave_temp_room_aux(y, x + 1 as libc::c_int);
            cave_temp_room_aux(y, x - 1 as libc::c_int);
            /* Spread diagonal */
            cave_temp_room_aux(y + 1 as libc::c_int, x + 1 as libc::c_int);
            cave_temp_room_aux(y - 1 as libc::c_int, x - 1 as libc::c_int);
            cave_temp_room_aux(y - 1 as libc::c_int, x + 1 as libc::c_int);
            cave_temp_room_aux(y + 1 as libc::c_int, x - 1 as libc::c_int);
        }
        i += 1
    }
    /* Now, darken them all at once */
    cave_temp_room_unlite();
}
/*
 * Hack -- call light around the player
 * Affect all monsters in the projection radius
 */
#[no_mangle]
pub unsafe extern "C" fn lite_area(mut dam: libc::c_int, mut rad: libc::c_int)
 -> bool_ {
    let mut flg: libc::c_int = 0x10 as libc::c_int | 0x40 as libc::c_int;
    /* Hack -- Message */
    if (*p_ptr).blind == 0 {
        msg_print(b"You are surrounded by a white light.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Hook into the "project()" function */
    project(0 as libc::c_int, rad, (*p_ptr).py as libc::c_int,
            (*p_ptr).px as libc::c_int, dam, 17 as libc::c_int, flg);
    /* Lite up the room */
    lite_room((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    /* Assume seen */
    return 1 as libc::c_int as bool_;
}
/*
 * Hack -- call darkness around the player
 * Affect all monsters in the projection radius
 */
#[no_mangle]
pub unsafe extern "C" fn unlite_area(mut dam: libc::c_int,
                                     mut rad: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x10 as libc::c_int | 0x40 as libc::c_int;
    /* Hack -- Message */
    if (*p_ptr).blind == 0 {
        msg_print(b"Darkness surrounds you.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Hook into the "project()" function */
    project(0 as libc::c_int, rad, (*p_ptr).py as libc::c_int,
            (*p_ptr).px as libc::c_int, dam, 18 as libc::c_int, flg);
    /* Lite up the room */
    unlite_room((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    /* Assume seen */
    return 1 as libc::c_int as bool_;
}
/*
 * Cast a ball spell
 * Stop if we hit a monster, act as a "ball"
 * Allow "target" mode to pass over monsters
 * Affect grids, objects, and monsters
 */
#[no_mangle]
pub unsafe extern "C" fn fire_ball(mut typ: libc::c_int, mut dir: libc::c_int,
                                   mut dam: libc::c_int, mut rad: libc::c_int)
 -> bool_ {
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut flg: libc::c_int =
        0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
            0x40 as libc::c_int;
    /* Use the given direction */
    tx =
        (*p_ptr).px as libc::c_int +
            99 as libc::c_int * ddx[dir as usize] as libc::c_int;
    ty =
        (*p_ptr).py as libc::c_int +
            99 as libc::c_int * ddy[dir as usize] as libc::c_int;
    /* Hack -- Use an actual "target" */
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        flg &= !(0x8 as libc::c_int);
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int
    }
    /* Analyze the "dir" and the "target".  Hurt items on floor. */
    return project(0 as libc::c_int,
                   if rad > 16 as libc::c_int {
                       16 as libc::c_int
                   } else { rad }, ty, tx, dam, typ, flg);
}
/*
 * Cast a cloud spell
 * Stop if we hit a monster, act as a "ball"
 * Allow "target" mode to pass over monsters
 * Affect grids, objects, and monsters
 */
#[no_mangle]
pub unsafe extern "C" fn fire_cloud(mut typ: libc::c_int,
                                    mut dir: libc::c_int,
                                    mut dam: libc::c_int,
                                    mut rad: libc::c_int,
                                    mut time: libc::c_int) -> bool_ {
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut flg: libc::c_int =
        0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
            0x40 as libc::c_int | 0x10000 as libc::c_int;
    /* Use the given direction */
    tx =
        (*p_ptr).px as libc::c_int +
            99 as libc::c_int * ddx[dir as usize] as libc::c_int;
    ty =
        (*p_ptr).py as libc::c_int +
            99 as libc::c_int * ddy[dir as usize] as libc::c_int;
    /* Hack -- Use an actual "target" */
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        flg &= !(0x8 as libc::c_int);
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int
    }
    project_time = time;
    /* Analyze the "dir" and the "target".  Hurt items on floor. */
    return project(0 as libc::c_int,
                   if rad > 16 as libc::c_int {
                       16 as libc::c_int
                   } else { rad }, ty, tx, dam, typ, flg);
}
/*
 * Cast a wave spell
 * Stop if we hit a monster, act as a "ball"
 * Allow "target" mode to pass over monsters
 * Affect grids, objects, and monsters
 */
#[no_mangle]
pub unsafe extern "C" fn fire_wave(mut typ: libc::c_int, mut dir: libc::c_int,
                                   mut dam: libc::c_int, mut rad: libc::c_int,
                                   mut time: libc::c_int, mut eff: s32b)
 -> bool_ {
    project_time_effect = eff;
    return fire_cloud(typ, dir, dam, rad, time);
}
/*
 * Cast a persistant beam spell
 * Pass through monsters, as a "beam"
 * Affect monsters (not grids or objects)
 */
#[no_mangle]
pub unsafe extern "C" fn fire_wall(mut typ: libc::c_int, mut dir: libc::c_int,
                                   mut dam: libc::c_int,
                                   mut time: libc::c_int) -> bool_ {
    let mut flg: libc::c_int =
        0x2 as libc::c_int | 0x40 as libc::c_int | 0x10000 as libc::c_int |
            0x10 as libc::c_int;
    project_time = time;
    return project_hook(typ, dir, dam, flg);
}
/*
 * Cast a druidistic ball spell
 * Stop if we hit a monster, act as a "ball"
 * Allow "target" mode to pass over monsters
 * Affect grids, objects, and monsters
 */
#[no_mangle]
pub unsafe extern "C" fn fire_druid_ball(mut typ: libc::c_int,
                                         mut dir: libc::c_int,
                                         mut dam: libc::c_int,
                                         mut rad: libc::c_int) -> bool_ {
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut flg: libc::c_int =
        0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
            0x40 as libc::c_int | 0x4000 as libc::c_int;
    /* Use the given direction */
    tx =
        (*p_ptr).px as libc::c_int +
            99 as libc::c_int * ddx[dir as usize] as libc::c_int;
    ty =
        (*p_ptr).py as libc::c_int +
            99 as libc::c_int * ddy[dir as usize] as libc::c_int;
    /* Hack -- Use an actual "target" */
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        flg &= !(0x8 as libc::c_int);
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int
    }
    /* Analyze the "dir" and the "target".  Hurt items on floor. */
    return project(0 as libc::c_int,
                   if rad > 16 as libc::c_int {
                       16 as libc::c_int
                   } else { rad }, ty, tx, dam, typ, flg);
}
/*
 * Cast a ball-beamed spell
 * Stop if we hit a monster, act as a "ball"
 * Allow "target" mode to pass over monsters
 * Affect grids, objects, and monsters
 */
#[no_mangle]
pub unsafe extern "C" fn fire_ball_beam(mut typ: libc::c_int,
                                        mut dir: libc::c_int,
                                        mut dam: libc::c_int,
                                        mut rad: libc::c_int) -> bool_ {
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut flg: libc::c_int =
        0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
            0x40 as libc::c_int | 0x2 as libc::c_int;
    /* Use the given direction */
    tx =
        (*p_ptr).px as libc::c_int +
            99 as libc::c_int * ddx[dir as usize] as libc::c_int;
    ty =
        (*p_ptr).py as libc::c_int +
            99 as libc::c_int * ddy[dir as usize] as libc::c_int;
    /* Hack -- Use an actual "target" */
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        flg &= !(0x8 as libc::c_int);
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int
    }
    /* Analyze the "dir" and the "target".  Hurt items on floor. */
    return project(0 as libc::c_int,
                   if rad > 16 as libc::c_int {
                       16 as libc::c_int
                   } else { rad }, ty, tx, dam, typ, flg);
}
#[no_mangle]
pub unsafe extern "C" fn teleport_swap(mut dir: libc::c_int) {
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    if (*p_ptr).resist_continuum != 0 {
        msg_print(b"The space-time continuum can\'t be disrupted.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int
    } else {
        tx = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
        ty = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int
    }
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(ty as isize)).offset(tx as isize) as
            *mut cave_type;
    if (*c_ptr).m_idx == 0 {
        msg_print(b"You can\'t trade places with that!\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        m_ptr =
            &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
        r_ptr =
            if !(*m_ptr).sr_ptr.is_null() {
                (*m_ptr).sr_ptr
            } else {
                race_info_idx((*m_ptr).r_idx as libc::c_int,
                              (*m_ptr).ego as libc::c_int)
            };
        if (*r_ptr).flags3 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            msg_print(b"Your teleportation is blocked!\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            sound(9 as libc::c_int);
            (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).m_idx =
                (*c_ptr).m_idx;
            /* Update the old location */
            (*c_ptr).m_idx = 0 as libc::c_int as s16b;
            /* Move the monster */
            (*m_ptr).fy = (*p_ptr).py as byte_hack;
            (*m_ptr).fx = (*p_ptr).px as byte_hack;
            /* Move the player */
            (*p_ptr).px = tx as s16b;
            (*p_ptr).py = ty as s16b;
            tx = (*m_ptr).fx as libc::c_int;
            ty = (*m_ptr).fy as libc::c_int;
            /* Update the monster (new location) */
            update_mon((*cave[ty as usize].offset(tx as isize)).m_idx as
                           libc::c_int, 1 as libc::c_int as bool_);
            /* Redraw the old grid */
            lite_spot(ty, tx);
            /* Redraw the new grid */
            lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
            /* Execute the inscription */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset((*m_ptr).fy as
                                                     isize)).offset((*m_ptr).fx
                                                                        as
                                                                        isize)
                    as *mut cave_type;
            if (*c_ptr).inscription != 0 {
                if inscription_info[(*c_ptr).inscription as usize].when as
                       libc::c_int & 0x4 as libc::c_int != 0 {
                    execute_inscription((*c_ptr).inscription as byte_hack,
                                        (*m_ptr).fy, (*m_ptr).fx);
                }
            }
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                                     isize)).offset((*p_ptr).px
                                                                        as
                                                                        isize)
                    as *mut cave_type;
            if (*c_ptr).inscription != 0 {
                msg_format(b"There is an inscription here: %s\x00" as
                               *const u8 as *const libc::c_char,
                           inscription_info[(*c_ptr).inscription as
                                                usize].text.as_mut_ptr());
                if inscription_info[(*c_ptr).inscription as usize].when as
                       libc::c_int & 0x2 as libc::c_int != 0 {
                    execute_inscription((*c_ptr).inscription as byte_hack,
                                        (*p_ptr).py as byte_hack,
                                        (*p_ptr).px as byte_hack);
                }
            }
            /* Check for new panel (redraw map) */
            verify_panel();
            /* Update stuff */
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long |
                     (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                          0x200000 as libc::c_long)) as u32b;
            /* Update the monsters */
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long)
                    as u32b;
            /* Redraw trap detection status */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x20000000 as libc::c_long)
                    as u32b;
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as
                    u32b;
            /* Handle stuff XXX XXX XXX */
            handle_stuff();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn swap_position(mut lty: libc::c_int,
                                       mut ltx: libc::c_int) {
    let mut tx: libc::c_int = ltx;
    let mut ty: libc::c_int = lty;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    if (*p_ptr).resist_continuum != 0 {
        msg_print(b"The space-time continuum can\'t be disrupted.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(ty as isize)).offset(tx as isize) as
            *mut cave_type;
    if (*c_ptr).m_idx == 0 {
        sound(9 as libc::c_int);
        /* Keep trace of the old location */
        tx = (*p_ptr).px as libc::c_int;
        ty = (*p_ptr).py as libc::c_int;
        /* Move the player */
        (*p_ptr).px = ltx as s16b;
        (*p_ptr).py = lty as s16b;
        /* Redraw the old grid */
        lite_spot(ty, tx);
        /* Redraw the new grid */
        lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
        /* Check for new panel (redraw map) */
        verify_panel();
        /* Update stuff */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                      0x200000 as libc::c_long)) as u32b;
        /* Update the monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long) as
                u32b;
        /* Redraw trap detection status */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x20000000 as libc::c_long) as
                u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Handle stuff XXX XXX XXX */
        handle_stuff();
    } else {
        m_ptr =
            &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
        sound(9 as libc::c_int);
        (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).m_idx =
            (*c_ptr).m_idx;
        /* Update the old location */
        (*c_ptr).m_idx = 0 as libc::c_int as s16b;
        /* Move the monster */
        (*m_ptr).fy = (*p_ptr).py as byte_hack;
        (*m_ptr).fx = (*p_ptr).px as byte_hack;
        /* Move the player */
        (*p_ptr).px = tx as s16b;
        (*p_ptr).py = ty as s16b;
        tx = (*m_ptr).fx as libc::c_int;
        ty = (*m_ptr).fy as libc::c_int;
        /* Update the monster (new location) */
        update_mon((*cave[ty as usize].offset(tx as isize)).m_idx as
                       libc::c_int, 1 as libc::c_int as bool_);
        /* Redraw the old grid */
        lite_spot(ty, tx);
        /* Redraw the new grid */
        lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
        /* Check for new panel (redraw map) */
        verify_panel();
        /* Update stuff */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                      0x200000 as libc::c_long)) as u32b;
        /* Update the monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long) as
                u32b;
        /* Redraw trap detection status */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x20000000 as libc::c_long) as
                u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Handle stuff XXX XXX XXX */
        handle_stuff();
    };
}
/*
 * Hack -- apply a "projection()" in a direction (or at the target)
 */
#[no_mangle]
pub unsafe extern "C" fn project_hook(mut typ: libc::c_int,
                                      mut dir: libc::c_int,
                                      mut dam: libc::c_int,
                                      mut flg: libc::c_int) -> bool_ {
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    /* Pass through the target if needed */
    flg |= 0x4 as libc::c_int;
    /* Use the given direction */
    tx = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
    ty = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
    /* Hack -- Use an actual "target" */
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int
    }
    /* Analyze the "dir" and the "target", do NOT explode */
    return project(0 as libc::c_int, 0 as libc::c_int, ty, tx, dam, typ, flg);
}
/*
 * Cast a bolt spell
 * Stop if we hit a monster, as a "bolt"
 * Affect monsters (not grids or objects)
 */
#[no_mangle]
pub unsafe extern "C" fn fire_bolt(mut typ: libc::c_int, mut dir: libc::c_int,
                                   mut dam: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(typ, dir, dam, flg);
}
/*
 * Cast a druidistic bolt spell
 * Stop if we hit a monster, as a "bolt"
 * Affect monsters (not grids or objects)
 */
#[no_mangle]
pub unsafe extern "C" fn fire_druid_bolt(mut typ: libc::c_int,
                                         mut dir: libc::c_int,
                                         mut dam: libc::c_int) -> bool_ {
    let mut flg: libc::c_int =
        0x8 as libc::c_int | 0x40 as libc::c_int | 0x4000 as libc::c_int;
    return project_hook(typ, dir, dam, flg);
}
/*
 * Cast a druidistic beam spell
 * Pass through monsters, as a "beam"
 * Affect monsters (not grids or objects)
 */
#[no_mangle]
pub unsafe extern "C" fn fire_druid_beam(mut typ: libc::c_int,
                                         mut dir: libc::c_int,
                                         mut dam: libc::c_int) -> bool_ {
    let mut flg: libc::c_int =
        0x2 as libc::c_int | 0x40 as libc::c_int | 0x4000 as libc::c_int;
    return project_hook(typ, dir, dam, flg);
}
/*
 * Cast a beam spell
 * Pass through monsters, as a "beam"
 * Affect monsters (not grids or objects)
 */
#[no_mangle]
pub unsafe extern "C" fn fire_beam(mut typ: libc::c_int, mut dir: libc::c_int,
                                   mut dam: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x2 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(typ, dir, dam, flg);
}
/*
 * Cast a bolt spell, or rarely, a beam spell
 */
#[no_mangle]
pub unsafe extern "C" fn fire_bolt_or_beam(mut prob: libc::c_int,
                                           mut typ: libc::c_int,
                                           mut dir: libc::c_int,
                                           mut dam: libc::c_int) -> bool_ {
    if Rand_div(100 as libc::c_int) < prob {
        return fire_beam(typ, dir, dam)
    } else { return fire_bolt(typ, dir, dam) };
}
#[no_mangle]
pub unsafe extern "C" fn fire_godly_wrath(mut y: libc::c_int,
                                          mut x: libc::c_int,
                                          mut typ: libc::c_int,
                                          mut rad: libc::c_int,
                                          mut dam: libc::c_int) -> bool_ {
    let mut flg: libc::c_int =
        0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
            0x40 as libc::c_int;
    return project(0 as libc::c_int, rad, y, x, dam, typ, flg);
}
#[no_mangle]
pub unsafe extern "C" fn fire_explosion(mut y: libc::c_int,
                                        mut x: libc::c_int,
                                        mut typ: libc::c_int,
                                        mut rad: libc::c_int,
                                        mut dam: libc::c_int) -> bool_ {
    let mut flg: libc::c_int =
        0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
            0x40 as libc::c_int;
    return project(0 as libc::c_int, rad, y, x, dam, typ, flg);
}
/*
 * Some of the old functions
 */
#[no_mangle]
pub unsafe extern "C" fn lite_line(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int =
        0x2 as libc::c_int | 0x10 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(17 as libc::c_int, dir,
                        damroll(6 as libc::c_int as s16b,
                                8 as libc::c_int as s16b), flg);
}
#[no_mangle]
pub unsafe extern "C" fn drain_life(mut dir: libc::c_int,
                                    mut dam: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(58 as libc::c_int, dir, dam, flg);
}
#[no_mangle]
pub unsafe extern "C" fn wall_to_mud(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int =
        0x2 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
            0x40 as libc::c_int;
    return project_hook(40 as libc::c_int, dir,
                        20 as libc::c_int +
                            (Rand_div(30 as libc::c_int) + 1 as libc::c_int),
                        flg);
}
#[no_mangle]
pub unsafe extern "C" fn wizard_lock(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int =
        0x2 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
            0x40 as libc::c_int;
    return project_hook(88 as libc::c_int, dir,
                        20 as libc::c_int +
                            (Rand_div(30 as libc::c_int) + 1 as libc::c_int),
                        flg);
}
#[no_mangle]
pub unsafe extern "C" fn destroy_door(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int =
        0x2 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int;
    return project_hook(41 as libc::c_int, dir, 0 as libc::c_int, flg);
}
#[no_mangle]
pub unsafe extern "C" fn disarm_trap(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int =
        0x2 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int;
    return project_hook(42 as libc::c_int, dir, 0 as libc::c_int, flg);
}
#[no_mangle]
pub unsafe extern "C" fn heal_monster(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(53 as libc::c_int, dir,
                        damroll(4 as libc::c_int as s16b,
                                6 as libc::c_int as s16b), flg);
}
#[no_mangle]
pub unsafe extern "C" fn speed_monster(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(54 as libc::c_int, dir, (*p_ptr).lev as libc::c_int,
                        flg);
}
#[no_mangle]
pub unsafe extern "C" fn slow_monster(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(55 as libc::c_int, dir, (*p_ptr).lev as libc::c_int,
                        flg);
}
#[no_mangle]
pub unsafe extern "C" fn sleep_monster(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(57 as libc::c_int, dir, (*p_ptr).lev as libc::c_int,
                        flg);
}
#[no_mangle]
pub unsafe extern "C" fn stasis_monster(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(75 as libc::c_int, dir, (*p_ptr).lev as libc::c_int,
                        flg);
}
#[no_mangle]
pub unsafe extern "C" fn confuse_monster(mut dir: libc::c_int,
                                         mut plev: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(56 as libc::c_int, dir, plev, flg);
}
#[no_mangle]
pub unsafe extern "C" fn stun_monster(mut dir: libc::c_int,
                                      mut plev: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(78 as libc::c_int, dir, plev, flg);
}
#[no_mangle]
pub unsafe extern "C" fn poly_monster(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(52 as libc::c_int, dir, (*p_ptr).lev as libc::c_int,
                        flg);
}
#[no_mangle]
pub unsafe extern "C" fn clone_monster(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(51 as libc::c_int, dir, 0 as libc::c_int, flg);
}
#[no_mangle]
pub unsafe extern "C" fn fear_monster(mut dir: libc::c_int,
                                      mut plev: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(66 as libc::c_int, dir, plev, flg);
}
#[no_mangle]
pub unsafe extern "C" fn death_ray(mut dir: libc::c_int,
                                   mut plev: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(77 as libc::c_int, dir, plev, flg);
}
#[no_mangle]
pub unsafe extern "C" fn teleport_monster(mut dir: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x2 as libc::c_int | 0x40 as libc::c_int;
    if (*p_ptr).resist_continuum != 0 {
        msg_print(b"The space-time continuum can\'t be disrupted.\x00" as
                      *const u8 as *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    return project_hook(63 as libc::c_int, dir,
                        20 as libc::c_int * 5 as libc::c_int, flg);
}
/*
 * Hooks -- affect adjacent grids (radius 1 ball attack)
 */
#[no_mangle]
pub unsafe extern "C" fn door_creation() -> bool_ {
    let mut flg: libc::c_int =
        0x10 as libc::c_int | 0x20 as libc::c_int | 0x80 as libc::c_int;
    return project(0 as libc::c_int, 1 as libc::c_int,
                   (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                   0 as libc::c_int, 46 as libc::c_int, flg);
}
#[no_mangle]
pub unsafe extern "C" fn trap_creation() -> bool_ {
    let mut flg: libc::c_int =
        0x10 as libc::c_int | 0x20 as libc::c_int | 0x80 as libc::c_int;
    return project(0 as libc::c_int, 1 as libc::c_int,
                   (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                   0 as libc::c_int, 47 as libc::c_int, flg);
}
#[no_mangle]
pub unsafe extern "C" fn glyph_creation() -> bool_ {
    let mut flg: libc::c_int = 0x10 as libc::c_int | 0x20 as libc::c_int;
    return project(0 as libc::c_int, 1 as libc::c_int,
                   (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                   0 as libc::c_int, 74 as libc::c_int, flg);
}
#[no_mangle]
pub unsafe extern "C" fn wall_stone(mut y: libc::c_int, mut x: libc::c_int)
 -> bool_ {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut flg: libc::c_int = 0x10 as libc::c_int | 0x20 as libc::c_int;
    let mut featflags: libc::c_int =
        (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_int;
    let mut dummy: bool_ =
        project(0 as libc::c_int, 1 as libc::c_int, y, x, 0 as libc::c_int,
                76 as libc::c_int, flg);
    if featflags as libc::c_long & 0x40 as libc::c_long == 0 &&
           featflags as libc::c_long & 0x20 as libc::c_long == 0 {
        cave_set_feat(y, x, 0x1 as libc::c_int);
    }
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                  0x200000 as libc::c_long)) as u32b;
    /* Update the monsters */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
    return dummy;
}
#[no_mangle]
pub unsafe extern "C" fn destroy_doors_touch() -> bool_ {
    let mut flg: libc::c_int =
        0x10 as libc::c_int | 0x20 as libc::c_int | 0x80 as libc::c_int;
    return project(0 as libc::c_int, 1 as libc::c_int,
                   (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                   0 as libc::c_int, 41 as libc::c_int, flg);
}
#[no_mangle]
pub unsafe extern "C" fn destroy_traps_touch() -> bool_ {
    let mut flg: libc::c_int =
        0x10 as libc::c_int | 0x20 as libc::c_int | 0x80 as libc::c_int;
    return project(0 as libc::c_int, 1 as libc::c_int,
                   (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                   0 as libc::c_int, 42 as libc::c_int, flg);
}
#[no_mangle]
pub unsafe extern "C" fn sleep_monsters_touch() -> bool_ {
    let mut flg: libc::c_int = 0x40 as libc::c_int | 0x80 as libc::c_int;
    return project(0 as libc::c_int, 1 as libc::c_int,
                   (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                   (*p_ptr).lev as libc::c_int, 57 as libc::c_int, flg);
}
#[no_mangle]
pub unsafe extern "C" fn call_chaos() {
    let mut Chaos_type: libc::c_int = 0;
    let mut dummy: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut plev: libc::c_int = (*p_ptr).lev as libc::c_int;
    let mut line_chaos: bool_ = 0 as libc::c_int as bool_;
    let mut hurt_types: [libc::c_int; 30] =
        [1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         4 as libc::c_int, 5 as libc::c_int, 10 as libc::c_int,
         11 as libc::c_int, 12 as libc::c_int, 79 as libc::c_int,
         14 as libc::c_int, 15 as libc::c_int, 16 as libc::c_int,
         23 as libc::c_int, 24 as libc::c_int, 26 as libc::c_int,
         27 as libc::c_int, 28 as libc::c_int, 30 as libc::c_int,
         31 as libc::c_int, 32 as libc::c_int, 20 as libc::c_int,
         21 as libc::c_int, 33 as libc::c_int, 22 as libc::c_int,
         34 as libc::c_int, 35 as libc::c_int, 72 as libc::c_int,
         73 as libc::c_int, 80 as libc::c_int, 81 as libc::c_int];
    Chaos_type =
        hurt_types[(Rand_div(30 as libc::c_int) + 1 as libc::c_int -
                        1 as libc::c_int) as usize];
    if Rand_div(4 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
        line_chaos = 1 as libc::c_int as bool_
    }
    if Rand_div(6 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
        dummy = 1 as libc::c_int;
        while dummy < 10 as libc::c_int {
            if dummy - 5 as libc::c_int != 0 {
                if line_chaos != 0 {
                    fire_beam(Chaos_type, dummy, 75 as libc::c_int);
                } else {
                    fire_ball(Chaos_type, dummy, 75 as libc::c_int,
                              2 as libc::c_int);
                }
            }
            dummy += 1
        }
    } else if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                  1 as libc::c_int {
        fire_ball(Chaos_type, 0 as libc::c_int, 300 as libc::c_int,
                  8 as libc::c_int);
    } else {
        if get_aim_dir(&mut dir) == 0 { return }
        if line_chaos != 0 {
            fire_beam(Chaos_type, dir, 150 as libc::c_int);
        } else {
            fire_ball(Chaos_type, dir, 150 as libc::c_int,
                      3 as libc::c_int + plev / 35 as libc::c_int);
        }
    };
}
/*
 * Activate the evil Topi Ylinen curse
 * rr9: Stop the nasty things when a Cyberdemon is summoned
 * or the player gets paralyzed.
 */
#[no_mangle]
pub unsafe extern "C" fn activate_ty_curse() {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut stop_ty: bool_ = 0 as libc::c_int as bool_;
    loop  {
        let mut current_block_24: u64;
        match Rand_div(27 as libc::c_int) + 1 as libc::c_int {
            1 | 2 | 3 | 16 | 17 => {
                aggravate_monsters(1 as libc::c_int);
                if Rand_div(6 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_24 = 14447253356787937536;
                } else { current_block_24 = 11209831452340133636; }
            }
            4 | 5 | 6 => { current_block_24 = 11209831452340133636; }
            7 | 8 | 9 | 18 => { current_block_24 = 6189327631830197737; }
            10 | 11 | 12 => { current_block_24 = 10246062425420730106; }
            13 | 14 | 15 | 19 | 20 => {
                current_block_24 = 12278530188328888592;
            }
            21 | 22 | 23 => { current_block_24 = 16020017309651753338; }
            24 => { current_block_24 = 1814183962057716864; }
            25 => {
                /*
			 * Only summon Cyberdemons deep in the dungeon.
			 */
                if dun_level as libc::c_int > 65 as libc::c_int &&
                       stop_ty == 0 {
                    summon_cyber();
                    stop_ty = 1 as libc::c_int as bool_;
                    current_block_24 = 14447253356787937536;
                } else { current_block_24 = 3993820815131920605; }
            }
            _ => { current_block_24 = 3993820815131920605; }
        }
        match current_block_24 {
            11209831452340133636 => {
                activate_hi_summon();
                if Rand_div(6 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_24 = 14447253356787937536;
                } else { current_block_24 = 6189327631830197737; }
            }
            3993820815131920605 => {
                while i < 6 as libc::c_int {
                    loop  {
                        do_dec_stat(i, 2 as libc::c_int);
                        if !(Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                                 1 as libc::c_int) {
                            break ;
                        }
                    }
                    i += 1
                }
                current_block_24 = 14447253356787937536;
            }
            _ => { }
        }
        match current_block_24 {
            6189327631830197737 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 0 as libc::c_int);
                if Rand_div(6 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_24 = 14447253356787937536;
                } else { current_block_24 = 10246062425420730106; }
            }
            _ => { }
        }
        match current_block_24 {
            10246062425420730106 => {
                msg_print(b"You feel your life draining away...\x00" as
                              *const u8 as *const libc::c_char);
                lose_exp((*p_ptr).exp / 16 as libc::c_int);
                if Rand_div(6 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_24 = 14447253356787937536;
                } else { current_block_24 = 12278530188328888592; }
            }
            _ => { }
        }
        match current_block_24 {
            12278530188328888592 => {
                if !((*p_ptr).free_act as libc::c_int != 0 &&
                         (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                             (*p_ptr).skill_sav as libc::c_int) {
                    msg_print(b"You feel like a statue!\x00" as *const u8 as
                                  *const libc::c_char);
                    if (*p_ptr).free_act != 0 {
                        set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                          (Rand_div(3 as libc::c_int) +
                                               1 as libc::c_int));
                    } else {
                        set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                          (Rand_div(13 as libc::c_int) +
                                               1 as libc::c_int));
                    }
                    stop_ty = 1 as libc::c_int as bool_
                }
                if Rand_div(6 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_24 = 14447253356787937536;
                } else { current_block_24 = 16020017309651753338; }
            }
            _ => { }
        }
        match current_block_24 {
            16020017309651753338 => {
                do_dec_stat(Rand_div(6 as libc::c_int) + 1 as libc::c_int -
                                1 as libc::c_int, 2 as libc::c_int);
                if Rand_div(6 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_24 = 14447253356787937536;
                } else { current_block_24 = 1814183962057716864; }
            }
            _ => { }
        }
        match current_block_24 {
            1814183962057716864 => {
                msg_print(b"Huh? Who am I? What am I doing here?\x00" as
                              *const u8 as *const libc::c_char);
                lose_all_info();
            }
            _ => { }
        }
        if !(Rand_div(3 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int
                 && stop_ty == 0) {
            break ;
        }
    };
}
/*
 * Activate the ultra evil Dark God curse
 */
#[no_mangle]
pub unsafe extern "C" fn activate_dg_curse() {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut stop_dg: bool_ = 0 as libc::c_int as bool_;
    loop  {
        let mut current_block_40: u64;
        match Rand_div(30 as libc::c_int) + 1 as libc::c_int {
            1 | 2 | 3 | 16 | 17 => {
                aggravate_monsters(1 as libc::c_int);
                if Rand_div(8 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_40 = 3392087639489470149;
                } else { current_block_40 = 11115405474911539371; }
            }
            4 | 5 | 6 => { current_block_40 = 11115405474911539371; }
            7 | 8 | 9 | 18 => { current_block_40 = 655865395110440796; }
            10 | 11 | 12 => { current_block_40 = 7114238226437799279; }
            13 | 14 | 15 => { current_block_40 = 10722803513630280402; }
            19 | 20 => { current_block_40 = 3198886612715173143; }
            21 | 22 | 23 => { current_block_40 = 270939517198655638; }
            24 => { current_block_40 = 18316076732687839917; }
            27 | 28 | 29 => {
                if (*p_ptr).inventory[24 as libc::c_int as usize].k_idx != 0 {
                    msg_print(b"Your weapon now seems useless...\x00" as
                                  *const u8 as *const libc::c_char);
                    (*p_ptr).inventory[24 as libc::c_int as usize].art_flags4
                        = 0x1 as libc::c_long as u32b
                }
                current_block_40 = 3392087639489470149;
            }
            25 => {
                /*
			 * Only summon Thunderlords not too shallow in the dungeon.
			 */
                if dun_level as libc::c_int > 25 as libc::c_int &&
                       stop_dg == 0 {
                    msg_print(b"Oh! You attracted some evil Thunderlords!\x00"
                                  as *const u8 as *const libc::c_char);
                    summon_dragon_riders();
                    /* This is evil -- DG */
                    if Rand_div(2 as libc::c_int) != 0 {
                        stop_dg = 1 as libc::c_int as bool_
                    }
                    current_block_40 = 3392087639489470149;
                } else { current_block_40 = 18010595784010950709; }
            }
            _ => { current_block_40 = 18010595784010950709; }
        }
        match current_block_40 {
            11115405474911539371 => {
                msg_print(b"Oh! You feel that the curse is replicating itself!\x00"
                              as *const u8 as *const libc::c_char);
                curse_equipment_dg(100 as libc::c_int,
                                   50 as libc::c_int *
                                       (Rand_div(2 as libc::c_int) +
                                            1 as libc::c_int));
                if Rand_div(8 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_40 = 3392087639489470149;
                } else { current_block_40 = 655865395110440796; }
            }
            18010595784010950709 => {
                while i < 6 as libc::c_int {
                    loop  {
                        do_dec_stat(i, 2 as libc::c_int);
                        if !(Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                                 1 as libc::c_int) {
                            break ;
                        }
                    }
                    i += 1
                }
                current_block_40 = 3392087639489470149;
            }
            _ => { }
        }
        match current_block_40 {
            655865395110440796 => {
                curse_equipment(100 as libc::c_int,
                                50 as libc::c_int *
                                    (Rand_div(2 as libc::c_int) +
                                         1 as libc::c_int));
                if Rand_div(8 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_40 = 3392087639489470149;
                } else { current_block_40 = 7114238226437799279; }
            }
            _ => { }
        }
        match current_block_40 {
            7114238226437799279 => {
                msg_print(b"You feel your life draining away...\x00" as
                              *const u8 as *const libc::c_char);
                lose_exp((*p_ptr).exp / 12 as libc::c_int);
                if Rand_div(2 as libc::c_int) != 0 {
                    msg_print(b"You feel the coldness of the Black Breath attacking you!\x00"
                                  as *const u8 as *const libc::c_char);
                    (*p_ptr).black_breath = 1 as libc::c_int as bool_
                }
                if Rand_div(8 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_40 = 3392087639489470149;
                } else { current_block_40 = 10722803513630280402; }
            }
            _ => { }
        }
        match current_block_40 {
            10722803513630280402 => {
                if !((*p_ptr).free_act as libc::c_int != 0 &&
                         (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                             (*p_ptr).skill_sav as libc::c_int) {
                    msg_print(b"You feel like a statue!\x00" as *const u8 as
                                  *const libc::c_char);
                    if (*p_ptr).free_act != 0 {
                        set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                          (Rand_div(3 as libc::c_int) +
                                               1 as libc::c_int));
                    } else {
                        set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                          (Rand_div(13 as libc::c_int) +
                                               1 as libc::c_int));
                    }
                    stop_dg = 1 as libc::c_int as bool_
                }
                if Rand_div(7 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_40 = 3392087639489470149;
                } else { current_block_40 = 3198886612715173143; }
            }
            _ => { }
        }
        match current_block_40 {
            3198886612715173143 => {
                msg_print(b"Woah! You see 10 little Morgoths dancing before you!\x00"
                              as *const u8 as *const libc::c_char);
                set_confused((*p_ptr).confused as libc::c_int +
                                 (Rand_div(13 as libc::c_int *
                                               2 as libc::c_int) +
                                      1 as libc::c_int));
                if Rand_div(2 as libc::c_int) != 0 {
                    stop_dg = 1 as libc::c_int as bool_
                }
                if Rand_div(7 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_40 = 3392087639489470149;
                } else { current_block_40 = 270939517198655638; }
            }
            _ => { }
        }
        match current_block_40 {
            270939517198655638 => {
                do_dec_stat(Rand_div(6 as libc::c_int) + 1 as libc::c_int -
                                1 as libc::c_int, 3 as libc::c_int);
                if Rand_div(7 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                    current_block_40 = 3392087639489470149;
                } else { current_block_40 = 18316076732687839917; }
            }
            _ => { }
        }
        match current_block_40 {
            18316076732687839917 => {
                msg_print(b"Huh? Who am I? What am I doing here?\x00" as
                              *const u8 as *const libc::c_char);
                lose_all_info();
            }
            _ => { }
        }
        if !(Rand_div(4 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int
                 && stop_dg == 0) {
            break ;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn activate_hi_summon() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              Rand_div(9 as libc::c_int) + 1 as libc::c_int +
                  dun_level as libc::c_int / 40 as libc::c_int {
        match Rand_div(26 as libc::c_int) + 1 as libc::c_int +
                  dun_level as libc::c_int / 20 as libc::c_int {
            1 | 2 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 11 as libc::c_int);
            }
            3 | 4 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 12 as libc::c_int);
            }
            5 | 6 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 13 as libc::c_int);
            }
            7 | 8 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 14 as libc::c_int);
            }
            9 | 10 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 15 as libc::c_int);
            }
            11 | 12 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 17 as libc::c_int);
            }
            13 | 14 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 18 as libc::c_int);
            }
            15 | 16 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 16 as libc::c_int);
            }
            17 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 31 as libc::c_int);
            }
            18 | 19 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 32 as libc::c_int);
            }
            20 | 21 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 21 as libc::c_int);
            }
            22 | 23 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 22 as libc::c_int);
            }
            24 | 25 => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                100 as libc::c_int, 39 as libc::c_int);
            }
            _ => {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int * 3 as libc::c_int /
                                    2 as libc::c_int + 5 as libc::c_int,
                                0 as libc::c_int);
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn summon_cyber() {
    let mut i: libc::c_int = 0;
    let mut max_cyber: libc::c_int =
        dun_level as libc::c_int / 50 as libc::c_int +
            (Rand_div(6 as libc::c_int) + 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < max_cyber {
        summon_specific((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, 100 as libc::c_int,
                        39 as libc::c_int);
        i += 1
    };
}
/* File: spells2.c */
/* Purpose: Spell code (part 2) */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Bias luck needs to be higher than weird luck,
 * since it is usually tested several times...
 */
#[no_mangle]
pub unsafe extern "C" fn summon_dragon_riders() {
    let mut i: libc::c_int = 0;
    let mut max_dr: libc::c_int =
        dun_level as libc::c_int / 50 as libc::c_int +
            (Rand_div(6 as libc::c_int) + 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < max_dr {
        summon_specific((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, 100 as libc::c_int,
                        49 as libc::c_int);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn wall_breaker() {
    let mut dummy: libc::c_int = 5 as libc::c_int;
    if (Rand_div(80 as libc::c_int + (*p_ptr).lev as libc::c_int) +
            1 as libc::c_int) < 70 as libc::c_int {
        loop  {
            dummy = Rand_div(9 as libc::c_int) + 1 as libc::c_int;
            if !(dummy == 5 as libc::c_int || dummy == 0 as libc::c_int) {
                break ;
            }
        }
        wall_to_mud(dummy);
    } else if Rand_div(100 as libc::c_int) + 1 as libc::c_int >
                  30 as libc::c_int {
        /* Prevent destruction of quest levels and town */
        if is_quest(dun_level as libc::c_int) == 0 &&
               dun_level as libc::c_int != 0 {
            earthquake((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                       1 as libc::c_int);
        }
    } else {
        dummy = 1 as libc::c_int;
        while dummy < 10 as libc::c_int {
            if dummy - 5 as libc::c_int != 0 { wall_to_mud(dummy); }
            dummy += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn bless_weapon() {
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Assume enchant weapon */
    item_tester_hook =
        Some(item_tester_hook_weapon as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q = b"Bless which weapon? \x00" as *const u8 as *const libc::c_char;
    s = b"You have weapon to bless.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s,
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Description */
    object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                0 as libc::c_int);
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        if f3 as libc::c_long & 0x40000000 as libc::c_long != 0 &&
               (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                   33 as libc::c_int ||
               f3 as libc::c_long & 0x80000000 as libc::c_long != 0 {
            msg_format(b"The black aura on %s %s disrupts the blessing!\x00"
                           as *const u8 as *const libc::c_char,
                       if item >= 0 as libc::c_int {
                           b"your\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"the\x00" as *const u8 as *const libc::c_char
                       }, o_name.as_mut_ptr());
            return
        }
        msg_format(b"A malignant aura leaves %s %s.\x00" as *const u8 as
                       *const libc::c_char,
                   if item >= 0 as libc::c_int {
                       b"your\x00" as *const u8 as *const libc::c_char
                   } else { b"the\x00" as *const u8 as *const libc::c_char },
                   o_name.as_mut_ptr());
        /* Uncurse it */
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int & !(0x40 as libc::c_int)) as
                byte_hack;
        /* Hack -- Assume felt */
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x1 as libc::c_int) as byte_hack;
        /* Take note */
        (*o_ptr).sense = 10 as libc::c_int as byte_hack;
        /* Recalculate the bonuses */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x2 as libc::c_long) as u32b
    }
    /*
	 * Next, we try to bless it. Artifacts have a 1/3 chance of
	 * being blessed, otherwise, the operation simply disenchants
	 * them, godly power negating the magic. Ok, the explanation
	 * is silly, but otherwise priests would always bless every
	 * artifact weapon they find. Ego weapons and normal weapons
	 * can be blessed automatically.
	 */
    if f3 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        msg_format(b"%s %s %s blessed already.\x00" as *const u8 as
                       *const libc::c_char,
                   if item >= 0 as libc::c_int {
                       b"Your\x00" as *const u8 as *const libc::c_char
                   } else { b"The\x00" as *const u8 as *const libc::c_char },
                   o_name.as_mut_ptr(),
                   if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                       b"were\x00" as *const u8 as *const libc::c_char
                   } else { b"was\x00" as *const u8 as *const libc::c_char });
        return
    }
    if !((*o_ptr).art_name as libc::c_int != 0 ||
             (*o_ptr).name1 as libc::c_int != 0) ||
           Rand_div(3 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
        /* Describe */
        msg_format(b"%s %s shine%s!\x00" as *const u8 as *const libc::c_char,
                   if item >= 0 as libc::c_int {
                       b"Your\x00" as *const u8 as *const libc::c_char
                   } else { b"The\x00" as *const u8 as *const libc::c_char },
                   o_name.as_mut_ptr(),
                   if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                       b"\x00" as *const u8 as *const libc::c_char
                   } else { b"s\x00" as *const u8 as *const libc::c_char });
        (*o_ptr).art_flags3 =
            ((*o_ptr).art_flags3 as libc::c_long | 0x10000000 as libc::c_long)
                as u32b
    } else {
        let mut dis_happened: bool_ = 0 as libc::c_int as bool_;
        msg_print(b"The artifact resists your blessing!\x00" as *const u8 as
                      *const libc::c_char);
        /* Disenchant tohit */
        if (*o_ptr).to_h as libc::c_int > 0 as libc::c_int {
            (*o_ptr).to_h -= 1;
            dis_happened = 1 as libc::c_int as bool_
        }
        if (*o_ptr).to_h as libc::c_int > 5 as libc::c_int &&
               Rand_div(100 as libc::c_int) < 33 as libc::c_int {
            (*o_ptr).to_h -= 1
        }
        /* Disenchant todam */
        if (*o_ptr).to_d as libc::c_int > 0 as libc::c_int {
            (*o_ptr).to_d -= 1;
            dis_happened = 1 as libc::c_int as bool_
        }
        if (*o_ptr).to_d as libc::c_int > 5 as libc::c_int &&
               Rand_div(100 as libc::c_int) < 33 as libc::c_int {
            (*o_ptr).to_d -= 1
        }
        /* Disenchant toac */
        if (*o_ptr).to_a as libc::c_int > 0 as libc::c_int {
            (*o_ptr).to_a -= 1;
            dis_happened = 1 as libc::c_int as bool_
        }
        if (*o_ptr).to_a as libc::c_int > 5 as libc::c_int &&
               Rand_div(100 as libc::c_int) < 33 as libc::c_int {
            (*o_ptr).to_a -= 1
        }
        if dis_happened != 0 {
            msg_print(b"There is a static feeling in the air...\x00" as
                          *const u8 as *const libc::c_char);
            msg_format(b"%s %s %s disenchanted!\x00" as *const u8 as
                           *const libc::c_char,
                       if item >= 0 as libc::c_int {
                           b"Your\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"The\x00" as *const u8 as *const libc::c_char
                       }, o_name.as_mut_ptr(),
                       if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                           b"were\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"was\x00" as *const u8 as *const libc::c_char
                       });
        }
    }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x2 as libc::c_long | 0x8 as libc::c_long)) as u32b;
}
/*
 * Detect all "nonliving", "undead" or "demonic" monsters on current panel
 */
#[no_mangle]
pub unsafe extern "C" fn detect_monsters_nonliving(mut rad: libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    /* Scan monsters */
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
        /* Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Location */
            y = (*m_ptr).fy as libc::c_int;
            x = (*m_ptr).fx as libc::c_int;
            /* Only detect nearby monsters */
            if !((*m_ptr).cdis as libc::c_int > rad) {
                /* Detect evil monsters */
                if (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0
                       ||
                       (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint
                           != 0 ||
                       (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint
                           != 0 {
                    /* Update monster recall window */
                    if monster_race_idx as libc::c_int ==
                           (*m_ptr).r_idx as libc::c_int {
                        /* Window stuff */
                        (*p_ptr).window =
                            ((*p_ptr).window as libc::c_long |
                                 0x100 as libc::c_long) as u32b
                    }
                    /* Repair visibility later */
                    repair_monsters = 1 as libc::c_int as bool_;
                    /* Hack -- Detect monster */
                    (*m_ptr).mflag |=
                        0x80 as libc::c_int | 0x40 as libc::c_int;
                    /* Hack -- See monster */
                    (*m_ptr).ml = 1 as libc::c_int as bool_;
                    /* Redraw */
                    if y >= panel_row_min as libc::c_int &&
                           y <= panel_row_max as libc::c_int &&
                           x >= panel_col_min as libc::c_int &&
                           x <= panel_col_max as libc::c_int {
                        lite_spot(y, x);
                    }
                    /* Detect */
                    flag = 1 as libc::c_int as bool_
                }
            }
        }
        i += 1
    }
    /* Describe */
    if flag != 0 {
        /* Describe result */
        msg_print(b"You sense the presence of unnatural beings!\x00" as
                      *const u8 as *const libc::c_char);
    }
    /* Result */
    return flag;
}
/*
 * Confuse monsters
 */
#[no_mangle]
pub unsafe extern "C" fn confuse_monsters(mut dam: libc::c_int) -> bool_ {
    return project_hack(56 as libc::c_int, dam);
}
/*
 * Charm monsters
 */
#[no_mangle]
pub unsafe extern "C" fn charm_monsters(mut dam: libc::c_int) -> bool_ {
    return project_hack(82 as libc::c_int, dam);
}
/*
 * Charm animals
 */
#[no_mangle]
pub unsafe extern "C" fn charm_animals(mut dam: libc::c_int) -> bool_ {
    return project_hack(84 as libc::c_int, dam);
}
/*
 * Charm demons
 */
#[no_mangle]
pub unsafe extern "C" fn charm_demons(mut dam: libc::c_int) -> bool_ {
    return project_hack(106 as libc::c_int, dam);
}
/*
 * Stun monsters
 */
#[no_mangle]
pub unsafe extern "C" fn stun_monsters(mut dam: libc::c_int) -> bool_ {
    return project_hack(78 as libc::c_int, dam);
}
/*
 * Stasis monsters
 */
#[no_mangle]
pub unsafe extern "C" fn stasis_monsters(mut dam: libc::c_int) -> bool_ {
    return project_hack(75 as libc::c_int, dam);
}
/*
 * Mindblast monsters
 */
#[no_mangle]
pub unsafe extern "C" fn mindblast_monsters(mut dam: libc::c_int) -> bool_ {
    return project_hack(85 as libc::c_int, dam);
}
/*
 * Banish all monsters
 */
#[no_mangle]
pub unsafe extern "C" fn banish_monsters(mut dist: libc::c_int) -> bool_ {
    return project_hack(63 as libc::c_int, dist);
}
/*
 * Turn evil
 */
#[no_mangle]
pub unsafe extern "C" fn turn_evil(mut dam: libc::c_int) -> bool_ {
    return project_hack(65 as libc::c_int, dam);
}
/*
 * Turn everyone
 */
#[no_mangle]
pub unsafe extern "C" fn turn_monsters(mut dam: libc::c_int) -> bool_ {
    return project_hack(66 as libc::c_int, dam);
}
/*
 * Death-ray all monsters (note: OBSCENELY powerful)
 */
#[no_mangle]
pub unsafe extern "C" fn deathray_monsters() -> bool_ {
    return project_hack(77 as libc::c_int, (*p_ptr).lev as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn charm_monster(mut dir: libc::c_int,
                                       mut plev: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(82 as libc::c_int, dir, plev, flg);
}
#[no_mangle]
pub unsafe extern "C" fn star_charm_monster(mut dir: libc::c_int,
                                            mut plev: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(99 as libc::c_int, dir, plev, flg);
}
#[no_mangle]
pub unsafe extern "C" fn control_one_undead(mut dir: libc::c_int,
                                            mut plev: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(83 as libc::c_int, dir, plev, flg);
}
#[no_mangle]
pub unsafe extern "C" fn charm_animal(mut dir: libc::c_int,
                                      mut plev: libc::c_int) -> bool_ {
    let mut flg: libc::c_int = 0x8 as libc::c_int | 0x40 as libc::c_int;
    return project_hook(84 as libc::c_int, dir, plev, flg);
}
#[no_mangle]
pub unsafe extern "C" fn change_wild_mode() {
    if (*p_ptr).immovable as libc::c_int != 0 && (*p_ptr).wild_mode == 0 {
        msg_print(b"Hmm, blinking there will take time.\x00" as *const u8 as
                      *const libc::c_char);
    }
    if (*p_ptr).word_recall as libc::c_int != 0 && (*p_ptr).wild_mode == 0 {
        msg_print(b"You will soon be recalled.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    (*p_ptr).wild_mode = ((*p_ptr).wild_mode == 0) as libc::c_int as bool_;
    autosave_checkpoint();
    /* Leaving */
    (*p_ptr).leaving = 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn alter_reality() {
    msg_print(b"The world changes!\x00" as *const u8 as *const libc::c_char);
    autosave_checkpoint();
    /* Leaving */
    (*p_ptr).leaving = 1 as libc::c_int as bool_;
}
/* Heal insanity. */
#[no_mangle]
pub unsafe extern "C" fn heal_insanity(mut val: libc::c_int) -> bool_ {
    if ((*p_ptr).csane as libc::c_int) < (*p_ptr).msane as libc::c_int {
        (*p_ptr).csane = ((*p_ptr).csane as libc::c_int + val) as s16b;
        if (*p_ptr).csane as libc::c_int >= (*p_ptr).msane as libc::c_int {
            (*p_ptr).csane = (*p_ptr).msane;
            (*p_ptr).csane_frac = 0 as libc::c_int as u16b
        }
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x800000 as libc::c_long) as
                u32b;
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
        if val < 5 as libc::c_int {
            msg_print(b"You feel a little better.\x00" as *const u8 as
                          *const libc::c_char);
        } else if val < 15 as libc::c_int {
            msg_print(b"You feel better.\x00" as *const u8 as
                          *const libc::c_char);
        } else if val < 35 as libc::c_int {
            msg_print(b"You feel much better.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            msg_print(b"You feel very good.\x00" as *const u8 as
                          *const libc::c_char);
        }
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Send the player shooting through walls in the given direction until
 * they reach a non-wall space, or a monster, or a permanent wall.
 */
#[no_mangle]
pub unsafe extern "C" fn passwall(mut dir: libc::c_int, mut safe: bool_)
 -> bool_ {
    let mut x: libc::c_int = (*p_ptr).px as libc::c_int;
    let mut y: libc::c_int = (*p_ptr).py as libc::c_int;
    let mut ox: libc::c_int = (*p_ptr).px as libc::c_int;
    let mut oy: libc::c_int = (*p_ptr).py as libc::c_int;
    let mut lx: libc::c_int = (*p_ptr).px as libc::c_int;
    let mut ly: libc::c_int = (*p_ptr).py as libc::c_int;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut ok: bool_ = 0 as libc::c_int as bool_;
    if (*p_ptr).wild_mode != 0 { return 0 as libc::c_int as bool_ }
    if (*p_ptr).inside_quest != 0 { return 0 as libc::c_int as bool_ }
    if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Must go somewhere */
    if dir == 5 as libc::c_int { return 0 as libc::c_int as bool_ }
    loop  {
        x += ddx[dir as usize] as libc::c_int;
        y += ddy[dir as usize] as libc::c_int;
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Perm walls stops the transfer */
        if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
                 y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                 x < cur_wid as libc::c_int - 1 as libc::c_int) &&
               (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long
                   & 0x40 as libc::c_long != 0 {
            /* get the last working position */
            x -= ddx[dir as usize] as libc::c_int;
            y -= ddy[dir as usize] as libc::c_int;
            ok = 0 as libc::c_int as bool_;
            break ;
        } else {
            /* Never on a monster */
            if (*c_ptr).m_idx != 0 { continue ; }
            /* Never stop in vaults */
            if (*c_ptr).info as libc::c_int & 0x4 as libc::c_int != 0 {
                continue ;
            }
            /* From now on, the location COULD be used in special case */
            lx = x;
            ly = y;
            /* Pass over walls */
            if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long
                   & 0x20 as libc::c_long != 0 {
                continue ;
            }
            /* So it must be ok */
            ok = 1 as libc::c_int as bool_;
            break ;
        }
    }
    if ok == 0 {
        x = lx;
        y = ly;
        if safe == 0 {
            msg_print(b"You emerge in the wall!\x00" as *const u8 as
                          *const libc::c_char);
            take_hit(damroll(10 as libc::c_int as s16b,
                             8 as libc::c_int as s16b),
                     b"becoming one with a wall\x00" as *const u8 as
                         *const libc::c_char);
        }
        place_floor_convert_glass(y, x);
    }
    /* Move */
    (*p_ptr).px = x as s16b;
    (*p_ptr).py = y as s16b;
    /* Redraw the old spot */
    lite_spot(oy, ox);
    /* Redraw the new spot */
    lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    /* Check for new panel (redraw map) */
    verify_panel();
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                  0x200000 as libc::c_long)) as u32b;
    /* Update the monsters */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long) as u32b;
    /* Redraw trap detection status */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x20000000 as libc::c_long) as
            u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Handle stuff XXX XXX XXX */
    handle_stuff();
    return 1 as libc::c_int as bool_;
}
/*
 * Print a batch of dungeons.
 */
unsafe extern "C" fn print_dungeon_batch(mut p: *mut libc::c_int,
                                         mut start: libc::c_int,
                                         mut max: libc::c_int,
                                         mut mode: bool_) {
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut attr: byte_hack = 0;
    if mode != 0 {
        prt(format(b"     %-31s\x00" as *const u8 as *const libc::c_char,
                   b"Name\x00" as *const u8 as *const libc::c_char) as cptr,
            1 as libc::c_int, 20 as libc::c_int);
    }
    i = 0 as libc::c_int;
    j = start;
    while i < 20 as libc::c_int && j < max {
        let mut d_ptr: *mut dungeon_info_type =
            &mut *d_info.offset(*p.offset(j as isize) as isize) as
                *mut dungeon_info_type;
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"  %c) %-30s\x00" as *const u8 as *const libc::c_char,
                i + 'a' as i32, d_name.offset((*d_ptr).name as isize));
        if mode != 0 {
            if (*d_ptr).min_plev as libc::c_int > (*p_ptr).lev as libc::c_int
               {
                attr = 8 as libc::c_int as byte_hack
            } else { attr = 1 as libc::c_int as byte_hack }
            c_prt(attr, buf.as_mut_ptr() as cptr, 2 as libc::c_int + i,
                  20 as libc::c_int);
        }
        i += 1;
        j += 1
    }
    if mode != 0 {
        prt(b"\x00" as *const u8 as *const libc::c_char, 2 as libc::c_int + i,
            20 as libc::c_int);
    }
    prt(format(b"Select a dungeon (a-%c), * to list, @ to select by name, +/- to scroll:\x00"
                   as *const u8 as *const libc::c_char,
               i - 1 as libc::c_int + 'a' as i32) as cptr, 0 as libc::c_int,
        0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reset_recall_aux() -> libc::c_int {
    let mut which: libc::c_char = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut mode: bool_ = 0 as libc::c_int as bool_;
    p =
        memset(ralloc((max_d_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_d_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    /* Count the max */
    i = 1 as libc::c_int;
    while i < max_d_idx as libc::c_int {
        /* skip "blocked" dungeons */
        if !((*d_info.offset(i as isize)).flags1 as libc::c_long &
                 0x40000000 as libc::c_long != 0) {
            if *max_dlv.offset(i as isize) != 0 {
                let fresh317 = max;
                max = max + 1;
                *p.offset(fresh317 as isize) = i
            }
        }
        i += 1
    }
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    loop  {
        print_dungeon_batch(p, start, max, mode);
        which = inkey();
        if which as libc::c_int == '\u{1b}' as i32 {
            ret = -(1 as libc::c_int);
            break ;
        } else if which as libc::c_int == '*' as i32 ||
                      which as libc::c_int == '?' as i32 ||
                      which as libc::c_int == ' ' as i32 {
            mode =
                if mode as libc::c_int != 0 {
                    0 as libc::c_int
                } else { 1 as libc::c_int } as bool_;
            Term_load();
            character_icky = 0 as libc::c_int as bool_
        } else if which as libc::c_int == '+' as i32 {
            start += 20 as libc::c_int;
            if start >= max { start -= 20 as libc::c_int }
            Term_load();
            character_icky = 0 as libc::c_int as bool_
        } else if which as libc::c_int == '-' as i32 {
            start -= 20 as libc::c_int;
            if start < 0 as libc::c_int { start += 20 as libc::c_int }
            Term_load();
            character_icky = 0 as libc::c_int as bool_
        } else if which as libc::c_int == '@' as i32 {
            let mut buf: [libc::c_char; 80] = [0; 80];
            let mut buf2: [libc::c_char; 80] = [0; 80];
            strcpy(buf.as_mut_ptr(),
                   d_name.offset((*d_info.offset((*p_ptr).recall_dungeon as
                                                     isize)).name as isize));
            if get_string(b"Which dungeon? \x00" as *const u8 as
                              *const libc::c_char, buf.as_mut_ptr(),
                          79 as libc::c_int) == 0 {
                continue ;
            }
            /* Find the index corresponding to the name */
            i = 1 as libc::c_int;
            while i < max_d_idx as libc::c_int {
                sprintf(buf2.as_mut_ptr(),
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        d_name.offset((*d_info.offset(i as isize)).name as
                                          isize));
                /* Lowercase the name */
                strlower(buf.as_mut_ptr());
                strlower(buf2.as_mut_ptr());
                if !strstr(buf2.as_mut_ptr(), buf.as_mut_ptr()).is_null() {
                    break ;
                }
                i += 1
            }
            if i >= max_d_idx as libc::c_int {
                msg_print(b"Never heard of that place!\x00" as *const u8 as
                              *const libc::c_char);
                msg_print(0 as cptr);
            } else if (*d_info.offset(i as isize)).flags1 as libc::c_long &
                          0x40000000 as libc::c_long != 0 {
                msg_print(b"This place blocks my magic!\x00" as *const u8 as
                              *const libc::c_char);
                msg_print(0 as cptr);
            } else if (*d_info.offset(i as isize)).min_plev as libc::c_int >
                          (*p_ptr).lev as libc::c_int {
                msg_print(b"You cannot go there yet!\x00" as *const u8 as
                              *const libc::c_char);
                msg_print(0 as cptr);
            } else { ret = i; break ; }
        } else {
            which = tolower(which as libc::c_int) as libc::c_char;
            if start + (which as libc::c_int - 'a' as i32) >= max {
                bell();
            } else if (start + (which as libc::c_int - 'a' as i32)) <
                          0 as libc::c_int {
                bell();
            } else {
                ret =
                    *p.offset((start + (which as libc::c_int - 'a' as i32)) as
                                  isize);
                break ;
            }
        }
    }
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    rnfree(p as vptr,
           (max_d_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn reset_recall(mut no_trepas_max_depth: bool_)
 -> bool_ {
    let mut dun: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    /* Choose dungeon */
    dun = reset_recall_aux();
    if dun < 1 as libc::c_int { return 0 as libc::c_int as bool_ }
    /* Choose depth */
    if no_trepas_max_depth == 0 {
        max = (*d_info.offset(dun as isize)).maxdepth as libc::c_int
    } else { max = *max_dlv.offset(dun as isize) as libc::c_int }
    depth =
        get_quantity(format(b"Which level in %s(%d-%d)? \x00" as *const u8 as
                                *const libc::c_char,
                            d_name.offset((*d_info.offset(dun as isize)).name
                                              as isize),
                            (*d_info.offset(dun as isize)).mindepth as
                                libc::c_int, max) as cptr, max);
    if depth < 1 as libc::c_int { return 0 as libc::c_int as bool_ }
    /* Enforce minimum level */
    if depth < (*d_info.offset(dun as isize)).mindepth as libc::c_int {
        depth = (*d_info.offset(dun as isize)).mindepth as libc::c_int
    }
    /* Mega hack -- Forbid levels 99 and 100 */
    if depth == 99 as libc::c_int || depth == 100 as libc::c_int {
        depth = 98 as libc::c_int
    }
    (*p_ptr).recall_dungeon = dun as s16b;
    *max_dlv.offset((*p_ptr).recall_dungeon as isize) = depth as s16b;
    return 1 as libc::c_int as bool_;
}
/* The only way to get rid of the dreaded DG_CURSE*/
#[no_mangle]
pub unsafe extern "C" fn remove_dg_curse() {
    let mut k: libc::c_int = 0;
    /* Parse all the items */
    k = 24 as libc::c_int;
    while k < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(k as isize) as
                *mut object_type;
        if (*o_ptr).k_idx as libc::c_int != 0 &&
               (*o_ptr).art_flags4 as libc::c_long & 0x20 as libc::c_long != 0
           {
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long &
                     !(0x40000000 as libc::c_long)) as u32b;
            (*o_ptr).art_flags3 =
                ((*o_ptr).art_flags3 as libc::c_long &
                     !(0x20000000 as libc::c_long)) as u32b;
            (*o_ptr).art_flags4 =
                ((*o_ptr).art_flags4 as libc::c_long &
                     !(0x20 as libc::c_long)) as u32b;
            msg_print(b"The Morgothian Curse withers away.\x00" as *const u8
                          as *const libc::c_char);
        }
        k += 1
    };
}
/*
 * Creates a between gate
 */
#[no_mangle]
pub unsafe extern "C" fn create_between_gate(mut dist: libc::c_int,
                                             mut y: libc::c_int,
                                             mut x: libc::c_int) {
    let mut ii: libc::c_int = 0;
    let mut ij: libc::c_int = 0;
    let mut plev: libc::c_int = get_skill(1 as libc::c_int) as libc::c_int;
    if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 {
        msg_print(b"Not on special levels!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if x == 0 || y == 0 {
        msg_print(b"You open a Void Jumpgate. Choose a destination.\x00" as
                      *const u8 as *const libc::c_char);
        if tgt_pt(&mut ii, &mut ij) == 0 { return }
        (*p_ptr).energy -= 60 as libc::c_int - plev;
        if !((*f_info.offset((*cave[ij as usize].offset(ii as isize)).feat as
                                 isize)).flags1 as libc::c_long &
                 0x10 as libc::c_long != 0 &&
                 (*cave[ij as usize].offset(ii as isize)).feat as libc::c_int
                     != 0xaf as libc::c_int &&
                 (*cave[ij as usize].offset(ii as isize)).m_idx == 0 &&
                 !(ij == (*p_ptr).py as libc::c_int &&
                       ii == (*p_ptr).px as libc::c_int)) ||
               (*cave[ij as usize].offset(ii as isize)).info as libc::c_int &
                   0x4 as libc::c_int != 0 ||
               distance(ij, ii, (*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int) > dist ||
               Rand_div(plev * plev / 2 as libc::c_int) == 0 as libc::c_int {
            msg_print(b"You fail to exit the void correctly!\x00" as *const u8
                          as *const libc::c_char);
            (*p_ptr).energy -= 100 as libc::c_int;
            get_pos_player(10 as libc::c_int, &mut ij, &mut ii);
        }
    } else { ij = y; ii = x }
    if (*f_info.offset((*cave[(*p_ptr).py as
                                  usize].offset((*p_ptr).px as isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x40 as libc::c_long == 0 {
        cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                      0xa0 as libc::c_int);
        (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).special =
            (ii + (ij << 8 as libc::c_int)) as s16b
    }
    if (*f_info.offset((*cave[ij as usize].offset(ii as isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x40 as libc::c_long == 0 {
        cave_set_feat(ij, ii, 0xa0 as libc::c_int);
        (*cave[ij as usize].offset(ii as isize)).special =
            ((*p_ptr).px as libc::c_int +
                 (((*p_ptr).py as libc::c_int) << 8 as libc::c_int)) as s16b
    };
}
