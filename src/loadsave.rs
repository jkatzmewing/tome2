use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut sex_info: [player_sex; 3];
    #[no_mangle]
    static mut option_info: [option_type; 0];
    #[no_mangle]
    static mut inscription_info: [inscription_info_type; 8];
    #[no_mangle]
    static mut sf_major: byte_hack;
    #[no_mangle]
    static mut sf_minor: byte_hack;
    #[no_mangle]
    static mut sf_patch: byte_hack;
    #[no_mangle]
    static mut sf_extra: byte_hack;
    #[no_mangle]
    static mut sf_xtra: u32b;
    #[no_mangle]
    static mut sf_when: u32b;
    #[no_mangle]
    static mut sf_lives: u16b;
    #[no_mangle]
    static mut sf_saves: u16b;
    #[no_mangle]
    static mut vernum: u32b;
    #[no_mangle]
    static mut arg_fiddle: bool_;
    #[no_mangle]
    static mut arg_wizard: bool_;
    #[no_mangle]
    static mut character_dungeon: bool_;
    #[no_mangle]
    static mut character_loaded: bool_;
    #[no_mangle]
    static mut character_saved: bool_;
    #[no_mangle]
    static mut seed_flavor: u32b;
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
    static mut turn: s32b;
    #[no_mangle]
    static mut old_turn: s32b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut total_winner: u16b;
    #[no_mangle]
    static mut has_won: u16b;
    #[no_mangle]
    static mut noscore: u16b;
    #[no_mangle]
    static mut inven_cnt: s16b;
    #[no_mangle]
    static mut equip_cnt: s16b;
    #[no_mangle]
    static mut o_max: s16b;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut cheat_peek: bool_;
    #[no_mangle]
    static mut cheat_hear: bool_;
    #[no_mangle]
    static mut cheat_room: bool_;
    #[no_mangle]
    static mut cheat_xtra: bool_;
    #[no_mangle]
    static mut cheat_know: bool_;
    #[no_mangle]
    static mut cheat_live: bool_;
    #[no_mangle]
    static mut hitpoint_warn: byte_hack;
    #[no_mangle]
    static mut delay_factor: byte_hack;
    #[no_mangle]
    static mut autosave_freq: s16b;
    #[no_mangle]
    static mut autosave_t: bool_;
    #[no_mangle]
    static mut autosave_l: bool_;
    #[no_mangle]
    static mut feeling: s16b;
    #[no_mangle]
    static mut max_panel_rows: s16b;
    #[no_mangle]
    static mut max_panel_cols: s16b;
    #[no_mangle]
    static mut floor_type: [s16b; 100];
    #[no_mangle]
    static mut fill_type: [s16b; 100];
    #[no_mangle]
    static mut player_name: [libc::c_char; 32];
    #[no_mangle]
    static mut player_base: [libc::c_char; 32];
    #[no_mangle]
    static mut died_from: [libc::c_char; 80];
    #[no_mangle]
    static mut history: [[libc::c_char; 60]; 4];
    #[no_mangle]
    static mut savefile: [libc::c_char; 1024];
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    static mut Rand_quick: bool_;
    #[no_mangle]
    static mut Rand_place: u16b;
    #[no_mangle]
    static mut Rand_state: [u32b; 63];
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut option_flag: [u32b; 8];
    #[no_mangle]
    static mut option_mask: [u32b; 8];
    #[no_mangle]
    static mut window_flag: [u32b; 8];
    #[no_mangle]
    static mut window_mask: [u32b; 8];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut km_list: *mut monster_type;
    #[no_mangle]
    static mut max_towns: u16b;
    #[no_mangle]
    static mut town_info: *mut town_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut sp_ptr: *mut player_sex;
    #[no_mangle]
    static mut rp_ptr: *mut player_race;
    #[no_mangle]
    static mut rmp_ptr: *mut player_race_mod;
    #[no_mangle]
    static mut cp_ptr: *mut player_class;
    #[no_mangle]
    static mut spp_ptr: *mut player_spec;
    #[no_mangle]
    static mut alchemist_known_egos: [u32b; 32];
    #[no_mangle]
    static mut alchemist_known_artifacts: [u32b; 6];
    #[no_mangle]
    static mut alchemist_gained: u32b;
    #[no_mangle]
    static mut player_hp: [s16b; 50];
    #[no_mangle]
    static mut ab_info: *mut ability_type;
    #[no_mangle]
    static mut s_info: *mut skill_type;
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
    static mut class_info: *mut player_class;
    #[no_mangle]
    static mut race_info: *mut player_race;
    #[no_mangle]
    static mut race_mod_info: *mut player_race_mod;
    #[no_mangle]
    static mut rmp_name: *mut libc::c_char;
    #[no_mangle]
    static mut rmp_text: *mut libc::c_char;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut ANGBAND_DIR_SAVE: cptr;
    #[no_mangle]
    static mut max_st_idx: u16b;
    #[no_mangle]
    static mut old_max_s_idx: u16b;
    #[no_mangle]
    static mut max_ab_idx: u16b;
    #[no_mangle]
    static mut max_s_idx: u16b;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_a_idx: u16b;
    #[no_mangle]
    static mut max_d_idx: u16b;
    #[no_mangle]
    static mut max_o_idx: u16b;
    #[no_mangle]
    static mut max_m_idx: u16b;
    #[no_mangle]
    static mut max_t_idx: u16b;
    #[no_mangle]
    static mut max_wild_x: u16b;
    #[no_mangle]
    static mut init_flags: libc::c_int;
    #[no_mangle]
    static mut ambush_flag: bool_;
    #[no_mangle]
    static mut fate_flag: bool_;
    #[no_mangle]
    static mut no_breeds: s16b;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut bounties: [[s16b; 2]; 24];
    #[no_mangle]
    static mut random_spells: [random_spell; 100];
    #[no_mangle]
    static mut spell_num: s16b;
    #[no_mangle]
    static mut rune_spells: [rune_spell; 100];
    #[no_mangle]
    static mut rune_num: s16b;
    #[no_mangle]
    static mut fates: [fate; 200];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut total_bounties: u32b;
    #[no_mangle]
    static mut plots: [s16b; 7];
    #[no_mangle]
    static mut random_quests: [random_quest; 99];
    #[no_mangle]
    static mut special_lvl: [*mut bool_; 128];
    #[no_mangle]
    static mut generate_special_feeling: bool_;
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut previous_char: birther;
    #[no_mangle]
    static mut extra_savefile_parts: s32b;
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    static mut effects: [effect_type; 128];
    #[no_mangle]
    static mut max_corruptions: s16b;
    #[no_mangle]
    static mut automatizer_enabled: bool_;
    #[no_mangle]
    static mut last_teleportation_y: s16b;
    #[no_mangle]
    static mut game_module: cptr;
    #[no_mangle]
    static mut VERSION_MAJOR: s32b;
    #[no_mangle]
    static mut VERSION_MINOR: s32b;
    #[no_mangle]
    static mut VERSION_PATCH: s32b;
    #[no_mangle]
    static mut process_hooks_return: [hook_return; 20];
    #[no_mangle]
    fn process_hooks_ret(h_idx: libc::c_int, ret: *mut libc::c_char,
                         fmt: *mut libc::c_char, _: ...) -> bool_;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn save_savefile_names();
    #[no_mangle]
    fn process_dungeon_file(name: cptr, yval: *mut libc::c_int,
                            xval: *mut libc::c_int, ymax: libc::c_int,
                            xmax: libc::c_int, init: bool_, full: bool_)
     -> errr;
    #[no_mangle]
    fn create_stores_stock(t: libc::c_int);
    #[no_mangle]
    fn fd_close(fd: libc::c_int) -> errr;
    #[no_mangle]
    fn fd_open(file: cptr, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn my_fclose(fff_0: *mut FILE) -> errr;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn m_pop() -> s16b;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn quark_str(num: s16b) -> cptr;
    #[no_mangle]
    fn quark_add(str: cptr) -> s16b;
    #[no_mangle]
    fn o_pop() -> s16b;
    #[no_mangle]
    fn compact_monsters(size: libc::c_int);
    #[no_mangle]
    fn compact_objects(size: libc::c_int);
    #[no_mangle]
    static mut max_wild_y: u16b;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn get_dungeon_save(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn fd_kill(file: cptr) -> errr;
    #[no_mangle]
    fn fd_move(file: cptr, what: cptr) -> errr;
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn wield_set(a_idx: s16b, set_idx: s16b, silent: bool_) -> bool_;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn message_type(age: libc::c_int) -> byte_hack;
    #[no_mangle]
    fn message_color(age: libc::c_int) -> byte_hack;
    #[no_mangle]
    fn message_str(age: libc::c_int) -> cptr;
    #[no_mangle]
    fn message_add(type_0: byte_hack, msg: cptr, color: byte_hack);
    #[no_mangle]
    fn message_num() -> s16b;
    #[no_mangle]
    fn fd_make(file: cptr, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
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
pub struct obj_theme {
    pub treasure: byte_hack,
    pub combat: byte_hack,
    pub magic: byte_hack,
    pub tools: byte_hack,
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
pub struct effect_type {
    pub time: s16b,
    pub dam: s16b,
    pub type_0: s16b,
    pub cy: s16b,
    pub cx: s16b,
    pub rad: s16b,
    pub flags: u32b,
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
pub struct option_type {
    pub o_var: *mut bool_,
    pub o_norm: byte_hack,
    pub o_page: byte_hack,
    pub o_bit: byte_hack,
    pub o_text: cptr,
    pub o_desc: cptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct store_type {
    pub st_idx: u16b,
    pub owner: u16b,
    pub insult_cur: s16b,
    pub good_buy: s16b,
    pub bad_buy: s16b,
    pub store_open: s32b,
    pub last_visit: s32b,
    pub stock_num: byte_hack,
    pub stock_size: s16b,
    pub stock: *mut object_type,
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
pub struct wilderness_map {
    pub feat: libc::c_int,
    pub seed: u32b,
    pub entrance: u16b,
    pub known: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct town_type {
    pub name: cptr,
    pub seed: u32b,
    pub store: *mut store_type,
    pub numstores: byte_hack,
    pub flags: byte_hack,
    pub stocked: bool_,
    pub destroyed: bool_,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random_quest {
    pub type_0: byte_hack,
    pub r_idx: s16b,
    pub done: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct birther {
    pub sex: s16b,
    pub race: s16b,
    pub rmod: s16b,
    pub pclass: s16b,
    pub spec: s16b,
    pub quests: byte_hack,
    pub god: byte_hack,
    pub grace: s32b,
    pub god_favor: s32b,
    pub age: s16b,
    pub wt: s16b,
    pub ht: s16b,
    pub sc: s16b,
    pub au: s32b,
    pub stat: [s16b; 6],
    pub luck: s16b,
    pub chaos_patron: s16b,
    pub weapon: u32b,
    pub history: [[libc::c_char; 60]; 4],
    pub quick_ok: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union hook_return {
    pub num: s32b,
    pub str_0: cptr,
    pub o_ptr: *mut object_type,
    pub m_ptr: *mut monster_type,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ability_type {
    pub name: u32b,
    pub desc: u32b,
    pub action_desc: u32b,
    pub action_mkey: s16b,
    pub cost: s16b,
    pub acquired: bool_,
    pub skills: [s16b; 10],
    pub skill_levels: [s16b; 10],
    pub stat: [s16b; 6],
    pub need_abilities: [s16b; 10],
    pub forbid_abilities: [s16b; 10],
}
static mut fff: *mut FILE = 0 as *const FILE as *mut FILE;
/* Local savefile ptr */
/*
 * Basic byte-level reading from savefile. This provides a single point
 * of interface to the pseudoencryption that ToME (and Angband)
 * uses. I'm thinking about if it might be faster/better to modify all
 * the do_* functions to directly do this stuff -- it'd make the code
 * somewhat uglier to maintain, but concievably might be much faster. Or
 * is it better maybe to scrap the pseudoencryption entirely and adopt
 * some other means of obfuscation, should it still prove useful in any
 * way? -- Improv
 *
 * What's the point of encryption on savefiles anyway? If I wanted to
 * make a cheater savefile, I'd activate debug mode, and hack the game
 * not to save it. There's no point. -- takkaria
 */
unsafe extern "C" fn sf_get() -> byte_hack {
    let mut c: byte_hack = 0;
    /* Get a character, decode the value */
    c = (getc(fff) & 0xff as libc::c_int) as byte_hack;
    /* Return the value */
    return c;
}
unsafe extern "C" fn sf_put(mut v: byte_hack) { putc(v as libc::c_int, fff); }
/*
 * Do object memory and similar stuff
 */
unsafe extern "C" fn do_xtra(mut k_idx: libc::c_int, mut flag: libc::c_int) {
    let mut tmp8u: byte_hack = 0 as libc::c_int as byte_hack;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(k_idx as isize) as *mut object_kind;
    if flag == 7 as libc::c_int {
        if (*k_ptr).aware != 0 {
            tmp8u = (tmp8u as libc::c_int | 0x1 as libc::c_int) as byte_hack
        }
        if (*k_ptr).tried != 0 {
            tmp8u = (tmp8u as libc::c_int | 0x2 as libc::c_int) as byte_hack
        }
        if (*k_ptr).know != 0 {
            tmp8u = (tmp8u as libc::c_int | 0x4 as libc::c_int) as byte_hack
        }
        if (*k_ptr).artifact != 0 {
            tmp8u = (tmp8u as libc::c_int | 0x80 as libc::c_int) as byte_hack
        }
        do_byte(&mut tmp8u, flag);
    }
    if flag == 3 as libc::c_int {
        do_byte(&mut tmp8u, flag);
        (*k_ptr).aware =
            if tmp8u as libc::c_int & 0x1 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_;
        (*k_ptr).tried =
            if tmp8u as libc::c_int & 0x2 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_;
        (*k_ptr).know =
            if tmp8u as libc::c_int & 0x4 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_;
        (*k_ptr).artifact =
            if tmp8u as libc::c_int & 0x80 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_
    };
}
/*
 * Load/Save quick start data
 */
#[no_mangle]
pub unsafe extern "C" fn do_quick_start(mut flag: libc::c_int) {
    let mut i: libc::c_int = 0;
    do_s16b(&mut previous_char.sex, flag);
    do_s16b(&mut previous_char.race, flag);
    do_s16b(&mut previous_char.rmod, flag);
    do_s16b(&mut previous_char.pclass, flag);
    do_s16b(&mut previous_char.spec, flag);
    do_byte(&mut previous_char.quests, flag);
    do_byte(&mut previous_char.god, flag);
    do_s32b(&mut previous_char.grace, flag);
    do_s16b(&mut previous_char.age, flag);
    do_s16b(&mut previous_char.wt, flag);
    do_s16b(&mut previous_char.ht, flag);
    do_s16b(&mut previous_char.sc, flag);
    do_s32b(&mut previous_char.au, flag);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        do_s16b(&mut *previous_char.stat.as_mut_ptr().offset(i as isize),
                flag);
        i += 1
    }
    do_s16b(&mut previous_char.luck, flag);
    do_s16b(&mut previous_char.chaos_patron, flag);
    do_u32b(&mut previous_char.weapon, flag);
    do_byte(&mut previous_char.quick_ok as *mut bool_ as *mut byte_hack,
            flag);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        do_string(previous_char.history[i as usize].as_mut_ptr(),
                  60 as libc::c_int, flag);
        i += 1
    };
}
/*
 * The special saved subrace
 */
unsafe extern "C" fn do_subrace(mut flag: libc::c_int) {
    let mut sr_ptr: *mut player_race_mod =
        &mut *race_mod_info.offset(9 as libc::c_int as isize) as
            *mut player_race_mod;
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 81] = [0; 81];
    if flag == 7 as libc::c_int {
        strncpy(buf.as_mut_ptr(), rmp_name.offset((*sr_ptr).title as isize),
                80 as libc::c_int as libc::c_ulong);
    }
    do_string(buf.as_mut_ptr(), 80 as libc::c_int, flag);
    if flag == 3 as libc::c_int {
        strncpy(rmp_name.offset((*sr_ptr).title as isize), buf.as_mut_ptr(),
                80 as libc::c_int as libc::c_ulong);
    }
    if flag == 7 as libc::c_int {
        strncpy(buf.as_mut_ptr(), rmp_text.offset((*sr_ptr).desc as isize),
                80 as libc::c_int as libc::c_ulong);
    }
    do_string(buf.as_mut_ptr(), 80 as libc::c_int, flag);
    if flag == 3 as libc::c_int {
        strncpy(rmp_text.offset((*sr_ptr).desc as isize), buf.as_mut_ptr(),
                80 as libc::c_int as libc::c_ulong);
    }
    do_byte(&mut (*sr_ptr).place as *mut bool_ as *mut byte_hack, flag);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        do_s16b(&mut *(*sr_ptr).r_adj.as_mut_ptr().offset(i as isize), flag);
        i += 1
    }
    do_byte(&mut (*sr_ptr).luck as *mut libc::c_char as *mut byte_hack, flag);
    do_s16b(&mut (*sr_ptr).mana, flag);
    do_s16b(&mut (*sr_ptr).r_dis, flag);
    do_s16b(&mut (*sr_ptr).r_dev, flag);
    do_s16b(&mut (*sr_ptr).r_sav, flag);
    do_s16b(&mut (*sr_ptr).r_stl, flag);
    do_s16b(&mut (*sr_ptr).r_srh, flag);
    do_s16b(&mut (*sr_ptr).r_fos, flag);
    do_s16b(&mut (*sr_ptr).r_thn, flag);
    do_s16b(&mut (*sr_ptr).r_thb, flag);
    do_byte(&mut (*sr_ptr).r_mhp as *mut libc::c_char as *mut byte_hack,
            flag);
    do_s16b(&mut (*sr_ptr).r_exp, flag);
    do_byte(&mut (*sr_ptr).b_age as *mut libc::c_char as *mut byte_hack,
            flag);
    do_byte(&mut (*sr_ptr).m_age as *mut libc::c_char as *mut byte_hack,
            flag);
    do_byte(&mut (*sr_ptr).m_b_ht as *mut libc::c_char as *mut byte_hack,
            flag);
    do_byte(&mut (*sr_ptr).m_m_ht as *mut libc::c_char as *mut byte_hack,
            flag);
    do_byte(&mut (*sr_ptr).f_b_ht as *mut libc::c_char as *mut byte_hack,
            flag);
    do_byte(&mut (*sr_ptr).f_m_ht as *mut libc::c_char as *mut byte_hack,
            flag);
    do_byte(&mut (*sr_ptr).m_b_wt as *mut libc::c_char as *mut byte_hack,
            flag);
    do_byte(&mut (*sr_ptr).m_m_wt as *mut libc::c_char as *mut byte_hack,
            flag);
    do_byte(&mut (*sr_ptr).f_b_wt as *mut libc::c_char as *mut byte_hack,
            flag);
    do_byte(&mut (*sr_ptr).f_m_wt as *mut libc::c_char as *mut byte_hack,
            flag);
    do_byte(&mut (*sr_ptr).infra as *mut libc::c_char as *mut byte_hack,
            flag);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        do_s16b(&mut *(*sr_ptr).powers.as_mut_ptr().offset(i as isize), flag);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        do_byte(&mut *(*sr_ptr).body_parts.as_mut_ptr().offset(i as isize) as
                    *mut libc::c_char as *mut byte_hack, flag);
        i += 1
    }
    do_u32b(&mut (*sr_ptr).flags1, flag);
    do_u32b(&mut (*sr_ptr).flags2, flag);
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int + 1 as libc::c_int {
        do_u32b(&mut *(*sr_ptr).oflags1.as_mut_ptr().offset(i as isize),
                flag);
        do_u32b(&mut *(*sr_ptr).oflags2.as_mut_ptr().offset(i as isize),
                flag);
        do_u32b(&mut *(*sr_ptr).oflags3.as_mut_ptr().offset(i as isize),
                flag);
        do_u32b(&mut *(*sr_ptr).oflags4.as_mut_ptr().offset(i as isize),
                flag);
        do_u32b(&mut *(*sr_ptr).oflags5.as_mut_ptr().offset(i as isize),
                flag);
        do_u32b(&mut *(*sr_ptr).oesp.as_mut_ptr().offset(i as isize), flag);
        do_s16b(&mut *(*sr_ptr).opval.as_mut_ptr().offset(i as isize), flag);
        i += 1
    }
    do_byte(&mut (*sr_ptr).g_attr, flag);
    do_byte(&mut (*sr_ptr).g_char as *mut libc::c_char as *mut byte_hack,
            flag);
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        do_byte(&mut *(*sr_ptr).skill_basem.as_mut_ptr().offset(i as isize) as
                    *mut libc::c_char as *mut byte_hack, flag);
        do_u32b(&mut *(*sr_ptr).skill_base.as_mut_ptr().offset(i as isize),
                flag);
        do_byte(&mut *(*sr_ptr).skill_modm.as_mut_ptr().offset(i as isize) as
                    *mut libc::c_char as *mut byte_hack, flag);
        do_s16b(&mut *(*sr_ptr).skill_mod.as_mut_ptr().offset(i as isize),
                flag);
        i += 1
    };
}
/*
 * Misc. other data
 */
static mut loaded_game_module: [libc::c_char; 80] = [0; 80];
unsafe extern "C" fn do_extra(mut flag: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp8u: byte_hack = 0;
    let mut tmp16s: s16b = 0;
    let mut tmp32u: u32b = 0;
    let mut tmp16b: u16b = 0;
    let mut dummy32u: u32b = 0 as libc::c_int as u32b;
    do_string(player_name.as_mut_ptr(), 32 as libc::c_int, flag);
    do_string(died_from.as_mut_ptr(), 80 as libc::c_int, flag);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        do_string(history[i as usize].as_mut_ptr(), 60 as libc::c_int, flag);
        i += 1
    }
    /* Handle the special levels info */
    if flag == 7 as libc::c_int {
        tmp8u = max_d_idx as byte_hack;
        tmp16s = 128 as libc::c_int as s16b
    }
    do_byte(&mut tmp8u, flag);
    if flag == 3 as libc::c_int {
        if tmp8u as libc::c_int > max_d_idx as libc::c_int {
            note(format(b"Too many (%d) dungeon types!\x00" as *const u8 as
                            *const libc::c_char, tmp8u as libc::c_int) as
                     cptr);
        }
    }
    do_s16b(&mut tmp16s, flag);
    if flag == 3 as libc::c_int {
        if tmp16s as libc::c_int > 128 as libc::c_int {
            note(format(b"Too many (%d) max level by dungeon type!\x00" as
                            *const u8 as *const libc::c_char,
                        tmp16s as libc::c_int) as cptr);
        }
    }
    /* Load the special levels history */
    i = 0 as libc::c_int;
    while i < tmp8u as libc::c_int {
        j = 0 as libc::c_int;
        while j < tmp16s as libc::c_int {
            do_byte(&mut *(*special_lvl.as_mut_ptr().offset(j as
                                                                isize)).offset(i
                                                                                   as
                                                                                   isize)
                        as *mut bool_ as *mut byte_hack, flag);
            j += 1
        }
        i += 1
    }
    do_byte(&mut generate_special_feeling as *mut bool_ as *mut byte_hack,
            flag);
    /* Load the quick start data */
    do_quick_start(flag);
    /* Load/save the special subrace */
    do_subrace(flag);
    /* Race/Class/Gender/Spells */
    do_s32b(&mut (*p_ptr).lives, flag);
    do_byte(&mut (*p_ptr).prace, flag);
    do_byte(&mut (*p_ptr).pracem, flag);
    do_byte(&mut (*p_ptr).pclass, flag);
    do_byte(&mut (*p_ptr).pspec, flag);
    do_byte(&mut (*p_ptr).psex, flag);
    do_u16b(&mut tmp16b, flag);
    do_u16b(&mut tmp16b, flag);
    do_byte(&mut (*p_ptr).mimic_form, flag);
    do_s16b(&mut (*p_ptr).mimic_level, flag);
    if flag == 7 as libc::c_int { tmp8u = 0 as libc::c_int as byte_hack }
    do_byte(&mut (*p_ptr).hitdie, flag);
    do_u16b(&mut (*p_ptr).expfact, flag);
    do_s16b(&mut (*p_ptr).age, flag);
    do_s16b(&mut (*p_ptr).ht, flag);
    do_s16b(&mut (*p_ptr).wt, flag);
    /* Dump the stats (maximum and current) */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        do_s16b(&mut *(*p_ptr).stat_max.as_mut_ptr().offset(i as isize),
                flag);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        do_s16b(&mut *(*p_ptr).stat_cur.as_mut_ptr().offset(i as isize),
                flag);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        do_s16b(&mut *(*p_ptr).stat_cnt.as_mut_ptr().offset(i as isize),
                flag);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        do_s16b(&mut *(*p_ptr).stat_los.as_mut_ptr().offset(i as isize),
                flag);
        i += 1
    }
    /* Dump the skills */
    do_s16b(&mut (*p_ptr).skill_points, flag);
    do_s16b(&mut (*p_ptr).skill_last_level, flag);
    do_s16b(&mut (*p_ptr).melee_style, flag);
    do_s16b(&mut (*p_ptr).use_piercing_shots, flag);
    tmp16s = 200 as libc::c_int as s16b;
    do_s16b(&mut tmp16s, flag);
    if flag == 3 as libc::c_int && tmp16s as libc::c_int > 200 as libc::c_int
       {
        quit(b"Too many skills\x00" as *const u8 as *const libc::c_char);
    }
    if flag == 7 as libc::c_int { old_max_s_idx = max_s_idx }
    do_u16b(&mut old_max_s_idx, flag);
    i = 0 as libc::c_int;
    while i < tmp16s as libc::c_int {
        if i < old_max_s_idx as libc::c_int {
            do_s32b(&mut (*s_info.offset(i as isize)).value, flag);
            do_s32b(&mut (*s_info.offset(i as isize)).mod_0, flag);
            do_byte(&mut (*s_info.offset(i as isize)).dev as *mut bool_ as
                        *mut byte_hack, flag);
            do_byte(&mut (*s_info.offset(i as isize)).hidden as *mut bool_ as
                        *mut byte_hack, flag);
            do_u32b(&mut (*s_info.offset(i as isize)).uses, flag);
        } else {
            do_u32b(&mut tmp32u, flag);
            do_s16b(&mut tmp16s, flag);
            do_byte(&mut tmp8u, flag);
            do_byte(&mut tmp8u, flag);
            do_u32b(&mut tmp32u, flag);
        }
        i += 1
    }
    tmp16s = max_ab_idx as s16b;
    do_s16b(&mut tmp16s, flag);
    if flag == 3 as libc::c_int &&
           tmp16s as libc::c_int > max_ab_idx as libc::c_int {
        quit(b"Too many abilities\x00" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < tmp16s as libc::c_int {
        do_byte(&mut (*ab_info.offset(i as isize)).acquired as *mut bool_ as
                    *mut byte_hack, flag);
        i += 1
    }
    do_s16b(&mut (*p_ptr).luck_base, flag);
    do_s16b(&mut (*p_ptr).luck_max, flag);
    /* Found 24 unused bytes here...
	   Converted it to be the alchemist's
	   known artifact flags.
	   Note that the ego flags and the gained levels
	   record are recorded here too, but we use the
	   _ver_ format to protect save file compatablity.
	   Note that the other alchemist knowledge (item types known)
	   is stored in do_aux, and is a bit flag in a previously
	   unused bit.
	   */
    i = 0 as libc::c_int; /* -KMW- */
    while i < 6 as libc::c_int {
        do_u32b(&mut *alchemist_known_artifacts.as_mut_ptr().offset(i as
                                                                        isize),
                flag);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        do_u32b(&mut *alchemist_known_egos.as_mut_ptr().offset(i as isize),
                flag);
        i += 1
    }
    do_u32b(&mut alchemist_gained, flag);
    do_s32b(&mut (*p_ptr).au, flag);
    do_s32b(&mut (*p_ptr).max_exp, flag);
    do_s32b(&mut (*p_ptr).exp, flag);
    do_u16b(&mut (*p_ptr).exp_frac, flag);
    do_s16b(&mut (*p_ptr).lev, flag);
    do_s16b(&mut (*p_ptr).town_num, flag);
    /* Write arena and rewards information -KMW- */
    do_s16b(&mut (*p_ptr).arena_number, flag);
    do_s16b(&mut (*p_ptr).inside_arena, flag);
    do_s16b(&mut (*p_ptr).inside_quest, flag);
    do_byte(&mut (*p_ptr).exit_bldg as *mut bool_ as *mut byte_hack, flag);
    /* Save/load spellbinder */
    do_byte(&mut (*p_ptr).spellbinder_num,
            flag); /* tmp8u should be 0 at this point */
    do_byte(&mut (*p_ptr).spellbinder_trigger, flag);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        do_u32b(&mut *(*p_ptr).spellbinder.as_mut_ptr().offset(i as isize),
                flag);
        i += 1
    }
    do_byte(&mut tmp8u, flag);
    if flag == 7 as libc::c_int { tmp8u = 7 as libc::c_int as byte_hack }
    do_byte(&mut tmp8u, flag);
    if flag == 3 as libc::c_int && tmp8u as libc::c_int > 7 as libc::c_int {
        quit(format(b"Too many plots, %d %d\x00" as *const u8 as
                        *const libc::c_char, tmp8u as libc::c_int,
                    7 as libc::c_int) as cptr);
    }
    i = 0 as libc::c_int;
    while i < tmp8u as libc::c_int {
        do_s16b(&mut *plots.as_mut_ptr().offset(i as isize), flag);
        i += 1
    }
    if flag == 7 as libc::c_int { tmp8u = 99 as libc::c_int as byte_hack }
    do_byte(&mut tmp8u, flag);
    if flag == 3 as libc::c_int && tmp8u as libc::c_int > 99 as libc::c_int {
        quit(b"Too many random quests\x00" as *const u8 as
                 *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < tmp8u as libc::c_int {
        do_byte(&mut (*random_quests.as_mut_ptr().offset(i as isize)).type_0,
                flag);
        do_s16b(&mut (*random_quests.as_mut_ptr().offset(i as isize)).r_idx,
                flag);
        do_byte(&mut (*random_quests.as_mut_ptr().offset(i as isize)).done as
                    *mut bool_ as *mut byte_hack, flag);
        i += 1
    }
    do_s16b(&mut (*p_ptr).oldpx, flag);
    do_s16b(&mut (*p_ptr).oldpy, flag);
    do_s16b(&mut (*p_ptr).mhp, flag);
    do_s16b(&mut (*p_ptr).chp, flag);
    do_u16b(&mut (*p_ptr).chp_frac, flag);
    do_s16b(&mut (*p_ptr).hp_mod, flag);
    do_s16b(&mut (*p_ptr).msane, flag);
    do_s16b(&mut (*p_ptr).csane, flag);
    do_u16b(&mut (*p_ptr).csane_frac, flag);
    do_s16b(&mut (*p_ptr).msp, flag);
    do_s16b(&mut (*p_ptr).csp, flag);
    do_u16b(&mut (*p_ptr).csp_frac, flag);
    /* XXX
	   Here's where tank points were.
	   Those who run the estate of you-know-who is really stupid.
	   I'll never even consider reading her books now. -- neil */
    do_s16b(&mut tmp16s, flag);
    do_s16b(&mut tmp16s, flag);
    do_s16b(&mut tmp16s, flag);
    do_s16b(&mut tmp16s, flag);
    /* Gods */
    do_s32b(&mut (*p_ptr).grace, flag);
    do_byte(&mut (*p_ptr).praying as *mut bool_ as *mut byte_hack, flag);
    do_s16b(&mut (*p_ptr).melkor_sacrifice, flag);
    do_byte(&mut (*p_ptr).pgod, flag);
    /* Max Player and Dungeon Levels */
    do_s16b(&mut (*p_ptr).max_plv, flag);
    if flag == 7 as libc::c_int { tmp8u = max_d_idx as byte_hack }
    do_byte(&mut tmp8u, flag);
    i = 0 as libc::c_int;
    while i < tmp8u as libc::c_int {
        if flag == 7 as libc::c_int { tmp16s = *max_dlv.offset(i as isize) }
        do_s16b(&mut tmp16s, flag);
        if flag == 3 as libc::c_int && i <= max_d_idx as libc::c_int {
            *max_dlv.offset(i as isize) = tmp16s
        }
        i += 1
    }
    /* Repair max player level??? */
    if flag == 3 as libc::c_int &&
           ((*p_ptr).max_plv as libc::c_int) < (*p_ptr).lev as libc::c_int {
        (*p_ptr).max_plv = (*p_ptr).lev
    }
    do_byte(&mut (*p_ptr).help.enabled as *mut bool_ as *mut byte_hack, flag);
    do_u32b(&mut (*p_ptr).help.help1, flag);
    /* More info */
    tmp16s = 0 as libc::c_int as s16b;
    do_s16b(&mut (*p_ptr).sc, flag);
    do_s16b(&mut (*p_ptr).blind, flag);
    do_s16b(&mut (*p_ptr).paralyzed, flag);
    do_s16b(&mut (*p_ptr).confused, flag);
    do_s16b(&mut (*p_ptr).food, flag);
    do_s32b(&mut (*p_ptr).energy, flag);
    do_s16b(&mut (*p_ptr).fast, flag);
    do_s16b(&mut (*p_ptr).speed_factor, flag);
    do_s16b(&mut (*p_ptr).slow, flag);
    do_s16b(&mut (*p_ptr).afraid, flag);
    do_s16b(&mut (*p_ptr).cut, flag);
    do_s16b(&mut (*p_ptr).stun, flag);
    do_s16b(&mut (*p_ptr).poisoned, flag);
    do_s16b(&mut (*p_ptr).image, flag);
    do_s16b(&mut (*p_ptr).protevil, flag);
    do_s16b(&mut (*p_ptr).protundead, flag);
    do_s16b(&mut (*p_ptr).invuln, flag);
    do_s16b(&mut (*p_ptr).hero, flag);
    do_s16b(&mut (*p_ptr).shero, flag);
    do_s16b(&mut (*p_ptr).shield, flag);
    do_s16b(&mut (*p_ptr).shield_power, flag);
    do_s16b(&mut (*p_ptr).shield_power_opt, flag);
    do_s16b(&mut (*p_ptr).shield_power_opt2, flag);
    do_s16b(&mut (*p_ptr).shield_opt, flag);
    do_s16b(&mut (*p_ptr).blessed, flag);
    do_s16b(&mut (*p_ptr).control, flag);
    do_byte(&mut (*p_ptr).control_dir, flag);
    do_s16b(&mut (*p_ptr).tim_thunder, flag);
    do_s16b(&mut (*p_ptr).tim_thunder_p1, flag);
    do_s16b(&mut (*p_ptr).tim_thunder_p2, flag);
    do_s16b(&mut (*p_ptr).tim_project, flag);
    do_s16b(&mut (*p_ptr).tim_project_dam, flag);
    do_s16b(&mut (*p_ptr).tim_project_gf, flag);
    do_s16b(&mut (*p_ptr).tim_project_rad, flag);
    do_s16b(&mut (*p_ptr).tim_project_flag, flag);
    do_s16b(&mut (*p_ptr).tim_magic_breath, flag);
    do_s16b(&mut (*p_ptr).tim_water_breath, flag);
    do_s16b(&mut (*p_ptr).tim_roots, flag);
    do_s16b(&mut (*p_ptr).tim_roots_ac, flag);
    do_s16b(&mut (*p_ptr).tim_roots_dam, flag);
    do_s16b(&mut (*p_ptr).tim_invis, flag);
    do_s16b(&mut (*p_ptr).word_recall, flag);
    do_s16b(&mut (*p_ptr).recall_dungeon, flag);
    do_s16b(&mut (*p_ptr).see_infra, flag);
    do_s16b(&mut (*p_ptr).tim_infra, flag);
    do_s16b(&mut (*p_ptr).oppose_fire, flag);
    do_s16b(&mut (*p_ptr).oppose_cold, flag);
    do_s16b(&mut (*p_ptr).oppose_acid, flag);
    do_s16b(&mut (*p_ptr).oppose_elec, flag);
    do_s16b(&mut (*p_ptr).oppose_pois, flag);
    do_s16b(&mut (*p_ptr).oppose_ld, flag);
    do_s16b(&mut (*p_ptr).oppose_cc, flag);
    do_s16b(&mut (*p_ptr).oppose_ss, flag);
    do_s16b(&mut (*p_ptr).oppose_nex, flag);
    do_s16b(&mut (*p_ptr).tim_esp, flag);
    do_s16b(&mut (*p_ptr).tim_wraith, flag);
    do_s16b(&mut (*p_ptr).tim_ffall, flag);
    do_ver_s16b(&mut (*p_ptr).tim_fly, 104 as libc::c_int as u32b,
                0 as libc::c_int as s16b, flag);
    do_s16b(&mut (*p_ptr).tim_fire_aura, flag);
    do_ver_s16b(&mut (*p_ptr).tim_poison, 104 as libc::c_int as u32b,
                0 as libc::c_int as s16b, flag);
    do_s16b(&mut (*p_ptr).resist_magic, flag);
    do_s16b(&mut (*p_ptr).tim_invisible, flag);
    do_s16b(&mut (*p_ptr).tim_inv_pow, flag);
    do_s16b(&mut (*p_ptr).tim_mimic, flag);
    do_s16b(&mut (*p_ptr).lightspeed, flag);
    do_s16b(&mut (*p_ptr).tim_lite, flag);
    do_ver_s16b(&mut (*p_ptr).tim_regen, 104 as libc::c_int as u32b,
                0 as libc::c_int as s16b, flag);
    do_ver_s16b(&mut (*p_ptr).tim_regen_pow, 104 as libc::c_int as u32b,
                0 as libc::c_int as s16b, flag);
    do_s16b(&mut (*p_ptr).holy, flag);
    do_s16b(&mut (*p_ptr).walk_water, flag);
    do_s16b(&mut (*p_ptr).tim_mental_barrier, flag);
    do_s16b(&mut (*p_ptr).immov_cntr, flag);
    do_s16b(&mut (*p_ptr).strike, flag);
    do_s16b(&mut (*p_ptr).meditation, flag);
    do_s16b(&mut (*p_ptr).tim_reflect, flag);
    do_s16b(&mut (*p_ptr).tim_res_time, flag);
    do_s16b(&mut (*p_ptr).tim_deadly, flag);
    do_s16b(&mut (*p_ptr).prob_travel, flag);
    do_s16b(&mut (*p_ptr).disrupt_shield, flag);
    do_s16b(&mut (*p_ptr).parasite, flag);
    do_s16b(&mut (*p_ptr).parasite_r_idx, flag);
    do_s32b(&mut (*p_ptr).loan, flag);
    do_s32b(&mut (*p_ptr).loan_time, flag);
    do_s16b(&mut (*p_ptr).absorb_soul, flag);
    do_s16b(&mut (*p_ptr).chaos_patron, flag);
    if flag == 7 as libc::c_int { tmp16s = max_corruptions }
    do_s16b(&mut tmp16s, flag);
    i = 0 as libc::c_int;
    while i < tmp16s as libc::c_int {
        if flag == 7 as libc::c_int && i < max_corruptions as libc::c_int {
            tmp8u = *(*p_ptr).corruptions.offset(i as isize) as byte_hack
        }
        do_byte(&mut tmp8u, flag);
        if flag == 3 as libc::c_int && i < max_corruptions as libc::c_int {
            *(*p_ptr).corruptions.offset(i as isize) = tmp8u as bool_
        }
        i += 1
    }
    do_byte(&mut (*p_ptr).confusing, flag);
    do_byte(&mut (*p_ptr).black_breath as *mut bool_ as *mut byte_hack, flag);
    do_byte(&mut fate_flag as *mut bool_ as *mut byte_hack, flag);
    do_byte(&mut (*p_ptr).searching, flag);
    do_byte(&mut (*p_ptr).maximize, flag);
    do_byte(&mut (*p_ptr).preserve, flag);
    do_byte(&mut (*p_ptr).special, flag);
    do_byte(&mut ambush_flag as *mut bool_ as *mut byte_hack, flag);
    do_byte(&mut (*p_ptr).allow_one_death, flag);
    do_s16b(&mut (*p_ptr).xtra_spells, flag);
    do_byte(&mut tmp8u, flag);
    do_s16b(&mut no_breeds, flag);
    do_s16b(&mut (*p_ptr).protgood, flag);
    /* Auxilliary variables */
    do_u32b(&mut (*p_ptr).mimic_extra, flag);
    do_u32b(&mut (*p_ptr).antimagic_extra, flag);
    do_u32b(&mut (*p_ptr).druid_extra, flag);
    do_u32b(&mut (*p_ptr).druid_extra2, flag);
    do_u32b(&mut (*p_ptr).druid_extra3, flag);
    do_u32b(&mut (*p_ptr).music_extra, flag);
    do_u32b(&mut (*p_ptr).music_extra2, flag);
    do_u32b(&mut (*p_ptr).necro_extra, flag);
    do_u32b(&mut (*p_ptr).necro_extra2, flag);
    do_u32b(&mut (*p_ptr).race_extra1, flag);
    do_u32b(&mut (*p_ptr).race_extra2, flag);
    do_u32b(&mut (*p_ptr).race_extra3, flag);
    do_u32b(&mut (*p_ptr).race_extra4, flag);
    do_u32b(&mut (*p_ptr).race_extra5, flag);
    do_u32b(&mut (*p_ptr).race_extra6, flag);
    do_u32b(&mut (*p_ptr).race_extra7, flag);
    do_u16b(&mut (*p_ptr).body_monster, flag);
    do_byte(&mut (*p_ptr).disembodied as *mut bool_ as *mut byte_hack, flag);
    /* Are we in astral mode? */
    do_byte(&mut (*p_ptr).astral as *mut bool_ as *mut byte_hack, flag);
    if flag == 7 as libc::c_int { tmp16s = 62 as libc::c_int as s16b }
    do_s16b(&mut tmp16s, flag);
    if flag == 3 as libc::c_int && tmp16s as libc::c_int > 62 as libc::c_int {
        note(format(b"Too many (%u) powers!\x00" as *const u8 as
                        *const libc::c_char, tmp16s as libc::c_int) as cptr);
    }
    if flag == 7 as libc::c_int { tmp16s = 62 as libc::c_int as s16b }
    i = 0 as libc::c_int;
    while i < tmp16s as libc::c_int {
        do_byte(&mut *(*p_ptr).powers_mod.as_mut_ptr().offset(i as isize) as
                    *mut bool_ as *mut byte_hack, flag);
        i += 1
    }
    skip_ver_byte(100 as libc::c_int as u32b, flag);
    /* The tactic */
    do_byte(&mut (*p_ptr).tactic as *mut libc::c_char as *mut byte_hack,
            flag);
    /* The movement */
    do_byte(&mut (*p_ptr).movement as *mut libc::c_char as *mut byte_hack,
            flag);
    /* The comapnions killed */
    do_s16b(&mut (*p_ptr).companion_killed, flag);
    /* The fate */
    do_byte(&mut (*p_ptr).no_mortal as *mut bool_ as *mut byte_hack, flag);
    /* The bounties */
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        do_s16b(&mut *(*bounties.as_mut_ptr().offset(i as
                                                         isize)).as_mut_ptr().offset(0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize),
                flag);
        do_s16b(&mut *(*bounties.as_mut_ptr().offset(i as
                                                         isize)).as_mut_ptr().offset(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize),
                flag);
        i += 1
    }
    do_u32b(&mut total_bounties, flag);
    do_s16b(&mut spell_num, flag);
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int { do_spells(i, flag); i += 1 }
    do_s16b(&mut rune_num, flag);
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        do_string(rune_spells[i as usize].name.as_mut_ptr(),
                  30 as libc::c_int, flag);
        do_s16b(&mut (*rune_spells.as_mut_ptr().offset(i as isize)).type_0,
                flag);
        do_s16b(&mut (*rune_spells.as_mut_ptr().offset(i as isize)).rune2,
                flag);
        do_s16b(&mut (*rune_spells.as_mut_ptr().offset(i as isize)).mana,
                flag);
        i += 1
    }
    /* Load random seeds */
    do_u32b(&mut dummy32u, flag); /* Load-compatibility with old savefiles. */
    do_u32b(&mut seed_flavor, flag); /* For consistent object flavors. */
    do_u32b(&mut dummy32u, flag); /* Load-compatibility with old savefiles. */
    /* Special stuff */
    do_u16b(&mut tmp16b, flag); /* Dummy */
    do_u16b(&mut total_winner, flag);
    do_u16b(&mut has_won, flag);
    do_u16b(&mut noscore, flag);
    /* Write death */
    if flag == 7 as libc::c_int { tmp8u = death as byte_hack }
    do_byte(&mut tmp8u, flag);
    if flag == 3 as libc::c_int { death = tmp8u as bool_ }
    /* Incompatible module? */
    if flag == 3 as libc::c_int {
        let mut ok: s32b = 0;
        call_lua(b"module_savefile_loadable\x00" as *const u8 as
                     *const libc::c_char,
                 b"(s,d)\x00" as *const u8 as *const libc::c_char,
                 b"d\x00" as *const u8 as *const libc::c_char,
                 loaded_game_module.as_mut_ptr(), death as libc::c_int,
                 &mut ok as *mut s32b);
        /* Argh bad game module! */
        if ok == 0 {
            note(format(b"Bad game module. Savefile was saved with module \'%s\' but game is \'%s\'.\x00"
                            as *const u8 as *const libc::c_char,
                        loaded_game_module.as_mut_ptr(), game_module) as
                     cptr);
            return 0 as libc::c_int as bool_
        }
    }
    /* Write feeling */
    if flag == 7 as libc::c_int { tmp8u = feeling as byte_hack }
    do_byte(&mut tmp8u, flag);
    if flag == 3 as libc::c_int { feeling = tmp8u as s16b }
    /* Turn of last "feeling" */
    do_s32b(&mut old_turn, flag);
    /* Current turn */
    do_s32b(&mut turn, flag);
    return 1 as libc::c_int as bool_;
}
/* Save the current persistent dungeon -SC- */
#[no_mangle]
pub unsafe extern "C" fn save_dungeon() {
    let mut tmp: [libc::c_char; 16] = [0; 16];
    let mut name: [libc::c_char; 1024] = [0; 1024];
    let mut buf: [libc::c_char; 5] = [0; 5];
    /* Save only persistent dungeons */
    if get_dungeon_save(buf.as_mut_ptr()) == 0 || dun_level == 0 { return }
    /* Construct filename */
    sprintf(tmp.as_mut_ptr(),
            b"%s.%s\x00" as *const u8 as *const libc::c_char,
            player_base.as_mut_ptr(), buf.as_mut_ptr());
    path_build(name.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_SAVE,
               tmp.as_mut_ptr() as cptr);
    /* Open the file */
    fff =
        my_fopen(name.as_mut_ptr() as cptr,
                 b"wb\x00" as *const u8 as *const libc::c_char);
    /* Save the dungeon */
    do_dungeon(7 as libc::c_int, 1 as libc::c_int as bool_);
    my_fclose(fff);
}
/*
 * Medium level player saver
 */
unsafe extern "C" fn save_player_aux(mut name: *mut libc::c_char) -> bool_ {
    let mut ok: bool_ = 0 as libc::c_int as bool_;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut mode: libc::c_int = 0o644 as libc::c_int;
    /* No file yet */
    fff = 0 as *mut FILE;
    /* File type is "SAVE" */
    /* Create the savefile */
    fd = fd_make(name as cptr, mode);
    /* File is okay */
    if fd >= 0 as libc::c_int {
        /* Close the "fd" */
        fd_close(fd);
        /* Open the savefile */
        fff =
            my_fopen(name as cptr,
                     b"wb\x00" as *const u8 as *const libc::c_char);
        /* Successful open */
        if !fff.is_null() {
            /* Write the savefile */
            if do_savefile_aux(7 as libc::c_int) != 0 {
                ok = 1 as libc::c_int as bool_
            }
            /* Attempt to close it */
            if my_fclose(fff) != 0 { ok = 0 as libc::c_int as bool_ }
        }
        /* "broken" savefile */
        if ok == 0 {
            /* Remove "broken" files */
            fd_kill(name as cptr);
        }
    }
    /* Failure */
    if ok == 0 { return 0 as libc::c_int as bool_ }
    /* Successful save */
    character_saved = 1 as libc::c_int as bool_;
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Attempt to save the player in a savefile
 */
#[no_mangle]
pub unsafe extern "C" fn save_player() -> bool_ {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut safe: [libc::c_char; 1024] = [0; 1024];
    /* New savefile */
    strcpy(safe.as_mut_ptr(), savefile.as_mut_ptr());
    strcat(safe.as_mut_ptr(),
           b".new\x00" as *const u8 as *const libc::c_char);
    /* Remove it */
    fd_kill(safe.as_mut_ptr() as cptr);
    /* Attempt to save the player */
    if save_player_aux(safe.as_mut_ptr()) != 0 {
        let mut temp: [libc::c_char; 1024] = [0; 1024];
        /* Old savefile */
        strcpy(temp.as_mut_ptr(), savefile.as_mut_ptr());
        strcat(temp.as_mut_ptr(),
               b".old\x00" as *const u8 as *const libc::c_char);
        /* Remove it */
        fd_kill(temp.as_mut_ptr() as cptr);
        /* Preserve old savefile */
        fd_move(savefile.as_mut_ptr() as cptr, temp.as_mut_ptr() as cptr);
        /* Activate new savefile */
        fd_move(safe.as_mut_ptr() as cptr, savefile.as_mut_ptr() as cptr);
        /* Remove preserved savefile */
        fd_kill(temp.as_mut_ptr() as cptr);
        /* Hack -- Pretend the character was loaded */
        character_loaded = 1 as libc::c_int as bool_;
        /* Success */
        result = 1 as libc::c_int
    }
    save_savefile_names();
    /* Return the result */
    return result as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn file_exist(mut buf: *mut libc::c_char) -> bool_ {
    let mut fd: libc::c_int = 0;
    let mut result: bool_ = 0;
    /* Open savefile */
    fd = fd_open(buf as cptr, 0 as libc::c_int);
    /* File exists */
    if fd >= 0 as libc::c_int {
        fd_close(fd);
        result = 1 as libc::c_int as bool_
    } else { result = 0 as libc::c_int as bool_ }
    return result;
}
/*
 * Attempt to Load a "savefile"
 *
 * On multi-user systems, you may only "read" a savefile if you will be
 * allowed to "write" it later, this prevents painful situations in which
 * the player loads a savefile belonging to someone else, and then is not
 * allowed to save his game when he quits.
 *
 * We return "TRUE" if the savefile was usable, and we set the global
 * flag "character_loaded" if a real, living, character was loaded.
 *
 * Note that we always try to load the "current" savefile, even if
 * there is no such file, so we must check for "empty" savefile names.
 */
#[no_mangle]
pub unsafe extern "C" fn load_player() -> bool_ {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut err: errr = 0 as libc::c_int;
    let mut what: cptr = b"generic\x00" as *const u8 as *const libc::c_char;
    /* Paranoia */
    turn = 0 as libc::c_int;
    /* Paranoia */
    death = 0 as libc::c_int as bool_;
    /* Allow empty savefile name */
    if savefile[0 as libc::c_int as usize] == 0 {
        return 1 as libc::c_int as bool_
    }
    /* XXX XXX XXX Fix this */
    /* Verify the existance of the savefile */
    if file_exist(savefile.as_mut_ptr()) == 0 {
        /* Give a message */
        msg_format(b"Savefile does not exist: %s\x00" as *const u8 as
                       *const libc::c_char, savefile.as_mut_ptr());
        msg_print(0 as cptr);
        /* Allow this */
        return 1 as libc::c_int as bool_
    }
    /* Okay */
    if err == 0 {
        /* Open the savefile */
        fd = fd_open(savefile.as_mut_ptr() as cptr, 0 as libc::c_int);
        /* No file */
        if fd < 0 as libc::c_int { err = -(1 as libc::c_int) }
        /* Message (below) */
        if err != 0 {
            what =
                b"Cannot open savefile\x00" as *const u8 as
                    *const libc::c_char
        }
    }
    /* Process file */
    if err == 0 {
        /* Open the file XXX XXX XXX XXX Should use Angband file interface */
        fff =
            my_fopen(savefile.as_mut_ptr() as cptr,
                     b"rb\x00" as *const u8 as *const libc::c_char);
        /*		fff = fdopen(fd, "r"); */
        /* Read the first four bytes */
        do_u32b(&mut vernum, 3 as libc::c_int);
        do_byte(&mut sf_extra, 3 as libc::c_int);
        /* XXX XXX XXX XXX Should use Angband file interface */
        my_fclose(fff);
        /* fclose(fff) */
        /* Close the file */
        fd_close(fd);
    }
    /* Process file */
    if err == 0 {
        /* Extract version */
        sf_major = VERSION_MAJOR as byte_hack;
        sf_minor = VERSION_MINOR as byte_hack;
        sf_patch = VERSION_PATCH as byte_hack;
        sf_extra = 0 as libc::c_int as byte_hack;
        /* Clear screen */
        Term_clear();
        /* Attempt to load */
        err = rd_savefile();
        /* Message (below) */
        if err != 0 {
            what =
                b"Cannot parse savefile\x00" as *const u8 as
                    *const libc::c_char
        }
    }
    /* Paranoia */
    if err == 0 {
        /* Invalid turn */
        if turn == 0 { err = -(1 as libc::c_int) }
        /* Message (below) */
        if err != 0 {
            what = b"Broken savefile\x00" as *const u8 as *const libc::c_char
        }
    }
    /* Okay */
    if err == 0 {
        /* Maybe the scripts want to resurrect char */
        if process_hooks_ret(61 as libc::c_int,
                             b"d\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                             b"(d)\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, death as libc::c_int) != 0
           {
            character_loaded =
                process_hooks_return[0 as libc::c_int as usize].num as bool_;
            death =
                process_hooks_return[1 as libc::c_int as usize].num as bool_;
            return 1 as libc::c_int as bool_
        }
        /* Player is dead */
        if death != 0 {
            /* Player is no longer "dead" */
            death = 0 as libc::c_int as bool_;
            /* Cheat death (unless the character retired) */
            if arg_wizard as libc::c_int != 0 && total_winner == 0 {
                /* A character was loaded */
                character_loaded = 1 as libc::c_int as bool_;
                /* Done */
                return 1 as libc::c_int as bool_
            }
            /* Count lives */
            sf_lives = sf_lives.wrapping_add(1);
            /* Forget turns */
            old_turn = 0 as libc::c_int;
            turn = old_turn;
            /* Done */
            return 1 as libc::c_int as bool_
        }
        /* A character was loaded */
        character_loaded = 1 as libc::c_int as bool_;
        /* Still alive */
        if (*p_ptr).chp as libc::c_int >= 0 as libc::c_int {
            /* Reset cause of death */
            strcpy(died_from.as_mut_ptr(),
                   b"(alive and well)\x00" as *const u8 as
                       *const libc::c_char);
        }
        /* Success */
        return 1 as libc::c_int as bool_
    }
    /* Message */
    msg_format(b"Error (%s) reading %d.%d.%d savefile.\x00" as *const u8 as
                   *const libc::c_char, what, sf_major as libc::c_int,
               sf_minor as libc::c_int, sf_patch as libc::c_int);
    msg_print(0 as cptr);
    /* Oops */
    return 0 as libc::c_int as bool_;
}
/* File: loadsave.c */
/* Purpose: interact with savefiles. This file was made by
   unifying load2.c and save.c from the old codebase. Doing it
   this way makes maintenance easier and lets us share code. */
/*
 * Size-aware read/write routines for the savefile, do all their
 * work through sf_get and sf_put.
 */
unsafe extern "C" fn do_byte(mut v: *mut byte_hack, mut flag: libc::c_int) {
    if flag == 3 as libc::c_int { *v = sf_get(); return }
    if flag == 7 as libc::c_int {
        let mut val: byte_hack = *v;
        sf_put(val);
        return
    }
    /* We should never reach here, so bail out */
    printf(b"FATAL: do_byte passed %d\n\x00" as *const u8 as
               *const libc::c_char, flag);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn do_u16b(mut v: *mut u16b, mut flag: libc::c_int) {
    if flag == 3 as libc::c_int {
        *v = sf_get() as u16b;
        *v =
            (*v as libc::c_int |
                 (sf_get() as u16b as libc::c_int) << 8 as libc::c_int) as
                u16b;
        return
    }
    if flag == 7 as libc::c_int {
        let mut val: u16b = 0;
        val = *v;
        sf_put((val as libc::c_int & 0xff as libc::c_int) as byte_hack);
        sf_put((val as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
                   as byte_hack);
        return
    }
    /* Never should reach here, bail out */
    printf(b"FATAL: do_u16b passed %d\n\x00" as *const u8 as
               *const libc::c_char, flag);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn do_s16b(mut ip: *mut s16b, mut flag: libc::c_int) {
    if flag == 3 as libc::c_int { do_u16b(ip as *mut u16b, flag); return }
    if flag == 7 as libc::c_int { do_u16b(ip as *mut u16b, flag); return }
    /* Blah blah, never should reach here, die */
    printf(b"FATAL: do_s16b passed %d\n\x00" as *const u8 as
               *const libc::c_char, flag);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn do_u32b(mut ip: *mut u32b, mut flag: libc::c_int) {
    if flag == 3 as libc::c_int {
        *ip = sf_get() as u32b;
        *ip |= (sf_get() as u32b) << 8 as libc::c_int;
        *ip |= (sf_get() as u32b) << 16 as libc::c_int;
        *ip |= (sf_get() as u32b) << 24 as libc::c_int;
        return
    }
    if flag == 7 as libc::c_int {
        let mut val: u32b = *ip;
        sf_put((val & 0xff as libc::c_int as libc::c_uint) as byte_hack);
        sf_put((val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                   as byte_hack);
        sf_put((val >> 16 as libc::c_int &
                    0xff as libc::c_int as libc::c_uint) as byte_hack);
        sf_put((val >> 24 as libc::c_int &
                    0xff as libc::c_int as libc::c_uint) as byte_hack);
        return
    }
    /* Bad news if you're here, because you're going down */
    printf(b"FATAL: do_u32b passed %d\n\x00" as *const u8 as
               *const libc::c_char, flag);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn do_s32b(mut ip: *mut s32b, mut flag: libc::c_int) {
    if flag == 3 as libc::c_int { do_u32b(ip as *mut u32b, flag); return }
    if flag == 7 as libc::c_int { do_u32b(ip as *mut u32b, flag); return }
    /* Raus! Schnell! */
    printf(b"FATAL: do_s32b passed %d\n\x00" as *const u8 as
               *const libc::c_char, flag);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn do_string(mut str: *mut libc::c_char,
                               mut max: libc::c_int, mut flag: libc::c_int) 
 /* Max is ignored for writing */
 {
    if flag == 3 as libc::c_int {
        let mut i: libc::c_int = 0;
        /* Read the string */
        i = 0 as libc::c_int;
        loop  {
            let mut tmp8u: byte_hack = 0;
            /* Read a byte */
            do_byte(&mut tmp8u, 3 as libc::c_int);
            /* Collect string while legal */
            if i < max { *str.offset(i as isize) = tmp8u as libc::c_char }
            /* End of string */
            if tmp8u == 0 { break ; }
            i += 1
        }
        /* Terminate */
        *str.offset((max - 1 as libc::c_int) as isize) =
            '\u{0}' as i32 as libc::c_char; /* Output a terminator */
        return
    }
    if flag == 7 as libc::c_int {
        while *str != 0 {
            do_byte(str as *mut byte_hack, flag);
            str = str.offset(1)
        }
        do_byte(str as *mut byte_hack, flag);
        return
    }
    printf(b"FATAL: do_string passed flag %d\n\x00" as *const u8 as
               *const libc::c_char, flag);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn skip_ver_byte(mut version: u32b, mut flag: libc::c_int) 
 /* Reads and discards a byte if the savefile is as old as/older than version */
 {
    if flag == 3 as libc::c_int && vernum <= version {
        let mut forget: byte_hack = 0;
        do_byte(&mut forget, flag);
    };
}
unsafe extern "C" fn do_ver_s16b(mut v: *mut s16b, mut version: u32b,
                                 mut defval: s16b, mut flag: libc::c_int) {
    if flag == 3 as libc::c_int && vernum < version { *v = defval; return }
    do_s16b(v, flag);
}
/*
 * Show information on the screen, one line at a time.
 *
 * Avoid the top two lines, to avoid interference with "msg_print()".
 */
unsafe extern "C" fn note(mut msg: cptr) {
    static mut y: libc::c_int = 2 as libc::c_int;
    /* Draw the message */
    prt(msg, y, 0 as libc::c_int);
    /* Advance one line (wrap if needed) */
    y += 1;
    if y >= 24 as libc::c_int { y = 2 as libc::c_int }
    /* Flush it */
    Term_fresh();
}
/*
 * Determine if an item can be wielded/worn (e.g. helmet, sword, bow, arrow)
 */
unsafe extern "C" fn wearable_p(mut o_ptr: *mut object_type) -> bool_ {
    /* Valid "tval" codes */
    match (*o_ptr).tval as libc::c_int {
        65 | 55 | 66 | 67 | 16 | 17 | 18 | 15 | 19 | 20 | 21 | 22 | 6 | 23 |
        24 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 70 | 39 | 71 | 72 |
        40 | 45 | 99 | 14 | 115 | 46 | 12 => {
            return 1 as libc::c_int as bool_
        }
        _ => { }
    }
    /* Nope */
    return 0 as libc::c_int as bool_;
}
/*
 * rd/wr an object
 *
 * FIXME! This code probably has a lot of cruft from the old Z/V codebase.
 *
 */
unsafe extern "C" fn do_item(mut o_ptr: *mut object_type,
                             mut flag: libc::c_int) {
    let mut old_dd: byte_hack = 0;
    let mut old_ds: byte_hack = 0;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut k_ptr: *mut object_kind = 0 as *mut object_kind;
    /* Kind */
    do_s16b(&mut (*o_ptr).k_idx, flag);
    /* Location */
    do_byte(&mut (*o_ptr).iy, flag);
    do_byte(&mut (*o_ptr).ix, flag);
    /* Type/Subtype */
    do_byte(&mut (*o_ptr).tval, flag);
    do_byte(&mut (*o_ptr).sval, flag);
    /* Special pval */
    do_s32b(&mut (*o_ptr).pval, flag);
    /* Special pval */
    do_s16b(&mut (*o_ptr).pval2, flag);
    /* Special pval */
    do_s32b(&mut (*o_ptr).pval3, flag);
    do_byte(&mut (*o_ptr).discount, flag);
    do_byte(&mut (*o_ptr).number, flag);
    do_s32b(&mut (*o_ptr).weight, flag);
    do_byte(&mut (*o_ptr).name1, flag);
    do_s16b(&mut (*o_ptr).name2, flag);
    do_s16b(&mut (*o_ptr).name2b, flag);
    do_s16b(&mut (*o_ptr).timeout, flag);
    do_s16b(&mut (*o_ptr).to_h, flag);
    do_s16b(&mut (*o_ptr).to_d, flag);
    do_s16b(&mut (*o_ptr).to_a, flag);
    do_s16b(&mut (*o_ptr).ac, flag);
    /* We do special processing of this flag when reading */
    if flag == 3 as libc::c_int {
        do_byte(&mut old_dd, 3 as libc::c_int);
        do_byte(&mut old_ds, 3 as libc::c_int);
    }
    if flag == 7 as libc::c_int {
        do_byte(&mut (*o_ptr).dd, 7 as libc::c_int);
        do_byte(&mut (*o_ptr).ds, 7 as libc::c_int);
    }
    do_byte(&mut (*o_ptr).ident, flag);
    do_byte(&mut (*o_ptr).marked, flag);
    /* flags */
    do_u32b(&mut (*o_ptr).art_flags1, flag);
    do_u32b(&mut (*o_ptr).art_flags2, flag);
    do_u32b(&mut (*o_ptr).art_flags3, flag);
    do_u32b(&mut (*o_ptr).art_flags4, flag);
    do_u32b(&mut (*o_ptr).art_flags5, flag);
    do_u32b(&mut (*o_ptr).art_esp, flag);
    /* obvious flags */
    do_u32b(&mut (*o_ptr).art_oflags1, flag);
    do_u32b(&mut (*o_ptr).art_oflags2, flag);
    do_u32b(&mut (*o_ptr).art_oflags3, flag);
    do_u32b(&mut (*o_ptr).art_oflags4, flag);
    do_u32b(&mut (*o_ptr).art_oflags5, flag);
    do_u32b(&mut (*o_ptr).art_oesp, flag);
    /* Monster holding object */
    do_s16b(&mut (*o_ptr).held_m_idx, flag);
    /* Special powers */
    do_byte(&mut (*o_ptr).xtra1, flag);
    do_s16b(&mut (*o_ptr).xtra2, flag);
    do_byte(&mut (*o_ptr).elevel, flag);
    do_s32b(&mut (*o_ptr).exp, flag);
    /* Read the pseudo-id */
    do_byte(&mut (*o_ptr).sense, flag);
    /* Read the found info */
    do_byte(&mut (*o_ptr).found, flag);
    do_s16b(&mut (*o_ptr).found_aux1, flag);
    do_s16b(&mut (*o_ptr).found_aux2, flag);
    do_s16b(&mut (*o_ptr).found_aux3, flag);
    do_s16b(&mut (*o_ptr).found_aux4, flag);
    if flag == 3 as libc::c_int {
        let mut buf: [libc::c_char; 128] = [0; 128];
        /* Inscription */
        do_string(buf.as_mut_ptr(), 128 as libc::c_int, 3 as libc::c_int);
        /* Save the inscription */
        if buf[0 as libc::c_int as usize] != 0 {
            (*o_ptr).note = quark_add(buf.as_mut_ptr() as cptr) as u16b
        }
        do_string(buf.as_mut_ptr(), 128 as libc::c_int, 3 as libc::c_int);
        if buf[0 as libc::c_int as usize] != 0 {
            (*o_ptr).art_name = quark_add(buf.as_mut_ptr() as cptr) as u16b
        }
    }
    if flag == 7 as libc::c_int {
        /* Save the inscription (if any) */
        if (*o_ptr).note != 0 {
            do_string(quark_str((*o_ptr).note as s16b) as *mut libc::c_char,
                      0 as libc::c_int,
                      7 as
                          libc::c_int); /* Stick any more shared code before this. The rest
										   of this function is reserved for LS_LOAD's
										   cleanup functions */
        } else {
            do_string(b"\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, 0 as libc::c_int,
                      7 as libc::c_int);
        }
        if (*o_ptr).art_name != 0 {
            do_string(quark_str((*o_ptr).art_name as s16b) as
                          *mut libc::c_char, 0 as libc::c_int,
                      7 as libc::c_int);
        } else {
            do_string(b"\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, 0 as libc::c_int,
                      7 as libc::c_int);
        }
    }
    if flag == 7 as libc::c_int { return }
    /* ********** END OF LS_SAVE ***************/
    /* Obtain the "kind" template */
    k_ptr = &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    /* Obtain tval/sval from k_info */
    (*o_ptr).tval = (*k_ptr).tval;
    if (*o_ptr).tval as libc::c_int != 102 as libc::c_int {
        (*o_ptr).sval = (*k_ptr).sval
    }
    /* Repair non "wearable" items */
    if wearable_p(o_ptr) == 0 {
        /* Acquire correct fields */
        (*o_ptr).to_h = (*k_ptr).to_h;
        (*o_ptr).to_d = (*k_ptr).to_d;
        (*o_ptr).to_a = (*k_ptr).to_a;
        /* Acquire correct fields */
        (*o_ptr).ac = (*k_ptr).ac;
        (*o_ptr).dd = (*k_ptr).dd;
        (*o_ptr).ds = (*k_ptr).ds;
        /* All done */
        return
    }
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Paranoia */
    if (*o_ptr).name1 != 0 {
        let mut a_ptr: *mut artifact_type = 0 as *mut artifact_type;
        /* Obtain the artifact info */
        a_ptr =
            &mut *a_info.offset((*o_ptr).name1 as isize) as
                *mut artifact_type;
        /* Verify that artifact */
        if (*a_ptr).name == 0 {
            (*o_ptr).name1 = 0 as libc::c_int as byte_hack
        }
    }
    /* Paranoia */
    if (*o_ptr).name2 != 0 {
        let mut e_ptr: *mut ego_item_type = 0 as *mut ego_item_type;
        /* Obtain the ego-item info */
        e_ptr =
            &mut *e_info.offset((*o_ptr).name2 as isize) as
                *mut ego_item_type;
        /* Verify that ego-item */
        if (*e_ptr).name == 0 { (*o_ptr).name2 = 0 as libc::c_int as s16b }
    }
    /* Acquire standard fields */
    (*o_ptr).ac = (*k_ptr).ac;
    (*o_ptr).dd = (*k_ptr).dd;
    (*o_ptr).ds = (*k_ptr).ds;
    /* Artifacts */
    if (*o_ptr).name1 != 0 {
        let mut a_ptr_0: *mut artifact_type = 0 as *mut artifact_type;
        /* Obtain the artifact info */
        a_ptr_0 =
            &mut *a_info.offset((*o_ptr).name1 as isize) as
                *mut artifact_type;
        /* Acquire new artifact fields */
        (*o_ptr).ac = (*a_ptr_0).ac;
        (*o_ptr).dd = (*a_ptr_0).dd;
        (*o_ptr).ds = (*a_ptr_0).ds;
        /* Acquire new artifact weight */
        (*o_ptr).weight = (*a_ptr_0).weight as s32b
    }
    /* Ego items */
    if (*o_ptr).name2 != 0 { (*o_ptr).dd = old_dd; (*o_ptr).ds = old_ds }
    if (*o_ptr).art_name != 0 {
        /* A random artifact */
        (*o_ptr).dd = old_dd;
        (*o_ptr).ds = old_ds
    };
}
/*
 * Read a monster
 */
unsafe extern "C" fn do_monster(mut m_ptr: *mut monster_type,
                                mut flag: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut tmp: bool_ = 0;
    /* Read the monster race */
    do_s16b(&mut (*m_ptr).r_idx, flag);
    do_u16b(&mut (*m_ptr).ego, flag);
    /* Read the other information */
    do_byte(&mut (*m_ptr).fy, flag);
    do_byte(&mut (*m_ptr).fx, flag);
    do_s32b(&mut (*m_ptr).hp, flag);
    do_s32b(&mut (*m_ptr).maxhp, flag);
    do_s16b(&mut (*m_ptr).csleep, flag);
    do_byte(&mut (*m_ptr).mspeed, flag);
    do_byte(&mut (*m_ptr).energy, flag);
    do_byte(&mut (*m_ptr).stunned, flag);
    do_byte(&mut (*m_ptr).confused, flag);
    do_byte(&mut (*m_ptr).monfear, flag);
    do_u32b(&mut (*m_ptr).smart, flag);
    do_s16b(&mut (*m_ptr).status, flag);
    do_s16b(&mut (*m_ptr).possessor, flag);
    do_byte(&mut (*m_ptr).speed, flag);
    do_byte(&mut (*m_ptr).level, flag);
    do_s16b(&mut (*m_ptr).ac, flag);
    do_u32b(&mut (*m_ptr).exp, flag);
    do_s16b(&mut (*m_ptr).target, flag);
    do_s16b(&mut (*m_ptr).bleeding, flag);
    do_s16b(&mut (*m_ptr).poisoned, flag);
    do_s32b(&mut (*m_ptr).mflag, flag);
    if flag == 3 as libc::c_int {
        (*m_ptr).mflag &=
            0x2 as libc::c_int | 0x200 as libc::c_int | 0x4 as libc::c_int |
                0x8 as libc::c_int | 0x100 as libc::c_int
    }
    /* Attacks */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        do_byte(&mut (*(*m_ptr).blow.as_mut_ptr().offset(i as isize)).method,
                flag);
        do_byte(&mut (*(*m_ptr).blow.as_mut_ptr().offset(i as isize)).effect,
                flag);
        do_byte(&mut (*(*m_ptr).blow.as_mut_ptr().offset(i as isize)).d_dice,
                flag);
        do_byte(&mut (*(*m_ptr).blow.as_mut_ptr().offset(i as isize)).d_side,
                flag);
        i += 1
    }
    /* Mind */
    tmp =
        if !(*m_ptr).mind.is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    do_byte(&mut tmp as *mut bool_ as *mut byte_hack, flag);
    if tmp != 0 {
        if flag == 3 as libc::c_int {
            (*m_ptr).mind =
                memset(ralloc(::std::mem::size_of::<monster_mind>() as
                                  libc::c_ulong) as *mut libc::c_char as
                           *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<monster_mind>() as libc::c_ulong)
                    as *mut monster_mind
        }
    }
    /* Special race */
    tmp =
        if !(*m_ptr).sr_ptr.is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    do_byte(&mut tmp as *mut bool_ as *mut byte_hack, flag);
    if tmp != 0 {
        if flag == 3 as libc::c_int {
            (*m_ptr).sr_ptr =
                memset(ralloc(::std::mem::size_of::<monster_race>() as
                                  libc::c_ulong) as *mut libc::c_char as
                           *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        do_u32b(&mut (*(*m_ptr).sr_ptr).name, flag);
        do_u32b(&mut (*(*m_ptr).sr_ptr).text, flag);
        do_u16b(&mut (*(*m_ptr).sr_ptr).hdice, flag);
        do_u16b(&mut (*(*m_ptr).sr_ptr).hside, flag);
        do_s16b(&mut (*(*m_ptr).sr_ptr).ac, flag);
        do_s16b(&mut (*(*m_ptr).sr_ptr).sleep, flag);
        do_byte(&mut (*(*m_ptr).sr_ptr).aaf, flag);
        do_byte(&mut (*(*m_ptr).sr_ptr).speed, flag);
        do_s32b(&mut (*(*m_ptr).sr_ptr).mexp, flag);
        do_s32b(&mut (*(*m_ptr).sr_ptr).weight, flag);
        do_byte(&mut (*(*m_ptr).sr_ptr).freq_inate, flag);
        do_byte(&mut (*(*m_ptr).sr_ptr).freq_spell, flag);
        do_u32b(&mut (*(*m_ptr).sr_ptr).flags1, flag);
        do_u32b(&mut (*(*m_ptr).sr_ptr).flags2, flag);
        do_u32b(&mut (*(*m_ptr).sr_ptr).flags3, flag);
        do_u32b(&mut (*(*m_ptr).sr_ptr).flags4, flag);
        do_u32b(&mut (*(*m_ptr).sr_ptr).flags5, flag);
        do_u32b(&mut (*(*m_ptr).sr_ptr).flags6, flag);
        do_u32b(&mut (*(*m_ptr).sr_ptr).flags7, flag);
        do_u32b(&mut (*(*m_ptr).sr_ptr).flags8, flag);
        do_u32b(&mut (*(*m_ptr).sr_ptr).flags9, flag);
        /* Attacks */
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            do_byte(&mut (*(*(*m_ptr).sr_ptr).blow.as_mut_ptr().offset(i as
                                                                           isize)).method,
                    flag);
            do_byte(&mut (*(*(*m_ptr).sr_ptr).blow.as_mut_ptr().offset(i as
                                                                           isize)).effect,
                    flag);
            do_byte(&mut (*(*(*m_ptr).sr_ptr).blow.as_mut_ptr().offset(i as
                                                                           isize)).d_dice,
                    flag);
            do_byte(&mut (*(*(*m_ptr).sr_ptr).blow.as_mut_ptr().offset(i as
                                                                           isize)).d_side,
                    flag);
            i += 1
        }
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            do_byte(&mut *(*(*m_ptr).sr_ptr).body_parts.as_mut_ptr().offset(i
                                                                                as
                                                                                isize),
                    flag);
            i += 1
        }
        do_byte(&mut (*(*m_ptr).sr_ptr).level, flag);
        do_byte(&mut (*(*m_ptr).sr_ptr).rarity, flag);
        do_byte(&mut (*(*m_ptr).sr_ptr).d_char as *mut libc::c_char as
                    *mut byte_hack, flag);
        do_byte(&mut (*(*m_ptr).sr_ptr).d_attr, flag);
        do_byte(&mut (*(*m_ptr).sr_ptr).x_char as *mut libc::c_char as
                    *mut byte_hack, flag);
        do_byte(&mut (*(*m_ptr).sr_ptr).x_attr, flag);
        do_s16b(&mut (*(*m_ptr).sr_ptr).max_num, flag);
        do_byte(&mut (*(*m_ptr).sr_ptr).cur_num, flag);
    };
}
/*
 * Handle monster lore
 */
unsafe extern "C" fn do_lore(mut r_idx: libc::c_int, mut flag: libc::c_int) {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Count sights/deaths/kills */
    do_s16b(&mut (*r_ptr).r_sights, flag);
    do_s16b(&mut (*r_ptr).r_deaths, flag);
    do_s16b(&mut (*r_ptr).r_pkills, flag);
    do_s16b(&mut (*r_ptr).r_tkills, flag);
    /* Count wakes and ignores */
    do_byte(&mut (*r_ptr).r_wake, flag);
    do_byte(&mut (*r_ptr).r_ignore, flag);
    /* Extra stuff */
    do_byte(&mut (*r_ptr).r_xtra1, flag);
    do_byte(&mut (*r_ptr).r_xtra2, flag);
    /* Count drops */
    do_byte(&mut (*r_ptr).r_drop_gold, flag);
    do_byte(&mut (*r_ptr).r_drop_item, flag);
    /* Count spells */
    do_byte(&mut (*r_ptr).r_cast_inate, flag);
    do_byte(&mut (*r_ptr).r_cast_spell, flag);
    /* Count blows of each type */
    do_byte(&mut *(*r_ptr).r_blows.as_mut_ptr().offset(0 as libc::c_int as
                                                           isize), flag);
    do_byte(&mut *(*r_ptr).r_blows.as_mut_ptr().offset(1 as libc::c_int as
                                                           isize), flag);
    do_byte(&mut *(*r_ptr).r_blows.as_mut_ptr().offset(2 as libc::c_int as
                                                           isize), flag);
    do_byte(&mut *(*r_ptr).r_blows.as_mut_ptr().offset(3 as libc::c_int as
                                                           isize), flag);
    /* Memorize flags */
    do_u32b(&mut (*r_ptr).r_flags1, flag); /* Just to remind you */
    do_u32b(&mut (*r_ptr).r_flags2, flag); /* flag is unrelated to */
    do_u32b(&mut (*r_ptr).r_flags3, flag); /* the other argument */
    do_u32b(&mut (*r_ptr).r_flags4, flag);
    do_u32b(&mut (*r_ptr).r_flags5, flag);
    do_u32b(&mut (*r_ptr).r_flags6, flag);
    do_u32b(&mut (*r_ptr).r_flags7, flag);
    do_u32b(&mut (*r_ptr).r_flags8, flag);
    do_u32b(&mut (*r_ptr).r_flags9, flag);
    /* Read the "Racial" monster tmp16b per level */
    do_s16b(&mut (*r_ptr).max_num, flag);
    do_byte(&mut (*r_ptr).on_saved as *mut bool_ as *mut byte_hack, flag);
    if flag == 3 as libc::c_int {
        /* Lore flag repair? */
        (*r_ptr).r_flags1 &= (*r_ptr).flags1;
        (*r_ptr).r_flags2 &= (*r_ptr).flags2;
        (*r_ptr).r_flags3 &= (*r_ptr).flags3;
        (*r_ptr).r_flags4 &= (*r_ptr).flags4;
        (*r_ptr).r_flags5 &= (*r_ptr).flags5;
        (*r_ptr).r_flags6 &= (*r_ptr).flags6
    };
}
/*
 * Read a store
 */
unsafe extern "C" fn do_store(mut str: *mut store_type, mut flag: libc::c_int)
 -> bool_ 
 /* FIXME! Why does this return anything when
   it always returns the same thing? */
 {
    let mut j: libc::c_int = 0;
    let mut num: byte_hack = 0;
    let mut store_inven_max: byte_hack = 255 as libc::c_int as byte_hack;
    /* Some basic info */
    do_s32b(&mut (*str).store_open, flag);
    do_s16b(&mut (*str).insult_cur, flag);
    do_u16b(&mut (*str).owner, flag);
    if flag == 7 as libc::c_int { num = (*str).stock_num }
    /* Could be cleaner, done this way for benefit of the for loop later on */
    do_byte(&mut num, flag);
    do_s16b(&mut (*str).good_buy, flag);
    do_s16b(&mut (*str).bad_buy, flag);
    /* Last visit */
    do_s32b(&mut (*str).last_visit, flag);
    /* Items */
    j = 0 as libc::c_int;
    while j < num as libc::c_int {
        if flag == 3 as libc::c_int {
            /* Can't this be cleaner? */
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
            /* Wipe the object */
            object_wipe(&mut forge);
            /* Read the item */
            do_item(&mut forge, 3 as libc::c_int);
            /* Acquire valid items */
            if ((*str).stock_num as libc::c_int) <
                   store_inven_max as libc::c_int &&
                   ((*str).stock_num as libc::c_int) <
                       (*str).stock_size as libc::c_int {
                let fresh0 = (*str).stock_num;
                (*str).stock_num = (*str).stock_num.wrapping_add(1);
                let mut k: libc::c_int = fresh0 as libc::c_int;
                /* Acquire the item */
                object_copy(&mut *(*str).stock.offset(k as isize),
                            &mut forge);
            }
        }
        if flag == 7 as libc::c_int {
            do_item(&mut *(*str).stock.offset(j as isize), flag);
        }
        j += 1
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * RNG state
 */
unsafe extern "C" fn do_randomizer(mut flag: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut tmp16u: u16b = 0 as libc::c_int as u16b;
    /* Tmp */
    do_u16b(&mut tmp16u, flag);
    /* Place */
    do_u16b(&mut Rand_place, flag);
    /* State */
    i = 0 as libc::c_int;
    while i < 63 as libc::c_int {
        do_u32b(&mut *Rand_state.as_mut_ptr().offset(i as isize), flag);
        i += 1
    }
    /* Accept */
    if flag == 3 as libc::c_int { Rand_quick = 0 as libc::c_int as bool_ };
}
/*
 * Handle options
 *
 * Normal options are stored as a set of 256 bit flags,
 * plus a set of 256 bit masks to indicate which bit flags were defined
 * at the time the savefile was created.  This will allow new options
 * to be added, and old options to be removed, at any time, without
 * hurting old savefiles.
 *
 * The window options are stored in the same way, but note that each
 * window gets 32 options, and their order is fixed by certain defines.
 */
unsafe extern "C" fn do_options(mut flag: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut oflag: [u32b; 8] = [0; 8];
    let mut mask: [u32b; 8] = [0; 8];
    /* ** Special info */
    /* Read "delay_factor" */
    do_byte(&mut delay_factor, flag);
    /* Read "hitpoint_warn" */
    do_byte(&mut hitpoint_warn, flag);
    /* ** Cheating options ***/
    if flag == 3 as libc::c_int {
        /* There *MUST* be some nice way to unify this! */
        let mut c: u16b = 0;
        do_u16b(&mut c, 3 as libc::c_int);
        if c as libc::c_int & 0x2 as libc::c_int != 0 {
            wizard = 1 as libc::c_int as bool_
        }
        cheat_peek =
            if c as libc::c_int & 0x100 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_;
        cheat_hear =
            if c as libc::c_int & 0x200 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_;
        cheat_room =
            if c as libc::c_int & 0x400 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_;
        cheat_xtra =
            if c as libc::c_int & 0x800 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_;
        cheat_know =
            if c as libc::c_int & 0x1000 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_;
        cheat_live =
            if c as libc::c_int & 0x2000 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_
    }
    if flag == 7 as libc::c_int {
        let mut c_0: u16b = 0 as libc::c_int as u16b;
        if wizard != 0 {
            c_0 = (c_0 as libc::c_int | 0x2 as libc::c_int) as u16b
        }
        if cheat_peek != 0 {
            c_0 = (c_0 as libc::c_int | 0x100 as libc::c_int) as u16b
        }
        if cheat_hear != 0 {
            c_0 = (c_0 as libc::c_int | 0x200 as libc::c_int) as u16b
        }
        if cheat_room != 0 {
            c_0 = (c_0 as libc::c_int | 0x400 as libc::c_int) as u16b
        }
        if cheat_xtra != 0 {
            c_0 = (c_0 as libc::c_int | 0x800 as libc::c_int) as u16b
        }
        if cheat_know != 0 {
            c_0 = (c_0 as libc::c_int | 0x1000 as libc::c_int) as u16b
        }
        if cheat_live != 0 {
            c_0 = (c_0 as libc::c_int | 0x2000 as libc::c_int) as u16b
        }
        do_u16b(&mut c_0, 7 as libc::c_int);
    }
    do_byte(&mut autosave_l as *mut bool_ as *mut byte_hack, flag);
    do_byte(&mut autosave_t as *mut bool_ as *mut byte_hack, flag);
    do_s16b(&mut autosave_freq, flag);
    if flag == 3 as libc::c_int {
        /* Read the option flags */
        n = 0 as libc::c_int;
        while n < 8 as libc::c_int {
            do_u32b(&mut *oflag.as_mut_ptr().offset(n as isize), flag);
            n += 1
        }
        /* Read the option masks */
        n = 0 as libc::c_int;
        while n < 8 as libc::c_int {
            do_u32b(&mut *mask.as_mut_ptr().offset(n as isize), flag);
            n += 1
        }
        /* Analyze the options */
        n = 0 as libc::c_int;
        while n < 8 as libc::c_int {
            /* Analyze the options */
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                /* Process valid flags */
                if mask[n as usize] as libc::c_long & (1 as libc::c_long) << i
                       != 0 {
                    /* Process valid flags */
                    if option_mask[n as usize] as libc::c_long &
                           (1 as libc::c_long) << i != 0 {
                        /* Set */
                        if oflag[n as usize] as libc::c_long &
                               (1 as libc::c_long) << i != 0 {
                            /* Set */
                            option_flag[n as usize] =
                                (option_flag[n as usize] as libc::c_long |
                                     (1 as libc::c_long) << i) as u32b
                        } else {
                            /* Clear */
                            /* Clear */
                            option_flag[n as usize] =
                                (option_flag[n as usize] as libc::c_long &
                                     !((1 as libc::c_long) << i)) as u32b
                        }
                    }
                }
                i += 1
            }
            n += 1
        }
        /* ** Window Options ***/
        /* Read the window flags */
        n = 0 as libc::c_int;
        while n < 8 as libc::c_int {
            do_u32b(&mut *oflag.as_mut_ptr().offset(n as isize), flag);
            n += 1
        }
        /* Read the window masks */
        n = 0 as libc::c_int;
        while n < 8 as libc::c_int {
            do_u32b(&mut *mask.as_mut_ptr().offset(n as isize), flag);
            n += 1
        }
        /* Analyze the options */
        n = 0 as libc::c_int;
        while n < 8 as libc::c_int {
            /* Analyze the options */
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                /* Process valid flags */
                if mask[n as usize] as libc::c_long & (1 as libc::c_long) << i
                       != 0 {
                    /* Process valid flags */
                    if window_mask[n as usize] as libc::c_long &
                           (1 as libc::c_long) << i != 0 {
                        /* Set */
                        if oflag[n as usize] as libc::c_long &
                               (1 as libc::c_long) << i != 0 {
                            /* Set */
                            window_flag[n as usize] =
                                (window_flag[n as usize] as libc::c_long |
                                     (1 as libc::c_long) << i) as u32b
                        } else {
                            /* Clear */
                            /* Clear */
                            window_flag[n as usize] =
                                (window_flag[n as usize] as libc::c_long &
                                     !((1 as libc::c_long) << i)) as u32b
                        }
                    }
                }
                i += 1
            }
            n += 1
        }
    }
    if flag == 7 as libc::c_int {
        /* Analyze the options */
        i = 0 as libc::c_int;
        while !(*option_info.as_mut_ptr().offset(i as isize)).o_desc.is_null()
              {
            let mut os: libc::c_int =
                (*option_info.as_mut_ptr().offset(i as isize)).o_page as
                    libc::c_int;
            let mut ob: libc::c_int =
                (*option_info.as_mut_ptr().offset(i as isize)).o_bit as
                    libc::c_int;
            /* Process real entries */
            if !(*option_info.as_mut_ptr().offset(i as isize)).o_var.is_null()
               {
                /* Set */
                if *(*option_info.as_mut_ptr().offset(i as isize)).o_var != 0
                   {
                    /* Set */
                    option_flag[os as usize] =
                        (option_flag[os as usize] as libc::c_long |
                             (1 as libc::c_long) << ob) as u32b
                } else {
                    /* Clear */
                    /* Clear */
                    option_flag[os as usize] =
                        (option_flag[os as usize] as libc::c_long &
                             !((1 as libc::c_long) << ob)) as u32b
                }
            }
            i += 1
        }
        /* ** Normal options ***/
        /* Dump the flags */
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            do_u32b(&mut *option_flag.as_mut_ptr().offset(i as isize), flag);
            i += 1
        }
        /* Dump the masks */
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            do_u32b(&mut *option_mask.as_mut_ptr().offset(i as isize), flag);
            i += 1
        }
        /* ** Window options ***/
        /* Dump the flags */
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            do_u32b(&mut *window_flag.as_mut_ptr().offset(i as isize), flag);
            i += 1
        }
        /* Dump the masks */
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            do_u32b(&mut *window_mask.as_mut_ptr().offset(i as isize), flag);
            i += 1
        }
    };
}
/* Load/Save the random spells info */
unsafe extern "C" fn do_spells(mut i: libc::c_int, mut flag: libc::c_int) {
    let mut s_ptr: *mut random_spell =
        &mut *random_spells.as_mut_ptr().offset(i as isize) as
            *mut random_spell;
    do_string((*s_ptr).name.as_mut_ptr(), 30 as libc::c_int, flag);
    do_string((*s_ptr).desc.as_mut_ptr(), 30 as libc::c_int, flag);
    do_s16b(&mut (*s_ptr).mana, flag);
    do_s16b(&mut (*s_ptr).fail, flag);
    do_u32b(&mut (*s_ptr).proj_flags, flag);
    do_byte(&mut (*s_ptr).GF, flag);
    do_byte(&mut (*s_ptr).radius, flag);
    do_byte(&mut (*s_ptr).dam_sides, flag);
    do_byte(&mut (*s_ptr).dam_dice, flag);
    do_byte(&mut (*s_ptr).level, flag);
    do_byte(&mut (*s_ptr).untried as *mut bool_ as *mut byte_hack, flag);
}
/*
 * Handle player inventory
 *
 * FIXME! This function probably could be unified better
 * Note that the inventory is "re-sorted" later by "dungeon()".
 */
unsafe extern "C" fn do_inventory(mut flag: libc::c_int) -> bool_ {
    if flag == 3 as libc::c_int {
        let mut slot: libc::c_int = 0 as libc::c_int;
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
        /* No items */
        inven_cnt = 0 as libc::c_int as s16b;
        equip_cnt = 0 as libc::c_int as s16b;
        loop 
             /* Read until done */
             {
            let mut n: u16b = 0;
            /* Get the next item index */
            do_u16b(&mut n, 3 as libc::c_int);
            /* Nope, we reached the end */
            if n as libc::c_int == 0xffff as libc::c_int { break ; }
            /* Get local object */
            q_ptr = &mut forge;
            /* Wipe the object */
            object_wipe(q_ptr);
            /* Read the item */
            do_item(q_ptr, 3 as libc::c_int);
            /* Hack -- verify item */
            if (*q_ptr).k_idx == 0 { return 0 as libc::c_int as bool_ }
            /* Wield equipment */
            if n as libc::c_int >= 24 as libc::c_int {
                /* Copy object */
                object_copy(&mut *(*p_ptr).inventory.as_mut_ptr().offset(n as
                                                                             isize),
                            q_ptr);
                /* Take care of item sets */
                if (*q_ptr).name1 != 0 {
                    wield_set((*q_ptr).name1 as s16b,
                              (*a_info.offset((*q_ptr).name1 as isize)).set,
                              1 as libc::c_int as bool_);
                }
                /* One more item */
                equip_cnt += 1
            } else if inven_cnt as libc::c_int == 23 as libc::c_int {
                /* Warning -- backpack is full */
                /* Oops */
                note(b"Too many items in the inventory!\x00" as *const u8 as
                         *const libc::c_char);
                /* Fail */
                return 0 as libc::c_int as bool_
            } else {
                /* Carry inventory */
                /* Get a slot */
                let fresh1 = slot;
                slot = slot + 1;
                n = fresh1 as u16b;
                object_copy(&mut *(*p_ptr).inventory.as_mut_ptr().offset(n as
                                                                             isize),
                            q_ptr);
                inven_cnt += 1
            }
        }
    }
    if flag == 7 as libc::c_int {
        let mut i: u16b = 0;
        let mut sent: u16b = 0xffff as libc::c_int as u16b;
        i = 0 as libc::c_int as u16b;
        while (i as libc::c_int) < 52 as libc::c_int {
            let mut o_ptr: *mut object_type =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                    *mut object_type;
            if !((*o_ptr).k_idx == 0) {
                do_u16b(&mut i, flag);
                do_item(o_ptr, flag);
            }
            i = i.wrapping_add(1)
        }
        do_u16b(&mut sent, 7 as libc::c_int);
        /* Copy object */
        /* One more item */
        /* Sentinel */
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Read the saved messages
 */
unsafe extern "C" fn do_messages(mut flag: libc::c_int) 
 /* FIXME! We should be able to unify this better */
 {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut color: byte_hack = 0;
    let mut type_0: byte_hack = 0;
    let mut num: s16b = 0;
    if flag == 7 as libc::c_int { num = message_num() }
    /* Total */
    do_s16b(&mut num, flag);
    /* Read the messages */
    if flag == 3 as libc::c_int {
        i = 0 as libc::c_int;
        while i < num as libc::c_int {
            /* Read the message */
            do_string(buf.as_mut_ptr(), 128 as libc::c_int, 3 as libc::c_int);
            do_byte(&mut color, flag);
            do_byte(&mut type_0, flag);
            /* Save the message */
            message_add(type_0, buf.as_mut_ptr() as cptr, color);
            i += 1
        }
    }
    if flag == 7 as libc::c_int {
        let mut holder: byte_hack = 0;
        i = num as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            do_string(message_str(i as s16b as libc::c_int) as
                          *mut libc::c_char, 0 as libc::c_int,
                      7 as libc::c_int);
            holder = message_color(i as s16b as libc::c_int);
            do_byte(&mut holder, flag);
            holder = message_type(i as s16b as libc::c_int);
            do_byte(&mut holder, flag);
            i -= 1
        }
    };
}
/*
 * Handle dungeon
 *
 * The monsters/objects must be loaded in the same order
 * that they were stored, since the actual indexes matter.
 */
unsafe extern "C" fn do_dungeon(mut flag: libc::c_int,
                                mut no_companions: bool_) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Read specific */
    let mut tmp16b: u16b = 0 as libc::c_int as u16b;
    my_sentinel(b"Before do_dungeon\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char, 324 as libc::c_int as u16b, flag);
    /* Header info */
    do_s16b(&mut dun_level, flag);
    do_byte(&mut dungeon_type, flag);
    do_s16b(&mut num_repro, flag);
    do_s16b(&mut (*p_ptr).py, flag);
    do_s16b(&mut (*p_ptr).px, flag);
    do_s16b(&mut cur_hgt, flag);
    do_s16b(&mut cur_wid, flag);
    do_s16b(&mut max_panel_rows, flag);
    do_s16b(&mut max_panel_cols, flag);
    do_u32b(&mut dungeon_flags1, flag);
    do_u32b(&mut dungeon_flags2, flag);
    /* Last teleportation */
    do_s16b(&mut last_teleportation_y, flag);
    do_s16b(&mut last_teleportation_y, flag);
    /* Spell effects */
    tmp16b = 128 as libc::c_int as u16b;
    do_u16b(&mut tmp16b, flag);
    if flag == 3 as libc::c_int && tmp16b as libc::c_int > 128 as libc::c_int
       {
        quit(b"Too many spell effects\x00" as *const u8 as
                 *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < tmp16b as libc::c_int {
        do_s16b(&mut (*effects.as_mut_ptr().offset(i as isize)).type_0, flag);
        do_s16b(&mut (*effects.as_mut_ptr().offset(i as isize)).dam, flag);
        do_s16b(&mut (*effects.as_mut_ptr().offset(i as isize)).time, flag);
        do_u32b(&mut (*effects.as_mut_ptr().offset(i as isize)).flags, flag);
        do_s16b(&mut (*effects.as_mut_ptr().offset(i as isize)).cx, flag);
        do_s16b(&mut (*effects.as_mut_ptr().offset(i as isize)).cy, flag);
        do_s16b(&mut (*effects.as_mut_ptr().offset(i as isize)).rad, flag);
        i += 1
    }
    /* TO prevent bugs with evolving dungeons */
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        do_s16b(&mut *floor_type.as_mut_ptr().offset(i as isize), flag);
        do_s16b(&mut *fill_type.as_mut_ptr().offset(i as isize), flag);
        i += 1
    }
    if flag == 3 as libc::c_int &&
           (dun_level == 0 && (*p_ptr).inside_quest == 0) {
        let mut xstart: libc::c_int = 0 as libc::c_int;
        let mut ystart: libc::c_int = 0 as libc::c_int;
        /* Init the wilderness */
        process_dungeon_file(b"w_info.txt\x00" as *const u8 as
                                 *const libc::c_char, &mut ystart,
                             &mut xstart, cur_hgt as libc::c_int,
                             cur_wid as libc::c_int,
                             1 as libc::c_int as bool_,
                             0 as libc::c_int as bool_);
        /* Init the town */
        xstart = 0 as libc::c_int;
        ystart = 0 as libc::c_int;
        init_flags = 0 as libc::c_int;
        process_dungeon_file(b"t_info.txt\x00" as *const u8 as
                                 *const libc::c_char, &mut ystart,
                             &mut xstart, cur_hgt as libc::c_int,
                             cur_wid as libc::c_int,
                             1 as libc::c_int as bool_,
                             0 as libc::c_int as bool_);
    }
    do_grid(flag);
    /* ** Objects ***/
    if flag == 7 as libc::c_int { compact_objects(0 as libc::c_int); }
    if flag == 7 as libc::c_int { compact_monsters(0 as libc::c_int); }
    if flag == 7 as libc::c_int {
        tmp16b = o_max as u16b;
        if no_companions != 0 {
            i = 1 as libc::c_int;
            while i < o_max as libc::c_int {
                let mut o_ptr: *mut object_type =
                    &mut *o_list.offset(i as isize) as *mut object_type;
                if (*o_ptr).held_m_idx as libc::c_int != 0 &&
                       (*m_list.offset((*o_ptr).held_m_idx as isize)).status
                           as libc::c_int == 4 as libc::c_int {
                    tmp16b = tmp16b.wrapping_sub(1)
                }
                i += 1
            }
        }
        /* Item count */
        do_u16b(&mut tmp16b, flag);
        tmp16b = o_max as u16b
    } else {
        /* Read item count */
        do_u16b(&mut tmp16b, flag);
    }
    /* Verify maximum */
    if flag == 3 as libc::c_int &&
           tmp16b as libc::c_int > max_o_idx as libc::c_int {
        note(format(b"Too many (%d) object entries!\x00" as *const u8 as
                        *const libc::c_char, tmp16b as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    /* Dungeon items */
    i = 1 as libc::c_int;
    while i < tmp16b as libc::c_int {
        let mut o_idx: libc::c_int = 0;
        let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
        if flag == 7 as libc::c_int {
            o_ptr_0 = &mut *o_list.offset(i as isize) as *mut object_type;
            /* Saving is easy */
            if !(no_companions as libc::c_int != 0 &&
                     (*o_ptr_0).held_m_idx as libc::c_int != 0 &&
                     (*m_list.offset((*o_ptr_0).held_m_idx as isize)).status
                         as libc::c_int == 4 as libc::c_int) {
                do_item(o_ptr_0, 7 as libc::c_int);
            }
        } else {
            /* Don't save objects held by companions when no_companions is set */
            /* Until the end of the loop, this is all LS_LOAD */
            /* Get a new record */
            o_idx = o_pop() as libc::c_int;
            /* Oops */
            if i != o_idx {
                note(format(b"Object allocation error (%d <> %d)\x00" as
                                *const u8 as *const libc::c_char, i, o_idx) as
                         cptr);
                return 0 as libc::c_int as bool_
            }
            /* Acquire place */
            o_ptr_0 = &mut *o_list.offset(o_idx as isize) as *mut object_type;
            /* Read the item */
            do_item(o_ptr_0, 3 as libc::c_int);
            /* Monster */
            if (*o_ptr_0).held_m_idx != 0 {
                let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
                /* Monster */
                m_ptr =
                    &mut *m_list.offset((*o_ptr_0).held_m_idx as isize) as
                        *mut monster_type;
                /* Build a stack */
                (*o_ptr_0).next_o_idx = (*m_ptr).hold_o_idx;
                /* Place the object */
                (*m_ptr).hold_o_idx = o_idx as s16b
            } else {
                /* Dungeon */
                /* Access the item location */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset((*o_ptr_0).iy as
                                                         isize)).offset((*o_ptr_0).ix
                                                                            as
                                                                            isize)
                        as *mut cave_type;
                (*o_ptr_0).next_o_idx = (*c_ptr).o_idx;
                (*c_ptr).o_idx = o_idx as s16b
            }
        }
        i += 1
    }
    /* Build a stack */
    /* Place the object */
    /* ** Monsters ***/
    if flag == 7 as libc::c_int {
        tmp16b = m_max as u16b;
        if no_companions != 0 {
            i = 1 as libc::c_int;
            while i < m_max as libc::c_int {
                let mut m_ptr_0: *mut monster_type =
                    &mut *m_list.offset(i as isize) as *mut monster_type;
                if (*m_ptr_0).status as libc::c_int == 4 as libc::c_int {
                    tmp16b = tmp16b.wrapping_sub(1)
                }
                i += 1
            }
        }
        /* Write the monster count */
        do_u16b(&mut tmp16b, flag);
        tmp16b = m_max as u16b
    } else {
        /* Read the monster count */
        do_u16b(&mut tmp16b, flag);
    }
    /* Validate */
    if flag == 3 as libc::c_int &&
           tmp16b as libc::c_int > max_m_idx as libc::c_int {
        note(format(b"Too many (%d) monster entries!\x00" as *const u8 as
                        *const libc::c_char, tmp16b as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    /* Read the monsters */
    i = 1 as libc::c_int;
    while i < tmp16b as libc::c_int {
        let mut m_idx: libc::c_int = 0;
        let mut m_ptr_1: *mut monster_type = 0 as *mut monster_type;
        let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
        if flag == 7 as libc::c_int {
            m_ptr_1 = &mut *m_list.offset(i as isize) as *mut monster_type;
            /* Easy to save a monster */
            if !(no_companions as libc::c_int != 0 &&
                     (*m_ptr_1).status as libc::c_int == 4 as libc::c_int) {
                do_monster(m_ptr_1, 7 as libc::c_int);
            }
        } else {
            /* Don't save companions when no_companions is set */
            /* From here on, it's all LS_LOAD */
		/* Get a new record */
            m_idx = m_pop() as libc::c_int;
            /* Oops */
            if i != m_idx {
                note(format(b"Monster allocation error (%d <> %d)\x00" as
                                *const u8 as *const libc::c_char, i, m_idx) as
                         cptr);
                return 0 as libc::c_int as bool_
            }
            /* Acquire monster */
            m_ptr_1 =
                &mut *m_list.offset(m_idx as isize) as *mut monster_type;
            /* Read the monster */
            do_monster(m_ptr_1, 3 as libc::c_int);
            /* Access grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset((*m_ptr_1).fy as
                                                     isize)).offset((*m_ptr_1).fx
                                                                        as
                                                                        isize)
                    as *mut cave_type;
            /* Mark the location */
            (*c_ptr).m_idx = m_idx as s16b;
            /* Controlled ? */
            if (*m_ptr_1).mflag & 0x8 as libc::c_int != 0 {
                (*p_ptr).control = m_idx as s16b
            }
            /* Access race */
            r_ptr =
                &mut *r_info.offset((*m_ptr_1).r_idx as isize) as
                    *mut monster_race;
            /* Count XXX XXX XXX */
            (*r_ptr).cur_num = (*r_ptr).cur_num.wrapping_add(1)
        }
        i += 1
    }
    /* Read the kept monsters */
    tmp16b =
        if flag == 7 as libc::c_int && no_companions == 0 {
            max_m_idx as libc::c_int
        } else { 0 as libc::c_int } as u16b;
    /* Read the monster count */
    do_u16b(&mut tmp16b, flag);
    /* Hack -- verify */
    if flag == 3 as libc::c_int &&
           tmp16b as libc::c_int > max_m_idx as libc::c_int {
        note(format(b"Too many (%d) monster entries!\x00" as *const u8 as
                        *const libc::c_char, tmp16b as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    i = 1 as libc::c_int;
    while i < tmp16b as libc::c_int {
        let mut m_ptr_2: *mut monster_type = 0 as *mut monster_type;
        /* Acquire monster */
        m_ptr_2 = &mut *km_list.offset(i as isize) as *mut monster_type;
        /* Read the monster */
        do_monster(m_ptr_2, flag);
        i += 1
    }
    /* ** Success ***/
    /* The dungeon is ready */
    if flag == 3 as libc::c_int {
        character_dungeon = 1 as libc::c_int as bool_
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/* Returns TRUE if we successfully load the dungeon */
#[no_mangle]
pub unsafe extern "C" fn load_dungeon(mut ext: *mut libc::c_char) -> bool_ {
    let mut tmp: [libc::c_char; 16] = [0; 16];
    let mut name: [libc::c_char; 1024] = [0; 1024];
    let mut old_dungeon_type: byte_hack = dungeon_type;
    let mut old_dun: s16b = dun_level;
    /* Construct name */
    sprintf(tmp.as_mut_ptr(),
            b"%s.%s\x00" as *const u8 as *const libc::c_char,
            player_base.as_mut_ptr(), ext);
    path_build(name.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_SAVE,
               tmp.as_mut_ptr() as cptr);
    /* Open the file */
    fff =
        my_fopen(name.as_mut_ptr() as cptr,
                 b"rb\x00" as *const u8 as *const libc::c_char);
    if fff.is_null() {
        dun_level = old_dun;
        dungeon_type = old_dungeon_type;
        my_fclose(fff);
        return 0 as libc::c_int as bool_
    }
    /* Read the dungeon */
    if do_dungeon(3 as libc::c_int, 0 as libc::c_int as bool_) == 0 {
        dun_level = old_dun;
        dungeon_type = old_dungeon_type;
        my_fclose(fff);
        return 0 as libc::c_int as bool_
    }
    dun_level = old_dun;
    dungeon_type = old_dungeon_type;
    /* Done */
    my_fclose(fff);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn do_blocks(mut flag: libc::c_int) 
 /* Handle blocked-allocation stuff for quests and lua stuff
   This depends on a dyn_tosave array of s32b's. Adjust as needed
   if other data structures are desirable. This also is not hooked
   in yet. Ideally, plug it near the end of the savefile.
 */
 {
    let mut numblks: s16b = 0; /* How many blocks do we have? */
    let mut n_iter: s16b = 0 as libc::c_int as s16b;
    do_s16b(&mut numblks, flag);
    while (n_iter as libc::c_int) < numblks as libc::c_int {
        /*	do_s32b(dyn_tosave[n_iter], flag); */
        n_iter += 1
    }
    my_sentinel(b"In blocked-allocation area\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                37 as libc::c_int as u16b, flag);
}
unsafe extern "C" fn do_fate(mut i: libc::c_int, mut flag: libc::c_int) {
    if flag == 3 as libc::c_int && i >= 200 as libc::c_int {
        i = 200 as libc::c_int - 1 as libc::c_int
    }
    do_byte(&mut (*fates.as_mut_ptr().offset(i as isize)).fate, flag);
    do_byte(&mut (*fates.as_mut_ptr().offset(i as isize)).level, flag);
    do_byte(&mut (*fates.as_mut_ptr().offset(i as isize)).serious, flag);
    do_s16b(&mut (*fates.as_mut_ptr().offset(i as isize)).o_idx, flag);
    do_s16b(&mut (*fates.as_mut_ptr().offset(i as isize)).e_idx, flag);
    do_s16b(&mut (*fates.as_mut_ptr().offset(i as isize)).a_idx, flag);
    do_s16b(&mut (*fates.as_mut_ptr().offset(i as isize)).v_idx, flag);
    do_s16b(&mut (*fates.as_mut_ptr().offset(i as isize)).r_idx, flag);
    do_s16b(&mut (*fates.as_mut_ptr().offset(i as isize)).count, flag);
    do_s16b(&mut (*fates.as_mut_ptr().offset(i as isize)).time, flag);
    do_byte(&mut (*fates.as_mut_ptr().offset(i as isize)).know as *mut bool_
                as *mut byte_hack, flag);
}
/*
 * Actually read the savefile
 */
unsafe extern "C" fn do_savefile_aux(mut flag: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp8u: byte_hack = 0;
    let mut tmp16u: u16b = 0;
    let mut tmp32u: u32b = 0;
    let mut reals: *mut bool_ = 0 as *mut bool_;
    let mut real_max: u16b = 0 as libc::c_int as u16b;
    /* Mention the savefile version */
    if flag == 3 as libc::c_int {
        if vernum < 100 as libc::c_int as libc::c_uint {
            note(format(b"Savefile version %lu too old! \x00" as *const u8 as
                            *const libc::c_char, vernum) as
                     cptr); /* Note when file was saved */
            return 0 as libc::c_int as bool_
        } else {
            note(format(b"Loading version %lu savefile... \x00" as *const u8
                            as *const libc::c_char, vernum) as cptr);
        }
    }
    if flag == 7 as libc::c_int {
        sf_when = time(0 as *mut time_t) as u32b;
        /* Increment the saves ctr */
        sf_xtra = 0 as libc::c_long as u32b; /* What the hell is this? */
        sf_saves = sf_saves.wrapping_add(1)
    }
    /* Handle version bytes. FIXME! DG wants me to change this all around */
    if flag == 3 as libc::c_int {
        let mut mt32b: u32b = 0;
        let mut mtbyte: byte_hack = 0;
        /* Discard all this, we've already read it */
        do_u32b(&mut mt32b, flag);
        do_byte(&mut mtbyte, flag);
    }
    if flag == 7 as libc::c_int {
        let mut saver: u32b = 0;
        saver = 104 as libc::c_int as u32b;
        do_u32b(&mut saver, flag);
        tmp8u = Rand_div(256 as libc::c_int) as byte_hack;
        do_byte(&mut tmp8u, flag);
        /* 'encryption' */
    }
    /* Operating system info? Not really. This is just set to 0L */
    do_u32b(&mut sf_xtra, flag);
    /* Time of last save */
    do_u32b(&mut sf_when, flag);
    /* Number of past lives */
    do_u16b(&mut sf_lives, flag);
    /* Number of times saved */
    do_u16b(&mut sf_saves, flag);
    /* Game module */
    if flag == 7 as libc::c_int {
        strcpy(loaded_game_module.as_mut_ptr(), game_module);
    }
    do_string(loaded_game_module.as_mut_ptr(), 80 as libc::c_int, flag);
    /* Read RNG state */
    do_randomizer(flag);
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded Randomizer Info\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Automatizer state */
    do_byte(&mut automatizer_enabled as *mut bool_ as *mut byte_hack, flag);
    /* Then the options */
    do_options(flag);
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded Option Flags\x00" as *const u8 as *const libc::c_char);
    }
    /* Then the "messages" */
    do_messages(flag);
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded Messages\x00" as *const u8 as *const libc::c_char);
    }
    /* Monster Memory */
    if flag == 7 as libc::c_int { tmp16u = max_r_idx }
    do_u16b(&mut tmp16u, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int &&
           tmp16u as libc::c_int > max_r_idx as libc::c_int {
        note(format(b"Too many (%u) monster races!\x00" as *const u8 as
                        *const libc::c_char, tmp16u as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    /* Read the available records */
    i = 0 as libc::c_int;
    while i < tmp16u as libc::c_int {
        /* Read the lore */
        do_lore(i, flag);
        i += 1
    }
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded Monster Memory\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Object Memory */
    if flag == 7 as libc::c_int { tmp16u = max_k_idx }
    do_u16b(&mut tmp16u, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int &&
           tmp16u as libc::c_int > max_k_idx as libc::c_int {
        note(format(b"Too many (%u) object kinds!\x00" as *const u8 as
                        *const libc::c_char, tmp16u as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    /* Read the object memory */
    i = 0 as libc::c_int;
    while i < tmp16u as libc::c_int { do_xtra(i, flag); i += 1 }
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded Object Memory\x00" as *const u8 as *const libc::c_char);
    }
    if flag == 3 as libc::c_int { junkinit(); }
    let mut max_towns_ldsv: u16b = 0;
    let mut max_quests_ldsv: u16b = 0;
    if flag == 7 as libc::c_int { max_towns_ldsv = max_towns }
    /* Number of towns */
    do_u16b(&mut max_towns_ldsv, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int &&
           max_towns_ldsv as libc::c_int > max_towns as libc::c_int {
        note(format(b"Too many (%u) towns!\x00" as *const u8 as
                        *const libc::c_char, max_towns_ldsv as libc::c_int) as
                 cptr);
        return 0 as libc::c_int as bool_
    }
    /* Min of random towns */
    if flag == 7 as libc::c_int { max_towns_ldsv = 20 as libc::c_int as u16b }
    do_u16b(&mut max_towns_ldsv, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int &&
           max_towns_ldsv as libc::c_int != 20 as libc::c_int {
        note(format(b"Different random towns base (%u)!\x00" as *const u8 as
                        *const libc::c_char, max_towns_ldsv as libc::c_int) as
                 cptr);
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < max_towns as libc::c_int {
        do_byte(&mut (*town_info.offset(i as isize)).destroyed as *mut bool_
                    as *mut byte_hack, flag);
        if i >= 20 as libc::c_int {
            do_u32b(&mut (*town_info.offset(i as isize)).seed, flag);
            do_byte(&mut (*town_info.offset(i as isize)).numstores, flag);
            do_byte(&mut (*town_info.offset(i as isize)).flags, flag);
            /* If the town is realy used create a sock */
            if (*town_info.offset(i as isize)).flags as libc::c_int &
                   0x1 as libc::c_int != 0 && flag == 3 as libc::c_int {
                create_stores_stock(i);
            }
        }
        i += 1
    }
    /* Number of dungeon */
    if flag == 7 as libc::c_int { max_towns_ldsv = max_d_idx }
    do_u16b(&mut max_towns_ldsv, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int &&
           max_towns_ldsv as libc::c_int > max_d_idx as libc::c_int {
        note(format(b"Too many dungeon types (%u)!\x00" as *const u8 as
                        *const libc::c_char, max_towns_ldsv as libc::c_int) as
                 cptr);
        return 0 as libc::c_int as bool_
    }
    /* Number of towns per dungeon */
    if flag == 7 as libc::c_int { max_quests_ldsv = 4 as libc::c_int as u16b }
    do_u16b(&mut max_quests_ldsv, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int &&
           max_quests_ldsv as libc::c_int > 4 as libc::c_int {
        note(format(b"Too many town per dungeons (%u)!\x00" as *const u8 as
                        *const libc::c_char, max_quests_ldsv as libc::c_int)
                 as cptr);
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < max_towns_ldsv as libc::c_int {
        j = 0 as libc::c_int;
        while j < max_quests_ldsv as libc::c_int {
            do_s16b(&mut *(*d_info.offset(i as
                                              isize)).t_idx.as_mut_ptr().offset(j
                                                                                    as
                                                                                    isize),
                    flag);
            do_s16b(&mut *(*d_info.offset(i as
                                              isize)).t_level.as_mut_ptr().offset(j
                                                                                      as
                                                                                      isize),
                    flag);
            j += 1
        }
        do_s16b(&mut (*d_info.offset(i as isize)).t_num, flag);
        i += 1
    }
    if flag == 7 as libc::c_int {
        max_quests_ldsv = 26 as libc::c_int as u16b
    }
    /* Number of quests */
    do_u16b(&mut max_quests_ldsv, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int &&
           max_quests_ldsv as libc::c_int > 26 as libc::c_int {
        note(format(b"Too many (%u) quests!\x00" as *const u8 as
                        *const libc::c_char, max_quests_ldsv as libc::c_int)
                 as cptr);
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < max_quests_ldsv as libc::c_int {
        do_s16b(&mut (*quest.offset(i as isize)).status, flag);
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            do_s32b(&mut *(*quest.offset(i as
                                             isize)).data.as_mut_ptr().offset(j
                                                                                  as
                                                                                  isize),
                    flag);
            j += 1
        }
        /* Init the hooks */
        if flag == 3 as libc::c_int &&
               (*quest.offset(i as isize)).type_0 as libc::c_int ==
                   0 as libc::c_int {
            (*quest.offset(i as
                               isize)).init.expect("non-null function pointer")(i);
        }
        i += 1
    }
    /* Position in the wilderness */
    do_s32b(&mut (*p_ptr).wilderness_x, flag);
    do_s32b(&mut (*p_ptr).wilderness_y, flag);
    do_byte(&mut (*p_ptr).wild_mode as *mut bool_ as *mut byte_hack, flag);
    do_byte(&mut (*p_ptr).old_wild_mode as *mut bool_ as *mut byte_hack,
            flag);
    let mut wild_x_size: s32b = 0;
    let mut wild_y_size: s32b = 0;
    if flag == 7 as libc::c_int {
        wild_x_size = max_wild_x as s32b;
        wild_y_size = max_wild_y as s32b
    }
    /* Size of the wilderness */
    do_s32b(&mut wild_x_size, flag);
    do_s32b(&mut wild_y_size, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int &&
           (wild_x_size > max_wild_x as libc::c_int ||
                wild_y_size > max_wild_y as libc::c_int) {
        note(format(b"Wilderness is too big (%u/%u)!\x00" as *const u8 as
                        *const libc::c_char, wild_x_size, wild_y_size) as
                 cptr);
        return 0 as libc::c_int as bool_
    }
    /* Wilderness seeds */
    i = 0 as libc::c_int;
    while i < wild_x_size {
        j = 0 as libc::c_int;
        while j < wild_y_size {
            do_u32b(&mut (*(*wild_map.offset(j as
                                                 isize)).offset(i as
                                                                    isize)).seed,
                    flag);
            do_u16b(&mut (*(*wild_map.offset(j as
                                                 isize)).offset(i as
                                                                    isize)).entrance,
                    flag);
            do_byte(&mut (*(*wild_map.offset(j as
                                                 isize)).offset(i as
                                                                    isize)).known
                        as *mut bool_ as *mut byte_hack, flag);
            j += 1
        }
        i += 1
    }
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded Quests\x00" as *const u8 as *const libc::c_char);
    }
    /* Load the random artifacts. */
    if flag == 7 as libc::c_int { tmp16u = 84 as libc::c_int as u16b }
    do_u16b(&mut tmp16u, flag);
    if flag == 3 as libc::c_int && tmp16u as libc::c_int > 84 as libc::c_int {
        note(format(b"Too many (%u) random artifacts!\x00" as *const u8 as
                        *const libc::c_char, tmp16u as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < tmp16u as libc::c_int {
        let mut ra_ptr: *mut random_artifact =
            &mut *random_artifacts.as_mut_ptr().offset(i as isize) as
                *mut random_artifact;
        do_string((*ra_ptr).name_full.as_mut_ptr(), 80 as libc::c_int, flag);
        do_string((*ra_ptr).name_short.as_mut_ptr(), 80 as libc::c_int, flag);
        do_byte(&mut (*ra_ptr).level, flag);
        do_byte(&mut (*ra_ptr).attr, flag);
        do_u32b(&mut (*ra_ptr).cost, flag);
        do_byte(&mut (*ra_ptr).activation, flag);
        do_byte(&mut (*ra_ptr).generated, flag);
        i += 1
    }
    /* Load the Artifacts */
    if flag == 7 as libc::c_int { tmp16u = max_a_idx }
    do_u16b(&mut tmp16u, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int &&
           tmp16u as libc::c_int > max_a_idx as libc::c_int {
        note(format(b"Too many (%u) artifacts!\x00" as *const u8 as
                        *const libc::c_char, tmp16u as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    /* Read the artifact flags */
    i = 0 as libc::c_int;
    while i < tmp16u as libc::c_int {
        do_byte(&mut (*a_info.offset(i as isize)).cur_num, flag);
        i += 1
    }
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded Artifacts\x00" as *const u8 as *const libc::c_char);
    }
    /* Fates */
    if flag == 7 as libc::c_int { tmp16u = 200 as libc::c_int as u16b }
    do_u16b(&mut tmp16u, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int && tmp16u as libc::c_int > 200 as libc::c_int
       {
        note(format(b"Too many (%u) fates!\x00" as *const u8 as
                        *const libc::c_char, tmp16u as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    /* Read the fate flags */
    i = 0 as libc::c_int;
    while i < tmp16u as libc::c_int { do_fate(i, flag); i += 1 }
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded Fates\x00" as *const u8 as *const libc::c_char);
    }
    /* Load the Traps */
    if flag == 7 as libc::c_int { tmp16u = max_t_idx }
    do_u16b(&mut tmp16u, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int &&
           tmp16u as libc::c_int > max_t_idx as libc::c_int {
        note(format(b"Too many (%u) traps!\x00" as *const u8 as
                        *const libc::c_char, tmp16u as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    /* fate flags */
    i = 0 as libc::c_int;
    while i < tmp16u as libc::c_int {
        do_byte(&mut (*t_info.offset(i as isize)).ident as *mut bool_ as
                    *mut byte_hack, flag);
        i += 1
    }
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded Traps\x00" as *const u8 as *const libc::c_char);
    }
    /* inscription knowledge */
    if flag == 7 as libc::c_int { tmp16u = 8 as libc::c_int as u16b }
    do_u16b(&mut tmp16u, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int && tmp16u as libc::c_int > 8 as libc::c_int {
        note(format(b"Too many (%u) inscriptions!\x00" as *const u8 as
                        *const libc::c_char, tmp16u as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    /* Read the inscription flag */
    i = 0 as libc::c_int;
    while i < tmp16u as libc::c_int {
        do_byte(&mut (*inscription_info.as_mut_ptr().offset(i as isize)).know
                    as *mut bool_ as *mut byte_hack, flag);
        i += 1
    }
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded Inscriptions\x00" as *const u8 as *const libc::c_char);
    }
    /* Read the extra stuff */
    if do_extra(flag) == 0 { return 0 as libc::c_int as bool_ }
    if flag == 3 as libc::c_int && arg_fiddle as libc::c_int != 0 {
        note(b"Loaded extra information\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* player_hp array */
    if flag == 7 as libc::c_int { tmp16u = 50 as libc::c_int as u16b }
    do_u16b(&mut tmp16u, flag);
    /* Incompatible save files */
    if flag == 3 as libc::c_int && tmp16u as libc::c_int > 50 as libc::c_int {
        note(format(b"Too many (%u) hitpoint entries!\x00" as *const u8 as
                        *const libc::c_char, tmp16u as libc::c_int) as cptr);
        return 0 as libc::c_int as bool_
    }
    /* Read the player_hp array */
    i = 0 as libc::c_int;
    while i < tmp16u as libc::c_int {
        do_s16b(&mut *player_hp.as_mut_ptr().offset(i as isize), flag);
        i += 1
    }
    if flag == 3 as libc::c_int { morejunk(); }
    /* Read the pet command settings */
    do_byte(&mut (*p_ptr).pet_follow_distance, flag);
    do_byte(&mut (*p_ptr).pet_open_doors, flag);
    do_byte(&mut (*p_ptr).pet_pickup_items, flag);
    /* Read the inventory */
    if do_inventory(flag) == 0 && flag == 3 as libc::c_int {
        /* do NOT reverse this ordering */
        note(b"Unable to read inventory\x00" as *const u8 as
                 *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Note that this forbids max_towns from shrinking, but that is fine */
    reals =
        memset(ralloc((max_towns as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_towns as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong)) as
            *mut bool_;
    /* Find the real towns */
    if flag == 7 as libc::c_int {
        i = 1 as libc::c_int;
        while i < max_towns as libc::c_int {
            if !((*town_info.offset(i as isize)).flags as libc::c_int &
                     0x1 as libc::c_int == 0) {
                let fresh2 = real_max;
                real_max = real_max.wrapping_add(1);
                *reals.offset(fresh2 as isize) = i as bool_
            }
            i += 1
        }
    }
    do_u16b(&mut real_max, flag);
    i = 0 as libc::c_int;
    while i < real_max as libc::c_int {
        do_byte(&mut *reals.offset(i as isize) as *mut bool_ as
                    *mut byte_hack, flag);
        i += 1
    }
    /* Read the stores */
    if flag == 7 as libc::c_int { tmp16u = max_st_idx }
    do_u16b(&mut tmp16u, flag);
    /* Ok now read the real towns */
    i = 0 as libc::c_int;
    while i < real_max as libc::c_int {
        let mut z: libc::c_int = *reals.offset(i as isize) as libc::c_int;
        /* Ultra paranoia */
        if (*town_info.offset(z as isize)).stocked == 0 {
            create_stores_stock(z);
        }
        j = 0 as libc::c_int;
        while j < tmp16u as libc::c_int {
            do_store(&mut *(*town_info.offset(z as
                                                  isize)).store.offset(j as
                                                                           isize),
                     flag);
            j += 1
        }
        i += 1
    }
    rnfree(reals as vptr,
           (max_towns as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>() as
                                                libc::c_ulong));
    if flag == 7 as libc::c_int { tmp32u = extra_savefile_parts as u32b }
    do_u32b(&mut tmp32u, flag);
    if flag == 7 as libc::c_int {
        /* Save the stuff */
        process_hooks(40 as libc::c_int,
                      b"()\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
    }
    if flag == 3 as libc::c_int {
        let mut len: u32b = tmp32u;
        while len != 0 {
            let mut key_buf: [libc::c_char; 100] = [0; 100];
            /* Load a key */
            load_number_key(key_buf.as_mut_ptr(), &mut tmp32u);
            /* Process it -- the hooks can use it or ignore it */
            process_hooks(41 as libc::c_int,
                          b"(s,l)\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, key_buf.as_mut_ptr(),
                          tmp32u);
            len = len.wrapping_sub(1)
        }
    }
    /* I'm not dead yet... */
    if death == 0 {
        /* Dead players have no dungeon */
        if flag == 3 as libc::c_int {
            note(b"Restoring Dungeon...\x00" as *const u8 as
                     *const libc::c_char);
        }
        if flag == 3 as libc::c_int &&
               do_dungeon(3 as libc::c_int, 0 as libc::c_int as bool_) == 0 {
            note(b"Error reading dungeon data\x00" as *const u8 as
                     *const libc::c_char);
            return 0 as libc::c_int as bool_
        }
        if flag == 7 as libc::c_int {
            do_dungeon(7 as libc::c_int, 0 as libc::c_int as bool_);
        }
        my_sentinel(b"Before ghost data\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    435 as libc::c_int as u16b, flag);
        my_sentinel(b"After ghost data\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    320 as libc::c_int as u16b, flag);
    }
    let mut foo: byte_hack = 0 as libc::c_int as byte_hack;
    if flag == 7 as libc::c_int {
        /*
			 * Safety Padding. It's there
			 * for a good reason. Trust me on
			 * this. Keep this at the *END*
			 * of the file, and do *NOT* try to
			 * read it. Insert any new stuff before
			 * this position.
			 */
        do_byte(&mut foo, 7 as libc::c_int);
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Actually read the savefile
 */
#[no_mangle]
pub unsafe extern "C" fn rd_savefile() -> errr {
    let mut err: errr = 0 as libc::c_int;
    /* The savefile is a binary file */
    fff =
        my_fopen(savefile.as_mut_ptr() as cptr,
                 b"rb\x00" as *const u8 as *const libc::c_char);
    /* Paranoia */
    if fff.is_null() { return -(1 as libc::c_int) }
    /* Call the sub-function */
    err = (do_savefile_aux(3 as libc::c_int) == 0) as libc::c_int;
    /* Check for errors */
    if ferror(fff) != 0 { err = -(1 as libc::c_int) }
    /* Close the file */
    my_fclose(fff);
    /* Result */
    return err;
}
/*
 * Note that this function may not be needed at all.
 * It was taken out of load_player_aux(). Do we need it?
 */
unsafe extern "C" fn junkinit() {
    let mut i: libc::c_int = 0; /* Sex */
    let mut j: libc::c_int = 0; /* Raceclass */
    (*p_ptr).arena_number = 0 as libc::c_int as s16b;
    (*p_ptr).inside_arena = 0 as libc::c_int as s16b;
    (*p_ptr).inside_quest = 0 as libc::c_int as s16b;
    (*p_ptr).exit_bldg = 1 as libc::c_int as bool_;
    (*p_ptr).exit_bldg = 1 as libc::c_int as bool_;
    (*p_ptr).town_num = 1 as libc::c_int as s16b;
    (*p_ptr).wilderness_x = 4 as libc::c_int;
    (*p_ptr).wilderness_y = 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < max_wild_x as libc::c_int {
        j = 0 as libc::c_int;
        while j < max_wild_y as libc::c_int {
            (*(*wild_map.offset(j as isize)).offset(i as isize)).seed =
                Rand_div(0x10000000 as libc::c_int) as u32b;
            j += 1
        }
        i += 1
    };
}
unsafe extern "C" fn morejunk() {
    sp_ptr =
        &mut *sex_info.as_mut_ptr().offset((*p_ptr).psex as isize) as
            *mut player_sex;
    rp_ptr =
        &mut *race_info.offset((*p_ptr).prace as isize) as *mut player_race;
    rmp_ptr =
        &mut *race_mod_info.offset((*p_ptr).pracem as isize) as
            *mut player_race_mod;
    cp_ptr =
        &mut *class_info.offset((*p_ptr).pclass as isize) as
            *mut player_class;
    spp_ptr =
        &mut *(*class_info.offset((*p_ptr).pclass as
                                      isize)).spec.as_mut_ptr().offset((*p_ptr).pspec
                                                                           as
                                                                           isize)
            as *mut player_spec;
}
unsafe extern "C" fn do_grid(mut flag: libc::c_int) 
 /* Does the grid, RLE, blahblah. RLE sucks. I hate it. */
 {
    let mut i: libc::c_int =
        0 as libc::c_int; /* Which section of the grid we're on */
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut count: byte_hack = 0 as libc::c_int as byte_hack;
    let mut tmp8u: byte_hack = 0 as libc::c_int as byte_hack;
    let mut tmp16s: s16b = 0 as libc::c_int as s16b;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut prev_char: byte_hack = 0 as libc::c_int as byte_hack;
    let mut prev_s16b: s16b = 0 as libc::c_int as s16b;
    let mut ymax: libc::c_int = cur_hgt as libc::c_int;
    let mut xmax: libc::c_int = cur_wid as libc::c_int;
    let mut part: libc::c_int = 0;
    part = 0 as libc::c_int;
    while part < 9 as libc::c_int {
        /* There are 8 fields to the grid, each stored
												   in a seperate RLE data structure */
        if flag == 7 as libc::c_int {
            count =
                0 as libc::c_int as byte_hack; /* Clear, prepare for RLE */
            prev_s16b = 0 as libc::c_int as s16b;
            prev_char = 0 as libc::c_int as byte_hack;
            y = 0 as libc::c_int;
            while y < cur_hgt as libc::c_int {
                x = 0 as libc::c_int;
                while x < cur_wid as libc::c_int {
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset(y as
                                                             isize)).offset(x
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    match part {
                        0 => { tmp16s = (*c_ptr).info as s16b }
                        1 => { tmp8u = (*c_ptr).feat }
                        2 => { tmp8u = (*c_ptr).mimic }
                        3 => { tmp16s = (*c_ptr).special }
                        4 => { tmp16s = (*c_ptr).special2 }
                        5 => { tmp16s = (*c_ptr).t_idx }
                        6 => { tmp16s = (*c_ptr).inscription }
                        7 => { tmp8u = (*c_ptr).mana }
                        8 => { tmp16s = (*c_ptr).effect }
                        _ => { }
                    }
                    /* Otherwise, keep going */
                    if part != 1 as libc::c_int && part != 2 as libc::c_int &&
                           part != 7 as libc::c_int &&
                           tmp16s as libc::c_int != prev_s16b as libc::c_int
                           ||
                           (part == 1 as libc::c_int ||
                                part == 2 as libc::c_int ||
                                part == 7 as libc::c_int) &&
                               tmp8u as libc::c_int !=
                                   prev_char as libc::c_int ||
                           count as libc::c_int == 255 as libc::c_int {
                        do_byte(&mut count, 7 as libc::c_int);
                        match part {
                            0 | 3 | 4 | 5 | 6 | 8 => {
                                do_s16b(&mut prev_s16b, 7 as libc::c_int);
                                prev_s16b = tmp16s
                            }
                            1 | 2 | 7 => {
                                do_byte(&mut prev_char, 7 as libc::c_int);
                                prev_char = tmp8u
                            }
                            _ => { }
                        }
                        count = 1 as libc::c_int as byte_hack
                        /* Flush a full run */
                        /* Reset RLE */
                    } else { count = count.wrapping_add(1) }
                    x += 1
                }
                y += 1
            }
            /* Fallen off the end of the world, flush anything left */
            if count != 0 {
                do_byte(&mut count, 7 as libc::c_int);
                match part {
                    0 | 3 | 4 | 5 | 6 | 8 => {
                        do_s16b(&mut prev_s16b, 7 as libc::c_int);
                    }
                    1 | 2 | 7 => {
                        do_byte(&mut prev_char, 7 as libc::c_int);
                    }
                    _ => { }
                }
            }
        }
        if flag == 3 as libc::c_int {
            x = 0 as libc::c_int;
            y = 0 as libc::c_int;
            while y < ymax {
                do_byte(&mut count, 3 as libc::c_int);
                match part {
                    0 | 3 | 4 | 5 | 6 | 8 => {
                        do_s16b(&mut tmp16s, 3 as libc::c_int);
                    }
                    1 | 2 | 7 => { do_byte(&mut tmp8u, 3 as libc::c_int); }
                    _ => { }
                }
                i = count as libc::c_int;
                while i > 0 as libc::c_int {
                    /* RLE */
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset(y as
                                                             isize)).offset(x
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    match part {
                        0 => { (*c_ptr).info = tmp16s as u16b }
                        1 => { (*c_ptr).feat = tmp8u }
                        2 => { (*c_ptr).mimic = tmp8u }
                        3 => { (*c_ptr).special = tmp16s }
                        4 => { (*c_ptr).special2 = tmp16s }
                        5 => { (*c_ptr).t_idx = tmp16s }
                        6 => { (*c_ptr).inscription = tmp16s }
                        7 => { (*c_ptr).mana = tmp8u }
                        8 => { (*c_ptr).effect = tmp16s }
                        _ => { }
                    }
                    x += 1;
                    if x >= xmax {
                        /* Wrap */
                        x = 0 as libc::c_int;
                        y += 1;
                        if y >= ymax { break ; }
                    }
                    i -= 1
                }
            }
        }
        part += 1
    };
}
unsafe extern "C" fn my_sentinel(mut place: *mut libc::c_char,
                                 mut value: u16b, mut flag: libc::c_int) 
 /* This function lets us know exactly where a savefile is
   broken by reading/writing conveniently a sentinel at this
   spot */
 {
    if flag == 7 as libc::c_int { do_u16b(&mut value, flag); return }
    if flag == 3 as libc::c_int {
        let mut found: u16b = 0;
        do_u16b(&mut found, flag);
        if found as libc::c_int == value as libc::c_int {
            /* All is good */
            return
        }
        /* All is bad */
        note(format(b"Savefile broken %s\x00" as *const u8 as
                        *const libc::c_char, place) as
                 cptr); /* Programmer error */
        return
    }
    note(format(b"Impossible has occurred\x00" as *const u8 as
                    *const libc::c_char) as cptr);
    exit(0 as libc::c_int);
}
/* ********* Variable savefile stuff **************/
/*
 * Add num slots to the savefile
 */
#[no_mangle]
pub unsafe extern "C" fn register_savefile(mut num: libc::c_int) {
    extra_savefile_parts +=
        if num > 0 as libc::c_int { num } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn save_number_key(mut key: *mut libc::c_char,
                                         mut val: u32b) {
    let mut len: byte_hack = strlen(key) as byte_hack;
    do_byte(&mut len, 7 as libc::c_int);
    while *key != 0 {
        do_byte(key as *mut byte_hack, 7 as libc::c_int);
        key = key.offset(1)
    }
    do_u32b(&mut val, 7 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn load_number_key(mut key: *mut libc::c_char,
                                         mut val: *mut u32b) {
    let mut len: byte_hack = 0;
    let mut i: byte_hack = 0 as libc::c_int as byte_hack;
    do_byte(&mut len, 3 as libc::c_int);
    while (i as libc::c_int) < len as libc::c_int {
        do_byte(&mut *key.offset(i as isize) as *mut libc::c_char as
                    *mut byte_hack, 3 as libc::c_int);
        i = i.wrapping_add(1)
    }
    *key.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    do_u32b(val, 3 as libc::c_int);
}
