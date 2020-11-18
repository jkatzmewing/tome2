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
    static mut player_exp: [s32b; 50];
    #[no_mangle]
    static mut chaos_patrons: [cptr; 16];
    #[no_mangle]
    static mut chaos_stats: [libc::c_int; 16];
    #[no_mangle]
    static mut chaos_rewards: [[libc::c_int; 20]; 16];
    #[no_mangle]
    static mut character_dungeon: bool_;
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_dir: s16b;
    #[no_mangle]
    static mut energy_use: s32b;
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
    static mut use_bigtile: bool_;
    #[no_mangle]
    static mut use_old_target: bool_;
    #[no_mangle]
    static mut disturb_panel: bool_;
    #[no_mangle]
    static mut disturb_state: bool_;
    #[no_mangle]
    static mut expand_look: bool_;
    #[no_mangle]
    static mut expand_list: bool_;
    #[no_mangle]
    static mut speak_unique: bool_;
    #[no_mangle]
    static mut panel_row_min: s16b;
    #[no_mangle]
    static mut panel_row_max: s16b;
    #[no_mangle]
    static mut panel_col_min: s16b;
    #[no_mangle]
    static mut panel_col_max: s16b;
    #[no_mangle]
    static mut panel_col_prt: s16b;
    #[no_mangle]
    static mut panel_row_prt: s16b;
    #[no_mangle]
    static mut target_who: s16b;
    #[no_mangle]
    static mut target_col: s16b;
    #[no_mangle]
    static mut target_row: s16b;
    #[no_mangle]
    static mut health_who: s16b;
    #[no_mangle]
    static mut temp_n: s16b;
    #[no_mangle]
    static mut temp_y: [byte_hack; 16384];
    #[no_mangle]
    static mut temp_x: [byte_hack; 16384];
    #[no_mangle]
    static mut angband_term: [*mut term; 8];
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
    static mut s_info: *mut skill_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut f_name: *mut libc::c_char;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut k_name: *mut libc::c_char;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut e_info: *mut ego_item_type;
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
    static mut d_text: *mut libc::c_char;
    #[no_mangle]
    static mut class_info: *mut player_class;
    #[no_mangle]
    static mut race_mod_info: *mut player_race_mod;
    #[no_mangle]
    static mut rmp_name: *mut libc::c_char;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut t_name: *mut libc::c_char;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut wf_name: *mut libc::c_char;
    #[no_mangle]
    static mut wf_text: *mut libc::c_char;
    #[no_mangle]
    static mut st_info: *mut store_info_type;
    #[no_mangle]
    static mut st_name: *mut libc::c_char;
    #[no_mangle]
    static mut ang_sort_comp:
           Option<unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                       _: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut ang_sort_swap:
           Option<unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                       _: libc::c_int) -> ()>;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_re_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_e_idx: u16b;
    #[no_mangle]
    static mut max_rmp_idx: u16b;
    #[no_mangle]
    static mut no_breeds: s16b;
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut doppleganger: s16b;
    #[no_mangle]
    static mut take_notes: bool_;
    #[no_mangle]
    static mut auto_notes: bool_;
    #[no_mangle]
    static mut center_player: bool_;
    #[no_mangle]
    static mut random_quests: [random_quest; 99];
    #[no_mangle]
    static mut max_plev: s32b;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn cave_valid_bold(y: libc::c_int, x: libc::c_int) -> bool_;
    #[no_mangle]
    fn move_cursor_relative(row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn projectable(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                   x2: libc::c_int) -> bool_;
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
    fn prefix(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn maxroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    static mut Term: *mut term;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_addstr(n: libc::c_int, a: byte_hack, s: cptr) -> errr;
    #[no_mangle]
    fn Term_redraw() -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn Term_activate(t: *mut term) -> errr;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn do_cmd_rerate();
    #[no_mangle]
    fn do_poly_self();
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    static mut conv_color: [byte_hack; 16];
    #[no_mangle]
    fn curse_equipment_dg(chance: libc::c_int, heavy_chance: libc::c_int);
    #[no_mangle]
    fn screen_roff(r_idx: libc::c_int, ego: libc::c_int,
                   remember: libc::c_int);
    #[no_mangle]
    fn mego_ok(r_idx: libc::c_int, ego: libc::c_int) -> bool_;
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn delete_monster_idx(i: libc::c_int);
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn monster_race_desc(desc: *mut libc::c_char, r_idx: libc::c_int,
                         ego: libc::c_int);
    #[no_mangle]
    fn lore_treasure(m_idx: libc::c_int, num_item: libc::c_int,
                     num_gold: libc::c_int);
    #[no_mangle]
    fn place_monster_aux(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         slp: bool_, grp: bool_, status: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn summon_specific(y1: libc::c_int, x1: libc::c_int, lev: libc::c_int,
                       type_0: libc::c_int) -> bool_;
    #[no_mangle]
    fn summon_specific_friendly(y1: libc::c_int, x1: libc::c_int,
                                lev: libc::c_int, type_0: libc::c_int,
                                Group_ok: bool_) -> bool_;
    #[no_mangle]
    fn place_monster_one(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         ego: libc::c_int, slp: bool_, status: libc::c_int)
     -> s16b;
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn can_create_companion() -> bool_;
    #[no_mangle]
    fn ai_deincarnate(m_idx: libc::c_int);
    #[no_mangle]
    fn is_friend(m_ptr: *mut monster_type) -> libc::c_int;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn delete_object_idx(o_idx: libc::c_int);
    #[no_mangle]
    fn delete_object(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
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
    fn random_artifact_resistance(o_ptr: *mut object_type);
    #[no_mangle]
    fn random_resistance(o_ptr: *mut object_type, is_scroll: bool_,
                         specific: libc::c_int);
    #[no_mangle]
    fn object_gain_level(o_ptr: *mut object_type);
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn dec_stat(stat: libc::c_int, amount: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn project(who: libc::c_int, rad: libc::c_int, y: libc::c_int,
               x: libc::c_int, dam: libc::c_int, typ: libc::c_int,
               flg: libc::c_int) -> bool_;
    #[no_mangle]
    fn hp_player(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_dec_stat(stat: libc::c_int, mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_res_stat(stat: libc::c_int, full: bool_) -> bool_;
    #[no_mangle]
    fn do_inc_stat(stat: libc::c_int) -> bool_;
    #[no_mangle]
    fn restore_level() -> bool_;
    #[no_mangle]
    fn genocide(player_cast: bool_) -> bool_;
    #[no_mangle]
    fn mass_genocide(player_cast: bool_) -> bool_;
    #[no_mangle]
    fn change_wild_mode();
    #[no_mangle]
    fn dispel_monsters(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn destroy_area(y1: libc::c_int, x1: libc::c_int, r: libc::c_int,
                    full: bool_, bypass: bool_);
    #[no_mangle]
    fn fire_ball(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int,
                 rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn call_chaos();
    #[no_mangle]
    fn activate_ty_curse();
    #[no_mangle]
    fn activate_dg_curse();
    #[no_mangle]
    fn activate_hi_summon();
    #[no_mangle]
    fn create_artifact(o_ptr: *mut object_type, a_scroll: bool_,
                       get_name: bool_) -> bool_;
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn quark_add(str: cptr) -> s16b;
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn is_a_vowel(ch: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_keymap_dir(ch: libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlower(buf: *mut libc::c_char);
    #[no_mangle]
    fn test_monster_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn curse_armor() -> bool_;
    #[no_mangle]
    fn curse_weapon() -> bool_;
    #[no_mangle]
    fn do_poly_wounds();
    #[no_mangle]
    fn add_note(note: *mut libc::c_char, code: libc::c_char);
    #[no_mangle]
    fn apply_level_abilities(level: libc::c_int);
    #[no_mangle]
    fn select_default_melee();
    #[no_mangle]
    fn get_skill_scale(skill: libc::c_int, scale: u32b) -> s16b;
    #[no_mangle]
    fn inc_piety(god: libc::c_int, amt: s32b);
    #[no_mangle]
    fn repeat_push(what: libc::c_int);
    #[no_mangle]
    fn repeat_pull(what: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn string_exec_lua(file: *mut libc::c_char) -> cptr;
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
pub type huge_hack = libc::c_ulong;
pub type s16b = libc::c_short;
pub type u16b = libc::c_ushort;
pub type s32b = libc::c_int;
pub type u32b = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term_win {
    pub cu: bool_,
    pub cv: bool_,
    pub cx: byte_hack,
    pub cy: byte_hack,
    pub a: *mut *mut byte_hack,
    pub c: *mut *mut libc::c_char,
    pub va: *mut byte_hack,
    pub vc: *mut libc::c_char,
    pub ta: *mut *mut byte_hack,
    pub tc: *mut *mut libc::c_char,
    pub vta: *mut byte_hack,
    pub vtc: *mut libc::c_char,
    pub ea: *mut *mut byte_hack,
    pub ec: *mut *mut libc::c_char,
    pub vea: *mut byte_hack,
    pub vec: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term {
    pub user: vptr,
    pub data: vptr,
    pub user_flag: bool_,
    pub data_flag: bool_,
    pub active_flag: bool_,
    pub mapped_flag: bool_,
    pub total_erase: bool_,
    pub fixed_shape: bool_,
    pub icky_corner: bool_,
    pub soft_cursor: bool_,
    pub always_pict: bool_,
    pub higher_pict: bool_,
    pub always_text: bool_,
    pub unused_flag: bool_,
    pub never_bored: bool_,
    pub never_frosh: bool_,
    pub attr_blank: byte_hack,
    pub char_blank: libc::c_char,
    pub key_queue: *mut libc::c_char,
    pub key_head: u16b,
    pub key_tail: u16b,
    pub key_xtra: u16b,
    pub key_size: u16b,
    pub wid: byte_hack,
    pub hgt: byte_hack,
    pub y1: byte_hack,
    pub y2: byte_hack,
    pub x1: *mut byte_hack,
    pub x2: *mut byte_hack,
    pub old: *mut term_win,
    pub scr: *mut term_win,
    pub tmp: *mut term_win,
    pub mem: *mut term_win,
    pub init_hook: Option<unsafe extern "C" fn(_: *mut term) -> ()>,
    pub nuke_hook: Option<unsafe extern "C" fn(_: *mut term) -> ()>,
    pub user_hook: Option<unsafe extern "C" fn(_: libc::c_int) -> errr>,
    pub xtra_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                              -> errr>,
    pub curs_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                              -> errr>,
    pub wipe_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                               _: libc::c_int) -> errr>,
    pub text_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                               _: libc::c_int, _: byte_hack,
                                               _: cptr) -> errr>,
    pub resize_hook: Option<unsafe extern "C" fn() -> ()>,
    pub pict_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                               _: libc::c_int,
                                               _: *const byte_hack,
                                               _: *const libc::c_char,
                                               _: *const byte_hack,
                                               _: *const libc::c_char,
                                               _: *const byte_hack,
                                               _: *const libc::c_char)
                              -> errr>,
}
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
pub struct random_quest {
    pub type_0: byte_hack,
    pub r_idx: s16b,
    pub done: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct skill_type {
    pub name: u32b,
    pub desc: u32b,
    pub action_desc: u32b,
    pub action_mkey: s16b,
    pub i_value: s32b,
    pub i_mod: s32b,
    pub value: s32b,
    pub mod_0: s32b,
    pub rate: s16b,
    pub uses: u32b,
    pub action: [s16b; 200],
    pub father: s16b,
    pub dev: bool_,
    pub order: s16b,
    pub hidden: bool_,
    pub random_gain_chance: byte_hack,
    pub flags1: u32b,
}
/* File: xtra2.c */
/* File: xtra2.c */
/* Purpose: effects of various "objects", targetting and panel handling */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Invoke The Rush
 */
#[no_mangle]
pub unsafe extern "C" fn set_rush(mut v: libc::c_int) -> bool_ {
    let mut j: libc::c_int = 0;
    /* Invoke The Bust */
    if v == 0 {
        (*p_ptr).rush = 0 as libc::c_int as s16b;
        j =
            50 as libc::c_int -
                (Rand_div((*p_ptr).lev as s32b) + 1 as libc::c_int);
        set_paralyzed(j);
        set_slow(j + 50 as libc::c_int -
                     (Rand_div((*p_ptr).lev as s32b) + 1 as libc::c_int));
        return 1 as libc::c_int as bool_
    }
    /* When is The Bust going to happen? */
    (*p_ptr).rush = v as s16b;
    /* The bonuses of The Rush */
    set_hero((*p_ptr).hero as libc::c_int + v);
    set_tim_deadly((*p_ptr).tim_deadly as libc::c_int + v);
    set_strike((*p_ptr).strike as libc::c_int + v);
    if Rand_div(100 as libc::c_int) <
           (*p_ptr).lev as libc::c_int / 2 as libc::c_int {
        set_light_speed((*p_ptr).lightspeed as libc::c_int + v);
    } else { set_fast((*p_ptr).fast as libc::c_int + v, 10 as libc::c_int); }
    if Rand_div(100 as libc::c_int) <
           (*p_ptr).lev as libc::c_int / 2 as libc::c_int {
        set_tim_esp((*p_ptr).tim_esp as libc::c_int + v);
    }
    return 1 as libc::c_int as bool_;
}
/*
 * Set "p_ptr->parasite" and "p_ptr->parasite_r_idx"
 * notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_parasite(mut v: libc::c_int, mut r: libc::c_int)
 -> bool_ {
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- Force good values */
    v =
        if v > 10000 as libc::c_int {
            10000 as libc::c_int
        } else if v < 0 as libc::c_int { 0 as libc::c_int } else { v };
    /* Open */
    if v != 0 {
        if (*p_ptr).parasite == 0 {
            msg_print(b"You feel something growing in you.\x00" as *const u8
                          as *const libc::c_char);
            notice = 1 as libc::c_int as bool_
        }
    } else if (*p_ptr).parasite != 0 {
        if Rand_div(100 as libc::c_int) < 80 as libc::c_int {
            let mut r_name_0: [libc::c_char; 80] = [0; 80];
            let mut wx: libc::c_int = 0;
            let mut wy: libc::c_int = 0;
            let mut attempts: libc::c_int = 500 as libc::c_int;
            monster_race_desc(r_name_0.as_mut_ptr(),
                              (*p_ptr).parasite_r_idx as libc::c_int,
                              0 as libc::c_int);
            loop  {
                scatter(&mut wy, &mut wx, (*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, 10 as libc::c_int);
                if !(!(wy > 0 as libc::c_int && wx > 0 as libc::c_int &&
                           wy < cur_hgt as libc::c_int - 1 as libc::c_int &&
                           wx < cur_wid as libc::c_int - 1 as libc::c_int &&
                           ((*f_info.offset((*cave[wy as
                                                       usize].offset(wx as
                                                                         isize)).feat
                                                as isize)).flags1 as
                                libc::c_long & 0x10 as libc::c_long != 0 &&
                                (*cave[wy as usize].offset(wx as isize)).feat
                                    as libc::c_int != 0xaf as libc::c_int)) &&
                         { attempts -= 1; (attempts) != 0 }) {
                    break ;
                }
            }
            if place_monster_one(wy, wx,
                                 (*p_ptr).parasite_r_idx as libc::c_int,
                                 0 as libc::c_int, 0 as libc::c_int as bool_,
                                 -(2 as libc::c_int)) != 0 {
                cmsg_format(14 as libc::c_int as byte_hack,
                            b"Your body convulses and spawns %s.\x00" as
                                *const u8 as *const libc::c_char,
                            r_name_0.as_mut_ptr());
                (*p_ptr).food =
                    ((*p_ptr).food as libc::c_int - 750 as libc::c_int) as
                        s16b;
                if ((*p_ptr).food as libc::c_int) < 100 as libc::c_int {
                    (*p_ptr).food = 100 as libc::c_int as s16b
                }
            }
        } else {
            cmsg_print(14 as libc::c_int as byte_hack,
                       b"The hideous thing growing in you seems to die.\x00"
                           as *const u8 as *const libc::c_char);
        }
        notice = 1 as libc::c_int as bool_
    }
    /* Shut */
    /* Use the value */
    (*p_ptr).parasite = v as s16b;
    (*p_ptr).parasite_r_idx = r as s16b;
    /* Nothing to notice */
    if notice == 0 { return 0 as libc::c_int as bool_ }
    /* Disturb */
    if disturb_state != 0 { disturb(0 as libc::c_int, 0 as libc::c_int); }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Result */
    return 1 as libc::c_int as bool_;
}
/*
 * Set a simple player field.
 */
unsafe extern "C" fn set_simple_field(mut p_field: *mut s16b, mut v: s16b,
                                      mut activate_color: byte_hack,
                                      mut activate_msg: cptr,
                                      mut deactivate_color: byte_hack,
                                      mut deactivate_msg: cptr) -> bool_ {
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- Force good values */
    v =
        if v as libc::c_int > 10000 as libc::c_int {
            10000 as libc::c_int
        } else if (v as libc::c_int) < 0 as libc::c_int {
            0 as libc::c_int
        } else { v as libc::c_int } as s16b;
    /* Open */
    if v != 0 {
        if *p_field == 0 {
            cmsg_print(activate_color, activate_msg);
            notice = 1 as libc::c_int as bool_
        }
    } else if *p_field != 0 {
        cmsg_print(deactivate_color, deactivate_msg);
        notice = 1 as libc::c_int as bool_
    }
    /* Shut */
    /* Use the value */
    *p_field = v;
    /* Nothing to notice */
    if notice == 0 { return 0 as libc::c_int as bool_ }
    /* Disturb */
    if disturb_state != 0 { disturb(0 as libc::c_int, 0 as libc::c_int); }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Result */
    return 1 as libc::c_int as bool_;
}
/*
 * Set "p_ptr->tim_project" and others
 * notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_project(mut v: libc::c_int, mut gf: s16b,
                                     mut dam: s16b, mut rad: s16b,
                                     mut flag: s16b) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).tim_project, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"Your weapon starts glowing.\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"Your weapon stops glowing.\x00" as *const u8 as
                             *const libc::c_char);
    /* Use the values */
    (*p_ptr).tim_project_gf = gf;
    (*p_ptr).tim_project_dam = dam;
    (*p_ptr).tim_project_rad = rad;
    (*p_ptr).tim_project_flag = flag;
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->tim_roots" and others
 * notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_roots(mut v: libc::c_int, mut ac: s16b,
                                   mut dam: s16b) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).tim_roots, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"Roots dive into the floor from your feet.\x00" as
                             *const u8 as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"The roots of your feet suddenly vanish.\x00" as
                             *const u8 as *const libc::c_char);
    /* Use the values */
    (*p_ptr).tim_roots_dam = dam;
    (*p_ptr).tim_roots_ac = ac;
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->tim_(magic|water)_breath" and others
 * notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_breath(mut v: libc::c_int,
                                        mut magical: bool_) -> bool_ {
    if magical != 0 {
        return set_simple_field(&mut (*p_ptr).tim_magic_breath, v as s16b,
                                1 as libc::c_int as byte_hack,
                                b"Air seems to fill your lungs without breathing.\x00"
                                    as *const u8 as *const libc::c_char,
                                1 as libc::c_int as byte_hack,
                                b"You need to breathe again.\x00" as *const u8
                                    as *const libc::c_char)
    } else {
        return set_simple_field(&mut (*p_ptr).tim_water_breath, v as s16b,
                                1 as libc::c_int as byte_hack,
                                b"Water seems to fill your lungs.\x00" as
                                    *const u8 as *const libc::c_char,
                                1 as libc::c_int as byte_hack,
                                b"The water filling your lungs evaporates.\x00"
                                    as *const u8 as *const libc::c_char)
    };
}
/*
 * Set "p_ptr->absorb_soul"
 * notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_absorb_soul(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).absorb_soul, v as s16b,
                            8 as libc::c_int as byte_hack,
                            b"You start absorbing the souls of your foes.\x00"
                                as *const u8 as *const libc::c_char,
                            8 as libc::c_int as byte_hack,
                            b"You stop absorbing the souls of dead foes.\x00"
                                as *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->disrupt_shield"
 * notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_disrupt_shield(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).disrupt_shield, v as s16b,
                            14 as libc::c_int as byte_hack,
                            b"You feel invulnerable.\x00" as *const u8 as
                                *const libc::c_char,
                            12 as libc::c_int as byte_hack,
                            b"You are more vulnerable.\x00" as *const u8 as
                                *const libc::c_char);
}
/*
 * Set "p_ptr->prob_travel"
 * notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_prob_travel(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).prob_travel, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You feel instable.\x00" as *const u8 as
                                *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are more stable.\x00" as *const u8 as
                                *const libc::c_char);
}
/*
 * Set "p_ptr->tim_invis", and "p_ptr->tim_inv_pow",
 * notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_invis(mut v: libc::c_int, mut p: libc::c_int)
 -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).tim_invisible, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel your body fade away.\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You are no longer invisible.\x00" as *const u8 as
                             *const libc::c_char);
    /* Use the power value */
    (*p_ptr).tim_inv_pow = p as s16b;
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->tim_poison",
 * notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_poison(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).tim_poison, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"Your hands are dripping with venom.\x00" as
                                *const u8 as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"The venom source dries out.\x00" as *const u8 as
                                *const libc::c_char);
}
/*
 * Set "no_breeds"
 */
#[no_mangle]
pub unsafe extern "C" fn set_no_breeders(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut no_breeds, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You feel an anti-sexual aura.\x00" as *const u8
                                as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You no longer feel an anti-sexual aura.\x00" as
                                *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->tim_deadly"
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_deadly(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).tim_deadly, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You feel extremely accurate.\x00" as *const u8
                                as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are suddenly much less accurate.\x00" as
                                *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->tim_ffall"
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_ffall(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).tim_ffall, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You feel very light.\x00" as *const u8 as
                                *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are suddenly heavier.\x00" as *const u8 as
                                *const libc::c_char);
}
/*
 * Set "p_ptr->tim_fly"
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_fly(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).tim_fly, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You feel able to reach the clouds.\x00" as
                                *const u8 as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are suddenly a lot heavier.\x00" as
                                *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->meditation"
 */
#[no_mangle]
pub unsafe extern "C" fn set_meditation(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).meditation, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You start meditating on yourself...\x00" as
                             *const u8 as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You stop your self meditation.\x00" as *const u8 as
                             *const libc::c_char);
    /* Recalculate bonuses */
    if notice != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x20 as libc::c_long) as u32b
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->tim_reflect"
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_reflect(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).tim_reflect, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You start reflecting the world around you.\x00"
                                as *const u8 as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You stop reflecting.\x00" as *const u8 as
                                *const libc::c_char);
}
/*
 * Set "p_ptr->tim_res_time"
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_res_time(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).tim_res_time, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You are now protected against space-time distortions.\x00"
                                as *const u8 as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are no longer protected against space-time distortions.\x00"
                                as *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->tim_fire_aura"
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_fire_aura(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).tim_fire_aura, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You are enveloped in flames.\x00" as *const u8
                                as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are no longer enveloped in flames.\x00" as
                                *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->strike"
 */
#[no_mangle]
pub unsafe extern "C" fn set_strike(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).strike, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You feel very accurate.\x00" as *const u8 as
                                *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are no longer very accurate.\x00" as
                                *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->oppose_ld"
 */
#[no_mangle]
pub unsafe extern "C" fn set_oppose_ld(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).oppose_ld, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You feel protected against light\'s fluctuation.\x00"
                                as *const u8 as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are no longer protected against light\'s fluctuation.\x00"
                                as *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->oppose_cc"
 */
#[no_mangle]
pub unsafe extern "C" fn set_oppose_cc(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).oppose_cc, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You feel protected against raw chaos.\x00" as
                                *const u8 as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are no longer protected against chaos.\x00"
                                as *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->oppose_ss"
 */
#[no_mangle]
pub unsafe extern "C" fn set_oppose_ss(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).oppose_ss, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You feel protected against the ravages of sound and shards.\x00"
                                as *const u8 as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are no longer protected against the ravages of sound and shards.\x00"
                                as *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->oppose_nex"
 */
#[no_mangle]
pub unsafe extern "C" fn set_oppose_nex(mut v: libc::c_int) -> bool_ {
    return set_simple_field(&mut (*p_ptr).oppose_nex, v as s16b,
                            1 as libc::c_int as byte_hack,
                            b"You feel protected against the strange forces of nexus.\x00"
                                as *const u8 as *const libc::c_char,
                            1 as libc::c_int as byte_hack,
                            b"You are no longer protected against the strange forces of nexus.\x00"
                                as *const u8 as *const libc::c_char);
}
/*
 * Set "p_ptr->tim_mimic", and "p_ptr->mimic_form",
 * notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_mimic(mut v: libc::c_int, mut p: libc::c_int,
                                   mut level: libc::c_int) -> bool_ {
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- Force good values */
    v =
        if v > 10000 as libc::c_int {
            10000 as libc::c_int
        } else if v < 0 as libc::c_int { 0 as libc::c_int } else { v };
    /* Open */
    if v != 0 {
        if (*p_ptr).tim_mimic == 0 {
            msg_print(b"You feel your body change.\x00" as *const u8 as
                          *const libc::c_char);
            (*p_ptr).mimic_form = p as byte_hack;
            notice = 1 as libc::c_int as bool_
        }
    } else if (*p_ptr).tim_mimic != 0 {
        msg_print(b"You are no longer transformed.\x00" as *const u8 as
                      *const libc::c_char);
        (*p_ptr).mimic_form = 0 as libc::c_int as byte_hack;
        notice = 1 as libc::c_int as bool_;
        if p ==
               resolve_mimic_name(b"Bear\x00" as *const u8 as
                                      *const libc::c_char) {
            (*s_info.offset(47 as libc::c_int as isize)).hidden =
                1 as libc::c_int as bool_;
            select_default_melee();
        }
        p = 0 as libc::c_int
    }
    /* Shut */
    /* Use the value */
    (*p_ptr).tim_mimic = v as s16b;
    (*p_ptr).mimic_level = level as s16b;
    /* Nothing to notice */
    if notice == 0 { return 0 as libc::c_int as bool_ }
    /* Disturb */
    if disturb_state != 0 { disturb(0 as libc::c_int, 0 as libc::c_int); }
    /* Redraw title */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x2 as libc::c_long) as u32b;
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x4 as libc::c_long | 0x1 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    /* Result */
    return 1 as libc::c_int as bool_;
}
/*
 * Set "p_ptr->blind", notice observable changes
 *
 * Note the use of "PU_UN_VIEW", which is needed to memorize any terrain
 * features which suddenly become "visible".
 * Note that blindness is currently the only thing which can affect
 * "player_can_see_bold()".
 */
#[no_mangle]
pub unsafe extern "C" fn set_blind(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).blind, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You are blind!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You can see again.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Fully update the visuals */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x10000 as libc::c_long | 0x100000 as libc::c_long |
                      0x1000000 as libc::c_long | 0x200000 as libc::c_long))
                as u32b;
        /* Redraw map */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as
                u32b;
        /* Redraw the "blind" */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x10000 as libc::c_long) as
                u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->tim_lite", notice observable changes
 *
 * Note the use of "PU_VIEW", which is needed to
 * memorize any terrain features which suddenly become "visible".
 * Note that blindness is currently the only thing which can affect
 * "player_can_see_bold()".
 */
#[no_mangle]
pub unsafe extern "C" fn set_lite(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).tim_lite, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You suddenly seem brighter!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You are no longer bright.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Fully update the visuals */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x1000000 as libc::c_long)) as
                u32b;
        /* Redraw map */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as
                u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->confused", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_confused(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).confused, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You are confused!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel less confused now.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Redraw the "confused" */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x20000 as libc::c_long) as
                u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->poisoned", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_poisoned(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).poisoned, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You are poisoned!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You are no longer poisoned.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Redraw the "poisoned" */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x80000 as libc::c_long) as
                u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->afraid", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_afraid(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).afraid, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You are terrified!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel bolder now.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Redraw the "afraid" */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x40000 as libc::c_long) as
                u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->paralyzed", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_paralyzed(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).paralyzed, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You are paralyzed!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You can move again.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->image", notice observable changes
 *
 * Note that we must redraw the map when hallucination changes.
 */
#[no_mangle]
pub unsafe extern "C" fn set_image(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).image, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"Oh, wow! Everything looks so cosmic now!\x00" as
                             *const u8 as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You can see clearly again.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Redraw map */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as
                u32b;
        /* Update monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as
                u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x80 as libc::c_long | 0x10 as libc::c_long)) as u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->lightspeed", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_light_speed(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).lightspeed, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel as if time has stopped!\x00" as *const u8
                             as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel time returning to its normal rate.\x00" as
                             *const u8 as *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
#[no_mangle]
pub unsafe extern "C" fn set_fast(mut v: libc::c_int, mut p: libc::c_int)
 -> bool_ {
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- Force good values */
    v =
        if v > 10000 as libc::c_int {
            10000 as libc::c_int
        } else if v < 0 as libc::c_int { 0 as libc::c_int } else { v };
    /* Open */
    if v != 0 {
        if (*p_ptr).fast == 0 {
            msg_print(b"You feel yourself moving faster!\x00" as *const u8 as
                          *const libc::c_char);
            notice = 1 as libc::c_int as bool_
        }
    } else if (*p_ptr).fast != 0 {
        msg_print(b"You feel yourself slow down.\x00" as *const u8 as
                      *const libc::c_char);
        p = 0 as libc::c_int;
        notice = 1 as libc::c_int as bool_
    }
    /* Shut */
    /* Use the value */
    (*p_ptr).fast = v as s16b;
    (*p_ptr).speed_factor = p as s16b;
    /* Nothing to notice */
    if notice == 0 { return 0 as libc::c_int as bool_ }
    /* Disturb */
    if disturb_state != 0 { disturb(0 as libc::c_int, 0 as libc::c_int); }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
    /* Result */
    return 1 as libc::c_int as bool_;
}
/*
 * Set "p_ptr->slow", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_slow(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).slow, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel yourself moving slower!\x00" as *const u8
                             as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel yourself speed up.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->shield", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_shield(mut v: libc::c_int, mut p: libc::c_int,
                                    mut o: s16b, mut d1: s16b, mut d2: s16b)
 -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).shield, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"A mystic shield forms around your body!\x00" as
                             *const u8 as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"Your mystic shield crumbles away.\x00" as *const u8
                             as *const libc::c_char);
    /* Use the values */
    (*p_ptr).shield_power = p as s16b;
    (*p_ptr).shield_opt = o;
    (*p_ptr).shield_power_opt = d1;
    (*p_ptr).shield_power_opt2 = d2;
    /* Notice? */
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->blessed", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_blessed(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).blessed, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel righteous!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"The prayer has expired.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->hero", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_hero(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).hero, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel like a hero!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"The heroism wears off.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Recalculate hitpoints */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x10 as libc::c_long) as u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->holy", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_holy(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).holy, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel a holy aura around you!\x00" as *const u8
                             as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"The holy aura vanishes.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->walk_water", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_walk_water(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).walk_water, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel strangely buoyant!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel much less buoyant.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->shero", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_shero(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).shero, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel like a killing machine!\x00" as *const u8
                             as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel less berserk.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Redraw map */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as
                u32b;
        /* Update monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x1000000 as libc::c_long | 0x10 as libc::c_long)) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->protevil", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_protevil(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).protevil, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel safe from evil!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You no longer feel safe from evil.\x00" as
                             *const u8 as *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->protgood", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_protgood(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).protgood, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel safe from good!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You no longer feel safe from good.\x00" as
                             *const u8 as *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->protundead", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_protundead(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).protundead, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel safe from undead!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You no longer feel safe from undead.\x00" as
                             *const u8 as *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->set_shadow", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_shadow(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).tim_wraith, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You leave the physical world and turn into a wraith-being!\x00"
                             as *const u8 as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel opaque.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Redraw map */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as
                u32b;
        /* Update monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as
                u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->invuln", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_invuln(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).invuln, v as s16b,
                         14 as libc::c_int as byte_hack,
                         b"Invulnerability!\x00" as *const u8 as
                             *const libc::c_char,
                         12 as libc::c_int as byte_hack,
                         b"The invulnerability wears off.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Redraw map */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as
                u32b;
        /* Update monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as
                u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->tim_esp", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_esp(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).tim_esp, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel your consciousness expand!\x00" as
                             *const u8 as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"Your consciousness contracts again.\x00" as
                             *const u8 as *const libc::c_char);
    if notice != 0 {
        /* Update the monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as
                u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->tim_thunder", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_thunder(mut v: libc::c_int,
                                         mut p1: libc::c_int,
                                         mut p2: libc::c_int) -> bool_ {
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- Force good values */
    v =
        if v > 10000 as libc::c_int {
            10000 as libc::c_int
        } else if v < 0 as libc::c_int { 0 as libc::c_int } else { v };
    /* Open */
    if v != 0 {
        if (*p_ptr).tim_thunder == 0 {
            msg_print(b"The air around you charges with lightning!\x00" as
                          *const u8 as *const libc::c_char);
            notice = 1 as libc::c_int as bool_
        }
    } else if (*p_ptr).tim_thunder != 0 {
        msg_print(b"The air around you discharges.\x00" as *const u8 as
                      *const libc::c_char);
        notice = 1 as libc::c_int as bool_;
        p2 = 0 as libc::c_int;
        p1 = p2
    }
    /* Shut */
    /* Use the value */
    (*p_ptr).tim_thunder = v as s16b;
    (*p_ptr).tim_thunder_p1 = p1 as s16b;
    (*p_ptr).tim_thunder_p2 = p2 as s16b;
    /* Nothing to notice */
    if notice == 0 { return 0 as libc::c_int as bool_ }
    /* Disturb */
    if disturb_state != 0 { disturb(0 as libc::c_int, 0 as libc::c_int); }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Update the monsters */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
    /* Result */
    return 1 as libc::c_int as bool_;
}
/*
 * Set "p_ptr->tim_invis", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_invis(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).tim_invis, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"Your eyes feel very sensitive!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"Your eyes feel less sensitive.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Update the monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as
                u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->tim_infra", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_infra(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).tim_infra, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"Your eyes begin to tingle!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"Your eyes stop tingling.\x00" as *const u8 as
                             *const libc::c_char);
    if notice != 0 {
        /* Update the monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as
                u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->tim_mental_barrier", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_mental_barrier(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).tim_mental_barrier, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"Your mind grows stronger!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"Your mind is no longer especially strong.\x00" as
                             *const u8 as *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->oppose_acid", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_oppose_acid(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).oppose_acid, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel resistant to acid!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel less resistant to acid.\x00" as *const u8
                             as *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->oppose_elec", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_oppose_elec(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).oppose_elec, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel resistant to electricity!\x00" as
                             *const u8 as *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel less resistant to electricity.\x00" as
                             *const u8 as *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->oppose_fire", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_oppose_fire(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).oppose_fire, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel resistant to fire!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel less resistant to fire.\x00" as *const u8
                             as *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->oppose_cold", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_oppose_cold(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).oppose_cold, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel resistant to cold!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel less resistant to cold.\x00" as *const u8
                             as *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->oppose_pois", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_oppose_pois(mut v: libc::c_int) -> bool_ {
    let mut notice: bool_ =
        set_simple_field(&mut (*p_ptr).oppose_pois, v as s16b,
                         1 as libc::c_int as byte_hack,
                         b"You feel resistant to poison!\x00" as *const u8 as
                             *const libc::c_char,
                         1 as libc::c_int as byte_hack,
                         b"You feel less resistant to poison.\x00" as
                             *const u8 as *const libc::c_char);
    if notice != 0 {
        /* Handle stuff */
        handle_stuff();
    }
    /* Result */
    return notice;
}
/*
 * Set "p_ptr->tim_regen", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_tim_regen(mut v: libc::c_int, mut p: libc::c_int)
 -> bool_ {
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- Force good values */
    v =
        if v > 10000 as libc::c_int {
            10000 as libc::c_int
        } else if v < 0 as libc::c_int { 0 as libc::c_int } else { v };
    /* Open */
    if v != 0 {
        if (*p_ptr).tim_regen == 0 {
            msg_print(b"Your body regenerates much more quickly!\x00" as
                          *const u8 as *const libc::c_char);
            notice = 1 as libc::c_int as bool_
        }
    } else if (*p_ptr).tim_regen != 0 {
        p = 0 as libc::c_int;
        msg_print(b"Your body regenerates much more slowly.\x00" as *const u8
                      as *const libc::c_char);
        notice = 1 as libc::c_int as bool_
    }
    /* Shut */
    /* Use the value */
    (*p_ptr).tim_regen = v as s16b;
    (*p_ptr).tim_regen_pow = p as s16b;
    /* Nothing to notice */
    if notice == 0 { return 0 as libc::c_int as bool_ }
    /* Disturb */
    if disturb_state != 0 { disturb(0 as libc::c_int, 0 as libc::c_int); }
    /* Handle stuff */
    handle_stuff();
    /* Result */
    return 1 as libc::c_int as bool_;
}
/*
 * Set "p_ptr->stun", notice observable changes
 *
 * Note the special code to only notice "range" changes.
 */
#[no_mangle]
pub unsafe extern "C" fn set_stun(mut v: libc::c_int) -> bool_ {
    let mut old_aux: libc::c_int = 0;
    let mut new_aux: libc::c_int = 0;
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- Force good values */
    v =
        if v > 10000 as libc::c_int {
            10000 as libc::c_int
        } else if v < 0 as libc::c_int { 0 as libc::c_int } else { v };
    if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
            (*spp_ptr).flags1) as libc::c_long & 0x8 as libc::c_long != 0 {
        v = 0 as libc::c_int
    }
    /* Knocked out */
    if (*p_ptr).stun as libc::c_int > 100 as libc::c_int {
        old_aux = 3 as libc::c_int
    } else if (*p_ptr).stun as libc::c_int > 50 as libc::c_int {
        old_aux = 2 as libc::c_int
    } else if (*p_ptr).stun as libc::c_int > 0 as libc::c_int {
        old_aux = 1 as libc::c_int
    } else {
        /* Heavy stun */
        /* Stun */
        /* None */
        old_aux = 0 as libc::c_int
    }
    /* Knocked out */
    if v > 100 as libc::c_int {
        new_aux = 3 as libc::c_int
    } else if v > 50 as libc::c_int {
        new_aux = 2 as libc::c_int
    } else if v > 0 as libc::c_int {
        new_aux = 1 as libc::c_int
    } else {
        /* Heavy stun */
        /* Stun */
        /* None */
        new_aux = 0 as libc::c_int
    }
    /* Increase cut */
    if new_aux > old_aux {
        /* Describe the state */
        match new_aux {
            1 => {
                /* Stun */
                msg_print(b"You have been stunned.\x00" as *const u8 as
                              *const libc::c_char);
            }
            2 => {
                /* Heavy stun */
                msg_print(b"You have been heavily stunned.\x00" as *const u8
                              as *const libc::c_char);
            }
            3 => {
                /* Knocked out */
                msg_print(b"You have been knocked out.\x00" as *const u8 as
                              *const libc::c_char);
            }
            _ => { }
        }
        if (Rand_div(1000 as libc::c_int) + 1 as libc::c_int) < v ||
               Rand_div(16 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
            msg_print(b"A vicious blow hits your head.\x00" as *const u8 as
                          *const libc::c_char);
            if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                if (*p_ptr).sustain_int == 0 {
                    do_dec_stat(1 as libc::c_int, 2 as libc::c_int);
                }
                if (*p_ptr).sustain_wis == 0 {
                    do_dec_stat(2 as libc::c_int, 2 as libc::c_int);
                }
            } else if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                          1 as libc::c_int {
                if (*p_ptr).sustain_int == 0 {
                    do_dec_stat(1 as libc::c_int, 2 as libc::c_int);
                }
            } else if (*p_ptr).sustain_wis == 0 {
                do_dec_stat(2 as libc::c_int, 2 as libc::c_int);
            }
        }
        /* Notice */
        notice = 1 as libc::c_int as bool_
    } else if new_aux < old_aux {
        /* Decrease cut */
        /* Describe the state */
        match new_aux {
            0 => {
                /* None */
                msg_print(b"You are no longer stunned.\x00" as *const u8 as
                              *const libc::c_char);
                if disturb_state != 0 {
                    disturb(0 as libc::c_int, 0 as libc::c_int);
                }
            }
            _ => { }
        }
        /* Notice */
        notice = 1 as libc::c_int as bool_
    }
    /* Use the value */
    (*p_ptr).stun = v as s16b;
    /* No change */
    if notice == 0 { return 0 as libc::c_int as bool_ }
    /* Disturb */
    if disturb_state != 0 { disturb(0 as libc::c_int, 0 as libc::c_int); }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Redraw the "stun" */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x2000 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
    /* Result */
    return 1 as libc::c_int as bool_;
}
/*
 * Set "p_ptr->cut", notice observable changes
 *
 * Note the special code to only notice "range" changes.
 */
#[no_mangle]
pub unsafe extern "C" fn set_cut(mut v: libc::c_int) -> bool_ {
    let mut old_aux: libc::c_int = 0;
    let mut new_aux: libc::c_int = 0;
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- Force good values */
    v =
        if v > 10000 as libc::c_int {
            10000 as libc::c_int
        } else if v < 0 as libc::c_int { 0 as libc::c_int } else { v };
    if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
            (*spp_ptr).flags1) as libc::c_long & 0x800 as libc::c_long != 0 {
        v = 0 as libc::c_int
    }
    /* Mortal wound */
    if (*p_ptr).cut as libc::c_int > 1000 as libc::c_int {
        old_aux = 7 as libc::c_int
    } else if (*p_ptr).cut as libc::c_int > 200 as libc::c_int {
        old_aux = 6 as libc::c_int
    } else if (*p_ptr).cut as libc::c_int > 100 as libc::c_int {
        old_aux = 5 as libc::c_int
    } else if (*p_ptr).cut as libc::c_int > 50 as libc::c_int {
        old_aux = 4 as libc::c_int
    } else if (*p_ptr).cut as libc::c_int > 25 as libc::c_int {
        old_aux = 3 as libc::c_int
    } else if (*p_ptr).cut as libc::c_int > 10 as libc::c_int {
        old_aux = 2 as libc::c_int
    } else if (*p_ptr).cut as libc::c_int > 0 as libc::c_int {
        old_aux = 1 as libc::c_int
    } else {
        /* Deep gash */
        /* Severe cut */
        /* Nasty cut */
        /* Bad cut */
        /* Light cut */
        /* Graze */
        /* None */
        old_aux = 0 as libc::c_int
    }
    /* Mortal wound */
    if v > 1000 as libc::c_int {
        new_aux = 7 as libc::c_int
    } else if v > 200 as libc::c_int {
        new_aux = 6 as libc::c_int
    } else if v > 100 as libc::c_int {
        new_aux = 5 as libc::c_int
    } else if v > 50 as libc::c_int {
        new_aux = 4 as libc::c_int
    } else if v > 25 as libc::c_int {
        new_aux = 3 as libc::c_int
    } else if v > 10 as libc::c_int {
        new_aux = 2 as libc::c_int
    } else if v > 0 as libc::c_int {
        new_aux = 1 as libc::c_int
    } else {
        /* Deep gash */
        /* Severe cut */
        /* Nasty cut */
        /* Bad cut */
        /* Light cut */
        /* Graze */
        /* None */
        new_aux = 0 as libc::c_int
    }
    /* Increase cut */
    if new_aux > old_aux {
        /* Describe the state */
        match new_aux {
            1 => {
                /* Graze */
                msg_print(b"You have been given a graze.\x00" as *const u8 as
                              *const libc::c_char);
            }
            2 => {
                /* Light cut */
                msg_print(b"You have been given a light cut.\x00" as *const u8
                              as *const libc::c_char);
            }
            3 => {
                /* Bad cut */
                msg_print(b"You have been given a bad cut.\x00" as *const u8
                              as *const libc::c_char);
            }
            4 => {
                /* Nasty cut */
                msg_print(b"You have been given a nasty cut.\x00" as *const u8
                              as *const libc::c_char);
            }
            5 => {
                /* Severe cut */
                msg_print(b"You have been given a severe cut.\x00" as
                              *const u8 as *const libc::c_char);
            }
            6 => {
                /* Deep gash */
                msg_print(b"You have been given a deep gash.\x00" as *const u8
                              as *const libc::c_char);
            }
            7 => {
                /* Mortal wound */
                msg_print(b"You have been given a mortal wound.\x00" as
                              *const u8 as *const libc::c_char);
            }
            _ => { }
        }
        /* Notice */
        notice = 1 as libc::c_int as bool_;
        if (Rand_div(1000 as libc::c_int) + 1 as libc::c_int) < v ||
               Rand_div(16 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
            if (*p_ptr).sustain_chr == 0 {
                msg_print(b"You have been horribly scarred.\x00" as *const u8
                              as *const libc::c_char);
                do_dec_stat(5 as libc::c_int, 2 as libc::c_int);
            }
        }
    } else if new_aux < old_aux {
        /* Decrease cut */
        /* Describe the state */
        match new_aux {
            0 => {
                /* None */
                msg_print(b"You are no longer bleeding.\x00" as *const u8 as
                              *const libc::c_char);
                if disturb_state != 0 {
                    disturb(0 as libc::c_int, 0 as libc::c_int);
                }
            }
            _ => { }
        }
        /* Notice */
        notice = 1 as libc::c_int as bool_
    }
    /* Use the value */
    (*p_ptr).cut = v as s16b;
    /* No change */
    if notice == 0 { return 0 as libc::c_int as bool_ }
    /* Disturb */
    if disturb_state != 0 { disturb(0 as libc::c_int, 0 as libc::c_int); }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Redraw the "cut" */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x1000 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
    /* Result */
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn drop_from_wild() {
    /* Hack -- Not if player were in normal mode in previous turn */
    if (*p_ptr).old_wild_mode == 0 { return }
    if (*p_ptr).wild_mode as libc::c_int != 0 && dun_level == 0 {
        (*p_ptr).wilderness_x = (*p_ptr).px as s32b;
        (*p_ptr).wilderness_y = (*p_ptr).py as s32b;
        change_wild_mode();
        (*p_ptr).energy = 100 as libc::c_int;
        energy_use = 0 as libc::c_int
    };
}
/*
 * Set "p_ptr->food", notice observable changes
 *
 * The "p_ptr->food" variable can get as large as 20000, allowing the
 * addition of the most "filling" item, Elvish Waybread, which adds
 * 7500 food units, without overflowing the 32767 maximum limit.
 *
 * Perhaps we should disturb the player with various messages,
 * especially messages about hunger status changes.  XXX XXX XXX
 *
 * Digestion of food is handled in "dungeon.c", in which, normally,
 * the player digests about 20 food units per 100 game turns, more
 * when "fast", more when "regenerating", less with "slow digestion",
 * but when the player is "gorged", he digests 100 food units per 10
 * game turns, or a full 1000 food units per 100 game turns.
 *
 * Note that the player's speed is reduced by 10 units while gorged,
 * so if the player eats a single food ration (5000 food units) when
 * full (15000 food units), he will be gorged for (5000/100)*10 = 500
 * game turns, or 500/(100/5) = 25 player turns (if nothing else is
 * affecting the player speed).
 */
#[no_mangle]
pub unsafe extern "C" fn set_food(mut v: libc::c_int) -> bool_ {
    let mut old_aux: libc::c_int = 0;
    let mut new_aux: libc::c_int = 0;
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- Force good values */
    v =
        if v > 20000 as libc::c_int {
            20000 as libc::c_int
        } else if v < 0 as libc::c_int { 0 as libc::c_int } else { v };
    /* Fainting / Starving */
    if ((*p_ptr).food as libc::c_int) < 500 as libc::c_int {
        old_aux = 0 as libc::c_int
    } else if ((*p_ptr).food as libc::c_int) < 1000 as libc::c_int {
        old_aux = 1 as libc::c_int
    } else if ((*p_ptr).food as libc::c_int) < 2000 as libc::c_int {
        old_aux = 2 as libc::c_int
    } else if ((*p_ptr).food as libc::c_int) < 10000 as libc::c_int {
        old_aux = 3 as libc::c_int
    } else if ((*p_ptr).food as libc::c_int) < 15000 as libc::c_int {
        old_aux = 4 as libc::c_int
    } else {
        /* Weak */
        /* Hungry */
        /* Normal */
        /* Full */
        /* Gorged */
        old_aux = 5 as libc::c_int
    }
    /* Fainting / Starving */
    if v < 500 as libc::c_int {
        new_aux = 0 as libc::c_int
    } else if v < 1000 as libc::c_int {
        new_aux = 1 as libc::c_int
    } else if v < 2000 as libc::c_int {
        new_aux = 2 as libc::c_int
    } else if v < 10000 as libc::c_int {
        new_aux = 3 as libc::c_int
    } else if v < 15000 as libc::c_int {
        new_aux = 4 as libc::c_int
    } else {
        /* Weak */
        /* Hungry */
        /* Normal */
        /* Full */
        /* Gorged */
        new_aux = 5 as libc::c_int
    }
    /* Food increase */
    if new_aux > old_aux {
        /* Describe the state */
        match new_aux {
            1 => {
                /* Weak */
                msg_print(b"You are still weak.\x00" as *const u8 as
                              *const libc::c_char);
            }
            2 => {
                /* Hungry */
                msg_print(b"You are still hungry.\x00" as *const u8 as
                              *const libc::c_char);
            }
            3 => {
                /* Normal */
                msg_print(b"You are no longer hungry.\x00" as *const u8 as
                              *const libc::c_char);
            }
            4 => {
                /* Full */
                msg_print(b"You are full!\x00" as *const u8 as
                              *const libc::c_char);
            }
            5 => {
                /* Bloated */
                msg_print(b"You have gorged yourself!\x00" as *const u8 as
                              *const libc::c_char);
            }
            _ => { }
        }
        /* Change */
        notice = 1 as libc::c_int as bool_
    } else if new_aux < old_aux {
        /* Food decrease */
        /* Describe the state */
        match new_aux {
            0 => {
                /* Fainting / Starving */
                msg_print(b"You are getting faint from hunger!\x00" as
                              *const u8 as *const libc::c_char);
                drop_from_wild();
            }
            1 => {
                /* Weak */
                msg_print(b"You are getting weak from hunger!\x00" as
                              *const u8 as *const libc::c_char);
                drop_from_wild();
            }
            2 => {
                /* Hungry */
                msg_print(b"You are getting hungry.\x00" as *const u8 as
                              *const libc::c_char);
            }
            3 => {
                /* Normal */
                msg_print(b"You are no longer full.\x00" as *const u8 as
                              *const libc::c_char);
            }
            4 => {
                /* Full */
                msg_print(b"You are no longer gorged.\x00" as *const u8 as
                              *const libc::c_char);
            }
            _ => { }
        }
        /* Change */
        notice = 1 as libc::c_int as bool_
    }
    /* Use the value */
    (*p_ptr).food = v as s16b;
    /* Nothing to notice */
    if notice == 0 { return 0 as libc::c_int as bool_ }
    /* Disturb */
    if disturb_state != 0 { disturb(0 as libc::c_int, 0 as libc::c_int); }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Redraw hunger */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
    /* Result */
    return 1 as libc::c_int as bool_;
}
/*
 * Advance experience levels and print experience
 */
#[no_mangle]
pub unsafe extern "C" fn check_experience() {
    let mut gained: libc::c_int = 0 as libc::c_int;
    let mut level_reward: bool_ = 0 as libc::c_int as bool_;
    let mut level_corruption: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- lower limit */
    if (*p_ptr).exp < 0 as libc::c_int { (*p_ptr).exp = 0 as libc::c_int }
    /* Hack -- lower limit */
    if (*p_ptr).max_exp < 0 as libc::c_int {
        (*p_ptr).max_exp = 0 as libc::c_int
    }
    /* Hack -- upper limit */
    if (*p_ptr).exp as libc::c_long > 99999999 as libc::c_long {
        (*p_ptr).exp = 99999999 as libc::c_long as s32b
    }
    /* Hack -- upper limit */
    if (*p_ptr).max_exp as libc::c_long > 99999999 as libc::c_long {
        (*p_ptr).max_exp = 99999999 as libc::c_long as s32b
    }
    /* Hack -- maintain "max" experience */
    if (*p_ptr).exp > (*p_ptr).max_exp { (*p_ptr).max_exp = (*p_ptr).exp }
    /* Redraw experience */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x8 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
    /* Lose levels while possible */
    while (*p_ptr).lev as libc::c_int > 1 as libc::c_int &&
              ((*p_ptr).exp as libc::c_long) <
                  (player_exp[((*p_ptr).lev as libc::c_int - 2 as libc::c_int)
                                  as usize] * (*p_ptr).expfact as libc::c_int)
                      as libc::c_long / 100 as libc::c_long {
        /* Lose a level */
        (*p_ptr).lev -= 1;
        gained -= 1;
        lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
        /* Update some stuff */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x1 as libc::c_long | 0x10 as libc::c_long |
                      0x20 as libc::c_long | 0x40 as libc::c_long |
                      0x8 as libc::c_long)) as u32b;
        /* Redraw some stuff */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long |
                 (0x4 as libc::c_long | 0x2 as libc::c_long |
                      0x8 as libc::c_long)) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
        /* Handle stuff */
        handle_stuff();
    }
    /* Gain levels while possible */
    while ((*p_ptr).lev as libc::c_int) < 50 as libc::c_int &&
              ((*p_ptr).lev as libc::c_int) < max_plev &&
              (*p_ptr).exp as libc::c_long >=
                  (player_exp[((*p_ptr).lev as libc::c_int - 1 as libc::c_int)
                                  as usize] * (*p_ptr).expfact as libc::c_int)
                      as libc::c_long / 100 as libc::c_long {
        /* Gain a level */
        (*p_ptr).lev += 1;
        gained += 1;
        lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
        /* Save the highest level */
        if (*p_ptr).lev as libc::c_int > (*p_ptr).max_plv as libc::c_int {
            (*p_ptr).max_plv = (*p_ptr).lev;
            if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                    (*spp_ptr).flags1) as libc::c_long &
                   0x1000 as libc::c_long != 0 &&
                   Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                level_corruption = 1 as libc::c_int as bool_
            }
        }
        /* Sound */
        sound(6 as libc::c_int);
        /* Message */
        cmsg_format(13 as libc::c_int as byte_hack,
                    b"Welcome to level %d.\x00" as *const u8 as
                        *const libc::c_char, (*p_ptr).lev as libc::c_int);
        if ((*p_ptr).skill_last_level as libc::c_int) <
               (*p_ptr).lev as libc::c_int {
            let mut pts: s32b = 0;
            call_lua(b"exec_module_info\x00" as *const u8 as
                         *const libc::c_char,
                     b"(s)\x00" as *const u8 as *const libc::c_char,
                     b"d\x00" as *const u8 as *const libc::c_char,
                     b"skill_per_level\x00" as *const u8 as
                         *const libc::c_char, &mut pts as *mut s32b);
            (*p_ptr).skill_last_level = (*p_ptr).lev;
            (*p_ptr).skill_points =
                ((*p_ptr).skill_points as libc::c_int + pts) as s16b;
            cmsg_format(13 as libc::c_int as byte_hack,
                        b"You can increase %d more skills.\x00" as *const u8
                            as *const libc::c_char,
                        (*p_ptr).skill_points as libc::c_int);
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x400000 as libc::c_long)
                    as u32b
        }
        /* Gain this level's abilities */
        apply_level_abilities((*p_ptr).lev as libc::c_int);
        /* If auto-note taking enabled, write a note to the file.
		 * Only write this note when the level is gained for the first
		 * time.
		 */
        if take_notes as libc::c_int != 0 && auto_notes as libc::c_int != 0 {
            let mut note: [libc::c_char; 80] = [0; 80];
            /* Write note */
            sprintf(note.as_mut_ptr(),
                    b"Reached level %d\x00" as *const u8 as
                        *const libc::c_char, (*p_ptr).lev as libc::c_int);
            add_note(note.as_mut_ptr(), 'L' as i32 as libc::c_char);
        }
        /* Update some stuff */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x1 as libc::c_long | 0x10 as libc::c_long |
                      0x20 as libc::c_long | 0x40 as libc::c_long |
                      0x8 as libc::c_long)) as u32b;
        /* Redraw some stuff */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long |
                 (0x4 as libc::c_long | 0x2 as libc::c_long |
                      0x8 as libc::c_long)) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
        /* Handle stuff */
        handle_stuff();
        if level_reward != 0 {
            gain_level_reward(0 as libc::c_int);
            level_reward = 0 as libc::c_int as bool_
        }
        if level_corruption != 0 {
            msg_print(b"You feel different...\x00" as *const u8 as
                          *const libc::c_char);
            corrupt_corrupted();
            level_corruption = 0 as libc::c_int as bool_
        }
    }
    /* Hook it! */
    process_hooks(20 as libc::c_int,
                  b"(d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, gained);
}
/*
 * Advance experience levels and print experience
 */
#[no_mangle]
pub unsafe extern "C" fn check_experience_obj(mut o_ptr: *mut object_type) {
    /* Hack -- lower limit */
    if (*o_ptr).exp < 0 as libc::c_int { (*o_ptr).exp = 0 as libc::c_int }
    /* Hack -- upper limit */
    if (*o_ptr).exp as libc::c_long > 99999999 as libc::c_long {
        (*o_ptr).exp = 99999999 as libc::c_long as s32b
    }
    /* Gain levels while possible */
    while ((*o_ptr).elevel as libc::c_int) < 50 as libc::c_int &&
              (*o_ptr).exp >=
                  player_exp[((*o_ptr).elevel as libc::c_int -
                                  1 as libc::c_int) as usize] *
                      5 as libc::c_int / 2 as libc::c_int {
        let mut buf: [libc::c_char; 100] = [0; 100];
        /* Add a level */
        (*o_ptr).elevel = (*o_ptr).elevel.wrapping_add(1);
        /* Get object name */
        object_desc(buf.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                    0 as libc::c_int);
        cmsg_format(14 as libc::c_int as byte_hack,
                    b"%s gains a level!\x00" as *const u8 as
                        *const libc::c_char, buf.as_mut_ptr());
        /* What does it gains ? */
        object_gain_level(o_ptr);
    };
}
/*
 * Gain experience (share it to objects if needed)
 */
#[no_mangle]
pub unsafe extern "C" fn gain_exp(mut amount: s32b) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 1 as libc::c_int;
    /* Count the gaining xp objects */
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        let mut f1: u32b = 0;
        let mut f2: u32b = 0;
        let mut f3: u32b = 0;
        let mut f4: u32b = 0;
        let mut f5: u32b = 0;
        let mut esp: u32b = 0;
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        if !((*o_ptr).k_idx == 0) {
            if f4 as libc::c_long & 0x20000000 as libc::c_long != 0 {
                num += 1
            }
        }
        i += 1
    }
    /* Now give the xp */
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr_0: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        let mut f1_0: u32b = 0;
        let mut f2_0: u32b = 0;
        let mut f3_0: u32b = 0;
        let mut f4_0: u32b = 0;
        let mut f5_0: u32b = 0;
        let mut esp_0: u32b = 0;
        object_flags(o_ptr_0, &mut f1_0, &mut f2_0, &mut f3_0, &mut f4_0,
                     &mut f5_0, &mut esp_0);
        if !((*o_ptr_0).k_idx == 0) {
            if f4_0 as libc::c_long & 0x20000000 as libc::c_long != 0 {
                (*o_ptr_0).exp +=
                    2 as libc::c_int * amount / (num * 3 as libc::c_int);
                /* Hack -- upper limit */
                if (*o_ptr_0).exp as libc::c_long > 99999999 as libc::c_long {
                    (*o_ptr_0).exp = 99999999 as libc::c_long as s32b
                }
            }
        }
        i += 1
    }
    if (*p_ptr).max_exp > 0 as libc::c_int &&
           ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                (*spp_ptr).flags1) as libc::c_long & 0x1000 as libc::c_long !=
               0 {
        if (Rand_div((*p_ptr).max_exp) + 1 as libc::c_int) < amount ||
               (Rand_div(12000000 as libc::c_int) + 1 as libc::c_int) < amount
           {
            msg_print(b"You feel different...\x00" as *const u8 as
                          *const libc::c_char);
            corrupt_corrupted();
        }
        /* 12,000,000 is equal to double Morgoth's raw XP value (60,000 * his Dlev (100))*/
    }
    /* Gain some experience */
    (*p_ptr).exp += amount / num;
    /* Hook it! */
    process_hooks(67 as libc::c_int,
                  b"(d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, amount / num);
    /* Slowly recover from experience drainage */
    if (*p_ptr).exp < (*p_ptr).max_exp {
        /* Gain max experience (20%) (was 10%) */
        (*p_ptr).max_exp += amount / 5 as libc::c_int
    }
    /* Check Experience */
    check_experience();
}
/*
 * Lose experience
 */
#[no_mangle]
pub unsafe extern "C" fn lose_exp(mut amount: s32b) {
    /* Never drop below zero experience */
    if amount > (*p_ptr).exp { amount = (*p_ptr).exp }
    /* Lose some experience */
    (*p_ptr).exp -= amount;
    /* Hook it! */
    process_hooks(67 as libc::c_int,
                  b"(d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, amount);
    /* Check Experience */
    check_experience();
}
/*
 * Hack -- Return the "automatic coin type" of a monster race
 * Used to allocate proper treasure when "Creeping coins" die
 *
 * XXX XXX XXX Note the use of actual "monster names"
 */
#[no_mangle]
pub unsafe extern "C" fn get_coin_type(mut r_ptr: *mut monster_race)
 -> libc::c_int {
    let mut name: cptr = r_name.offset((*r_ptr).name as isize) as cptr;
    /* Analyze "coin" monsters */
    if (*r_ptr).d_char as libc::c_int == '$' as i32 {
        /* Look for textual clues */
        if !strstr(name,
                   b" copper \x00" as *const u8 as
                       *const libc::c_char).is_null() {
            return 2 as libc::c_int
        }
        if !strstr(name,
                   b" silver \x00" as *const u8 as
                       *const libc::c_char).is_null() {
            return 5 as libc::c_int
        }
        if !strstr(name,
                   b" gold \x00" as *const u8 as
                       *const libc::c_char).is_null() {
            return 10 as libc::c_int
        }
        if !strstr(name,
                   b" mithril \x00" as *const u8 as
                       *const libc::c_char).is_null() {
            return 16 as libc::c_int
        }
        if !strstr(name,
                   b" adamantite \x00" as *const u8 as
                       *const libc::c_char).is_null() {
            return 17 as libc::c_int
        }
        /* Look for textual clues */
        if !strstr(name,
                   b"Copper \x00" as *const u8 as
                       *const libc::c_char).is_null() {
            return 2 as libc::c_int
        }
        if !strstr(name,
                   b"Silver \x00" as *const u8 as
                       *const libc::c_char).is_null() {
            return 5 as libc::c_int
        }
        if !strstr(name,
                   b"Gold \x00" as *const u8 as *const libc::c_char).is_null()
           {
            return 10 as libc::c_int
        }
        if !strstr(name,
                   b"Mithril \x00" as *const u8 as
                       *const libc::c_char).is_null() {
            return 16 as libc::c_int
        }
        if !strstr(name,
                   b"Adamantite \x00" as *const u8 as
                       *const libc::c_char).is_null() {
            return 17 as libc::c_int
        }
    }
    /* Assume nothing */
    return 0 as libc::c_int;
}
/*
 * This routine handles the production of corpses/skeletons/heads/skulls
 * when a monster is killed.
 */
#[no_mangle]
pub unsafe extern "C" fn place_corpse(mut m_ptr: *mut monster_type) {
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
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
    let mut x: libc::c_int = (*m_ptr).fx as libc::c_int;
    let mut y: libc::c_int = (*m_ptr).fy as libc::c_int;
    /* Get local object */
    i_ptr = &mut object_type_body;
    /* It has a physical form */
    if (*r_ptr).flags9 & 0x1 as libc::c_int as libc::c_uint != 0 {
        /* Wipe the object */
        object_prep(i_ptr,
                    lookup_kind(9 as libc::c_int, 1 as libc::c_int) as
                        libc::c_int);
        /* Unique corpses are unique */
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            object_aware(i_ptr);
            (*i_ptr).name1 = 201 as libc::c_int as byte_hack
        }
        /* Calculate length of time before decay */
        (*i_ptr).pval = (*r_ptr).weight + Rand_div((*r_ptr).weight);
        /* Set weight */
        (*i_ptr).weight =
            (*r_ptr).weight + Rand_div((*r_ptr).weight) / 10 as libc::c_int +
                1 as libc::c_int;
        /* Remember what we are */
        (*i_ptr).pval2 = (*m_ptr).r_idx;
        /* Some hp */
        (*i_ptr).pval3 =
            (maxroll((*r_ptr).hdice as s16b, (*r_ptr).hside as s16b) +
                 (*p_ptr).mhp as libc::c_int) / 2 as libc::c_int;
        (*i_ptr).pval3 -=
            (Rand_div((*i_ptr).pval3) + 1 as libc::c_int) / 3 as libc::c_int;
        (*i_ptr).found = 1 as libc::c_int as byte_hack;
        (*i_ptr).found_aux1 = (*m_ptr).r_idx;
        (*i_ptr).found_aux2 = (*m_ptr).ego as s16b;
        (*i_ptr).found_aux3 = dungeon_type as s16b;
        (*i_ptr).found_aux4 =
            if dungeon_type as libc::c_int == 0 as libc::c_int {
                (*(*wild_map.offset((*p_ptr).wilderness_y as
                                        isize)).offset((*p_ptr).wilderness_x
                                                           as isize)).feat
            } else { dun_level as libc::c_int } as s16b;
        /* Drop it in the dungeon */
        drop_near(i_ptr, -(1 as libc::c_int), y, x);
    }
    /* The creature is an animated skeleton. */
    if (*r_ptr).flags9 & 0x1 as libc::c_int as libc::c_uint == 0 &&
           (*r_ptr).flags9 & 0x2 as libc::c_int as libc::c_uint != 0 {
        /* Wipe the object */
        object_prep(i_ptr,
                    lookup_kind(9 as libc::c_int, 2 as libc::c_int) as
                        libc::c_int);
        /* Unique corpses are unique */
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            object_aware(i_ptr);
            (*i_ptr).name1 = 201 as libc::c_int as byte_hack
        }
        (*i_ptr).pval = 0 as libc::c_int;
        /* Set weight */
        (*i_ptr).weight =
            (*r_ptr).weight / 4 as libc::c_int +
                Rand_div((*r_ptr).weight) / 40 as libc::c_int +
                1 as libc::c_int;
        /* Remember what we are */
        (*i_ptr).pval2 = (*m_ptr).r_idx;
        (*i_ptr).found = 1 as libc::c_int as byte_hack;
        (*i_ptr).found_aux1 = (*m_ptr).r_idx;
        (*i_ptr).found_aux2 = (*m_ptr).ego as s16b;
        (*i_ptr).found_aux3 = dungeon_type as s16b;
        (*i_ptr).found_aux4 =
            if dungeon_type as libc::c_int == 0 as libc::c_int {
                (*(*wild_map.offset((*p_ptr).wilderness_y as
                                        isize)).offset((*p_ptr).wilderness_x
                                                           as isize)).feat
            } else { dun_level as libc::c_int } as s16b;
        /* Drop it in the dungeon */
        drop_near(i_ptr, -(1 as libc::c_int), y, x);
    };
}
/*
 * Handle the "death" of a monster.
 *
 * Disperse treasures centered at the monster location based on the
 * various flags contained in the monster flags fields.
 *
 * Check for "Quest" completion when a quest monster is killed.
 *
 * Note that only the player can induce "monster_death()" on Uniques.
 * Thus (for now) all Quest monsters should be Uniques.
 *
 * Note that monsters can now carry objects, and when a monster dies,
 * it drops all of its objects, which may disappear in crowded rooms.
 */
#[no_mangle]
pub unsafe extern "C" fn monster_death(mut m_idx: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut dump_item: libc::c_int = 0 as libc::c_int;
    let mut dump_gold: libc::c_int = 0 as libc::c_int;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut visible: bool_ =
        ((*m_ptr).ml as libc::c_int != 0 ||
             (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0) as
            libc::c_int as bool_;
    let mut create_stairs: bool_ = 0 as libc::c_int as bool_;
    let mut force_coin: libc::c_int = get_coin_type(r_ptr);
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
    /* Get the location */
    y = (*m_ptr).fy as libc::c_int;
    x = (*m_ptr).fx as libc::c_int;
    /* Process the appropriate hooks */
    process_hooks(0 as libc::c_int,
                  b"(d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, m_idx);
    /* If companion dies, take note */
    if (*m_ptr).status as libc::c_int == 4 as libc::c_int {
        (*p_ptr).companion_killed += 1
    }
    /* Handle reviving if undead */
    if (*p_ptr).necro_extra & 0x8 as libc::c_int as libc::c_uint != 0 &&
           (*p_ptr).necro_extra2 != 0 {
        (*p_ptr).necro_extra2 = (*p_ptr).necro_extra2.wrapping_sub(1);
        if (*p_ptr).necro_extra2 == 0 {
            msg_print(b"Your death has been avenged -- you return to life.\x00"
                          as *const u8 as *const libc::c_char);
            (*p_ptr).necro_extra &= !(0x8 as libc::c_int) as libc::c_uint;
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
        } else {
            msg_format(b"You still have to kill %d monster%s.\x00" as
                           *const u8 as *const libc::c_char,
                       (*p_ptr).necro_extra2,
                       if (*p_ptr).necro_extra2 ==
                              1 as libc::c_int as libc::c_uint {
                           b"\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"s\x00" as *const u8 as *const libc::c_char
                       });
        }
    }
    /* Handle the possibility of player vanquishing arena combatant -KMW- */
    if (*p_ptr).inside_arena != 0 {
        (*p_ptr).exit_bldg = 1 as libc::c_int as bool_;
        msg_print(b"Victorious! You\'re on your way to becoming Champion.\x00"
                      as *const u8 as *const libc::c_char);
        (*p_ptr).arena_number += 1
    }
    /* If the doppleganger die, the variable must be set accordingly */
    if (*r_ptr).flags9 & 0x800 as libc::c_int as libc::c_uint != 0 {
        doppleganger = 0 as libc::c_int as s16b
    }
    /* Drop objects being carried */
    this_o_idx = (*m_ptr).hold_o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
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
        if (*q_ptr).tval as libc::c_int == 100 as libc::c_int {
            dump_gold += 1
        } else { dump_item += 1 }
        /* Drop it */
        drop_near(q_ptr, -(1 as libc::c_int), y, x);
        this_o_idx = next_o_idx
    }
    /* Forget objects */
    (*m_ptr).hold_o_idx = 0 as libc::c_int as s16b;
    /* Average dungeon and monster levels */
    object_level =
        ((dun_level as libc::c_int + (*m_ptr).level as libc::c_int) /
             2 as libc::c_int) as s16b;
    /* Mega^2-hack -- destroying the Stormbringer gives it us! */
    if !strstr(r_name.offset((*r_ptr).name as isize),
               b"Stormbringer\x00" as *const u8 as
                   *const libc::c_char).is_null() {
        /* Get local object */
        q_ptr = &mut forge;
        /* Prepare to make the Stormbringer */
        object_prep(q_ptr,
                    lookup_kind(23 as libc::c_int, 30 as libc::c_int) as
                        libc::c_int);
        /* Mega-Hack -- Name the sword  */
        (*q_ptr).art_name =
            quark_add(b"\'Stormbringer\'\x00" as *const u8 as
                          *const libc::c_char) as
                u16b; /* No longer resist_disen */
        (*q_ptr).to_h = 16 as libc::c_int as s16b;
        (*q_ptr).to_d = 16 as libc::c_int as s16b;
        (*q_ptr).ds = 6 as libc::c_int as byte_hack;
        (*q_ptr).dd = 6 as libc::c_int as byte_hack;
        (*q_ptr).pval = 2 as libc::c_int;
        (*q_ptr).art_flags1 =
            ((*q_ptr).art_flags1 as libc::c_long |
                 (0x8000 as libc::c_long | 0x1 as libc::c_long |
                      0x10 as libc::c_long | 0x2000 as libc::c_long)) as u32b;
        (*q_ptr).art_flags2 =
            ((*q_ptr).art_flags2 as libc::c_long |
                 (0x4000 as libc::c_long | 0x8000 as libc::c_long |
                      0x20000000 as libc::c_long | 0x40000000 as libc::c_long
                      | 0x10000000 as libc::c_long |
                      0x2000000 as libc::c_long)) as u32b;
        (*q_ptr).art_flags3 =
            ((*q_ptr).art_flags3 as libc::c_long |
                 (0x100000 as libc::c_long | 0x200000 as libc::c_long |
                      0x400000 as libc::c_long | 0x800000 as libc::c_long)) as
                u32b;
        /* Just to be sure */
        (*q_ptr).art_flags3 =
            ((*q_ptr).art_flags3 as libc::c_long | 0x10 as libc::c_long) as
                u32b; /* How's that for a downside? */
        /* For game balance... */
        (*q_ptr).art_flags3 =
            ((*q_ptr).art_flags3 as libc::c_long |
                 (0x20000000 as libc::c_long | 0x40000000 as libc::c_long)) as
                u32b;
        (*q_ptr).ident =
            ((*q_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                byte_hack;
        if Rand_div(2 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
            (*q_ptr).art_flags3 =
                ((*q_ptr).art_flags3 as libc::c_long |
                     0x2000000 as libc::c_long) as u32b
        } else {
            (*q_ptr).art_flags3 =
                ((*q_ptr).art_flags3 as libc::c_long |
                     0x8000000 as libc::c_long) as u32b
        }
        (*q_ptr).found = 1 as libc::c_int as byte_hack;
        (*q_ptr).found_aux1 = (*m_ptr).r_idx;
        (*q_ptr).found_aux2 = (*m_ptr).ego as s16b;
        (*q_ptr).found_aux3 = dungeon_type as s16b;
        (*q_ptr).found_aux4 =
            if dungeon_type as libc::c_int == 0 as libc::c_int {
                (*(*wild_map.offset((*p_ptr).wilderness_y as
                                        isize)).offset((*p_ptr).wilderness_x
                                                           as isize)).feat
            } else { dun_level as libc::c_int } as s16b;
        /* Drop it in the dungeon */
        drop_near(q_ptr, -(1 as libc::c_int), y, x);
    } else if !strstr(r_name.offset((*r_ptr).name as isize),
                      b"the Dawn\x00" as *const u8 as
                          *const libc::c_char).is_null() {
        if !(Rand_div(20 as libc::c_int) + 1 as libc::c_int ==
                 13 as libc::c_int) {
            let mut wy: libc::c_int = (*p_ptr).py as libc::c_int;
            let mut wx: libc::c_int = (*p_ptr).px as libc::c_int;
            let mut attempts: libc::c_int = 100 as libc::c_int;
            loop  {
                scatter(&mut wy, &mut wx, (*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, 20 as libc::c_int);
                if !(!(wy > 0 as libc::c_int && wx > 0 as libc::c_int &&
                           wy < cur_hgt as libc::c_int - 1 as libc::c_int &&
                           wx < cur_wid as libc::c_int - 1 as libc::c_int &&
                           ((*f_info.offset((*cave[wy as
                                                       usize].offset(wx as
                                                                         isize)).feat
                                                as isize)).flags1 as
                                libc::c_long & 0x10 as libc::c_long != 0 &&
                                (*cave[wy as usize].offset(wx as isize)).feat
                                    as libc::c_int != 0xaf as libc::c_int)) &&
                         { attempts -= 1; (attempts) != 0 }) {
                    break ;
                }
            }
            if attempts > 0 as libc::c_int {
                if is_friend(m_ptr) > 0 as libc::c_int {
                    if summon_specific_friendly(wy, wx, 100 as libc::c_int,
                                                41 as libc::c_int,
                                                0 as libc::c_int as bool_) !=
                           0 {
                        if (*cave[wy as usize].offset(wx as isize)).info as
                               libc::c_int & 0x10 as libc::c_int !=
                               0 as libc::c_int {
                            msg_print(b"A new warrior steps forth!\x00" as
                                          *const u8 as *const libc::c_char);
                        }
                    }
                } else if summon_specific(wy, wx, 100 as libc::c_int,
                                          41 as libc::c_int) != 0 {
                    if (*cave[wy as usize].offset(wx as isize)).info as
                           libc::c_int & 0x10 as libc::c_int !=
                           0 as libc::c_int {
                        msg_print(b"A new warrior steps forth!\x00" as
                                      *const u8 as *const libc::c_char);
                    }
                }
            }
        }
    } else if !strstr(r_name.offset((*r_ptr).name as isize),
                      b"Unmaker\x00" as *const u8 as
                          *const libc::c_char).is_null() {
        let mut flg: libc::c_int =
            0x10 as libc::c_int | 0x20 as libc::c_int | 0x40 as libc::c_int;
        project(m_idx, 6 as libc::c_int, y, x, 100 as libc::c_int,
                30 as libc::c_int, flg);
    } else if !strstr(r_name.offset((*r_ptr).name as isize),
                      b"ink horror\x00" as *const u8 as
                          *const libc::c_char).is_null() {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            let mut wy_0: libc::c_int = (*p_ptr).py as libc::c_int;
            let mut wx_0: libc::c_int = (*p_ptr).px as libc::c_int;
            let mut attempts_0: libc::c_int = 100 as libc::c_int;
            loop  {
                scatter(&mut wy_0, &mut wx_0, (*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, 3 as libc::c_int);
                if !(!(wy_0 > 0 as libc::c_int && wx_0 > 0 as libc::c_int &&
                           wy_0 < cur_hgt as libc::c_int - 1 as libc::c_int &&
                           wx_0 < cur_wid as libc::c_int - 1 as libc::c_int &&
                           ((*f_info.offset((*cave[wy_0 as
                                                       usize].offset(wx_0 as
                                                                         isize)).feat
                                                as isize)).flags1 as
                                libc::c_long & 0x10 as libc::c_long != 0 &&
                                (*cave[wy_0 as
                                           usize].offset(wx_0 as isize)).feat
                                    as libc::c_int != 0xaf as libc::c_int)) &&
                         { attempts_0 -= 1; (attempts_0) != 0 }) {
                    break ;
                }
            }
            if attempts_0 > 0 as libc::c_int {
                if summon_specific(wy_0, wx_0, 100 as libc::c_int,
                                   50 as libc::c_int) != 0 {
                    if (*cave[wy_0 as usize].offset(wx_0 as isize)).info as
                           libc::c_int & 0x10 as libc::c_int !=
                           0 as libc::c_int {
                        msg_print(b"A blue horror appears!\x00" as *const u8
                                      as *const libc::c_char);
                    }
                }
            }
            i += 1
        }
    } else if (*r_ptr).flags1 & 0x80000000 as libc::c_uint != 0 {
        if !strstr(r_name.offset((*r_ptr).name as isize),
                   b"Morgoth, Lord of Darkness\x00" as *const u8 as
                       *const libc::c_char).is_null() {
            /*
	 * Mega^3-hack: killing a 'Warrior of the Dawn' is likely to
	 * spawn another in the fallen one's place!
	 */
            /* One more ultra-hack: An Unmaker goes out with a big bang! */
            /* Pink horrors are replaced with 2 Blue horrors */
            /* Mega-Hack -- drop "winner" treasures */
            /* Get local object */
            q_ptr = &mut forge;
            /* Mega-Hack -- Prepare to make "Grond" */
            object_prep(q_ptr,
                        lookup_kind(21 as libc::c_int, 50 as libc::c_int) as
                            libc::c_int);
            /* Mega-Hack -- Mark this item as "Grond" */
            (*q_ptr).name1 = 111 as libc::c_int as byte_hack;
            /* Mega-Hack -- Actually create "Grond" */
            apply_magic(q_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                        1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
            /* Drop it in the dungeon */
            drop_near(q_ptr, -(1 as libc::c_int), y, x);
            /* Get local object */
            q_ptr = &mut forge;
            /* Mega-Hack -- Prepare to make "Morgoth" */
            object_prep(q_ptr,
                        lookup_kind(33 as libc::c_int, 50 as libc::c_int) as
                            libc::c_int);
            /* Mega-Hack -- Mark this item as "Morgoth" */
            (*q_ptr).name1 = 34 as libc::c_int as byte_hack;
            /* Mega-Hack -- Actually create "Morgoth" */
            apply_magic(q_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                        1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
            (*q_ptr).found = 1 as libc::c_int as byte_hack;
            (*q_ptr).found_aux1 = (*m_ptr).r_idx;
            (*q_ptr).found_aux2 = (*m_ptr).ego as s16b;
            (*q_ptr).found_aux3 = dungeon_type as s16b;
            (*q_ptr).found_aux4 =
                if dungeon_type as libc::c_int == 0 as libc::c_int {
                    (*(*wild_map.offset((*p_ptr).wilderness_y as
                                            isize)).offset((*p_ptr).wilderness_x
                                                               as isize)).feat
                } else { dun_level as libc::c_int } as s16b;
            /* Drop it in the dungeon */
            drop_near(q_ptr, -(1 as libc::c_int), y, x);
        } else if !strstr(r_name.offset((*r_ptr).name as isize),
                          b"Smeagol\x00" as *const u8 as
                              *const libc::c_char).is_null() {
            /* Get local object */
            q_ptr = &mut forge;
            object_wipe(q_ptr);
            /* Mega-Hack -- Prepare to make a ring of invisibility */
            object_prep(q_ptr,
                        lookup_kind(45 as libc::c_int, 53 as libc::c_int) as
                            libc::c_int);
            (*q_ptr).number = 1 as libc::c_int as byte_hack;
            apply_magic(q_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                        1 as libc::c_int as bool_, 0 as libc::c_int as bool_);
            (*q_ptr).found = 1 as libc::c_int as byte_hack;
            (*q_ptr).found_aux1 = (*m_ptr).r_idx;
            (*q_ptr).found_aux2 = (*m_ptr).ego as s16b;
            (*q_ptr).found_aux3 = dungeon_type as s16b;
            (*q_ptr).found_aux4 =
                if dungeon_type as libc::c_int == 0 as libc::c_int {
                    (*(*wild_map.offset((*p_ptr).wilderness_y as
                                            isize)).offset((*p_ptr).wilderness_x
                                                               as isize)).feat
                } else { dun_level as libc::c_int } as s16b;
            /* Drop it in the dungeon */
            drop_near(q_ptr, -(1 as libc::c_int), y, x);
        } else if (*r_ptr).flags7 & 0x80 as libc::c_int as libc::c_uint != 0 {
            /* Get local object */
            q_ptr = &mut forge;
            object_wipe(q_ptr);
            /* Mega-Hack -- Prepare to make a Ring of Power */
            object_prep(q_ptr,
                        lookup_kind(45 as libc::c_int, 5 as libc::c_int) as
                            libc::c_int);
            (*q_ptr).number = 1 as libc::c_int as byte_hack;
            apply_magic(q_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                        1 as libc::c_int as bool_, 0 as libc::c_int as bool_);
            /* Create a random artifact */
            create_artifact(q_ptr, 1 as libc::c_int as bool_,
                            0 as libc::c_int as bool_);
            /* Save the inscription */
            (*q_ptr).art_name =
                quark_add(format(b"of %s\x00" as *const u8 as
                                     *const libc::c_char,
                                 r_name.offset((*r_ptr).name as isize)) as
                              cptr) as u16b;
            (*q_ptr).found = 1 as libc::c_int as byte_hack;
            (*q_ptr).found_aux1 = (*m_ptr).r_idx;
            (*q_ptr).found_aux2 = (*m_ptr).ego as s16b;
            (*q_ptr).found_aux3 = dungeon_type as s16b;
            (*q_ptr).found_aux4 =
                if dungeon_type as libc::c_int == 0 as libc::c_int {
                    (*(*wild_map.offset((*p_ptr).wilderness_y as
                                            isize)).offset((*p_ptr).wilderness_x
                                                               as isize)).feat
                } else { dun_level as libc::c_int } as s16b;
            /* Drop it in the dungeon */
            drop_near(q_ptr, -(1 as libc::c_int), y, x);
        } else {
            let mut a_idx: byte_hack = 0 as libc::c_int as byte_hack;
            let mut chance: libc::c_int = 0 as libc::c_int;
            let mut I_kind: libc::c_int = 0 as libc::c_int;
            if !strstr(r_name.offset((*r_ptr).name as isize),
                       b"Marda, rider of the Gold Laronth\x00" as *const u8 as
                           *const libc::c_char).is_null() {
                a_idx = 26 as libc::c_int as byte_hack;
                chance = 50 as libc::c_int
            } else if !strstr(r_name.offset((*r_ptr).name as isize),
                              b"Saruman of Many Colours\x00" as *const u8 as
                                  *const libc::c_char).is_null() {
                a_idx = 202 as libc::c_int as byte_hack;
                chance = 30 as libc::c_int
            } else if !strstr(r_name.offset((*r_ptr).name as isize),
                              b"Hagen, son of Alberich\x00" as *const u8 as
                                  *const libc::c_char).is_null() {
                a_idx = 99 as libc::c_int as byte_hack;
                chance = 66 as libc::c_int
            } else if !strstr(r_name.offset((*r_ptr).name as isize),
                              b"Durin\'s Bane\x00" as *const u8 as
                                  *const libc::c_char).is_null() {
                a_idx = 71 as libc::c_int as byte_hack;
                chance = 60 as libc::c_int
            } else if !strstr(r_name.offset((*r_ptr).name as isize),
                              b"Gothmog, the High Captain of Balrogs\x00" as
                                  *const u8 as *const libc::c_char).is_null()
             {
                a_idx = 123 as libc::c_int as byte_hack;
                chance = 50 as libc::c_int
            } else if !strstr(r_name.offset((*r_ptr).name as isize),
                              b"Eol, the Dark Elf\x00" as *const u8 as
                                  *const libc::c_char).is_null() {
                a_idx = 84 as libc::c_int as byte_hack;
                chance = 50 as libc::c_int
            }
            if a_idx as libc::c_int > 0 as libc::c_int &&
                   ((Rand_div(99 as libc::c_int) + 1 as libc::c_int) < chance
                        || wizard as libc::c_int != 0) {
                if (*a_info.offset(a_idx as isize)).cur_num as libc::c_int ==
                       0 as libc::c_int {
                    let mut a_ptr: *mut artifact_type =
                        &mut *a_info.offset(a_idx as isize) as
                            *mut artifact_type;
                    /* Get local object */
                    q_ptr = &mut forge;
                    /* Wipe the object */
                    object_wipe(q_ptr);
                    /* Acquire the "kind" index */
                    I_kind =
                        lookup_kind((*a_ptr).tval as libc::c_int,
                                    (*a_ptr).sval as libc::c_int) as
                            libc::c_int;
                    /* Create the artifact */
                    object_prep(q_ptr, I_kind);
                    /* Save the name */
                    (*q_ptr).name1 = a_idx;
                    /* Extract the fields */
                    (*q_ptr).pval = (*a_ptr).pval as s32b;
                    (*q_ptr).ac = (*a_ptr).ac;
                    (*q_ptr).dd = (*a_ptr).dd;
                    (*q_ptr).ds = (*a_ptr).ds;
                    (*q_ptr).to_a = (*a_ptr).to_a;
                    (*q_ptr).to_h = (*a_ptr).to_h;
                    (*q_ptr).to_d = (*a_ptr).to_d;
                    (*q_ptr).weight = (*a_ptr).weight as s32b;
                    /* Hack -- acquire "cursed" flag */
                    if (*a_ptr).flags3 as libc::c_long &
                           0x20000000 as libc::c_long != 0 {
                        (*q_ptr).ident =
                            ((*q_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack
                    }
                    random_artifact_resistance(q_ptr);
                    (*a_info.offset(a_idx as isize)).cur_num =
                        1 as libc::c_int as byte_hack;
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
                    /* Drop the artifact from heaven */
                    drop_near(q_ptr, -(1 as libc::c_int), y, x);
                }
            }
        }
    } else if (*r_ptr).flags9 & 0x400 as libc::c_int as libc::c_uint != 0 {
        let mut xx: libc::c_int = x;
        let mut yy: libc::c_int = y;
        let mut attempts_1: libc::c_int = 100 as libc::c_int;
        cmsg_print(10 as libc::c_int as byte_hack,
                   b"This monster was under the protection of a Great Wyrm of Power!\x00"
                       as *const u8 as *const libc::c_char);
        loop  {
            scatter(&mut yy, &mut xx, y, x, 6 as libc::c_int);
            if !(!(yy > 0 as libc::c_int && xx > 0 as libc::c_int &&
                       yy < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       xx < cur_wid as libc::c_int - 1 as libc::c_int &&
                       ((*f_info.offset((*cave[yy as
                                                   usize].offset(xx as
                                                                     isize)).feat
                                            as isize)).flags1 as libc::c_long
                            & 0x10 as libc::c_long != 0 &&
                            (*cave[yy as usize].offset(xx as isize)).feat as
                                libc::c_int != 0xaf as libc::c_int)) &&
                     { attempts_1 -= 1; (attempts_1) != 0 }) {
                break ;
            }
        }
        place_monster_aux(yy, xx,
                          test_monster_name(b"Great Wyrm of Power\x00" as
                                                *const u8 as
                                                *const libc::c_char),
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_,
                          (*m_ptr).status as libc::c_int);
    }
    /* Hack - the protected monsters must be advanged */
    /* Let monsters explode! */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*m_ptr).blow[i as usize].method as libc::c_int ==
               16 as libc::c_int {
            let mut flg_0: libc::c_int =
                0x10 as libc::c_int | 0x20 as libc::c_int |
                    0x40 as libc::c_int;
            let mut typ: libc::c_int = 10 as libc::c_int;
            let mut d_dice: libc::c_int =
                (*m_ptr).blow[i as usize].d_dice as libc::c_int;
            let mut d_side: libc::c_int =
                (*m_ptr).blow[i as usize].d_side as libc::c_int;
            let mut damage: libc::c_int =
                damroll(d_dice as s16b, d_side as s16b);
            match (*m_ptr).blow[i as usize].effect as libc::c_int {
                1 => { typ = 10 as libc::c_int }
                2 => { typ = 2 as libc::c_int }
                3 => { typ = 32 as libc::c_int }
                4 => { typ = 10 as libc::c_int }
                5 => { typ = 10 as libc::c_int }
                6 => { typ = 10 as libc::c_int }
                7 => { typ = 10 as libc::c_int }
                8 => { typ = 10 as libc::c_int }
                9 => { typ = 3 as libc::c_int }
                10 => { typ = 1 as libc::c_int }
                11 => { typ = 5 as libc::c_int }
                12 => { typ = 4 as libc::c_int }
                13 => { typ = 10 as libc::c_int }
                32 => { typ = 22 as libc::c_int }
                14 => { typ = 22 as libc::c_int }
                15 => { typ = 10 as libc::c_int }
                16 => { typ = 10 as libc::c_int }
                17 => { typ = 10 as libc::c_int }
                20 => { typ = 10 as libc::c_int }
                21 => { typ = 10 as libc::c_int }
                18 => { typ = 10 as libc::c_int }
                19 => { typ = 10 as libc::c_int }
                22 => { typ = 10 as libc::c_int }
                23 => { typ = 10 as libc::c_int }
                33 => { typ = 10 as libc::c_int }
                24 => { typ = 72 as libc::c_int }
                25 => { typ = 10 as libc::c_int }
                26 => { typ = 10 as libc::c_int }
                27 => { typ = 10 as libc::c_int }
                28 => { typ = 10 as libc::c_int }
                29 => { typ = 2 as libc::c_int }
                30 => { typ = 34 as libc::c_int }
                31 => { typ = 10 as libc::c_int }
                _ => { }
            }
            project(m_idx, 3 as libc::c_int, y, x, damage, typ, flg_0);
            break ;
        } else { i += 1 }
    }
    if force_coin == 0 &&
           Rand_div(100 as libc::c_int) <
               10 as libc::c_int +
                   get_skill_scale(49 as libc::c_int,
                                   75 as libc::c_int as u32b) as libc::c_int
           && (*m_ptr).mflag & 0x100 as libc::c_int == 0 {
        place_corpse(m_ptr);
    }
    /* Take note of any dropped treasure */
    if visible as libc::c_int != 0 && (dump_item != 0 || dump_gold != 0) {
        /* Take notes on treasure */
        lore_treasure(m_idx, dump_item, dump_gold);
    }
    /* Current quest */
    i = (*p_ptr).inside_quest as libc::c_int;
    /* Create a magical staircase */
    if create_stairs as libc::c_int != 0 &&
           (dun_level as libc::c_int) <
               (*d_info.offset(dungeon_type as isize)).maxdepth as libc::c_int
       {
        let mut i_0: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        i_0 = -(1 as libc::c_int);
        while i_0 <= 1 as libc::c_int {
            j = -(1 as libc::c_int);
            while j <= 1 as libc::c_int {
                if (*f_info.offset((*cave[(y + j) as
                                              usize].offset((x + i_0) as
                                                                isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x40 as libc::c_long == 0 {
                    cave_set_feat(y + j, x + i_0,
                                  (*d_info.offset(dungeon_type as
                                                      isize)).floor1 as
                                      libc::c_int);
                }
                j += 1
            }
            i_0 += 1
        }
        /* Stagger around */
        while cave_valid_bold(y, x) == 0 {
            let mut d: libc::c_int = 1 as libc::c_int;
            /* Pick a location */
            scatter(&mut ny, &mut nx, y, x, d);
            /* Stagger */
            y = ny;
            x = nx
        }
        /* Destroy any objects */
        delete_object(y, x);
        /* Explain the staircase */
        msg_print(b"A magical staircase appears...\x00" as *const u8 as
                      *const libc::c_char);
        /* Create stairs down */
        cave_set_feat(y, x, 0x7 as libc::c_int);
        /* Remember to update everything */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                      0x1000000 as libc::c_long)) as u32b
    };
}
/*
 * Decreases monsters hit points, handling monster death.
 *
 * We return TRUE if the monster has been killed (and deleted).
 *
 * We announce monster death (using an optional "death message"
 * if given, and a otherwise a generic killed/destroyed message).
 *
 * Only "physical attacks" can induce the "You have slain" message.
 * Missile and Spell attacks will induce the "dies" message, or
 * various "specialized" messages.  Note that "You have destroyed"
 * and "is destroyed" are synonyms for "You have slain" and "dies".
 *
 * Hack -- unseen monsters yield "You have killed it." message.
 *
 * Added fear (DGK) and check whether to print fear messages -CWS
 *
 * Genericized name, sex, and capitilization -BEN-
 *
 * Hack -- we "delay" fear messages by passing around a "fear" flag.
 *
 * XXX XXX XXX Consider decreasing monster experience over time, say,
 * by using "(m_exp * m_lev * (m_lev)) / (p_lev * (m_lev + n_killed))"
 * instead of simply "(m_exp * m_lev) / (p_lev)", to make the first
 * monster worth more than subsequent monsters.  This would also need
 * to induce changes in the monster recall code.
 */
#[no_mangle]
pub unsafe extern "C" fn mon_take_hit(mut m_idx: libc::c_int,
                                      mut dam: libc::c_int,
                                      mut fear: *mut bool_, mut note: cptr)
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
    /* It is dead now */
    if (*m_ptr).hp < 0 as libc::c_int {
        let mut m_name: [libc::c_char; 80] = [0; 80];
        /* Lets face it, you cannot get rid of a possessor that easily */
        if (*m_ptr).possessor != 0 {
            ai_deincarnate(m_idx);
            return 0 as libc::c_int as bool_
        }
        /* Extract monster name */
        monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
        if (*r_ptr).flags7 & 0x100 as libc::c_int as libc::c_uint != 0 &&
               Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
            let mut curses: libc::c_int =
                2 as libc::c_int +
                    (Rand_div(5 as libc::c_int) + 1 as libc::c_int);
            cmsg_format(10 as libc::c_int as byte_hack,
                        b"%^s puts a terrible Morgothian curse on you!\x00" as
                            *const u8 as *const libc::c_char,
                        m_name.as_mut_ptr());
            curse_equipment_dg(100 as libc::c_int, 50 as libc::c_int);
            loop  {
                activate_dg_curse();
                curses -= 1;
                if !(curses != 0) { break ; }
            }
        }
        if speak_unique as libc::c_int != 0 &&
               (*r_ptr).flags2 & 0x4 as libc::c_int as libc::c_uint != 0 {
            let mut line_got: [libc::c_char; 80] = [0; 80];
            /* Dump a message */
            get_rnd_line(b"mondeath.txt\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         line_got.as_mut_ptr());
            msg_format(b"%^s says: %s\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr(),
                       line_got.as_mut_ptr());
        }
        /* Make a sound */
        sound(5 as libc::c_int);
        /* Death by Missile/Spell attack */
        if !note.is_null() {
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"%^s%s\x00" as *const u8 as *const libc::c_char,
                        m_name.as_mut_ptr(), note);
        } else if (*m_ptr).ml == 0 {
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"You have killed %s.\x00" as *const u8 as
                            *const libc::c_char, m_name.as_mut_ptr());
        } else if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
                      ||
                      (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint !=
                          0 ||
                      (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint !=
                          0 ||
                      (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint
                          != 0 ||
                      !strchr(b"Evg\x00" as *const u8 as *const libc::c_char,
                              (*r_ptr).d_char as libc::c_int).is_null() {
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"You have destroyed %s.\x00" as *const u8 as
                            *const libc::c_char, m_name.as_mut_ptr());
        } else {
            /* Death by physical attack -- invisible monster */
            /* Death by Physical attack -- non-living monster */
            /* Death by Physical attack -- living monster */
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"You have slain %s.\x00" as *const u8 as
                            *const libc::c_char, m_name.as_mut_ptr());
        }
        /* Maximum player level */
        div = (*p_ptr).max_plv as s32b;
        if ((*m_ptr).status as libc::c_int) < 2 as libc::c_int {
            /* Give some experience for the kill */
            new_exp =
                ((*r_ptr).mexp as libc::c_long *
                     (*m_ptr).level as libc::c_long / div as libc::c_long) as
                    s32b;
            /* Handle fractional experience */
            new_exp_frac =
                ((*r_ptr).mexp as libc::c_long *
                     (*m_ptr).level as libc::c_long % div as libc::c_long *
                     0x10000 as libc::c_long / div as libc::c_long +
                     (*p_ptr).exp_frac as libc::c_long) as s32b;
            /* Keep track of experience */
            if new_exp_frac as libc::c_long >= 0x10000 as libc::c_long {
                new_exp += 1;
                (*p_ptr).exp_frac =
                    (new_exp_frac as libc::c_long - 0x10000 as libc::c_long)
                        as u16b
            } else { (*p_ptr).exp_frac = new_exp_frac as u16b }
            /* Gain experience */
            gain_exp(new_exp);
        }
        if note.is_null() {
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
            let mut f1: u32b = 0;
            let mut f2: u32b = 0;
            let mut f3: u32b = 0;
            let mut f4: u32b = 0;
            let mut f5: u32b = 0;
            let mut esp: u32b = 0;
            /* Access the weapon */
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int
                                                                 as isize) as
                    *mut object_type;
            object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                         &mut esp);
            /* Can the weapon gain levels ? */
            if (*o_ptr).k_idx as libc::c_int != 0 &&
                   f4 as libc::c_long & 0x100 as libc::c_long != 0 {
                /* Give some experience for the kill */
                new_exp =
                    ((*r_ptr).mexp as libc::c_long *
                         (*m_ptr).level as libc::c_long /
                         (div * 2 as libc::c_int) as libc::c_long) as s32b;
                /* Gain experience */
                (*o_ptr).exp += new_exp;
                check_experience_obj(o_ptr);
            }
        }
        /* When the player kills a Unique, it stays dead */
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            (*r_ptr).max_num = 0 as libc::c_int as s16b
        }
        /* Generate treasure */
        monster_death(m_idx);
        /* Eru doesn't appreciate good monster death */
        if (*r_ptr).flags3 & 0x200 as libc::c_int as libc::c_uint != 0 {
            inc_piety(1 as libc::c_int,
                      -(7 as libc::c_int) * (*m_ptr).level as libc::c_int);
            inc_piety(2 as libc::c_int,
                      -(10 as libc::c_int) * (*m_ptr).level as libc::c_int);
            inc_piety(4 as libc::c_int,
                      3 as libc::c_int * (*m_ptr).level as libc::c_int);
        } else {
            inc_piety(4 as libc::c_int,
                      1 as libc::c_int +
                          (*m_ptr).level as libc::c_int / 2 as libc::c_int);
        }
        /* Manwe appreciate evil monster death */
        if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let mut inc: libc::c_int =
                (*m_ptr).level as libc::c_int / 2 as libc::c_int;
            if inc == 0 { inc = 1 as libc::c_int }
            if (*p_ptr).pgod as libc::c_int == 2 as libc::c_int &&
                   (*p_ptr).praying as libc::c_int != 0 {
                inc_piety(2 as libc::c_int, inc);
            }
            if inc < 2 as libc::c_int { inc = 2 as libc::c_int }
            inc_piety(3 as libc::c_int, inc / 2 as libc::c_int);
            if (*p_ptr).pgod as libc::c_int == 3 as libc::c_int &&
                   (*p_ptr).praying as libc::c_int != 0 {
                inc_piety(3 as libc::c_int, inc / 2 as libc::c_int);
                if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
                   {
                    inc_piety(3 as libc::c_int, inc);
                }
            }
        }
        /* Yavanna likes when corruption is destroyed */
        if (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0 ||
               (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 ||
               (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
            let mut inc_0: libc::c_int =
                (*m_ptr).level as libc::c_int / 2 as libc::c_int;
            if inc_0 == 0 { inc_0 = 1 as libc::c_int }
            inc_piety(5 as libc::c_int, inc_0);
        }
        /* Yavanna doesnt like any killing in her name */
        if (*p_ptr).pgod as libc::c_int == 5 as libc::c_int &&
               (*p_ptr).praying as libc::c_int != 0 {
            let mut inc_1: libc::c_int =
                (*m_ptr).level as libc::c_int / 2 as libc::c_int;
            if inc_1 == 0 { inc_1 = 1 as libc::c_int }
            inc_piety(5 as libc::c_int, -inc_1);
            /* Killing animals in her name is a VERY bad idea */
            if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 {
                inc_piety(5 as libc::c_int, -(inc_1 * 3 as libc::c_int));
            }
        }
        /* SHould we absorb its soul? */
        if (*p_ptr).absorb_soul as libc::c_int != 0 &&
               (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint == 0 &&
               (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint == 0 {
            msg_print(b"You absorb the life of the dying soul.\x00" as
                          *const u8 as *const libc::c_char);
            hp_player(1 as libc::c_int +
                          (*m_ptr).level as libc::c_int / 2 as libc::c_int +
                          get_skill_scale(31 as libc::c_int,
                                          40 as libc::c_int as u32b) as
                              libc::c_int);
        }
        /*
		* XXX XXX XXX Mega-Hack -- Remove random quest rendered
		* impossible
		*/
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let mut i: libc::c_int = 0;
            /* Search for all the random quests */
            i = 0 as libc::c_int;
            while i < 99 as libc::c_int {
                let mut q_ptr: *mut random_quest =
                    &mut *random_quests.as_mut_ptr().offset(i as isize) as
                        *mut random_quest;
                /* Ignore invalid entries */
                if !((*q_ptr).type_0 as libc::c_int == 0 as libc::c_int) {
                    /* It's done */
                    if !((*q_ptr).done != 0) {
                        /*
				 * XXX XXX XXX Not yet completed quest is
				 * to kill a unique you've just killed
				 */
                        if r_ptr ==
                               &mut *r_info.offset((*q_ptr).r_idx as isize) as
                                   *mut monster_race {
                            /* Invalidate it */
                            (*q_ptr).type_0 = 0 as libc::c_int as byte_hack
                        }
                    }
                }
                i += 1
            }
        }
        /* If the player kills a Unique, and the notes options are on, write a note */
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
               take_notes as libc::c_int != 0 &&
               auto_notes as libc::c_int != 0 {
            let mut note_0: [libc::c_char; 80] = [0; 80];
            /* Get true name even if blinded/hallucinating */
            let mut monst: cptr =
                r_name.offset((*r_ptr).name as isize) as cptr;
            /* Write note */
            sprintf(note_0.as_mut_ptr(),
                    b"Killed %s\x00" as *const u8 as *const libc::c_char,
                    monst);
            add_note(note_0.as_mut_ptr(), 'U' as i32 as libc::c_char);
        }
        /* Recall even invisible uniques or winners */
        if (*m_ptr).ml as libc::c_int != 0 ||
               (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            /* Count kills this life */
            if ((*r_ptr).r_pkills as libc::c_int) < 32767 as libc::c_int {
                (*r_ptr).r_pkills += 1
            }
            /* Count kills in all lives */
            if ((*r_ptr).r_tkills as libc::c_int) < 32767 as libc::c_int {
                (*r_ptr).r_tkills += 1
            }
            /* Hack -- Auto-recall */
            monster_race_track((*m_ptr).r_idx as libc::c_int,
                               (*m_ptr).ego as libc::c_int);
        }
        /* Delete the monster */
        delete_monster_idx(m_idx);
        /* Not afraid */
        *fear = 0 as libc::c_int as bool_;
        /* Monster is dead */
        return 1 as libc::c_int as bool_
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
    /* Not dead yet */
    return 0 as libc::c_int as bool_;
}
/*
 * Get term size and calculate screen size
 */
#[no_mangle]
pub unsafe extern "C" fn get_screen_size(mut wid_p: *mut libc::c_int,
                                         mut hgt_p: *mut libc::c_int) {
    Term_get_size(wid_p, hgt_p);
    *hgt_p -= 1 as libc::c_int + 1 as libc::c_int;
    *wid_p -= 13 as libc::c_int + 1 as libc::c_int;
    if use_bigtile != 0 { *wid_p /= 2 as libc::c_int };
}
/*
 * Calculates current boundaries
 * Called below.
 */
unsafe extern "C" fn panel_bounds() {
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    /* Retrieve current screen size */
    get_screen_size(&mut wid, &mut hgt);
    /* + 24 - 1 - 2 =  + 21 */
    panel_row_max =
        (panel_row_min as libc::c_int + hgt - 1 as libc::c_int) as s16b;
    panel_row_prt = (panel_row_min as libc::c_int - 1 as libc::c_int) as s16b;
    /* Paranoia -- Boundary check */
    if panel_row_max as libc::c_int >
           cur_hgt as libc::c_int - 1 as libc::c_int {
        panel_row_max = (cur_hgt as libc::c_int - 1 as libc::c_int) as s16b
    }
    panel_col_max =
        (panel_col_min as libc::c_int + wid - 1 as libc::c_int) as s16b;
    panel_col_prt =
        (panel_col_min as libc::c_int - 13 as libc::c_int) as s16b;
    /* Paranoia -- Boundary check */
    if panel_col_max as libc::c_int >
           cur_wid as libc::c_int - 1 as libc::c_int {
        panel_col_max = (cur_wid as libc::c_int - 1 as libc::c_int) as s16b
    };
}
/*
 * Handle a request to change the current panel
 *
 * Return TRUE if the panel was changed.
 *
 * Also used in do_cmd_locate()
 */
#[no_mangle]
pub unsafe extern "C" fn change_panel(mut dy: libc::c_int,
                                      mut dx: libc::c_int) -> bool_ {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    /* Get size */
    get_screen_size(&mut wid, &mut hgt);
    /* Apply the motion */
    y = panel_row_min as libc::c_int + dy * (hgt / 2 as libc::c_int);
    x = panel_col_min as libc::c_int + dx * (wid / 2 as libc::c_int);
    /* Calculate bounds */
    if y > cur_hgt as libc::c_int - hgt { y = cur_hgt as libc::c_int - hgt }
    if y < 0 as libc::c_int { y = 0 as libc::c_int }
    if x > cur_wid as libc::c_int - wid { x = cur_wid as libc::c_int - wid }
    if x < 0 as libc::c_int { x = 0 as libc::c_int }
    /* Handle changes */
    if y != panel_row_min as libc::c_int || x != panel_col_min as libc::c_int
       {
        /* Save the new panel info */
        panel_row_min = y as s16b;
        panel_col_min = x as s16b;
        /* Recalculate the boundaries */
        panel_bounds();
        /* Update stuff */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as
                u32b;
        /* Redraw map */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as
                u32b;
        /* Handle stuff */
        handle_stuff();
        /* Success */
        return 1 as libc::c_int as bool_
    }
    /* No changes */
    return 0 as libc::c_int as bool_;
}
/*
 * Given an row (y) and col (x), this routine detects when a move
 * off the screen has occurred and figures new borders. -RAK-
 *
 * "Update" forces a "full update" to take place.
 *
 * The map is reprinted if necessary, and "TRUE" is returned.
 */
#[no_mangle]
pub unsafe extern "C" fn verify_panel() {
    let mut y: libc::c_int = (*p_ptr).py as libc::c_int;
    let mut x: libc::c_int = (*p_ptr).px as libc::c_int;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut prow_min: libc::c_int = 0;
    let mut pcol_min: libc::c_int = 0;
    let mut panel_hgt: libc::c_int = 0;
    let mut panel_wid: libc::c_int = 0;
    let mut max_prow_min: libc::c_int = 0;
    let mut max_pcol_min: libc::c_int = 0;
    /*
	 * Make sure panel_row/col_max have correct values -- now taken care of
	 * by the hook function below, which eliminates glitches, but does it
	 * in a very hackish way XXX XXX XXX
	 */
	/* panel_bounds(); */
    /* Get size */
    get_screen_size(&mut wid, &mut hgt);
    /* Calculate scroll amount */
    panel_hgt = hgt / 2 as libc::c_int;
    panel_wid = wid / 2 as libc::c_int;
    /* Upper boundary of panel_row/col_min */
    max_prow_min = cur_hgt as libc::c_int - hgt;
    max_pcol_min = cur_wid as libc::c_int - wid;
    /* Boundary check */
    if max_prow_min < 0 as libc::c_int { max_prow_min = 0 as libc::c_int }
    if max_pcol_min < 0 as libc::c_int { max_pcol_min = 0 as libc::c_int }
    /* An option: center on player */
    if center_player != 0 {
        /* Center vertically */
        prow_min = y - panel_hgt;
        /* Boundary check */
        if prow_min < 0 as libc::c_int {
            prow_min = 0 as libc::c_int
        } else if prow_min > max_prow_min { prow_min = max_prow_min }
        /* Center horizontally */
        pcol_min = x - panel_wid;
        /* Boundary check */
        if pcol_min < 0 as libc::c_int {
            pcol_min = 0 as libc::c_int
        } else if pcol_min > max_pcol_min { pcol_min = max_pcol_min }
    } else {
        /* No centering */
        prow_min = panel_row_min as libc::c_int;
        pcol_min = panel_col_min as libc::c_int;
        /* Scroll screen when 2 grids from top/bottom edge */
        if y > panel_row_max as libc::c_int - 2 as libc::c_int {
            while y > prow_min + hgt - 2 as libc::c_int {
                prow_min += panel_hgt
            }
            if prow_min > max_prow_min { prow_min = max_prow_min }
        }
        if y < panel_row_min as libc::c_int + 2 as libc::c_int {
            while y < prow_min + 2 as libc::c_int { prow_min -= panel_hgt }
            if prow_min < 0 as libc::c_int { prow_min = 0 as libc::c_int }
        }
        /* Scroll screen when 4 grids from left/right edge */
        if x > panel_col_max as libc::c_int - 4 as libc::c_int {
            while x > pcol_min + wid - 4 as libc::c_int {
                pcol_min += panel_wid
            }
            if pcol_min > max_pcol_min { pcol_min = max_pcol_min }
        }
        if x < panel_col_min as libc::c_int + 4 as libc::c_int {
            while x < pcol_min + 4 as libc::c_int { pcol_min -= panel_wid }
            if pcol_min < 0 as libc::c_int { pcol_min = 0 as libc::c_int }
        }
    }
    /* Check for "no change" */
    if prow_min == panel_row_min as libc::c_int &&
           pcol_min == panel_col_min as libc::c_int {
        return
    }
    /* Save the new panel info */
    panel_row_min = prow_min as s16b;
    panel_col_min = pcol_min as s16b;
    /* Hack -- optional disturb on "panel change" */
    if disturb_panel as libc::c_int != 0 && center_player == 0 {
        disturb(0 as libc::c_int, 0 as libc::c_int);
    }
    /* Recalculate the boundaries */
    panel_bounds();
    /* Update stuff */
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
 * Map resizing whenever the main term changes size
 */
#[no_mangle]
pub unsafe extern "C" fn resize_map() {
    /* Only if the dungeon exists */
    if character_dungeon == 0 { return }
    /* Mega-Hack -- No panel yet, assume illegal panel */
    panel_row_min = cur_hgt;
    panel_row_max = 0 as libc::c_int as s16b;
    panel_col_min = cur_wid;
    panel_col_max = 0 as libc::c_int as s16b;
    /* Select player panel */
    verify_panel();
    /*
	 * The following should be the same as the main window code
	 * in the do_cmd_redraw()
	 */
    /* Combine and reorder the pack (later) */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* Update torch */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as u32b;
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x1 as libc::c_long | 0x10 as libc::c_long |
                  0x20 as libc::c_long | 0x40 as libc::c_long |
                  0x80 as libc::c_long | 0x8 as libc::c_long |
                  0x4 as libc::c_long)) as u32b;
    /* Forget and update view */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x10000 as libc::c_long | 0x100000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x200000 as libc::c_long)) as
            u32b;
    /* Redraw everything */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x8000000 as libc::c_long | 0x2000000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x4000000 as libc::c_long)) as
            u32b;
    /* Hack -- update */
    handle_stuff();
    /* Redraw */
    Term_redraw();
    /* Refresh */
    Term_fresh();
}
/*
 * Redraw a term when it is resized
 */
#[no_mangle]
pub unsafe extern "C" fn resize_window() {
    /* Only if the dungeon exists */
    if character_dungeon == 0 { return }
    /* Hack -- Activate the Angband window for the redraw */
    Term_activate(&mut *(*angband_term.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize)).offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
    /* Hack -- react to changes */
    Term_xtra(10 as libc::c_int, 0 as libc::c_int);
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x10 as libc::c_long | 0x40 as libc::c_long |
                  0x80 as libc::c_long | 0x100 as libc::c_long |
                  0x200 as libc::c_long)) as u32b;
    /* Hack -- update */
    handle_stuff();
    /* Refresh */
    Term_fresh();
}
/*
 * Monster health description
 */
#[no_mangle]
pub unsafe extern "C" fn look_mon_desc(mut m_idx: libc::c_int) -> cptr {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut living: bool_ = 1 as libc::c_int as bool_;
    let mut perc: libc::c_int = 0;
    /* Determine if the monster is "living" (vs "undead") */
    if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
        living = 0 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 {
        living = 0 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0 {
        living = 0 as libc::c_int as bool_
    }
    if !strchr(b"Egv\x00" as *const u8 as *const libc::c_char,
               (*r_ptr).d_char as libc::c_int).is_null() {
        living = 0 as libc::c_int as bool_
    }
    /* Healthy monsters */
    if (*m_ptr).hp >= (*m_ptr).maxhp {
        /* No damage */
        return if living as libc::c_int != 0 {
                   b"unhurt\x00" as *const u8 as *const libc::c_char
               } else { b"undamaged\x00" as *const u8 as *const libc::c_char }
    }
    /* Calculate a health "percentage" */
    perc =
        (100 as libc::c_long * (*m_ptr).hp as libc::c_long /
             (*m_ptr).maxhp as libc::c_long) as libc::c_int;
    if perc >= 60 as libc::c_int {
        return if living as libc::c_int != 0 {
                   b"somewhat wounded\x00" as *const u8 as *const libc::c_char
               } else {
                   b"somewhat damaged\x00" as *const u8 as *const libc::c_char
               }
    }
    if perc >= 25 as libc::c_int {
        return if living as libc::c_int != 0 {
                   b"wounded\x00" as *const u8 as *const libc::c_char
               } else { b"damaged\x00" as *const u8 as *const libc::c_char }
    }
    if perc >= 10 as libc::c_int {
        return if living as libc::c_int != 0 {
                   b"badly wounded\x00" as *const u8 as *const libc::c_char
               } else {
                   b"badly damaged\x00" as *const u8 as *const libc::c_char
               }
    }
    return if living as libc::c_int != 0 {
               b"almost dead\x00" as *const u8 as *const libc::c_char
           } else {
               b"almost destroyed\x00" as *const u8 as *const libc::c_char
           };
}
/*
 * Angband sorting algorithm -- quick sort in place
 *
 * Note that the details of the data we are sorting is hidden,
 * and we rely on the "ang_sort_comp()" and "ang_sort_swap()"
 * function hooks to interact with the data, which is given as
 * two pointers, and which may have any user-defined form.
 */
#[no_mangle]
pub unsafe extern "C" fn ang_sort_aux(mut u: vptr, mut v: vptr,
                                      mut p: libc::c_int,
                                      mut q: libc::c_int) {
    let mut z: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    /* Done sort */
    if p >= q { return }
    /* Pivot */
    z = p;
    /* Begin */
    a = p;
    b = q;
    loop 
         /* Partition */
         /* Slide i2 */
         {
        while Some(ang_sort_comp.expect("non-null function pointer")).expect("non-null function pointer")(u,
                                                                                                          v,
                                                                                                          b,
                                                                                                          z)
                  == 0 {
            b -= 1
        }
        /* Slide i1 */
        while Some(ang_sort_comp.expect("non-null function pointer")).expect("non-null function pointer")(u,
                                                                                                          v,
                                                                                                          z,
                                                                                                          a)
                  == 0 {
            a += 1
        }
        /* Done partition */
        if a >= b { break ; }
        /* Swap */
        Some(ang_sort_swap.expect("non-null function pointer")).expect("non-null function pointer")(u,
                                                                                                    v,
                                                                                                    a,
                                                                                                    b);
        /* Advance */
        a += 1;
        b -= 1
    }
    /* Recurse left side */
    ang_sort_aux(u, v, p, b);
    /* Recurse right side */
    ang_sort_aux(u, v, b + 1 as libc::c_int, q);
}
/*
 * Angband sorting algorithm -- quick sort in place
 *
 * Note that the details of the data we are sorting is hidden,
 * and we rely on the "ang_sort_comp()" and "ang_sort_swap()"
 * function hooks to interact with the data, which is given as
 * two pointers, and which may have any user-defined form.
 */
#[no_mangle]
pub unsafe extern "C" fn ang_sort(mut u: vptr, mut v: vptr,
                                  mut n: libc::c_int) {
    /* Sort the array */
    ang_sort_aux(u, v, 0 as libc::c_int, n - 1 as libc::c_int);
}
/* ** Targetting Code ***/
/*
 * Determine is a monster makes a reasonable target
 *
 * The concept of "targetting" was stolen from "Morgul" (?)
 *
 * The player can target any location, or any "target-able" monster.
 *
 * Currently, a monster is "target_able" if it is visible, and if
 * the player can hit it with a projection, and the player is not
 * hallucinating.  This allows use of "use closest target" macros.
 *
 * Future versions may restrict the ability to target "trappers"
 * and "mimics", but the semantics is a little bit weird.
 */
#[no_mangle]
pub unsafe extern "C" fn target_able(mut m_idx: libc::c_int) -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    /* Monster must be alive */
    if (*m_ptr).r_idx == 0 { return 0 as libc::c_int as bool_ }
    /* Monster must be visible */
    if (*m_ptr).ml == 0 { return 0 as libc::c_int as bool_ }
    /* Monster must be projectable */
    if projectable((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                   (*m_ptr).fy as libc::c_int, (*m_ptr).fx as libc::c_int) ==
           0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- no targeting hallucinations */
    if (*p_ptr).image != 0 { return 0 as libc::c_int as bool_ }
    /* Dont target pets */
    if is_friend(m_ptr) > 0 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Honor flag */
    if (*r_info.offset((*m_ptr).r_idx as isize)).flags7 &
           0x800 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* XXX XXX XXX Hack -- Never target trappers */
	/* if (CLEAR_ATTR && (CLEAR_CHAR)) return (FALSE); */
    /* Assume okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Update (if necessary) and verify (if possible) the target.
 *
 * We return TRUE if the target is "okay" and FALSE otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn target_okay() -> bool_ {
    /* Accept stationary targets */
    if (target_who as libc::c_int) < 0 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Check moving targets */
    if target_who as libc::c_int > 0 as libc::c_int {
        /* Accept reasonable targets */
        if target_able(target_who as libc::c_int) != 0 {
            let mut m_ptr: *mut monster_type =
                &mut *m_list.offset(target_who as isize) as *mut monster_type;
            /* Acquire monster location */
            target_row = (*m_ptr).fy as s16b;
            target_col = (*m_ptr).fx as s16b;
            /* Good target */
            return 1 as libc::c_int as bool_
        }
    }
    /* Assume no target */
    return 0 as libc::c_int as bool_;
}
/*
 * Sorting hook -- comp function -- by "distance to player"
 *
 * We use "u" and "v" to point to arrays of "x" and "y" positions,
 * and sort the arrays by double-distance to the player.
 */
unsafe extern "C" fn ang_sort_comp_distance(mut u: vptr, mut v: vptr,
                                            mut a: libc::c_int,
                                            mut b: libc::c_int) -> bool_ {
    let mut x: *mut byte_hack = u as *mut byte_hack;
    let mut y: *mut byte_hack = v as *mut byte_hack;
    let mut da: libc::c_int = 0;
    let mut db: libc::c_int = 0;
    let mut kx: libc::c_int = 0;
    let mut ky: libc::c_int = 0;
    /* Absolute distance components */
    kx = *x.offset(a as isize) as libc::c_int;
    kx -= (*p_ptr).px as libc::c_int;
    kx = if kx < 0 as libc::c_int { -kx } else { kx };
    ky = *y.offset(a as isize) as libc::c_int;
    ky -= (*p_ptr).py as libc::c_int;
    ky = if ky < 0 as libc::c_int { -ky } else { ky };
    /* Approximate Double Distance to the first point */
    da = if kx > ky { (kx + kx) + ky } else { (ky + ky) + kx };
    /* Absolute distance components */
    kx = *x.offset(b as isize) as libc::c_int;
    kx -= (*p_ptr).px as libc::c_int;
    kx = if kx < 0 as libc::c_int { -kx } else { kx };
    ky = *y.offset(b as isize) as libc::c_int;
    ky -= (*p_ptr).py as libc::c_int;
    ky = if ky < 0 as libc::c_int { -ky } else { ky };
    /* Approximate Double Distance to the first point */
    db = if kx > ky { (kx + kx) + ky } else { (ky + ky) + kx };
    /* Compare the distances */
    return (da <= db) as libc::c_int as bool_;
}
/*
 * Sorting hook -- swap function -- by "distance to player"
 *
 * We use "u" and "v" to point to arrays of "x" and "y" positions,
 * and sort the arrays by distance to the player.
 */
unsafe extern "C" fn ang_sort_swap_distance(mut u: vptr, mut v: vptr,
                                            mut a: libc::c_int,
                                            mut b: libc::c_int) {
    let mut x: *mut byte_hack = u as *mut byte_hack;
    let mut y: *mut byte_hack = v as *mut byte_hack;
    let mut temp: byte_hack = 0;
    /* Swap "x" */
    temp = *x.offset(a as isize);
    *x.offset(a as isize) = *x.offset(b as isize);
    *x.offset(b as isize) = temp;
    /* Swap "y" */
    temp = *y.offset(a as isize);
    *y.offset(a as isize) = *y.offset(b as isize);
    *y.offset(b as isize) = temp;
}
/*
 * Hack -- help "select" a location (see below)
 */
unsafe extern "C" fn target_pick(mut y1: libc::c_int, mut x1: libc::c_int,
                                 mut dy: libc::c_int, mut dx: libc::c_int)
 -> s16b {
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x3: libc::c_int = 0;
    let mut y3: libc::c_int = 0;
    let mut x4: libc::c_int = 0;
    let mut y4: libc::c_int = 0;
    let mut b_i: libc::c_int = -(1 as libc::c_int);
    let mut b_v: libc::c_int = 9999 as libc::c_int;
    /* Scan the locations */
    i = 0 as libc::c_int;
    while i < temp_n as libc::c_int {
        /* Point 2 */
        x2 = temp_x[i as usize] as libc::c_int;
        y2 = temp_y[i as usize] as libc::c_int;
        /* Directed distance */
        x3 = x2 - x1;
        y3 = y2 - y1;
        /* Verify quadrant */
        if !(dx != 0 && x3 * dx <= 0 as libc::c_int) {
            if !(dy != 0 && y3 * dy <= 0 as libc::c_int) {
                /* Absolute distance */
                x4 = if x3 < 0 as libc::c_int { -x3 } else { x3 };
                y4 = if y3 < 0 as libc::c_int { -y3 } else { y3 };
                /* Verify quadrant */
                if !(dy != 0 && dx == 0 && x4 > y4) {
                    if !(dx != 0 && dy == 0 && y4 > x4) {
                        /* Approximate Double Distance */
                        v =
                            if x4 > y4 {
                                (x4 + x4) + y4
                            } else { (y4 + y4) + x4 };
                        /* XXX XXX XXX Penalize location */
                        /* Track best */
                        if !(b_i >= 0 as libc::c_int && v >= b_v) {
                            /* Track best */
                            b_i = i;
                            b_v = v
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Result */
    return b_i as s16b;
}
/*
 * Hack -- determine if a given location is "interesting"
 */
unsafe extern "C" fn target_set_accept(mut y: libc::c_int, mut x: libc::c_int)
 -> bool_ {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    /* Player grid is always interesting */
    if y == (*p_ptr).py as libc::c_int && x == (*p_ptr).px as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Handle hallucination */
    if (*p_ptr).image != 0 { return 0 as libc::c_int as bool_ }
    /* Examine the grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Visible monsters */
    if (*c_ptr).m_idx as libc::c_int != 0 &&
           ((*c_ptr).m_idx as libc::c_int) < max_r_idx as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
        /* Visible monsters */
        if (*m_ptr).ml != 0 { return 1 as libc::c_int as bool_ }
    }
    /* Scan all objects in the grid */
    this_o_idx = (*c_ptr).o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Memorized object */
        if (*o_ptr).marked != 0 { return 1 as libc::c_int as bool_ }
        this_o_idx = next_o_idx
    }
    /* Interesting memorized features */
    if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int != 0 {
        /* Traps are interesting */
        if (*c_ptr).info as libc::c_int & 0x100 as libc::c_int != 0 {
            return 1 as libc::c_int as bool_
        }
        /* Hack -- Doors are boring */
        if (*c_ptr).feat as libc::c_int == 0x4 as libc::c_int {
            return 0 as libc::c_int as bool_
        }
        if (*c_ptr).feat as libc::c_int == 0x5 as libc::c_int {
            return 0 as libc::c_int as bool_
        }
        if (*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
               (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int {
            return 0 as libc::c_int as bool_
        }
        /* Accept 'naturally' interesting features */
        if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
               0x200 as libc::c_long != 0 {
            return 1 as libc::c_int as bool_
        }
    }
    /* Nope */
    return 0 as libc::c_int as bool_;
}
/*
 * Prepare the "temp" array for "target_set"
 *
 * Return the number of target_able monsters in the set.
 */
unsafe extern "C" fn target_set_prepare(mut mode: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Reset "temp" array */
    temp_n = 0 as libc::c_int as s16b;
    /* Scan the current panel */
    y = panel_row_min as libc::c_int;
    while y <= panel_row_max as libc::c_int {
        x = panel_col_min as libc::c_int;
        while x <= panel_col_max as libc::c_int {
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            /* Require line of sight, unless "look" is "expanded" */
            if !(expand_look == 0 &&
                     !((*cave[y as usize].offset(x as isize)).info as
                           libc::c_int & 0x20 as libc::c_int !=
                           0 as libc::c_int)) {
                /* Require "interesting" contents */
                if !(target_set_accept(y, x) == 0) {
                    /* Require target_able monsters for "TARGET_KILL" */
                    if !(mode & 0x1 as libc::c_int != 0 &&
                             target_able((*c_ptr).m_idx as libc::c_int) == 0)
                       {
                        /* Save the location */
                        temp_x[temp_n as usize] = x as byte_hack;
                        temp_y[temp_n as usize] = y as byte_hack;
                        temp_n += 1
                    }
                }
            }
            x += 1
        }
        y += 1
    }
    /* Set the sort hooks */
    ang_sort_comp =
        Some(ang_sort_comp_distance as
                 unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                      _: libc::c_int) -> bool_);
    ang_sort_swap =
        Some(ang_sort_swap_distance as
                 unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                      _: libc::c_int) -> ());
    /* Sort the positions */
    ang_sort(temp_x.as_mut_ptr() as vptr, temp_y.as_mut_ptr() as vptr,
             temp_n as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn target_object(mut y: libc::c_int, mut x: libc::c_int,
                                       mut mode: libc::c_int, mut info: cptr,
                                       mut boring: *mut bool_,
                                       mut o_ptr: *mut object_type,
                                       mut out_val: *mut libc::c_char,
                                       mut s1: *mut cptr, mut s2: *mut cptr,
                                       mut s3: *mut cptr,
                                       mut query: *mut libc::c_int) -> bool_ {
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Not boring */
    *boring = 0 as libc::c_int as bool_;
    /* Obtain an object description */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Describe the object */
    sprintf(out_val, b"%s%s%s%s [%s]\x00" as *const u8 as *const libc::c_char,
            *s1, *s2, *s3, o_name.as_mut_ptr(), info);
    prt(out_val as cptr, 0 as libc::c_int, 0 as libc::c_int);
    move_cursor_relative(y, x);
    *query = inkey() as libc::c_int;
    /* Always stop at "normal" keys */
    if *query != '\r' as i32 && *query != '\n' as i32 && *query != ' ' as i32
       {
        return 1 as libc::c_int as bool_
    }
    /* Sometimes stop at "space" key */
    if *query == ' ' as i32 && mode & 0x2 as libc::c_int == 0 {
        return 1 as libc::c_int as bool_
    }
    /* Change the intro */
    *s1 = b"It is \x00" as *const u8 as *const libc::c_char;
    /* Plurals */
    if (*o_ptr).number as libc::c_int != 1 as libc::c_int {
        *s1 = b"They are \x00" as *const u8 as *const libc::c_char
    }
    /* Preposition */
    *s2 = b"on \x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int as bool_;
}
/*
 * Examine a grid, return a keypress.
 *
 * The "mode" argument contains the "TARGET_LOOK" bit flag, which
 * indicates that the "space" key should scan through the contents
 * of the grid, instead of simply returning immediately.  This lets
 * the "look" command get complete information, without making the
 * "target" command annoying.
 *
 * The "info" argument contains the "commands" which should be shown
 * inside the "[xxx]" text.  This string must never be empty, or grids
 * containing monsters will be displayed with an extra comma.
 *
 * Note that if a monster is in the grid, we update both the monster
 * recall info and the health bar info to track that monster.
 *
 * Eventually, we may allow multiple objects per grid, or objects
 * and terrain features in the same grid. XXX XXX XXX
 *
 * This function must handle blindness/hallucination.
 */
unsafe extern "C" fn target_set_aux(mut y: libc::c_int, mut x: libc::c_int,
                                    mut mode: libc::c_int, mut info: cptr)
 -> libc::c_int {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut s1: cptr = 0 as *const libc::c_char;
    let mut s2: cptr = 0 as *const libc::c_char;
    let mut s3: cptr = 0 as *const libc::c_char;
    let mut boring: bool_ = 0;
    let mut feat: libc::c_int = 0;
    let mut query: libc::c_int = 0;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    loop 
         /* Repeat forever */
         /* Paranoia */
         {
        query = ' ' as i32;
        /* Assume boring */
        boring = 1 as libc::c_int as bool_;
        /* Default */
        s1 = b"You see \x00" as *const u8 as *const libc::c_char;
        s2 = b"\x00" as *const u8 as *const libc::c_char;
        s3 = b"\x00" as *const u8 as *const libc::c_char;
        /* Hack -- under the player */
        if y == (*p_ptr).py as libc::c_int && x == (*p_ptr).px as libc::c_int
           {
            /* Description */
            s1 = b"You are \x00" as *const u8 as *const libc::c_char;
            /* Preposition */
            s2 = b"on \x00" as *const u8 as *const libc::c_char
        }
        /* Hack -- hallucination */
        if (*p_ptr).image != 0 {
            let mut name: cptr =
                b"something strange\x00" as *const u8 as *const libc::c_char;
            /* Display a message */
            sprintf(out_val.as_mut_ptr(),
                    b"%s%s%s%s [%s]\x00" as *const u8 as *const libc::c_char,
                    s1, s2, s3, name, info);
            prt(out_val.as_mut_ptr() as cptr, 0 as libc::c_int,
                0 as libc::c_int);
            move_cursor_relative(y, x);
            query = inkey() as libc::c_int;
            /* Stop on everything but "return" */
            if query != '\r' as i32 && query != '\n' as i32 { break ; }
        } else {
            /* Actual monsters */
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
                /* Mimics special treatment -- looks like an object */
                if (*r_ptr).flags9 & 0x8 as libc::c_int as libc::c_uint != 0
                       && (*m_ptr).csleep as libc::c_int != 0 {
                    let mut o_ptr: *mut object_type = 0 as *mut object_type;
                    /* Acquire object */
                    o_ptr =
                        &mut *o_list.offset((*m_ptr).hold_o_idx as isize) as
                            *mut object_type;
                    if (*o_ptr).marked != 0 {
                        if target_object(y, x, mode, info, &mut boring, o_ptr,
                                         out_val.as_mut_ptr(), &mut s1,
                                         &mut s2, &mut s3, &mut query) != 0 {
                            break ;
                        }
                    }
                } else if (*m_ptr).ml != 0 {
                    let mut recall: bool_ = 0 as libc::c_int as bool_;
                    let mut m_name: [libc::c_char; 80] = [0; 80];
                    /* Visible */
                    /* Not boring */
                    boring = 0 as libc::c_int as bool_;
                    /* Get the monster name ("a kobold") */
                    monster_desc(m_name.as_mut_ptr(), m_ptr,
                                 0x8 as libc::c_int);
                    /* Hack -- track this monster race */
                    monster_race_track((*m_ptr).r_idx as libc::c_int,
                                       (*m_ptr).ego as libc::c_int);
                    /* Hack -- health bar for this monster */
                    health_track((*c_ptr).m_idx as libc::c_int);
                    /* Hack -- handle stuff */
                    handle_stuff();
                    loop 
                         /* Interact */
                         /* Recall */
                         {
                        if recall != 0 {
                            /* Save */
                            character_icky = 1 as libc::c_int as bool_;
                            Term_save();
                            /* Recall on screen */
                            screen_roff((*m_ptr).r_idx as libc::c_int,
                                        (*m_ptr).ego as libc::c_int,
                                        0 as libc::c_int);
                            /* Hack -- Complete the prompt (again) */
                            Term_addstr(-(1 as libc::c_int),
                                        1 as libc::c_int as byte_hack,
                                        format(b"  [r,%s]\x00" as *const u8 as
                                                   *const libc::c_char, info)
                                            as cptr);
                            /* Command */
                            query = inkey() as libc::c_int;
                            /* Restore */
                            Term_load();
                            character_icky = 0 as libc::c_int as bool_
                        } else {
                            /* Normal */
                            let mut mstat: cptr = 0 as *const libc::c_char;
                            match (*m_ptr).status as libc::c_int {
                                0 | -1 | 1 => {
                                    mstat =
                                        b" (neutral) \x00" as *const u8 as
                                            *const libc::c_char
                                }
                                3 => {
                                    mstat =
                                        b" (pet) \x00" as *const u8 as
                                            *const libc::c_char
                                }
                                2 => {
                                    mstat =
                                        b" (coaligned) \x00" as *const u8 as
                                            *const libc::c_char
                                }
                                4 => {
                                    mstat =
                                        b" (companion) \x00" as *const u8 as
                                            *const libc::c_char
                                }
                                _ => {
                                    mstat =
                                        b" \x00" as *const u8 as
                                            *const libc::c_char
                                }
                            }
                            if (*m_ptr).mflag & 0x4 as libc::c_int != 0 {
                                mstat =
                                    b" (partial) \x00" as *const u8 as
                                        *const libc::c_char
                            }
                            /* Describe, and prompt for recall */
                            sprintf(out_val.as_mut_ptr(),
                                    b"%s%s%s%s (level %d, %s%s)%s%s[r,%s]\x00"
                                        as *const u8 as *const libc::c_char,
                                    s1, s2, s3, m_name.as_mut_ptr(),
                                    (*m_ptr).level as libc::c_int,
                                    look_mon_desc((*c_ptr).m_idx as
                                                      libc::c_int),
                                    if (*m_ptr).mflag & 0x2 as libc::c_int !=
                                           0 {
                                        b", quest\x00" as *const u8 as
                                            *const libc::c_char
                                    } else {
                                        b"\x00" as *const u8 as
                                            *const libc::c_char
                                    },
                                    if (*m_ptr).smart &
                                           0x400000 as libc::c_int as
                                               libc::c_uint != 0 {
                                        b" (clone)\x00" as *const u8 as
                                            *const libc::c_char
                                    } else {
                                        b"\x00" as *const u8 as
                                            *const libc::c_char
                                    }, mstat, info);
                            prt(out_val.as_mut_ptr() as cptr,
                                0 as libc::c_int, 0 as libc::c_int);
                            /* Place cursor */
                            move_cursor_relative(y, x);
                            /* Command */
                            query = inkey() as libc::c_int
                        }
                        /* Normal commands */
                        if query != 'r' as i32 { break ; }
                        /* Toggle recall */
                        recall = (recall == 0) as libc::c_int as bool_
                    }
                    /* Always stop at "normal" keys */
                    if query != '\r' as i32 && query != '\n' as i32 &&
                           query != ' ' as i32 {
                        break ;
                    }
                    /* Sometimes stop at "space" key */
                    if query == ' ' as i32 && mode & 0x2 as libc::c_int == 0 {
                        break ;
                    }
                    /* Change the intro */
                    s1 = b"It is \x00" as *const u8 as *const libc::c_char;
                    /* Hack -- take account of gender */
                    if (*r_ptr).flags1 & 0x8 as libc::c_int as libc::c_uint !=
                           0 {
                        s1 =
                            b"She is \x00" as *const u8 as *const libc::c_char
                    } else if (*r_ptr).flags1 &
                                  0x4 as libc::c_int as libc::c_uint != 0 {
                        s1 = b"He is \x00" as *const u8 as *const libc::c_char
                    }
                    /* Use a preposition */
                    s2 = b"carrying \x00" as *const u8 as *const libc::c_char;
                    /* Scan all objects being carried */
                    this_o_idx = (*m_ptr).hold_o_idx;
                    while this_o_idx != 0 {
                        let mut o_name: [libc::c_char; 80] = [0; 80];
                        let mut o_ptr_0: *mut object_type =
                            0 as *mut object_type;
                        /* Acquire object */
                        o_ptr_0 =
                            &mut *o_list.offset(this_o_idx as isize) as
                                *mut object_type;
                        /* Acquire next object */
                        next_o_idx = (*o_ptr_0).next_o_idx;
                        /* Obtain an object description */
                        object_desc(o_name.as_mut_ptr(), o_ptr_0,
                                    1 as libc::c_int, 3 as libc::c_int);
                        /* Describe the object */
                        sprintf(out_val.as_mut_ptr(),
                                b"%s%s%s%s [%s]\x00" as *const u8 as
                                    *const libc::c_char, s1, s2, s3,
                                o_name.as_mut_ptr(), info);
                        prt(out_val.as_mut_ptr() as cptr, 0 as libc::c_int,
                            0 as libc::c_int);
                        move_cursor_relative(y, x);
                        query = inkey() as libc::c_int;
                        /* Always stop at "normal" keys */
                        if query != '\r' as i32 && query != '\n' as i32 &&
                               query != ' ' as i32 {
                            break ;
                        }
                        /* Sometimes stop at "space" key */
                        if query == ' ' as i32 &&
                               mode & 0x2 as libc::c_int == 0 {
                            break ;
                        }
                        /* Change the intro */
                        s2 =
                            b"also carrying \x00" as *const u8 as
                                *const libc::c_char;
                        this_o_idx = next_o_idx
                    }
                    /* Double break */
                    if this_o_idx != 0 { break ; }
                    /* Use a preposition */
                    s2 = b"on \x00" as *const u8 as *const libc::c_char
                }
            }
            /* Scan all objects in the grid */
            this_o_idx = (*c_ptr).o_idx;
            while this_o_idx != 0 {
                let mut o_ptr_1: *mut object_type = 0 as *mut object_type;
                /* Acquire object */
                o_ptr_1 =
                    &mut *o_list.offset(this_o_idx as isize) as
                        *mut object_type;
                /* Acquire next object */
                next_o_idx = (*o_ptr_1).next_o_idx;
                /* Describe it */
                if (*o_ptr_1).marked != 0 {
                    if target_object(y, x, mode, info, &mut boring, o_ptr_1,
                                     out_val.as_mut_ptr(), &mut s1, &mut s2,
                                     &mut s3, &mut query) != 0 {
                        break ;
                    }
                }
                this_o_idx = next_o_idx
            }
            /* Double break */
            if this_o_idx != 0 { break ; }
            /* Actual traps */
            if (*c_ptr).info as libc::c_int & 0x100 as libc::c_int != 0 &&
                   (*c_ptr).t_idx as libc::c_int != 0 {
                let mut name_0: cptr =
                    b"a trap\x00" as *const u8 as *const libc::c_char;
                let mut s4: cptr = 0 as *const libc::c_char;
                /* Name trap */
                if (*t_info.offset((*c_ptr).t_idx as isize)).ident != 0 {
                    s4 =
                        format(b"(%s)\x00" as *const u8 as
                                   *const libc::c_char,
                               t_name.offset((*t_info.offset((*c_ptr).t_idx as
                                                                 isize)).name
                                                 as libc::c_int as isize)) as
                            cptr
                } else {
                    s4 =
                        b"an unknown trap\x00" as *const u8 as
                            *const libc::c_char
                }
                /* Display a message */
                sprintf(out_val.as_mut_ptr(),
                        b"%s%s%s%s [%s]\x00" as *const u8 as
                            *const libc::c_char, s1, s2, s3, name_0, s4);
                prt(out_val.as_mut_ptr() as cptr, 0 as libc::c_int,
                    0 as libc::c_int);
                move_cursor_relative(y, x);
                query = inkey() as libc::c_int;
                /* Stop on everything but "return" */
                if query != '\r' as i32 && query != '\n' as i32 { break ; }
            } else {
                /* Feature (apply "mimic") */
                if (*c_ptr).mimic != 0 {
                    feat = (*c_ptr).mimic as libc::c_int
                } else {
                    feat =
                        (*f_info.offset((*c_ptr).feat as isize)).mimic as
                            libc::c_int
                }
                /* Require knowledge about grid, or ability to see grid */
                if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int == 0 &&
                       !((*cave[y as usize].offset(x as isize)).info as
                             libc::c_int & 0x10 as libc::c_int !=
                             0 as libc::c_int) {
                    /* Forget feature */
                    feat = 0 as libc::c_int
                }
                /* Terrain feature if needed */
                if boring as libc::c_int != 0 || feat >= 0x3 as libc::c_int {
                    let mut name_1: cptr = 0 as *const libc::c_char;
                    /* Hack -- special handling for building doors */
                    if feat == 0x4a as libc::c_int {
                        name_1 =
                            st_name.offset((*st_info.offset((*c_ptr).special
                                                                as
                                                                isize)).name
                                               as isize) as cptr
                    } else {
                        name_1 =
                            f_name.offset((*f_info.offset(feat as isize)).name
                                              as isize) as cptr
                    }
                    /* Hack -- handle unknown grids */
                    if feat == 0 as libc::c_int {
                        name_1 =
                            b"unknown grid\x00" as *const u8 as
                                *const libc::c_char
                    }
                    /* Pick a prefix */
                    if *s2 as libc::c_int != 0 &&
                           (feat >= 0x40 as libc::c_int &&
                                feat <= 0x49 as libc::c_int ||
                                feat == 0x58 as libc::c_int ||
                                feat == 0x59 as libc::c_int ||
                                feat == 0xc7 as libc::c_int) {
                        s2 = b"on \x00" as *const u8 as *const libc::c_char
                    } else if *s2 as libc::c_int != 0 &&
                                  feat == 0xca as libc::c_int {
                        s2 = b"by \x00" as *const u8 as *const libc::c_char
                    } else if *s2 as libc::c_int != 0 &&
                                  feat >= 0x20 as libc::c_int {
                        s2 = b"in \x00" as *const u8 as *const libc::c_char
                    }
                    /* Pick proper indefinite article */
                    s3 =
                        if is_a_vowel(*name_1.offset(0 as libc::c_int as
                                                         isize) as
                                          libc::c_int) as libc::c_int != 0 {
                            b"an \x00" as *const u8 as *const libc::c_char
                        } else {
                            b"a \x00" as *const u8 as *const libc::c_char
                        };
                    /* Hack -- special introduction for store & building doors */
                    if feat == 0x4a as libc::c_int {
                        s3 =
                            b"the entrance to the \x00" as *const u8 as
                                *const libc::c_char
                    }
                    if feat == 0x7 as libc::c_int &&
                           (*c_ptr).special as libc::c_int != 0 {
                        s3 = b"\x00" as *const u8 as *const libc::c_char;
                        name_1 =
                            d_text.offset((*d_info.offset((*c_ptr).special as
                                                              isize)).text as
                                              isize) as cptr
                    }
                    if (*p_ptr).wild_mode as libc::c_int != 0 &&
                           feat == 0xcb as libc::c_int {
                        s3 = b"\x00" as *const u8 as *const libc::c_char;
                        name_1 =
                            format(b"%s(%s)\x00" as *const u8 as
                                       *const libc::c_char,
                                   wf_name.offset((*wf_info.offset((*(*wild_map.offset(y
                                                                                           as
                                                                                           isize)).offset(x
                                                                                                              as
                                                                                                              isize)).feat
                                                                       as
                                                                       isize)).name
                                                      as isize),
                                   wf_text.offset((*wf_info.offset((*(*wild_map.offset(y
                                                                                           as
                                                                                           isize)).offset(x
                                                                                                              as
                                                                                                              isize)).feat
                                                                       as
                                                                       isize)).text
                                                      as isize)) as cptr
                    }
                    if feat == 0x2 as libc::c_int &&
                           (*c_ptr).info as libc::c_int & 0x200 as libc::c_int
                               != 0 {
                        let mut k_ptr: *mut object_kind =
                            0 as *mut object_kind;
                        let mut tv: libc::c_int = 0;
                        let mut sv: libc::c_int = 0;
                        if (*c_ptr).special as libc::c_int <=
                               63 as libc::c_int {
                            tv = 71 as libc::c_int;
                            sv = (*c_ptr).special as libc::c_int
                        } else {
                            tv = 72 as libc::c_int;
                            sv =
                                (*c_ptr).special as libc::c_int -
                                    63 as libc::c_int
                        }
                        k_ptr =
                            &mut *k_info.offset((lookup_kind as
                                                     unsafe extern "C" fn(_:
                                                                              libc::c_int,
                                                                          _:
                                                                              libc::c_int)
                                                         -> s16b)(tv, sv) as
                                                    isize) as
                                *mut object_kind;
                        info = k_name.offset((*k_ptr).name as isize) as cptr
                    }
                    /* Display a message */
                    if wizard == 0 {
                        sprintf(out_val.as_mut_ptr(),
                                b"%s%s%s%s [%s]\x00" as *const u8 as
                                    *const libc::c_char, s1, s2, s3, name_1,
                                info);
                    } else {
                        sprintf(out_val.as_mut_ptr(),
                                b"%s%s%s%s [%s] (%d:%d:%d)\x00" as *const u8
                                    as *const libc::c_char, s1, s2, s3,
                                name_1, info, (*c_ptr).feat as libc::c_int,
                                (*c_ptr).mimic as libc::c_int,
                                (*c_ptr).special as libc::c_int);
                    }
                    prt(out_val.as_mut_ptr() as cptr, 0 as libc::c_int,
                        0 as libc::c_int);
                    move_cursor_relative(y, x);
                    query = inkey() as libc::c_int;
                    /* Always stop at "normal" keys */
                    if query != '\r' as i32 && query != '\n' as i32 &&
                           query != ' ' as i32 {
                        break ;
                    }
                }
                /* Stop on everything but "return" */
                if query != '\r' as i32 && query != '\n' as i32 { break ; }
            }
        }
    }
    /* Keep going */
    return query;
}
/*
 * Handle "target" and "look".
 *
 * Note that this code can be called from "get_aim_dir()".
 *
 * All locations must be on the current panel.  Consider the use of
 * "panel_bounds()" to allow "off-panel" targets, perhaps by using
 * some form of "scrolling" the map around the cursor.  XXX XXX XXX
 * That is, consider the possibility of "auto-scrolling" the screen
 * while the cursor moves around.  This may require changes in the
 * "update_mon()" code to allow "visibility" even if off panel, and
 * may require dynamic recalculation of the "temp" grid set.
 *
 * Hack -- targetting/observing an "outer border grid" may induce
 * problems, so this is not currently allowed.
 *
 * The player can use the direction keys to move among "interesting"
 * grids in a heuristic manner, or the "space", "+", and "-" keys to
 * move through the "interesting" grids in a sequential manner, or
 * can enter "location" mode, and use the direction keys to move one
 * grid at a time in any direction.  The "t" (set target) command will
 * only target a monster (as opposed to a location) if the monster is
 * target_able and the "interesting" mode is being used.
 *
 * The current grid is described using the "look" method above, and
 * a new command may be entered at any time, but note that if the
 * "TARGET_LOOK" bit flag is set (or if we are in "location" mode,
 * where "space" has no obvious meaning) then "space" will scan
 * through the description of the current grid until done, instead
 * of immediately jumping to the next "interesting" grid.  This
 * allows the "target" command to retain its old semantics.
 *
 * The "*", "+", and "-" keys may always be used to jump immediately
 * to the next (or previous) interesting grid, in the proper mode.
 *
 * The "return" key may always be used to scan through a complete
 * grid description (forever).
 *
 * This command will cancel any old target, even if used from
 * inside the "look" command.
 */
#[no_mangle]
pub unsafe extern "C" fn target_set(mut mode: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut y: libc::c_int = (*p_ptr).py as libc::c_int;
    let mut x: libc::c_int = (*p_ptr).px as libc::c_int;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    let mut flag: bool_ = 1 as libc::c_int as bool_;
    let mut query: libc::c_char = 0;
    let mut info: [libc::c_char; 80] = [0; 80];
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut screen_wid: libc::c_int = 0;
    let mut screen_hgt: libc::c_int = 0;
    let mut panel_wid: libc::c_int = 0;
    let mut panel_hgt: libc::c_int = 0;
    /* Get size */
    get_screen_size(&mut screen_wid, &mut screen_hgt);
    /* Calculate the amount of panel movement */
    panel_hgt = screen_hgt / 2 as libc::c_int;
    panel_wid = screen_wid / 2 as libc::c_int;
    /* Cancel target */
    target_who = 0 as libc::c_int as s16b;
    /* Cancel tracking */
	/* health_track(0); */
    /* Prepare the "temp" array */
    target_set_prepare(mode);
    /* Start near the player */
    m = 0 as libc::c_int;
    /* Interact */
    while done == 0 {
        /* Interesting grids */
        if flag as libc::c_int != 0 && temp_n as libc::c_int != 0 {
            y = temp_y[m as usize] as libc::c_int;
            x = temp_x[m as usize] as libc::c_int;
            /* Access */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            /* Allow target */
            if target_able((*c_ptr).m_idx as libc::c_int) != 0 {
                strcpy(info.as_mut_ptr(),
                       b"q,t,p,o,+,-,\'dir\'\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                /* Dis-allow target */
                strcpy(info.as_mut_ptr(),
                       b"q,p,o,+,-,\'dir\'\x00" as *const u8 as
                           *const libc::c_char);
            }
            /* Describe and Prompt */
            query =
                target_set_aux(y, x, mode, info.as_mut_ptr() as cptr) as
                    libc::c_char;
            /* Cancel tracking */
			/* health_track(0); */
            /* Assume no "direction" */
            d = 0 as libc::c_int;
            let mut current_block_48: u64;
            /* Analyze */
            match query as libc::c_int {
                27 | 113 => {
                    done = 1 as libc::c_int as bool_;
                    current_block_48 = 7252614138838059896;
                }
                116 | 46 | 53 | 48 => {
                    if target_able((*c_ptr).m_idx as libc::c_int) != 0 {
                        health_track((*c_ptr).m_idx as libc::c_int);
                        target_who = (*c_ptr).m_idx;
                        target_row = y as s16b;
                        target_col = x as s16b;
                        done = 1 as libc::c_int as bool_
                    } else { bell(); }
                    current_block_48 = 7252614138838059896;
                }
                32 | 42 | 43 => {
                    m += 1;
                    if m == temp_n as libc::c_int {
                        m = 0 as libc::c_int;
                        if expand_list == 0 {
                            done = 1 as libc::c_int as bool_
                        }
                    }
                    current_block_48 = 7252614138838059896;
                }
                45 => {
                    let fresh0 = m;
                    m = m - 1;
                    if fresh0 == 0 as libc::c_int {
                        m = temp_n as libc::c_int - 1 as libc::c_int;
                        if expand_list == 0 {
                            done = 1 as libc::c_int as bool_
                        }
                    }
                    current_block_48 = 7252614138838059896;
                }
                112 => {
                    /* Recenter the map around the player */
                    verify_panel();
                    /* Fall through... */
                    (*p_ptr).update =
                        ((*p_ptr).update as libc::c_long |
                             0x1000000 as libc::c_long) as u32b;
                    (*p_ptr).redraw =
                        ((*p_ptr).redraw as libc::c_long |
                             0x4000000 as libc::c_long) as u32b;
                    (*p_ptr).window =
                        ((*p_ptr).window as libc::c_long |
                             0x80 as libc::c_long) as u32b;
                    handle_stuff();
                    target_set_prepare(mode);
                    y = (*p_ptr).py as libc::c_int;
                    x = (*p_ptr).px as libc::c_int;
                    current_block_48 = 13281731871476506071;
                }
                111 => { current_block_48 = 13281731871476506071; }
                109 => { current_block_48 = 7252614138838059896; }
                _ => {
                    /* Update stuff */
                    /* Redraw map */
                    /* Window stuff */
                    /* Handle stuff */
                    /* Recalculate interesting grids */
                    /* Extract the action (if any) */
                    d = get_keymap_dir(query);
                    if d == 0 { bell(); }
                    current_block_48 = 7252614138838059896;
                }
            }
            match current_block_48 {
                13281731871476506071 => { flag = 0 as libc::c_int as bool_ }
                _ => { }
            }
            /* Hack -- move around */
            if d != 0 {
                /* Find a new monster */
                i =
                    target_pick(temp_y[m as usize] as libc::c_int,
                                temp_x[m as usize] as libc::c_int,
                                ddy[d as usize] as libc::c_int,
                                ddx[d as usize] as libc::c_int) as
                        libc::c_int;
                /* Scroll to find interesting grid */
                if i < 0 as libc::c_int {
                    let mut dy: libc::c_int = 0;
                    let mut dx: libc::c_int = 0;
                    dy = ddy[d as usize] as libc::c_int;
                    dx = ddx[d as usize] as libc::c_int;
                    /* Note panel change */
                    if change_panel(dy, dx) != 0 {
                        let mut ty: libc::c_int =
                            temp_y[m as usize] as libc::c_int;
                        let mut tx: libc::c_int =
                            temp_x[m as usize] as libc::c_int;
                        /* Recalculate interesting grids */
                        target_set_prepare(mode);
                        /* Find a new monster */
                        i = target_pick(ty, tx, dy, dx) as libc::c_int;
                        /* Restore panel if needed */
                        if i < 0 as libc::c_int {
                            /* Restore panel */
                            change_panel(-dy, -dx);
                            /* Recalculate interesting grids */
                            target_set_prepare(mode);
                        }
                    }
                }
                /* Use that grids */
                if i >= 0 as libc::c_int { m = i }
            }
        } else {
            /* Arbitrary grids */
            /* Access */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            strcpy(info.as_mut_ptr(),
                   b"q,t,p,m,+,-,\'dir\'\x00" as *const u8 as
                       *const libc::c_char);
            query =
                target_set_aux(y, x, mode | 0x2 as libc::c_int,
                               info.as_mut_ptr() as cptr) as libc::c_char;
            d = 0 as libc::c_int;
            match query as libc::c_int {
                27 | 113 => { done = 1 as libc::c_int as bool_ }
                116 | 46 | 53 | 48 => {
                    target_who = -(1 as libc::c_int) as s16b;
                    target_row = y as s16b;
                    target_col = x as s16b;
                    done = 1 as libc::c_int as bool_
                }
                112 => {
                    y = (*p_ptr).py as libc::c_int;
                    x = (*p_ptr).px as libc::c_int
                }
                32 | 42 | 43 | 45 | 111 => { }
                109 => { flag = 1 as libc::c_int as bool_ }
                _ => {
                    /* Default prompt */
                    /* Describe and Prompt (enable "TARGET_LOOK") */
                    /* Cancel tracking */
			/* health_track(0); */
                    /* Assume no direction */
                    /* Extract the action (if any) */
                    d = get_keymap_dir(query);
                    if d == 0 { bell(); }
                }
            }
            if d != 0 {
                let mut dy_0: libc::c_int = ddy[d as usize] as libc::c_int;
                let mut dx_0: libc::c_int = ddx[d as usize] as libc::c_int;
                /* Analyze the keypress */
                /* Move */
                y += dy_0;
                x += dx_0;
                /* Do not move horizontally if unnecessary */
                if x < panel_col_min as libc::c_int + panel_wid &&
                       dx_0 > 0 as libc::c_int ||
                       x > panel_col_min as libc::c_int + panel_wid &&
                           dx_0 < 0 as libc::c_int {
                    dx_0 = 0 as libc::c_int
                }
                /* Do not move vertically if unnecessary */
                if y < panel_row_min as libc::c_int + panel_hgt &&
                       dy_0 > 0 as libc::c_int ||
                       y > panel_row_min as libc::c_int + panel_hgt &&
                           dy_0 < 0 as libc::c_int {
                    dy_0 = 0 as libc::c_int
                }
                /* Apply the motion */
                if y >= panel_row_min as libc::c_int + screen_hgt ||
                       y < panel_row_min as libc::c_int ||
                       x > panel_col_min as libc::c_int + screen_wid ||
                       x < panel_col_min as libc::c_int {
                    /* Change panel and recalculate interesting grids */
                    if change_panel(dy_0, dx_0) != 0 {
                        target_set_prepare(mode);
                    }
                }
                /* Boundary checks */
                if wizard == 0 {
                    /* Hack -- Verify y */
                    if y <= 0 as libc::c_int {
                        y = 1 as libc::c_int
                    } else if y >= cur_hgt as libc::c_int - 1 as libc::c_int {
                        y = cur_hgt as libc::c_int - 2 as libc::c_int
                    }
                    /* Hack -- Verify x */
                    if x <= 0 as libc::c_int {
                        x = 1 as libc::c_int
                    } else if x >= cur_wid as libc::c_int - 1 as libc::c_int {
                        x = cur_wid as libc::c_int - 2 as libc::c_int
                    }
                } else {
                    /* Hack -- Verify y */
                    if y < 0 as libc::c_int {
                        y = 0 as libc::c_int
                    } else if y > cur_hgt as libc::c_int - 1 as libc::c_int {
                        y = cur_hgt as libc::c_int - 1 as libc::c_int
                    }
                    /* Hack -- Verify x */
                    if x < 0 as libc::c_int {
                        x = 0 as libc::c_int
                    } else if x > cur_wid as libc::c_int - 1 as libc::c_int {
                        x = cur_wid as libc::c_int - 1 as libc::c_int
                    }
                }
            }
        }
    }
    /* Handle "direction" */
    /* Forget */
    temp_n = 0 as libc::c_int as s16b;
    /* Clear the top line */
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
    /* Recenter the map around the player */
    verify_panel();
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
    /* Failure to set target */
    if target_who == 0 { return 0 as libc::c_int as bool_ }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Get an "aiming direction" from the user.
 *
 * The "dir" is loaded with 1,2,3,4,6,7,8,9 for "actual direction", and
 * "0" for "current target", and "-1" for "entry aborted".
 *
 * Note that "Force Target", if set, will pre-empt user interaction,
 * if there is a usable target already set.
 *
 * Note that confusion over-rides any (explicit?) user choice.
 */
#[no_mangle]
pub unsafe extern "C" fn get_aim_dir(mut dp: *mut libc::c_int) -> bool_ {
    let mut dir: libc::c_int = 0;
    let mut command: libc::c_char = 0;
    let mut p: cptr = 0 as *const libc::c_char;
    if repeat_pull(dp) != 0 {
        /* Confusion? */
        /* Verify */
        if !(*dp == 5 as libc::c_int && target_okay() == 0) {
            return 1 as libc::c_int as bool_
        }
    }
    /* Initialize */
    *dp = 0 as libc::c_int;
    /* Global direction */
    dir = command_dir as libc::c_int;
    /* Hack -- auto-target if requested */
    if use_old_target as libc::c_int != 0 && target_okay() as libc::c_int != 0
       {
        dir = 5 as libc::c_int
    }
    /* Ask until satisfied */
    while dir == 0 {
        /* Choose a prompt */
        if target_okay() == 0 {
            p =
                b"Direction (\'*\' to choose a target, Escape to cancel)? \x00"
                    as *const u8 as *const libc::c_char
        } else {
            p =
                b"Direction (\'5\' for target, \'*\' to re-target, Escape to cancel)? \x00"
                    as *const u8 as *const libc::c_char
        }
        /* Get a command (or Cancel) */
        if get_com(p, &mut command) == 0 { break ; }
        /* Convert various keys to "standard" keys */
        match command as libc::c_int {
            84 | 116 | 46 | 53 | 48 => {
                /* Use current target */
                dir = 5 as libc::c_int
            }
            42 => {
                /* Set new target */
                if target_set(0x1 as libc::c_int) != 0 {
                    dir = 5 as libc::c_int
                }
            }
            _ => {
                /* Extract the action (if any) */
                dir = get_keymap_dir(command)
            }
        }
        /* Verify requested targets */
        if dir == 5 as libc::c_int && target_okay() == 0 {
            dir = 0 as libc::c_int
        }
        /* Error */
        if dir == 0 { bell(); }
    }
    /* No direction */
    if dir == 0 { return 0 as libc::c_int as bool_ }
    /* Save the direction */
    command_dir = dir as s16b;
    /* Check for confusion */
    if (*p_ptr).confused != 0 {
        /* XXX XXX XXX */
		/* Random direction */
        dir = ddd[Rand_div(8 as libc::c_int) as usize] as libc::c_int
    }
    /* Notice confusion */
    if command_dir as libc::c_int != dir {
        /* Warn the user */
        msg_print(b"You are confused.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Save direction */
    *dp = dir;
    repeat_push(dir);
    /* A "valid" direction was entered */
    return 1 as libc::c_int as bool_;
}
/*
 * Request a "movement" direction (1,2,3,4,6,7,8,9) from the user,
 * and place it into "command_dir", unless we already have one.
 *
 * This function should be used for all "repeatable" commands, such as
 * run, walk, open, close, bash, disarm, spike, tunnel, etc, as well
 * as all commands which must reference a grid adjacent to the player,
 * and which may not reference the grid under the player.  Note that,
 * for example, it is no longer possible to "disarm" or "open" chests
 * in the same grid as the player.
 *
 * Direction "5" is illegal and will (cleanly) abort the command.
 *
 * This function tracks and uses the "global direction", and uses
 * that as the "desired direction", to which "confusion" is applied.
 */
#[no_mangle]
pub unsafe extern "C" fn get_rep_dir(mut dp: *mut libc::c_int) -> bool_ {
    let mut dir: libc::c_int = 0;
    if repeat_pull(dp) != 0 { return 1 as libc::c_int as bool_ }
    /* Initialize */
    *dp = 0 as libc::c_int;
    /* Global direction */
    dir = command_dir as libc::c_int;
    /* Get a direction */
    while dir == 0 {
        let mut ch: libc::c_char = 0;
        /* Get a command (or Cancel) */
        if get_com(b"Direction (Escape to cancel)? \x00" as *const u8 as
                       *const libc::c_char, &mut ch) == 0 {
            break ;
        }
        /* Look up the direction */
        dir = get_keymap_dir(ch);
        /* Oops */
        if dir == 0 { bell(); }
    }
    /* Prevent weirdness */
    if dir == 5 as libc::c_int { dir = 0 as libc::c_int }
    /* Aborted */
    if dir == 0 { return 0 as libc::c_int as bool_ }
    /* Save desired direction */
    command_dir = dir as s16b;
    /* Apply "confusion" */
    if (*p_ptr).confused != 0 {
        /* Standard confusion */
        if Rand_div(100 as libc::c_int) < 75 as libc::c_int {
            /* Random direction */
            dir = ddd[Rand_div(8 as libc::c_int) as usize] as libc::c_int
        }
    }
    /* Notice confusion */
    if command_dir as libc::c_int != dir {
        /* Warn the user */
        msg_print(b"You are confused.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Save direction */
    *dp = dir;
    repeat_push(dir);
    /* Success */
    return 1 as libc::c_int as bool_; /* Or disallow them */
}
#[no_mangle]
pub unsafe extern "C" fn get_chaos_patron() -> libc::c_int {
    return ((*p_ptr).age as libc::c_int + (*p_ptr).sc as libc::c_int) %
               16 as libc::c_int; /* Allow the 'nasty' effects */
}
#[no_mangle]
pub unsafe extern "C" fn gain_level_reward(mut chosen_reward: libc::c_int) {
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
    let mut wrath_reason: [libc::c_char; 32] =
        *::std::mem::transmute::<&[u8; 32],
                                 &mut [libc::c_char; 32]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut nasty_chance: libc::c_int = 6 as libc::c_int;
    let mut dummy: libc::c_int = 0 as libc::c_int;
    let mut dummy2: libc::c_int = 0 as libc::c_int;
    let mut type_0: libc::c_int = 0;
    let mut effect: libc::c_int = 0;
    if (*p_ptr).lev as libc::c_int == 13 as libc::c_int {
        nasty_chance = 2 as libc::c_int
    } else if (*p_ptr).lev as libc::c_int % 13 as libc::c_int == 0 {
        nasty_chance = 3 as libc::c_int
    } else if (*p_ptr).lev as libc::c_int % 14 as libc::c_int == 0 {
        nasty_chance = 12 as libc::c_int
    }
    if Rand_div(nasty_chance) + 1 as libc::c_int == 1 as libc::c_int {
        type_0 = Rand_div(20 as libc::c_int) + 1 as libc::c_int
    } else {
        type_0 =
            Rand_div(15 as libc::c_int) + 1 as libc::c_int + 5 as libc::c_int
    }
    if type_0 < 1 as libc::c_int { type_0 = 1 as libc::c_int }
    if type_0 > 20 as libc::c_int { type_0 = 20 as libc::c_int }
    type_0 -= 1;
    sprintf(wrath_reason.as_mut_ptr(),
            b"the Wrath of %s\x00" as *const u8 as *const libc::c_char,
            chaos_patrons[(*p_ptr).chaos_patron as usize]);
    effect = chaos_rewards[(*p_ptr).chaos_patron as usize][type_0 as usize];
    if Rand_div(6 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int &&
           chosen_reward == 0 {
        msg_format(b"%^s rewards you with a corruption!\x00" as *const u8 as
                       *const libc::c_char,
                   chaos_patrons[(*p_ptr).chaos_patron as usize]);
        gain_random_corruption(0 as libc::c_int);
        return
    }
    match if chosen_reward != 0 { chosen_reward } else { effect } {
        0 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Thou needst a new form, mortal!\'\x00" as *const u8
                          as *const libc::c_char);
            do_poly_self();
        }
        1 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Well done, mortal! Lead on!\'\x00" as *const u8 as
                          *const libc::c_char);
            if ((*p_ptr).exp as libc::c_long) < 99999999 as libc::c_long {
                let mut ee: s32b =
                    (*p_ptr).exp / 2 as libc::c_int + 10 as libc::c_int;
                if ee as libc::c_long > 100000 as libc::c_long {
                    ee = 100000 as libc::c_long as s32b
                }
                msg_print(b"You feel more experienced.\x00" as *const u8 as
                              *const libc::c_char);
                gain_exp(ee);
            }
        }
        2 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Thou didst not deserve that, slave.\'\x00" as
                          *const u8 as *const libc::c_char);
            lose_exp((*p_ptr).exp / 6 as libc::c_int);
        }
        3 => {
            msg_format(b"The voice of %s whispers:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Use my gift wisely.\'\x00" as *const u8 as
                          *const libc::c_char);
            acquirement((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, 1 as libc::c_int,
                        0 as libc::c_int as bool_, 0 as libc::c_int as bool_);
        }
        4 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Use my gift wisely.\'\x00" as *const u8 as
                          *const libc::c_char);
            acquirement((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, 1 as libc::c_int,
                        1 as libc::c_int as bool_, 0 as libc::c_int as bool_);
        }
        5 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Thy deed hath earned thee a worthy blade.\'\x00" as
                          *const u8 as *const libc::c_char);
            /* Get local object */
            q_ptr = &mut forge;
            dummy = 23 as libc::c_int;
            match Rand_div((*p_ptr).lev as s32b) + 1 as libc::c_int {
                0 | 1 => { dummy2 = 4 as libc::c_int }
                2 | 3 => { dummy2 = 5 as libc::c_int }
                4 | 5 | 6 => { dummy2 = 7 as libc::c_int }
                7 | 8 => { dummy2 = 8 as libc::c_int }
                9 | 10 => { dummy2 = 9 as libc::c_int }
                11 | 12 | 13 => { dummy2 = 10 as libc::c_int }
                14 | 15 => { dummy2 = 11 as libc::c_int }
                16 | 17 => { dummy2 = 12 as libc::c_int }
                18 | 19 => { dummy2 = 14 as libc::c_int }
                20 => { dummy2 = 15 as libc::c_int }
                21 => { dummy2 = 16 as libc::c_int }
                22 | 23 => { dummy2 = 17 as libc::c_int }
                24 | 25 => { dummy2 = 18 as libc::c_int }
                26 | 27 => { dummy2 = 20 as libc::c_int }
                28 | 29 => { dummy2 = 21 as libc::c_int }
                30 => { dummy2 = 22 as libc::c_int }
                31 => { dummy2 = 23 as libc::c_int }
                32 => { dummy2 = 24 as libc::c_int }
                33 => { dummy2 = 25 as libc::c_int }
                34 => { dummy2 = 26 as libc::c_int }
                35 | 36 => { dummy2 = 28 as libc::c_int }
                37 => { dummy2 = 29 as libc::c_int }
                _ => { dummy2 = 30 as libc::c_int }
            }
            object_prep(q_ptr, lookup_kind(dummy, dummy2) as libc::c_int);
            (*q_ptr).to_h =
                (3 as libc::c_int +
                     (Rand_div(dun_level as s32b) + 1 as libc::c_int) %
                         10 as libc::c_int) as s16b;
            (*q_ptr).to_d =
                (3 as libc::c_int +
                     (Rand_div(dun_level as s32b) + 1 as libc::c_int) %
                         10 as libc::c_int) as s16b;
            random_resistance(q_ptr, 0 as libc::c_int as bool_,
                              Rand_div(34 as libc::c_int) + 1 as libc::c_int +
                                  4 as libc::c_int);
            (*q_ptr).name2 = 78 as libc::c_int as s16b;
            /* Apply the ego */
            apply_magic(q_ptr, dun_level as libc::c_int,
                        0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                        0 as libc::c_int as bool_);
            /* Drop it in the dungeon */
            drop_near(q_ptr, -(1 as libc::c_int), (*p_ptr).py as libc::c_int,
                      (*p_ptr).px as libc::c_int);
        }
        6 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Thy deed hath earned thee a worthy reward.\'\x00" as
                          *const u8 as *const libc::c_char);
            acquirement((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int,
                        Rand_div(2 as libc::c_int) + 1 as libc::c_int +
                            1 as libc::c_int, 0 as libc::c_int as bool_,
                        0 as libc::c_int as bool_);
        }
        7 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Behold, mortal, how generously I reward thy loyalty.\'\x00"
                          as *const u8 as *const libc::c_char);
            acquirement((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int,
                        Rand_div(2 as libc::c_int) + 1 as libc::c_int +
                            1 as libc::c_int, 1 as libc::c_int as bool_,
                        0 as libc::c_int as bool_);
        }
        8 => {
            msg_format(b"The voice of %s thunders:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Thou art growing arrogant, mortal.\'\x00" as
                          *const u8 as *const libc::c_char);
            activate_ty_curse();
        }
        9 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'My pets, destroy the arrogant mortal!\'\x00" as
                          *const u8 as *const libc::c_char);
            dummy = 0 as libc::c_int;
            while dummy <
                      Rand_div(5 as libc::c_int) + 1 as libc::c_int +
                          1 as libc::c_int {
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int, 0 as libc::c_int);
                dummy += 1
            }
        }
        10 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Thou needst worthier opponents!\'\x00" as *const u8
                          as *const libc::c_char);
            activate_hi_summon();
        }
        11 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Death and destruction! This pleaseth me!\'\x00" as
                          *const u8 as *const libc::c_char);
            call_chaos();
        }
        12 => {
            msg_format(b"The voice of %s rings out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Stay, mortal, and let me mold thee.\'\x00" as
                          *const u8 as *const libc::c_char);
            if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int &&
                   !(chaos_stats[(*p_ptr).chaos_patron as usize] <
                         0 as libc::c_int) {
                do_inc_stat(chaos_stats[(*p_ptr).chaos_patron as usize]);
            } else {
                do_inc_stat(Rand_div(6 as libc::c_int) + 1 as libc::c_int -
                                1 as libc::c_int);
            }
        }
        13 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'I grow tired of thee, mortal.\'\x00" as *const u8 as
                          *const libc::c_char);
            if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int &&
                   !(chaos_stats[(*p_ptr).chaos_patron as usize] <
                         0 as libc::c_int) {
                do_dec_stat(chaos_stats[(*p_ptr).chaos_patron as usize],
                            2 as libc::c_int);
            } else {
                do_dec_stat(Rand_div(6 as libc::c_int) + 1 as libc::c_int -
                                1 as libc::c_int, 2 as libc::c_int);
            }
        }
        14 => {
            msg_format(b"The voice of %s thunders:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Thou needst a lesson in humility, mortal!\'\x00" as
                          *const u8 as *const libc::c_char);
            msg_print(b"You feel less powerful!\x00" as *const u8 as
                          *const libc::c_char);
            dummy = 0 as libc::c_int;
            while dummy < 6 as libc::c_int {
                dec_stat(dummy,
                         10 as libc::c_int +
                             (Rand_div(15 as libc::c_int) + 1 as libc::c_int),
                         1 as libc::c_int);
                dummy += 1
            }
        }
        16 => {
            msg_format(b"You feel the power of %s touch you.\x00" as *const u8
                           as *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            do_poly_wounds();
        }
        15 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Receive this modest gift from me!\'\x00" as
                          *const u8 as *const libc::c_char);
            dummy = 0 as libc::c_int;
            while dummy < 6 as libc::c_int { do_inc_stat(dummy); dummy += 1 }
        }
        18 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Suffer, pathetic fool!\'\x00" as *const u8 as
                          *const libc::c_char);
            fire_ball(81 as libc::c_int, 0 as libc::c_int,
                      (*p_ptr).lev as libc::c_int * 4 as libc::c_int,
                      4 as libc::c_int);
            take_hit((*p_ptr).lev as libc::c_int * 4 as libc::c_int,
                     wrath_reason.as_mut_ptr() as cptr);
        }
        17 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Rise, my servant!\'\x00" as *const u8 as
                          *const libc::c_char);
            restore_level();
            set_poisoned(0 as libc::c_int);
            set_blind(0 as libc::c_int);
            set_confused(0 as libc::c_int);
            set_image(0 as libc::c_int);
            set_stun(0 as libc::c_int);
            set_cut(0 as libc::c_int);
            hp_player(5000 as libc::c_int);
            dummy = 0 as libc::c_int;
            while dummy < 6 as libc::c_int {
                do_res_stat(dummy, 1 as libc::c_int as bool_);
                dummy += 1
            }
        }
        19 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Thou reliest too much on thy weapon.\'\x00" as
                          *const u8 as *const libc::c_char);
            curse_weapon();
        }
        20 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Thou reliest too much on thine equipment.\'\x00" as
                          *const u8 as *const libc::c_char);
            curse_armor();
        }
        21 => {
            msg_format(b"The voice of %s whispers:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Now thou shalt pay for annoying me.\'\x00" as
                          *const u8 as *const libc::c_char);
            match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
                1 => { activate_ty_curse(); }
                2 => { activate_hi_summon(); }
                3 => {
                    if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        curse_weapon();
                    } else { curse_armor(); }
                }
                _ => {
                    dummy = 0 as libc::c_int;
                    while dummy < 6 as libc::c_int {
                        dec_stat(dummy,
                                 10 as libc::c_int +
                                     (Rand_div(15 as libc::c_int) +
                                          1 as libc::c_int),
                                 1 as libc::c_int);
                        dummy += 1
                    }
                }
            }
        }
        22 => {
            msg_format(b"The voice of %s thunders:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Die, mortal!\'\x00" as *const u8 as
                          *const libc::c_char);
            take_hit((*p_ptr).lev as libc::c_int * 4 as libc::c_int,
                     wrath_reason.as_mut_ptr() as cptr);
            dummy = 0 as libc::c_int;
            while dummy < 6 as libc::c_int {
                dec_stat(dummy,
                         10 as libc::c_int +
                             (Rand_div(15 as libc::c_int) + 1 as libc::c_int),
                         0 as libc::c_int);
                dummy += 1
            }
            activate_hi_summon();
            activate_ty_curse();
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                curse_weapon();
            }
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                curse_armor();
            }
        }
        23 => {
            /* Prevent destruction of quest levels and town */
            if is_quest(dun_level as libc::c_int) == 0 &&
                   dun_level as libc::c_int != 0 {
                msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                               *const libc::c_char,
                           chaos_patrons[(*p_ptr).chaos_patron as usize]);
                msg_print(b"\'Death and destruction! This pleaseth me!\'\x00"
                              as *const u8 as *const libc::c_char);
                destroy_area((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int, 25 as libc::c_int,
                             1 as libc::c_int as bool_,
                             0 as libc::c_int as bool_);
            }
        }
        24 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Let me relieve thee of thine oppressors!\'\x00" as
                          *const u8 as *const libc::c_char);
            genocide(0 as libc::c_int as bool_);
        }
        25 => {
            msg_format(b"The voice of %s booms out:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_print(b"\'Let me relieve thee of thine oppressors!\'\x00" as
                          *const u8 as *const libc::c_char);
            mass_genocide(0 as libc::c_int as bool_);
        }
        26 => {
            msg_format(b"You can feel the power of %s assault your enemies!\x00"
                           as *const u8 as *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            dispel_monsters((*p_ptr).lev as libc::c_int * 4 as libc::c_int);
        }
        32 => {
            msg_format(b"%s ignores you.\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
        }
        34 => {
            msg_format(b"%s rewards you with a demonic servant!\x00" as
                           *const u8 as *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            if summon_specific_friendly((*p_ptr).py as libc::c_int,
                                        (*p_ptr).px as libc::c_int,
                                        dun_level as libc::c_int,
                                        16 as libc::c_int,
                                        0 as libc::c_int as bool_) == 0 {
                msg_print(b"Nobody ever turns up...\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        35 => {
            msg_format(b"%s rewards you with a servant!\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            if summon_specific_friendly((*p_ptr).py as libc::c_int,
                                        (*p_ptr).px as libc::c_int,
                                        dun_level as libc::c_int,
                                        46 as libc::c_int,
                                        0 as libc::c_int as bool_) == 0 {
                msg_print(b"Nobody ever turns up...\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        33 => {
            msg_format(b"%s rewards you with an undead servant!\x00" as
                           *const u8 as *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            if summon_specific_friendly((*p_ptr).py as libc::c_int,
                                        (*p_ptr).px as libc::c_int,
                                        dun_level as libc::c_int,
                                        17 as libc::c_int,
                                        0 as libc::c_int as bool_) == 0 {
                msg_print(b"Nobody ever turns up...\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        _ => {
            msg_format(b"The voice of %s stammers:\x00" as *const u8 as
                           *const libc::c_char,
                       chaos_patrons[(*p_ptr).chaos_patron as usize]);
            msg_format(b"\'Uh... uh... the answer\'s %d/%d, what\'s the question?\'\x00"
                           as *const u8 as *const libc::c_char, type_0,
                       effect);
        }
    };
}
/*
 * old -- from PsiAngband.
 */
#[no_mangle]
pub unsafe extern "C" fn tgt_pt(mut x: *mut libc::c_int,
                                mut y: *mut libc::c_int) -> bool_ {
    let mut ch: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut d: libc::c_int = 0;
    let mut cu: libc::c_int = 0;
    let mut cv: libc::c_int = 0;
    let mut screen_wid: libc::c_int = 0;
    let mut screen_hgt: libc::c_int = 0;
    let mut success: bool_ = 0 as libc::c_int as bool_;
    *x = (*p_ptr).px as libc::c_int;
    *y = (*p_ptr).py as libc::c_int;
    /* Get size */
    get_screen_size(&mut screen_wid, &mut screen_hgt);
    cu = (*(*Term).scr).cu as libc::c_int;
    cv = (*(*Term).scr).cv as libc::c_int;
    (*(*Term).scr).cu = 0 as libc::c_int as bool_;
    (*(*Term).scr).cv = 1 as libc::c_int as bool_;
    msg_print(b"Select a point and press space.\x00" as *const u8 as
                  *const libc::c_char);
    while ch as libc::c_int != 27 as libc::c_int &&
              ch as libc::c_int != ' ' as i32 {
        move_cursor_relative(*y, *x);
        ch = inkey();
        match ch as libc::c_int {
            27 => { }
            32 => { success = 1 as libc::c_int as bool_ }
            _ => {
                /* Look up the direction */
                d = get_keymap_dir(ch);
                if !(d == 0) {
                    *x += ddx[d as usize] as libc::c_int;
                    *y += ddy[d as usize] as libc::c_int;
                    /* Hack -- Verify x */
                    if *x >= cur_wid as libc::c_int - 1 as libc::c_int ||
                           *x >= panel_col_min as libc::c_int + screen_wid {
                        *x -= 1
                    } else if *x <= 0 as libc::c_int ||
                                  *x <= panel_col_min as libc::c_int {
                        *x += 1
                    }
                    /* Hack -- Verify y */
                    if *y >= cur_hgt as libc::c_int - 1 as libc::c_int ||
                           *y >= panel_row_min as libc::c_int + screen_hgt {
                        *y -= 1
                    } else if *y <= 0 as libc::c_int ||
                                  *y <= panel_row_min as libc::c_int {
                        *y += 1
                    }
                }
            }
        }
    }
    (*(*Term).scr).cu = cu as bool_;
    (*(*Term).scr).cv = cv as bool_;
    Term_fresh();
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn gain_random_corruption(mut choose_mut: libc::c_int)
 -> bool_ {
    exec_lua(b"gain_corruption()\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn lose_corruption(mut choose_mut: libc::c_int)
 -> bool_ {
    exec_lua(b"lose_corruption()\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn lose_all_corruptions() -> bool_ {
    exec_lua(b"lose_all_corruptions()\x00" as *const u8 as *const libc::c_char
                 as *mut libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn get_hack_dir(mut dp: *mut libc::c_int) -> bool_ {
    let mut dir: libc::c_int = 0;
    let mut p: cptr = 0 as *const libc::c_char;
    let mut command: libc::c_char = 0;
    /* Initialize */
    *dp = 0 as libc::c_int;
    /* Global direction */
    dir = 0 as libc::c_int;
    /* (No auto-targetting */
    /* Ask until satisfied */
    while dir == 0 {
        /* Choose a prompt */
        if target_okay() == 0 {
            p =
                b"Direction (\'*\' to choose a target, Escape to cancel)? \x00"
                    as *const u8 as *const libc::c_char
        } else {
            p =
                b"Direction (\'5\' for target, \'*\' to re-target, Escape to cancel)? \x00"
                    as *const u8 as *const libc::c_char
        }
        /* Get a command (or Cancel) */
        if get_com(p, &mut command) == 0 { break ; }
        /* Convert various keys to "standard" keys */
        match command as libc::c_int {
            84 | 116 | 46 | 53 | 48 => {
                /* Use current target */
                dir = 5 as libc::c_int
            }
            42 => {
                /* Set new target */
                if target_set(0x1 as libc::c_int) != 0 {
                    dir = 5 as libc::c_int
                }
            }
            _ => {
                /* Look up the direction */
                dir = get_keymap_dir(command)
            }
        }
        /* Verify requested targets */
        if dir == 5 as libc::c_int && target_okay() == 0 {
            dir = 0 as libc::c_int
        }
        /* Error */
        if dir == 0 { bell(); }
    }
    /* No direction */
    if dir == 0 { return 0 as libc::c_int as bool_ }
    /* Save the direction */
    command_dir = dir as s16b;
    /* Check for confusion */
    if (*p_ptr).confused != 0 {
        /* XXX XXX XXX */
		/* Random direction */
        dir = ddd[Rand_div(8 as libc::c_int) as usize] as libc::c_int
    }
    /* Notice confusion */
    if command_dir as libc::c_int != dir {
        /* Warn the user */
        msg_print(b"You are confused.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Save direction */
    *dp = dir;
    /* A "valid" direction was entered */
    return 1 as libc::c_int as bool_;
}
/*
 * Do we have at least one corruption?
 */
#[no_mangle]
pub unsafe extern "C" fn got_corruptions() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    max =
        exec_lua(b"return __corruptions_max\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char);
    i = 0 as libc::c_int;
    while i < max {
        if exec_lua(format(b"if test_depend_corrupt(%d) == TRUE then return TRUE else return FALSE end\x00"
                               as *const u8 as *const libc::c_char, i)) != 0 {
            return 1 as libc::c_int as bool_
        }
        i += 1
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Dump the corruption list
 */
#[no_mangle]
pub unsafe extern "C" fn dump_corruptions(mut fff: *mut FILE,
                                          mut color: bool_) {
    let mut i: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    if fff.is_null() { return }
    max =
        exec_lua(b"return __corruptions_max\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char);
    i = 0 as libc::c_int;
    while i < max {
        if exec_lua(format(b"if test_depend_corrupt(%d) == TRUE then return TRUE else return FALSE end\x00"
                               as *const u8 as *const libc::c_char, i)) != 0 {
            let mut c: libc::c_int =
                exec_lua(format(b"return __corruptions[%d].color\x00" as
                                    *const u8 as *const libc::c_char, i));
            if color != 0 {
                fprintf(fff,
                        b"#####%c%s:\n\x00" as *const u8 as
                            *const libc::c_char,
                        conv_color[c as usize] as libc::c_int,
                        string_exec_lua(format(b"return __corruptions[%d].name\x00"
                                                   as *const u8 as
                                                   *const libc::c_char, i)));
            } else {
                fprintf(fff, b"%s:\n\x00" as *const u8 as *const libc::c_char,
                        string_exec_lua(format(b"return __corruptions[%d].name\x00"
                                                   as *const u8 as
                                                   *const libc::c_char, i)));
            }
            fprintf(fff, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    string_exec_lua(format(b"return __corruptions[%d].desc\x00"
                                               as *const u8 as
                                               *const libc::c_char, i)));
        }
        i += 1
    };
}
/*
 * Set "p_ptr->grace", notice observable changes
 */
#[no_mangle]
pub unsafe extern "C" fn set_grace(mut v: s32b) {
    if v < -(300000 as libc::c_int) { v = -(300000 as libc::c_int) }
    if v > 300000 as libc::c_int { v = 300000 as libc::c_int }
    (*p_ptr).grace = v;
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x8000 as libc::c_long) as u32b;
    handle_stuff();
}
#[no_mangle]
pub unsafe extern "C" fn test_object_wish(mut name: *mut libc::c_char,
                                          mut o_ptr: *mut object_type,
                                          mut forge: *mut object_type,
                                          mut what: *mut libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jb: libc::c_int = 0;
    let mut save_aware: libc::c_int = 0;
    let mut buf: [libc::c_char; 200] = [0; 200];
    /* try all objects, this *IS* a very ugly and slow method :( */
    i = 0 as libc::c_int;
    while i < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(i as isize) as *mut object_kind;
        o_ptr = forge;
        if !((*k_ptr).name == 0) {
            if !((*k_ptr).flags3 as libc::c_long & 0x8000 as libc::c_long !=
                     0) {
                if !((*k_ptr).flags3 as libc::c_long & 0x800 as libc::c_long
                         != 0) {
                    if !((*k_ptr).tval as libc::c_int == 100 as libc::c_int) {
                        object_prep(o_ptr, i);
                        (*o_ptr).name1 = 0 as libc::c_int as byte_hack;
                        (*o_ptr).name2 = 0 as libc::c_int as s16b;
                        apply_magic(o_ptr, dun_level as libc::c_int,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_);
                        /* Hack : aware status must be restored after describing the item name */
                        save_aware = (*k_ptr).aware as libc::c_int;
                        object_aware(o_ptr);
                        object_known(o_ptr);
                        object_desc(buf.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                                    0 as libc::c_int);
                        strlower(buf.as_mut_ptr());
                        (*k_ptr).aware = save_aware as bool_;
                        if !strstr(name, buf.as_mut_ptr()).is_null() ||
                               (*o_ptr).tval as libc::c_int ==
                                   67 as libc::c_int &&
                                   !strstr(name,
                                           b"rod of\x00" as *const u8 as
                                               *const libc::c_char).is_null()
                           {
                            let mut current_block_40: u64;
                            /* try all ego */
                            j = max_e_idx as libc::c_int - 1 as libc::c_int;
                            while j >= 0 as libc::c_int {
                                let mut e_ptr: *mut ego_item_type =
                                    &mut *e_info.offset(j as isize) as
                                        *mut ego_item_type;
                                let mut ok: bool_ = 0 as libc::c_int as bool_;
                                if !(j != 0 && (*e_ptr).name == 0) {
                                    /* Must have the correct fields */
                                    if j != 0 {
                                        let mut z: libc::c_int = 0;
                                        z = 0 as libc::c_int;
                                        while z < 6 as libc::c_int {
                                            if (*e_ptr).tval[z as usize] as
                                                   libc::c_int ==
                                                   (*k_ptr).tval as
                                                       libc::c_int {
                                                if (*e_ptr).min_sval[z as
                                                                         usize]
                                                       as libc::c_int <=
                                                       (*k_ptr).sval as
                                                           libc::c_int &&
                                                       (*e_ptr).max_sval[z as
                                                                             usize]
                                                           as libc::c_int >=
                                                           (*k_ptr).sval as
                                                               libc::c_int {
                                                    ok =
                                                        1 as libc::c_int as
                                                            bool_
                                                }
                                            }
                                            if ok != 0 { break ; }
                                            z += 1
                                        }
                                        if ok == 0 {
                                            current_block_40 =
                                                2370887241019905314;
                                        } else {
                                            current_block_40 =
                                                9853141518545631134;
                                        }
                                    } else {
                                        current_block_40 =
                                            9853141518545631134;
                                    }
                                    match current_block_40 {
                                        2370887241019905314 => { }
                                        _ => {
                                            let mut current_block_39: u64;
                                            /* try all ego */
                                            jb =
                                                max_e_idx as libc::c_int -
                                                    1 as libc::c_int;
                                            while jb >= 0 as libc::c_int {
                                                let mut eb_ptr:
                                                        *mut ego_item_type =
                                                    &mut *e_info.offset(jb as
                                                                            isize)
                                                        as *mut ego_item_type;
                                                let mut ok_0: bool_ =
                                                    0 as libc::c_int as bool_;
                                                if !(jb != 0 &&
                                                         (*eb_ptr).name == 0)
                                                   {
                                                    if !(j != 0 && jb != 0 &&
                                                             (*e_ptr).before
                                                                 as
                                                                 libc::c_int
                                                                 ==
                                                                 (*eb_ptr).before
                                                                     as
                                                                     libc::c_int)
                                                       {
                                                        /* Must have the correct fields */
                                                        if jb != 0 {
                                                            let mut z_0:
                                                                    libc::c_int =
                                                                0;
                                                            z_0 =
                                                                0 as
                                                                    libc::c_int;
                                                            while z_0 <
                                                                      6 as
                                                                          libc::c_int
                                                                  {
                                                                if (*eb_ptr).tval[z_0
                                                                                      as
                                                                                      usize]
                                                                       as
                                                                       libc::c_int
                                                                       ==
                                                                       (*k_ptr).tval
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    if (*eb_ptr).min_sval[z_0
                                                                                              as
                                                                                              usize]
                                                                           as
                                                                           libc::c_int
                                                                           <=
                                                                           (*k_ptr).sval
                                                                               as
                                                                               libc::c_int
                                                                           &&
                                                                           (*eb_ptr).max_sval[z_0
                                                                                                  as
                                                                                                  usize]
                                                                               as
                                                                               libc::c_int
                                                                               >=
                                                                               (*k_ptr).sval
                                                                                   as
                                                                                   libc::c_int
                                                                       {
                                                                        ok_0 =
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                bool_
                                                                    }
                                                                }
                                                                if ok_0 != 0 {
                                                                    break ;
                                                                }
                                                                z_0 += 1
                                                            }
                                                            if ok_0 == 0 {
                                                                current_block_39
                                                                    =
                                                                    7245201122033322888;
                                                            } else {
                                                                current_block_39
                                                                    =
                                                                    12556861819962772176;
                                                            }
                                                        } else {
                                                            current_block_39 =
                                                                12556861819962772176;
                                                        }
                                                        match current_block_39
                                                            {
                                                            7245201122033322888
                                                            => {
                                                            }
                                                            _ => {
                                                                object_prep(o_ptr,
                                                                            i);
                                                                (*o_ptr).name1
                                                                    =
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        byte_hack;
                                                                (*o_ptr).name2
                                                                    =
                                                                    j as s16b;
                                                                (*o_ptr).name2b
                                                                    =
                                                                    jb as
                                                                        s16b;
                                                                apply_magic(o_ptr,
                                                                            dun_level
                                                                                as
                                                                                libc::c_int,
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                bool_,
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                bool_,
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                bool_);
                                                                object_aware(o_ptr);
                                                                object_known(o_ptr);
                                                                object_desc(buf.as_mut_ptr(),
                                                                            o_ptr,
                                                                            0
                                                                                as
                                                                                libc::c_int,
                                                                            0
                                                                                as
                                                                                libc::c_int);
                                                                strlower(buf.as_mut_ptr());
                                                                if strcasecmp(buf.as_mut_ptr(),
                                                                              name)
                                                                       == 0 {
                                                                    /* Don't search any more */
                                                                    return 1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               bool_
                                                                } else {
                                                                    /* Restore again the aware status */
                                                                    (*k_ptr).aware
                                                                        =
                                                                        save_aware
                                                                            as
                                                                            bool_
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                jb -= 1
                                            }
                                        }
                                    }
                                }
                                j -= 1
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn clean_wish_name(mut buf: *mut libc::c_char,
                                         mut name: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* Lowercase the wish */
    strlower(buf);
    /* Nuke uneccesary spaces */
    p = buf;
    while *p as libc::c_int == ' ' as i32 { p = p.offset(1) }
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while *p.offset(i as isize) != 0 {
        if *p.offset(i as isize) as libc::c_int == ' ' as i32 &&
               *p.offset((i + 1 as libc::c_int) as isize) as libc::c_int ==
                   ' ' as i32 {
            i += 1
        } else {
            let fresh1 = i;
            i = i + 1;
            let fresh2 = j;
            j = j + 1;
            *name.offset(fresh2 as isize) = *p.offset(fresh1 as isize)
        }
    }
    let fresh3 = j;
    j = j + 1;
    *name.offset(fresh3 as isize) = '\u{0}' as i32 as libc::c_char;
    if j != 0 {
        j -= 1;
        while j != 0 && *name.offset(j as isize) as libc::c_int == ' ' as i32
              {
            *name.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
            j -= 1
        }
    };
}
/*
 * Allow the player to make a wish
 */
#[no_mangle]
pub unsafe extern "C" fn make_wish() {
    let mut buf: [libc::c_char; 200] = [0; 200];
    let mut name: [libc::c_char; 200] = [0; 200];
    let mut mname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut mstatus: libc::c_int = -(2 as libc::c_int);
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
    /* Make an empty string */
    buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    /* Ask for the wish */
    if get_string(b"Wish for what? \x00" as *const u8 as *const libc::c_char,
                  buf.as_mut_ptr(), 80 as libc::c_int) == 0 {
        return
    }
    clean_wish_name(buf.as_mut_ptr(), name.as_mut_ptr());
    /* You can't wish for a wish! */
    if !strstr(name.as_mut_ptr(),
               b"wish\x00" as *const u8 as *const libc::c_char).is_null() {
        msg_print(b"You can\'t wish for a wish!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if test_object_wish(name.as_mut_ptr(), o_ptr, &mut forge,
                        b"wish\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char) != 0 {
        msg_print(b"Your wish becomes truth!\x00" as *const u8 as
                      *const libc::c_char);
        /* Give it to the player */
        drop_near(o_ptr, -(1 as libc::c_int), (*p_ptr).py as libc::c_int,
                  (*p_ptr).px as libc::c_int);
        return
    }
    /* try monsters */
    if prefix(name.as_mut_ptr() as cptr,
              b"enemy \x00" as *const u8 as *const libc::c_char) != 0 {
        mstatus = -(2 as libc::c_int);
        mname = name.as_mut_ptr().offset(6 as libc::c_int as isize)
    } else if prefix(name.as_mut_ptr() as cptr,
                     b"neutral \x00" as *const u8 as *const libc::c_char) != 0
     {
        mstatus = 0 as libc::c_int;
        mname = name.as_mut_ptr().offset(8 as libc::c_int as isize)
    } else if prefix(name.as_mut_ptr() as cptr,
                     b"friendly \x00" as *const u8 as *const libc::c_char) !=
                  0 {
        mstatus = 2 as libc::c_int;
        mname = name.as_mut_ptr().offset(9 as libc::c_int as isize)
    } else if prefix(name.as_mut_ptr() as cptr,
                     b"pet \x00" as *const u8 as *const libc::c_char) != 0 {
        mstatus = 3 as libc::c_int;
        mname = name.as_mut_ptr().offset(4 as libc::c_int as isize)
    } else if prefix(name.as_mut_ptr() as cptr,
                     b"companion \x00" as *const u8 as *const libc::c_char) !=
                  0 {
        if can_create_companion() != 0 {
            mstatus = 4 as libc::c_int
        } else { mstatus = 3 as libc::c_int }
        mname = name.as_mut_ptr().offset(10 as libc::c_int as isize)
    } else { mname = name.as_mut_ptr() }
    i = 1 as libc::c_int;
    while i < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(i as isize) as *mut monster_race;
        if !((*r_ptr).name == 0) {
            if !((*r_ptr).flags9 & 0x2000 as libc::c_int as libc::c_uint != 0)
               {
                if !((*r_ptr).flags9 & 0x4000 as libc::c_int as libc::c_uint
                         != 0) {
                    if !((*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint
                             != 0) {
                        sprintf(buf.as_mut_ptr(),
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                r_name.offset((*r_ptr).name as isize));
                        strlower(buf.as_mut_ptr());
                        if !strstr(mname, buf.as_mut_ptr()).is_null() {
                            /* try all ego */
                            j = max_re_idx as libc::c_int - 1 as libc::c_int;
                            while j >= 0 as libc::c_int {
                                let mut re_ptr: *mut monster_ego =
                                    &mut *re_info.offset(j as isize) as
                                        *mut monster_ego;
                                if !(j != 0 && (*re_ptr).name == 0) {
                                    if !(mego_ok(i, j) == 0) {
                                        if j != 0 {
                                            if (*re_ptr).before != 0 {
                                                sprintf(buf.as_mut_ptr(),
                                                        b"%s %s\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        re_name.offset((*re_ptr).name
                                                                           as
                                                                           isize),
                                                        r_name.offset((*r_ptr).name
                                                                          as
                                                                          isize));
                                            } else {
                                                sprintf(buf.as_mut_ptr(),
                                                        b"%s %s\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        r_name.offset((*r_ptr).name
                                                                          as
                                                                          isize),
                                                        re_name.offset((*re_ptr).name
                                                                           as
                                                                           isize));
                                            }
                                        } else {
                                            sprintf(buf.as_mut_ptr(),
                                                    b"%s\x00" as *const u8 as
                                                        *const libc::c_char,
                                                    r_name.offset((*r_ptr).name
                                                                      as
                                                                      isize));
                                        }
                                        strlower(buf.as_mut_ptr());
                                        if strcasecmp(mname, buf.as_mut_ptr())
                                               == 0 {
                                            let mut wy: libc::c_int =
                                                (*p_ptr).py as libc::c_int;
                                            let mut wx: libc::c_int =
                                                (*p_ptr).px as libc::c_int;
                                            let mut attempts: libc::c_int =
                                                100 as libc::c_int;
                                            loop  {
                                                scatter(&mut wy, &mut wx,
                                                        (*p_ptr).py as
                                                            libc::c_int,
                                                        (*p_ptr).px as
                                                            libc::c_int,
                                                        5 as libc::c_int);
                                                if !(!(wy > 0 as libc::c_int
                                                           &&
                                                           wx >
                                                               0 as
                                                                   libc::c_int
                                                           &&
                                                           wy <
                                                               cur_hgt as
                                                                   libc::c_int
                                                                   -
                                                                   1 as
                                                                       libc::c_int
                                                           &&
                                                           wx <
                                                               cur_wid as
                                                                   libc::c_int
                                                                   -
                                                                   1 as
                                                                       libc::c_int
                                                           &&
                                                           ((*f_info.offset((*cave[wy
                                                                                       as
                                                                                       usize].offset(wx
                                                                                                         as
                                                                                                         isize)).feat
                                                                                as
                                                                                isize)).flags1
                                                                as
                                                                libc::c_long &
                                                                0x10 as
                                                                    libc::c_long
                                                                != 0 &&
                                                                (*cave[wy as
                                                                           usize].offset(wx
                                                                                             as
                                                                                             isize)).feat
                                                                    as
                                                                    libc::c_int
                                                                    !=
                                                                    0xaf as
                                                                        libc::c_int))
                                                         &&
                                                         {
                                                             attempts -= 1;
                                                             (attempts) != 0
                                                         }) {
                                                    break ;
                                                }
                                            }
                                            /* Create the monster */
                                            if place_monster_one(wy, wx, i, j,
                                                                 0 as
                                                                     libc::c_int
                                                                     as bool_,
                                                                 mstatus) != 0
                                               {
                                                msg_print(b"Your wish becomes truth!\x00"
                                                              as *const u8 as
                                                              *const libc::c_char);
                                            }
                                            /* Don't search any more */
                                            return
                                        }
                                    }
                                }
                                j -= 1
                            }
                        }
                    }
                }
            }
        }
        i += 1
    };
}
/*
 * Corrupted have a 1/3 chance of losing a mutation each time this is called, 
 * assuming they have any in the first place
 */
#[no_mangle]
pub unsafe extern "C" fn corrupt_corrupted() {
    if Rand_div(100 as libc::c_int) < 45 as libc::c_int {
        lose_corruption(0 as libc::c_int);
    } else { gain_random_corruption(0 as libc::c_int); };
}
/*
 * Change to an other class
 */
#[no_mangle]
pub unsafe extern "C" fn switch_class(mut sclass: libc::c_int) {
    (*p_ptr).pclass = sclass as byte_hack;
    cp_ptr =
        &mut *class_info.offset((*p_ptr).pclass as isize) as
            *mut player_class;
}
/*
 * Change to an other subclass
 */
#[no_mangle]
pub unsafe extern "C" fn switch_subclass(mut sclass: libc::c_int) {
    (*p_ptr).pspec = sclass as byte_hack;
    spp_ptr =
        &mut *(*class_info.offset((*p_ptr).pclass as
                                      isize)).spec.as_mut_ptr().offset((*p_ptr).pspec
                                                                           as
                                                                           isize)
            as *mut player_spec;
}
/*
 * Change to an other subrace
 */
#[no_mangle]
pub unsafe extern "C" fn switch_subrace(mut racem: libc::c_int,
                                        mut copy_old: bool_) {
    if racem < 0 as libc::c_int && racem >= max_rmp_idx as libc::c_int {
        return
    }
    /* If we switch to the saved subrace, we copy over the old subrace data */
    if copy_old as libc::c_int != 0 && racem == 9 as libc::c_int {
        let mut old_title: s32b =
            (*race_mod_info.offset(9 as libc::c_int as isize)).title;
        let mut old_desc: s32b =
            (*race_mod_info.offset(9 as libc::c_int as isize)).desc;
        memcpy(&mut *race_mod_info.offset(9 as libc::c_int as isize) as
                   *mut player_race_mod as *mut libc::c_char as
                   *mut libc::c_void,
               &mut *race_mod_info.offset((*p_ptr).pracem as isize) as
                   *mut player_race_mod as *mut libc::c_char as
                   *const libc::c_void,
               ::std::mem::size_of::<player_race_mod>() as libc::c_ulong);
        (*race_mod_info.offset(9 as libc::c_int as isize)).title = old_title;
        (*race_mod_info.offset(9 as libc::c_int as isize)).desc = old_desc;
        strcpy(rmp_name.offset((*race_mod_info.offset(9 as libc::c_int as
                                                          isize)).title as
                                   isize),
               rmp_name.offset((*race_mod_info.offset((*p_ptr).pracem as
                                                          isize)).title as
                                   isize));
    }
    (*p_ptr).pracem = racem as byte_hack;
    rmp_ptr =
        &mut *race_mod_info.offset((*p_ptr).pracem as isize) as
            *mut player_race_mod;
}
#[no_mangle]
pub unsafe extern "C" fn get_subrace_title(mut racem: libc::c_int) -> cptr {
    return rmp_name.offset((*race_mod_info.offset(racem as isize)).title as
                               isize) as cptr;
}
#[no_mangle]
pub unsafe extern "C" fn set_subrace_title(mut racem: libc::c_int,
                                           mut name: cptr) {
    strcpy(rmp_name.offset((*race_mod_info.offset(racem as isize)).title as
                               isize), name);
}
/*
 * Rebirth, recalc hp & exp/level
 */
#[no_mangle]
pub unsafe extern "C" fn do_rebirth() {
    /* Experience factor */
    (*p_ptr).expfact =
        ((*rp_ptr).r_exp as libc::c_int + (*rmp_ptr).r_exp as libc::c_int +
             (*cp_ptr).c_exp as libc::c_int) as u16b;
    /* Hitdice */
    (*p_ptr).hitdie =
        ((*rp_ptr).r_mhp as libc::c_int + (*rmp_ptr).r_mhp as libc::c_int +
             (*cp_ptr).c_mhp as libc::c_int) as byte_hack;
    /* Recalc HP */
    do_cmd_rerate();
    /* Change the level if needed */
    check_experience();
    (*p_ptr).max_plv = (*p_ptr).lev;
    /* Redraw/calc stuff */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x2000000 as libc::c_long) as u32b;
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    handle_stuff();
    lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
}
/*
 * Quick mimic name to index function
 */
#[no_mangle]
pub unsafe extern "C" fn resolve_mimic_name(mut name: cptr) -> libc::c_int {
    let mut idx: s32b = 0;
    call_lua(b"resolve_mimic_name\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char, name,
             &mut idx as *mut s32b);
    return idx;
}
