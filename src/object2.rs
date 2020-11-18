use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut player_exp: [s32b; 50];
    #[no_mangle]
    static mut activation_info: [activation; 51];
    #[no_mangle]
    static mut character_dungeon: bool_;
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
    static mut coin_type: s16b;
    #[no_mangle]
    static mut opening_chest: bool_;
    #[no_mangle]
    static mut inven_cnt: s16b;
    #[no_mangle]
    static mut equip_cnt: s16b;
    #[no_mangle]
    static mut o_max: s16b;
    #[no_mangle]
    static mut o_cnt: s16b;
    #[no_mangle]
    static mut artifact_bias: libc::c_int;
    #[no_mangle]
    static mut stack_allow_items: bool_;
    #[no_mangle]
    static mut stack_force_notes: bool_;
    #[no_mangle]
    static mut stack_force_costs: bool_;
    #[no_mangle]
    static mut testing_stack: bool_;
    #[no_mangle]
    static mut cheat_peek: bool_;
    #[no_mangle]
    static mut rating: s16b;
    #[no_mangle]
    static mut good_item_flag: bool_;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut alloc_kind_size: s16b;
    #[no_mangle]
    static mut alloc_kind_table: *mut alloc_entry;
    #[no_mangle]
    static mut alloc_kind_table_valid: bool_;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut e_info: *mut ego_item_type;
    #[no_mangle]
    static mut ra_info: *mut randart_part_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut get_obj_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_a_idx: u16b;
    #[no_mangle]
    static mut max_e_idx: u16b;
    #[no_mangle]
    static mut max_ra_idx: u16b;
    #[no_mangle]
    static mut max_o_idx: u16b;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut RANDART_WEAPON: s32b;
    #[no_mangle]
    static mut RANDART_ARMOR: s32b;
    #[no_mangle]
    static mut RANDART_JEWEL: s32b;
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut k_allow_special: *mut bool_;
    #[no_mangle]
    static mut a_allow_special: *mut bool_;
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    static mut school_spells: *mut spell_type;
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
    fn note_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn randnor(mean: libc::c_int, stand: libc::c_int) -> s16b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn maxroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    static mut Term: *mut term;
    #[no_mangle]
    fn Term_putstr(x: libc::c_int, y: libc::c_int, n: libc::c_int,
                   a: byte_hack, s: cptr) -> errr;
    #[no_mangle]
    fn Term_erase(x: libc::c_int, y: libc::c_int, n: libc::c_int) -> errr;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn get_mon_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn takeoff_set(a_idx: s16b, set_idx: s16b) -> bool_;
    #[no_mangle]
    fn random_resistance(o_ptr: *mut object_type, is_scroll: bool_,
                         specific: libc::c_int);
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc_store(buf: *mut libc::c_char, o_ptr: *mut object_type,
                         pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn index_to_label(i: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn count_bits(array: u32b) -> byte_hack;
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn place_trap_object(o_ptr: *mut object_type);
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luck(min: libc::c_int, max: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn create_artifact(o_ptr: *mut object_type, a_scroll: bool_,
                       get_name: bool_) -> bool_;
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
pub struct randart_part_type {
    pub tval: [byte_hack; 20],
    pub min_sval: [byte_hack; 20],
    pub max_sval: [byte_hack; 20],
    pub level: byte_hack,
    pub rarity: byte_hack,
    pub mrarity: byte_hack,
    pub max_to_h: s16b,
    pub max_to_d: s16b,
    pub max_to_a: s16b,
    pub max_pval: s32b,
    pub value: s32b,
    pub max: s16b,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags4: u32b,
    pub flags5: u32b,
    pub esp: u32b,
    pub fego: u32b,
    pub aflags1: u32b,
    pub aflags2: u32b,
    pub aflags3: u32b,
    pub aflags4: u32b,
    pub aflags5: u32b,
    pub aesp: u32b,
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
pub struct spell_type {
    pub name: cptr,
    pub skill_level: byte_hack,
    pub mana: byte_hack,
    pub mana_max: byte_hack,
    pub fail: s16b,
    pub level: s16b,
}
pub type optimize_flag = libc::c_uint;
pub const NO_OPTIMIZE: optimize_flag = 1;
pub const OPTIMIZE: optimize_flag = 0;
pub type describe_flag = libc::c_uint;
pub const NO_DESCRIBE: describe_flag = 1;
pub const DESCRIBE: describe_flag = 0;
/* File: object2.c */
/* Purpose: Object code, part 2 */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Calculate the player's total inventory weight.
 */
#[no_mangle]
pub unsafe extern "C" fn calc_total_weight() -> s32b {
    let mut i: libc::c_int = 0;
    let mut total: s32b = 0;
    total = 0 as libc::c_int;
    i = total;
    while i < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if (*o_ptr).k_idx != 0 {
            total += (*o_ptr).weight * (*o_ptr).number as libc::c_int
        }
        i += 1
    }
    return total;
}
/*
 * Excise a dungeon object from any stacks
 */
#[no_mangle]
pub unsafe extern "C" fn excise_object_idx(mut o_idx: libc::c_int) {
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut prev_o_idx: s16b = 0 as libc::c_int as s16b;
    /* Object */
    j_ptr = &mut *o_list.offset(o_idx as isize) as *mut object_type;
    /* Monster */
    if (*j_ptr).held_m_idx != 0 {
        let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
        /* Monster */
        m_ptr =
            &mut *m_list.offset((*j_ptr).held_m_idx as isize) as
                *mut monster_type;
        /* Scan all objects in the grid */
        this_o_idx = (*m_ptr).hold_o_idx;
        while this_o_idx != 0 {
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
            /* Acquire object */
            o_ptr =
                &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
            /* Acquire next object */
            next_o_idx = (*o_ptr).next_o_idx;
            /* Done */
            if this_o_idx as libc::c_int == o_idx {
                /* No previous */
                if prev_o_idx as libc::c_int == 0 as libc::c_int {
                    /* Remove from list */
                    (*m_ptr).hold_o_idx = next_o_idx
                } else {
                    /* Real previous */
                    let mut k_ptr: *mut object_type = 0 as *mut object_type;
                    /* Previous object */
                    k_ptr =
                        &mut *o_list.offset(prev_o_idx as isize) as
                            *mut object_type;
                    /* Remove from list */
                    (*k_ptr).next_o_idx = next_o_idx
                }
                /* Forget next pointer */
                (*o_ptr).next_o_idx = 0 as libc::c_int as s16b;
                break ;
            } else {
                /* Save prev_o_idx */
                prev_o_idx = this_o_idx;
                this_o_idx = next_o_idx
            }
        }
    } else {
        /* Dungeon */
        let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
        let mut y: libc::c_int = (*j_ptr).iy as libc::c_int;
        let mut x: libc::c_int = (*j_ptr).ix as libc::c_int;
        /* Grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Scan all objects in the grid */
        this_o_idx = (*c_ptr).o_idx;
        while this_o_idx != 0 {
            let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
            /* Acquire object */
            o_ptr_0 =
                &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
            /* Acquire next object */
            next_o_idx = (*o_ptr_0).next_o_idx;
            /* Done */
            if this_o_idx as libc::c_int == o_idx {
                /* No previous */
                if prev_o_idx as libc::c_int == 0 as libc::c_int {
                    /* Remove from list */
                    (*c_ptr).o_idx = next_o_idx
                } else {
                    /* Real previous */
                    let mut k_ptr_0: *mut object_type = 0 as *mut object_type;
                    /* Previous object */
                    k_ptr_0 =
                        &mut *o_list.offset(prev_o_idx as isize) as
                            *mut object_type;
                    /* Remove from list */
                    (*k_ptr_0).next_o_idx = next_o_idx
                }
                /* Forget next pointer */
                (*o_ptr_0).next_o_idx = 0 as libc::c_int as s16b;
                break ;
            } else {
                /* Save prev_o_idx */
                prev_o_idx = this_o_idx;
                this_o_idx = next_o_idx
            }
        }
    };
}
/*
 * Delete a dungeon object
 *
 * Handle "stacks" of objects correctly.
 */
#[no_mangle]
pub unsafe extern "C" fn delete_object_idx(mut o_idx: libc::c_int) {
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    /* Excise */
    excise_object_idx(o_idx);
    /* Object */
    j_ptr = &mut *o_list.offset(o_idx as isize) as *mut object_type;
    /* Dungeon floor */
    if (*j_ptr).held_m_idx == 0 {
        let mut y: libc::c_int = 0;
        let mut x: libc::c_int = 0;
        /* Location */
        y = (*j_ptr).iy as libc::c_int;
        x = (*j_ptr).ix as libc::c_int;
        /* Visual update */
        lite_spot(y, x);
    }
    /* Wipe the object */
    object_wipe(j_ptr);
    /* Count objects */
    o_cnt -= 1;
}
/*
 * Deletes all objects at given location
 */
#[no_mangle]
pub unsafe extern "C" fn delete_object(mut y: libc::c_int,
                                       mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    /* Refuse "illegal" locations */
    if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
             y < cur_hgt as libc::c_int - 1 as libc::c_int &&
             x < cur_wid as libc::c_int - 1 as libc::c_int) {
        return
    }
    /* Grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Scan all objects in the grid */
    this_o_idx = (*c_ptr).o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Wipe the object */
        object_wipe(o_ptr);
        /* Count objects */
        o_cnt -= 1;
        this_o_idx = next_o_idx
    }
    /* Objects are gone */
    (*c_ptr).o_idx = 0 as libc::c_int as s16b;
    /* Visual update */
    lite_spot(y, x);
}
/*
 * Move an object from index i1 to index i2 in the object list
 */
unsafe extern "C" fn compact_objects_aux(mut i1: libc::c_int,
                                         mut i2: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Do nothing */
    if i1 == i2 { return }
    /* Repair objects */
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        /* Acquire object */
        o_ptr = &mut *o_list.offset(i as isize) as *mut object_type;
        /* Skip "dead" objects */
        if !((*o_ptr).k_idx == 0) {
            /* Repair "next" pointers */
            if (*o_ptr).next_o_idx as libc::c_int == i1 {
                /* Repair */
                (*o_ptr).next_o_idx = i2 as s16b
            }
        }
        i += 1
    }
    /* Acquire object */
    o_ptr = &mut *o_list.offset(i1 as isize) as *mut object_type;
    /* Monster */
    if (*o_ptr).held_m_idx != 0 {
        let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
        /* Acquire monster */
        m_ptr =
            &mut *m_list.offset((*o_ptr).held_m_idx as isize) as
                *mut monster_type;
        /* Repair monster */
        if (*m_ptr).hold_o_idx as libc::c_int == i1 {
            /* Repair */
            (*m_ptr).hold_o_idx = i2 as s16b
        }
    } else {
        /* Dungeon */
        let mut y: libc::c_int = 0;
        let mut x: libc::c_int = 0;
        /* Acquire location */
        y = (*o_ptr).iy as libc::c_int;
        x = (*o_ptr).ix as libc::c_int;
        /* Acquire grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Repair grid */
        if (*c_ptr).o_idx as libc::c_int == i1 {
            /* Repair */
            (*c_ptr).o_idx = i2 as s16b
        }
    }
    /* Structure copy */
    *o_list.offset(i2 as isize) = *o_list.offset(i1 as isize);
    /* Wipe the hole */
    object_wipe(o_ptr);
}
/*
 * Compact and Reorder the object list
 *
 * This function can be very dangerous, use with caution!
 *
 * When actually "compacting" objects, we base the saving throw on a
 * combination of object level, distance from player, and current
 * "desperation".
 *
 * After "compacting" (if needed), we "reorder" the objects into a more
 * compact order, and we reset the allocation info, and the "live" array.
 */
#[no_mangle]
pub unsafe extern "C" fn compact_objects(mut size: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut cur_lev: libc::c_int = 0;
    let mut cur_dis: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    /* Compact */
    if size != 0 {
        /* Message */
        msg_print(b"Compacting objects...\x00" as *const u8 as
                      *const libc::c_char);
        /* Redraw map */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as
                u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b
    }
    /* Compact at least 'size' objects */
    num = 0 as libc::c_int;
    cur_lev = 1 as libc::c_int;
    while num < size {
        /* Get closer each iteration (start at distance 12). Around level 100 distance-protect nothing. */
        cur_dis =
            12 as libc::c_int * (101 as libc::c_int - cur_lev) /
                100 as libc::c_int;
        let mut current_block_22: u64;
        /* Examine the objects */
        i = 1 as libc::c_int;
        while i < o_max as libc::c_int {
            let mut o_ptr: *mut object_type =
                &mut *o_list.offset(i as isize) as *mut object_type;
            let mut k_ptr: *mut object_kind =
                &mut *k_info.offset((*o_ptr).k_idx as isize) as
                    *mut object_kind;
            /* Skip dead objects */
            if !((*o_ptr).k_idx == 0) {
                /* High level objects are "immune" as long as we're not desperate enough */
                if !((*k_ptr).level as libc::c_int > cur_lev) {
                    /* Monster owned objects */
                    if (*o_ptr).held_m_idx != 0 {
                        let mut m_ptr: *mut monster_type =
                            0 as *mut monster_type;
                        /* Acquire monster */
                        m_ptr =
                            &mut *m_list.offset((*o_ptr).held_m_idx as isize)
                                as *mut monster_type;
                        /* Monsters start with protecting objects well */
                        chance = 100 as libc::c_int;
                        /* Get the location */
                        y = (*m_ptr).fy as libc::c_int;
                        x = (*m_ptr).fx as libc::c_int
                    } else {
                        /* Dungeon floor objects */
                        /* Floor objects start with lower protection */
                        chance = 90 as libc::c_int;
                        y = (*o_ptr).iy as libc::c_int;
                        x = (*o_ptr).ix as libc::c_int
                    }
                    /* Get the location */
                    /* Near enough objects are "immune", even if low level			*/
			/* (like, importantly, food rations after hitting a trap of drop items) */
                    if !(cur_dis > 0 as libc::c_int &&
                             distance((*p_ptr).py as libc::c_int,
                                      (*p_ptr).px as libc::c_int, y, x) <
                                 cur_dis) {
                        /* object protection goes down as we get vicious   */
			/* around level 200 only artifacts have protection */
                        chance = chance - cur_lev / 2 as libc::c_int;
                        /* Artifacts */
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
                               (*o_ptr).art_name as libc::c_int != 0 {
                            /* Artifacts are "immune if the level is lower     */
				/* than 300 + artifact level                       */
                            if cur_lev <
                                   300 as libc::c_int +
                                       (*k_ptr).level as libc::c_int {
                                current_block_22 = 3276175668257526147;
                            } else if (*k_ptr).flags3 as libc::c_long &
                                          0x8000 as libc::c_long != 0 &&
                                          cur_lev <
                                              400 as libc::c_int +
                                                  (*k_ptr).level as
                                                      libc::c_int {
                                current_block_22 = 3276175668257526147;
                            } else {
                                /* That's 400 +  level for fixed artifacts */
                                /* Never protect if level is high enough; so we don't wipe a better artifact */
                                chance = -(1 as libc::c_int);
                                /* rewind the level so we never wipe many    */
				/* artifacts of same level if one will do!!! */
                                cur_lev -= 1;
                                current_block_22 = 8693738493027456495;
                            }
                        } else { current_block_22 = 8693738493027456495; }
                        match current_block_22 {
                            3276175668257526147 => { }
                            _ =>
                            /* Maybe some code to spare the God relic here. But I'd rather raise its level to 150 */
                            /* Apply the saving throw */
                            {
                                if !(Rand_div(100 as libc::c_int) < chance) {
                                    /* Delete the object */
                                    delete_object_idx(i);
                                    /* Count it */
                                    num += 1
                                }
                            }
                        }
                    }
                }
            }
            i += 1
        }
        cur_lev += 1
    }
    /* Excise dead objects (backwards!) */
    i = o_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut o_ptr_0: *mut object_type =
            &mut *o_list.offset(i as isize) as *mut object_type;
        /* Skip real objects */
        if !((*o_ptr_0).k_idx != 0) {
            /* Move last object into open hole */
            compact_objects_aux(o_max as libc::c_int - 1 as libc::c_int, i);
            /* Compress "o_max" */
            o_max -= 1
        }
        i -= 1
    };
}
/*
 * Delete all the items when player leaves the level
 *
 * Note -- we do NOT visually reflect these (irrelevant) changes
 *
 * Hack -- we clear the "c_ptr->o_idx" field for every grid,
 * and the "m_ptr->next_o_idx" field for every monster, since
 * we know we are clearing every object.  Technically, we only
 * clear those fields for grids/monsters containing objects,
 * and we clear it once for every such object.
 */
#[no_mangle]
pub unsafe extern "C" fn wipe_o_list() {
    let mut i: libc::c_int = 0;
    /* Delete the existing objects */
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *o_list.offset(i as isize) as *mut object_type;
        /* Skip dead objects */
        if !((*o_ptr).k_idx == 0) {
            /* Mega-Hack -- preserve artifacts */
            if character_dungeon == 0 || (*p_ptr).preserve as libc::c_int != 0
               {
                /* Hack -- Preserve unknown artifacts */
                if ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
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
                       !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int !=
                             0 ||
                             (*k_info.offset((*o_ptr).k_idx as
                                                 isize)).easy_know as
                                 libc::c_int != 0 &&
                                 (*k_info.offset((*o_ptr).k_idx as
                                                     isize)).aware as
                                     libc::c_int != 0) {
                    /* Mega-Hack -- Preserve the artifact */
                    if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
                        random_artifacts[(*o_ptr).sval as usize].generated =
                            0 as libc::c_int as byte_hack
                    } else if (*k_info.offset((*o_ptr).k_idx as isize)).flags3
                                  as libc::c_long & 0x8000 as libc::c_long !=
                                  0 {
                        (*k_info.offset((*o_ptr).k_idx as isize)).artifact =
                            0 as libc::c_int as bool_
                    } else {
                        (*a_info.offset((*o_ptr).name1 as isize)).cur_num =
                            0 as libc::c_int as byte_hack
                    }
                }
            }
            /* Monster */
            if (*o_ptr).held_m_idx != 0 {
                let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
                /* Monster */
                m_ptr =
                    &mut *m_list.offset((*o_ptr).held_m_idx as isize) as
                        *mut monster_type;
                /* Hack -- see above */
                (*m_ptr).hold_o_idx = 0 as libc::c_int as s16b
            } else {
                /* Dungeon */
                let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
                /* Access location */
                let mut y: libc::c_int = (*o_ptr).iy as libc::c_int;
                let mut x: libc::c_int = (*o_ptr).ix as libc::c_int;
                /* Access grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(y as
                                                         isize)).offset(x as
                                                                            isize)
                        as *mut cave_type;
                /* Hack -- see above */
                (*c_ptr).o_idx = 0 as libc::c_int as s16b
            }
            /* Wipe the object */
            o_ptr =
                memset(o_ptr as *mut libc::c_char as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<object_type>() as libc::c_ulong)
                    as *mut object_type
        }
        i += 1
    }
    /* Reset "o_max" */
    o_max = 1 as libc::c_int as s16b;
    /* Reset "o_cnt" */
    o_cnt = 0 as libc::c_int as s16b;
}
/*
 * Acquires and returns the index of a "free" object.
 *
 * This routine should almost never fail, but in case it does,
 * we must be sure to handle "failure" of this routine.
 */
#[no_mangle]
pub unsafe extern "C" fn o_pop() -> s16b {
    let mut i: libc::c_int = 0;
    /* Initial allocation */
    if (o_max as libc::c_int) < max_o_idx as libc::c_int {
        /* Get next space */
        i = o_max as libc::c_int;
        /* Expand object array */
        o_max += 1;
        /* Count objects */
        o_cnt += 1;
        /* Use this object */
        return i as s16b
    }
    /* Recycle dead objects */
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(i as isize) as *mut object_type;
        /* Skip live objects */
        if (*o_ptr).k_idx != 0 {
            i += 1
        } else {
            /* Count objects */
            o_cnt += 1;
            /* Use this object */
            return i as s16b
        }
    }
    /* Warn the player (except during dungeon creation) */
    if character_dungeon != 0 {
        msg_print(b"Too many objects!\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Oops */
    return 0 as libc::c_int as s16b;
}
/*
 * Apply a "object restriction function" to the "object allocation table"
 */
#[no_mangle]
pub unsafe extern "C" fn get_obj_num_prep() -> errr {
    let mut i: libc::c_int = 0;
    /* Get the entry */
    let mut table: *mut alloc_entry = alloc_kind_table;
    /* Scan the allocation table */
    i = 0 as libc::c_int;
    while i < alloc_kind_size as libc::c_int {
        /* Accept objects which pass the restriction, if any */
        if get_obj_num_hook.is_none() ||
               Some(get_obj_num_hook.expect("non-null function pointer")).expect("non-null function pointer")((*table.offset(i
                                                                                                                                 as
                                                                                                                                 isize)).index
                                                                                                                  as
                                                                                                                  libc::c_int)
                   as libc::c_int != 0 {
            /* Accept this object */
            (*table.offset(i as isize)).prob2 =
                (*table.offset(i as isize)).prob1
        } else {
            /* Do not use this object */
            /* Decline this object */
            (*table.offset(i as isize)).prob2 = 0 as libc::c_int as byte_hack
        }
        i += 1
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Choose an object kind that seems "appropriate" to the given level
 *
 * This function uses the "prob2" field of the "object allocation table",
 * and various local information, to calculate the "prob3" field of the
 * same table, which is then used to choose an "appropriate" object, in
 * a relatively efficient manner.
 *
 * It is (slightly) more likely to acquire an object of the given level
 * than one of a lower level.  This is done by choosing several objects
 * appropriate to the given level and keeping the "hardest" one.
 *
 * Note that if no objects are "appropriate", then this function will
 * fail, and return zero, but this should *almost* never happen.
 */
#[no_mangle]
pub unsafe extern "C" fn get_obj_num(mut level: libc::c_int) -> s16b {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut k_idx: libc::c_int = 0;
    let mut value: libc::c_long = 0;
    let mut total: libc::c_long = 0;
    let mut k_ptr: *mut object_kind = 0 as *mut object_kind;
    let mut table: *mut alloc_entry = alloc_kind_table;
    /* Boost level */
    if level > 0 as libc::c_int {
        /* Occasional "boost" */
        if Rand_div(20 as libc::c_int) == 0 as libc::c_int {
            /* What a bizarre calculation */
            level =
                1 as libc::c_int +
                    level * 128 as libc::c_int /
                        (Rand_div(128 as libc::c_int) + 1 as libc::c_int)
        }
    }
    /* Reset total */
    total = 0 as libc::c_long;
    /* Process probabilities */
    i = 0 as libc::c_int;
    while i < alloc_kind_size as libc::c_int {
        /* Objects are sorted by depth */
        if (*table.offset(i as isize)).level as libc::c_int > level {
            break ;
        }
        /* Default */
        (*table.offset(i as isize)).prob3 = 0 as libc::c_int as byte_hack;
        /* Access the index */
        k_idx = (*table.offset(i as isize)).index as libc::c_int;
        /* Access the actual kind */
        k_ptr = &mut *k_info.offset(k_idx as isize) as *mut object_kind;
        /* Hack -- prevent embedded chests */
        if !(opening_chest as libc::c_int != 0 &&
                 (*k_ptr).tval as libc::c_int == 7 as libc::c_int) {
            /* Accept */
            (*table.offset(i as isize)).prob3 =
                (*table.offset(i as isize)).prob2;
            /* Total */
            total += (*table.offset(i as isize)).prob3 as libc::c_long
        }
        i += 1
    }
    /* No legal objects */
    if total <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as s16b
    }
    /* Pick an object */
    value = Rand_div(total as s32b) as libc::c_long;
    /* Find the object */
    i = 0 as libc::c_int;
    while i < alloc_kind_size as libc::c_int {
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
    /* Try for a "better" object once (50%) or twice (10%) */
    if p < 60 as libc::c_int {
        /* Save old */
        j = i;
        /* Pick a object */
        value = Rand_div(total as s32b) as libc::c_long;
        /* Find the monster */
        i = 0 as libc::c_int;
        while i < alloc_kind_size as libc::c_int {
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
    /* Try for a "better" object twice (10%) */
    if p < 10 as libc::c_int {
        /* Save old */
        j = i;
        /* Pick a object */
        value = Rand_div(total as s32b) as libc::c_long;
        /* Find the object */
        i = 0 as libc::c_int;
        while i < alloc_kind_size as libc::c_int {
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
 * Known is true when the "attributes" of an object are "known".
 * These include tohit, todam, toac, cost, and pval (charges).
 *
 * Note that "knowing" an object gives you everything that an "awareness"
 * gives you, and much more.  In fact, the player is always "aware" of any
 * item of which he has full "knowledge".
 *
 * But having full knowledge of, say, one "wand of wonder", does not, by
 * itself, give you knowledge, or even awareness, of other "wands of wonder".
 * It happens that most "identify" routines (including "buying from a shop")
 * will make the player "aware" of the object as well as fully "know" it.
 *
 * This routine also removes any inscriptions generated by "feelings".
 */
#[no_mangle]
pub unsafe extern "C" fn object_known(mut o_ptr: *mut object_type) {
    /* No Sensing */
    (*o_ptr).sense = 0 as libc::c_int as byte_hack;
    /* Clear the "Felt" info */
    (*o_ptr).ident =
        ((*o_ptr).ident as libc::c_int & !(0x1 as libc::c_int)) as byte_hack;
    /* Clear the "Empty" info */
    (*o_ptr).ident =
        ((*o_ptr).ident as libc::c_int & !(0x4 as libc::c_int)) as byte_hack;
    /* Now we know about the item */
    (*o_ptr).ident =
        ((*o_ptr).ident as libc::c_int | 0x8 as libc::c_int) as byte_hack;
}
/*
 * The player is now aware of the effects of the given object.
 */
#[no_mangle]
pub unsafe extern "C" fn object_aware(mut o_ptr: *mut object_type) {
    /* Fully aware of the effects */
    (*k_info.offset((*o_ptr).k_idx as isize)).aware =
        1 as libc::c_int as bool_;
}
/*
 * Something has been "sampled"
 */
#[no_mangle]
pub unsafe extern "C" fn object_tried(mut o_ptr: *mut object_type) {
    /* Mark it as tried (even if "aware") */
    (*k_info.offset((*o_ptr).k_idx as isize)).tried =
        1 as libc::c_int as bool_;
}
/*
 * Return the "value" of an "unknown" item
 * Make a guess at the value of non-aware items
 */
unsafe extern "C" fn object_value_base(mut o_ptr: *mut object_type) -> s32b {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    /* Aware item -- use template cost */
    if (*k_info.offset((*o_ptr).k_idx as isize)).aware as libc::c_int != 0 &&
           (*o_ptr).tval as libc::c_int != 10 as libc::c_int {
        return (*k_ptr).cost
    }
    /* Analyze the type */
    match (*o_ptr).tval as libc::c_int {
        80 => {
            /* Un-aware Food */
            return 5 as libc::c_long as s32b
        }
        72 => {
            /* Un-aware Potions */
            return 20 as libc::c_long as s32b
        }
        71 => {
            /* Un-aware Potions */
            return 20 as libc::c_long as s32b
        }
        70 => {
            /* Un-aware Scrolls */
            return 20 as libc::c_long as s32b
        }
        55 => {
            /* Un-aware Staffs */
            return 70 as libc::c_long as s32b
        }
        65 => {
            /* Un-aware Wands */
            return 50 as libc::c_long as s32b
        }
        66 => {
            /* Un-aware Rods */
            return 90 as libc::c_long as s32b
        }
        45 => {
            /* Un-aware Rings */
            return 45 as libc::c_long as s32b
        }
        40 => {
            /* Un-aware Amulets */
            return 45 as libc::c_long as s32b
        }
        10 => {
            /* Eggs */
            let mut r_ptr: *mut monster_race =
                &mut *r_info.offset((*o_ptr).pval2 as isize) as
                    *mut monster_race;
            /* Pay the monster level */
            return (*r_ptr).level as libc::c_int * 100 as libc::c_int +
                       100 as libc::c_int
        }
        _ => { }
    }
    /* Paranoia -- Oops */
    return 0 as libc::c_long as s32b;
}
/* Return the value of the flags the object has... */
#[no_mangle]
pub unsafe extern "C" fn flag_cost(mut o_ptr: *mut object_type,
                                   mut plusses: libc::c_int) -> s32b {
    let mut total: s32b = 0 as libc::c_int;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f5 as libc::c_long & 0x1 as libc::c_long != 0 {
        return 0 as libc::c_int
    }
    if f4 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        return 0 as libc::c_int
    }
    if f1 as libc::c_long & 0x1 as libc::c_long != 0 {
        total += 1000 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x2 as libc::c_long != 0 {
        total += 1000 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x4 as libc::c_long != 0 {
        total += 1000 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x8 as libc::c_long != 0 {
        total += 1000 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x10 as libc::c_long != 0 {
        total += 1000 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x20 as libc::c_long != 0 {
        total += 250 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x4000 as libc::c_long != 0 {
        total += 10000 as libc::c_int
    }
    if f1 as libc::c_long & 0x8000 as libc::c_long != 0 {
        total += 13000 as libc::c_int
    }
    if f1 as libc::c_long & 0x100 as libc::c_long != 0 {
        total += 250 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x200 as libc::c_long != 0 {
        total += 100 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x400 as libc::c_long != 0 {
        total += 150 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x800 as libc::c_long != 0 {
        total += 175 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x1000 as libc::c_long != 0 &&
           plusses > 0 as libc::c_int {
        total += 10000 as libc::c_int + 2500 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x2000 as libc::c_long != 0 &&
           plusses > 0 as libc::c_int {
        total += 10000 as libc::c_int + 2500 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x40 as libc::c_long != 0 {
        total += 1000 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x80 as libc::c_long != 0 {
        total += 2000 as libc::c_int * plusses
    }
    if f1 as libc::c_long & 0x10000 as libc::c_long != 0 {
        total += 3500 as libc::c_int
    }
    if f1 as libc::c_long & 0x20000 as libc::c_long != 0 {
        total += 4500 as libc::c_int
    }
    if f1 as libc::c_long & 0x40000 as libc::c_long != 0 {
        total += 3500 as libc::c_int
    }
    if f1 as libc::c_long & 0x80000 as libc::c_long != 0 {
        total += 3500 as libc::c_int
    }
    if f1 as libc::c_long & 0x100000 as libc::c_long != 0 {
        total += 3000 as libc::c_int
    }
    if f1 as libc::c_long & 0x200000 as libc::c_long != 0 {
        total += 3500 as libc::c_int
    }
    if f1 as libc::c_long & 0x400000 as libc::c_long != 0 {
        total += 3500 as libc::c_int
    }
    if f1 as libc::c_long & 0x800000 as libc::c_long != 0 {
        total += 3500 as libc::c_int
    }
    if f5 as libc::c_long & 0x8 as libc::c_long != 0 {
        total += 5500 as libc::c_int
    }
    if f5 as libc::c_long & 0x10 as libc::c_long != 0 {
        total += 5500 as libc::c_int
    }
    if f1 as libc::c_long & 0x1000000 as libc::c_long != 0 {
        total += 5500 as libc::c_int
    }
    if f1 as libc::c_long & 0x2000000 as libc::c_long != 0 {
        total += 5000 as libc::c_int
    }
    if f1 as libc::c_long & 0x4000000 as libc::c_long != 0 {
        total += 5000 as libc::c_int
    }
    if f1 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        total += 7500 as libc::c_int
    }
    if f1 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        total += 7500 as libc::c_int
    }
    if f1 as libc::c_long & 0x20000000 as libc::c_long != 0 {
        total += 7500 as libc::c_int
    }
    if f1 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        total += 5000 as libc::c_int
    }
    if f1 as libc::c_long & 0x80000000 as libc::c_long != 0 {
        total += 5000 as libc::c_int
    }
    if f2 as libc::c_long & 0x1 as libc::c_long != 0 {
        total += 850 as libc::c_int
    }
    if f2 as libc::c_long & 0x2 as libc::c_long != 0 {
        total += 850 as libc::c_int
    }
    if f2 as libc::c_long & 0x4 as libc::c_long != 0 {
        total += 850 as libc::c_int
    }
    if f2 as libc::c_long & 0x8 as libc::c_long != 0 {
        total += 850 as libc::c_int
    }
    if f2 as libc::c_long & 0x10 as libc::c_long != 0 {
        total += 850 as libc::c_int
    }
    if f2 as libc::c_long & 0x20 as libc::c_long != 0 {
        total += 250 as libc::c_int
    }
    if f2 as libc::c_long & 0x40 as libc::c_long != 0 {
        total += 3000 as libc::c_int
    }
    if f2 as libc::c_long & 0x80 as libc::c_long != 0 {
        total += 5000 as libc::c_int * plusses
    }
    if f2 as libc::c_long & 0x100 as libc::c_long != 0 {
        total += 10000 as libc::c_int
    }
    if f2 as libc::c_long & 0x200 as libc::c_long != 0 {
        total += 10000 as libc::c_int
    }
    if f2 as libc::c_long & 0x400 as libc::c_long != 0 {
        total += 10000 as libc::c_int
    }
    if f2 as libc::c_long & 0x800 as libc::c_long != 0 {
        total += 10000 as libc::c_int
    }
    if f2 as libc::c_long & 0x1000 as libc::c_long != 0 {
        total -= 100 as libc::c_int
    }
    if f2 as libc::c_long & 0x2000 as libc::c_long != 0 {
        total += 10000 as libc::c_int
    }
    if f2 as libc::c_long & 0x4000 as libc::c_long != 0 {
        total += 4500 as libc::c_int
    }
    if f2 as libc::c_long & 0x8000 as libc::c_long != 0 {
        total += 8500 as libc::c_int
    }
    if f2 as libc::c_long & 0x10000 as libc::c_long != 0 {
        total += 1250 as libc::c_int
    }
    if f2 as libc::c_long & 0x20000 as libc::c_long != 0 {
        total += 1250 as libc::c_int
    }
    if f2 as libc::c_long & 0x40000 as libc::c_long != 0 {
        total += 1250 as libc::c_int
    }
    if f2 as libc::c_long & 0x80000 as libc::c_long != 0 {
        total += 1250 as libc::c_int
    }
    if f2 as libc::c_long & 0x100000 as libc::c_long != 0 {
        total += 2500 as libc::c_int
    }
    if f2 as libc::c_long & 0x200000 as libc::c_long != 0 {
        total += 2500 as libc::c_int
    }
    if f2 as libc::c_long & 0x400000 as libc::c_long != 0 {
        total += 1750 as libc::c_int
    }
    if f2 as libc::c_long & 0x800000 as libc::c_long != 0 {
        total += 1750 as libc::c_int
    }
    if f2 as libc::c_long & 0x1000000 as libc::c_long != 0 {
        total += 2000 as libc::c_int
    }
    if f2 as libc::c_long & 0x2000000 as libc::c_long != 0 {
        total += 2000 as libc::c_int
    }
    if f2 as libc::c_long & 0x4000000 as libc::c_long != 0 {
        total += 2000 as libc::c_int
    }
    if f2 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        total += 2000 as libc::c_int
    }
    if f2 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        total += 2000 as libc::c_int
    }
    if f2 as libc::c_long & 0x20000000 as libc::c_long != 0 {
        total += 2000 as libc::c_int
    }
    if f2 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        total += 2000 as libc::c_int
    }
    if f2 as libc::c_long & 0x80000000 as libc::c_long != 0 {
        total += 10000 as libc::c_int
    }
    if f3 as libc::c_long & 0x1 as libc::c_long != 0 {
        total += 5000 as libc::c_int
    }
    if f3 as libc::c_long & 0x2 as libc::c_long != 0 {
        total += 5000 as libc::c_int
    }
    if f3 as libc::c_long & 0x8 as libc::c_long != 0 {
        total += 0 as libc::c_int
    }
    if f3 as libc::c_long & 0x10 as libc::c_long != 0 {
        total += 2500 as libc::c_int
    }
    if f3 as libc::c_long & 0x20 as libc::c_long != 0 {
        total += 2500 as libc::c_int
    }
    if f3 as libc::c_long & 0x40 as libc::c_long != 0 {
        total += 250000 as libc::c_int
    }
    if f3 as libc::c_long & 0x80 as libc::c_long != 0 {
        total -= 15000 as libc::c_int
    }
    if f3 as libc::c_long & 0x100 as libc::c_long != 0 {
        total += 0 as libc::c_int
    }
    if f3 as libc::c_long & 0x200 as libc::c_long != 0 {
        total += 0 as libc::c_int
    }
    if f3 as libc::c_long & 0x400 as libc::c_long != 0 {
        total += 0 as libc::c_int
    }
    if f3 as libc::c_long & 0x800 as libc::c_long != 0 {
        total += 0 as libc::c_int
    }
    if f3 as libc::c_long & 0x2000 as libc::c_long != 0 {
        total += 750 as libc::c_int
    }
    if f4 as libc::c_long & 0x4000000 as libc::c_long != 0 {
        total += 1250 as libc::c_int
    }
    if f4 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        total += 2750 as libc::c_int
    }
    if f3 as libc::c_long & 0x4000 as libc::c_long != 0 {
        total += 2000 as libc::c_int
    }
    if esp != 0 {
        total += 12500 as libc::c_int * count_bits(esp) as libc::c_int
    }
    if f3 as libc::c_long & 0x10000 as libc::c_long != 0 {
        total += 750 as libc::c_int
    }
    if f3 as libc::c_long & 0x20000 as libc::c_long != 0 {
        total += 2500 as libc::c_int
    }
    if f3 as libc::c_long & 0x40000 as libc::c_long != 0 {
        total += 2250 as libc::c_int
    }
    if f3 as libc::c_long & 0x80000 as libc::c_long != 0 {
        total += 10000 as libc::c_int
    }
    if f3 as libc::c_long & 0x100000 as libc::c_long != 0 {
        total += 100 as libc::c_int
    }
    if f3 as libc::c_long & 0x200000 as libc::c_long != 0 {
        total += 100 as libc::c_int
    }
    if f3 as libc::c_long & 0x400000 as libc::c_long != 0 {
        total += 100 as libc::c_int
    }
    if f3 as libc::c_long & 0x800000 as libc::c_long != 0 {
        total += 100 as libc::c_int
    }
    if f3 as libc::c_long & 0x1000000 as libc::c_long != 0 {
        total += 100 as libc::c_int
    }
    if f3 as libc::c_long & 0x2000000 as libc::c_long != 0 {
        total -= 12500 as libc::c_int
    }
    if f3 as libc::c_long & 0x4000000 as libc::c_long != 0 {
        if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
            total -= 7500 as libc::c_int
        } else { total += 250 as libc::c_int }
    }
    if f3 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        total -= 10000 as libc::c_int
    }
    if f3 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        total += 750 as libc::c_int
    }
    if f3 as libc::c_long & 0x20000000 as libc::c_long != 0 &&
           (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        total -= 5000 as libc::c_int
    }
    if f3 as libc::c_long & 0x40000000 as libc::c_long != 0 &&
           (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        total -= 12500 as libc::c_int
    }
    if f3 as libc::c_long & 0x80000000 as libc::c_long != 0 {
        total -= 15000 as libc::c_int
    }
    if f3 as libc::c_long & 0x1000 as libc::c_long != 0 {
        total += 1250 as libc::c_int
    }
    if f4 as libc::c_long & 0x10 as libc::c_long != 0 {
        total += 10000 as libc::c_int
    }
    if f4 as libc::c_long & 0x1 as libc::c_long != 0 {
        total -= 15000 as libc::c_int
    }
    if f4 as libc::c_long & 0x2 as libc::c_long != 0 {
        total += 250000 as libc::c_int
    }
    if f4 as libc::c_long & 0x4 as libc::c_long != 0 {
        total -= 12500 as libc::c_int
    }
    if f4 as libc::c_long & 0x20 as libc::c_long != 0 {
        total -= 25000 as libc::c_int
    }
    if f4 as libc::c_long & 0x200 as libc::c_long != 0 {
        total -= 10000 as libc::c_int
    }
    if f4 as libc::c_long & 0x100 as libc::c_long != 0 {
        total += (*o_ptr).elevel as libc::c_int * 2000 as libc::c_int
    }
    /* Also, give some extra for activatable powers... */
    if (*o_ptr).art_name as libc::c_int != 0 &&
           (*o_ptr).art_flags3 as libc::c_long & 0x1000000 as libc::c_long !=
               0 {
        let mut type_0: libc::c_int = (*o_ptr).xtra2 as libc::c_int;
        if type_0 == 1 as libc::c_int {
            total += 250 as libc::c_int
        } else if type_0 == 2 as libc::c_int {
            total += 250 as libc::c_int
        } else if type_0 == 3 as libc::c_int {
            total += 300 as libc::c_int
        } else if type_0 == 4 as libc::c_int {
            total += 250 as libc::c_int
        } else if type_0 == 5 as libc::c_int {
            total += 250 as libc::c_int
        } else if type_0 == 6 as libc::c_int {
            total += 250 as libc::c_int
        } else if type_0 == 7 as libc::c_int {
            total += 250 as libc::c_int
        } else if type_0 == 8 as libc::c_int {
            total += 750 as libc::c_int
        } else if type_0 == 9 as libc::c_int {
            total += 1000 as libc::c_int
        } else if type_0 == 10 as libc::c_int {
            total += 500 as libc::c_int
        } else if type_0 == 11 as libc::c_int {
            total += 1250 as libc::c_int
        } else if type_0 == 12 as libc::c_int {
            total += 1500 as libc::c_int
        } else if type_0 == 13 as libc::c_int {
            total += 750 as libc::c_int
        } else if type_0 == 14 as libc::c_int {
            total += 1000 as libc::c_int
        } else if type_0 == 15 as libc::c_int {
            total += 1000 as libc::c_int
        } else if type_0 == 16 as libc::c_int {
            total += 1750 as libc::c_int
        } else if type_0 == 17 as libc::c_int {
            total += 2500 as libc::c_int
        } else if type_0 == 18 as libc::c_int {
            total += 2500 as libc::c_int
        } else if type_0 == 19 as libc::c_int {
            total += 7500 as libc::c_int
        } else if type_0 == 20 as libc::c_int {
            total += 2500 as libc::c_int
        } else if type_0 == 21 as libc::c_int {
            total += 5000 as libc::c_int
        } else if type_0 == 22 as libc::c_int {
            total += 5000 as libc::c_int
        } else if type_0 == 23 as libc::c_int {
            total += 4000 as libc::c_int
        } else if type_0 == 25 as libc::c_int {
            total += 3500 as libc::c_int
        } else if type_0 == 24 as libc::c_int {
            total += 5000 as libc::c_int
        } else if type_0 == 51 as libc::c_int {
            total += 500 as libc::c_int
        } else if type_0 == 52 as libc::c_int {
            total += 750 as libc::c_int
        } else if type_0 == 53 as libc::c_int {
            total += 600 as libc::c_int
        } else if type_0 == 54 as libc::c_int {
            total += 2500 as libc::c_int
        } else if type_0 == 55 as libc::c_int {
            total += 2000 as libc::c_int
        } else if type_0 == 57 as libc::c_int {
            total += 10000 as libc::c_int
        } else if type_0 == 58 as libc::c_int {
            total += 10000 as libc::c_int
        } else if type_0 == 65 as libc::c_int {
            total += 7500 as libc::c_int
        } else if type_0 == 66 as libc::c_int {
            total += 10000 as libc::c_int
        } else if type_0 == 67 as libc::c_int {
            total += 10000 as libc::c_int
        } else if type_0 == 68 as libc::c_int {
            total += 12500 as libc::c_int
        } else if type_0 == 69 as libc::c_int {
            total += 17500 as libc::c_int
        } else if type_0 == 70 as libc::c_int {
            total += 10000 as libc::c_int
        } else if type_0 == 71 as libc::c_int {
            total += 12000 as libc::c_int
        } else if type_0 == 72 as libc::c_int {
            total += 15000 as libc::c_int
        } else if type_0 == 73 as libc::c_int {
            total += 20000 as libc::c_int
        } else if type_0 == 74 as libc::c_int {
            total += 20000 as libc::c_int
        } else if type_0 == 81 as libc::c_int {
            total += 500 as libc::c_int
        } else if type_0 == 82 as libc::c_int {
            total += 750 as libc::c_int
        } else if type_0 == 84 as libc::c_int {
            total += 7500 as libc::c_int
        } else if type_0 == 85 as libc::c_int {
            total += 15000 as libc::c_int
        } else if type_0 == 86 as libc::c_int {
            total += 10000 as libc::c_int
        } else if type_0 == 87 as libc::c_int {
            total += 15000 as libc::c_int
        } else if type_0 == 91 as libc::c_int {
            total += 1500 as libc::c_int
        } else if type_0 == 92 as libc::c_int {
            total += 800 as libc::c_int
        } else if type_0 == 93 as libc::c_int {
            total += 5000 as libc::c_int
        } else if type_0 == 94 as libc::c_int {
            total += 5000 as libc::c_int
        } else if type_0 == 95 as libc::c_int {
            total += 15000 as libc::c_int
        } else if type_0 == 96 as libc::c_int {
            total += 25000 as libc::c_int
        } else if type_0 == 97 as libc::c_int {
            total += 25000 as libc::c_int
        } else if type_0 == 98 as libc::c_int {
            total += 25000 as libc::c_int
        } else if type_0 == 111 as libc::c_int {
            total += 150 as libc::c_int
        } else if type_0 == 112 as libc::c_int {
            total += 500 as libc::c_int
        } else if type_0 == 113 as libc::c_int {
            total += 1000 as libc::c_int
        } else if type_0 == 114 as libc::c_int {
            total += 12500 as libc::c_int
        } else if type_0 == 115 as libc::c_int {
            total += 10000 as libc::c_int
        } else if type_0 == 116 as libc::c_int {
            total += 1250 as libc::c_int
        } else if type_0 == 117 as libc::c_int {
            total += 4000 as libc::c_int
        } else if type_0 == 118 as libc::c_int {
            total += 10000 as libc::c_int
        } else if type_0 == 119 as libc::c_int {
            total += 2000 as libc::c_int
        } else if type_0 == 120 as libc::c_int {
            total += 100 as libc::c_int
        } else if type_0 == 121 as libc::c_int {
            total += 1000 as libc::c_int
        } else if type_0 == 122 as libc::c_int {
            total += 1000 as libc::c_int
        } else if type_0 == 123 as libc::c_int {
            total += 10000 as libc::c_int
        } else if type_0 == 124 as libc::c_int {
            total += 10000 as libc::c_int
        } else if type_0 == 125 as libc::c_int {
            total += 2000 as libc::c_int
        } else if type_0 == 126 as libc::c_int {
            total += 7500 as libc::c_int
        }
    }
    return total;
}
/*
 * Return the "real" price of a "known" item, not including discounts
 *
 * Wand and staffs get cost for each charge
 *
 * Armor is worth an extra 100 gold per bonus point to armor class.
 *
 * Weapons are worth an extra 100 gold per bonus point (AC,TH,TD).
 *
 * Missiles are only worth 5 gold per bonus point, since they
 * usually appear in groups of 20, and we want the player to get
 * the same amount of cash for any "equivalent" item.  Note that
 * missiles never have any of the "pval" flags, and in fact, they
 * only have a few of the available flags, primarily of the "slay"
 * and "brand" and "ignore" variety.
 *
 * Armor with a negative armor bonus is worthless.
 * Weapons with negative hit+damage bonuses are worthless.
 *
 * Every wearable item with a "pval" bonus is worth extra (see below).
 */
#[no_mangle]
pub unsafe extern "C" fn object_value_real(mut o_ptr: *mut object_type)
 -> s32b {
    let mut value: s32b = 0;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
        return random_artifacts[(*o_ptr).sval as usize].cost as s32b
    }
    /* Hack -- "worthless" items */
    if (*k_ptr).cost == 0 { return 0 as libc::c_long as s32b }
    /* Base cost */
    value = (*k_ptr).cost;
    /* Extract some flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f5 as libc::c_long & 0x1 as libc::c_long != 0 {
        return 0 as libc::c_long as s32b
    }
    if (*o_ptr).art_flags1 != 0 || (*o_ptr).art_flags2 != 0 ||
           (*o_ptr).art_flags3 != 0 {
        value += flag_cost(o_ptr, (*o_ptr).pval)
    } else if (*o_ptr).name1 != 0 {
        let mut a_ptr: *mut artifact_type =
            &mut *a_info.offset((*o_ptr).name1 as isize) as
                *mut artifact_type;
        /* Artifact */
        /* Hack -- "worthless" artifacts */
        if (*a_ptr).cost == 0 { return 0 as libc::c_long as s32b }
        /* Hack -- Use the artifact cost instead */
        value = (*a_ptr).cost
    } else if (*o_ptr).name2 != 0 {
        let mut e_ptr: *mut ego_item_type =
            &mut *e_info.offset((*o_ptr).name2 as isize) as
                *mut ego_item_type;
        /* Ego-Item */
        /* Hack -- "worthless" ego-items */
        if (*e_ptr).cost == 0 { return 0 as libc::c_long as s32b }
        /* Hack -- Reward the ego-item with a bonus */
        value += (*e_ptr).cost;
        if (*o_ptr).name2b != 0 {
            let mut e_ptr_0: *mut ego_item_type =
                &mut *e_info.offset((*o_ptr).name2b as isize) as
                    *mut ego_item_type;
            /* Hack -- "worthless" ego-items */
            if (*e_ptr_0).cost == 0 { return 0 as libc::c_long as s32b }
            /* Hack -- Reward the ego-item with a bonus */
            value += (*e_ptr_0).cost
        }
    }
    /* Pay the spell */
    if f5 as libc::c_long & 0x800 as libc::c_long != 0 {
        if (*o_ptr).pval2 as libc::c_int != -(1 as libc::c_int) {
            value +=
                5000 as libc::c_int +
                    500 as libc::c_int *
                        (*school_spells.offset((*o_ptr).pval2 as
                                                   isize)).skill_level as
                            libc::c_int
        } else { value += 5000 as libc::c_int }
    }
    /* Analyze pval bonus */
    match (*o_ptr).tval as libc::c_int {
        19 | 15 | 20 | 21 | 22 | 23 | 24 | 30 | 31 | 32 | 33 | 34 | 35 | 36 |
        37 | 38 | 39 | 40 | 45 | 6 | 46 | 14 => {
            /* No pval */
            if !((*o_ptr).pval == 0) {
                /* Give credit for stat bonuses */
                if f1 as libc::c_long & 0x1 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 200 as libc::c_long) as s32b
                }
                if f1 as libc::c_long & 0x2 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 200 as libc::c_long) as s32b
                }
                if f1 as libc::c_long & 0x4 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 200 as libc::c_long) as s32b
                }
                if f1 as libc::c_long & 0x8 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 200 as libc::c_long) as s32b
                }
                if f1 as libc::c_long & 0x10 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 200 as libc::c_long) as s32b
                }
                if f1 as libc::c_long & 0x20 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 200 as libc::c_long) as s32b
                }
                if f5 as libc::c_long & 0x20 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 500 as libc::c_long) as s32b
                }
                /* Give credit for stealth and searching */
                if f1 as libc::c_long & 0x100 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 100 as libc::c_long) as s32b
                }
                if f1 as libc::c_long & 0x200 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 100 as libc::c_long) as s32b
                }
                /* Give credit for infra-vision and tunneling */
                if f1 as libc::c_long & 0x400 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 50 as libc::c_long) as s32b
                }
                if f1 as libc::c_long & 0x800 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 50 as libc::c_long) as s32b
                }
                /* Give credit for extra attacks */
                if f1 as libc::c_long & 0x2000 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 2000 as libc::c_long) as s32b
                }
                /* Give credit for speed bonus */
                if f1 as libc::c_long & 0x1000 as libc::c_long != 0 {
                    value =
                        (value as libc::c_long +
                             (*o_ptr).pval as libc::c_long *
                                 30000 as libc::c_long) as s32b
                }
            }
        }
        _ => { }
    }
    /* Analyze the item */
    match (*o_ptr).tval as libc::c_int {
        10 => {
            /* Eggs */
            let mut r_ptr: *mut monster_race =
                &mut *r_info.offset((*o_ptr).pval2 as isize) as
                    *mut monster_race;
            /* Pay the monster level */
            value += (*r_ptr).level as libc::c_int * 100 as libc::c_int
        }
        65 => {
            /* Wands/Staffs */
            /* Par for the spell */
            value *=
                (*school_spells.offset((*o_ptr).pval2 as isize)).skill_level
                    as libc::c_int;
            /* Take the average of the base and max spell levels */
            value *=
                (((*o_ptr).pval3 >> 16 as libc::c_int & 0xffff as libc::c_int)
                     + ((*o_ptr).pval3 & 0xffff as libc::c_int)) /
                    2 as libc::c_int;
            /* Hack */
            value /= 6 as libc::c_int;
            /* Pay extra for charges */
            value +=
                value / 20 as libc::c_int * (*o_ptr).pval /
                    (*o_ptr).number as libc::c_int
        }
        55 => {
            /* Par for the spell */
            value *=
                (*school_spells.offset((*o_ptr).pval2 as isize)).skill_level
                    as libc::c_int;
            /* Take the average of the base and max spell levels */
            value *=
                (((*o_ptr).pval3 >> 16 as libc::c_int & 0xffff as libc::c_int)
                     + ((*o_ptr).pval3 & 0xffff as libc::c_int)) /
                    2 as libc::c_int;
            /* Hack */
            value /= 6 as libc::c_int;
            /* Pay extra for charges */
            value += value / 20 as libc::c_int * (*o_ptr).pval
        }
        111 => {
            if (*o_ptr).sval as libc::c_int == 255 as libc::c_int {
                /* Pay extra for the spell */
                value =
                    value *
                        (*school_spells.offset((*o_ptr).pval as
                                                   isize)).skill_level as
                            libc::c_int
            }
        }
        67 => {
            /* Rods */
            let mut tip_idx: s16b = 0;
            /* It's not combined */
            if !((*o_ptr).pval == 0 as libc::c_int) {
                /* Look up the tip attached */
                tip_idx = lookup_kind(66 as libc::c_int, (*o_ptr).pval);
                /* Paranoia */
                if tip_idx as libc::c_int > 0 as libc::c_int {
                    /* Add its cost */
                    value += (*k_info.offset(tip_idx as isize)).cost
                }
            }
        }
        45 | 40 => {
            /* Rings/Amulets */
            /* Hack -- negative bonuses are bad */
            if ((*o_ptr).to_a as libc::c_int) < 0 as libc::c_int && value == 0
               {
                return 0 as libc::c_long as s32b
            }
            if ((*o_ptr).to_h as libc::c_int) < 0 as libc::c_int && value == 0
               {
                return 0 as libc::c_long as s32b
            }
            if ((*o_ptr).to_d as libc::c_int) < 0 as libc::c_int && value == 0
               {
                return 0 as libc::c_long as s32b
            }
            /* Give credit for bonuses */
            value =
                (value as libc::c_long +
                     ((*o_ptr).to_h as libc::c_int +
                          (*o_ptr).to_d as libc::c_int +
                          (*o_ptr).to_a as libc::c_int) as libc::c_long *
                         100 as libc::c_long) as s32b
        }
        30 | 31 | 35 | 33 | 32 | 34 | 36 | 37 | 38 => {
            /* Armor */
            /* Hack -- negative armor bonus */
            if ((*o_ptr).to_a as libc::c_int) < 0 as libc::c_int && value == 0
               {
                return 0 as libc::c_long as s32b
            }
            /* Give credit for bonuses */
            value =
                (value as libc::c_long +
                     ((*o_ptr).to_h as libc::c_int +
                          (*o_ptr).to_d as libc::c_int +
                          (*o_ptr).to_a as libc::c_int) as libc::c_long *
                         100 as libc::c_long) as s32b
        }
        19 | 15 | 20 | 21 | 23 | 115 | 24 | 22 | 46 => {
            /* Bows/Weapons */
            /* Hack -- negative hit/damage bonuses */
            if ((*o_ptr).to_h as libc::c_int + (*o_ptr).to_d as libc::c_int) <
                   0 as libc::c_int && value == 0 {
                return 0 as libc::c_long as s32b
            }
            /* Factor in the bonuses */
            value =
                (value as libc::c_long +
                     ((*o_ptr).to_h as libc::c_int +
                          (*o_ptr).to_d as libc::c_int +
                          (*o_ptr).to_a as libc::c_int) as libc::c_long *
                         100 as libc::c_long) as s32b;
            /* Hack -- Factor in extra damage dice */
            if (*o_ptr).dd as libc::c_int > (*k_ptr).dd as libc::c_int &&
                   (*o_ptr).ds as libc::c_int == (*k_ptr).ds as libc::c_int {
                value =
                    (value as libc::c_long +
                         (((*o_ptr).dd as libc::c_int -
                               (*k_ptr).dd as libc::c_int) *
                              (*o_ptr).ds as libc::c_int) as libc::c_long *
                             100 as libc::c_long) as s32b
            }
        }
        16 | 17 | 18 => {
            /* Ammo */
            /* Hack -- negative hit/damage bonuses */
            if ((*o_ptr).to_h as libc::c_int + (*o_ptr).to_d as libc::c_int) <
                   0 as libc::c_int && value == 0 {
                return 0 as libc::c_long as s32b
            }
            /* Factor in the bonuses */
            value =
                (value as libc::c_long +
                     ((*o_ptr).to_h as libc::c_int +
                          (*o_ptr).to_d as libc::c_int) as libc::c_long *
                         5 as libc::c_long) as s32b;
            /* Hack -- Factor in extra damage dice */
            if (*o_ptr).dd as libc::c_int > (*k_ptr).dd as libc::c_int &&
                   (*o_ptr).ds as libc::c_int == (*k_ptr).ds as libc::c_int {
                value =
                    (value as libc::c_long +
                         (((*o_ptr).dd as libc::c_int -
                               (*k_ptr).dd as libc::c_int) *
                              (*o_ptr).ds as libc::c_int) as libc::c_long *
                             5 as libc::c_long) as s32b
            }
            /* Special attack (exploding arrow) */
            if (*o_ptr).pval2 as libc::c_int != 0 as libc::c_int {
                value *= 14 as libc::c_int
            }
        }
        _ => { }
    }
    /* Return the value */
    return value;
}
/*
 * Return the price of an item including plusses (and charges)
 *
 * This function returns the "value" of the given item (qty one)
 *
 * Never notice "unknown" bonuses or properties, including "curses",
 * since that would give the player information he did not have.
 *
 * Note that discounted items stay discounted forever, even if
 * the discount is "forgotten" by the player via memory loss.
 */
#[no_mangle]
pub unsafe extern "C" fn object_value(mut o_ptr: *mut object_type) -> s32b {
    let mut value: s32b = 0;
    /* Unknown items -- acquire a base value */
    if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
           (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as libc::c_int
               != 0 &&
               (*k_info.offset((*o_ptr).k_idx as isize)).aware as libc::c_int
                   != 0 {
        /* Cursed items -- worthless */
        if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
            return 0 as libc::c_long as s32b
        }
        /* Real value (see above) */
        value = object_value_real(o_ptr)
    } else {
        /* Known items -- acquire the actual value */
        /* Hack -- Felt cursed items */
        if (*o_ptr).ident as libc::c_int & 0x1 as libc::c_int != 0 &&
               (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
            return 0 as libc::c_long as s32b
        }
        value = object_value_base(o_ptr)
    }
    /* Base value (see above) */
    /* Apply discount (if any) */
    if (*o_ptr).discount != 0 {
        value =
            (value as libc::c_long -
                 (value * (*o_ptr).discount as libc::c_int) as libc::c_long /
                     100 as libc::c_long) as s32b
    }
    /* Return the final value */
    return value;
}
/*
 * Determine if an item can "absorb" a second item
 *
 * See "object_absorb()" for the actual "absorption" code.
 *
 * If permitted, we allow wands/staffs (if they are known to have equal
 * charges) and rods (if fully charged) to combine.  They will unstack
 * (if necessary) when they are used.
 *
 * If permitted, we allow weapons/armor to stack, if fully "known".
 *
 * Missiles will combine if both stacks have the same "known" status.
 * This is done to make unidentified stacks of missiles useful.
 *
 * Food, potions, scrolls, and "easy know" items always stack.
 *
 * Chests, and activatable items, never stack (for various reasons).
 */
#[no_mangle]
pub unsafe extern "C" fn object_similar(mut o_ptr: *mut object_type,
                                        mut j_ptr: *mut object_type)
 -> bool_ {
    let mut total: libc::c_int =
        (*o_ptr).number as libc::c_int + (*j_ptr).number as libc::c_int;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut f11: u32b = 0;
    let mut f12: u32b = 0;
    let mut f13: u32b = 0;
    let mut f14: u32b = 0;
    let mut esp1: u32b = 0;
    let mut f15: u32b = 0;
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    object_flags(j_ptr, &mut f11, &mut f12, &mut f13, &mut f14, &mut f15,
                 &mut esp1);
    /* Require identical object types */
    if (*o_ptr).k_idx as libc::c_int != (*j_ptr).k_idx as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if f5 as libc::c_long & 0x800 as libc::c_long != 0 ||
           f15 as libc::c_long & 0x800 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    let mut current_block_116: u64;
    /* Analyze the items */
    match (*o_ptr).tval as libc::c_int {
        111 => {
            /* School Book */
            if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                     (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                         libc::c_int != 0 &&
                         (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                             libc::c_int != 0) ||
                   !((*j_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0
                         ||
                         (*k_info.offset((*j_ptr).k_idx as isize)).easy_know
                             as libc::c_int != 0 &&
                             (*k_info.offset((*j_ptr).k_idx as isize)).aware
                                 as libc::c_int != 0) {
                return 0 as libc::c_int as bool_
            }
            /* Beware artifatcs should not combibne with "lesser" thing */
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
                     } else { 0 as libc::c_int }) != 0) as libc::c_int !=
                   ((*j_ptr).tval as libc::c_int == 102 as libc::c_int ||
                        (if (*j_ptr).name1 as libc::c_int != 0 {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0 ||
                        (if (*j_ptr).art_name as libc::c_int != 0 {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0 ||
                        (if (*k_info.offset((*j_ptr).k_idx as isize)).flags3
                                as libc::c_long & 0x8000 as libc::c_long != 0
                            {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0) as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine different ego or normal ones */
            if (if (*o_ptr).name2 as libc::c_int != 0 ||
                       (*o_ptr).name2b as libc::c_int != 0 {
                    1 as libc::c_int
                } else { 0 as libc::c_int }) !=
                   (if (*j_ptr).name2 as libc::c_int != 0 ||
                           (*j_ptr).name2b as libc::c_int != 0 {
                        1 as libc::c_int
                    } else { 0 as libc::c_int }) {
                return 0 as libc::c_int as bool_
            }
            /* Random books should stack if they are identical */
            if (*o_ptr).sval as libc::c_int == 255 as libc::c_int &&
                   (*j_ptr).sval as libc::c_int == 255 as libc::c_int {
                if (*o_ptr).pval != (*j_ptr).pval {
                    return 0 as libc::c_int as bool_
                }
            }
            return 1 as libc::c_int as bool_
        }
        7 => {
            /* Chests */
            /* Never okay */
            return 0 as libc::c_int as bool_
        }
        102 => { return 0 as libc::c_int as bool_ }
        104 => { return 1 as libc::c_int as bool_ }
        105 => {
            if (*o_ptr).sval as libc::c_int == 0xff as libc::c_int ||
                   (*j_ptr).sval as libc::c_int == 0xff as libc::c_int {
                return 0 as libc::c_int as bool_
            } else { return 1 as libc::c_int as bool_ }
        }
        14 => { return 0 as libc::c_int as bool_ }
        99 | 10 => { return 0 as libc::c_int as bool_ }
        54 => {
            /* Totems */
            if (*o_ptr).pval == (*j_ptr).pval &&
                   (*o_ptr).pval2 as libc::c_int ==
                       (*j_ptr).pval2 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            return 0 as libc::c_int as bool_
        }
        9 => {
            /* Corpses*/
            return 0 as libc::c_int as bool_
        }
        71 | 72 => {
            /* Food and Potions and Scrolls */
            if (*o_ptr).pval2 as libc::c_int != (*j_ptr).pval2 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            /* Assume okay */
            current_block_116 = 8225018548522317130;
        }
        70 => {
            if (*o_ptr).pval != (*j_ptr).pval {
                return 0 as libc::c_int as bool_
            }
            if (*o_ptr).pval2 as libc::c_int != (*j_ptr).pval2 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            current_block_116 = 8225018548522317130;
        }
        55 => {
            /* Staffs */
            /* Require either knowledge or known empty for both staffs. */
            if (*o_ptr).ident as libc::c_int & 0x4 as libc::c_int == 0 &&
                   !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0
                         ||
                         (*k_info.offset((*o_ptr).k_idx as isize)).easy_know
                             as libc::c_int != 0 &&
                             (*k_info.offset((*o_ptr).k_idx as isize)).aware
                                 as libc::c_int != 0) ||
                   (*j_ptr).ident as libc::c_int & 0x4 as libc::c_int == 0 &&
                       !((*j_ptr).ident as libc::c_int & 0x8 as libc::c_int !=
                             0 ||
                             (*k_info.offset((*j_ptr).k_idx as
                                                 isize)).easy_know as
                                 libc::c_int != 0 &&
                                 (*k_info.offset((*j_ptr).k_idx as
                                                     isize)).aware as
                                     libc::c_int != 0) {
                return 0 as libc::c_int as bool_
            }
            /* Require identical charges, since staffs are bulky. */
            if (*o_ptr).pval != (*j_ptr).pval {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine recharged ones with non recharged ones. */
            if f4 as libc::c_long & 0x800000 as libc::c_long !=
                   f14 as libc::c_long & 0x800000 as libc::c_long {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine different spells */
            if (*o_ptr).pval2 as libc::c_int != (*j_ptr).pval2 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine different base levels */
            if (*o_ptr).pval3 != (*j_ptr).pval3 {
                return 0 as libc::c_int as bool_
            }
            /* Beware artifatcs should not combibne with "lesser" thing */
            if (*o_ptr).name1 as libc::c_int != (*j_ptr).name1 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine different ego or normal ones */
            if (*o_ptr).name2 as libc::c_int != (*j_ptr).name2 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine different ego or normal ones */
            if (*o_ptr).name2b as libc::c_int !=
                   (*j_ptr).name2b as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            /* Assume okay */
            current_block_116 = 8225018548522317130;
        }
        65 => {
            /* Wands */
            /* Require either knowledge or known empty for both wands. */
            if (*o_ptr).ident as libc::c_int & 0x4 as libc::c_int == 0 &&
                   !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0
                         ||
                         (*k_info.offset((*o_ptr).k_idx as isize)).easy_know
                             as libc::c_int != 0 &&
                             (*k_info.offset((*o_ptr).k_idx as isize)).aware
                                 as libc::c_int != 0) ||
                   (*j_ptr).ident as libc::c_int & 0x4 as libc::c_int == 0 &&
                       !((*j_ptr).ident as libc::c_int & 0x8 as libc::c_int !=
                             0 ||
                             (*k_info.offset((*j_ptr).k_idx as
                                                 isize)).easy_know as
                                 libc::c_int != 0 &&
                                 (*k_info.offset((*j_ptr).k_idx as
                                                     isize)).aware as
                                     libc::c_int != 0) {
                return 0 as libc::c_int as bool_
            }
            /* Beware artifatcs should not combibne with "lesser" thing */
            if (*o_ptr).name1 as libc::c_int != (*j_ptr).name1 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine recharged ones with non recharged ones. */
            if f4 as libc::c_long & 0x800000 as libc::c_long !=
                   f14 as libc::c_long & 0x800000 as libc::c_long {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine different spells */
            if (*o_ptr).pval2 as libc::c_int != (*j_ptr).pval2 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine different base levels */
            if (*o_ptr).pval3 != (*j_ptr).pval3 {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine different ego or normal ones */
            if (*o_ptr).name2 as libc::c_int != (*j_ptr).name2 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine different ego or normal ones */
            if (*o_ptr).name2b as libc::c_int !=
                   (*j_ptr).name2b as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            /* Assume okay */
            current_block_116 = 8225018548522317130;
        }
        66 => {
            /* Probably okay */
            current_block_116 = 8225018548522317130;
        }
        67 => {
            /* Rods */
            return 0 as libc::c_int as bool_
        }
        19 | 15 | 20 | 21 | 22 | 6 | 23 | 24 | 30 | 31 | 32 | 33 | 34 | 35 |
        36 | 37 | 38 | 46 | 115 => {
            /* Weapons and Armor */
            /* Require permission */
            if stack_allow_items == 0 { return 0 as libc::c_int as bool_ }
            current_block_116 = 15871111637229080974;
            /* Fall through */
        }
        45 | 40 | 39 => { current_block_116 = 15871111637229080974; }
        18 | 17 | 16 => { current_block_116 = 5722677567366458307; }
        80 => {
            /* UHH ugly hack for the mushroom quest, sorry */
            if (*o_ptr).pval2 as libc::c_int != (*j_ptr).pval2 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            current_block_116 = 8225018548522317130;
        }
        4 => {
            /* UHH ugly hack for the fireproof quest, sorry */
            if (*o_ptr).pval2 as libc::c_int != (*j_ptr).pval2 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            current_block_116 = 8225018548522317130;
        }
        _ => {
            /* Various */
            /* Require knowledge */
            if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                     (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                         libc::c_int != 0 &&
                         (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                             libc::c_int != 0) ||
                   !((*j_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0
                         ||
                         (*k_info.offset((*j_ptr).k_idx as isize)).easy_know
                             as libc::c_int != 0 &&
                             (*k_info.offset((*j_ptr).k_idx as isize)).aware
                                 as libc::c_int != 0) {
                return 0 as libc::c_int as bool_
            }
            /* Probably okay */
            current_block_116 = 8225018548522317130;
        }
    }
    match current_block_116 {
        15871111637229080974 =>
        /* Rings, Amulets, Lites */
        /* Require full knowledge of both items */
        {
            if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                     (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                         libc::c_int != 0 &&
                         (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                             libc::c_int != 0) ||
                   !((*j_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0
                         ||
                         (*k_info.offset((*j_ptr).k_idx as isize)).easy_know
                             as libc::c_int != 0 &&
                             (*k_info.offset((*j_ptr).k_idx as isize)).aware
                                 as libc::c_int != 0) {
                return 0 as libc::c_int as bool_
            }
            /* Fall through */
            /* Require identical "turns of light" */
            if (*o_ptr).timeout as libc::c_int !=
                   (*j_ptr).timeout as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            current_block_116 = 5722677567366458307;
        }
        _ => { }
    }
    match current_block_116 {
        5722677567366458307 =>
        /* Missiles */
        /* Require identical knowledge of both items */
        {
            if ((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                    (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                        libc::c_int != 0 &&
                        (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                            libc::c_int != 0) as libc::c_int !=
                   ((*j_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                        (*k_info.offset((*j_ptr).k_idx as isize)).easy_know as
                            libc::c_int != 0 &&
                            (*k_info.offset((*j_ptr).k_idx as isize)).aware as
                                libc::c_int != 0) as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            /* Require identical "bonuses" */
            if (*o_ptr).to_h as libc::c_int != (*j_ptr).to_h as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            if (*o_ptr).to_d as libc::c_int != (*j_ptr).to_d as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            if (*o_ptr).to_a as libc::c_int != (*j_ptr).to_a as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            /* Require identical "pval" code */
            if (*o_ptr).pval != (*j_ptr).pval {
                return 0 as libc::c_int as bool_
            }
            /* Require identical exploding status code */
            if (*o_ptr).pval2 as libc::c_int != (*j_ptr).pval2 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            /* Require identical "artifact" names */
            if (*o_ptr).name1 as libc::c_int != (*j_ptr).name1 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            /* Require identical "ego-item" names */
            if (*o_ptr).name2 as libc::c_int != (*j_ptr).name2 as libc::c_int
               {
                return 0 as libc::c_int as bool_
            }
            /* Do not combine different ego or normal ones */
            if (*o_ptr).name2b as libc::c_int !=
                   (*j_ptr).name2b as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            /* Hack -- Never stack "powerful" items */
			/*
			   Why?!
			-- wilh
			*/
			/* #if 0 */
            if (*o_ptr).xtra1 as libc::c_int != 0 ||
                   (*j_ptr).xtra1 as libc::c_int != 0 {
                return 0 as libc::c_int as bool_
            }
            /* #endif */
            /* Hack -- Never stack recharging items */
            if ((*o_ptr).timeout as libc::c_int != 0 ||
                    (*j_ptr).timeout as libc::c_int != 0) &&
                   (*o_ptr).tval as libc::c_int != 39 as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            /* Require identical "values" */
            if (*o_ptr).ac as libc::c_int != (*j_ptr).ac as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            if (*o_ptr).dd as libc::c_int != (*j_ptr).dd as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            if (*o_ptr).ds as libc::c_int != (*j_ptr).ds as libc::c_int {
                return 0 as libc::c_int as bool_
            }
        }
        _ => { }
    }
    /* Hack -- Identical art_flags! */
    if (*o_ptr).art_flags1 != (*j_ptr).art_flags1 ||
           (*o_ptr).art_flags2 != (*j_ptr).art_flags2 ||
           (*o_ptr).art_flags3 != (*j_ptr).art_flags3 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Require identical "cursed" status */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int !=
           (*j_ptr).ident as libc::c_int & 0x40 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- require semi-matching "inscriptions" */
    if (*o_ptr).note as libc::c_int != 0 && (*j_ptr).note as libc::c_int != 0
           && (*o_ptr).note as libc::c_int != (*j_ptr).note as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- normally require matching "inscriptions" */
    if stack_force_notes == 0 &&
           (*o_ptr).note as libc::c_int != (*j_ptr).note as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- normally require matching "discounts" */
    if stack_force_costs == 0 &&
           (*o_ptr).discount as libc::c_int !=
               (*j_ptr).discount as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Maximal "stacking" limit */
    if total >= 100 as libc::c_int { return 0 as libc::c_int as bool_ }
    /* They match, so they must be similar */
    return 1 as libc::c_int as bool_;
}
/*
 * Allow one item to "absorb" another, assuming they are similar
 */
#[no_mangle]
pub unsafe extern "C" fn object_absorb(mut o_ptr: *mut object_type,
                                       mut j_ptr: *mut object_type) {
    let mut total: libc::c_int =
        (*o_ptr).number as libc::c_int + (*j_ptr).number as libc::c_int;
    /* Add together the item counts */
    (*o_ptr).number =
        if total < 100 as libc::c_int {
            total
        } else { (100 as libc::c_int) - 1 as libc::c_int } as byte_hack;
    /* Hack -- blend "known" status */
    if (*j_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
           (*k_info.offset((*j_ptr).k_idx as isize)).easy_know as libc::c_int
               != 0 &&
               (*k_info.offset((*j_ptr).k_idx as isize)).aware as libc::c_int
                   != 0 {
        object_known(o_ptr);
    }
    /* Hack -- clear "storebought" if only one has it */
    if ((*o_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 ||
            (*j_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0) &&
           !((*o_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 &&
                 (*j_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0) {
        if (*j_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 {
            (*j_ptr).ident =
                ((*j_ptr).ident as libc::c_int & 0xef as libc::c_int) as
                    byte_hack
        }
        if (*o_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 {
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int & 0xef as libc::c_int) as
                    byte_hack
        }
    }
    /* Hack -- blend "mental" status */
    if (*j_ptr).ident as libc::c_int & 0x20 as libc::c_int != 0 {
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x20 as libc::c_int) as byte_hack
    }
    /* Hack -- blend "inscriptions" */
    if (*j_ptr).note != 0 { (*o_ptr).note = (*j_ptr).note }
    /* Hack -- could average discounts XXX XXX XXX */
	/* Hack -- save largest discount XXX XXX XXX */
    if ((*o_ptr).discount as libc::c_int) < (*j_ptr).discount as libc::c_int {
        (*o_ptr).discount = (*j_ptr).discount
    }
    /* Hack -- if wands are stacking, combine the charges. -LM- */
    if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
        (*o_ptr).pval += (*j_ptr).pval
    };
}
/*
 * Find the index of the object_kind with the given tval and sval
 */
#[no_mangle]
pub unsafe extern "C" fn lookup_kind(mut tval: libc::c_int,
                                     mut sval: libc::c_int) -> s16b {
    let mut k: libc::c_int = 0;
    /* Look for it */
    k = 1 as libc::c_int;
    while k < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(k as isize) as *mut object_kind;
        /* Found a match */
        if (*k_ptr).tval as libc::c_int == tval &&
               (*k_ptr).sval as libc::c_int == sval {
            return k as s16b
        }
        k += 1
    }
    /* Oops */
    if wizard != 0 {
        msg_format(b"No object (%d,%d)\x00" as *const u8 as
                       *const libc::c_char, tval, sval);
    }
    /* Oops */
    return 0 as libc::c_int as s16b;
}
/*
 * Wipe an object clean.
 */
#[no_mangle]
pub unsafe extern "C" fn object_wipe(mut o_ptr: *mut object_type) {
    /* Wipe the structure */
    o_ptr =
        memset(o_ptr as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<object_type>() as libc::c_ulong) as
            *mut object_type;
}
/*
 * Prepare an object based on an existing object
 */
#[no_mangle]
pub unsafe extern "C" fn object_copy(mut o_ptr: *mut object_type,
                                     mut j_ptr: *mut object_type) {
    /* Copy the structure */
    memcpy(o_ptr as *mut libc::c_char as *mut libc::c_void,
           j_ptr as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<object_type>() as libc::c_ulong);
}
/*
 * Prepare an object based on an object kind.
 */
#[no_mangle]
pub unsafe extern "C" fn object_prep(mut o_ptr: *mut object_type,
                                     mut k_idx: libc::c_int) {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(k_idx as isize) as *mut object_kind;
    /* Clear the record */
    o_ptr =
        memset(o_ptr as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<object_type>() as libc::c_ulong) as
            *mut object_type;
    /* Save the kind index */
    (*o_ptr).k_idx = k_idx as s16b;
    /* Efficiency -- tval/sval */
    (*o_ptr).tval = (*k_ptr).tval;
    (*o_ptr).sval = (*k_ptr).sval;
    /* Default "pval" */
    (*o_ptr).pval = (*k_ptr).pval;
    (*o_ptr).pval2 = (*k_ptr).pval2 as s16b;
    /* Default number */
    (*o_ptr).number = 1 as libc::c_int as byte_hack;
    /* Default weight */
    (*o_ptr).weight = (*k_ptr).weight;
    /* Default magic */
    (*o_ptr).to_h = (*k_ptr).to_h;
    (*o_ptr).to_d = (*k_ptr).to_d;
    (*o_ptr).to_a = (*k_ptr).to_a;
    /* Default power */
    (*o_ptr).ac = (*k_ptr).ac;
    (*o_ptr).dd = (*k_ptr).dd;
    (*o_ptr).ds = (*k_ptr).ds;
    /* Hack -- cursed items are always "cursed" */
    if (*k_ptr).flags3 as libc::c_long & 0x20000000 as libc::c_long != 0 {
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as byte_hack
    }
    /* Hack give a basic exp/exp level to an object that needs it */
    if (*k_ptr).flags4 as libc::c_long & 0x100 as libc::c_long != 0 {
        (*o_ptr).elevel =
            ((*k_ptr).level as libc::c_int / 10 as libc::c_int +
                 1 as libc::c_int) as byte_hack;
        (*o_ptr).exp =
            player_exp[((*o_ptr).elevel as libc::c_int - 1 as libc::c_int) as
                           usize];
        /* No flags groups */
        (*o_ptr).pval2 = 1 as libc::c_int as s16b; /* Start with one point */
        (*o_ptr).pval3 = 0 as libc::c_int
    };
}
/*
 * Help determine an "enchantment bonus" for an object.
 *
 * To avoid floating point but still provide a smooth distribution of bonuses,
 * we simply round the results of division in such a way as to "average" the
 * correct floating point value.
 *
 * This function has been changed.  It uses "randnor()" to choose values from
 * a normal distribution, whose mean moves from zero towards the max as the
 * level increases, and whose standard deviation is equal to 1/4 of the max,
 * and whose values are forced to lie between zero and the max, inclusive.
 *
 * Since the "level" rarely passes 100 before Morgoth is dead, it is very
 * rare to get the "full" enchantment on an object, even a deep levels.
 *
 * It is always possible (albeit unlikely) to get the "full" enchantment.
 *
 * A sample distribution of values from "m_bonus(10, N)" is shown below:
 *
 *   N       0     1     2     3     4     5     6     7     8     9    10
 * ---    ----  ----  ----  ----  ----  ----  ----  ----  ----  ----  ----
 *   0   66.37 13.01  9.73  5.47  2.89  1.31  0.72  0.26  0.12  0.09  0.03
 *   8   46.85 24.66 12.13  8.13  4.20  2.30  1.05  0.36  0.19  0.08  0.05
 *  16   30.12 27.62 18.52 10.52  6.34  3.52  1.95  0.90  0.31  0.15  0.05
 *  24   22.44 15.62 30.14 12.92  8.55  5.30  2.39  1.63  0.62  0.28  0.11
 *  32   16.23 11.43 23.01 22.31 11.19  7.18  4.46  2.13  1.20  0.45  0.41
 *  40   10.76  8.91 12.80 29.51 16.00  9.69  5.90  3.43  1.47  0.88  0.65
 *  48    7.28  6.81 10.51 18.27 27.57 11.76  7.85  4.99  2.80  1.22  0.94
 *  56    4.41  4.73  8.52 11.96 24.94 19.78 11.06  7.18  3.68  1.96  1.78
 *  64    2.81  3.07  5.65  9.17 13.01 31.57 13.70  9.30  6.04  3.04  2.64
 *  72    1.87  1.99  3.68  7.15 10.56 20.24 25.78 12.17  7.52  4.42  4.62
 *  80    1.02  1.23  2.78  4.75  8.37 12.04 27.61 18.07 10.28  6.52  7.33
 *  88    0.70  0.57  1.56  3.12  6.34 10.06 15.76 30.46 12.58  8.47 10.38
 *  96    0.27  0.60  1.25  2.28  4.30  7.60 10.77 22.52 22.51 11.37 16.53
 * 104    0.22  0.42  0.77  1.36  2.62  5.33  8.93 13.05 29.54 15.23 22.53
 * 112    0.15  0.20  0.56  0.87  2.00  3.83  6.86 10.06 17.89 27.31 30.27
 * 120    0.03  0.11  0.31  0.46  1.31  2.48  4.60  7.78 11.67 25.53 45.72
 * 128    0.02  0.01  0.13  0.33  0.83  1.41  3.24  6.17  9.57 14.22 64.07
 */
#[no_mangle]
pub unsafe extern "C" fn m_bonus(mut max: libc::c_int, mut level: libc::c_int)
 -> s16b {
    let mut bonus: libc::c_int = 0;
    let mut stand: libc::c_int = 0;
    let mut extra: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    /* Paranoia -- enforce maximal "level" */
    if level > 128 as libc::c_int - 1 as libc::c_int {
        level = 128 as libc::c_int - 1 as libc::c_int
    }
    /* The "bonus" moves towards the max */
    bonus = max * level / 128 as libc::c_int;
    /* Hack -- determine fraction of error */
    extra = max * level % 128 as libc::c_int;
    /* Hack -- simulate floating point computations */
    if Rand_div(128 as libc::c_int) < extra { bonus += 1 }
    /* The "stand" is equal to one quarter of the max */
    stand = max / 4 as libc::c_int;
    /* Hack -- determine fraction of error */
    extra = max % 4 as libc::c_int;
    /* Hack -- simulate floating point computations */
    if Rand_div(4 as libc::c_int) < extra { stand += 1 }
    /* Choose an "interesting" value */
    value = randnor(bonus, stand) as libc::c_int;
    /* Enforce the minimum value */
    if value < 0 as libc::c_int { return 0 as libc::c_int as s16b }
    /* Enforce the maximum value */
    if value > max { return max as s16b }
    /* Result */
    return value as s16b;
}
/*
 * Tinker with the random artifact to make it acceptable
 * for a certain depth; also connect a random artifact to an 
 * object.
 */
unsafe extern "C" fn finalize_randart(mut o_ptr: *mut object_type,
                                      mut lev: libc::c_int) {
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut foo: libc::c_int =
        lev + randnor(0 as libc::c_int, 5 as libc::c_int) as libc::c_int;
    let mut flag: bool_ = 1 as libc::c_int as bool_;
    /* Paranoia */
    if (*o_ptr).tval as libc::c_int != 102 as libc::c_int { return }
    if foo < 1 as libc::c_int { foo = 1 as libc::c_int }
    if foo > 100 as libc::c_int { foo = 100 as libc::c_int }
    while flag != 0 {
        r = Rand_div(84 as libc::c_int);
        if random_artifacts[r as usize].generated == 0 ||
               i > 2000 as libc::c_int {
            let mut ra_ptr: *mut random_artifact =
                &mut *random_artifacts.as_mut_ptr().offset(r as isize) as
                    *mut random_artifact;
            (*o_ptr).sval = r as byte_hack;
            (*o_ptr).pval2 = (*ra_ptr).activation as s16b;
            (*o_ptr).xtra2 =
                activation_info[(*ra_ptr).activation as usize].spell;
            (*ra_ptr).level = lev as byte_hack;
            (*ra_ptr).generated = 1 as libc::c_int as byte_hack;
            flag = 0 as libc::c_int as bool_
        }
        i += 1
    };
}
/*
 * Cheat -- describe a created object for the user
 */
unsafe extern "C" fn object_mention(mut o_ptr: *mut object_type) {
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Describe */
    object_desc_store(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                      0 as libc::c_int);
    /* Artifact */
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
        /* Silly message */
        msg_format(b"Artifact (%s)\x00" as *const u8 as *const libc::c_char,
                   o_name.as_mut_ptr());
    } else if (*o_ptr).art_name != 0 {
        msg_print(b"Random artifact\x00" as *const u8 as *const libc::c_char);
    } else if if (*o_ptr).name2 as libc::c_int != 0 ||
                     (*o_ptr).name2b as libc::c_int != 0 {
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
        /* Random Artifact */
        /* Ego-item */
        /* Silly message */
        msg_format(b"Ego-item (%s)\x00" as *const u8 as *const libc::c_char,
                   o_name.as_mut_ptr());
    } else {
        /* Normal item */
        /* Silly message */
        msg_format(b"Object (%s)\x00" as *const u8 as *const libc::c_char,
                   o_name.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn random_artifact_resistance(mut o_ptr:
                                                        *mut object_type) {
    let mut give_resistance: bool_ = 0 as libc::c_int as bool_;
    let mut give_power: bool_ = 0 as libc::c_int as bool_;
    match (*o_ptr).name1 as libc::c_int {
        23 | 24 | 25 | 27 | 21 | 31 | 32 | 36 | 49 | 62 | 30 | 67 | 68 | 66 |
        88 | 122 | 28 => {
            /* Give a resistance */
            give_resistance = 1 as libc::c_int as bool_
        }
        64 | 73 | 75 | 83 | 77 | 76 | 89 | 126 | 110 => {
            /* Give a resistance OR a power */
            if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                give_resistance = 1 as libc::c_int as bool_
            } else { give_power = 1 as libc::c_int as bool_ }
        }
        11 | 12 | 35 | 59 | 46 | 108 | 120 => {
            /* Give a power */
            give_power = 1 as libc::c_int as bool_
        }
        13 | 42 | 117 => {
            /* Give both */
            give_power = 1 as libc::c_int as bool_;
            give_resistance = 1 as libc::c_int as bool_
        }
        _ => { }
    }
    if give_power != 0 {
        (*o_ptr).xtra1 = 3 as libc::c_int as byte_hack;
        /* Randomize the "xtra" power */
        if (*o_ptr).xtra1 != 0 {
            (*o_ptr).xtra2 =
                (Rand_div(256 as libc::c_int) + 1 as libc::c_int) as s16b
        }
    }
    artifact_bias = 0 as libc::c_int;
    if give_resistance != 0 {
        random_resistance(o_ptr, 0 as libc::c_int as bool_,
                          Rand_div(22 as libc::c_int) + 1 as libc::c_int +
                              16 as libc::c_int);
    };
}
/*
 * Mega-Hack -- Attempt to create one of the "Special Objects"
 *
 * We are only called from "make_object()", and we assume that
 * "apply_magic()" is called immediately after we return.
 *
 * Note -- see "make_artifact()" and "apply_magic()"
 */
unsafe extern "C" fn make_artifact_special(mut o_ptr: *mut object_type)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut k_idx: libc::c_int = 0 as libc::c_int;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* No artifacts in the town */
    if dun_level == 0 { return 0 as libc::c_int as bool_ }
    let mut current_block_11: u64;
    /* Check the artifact list (just the "specials") */
    i = 0 as libc::c_int;
    while i < max_a_idx as libc::c_int {
        let mut a_ptr: *mut artifact_type =
            &mut *a_info.offset(i as isize) as *mut artifact_type;
        /* Skip "empty" artifacts */
        if !((*a_ptr).name == 0) {
            /* Cannot make an artifact twice */
            if !((*a_ptr).cur_num != 0) {
                /* Cannot generate non special ones */
                if !((*a_ptr).flags3 as libc::c_long & 0x800 as libc::c_long
                         == 0) {
                    /* Cannot generate some artifacts because they can only exists in special dungeons/quests/... */
                    if !((*a_ptr).flags4 as libc::c_long &
                             0x400 as libc::c_long != 0 &&
                             *a_allow_special.offset(i as isize) == 0) {
                        /* XXX XXX Enforce minimum "depth" (loosely) */
                        if (*a_ptr).level as libc::c_int >
                               dun_level as libc::c_int {
                            /* Acquire the "out-of-depth factor" */
                            let mut d: libc::c_int =
                                ((*a_ptr).level as libc::c_int -
                                     dun_level as libc::c_int) *
                                    2 as libc::c_int;
                            /* Roll for out-of-depth creation */
                            if Rand_div(d) != 0 as libc::c_int {
                                current_block_11 = 17179679302217393232;
                            } else { current_block_11 = 3512920355445576850; }
                        } else { current_block_11 = 3512920355445576850; }
                        match current_block_11 {
                            17179679302217393232 => { }
                            _ =>
                            /* Artifact "rarity roll" */
                            {
                                if !(Rand_div((*a_ptr).rarity as libc::c_int -
                                                  luck(-((*a_ptr).rarity as
                                                             libc::c_int /
                                                             2 as
                                                                 libc::c_int),
                                                       (*a_ptr).rarity as
                                                           libc::c_int /
                                                           2 as libc::c_int))
                                         != 0 as libc::c_int) {
                                    /* Find the base object */
                                    k_idx =
                                        lookup_kind((*a_ptr).tval as
                                                        libc::c_int,
                                                    (*a_ptr).sval as
                                                        libc::c_int) as
                                            libc::c_int;
                                    /* XXX XXX Enforce minimum "object" level (loosely) */
                                    if (*k_info.offset(k_idx as isize)).level
                                           as libc::c_int >
                                           object_level as libc::c_int {
                                        /* Acquire the "out-of-depth factor" */
                                        let mut d_0: libc::c_int =
                                            ((*k_info.offset(k_idx as
                                                                 isize)).level
                                                 as libc::c_int -
                                                 object_level as libc::c_int)
                                                * 5 as libc::c_int;
                                        /* Roll for out-of-depth creation */
                                        if Rand_div(d_0) != 0 as libc::c_int {
                                            current_block_11 =
                                                17179679302217393232;
                                        } else {
                                            current_block_11 =
                                                7175849428784450219;
                                        }
                                    } else {
                                        current_block_11 =
                                            7175849428784450219;
                                    }
                                    match current_block_11 {
                                        17179679302217393232 => { }
                                        _ => {
                                            /* Assign the template */
                                            object_prep(o_ptr, k_idx);
                                            /* Mega-Hack -- mark the item as an artifact */
                                            (*o_ptr).name1 = i as byte_hack;
                                            /* Extract some flags */
                                            object_flags(o_ptr, &mut f1,
                                                         &mut f2, &mut f3,
                                                         &mut f4, &mut f5,
                                                         &mut esp);
                                            /* Hack give a basic exp/exp level to an object that needs it */
                                            if f4 as libc::c_long &
                                                   0x100 as libc::c_long != 0
                                               {
                                                (*o_ptr).elevel =
                                                    ((*k_info.offset(k_idx as
                                                                         isize)).level
                                                         as libc::c_int /
                                                         10 as libc::c_int +
                                                         1 as libc::c_int) as
                                                        byte_hack;
                                                (*o_ptr).exp =
                                                    player_exp[((*o_ptr).elevel
                                                                    as
                                                                    libc::c_int
                                                                    -
                                                                    1 as
                                                                        libc::c_int)
                                                                   as usize]
                                            }
                                            /* Success */
                                            return 1 as libc::c_int as bool_
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Failure */
    return 0 as libc::c_int as bool_;
}
/*
 * Attempt to change an object into an artifact
 *
 * This routine should only be called by "apply_magic()"
 *
 * Note -- see "make_artifact_special()" and "apply_magic()"
 */
unsafe extern "C" fn make_artifact(mut o_ptr: *mut object_type) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    /* No artifacts in the town */
    if dun_level == 0 { return 0 as libc::c_int as bool_ }
    /* Paranoia -- no "plural" artifacts */
    if (*o_ptr).number as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    let mut current_block_12: u64;
    /* Check the artifact list (skip the "specials") */
    i = 0 as libc::c_int;
    while i < max_a_idx as libc::c_int {
        let mut a_ptr: *mut artifact_type =
            &mut *a_info.offset(i as isize) as *mut artifact_type;
        /* Skip "empty" items */
        if !((*a_ptr).name == 0) {
            /* Cannot make an artifact twice */
            if !((*a_ptr).cur_num != 0) {
                /* Cannot generate special ones */
                if !((*a_ptr).flags3 as libc::c_long & 0x800 as libc::c_long
                         != 0) {
                    /* Cannot generate some artifacts because they can only exists in special dungeons/quests/... */
                    if !((*a_ptr).flags4 as libc::c_long &
                             0x400 as libc::c_long != 0 &&
                             *a_allow_special.offset(i as isize) == 0) {
                        /* Must have the correct fields */
                        if !((*a_ptr).tval as libc::c_int !=
                                 (*o_ptr).tval as libc::c_int) {
                            if !((*a_ptr).sval as libc::c_int !=
                                     (*o_ptr).sval as libc::c_int) {
                                /* XXX XXX Enforce minimum "depth" (loosely) */
                                if (*a_ptr).level as libc::c_int >
                                       dun_level as libc::c_int {
                                    /* Acquire the "out-of-depth factor" */
                                    let mut d: libc::c_int =
                                        ((*a_ptr).level as libc::c_int -
                                             dun_level as libc::c_int) *
                                            2 as libc::c_int;
                                    /* Roll for out-of-depth creation */
                                    if Rand_div(d) != 0 as libc::c_int {
                                        current_block_12 =
                                            6873731126896040597;
                                    } else {
                                        current_block_12 =
                                            6009453772311597924;
                                    }
                                } else {
                                    current_block_12 = 6009453772311597924;
                                }
                                match current_block_12 {
                                    6873731126896040597 => { }
                                    _ =>
                                    /* We must make the "rarity roll" */
                                    {
                                        if !(Rand_div((*a_ptr).rarity as
                                                          libc::c_int -
                                                          luck(-((*a_ptr).rarity
                                                                     as
                                                                     libc::c_int
                                                                     /
                                                                     2 as
                                                                         libc::c_int),
                                                               (*a_ptr).rarity
                                                                   as
                                                                   libc::c_int
                                                                   /
                                                                   2 as
                                                                       libc::c_int))
                                                 != 0 as libc::c_int) {
                                            /* Hack -- mark the item as an artifact */
                                            (*o_ptr).name1 = i as byte_hack;
                                            /* Hack: Some artifacts get random extra powers */
                                            random_artifact_resistance(o_ptr);
                                            /* Extract some flags */
                                            object_flags(o_ptr, &mut f1,
                                                         &mut f2, &mut f3,
                                                         &mut f4, &mut f5,
                                                         &mut esp);
                                            /* Hack give a basic exp/exp level to an object that needs it */
                                            if f4 as libc::c_long &
                                                   0x100 as libc::c_long != 0
                                               {
                                                (*o_ptr).elevel =
                                                    ((*k_ptr).level as
                                                         libc::c_int /
                                                         10 as libc::c_int +
                                                         1 as libc::c_int) as
                                                        byte_hack;
                                                (*o_ptr).exp =
                                                    player_exp[((*o_ptr).elevel
                                                                    as
                                                                    libc::c_int
                                                                    -
                                                                    1 as
                                                                        libc::c_int)
                                                                   as usize]
                                            }
                                            /* Success */
                                            return 1 as libc::c_int as bool_
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Failure */
    return 0 as libc::c_int as bool_;
}
/*
 * Attempt to change an object into an ego
 *
 * This routine should only be called by "apply_magic()"
 */
unsafe extern "C" fn make_ego_item(mut o_ptr: *mut object_type,
                                   mut good: bool_) -> bool_ {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut ok_ego: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ok_num: libc::c_int = 0 as libc::c_int;
    let mut ret: bool_ = 0 as libc::c_int as bool_;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
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
           (*o_ptr).name2 as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    ok_ego =
        memset(ralloc((max_e_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_e_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    /* Grab the ok ego */
    i = 0 as libc::c_int;
    while i < max_e_idx as libc::c_int {
        let mut e_ptr: *mut ego_item_type =
            &mut *e_info.offset(i as isize) as *mut ego_item_type;
        let mut ok: bool_ = 0 as libc::c_int as bool_;
        /* Skip "empty" items */
        if !((*e_ptr).name == 0) {
            /* Must have the correct fields */
            j = 0 as libc::c_int;
            while j < 6 as libc::c_int {
                if (*e_ptr).tval[j as usize] as libc::c_int ==
                       (*o_ptr).tval as libc::c_int {
                    if (*e_ptr).min_sval[j as usize] as libc::c_int <=
                           (*o_ptr).sval as libc::c_int &&
                           (*e_ptr).max_sval[j as usize] as libc::c_int >=
                               (*o_ptr).sval as libc::c_int {
                        ok = 1 as libc::c_int as bool_
                    }
                }
                if ok != 0 { break ; }
                j += 1
            }
            if !(ok == 0) {
                /* Good should be good, bad should be bad */
                if !(good as libc::c_int != 0 && (*e_ptr).cost == 0) {
                    if !(good == 0 && (*e_ptr).cost != 0) {
                        /* Must posses the good flags */
                        if !((*k_ptr).flags1 & (*e_ptr).need_flags1 !=
                                 (*e_ptr).need_flags1 ||
                                 (*k_ptr).flags2 & (*e_ptr).need_flags2 !=
                                     (*e_ptr).need_flags2 ||
                                 (*k_ptr).flags3 & (*e_ptr).need_flags3 !=
                                     (*e_ptr).need_flags3 ||
                                 (*k_ptr).flags4 & (*e_ptr).need_flags4 !=
                                     (*e_ptr).need_flags4 ||
                                 (*k_ptr).flags5 & (*e_ptr).need_flags5 !=
                                     (*e_ptr).need_flags5 ||
                                 (*k_ptr).esp & (*e_ptr).need_esp !=
                                     (*e_ptr).need_esp) {
                            if !((*k_ptr).flags1 & (*e_ptr).forbid_flags1 != 0
                                     ||
                                     (*k_ptr).flags2 & (*e_ptr).forbid_flags2
                                         != 0 ||
                                     (*k_ptr).flags3 & (*e_ptr).forbid_flags3
                                         != 0 ||
                                     (*k_ptr).flags4 & (*e_ptr).forbid_flags4
                                         != 0 ||
                                     (*k_ptr).flags5 & (*e_ptr).forbid_flags5
                                         != 0 ||
                                     (*k_ptr).esp & (*e_ptr).forbid_esp != 0)
                               {
                                /* ok */
                                let fresh0 = ok_num;
                                ok_num = ok_num + 1;
                                *ok_ego.offset(fresh0 as isize) = i
                            }
                        }
                    }
                }
            }
        }
        /* Doesnt count as a try*/
        i += 1
    }
    let mut current_block_15: u64;
    /* Now test them a few times */
    i = 0 as libc::c_int;
    while i < ok_num * 10 as libc::c_int {
        let mut e_ptr_0: *mut ego_item_type = 0 as *mut ego_item_type;
        let mut j_0: libc::c_int = *ok_ego.offset(Rand_div(ok_num) as isize);
        e_ptr_0 = &mut *e_info.offset(j_0 as isize) as *mut ego_item_type;
        /* XXX XXX Enforce minimum "depth" (loosely) */
        if (*e_ptr_0).level as libc::c_int > dun_level as libc::c_int {
            /* Acquire the "out-of-depth factor" */
            let mut d: libc::c_int =
                (*e_ptr_0).level as libc::c_int - dun_level as libc::c_int;
            /* Roll for out-of-depth creation */
            if Rand_div(d) != 0 as libc::c_int {
                current_block_15 = 4068382217303356765;
            } else { current_block_15 = 8704759739624374314; }
        } else { current_block_15 = 8704759739624374314; }
        match current_block_15 {
            8704759739624374314 =>
            /* We must make the "rarity roll" */
            {
                if !(Rand_div((*e_ptr_0).mrarity as libc::c_int -
                                  luck(-((*e_ptr_0).mrarity as libc::c_int /
                                             2 as libc::c_int),
                                       (*e_ptr_0).mrarity as libc::c_int /
                                           2 as libc::c_int)) >
                         (*e_ptr_0).rarity as libc::c_int) {
                    /* Hack -- mark the item as an ego */
                    (*o_ptr).name2 = j_0 as s16b;
                    /* Success */
                    ret = 1 as libc::c_int as bool_;
                    break ;
                }
            }
            _ => { }
        }
        i += 1
    }
    /*
	 * Sometimes(rarely) tries for a double ego
	        * Also make sure we dont already have a name2b, wchih would mean a special ego item
	 */
    if Rand_div(100 as libc::c_int) <
           7 as libc::c_int + luck(-(7 as libc::c_int), 7 as libc::c_int) &&
           (*o_ptr).name2b == 0 {
        let mut current_block_20: u64;
        /* Now test them a few times */
        i = 0 as libc::c_int;
        while i < ok_num * 10 as libc::c_int {
            let mut e_ptr_1: *mut ego_item_type = 0 as *mut ego_item_type;
            let mut j_1: libc::c_int =
                *ok_ego.offset(Rand_div(ok_num) as isize);
            e_ptr_1 = &mut *e_info.offset(j_1 as isize) as *mut ego_item_type;
            /* Cannot be a double ego of the same ego type */
            if !(j_1 == (*o_ptr).name2 as libc::c_int) {
                /* Cannot have 2 suffixes or 2 prefixes */
                if !((*e_info.offset((*o_ptr).name2 as isize)).before as
                         libc::c_int != 0 &&
                         (*e_ptr_1).before as libc::c_int != 0) {
                    if !((*e_info.offset((*o_ptr).name2 as isize)).before == 0
                             && (*e_ptr_1).before == 0) {
                        /* XXX XXX Enforce minimum "depth" (loosely) */
                        if (*e_ptr_1).level as libc::c_int >
                               dun_level as libc::c_int {
                            /* Acquire the "out-of-depth factor" */
                            let mut d_0: libc::c_int =
                                (*e_ptr_1).level as libc::c_int -
                                    dun_level as libc::c_int;
                            /* Roll for out-of-depth creation */
                            if Rand_div(d_0) != 0 as libc::c_int {
                                current_block_20 = 11743904203796629665;
                            } else {
                                current_block_20 = 14945149239039849694;
                            }
                        } else { current_block_20 = 14945149239039849694; }
                        match current_block_20 {
                            11743904203796629665 => { }
                            _ =>
                            /* We must make the "rarity roll" */
                            {
                                if !(Rand_div((*e_ptr_1).mrarity as
                                                  libc::c_int -
                                                  luck(-((*e_ptr_1).mrarity as
                                                             libc::c_int /
                                                             2 as
                                                                 libc::c_int),
                                                       (*e_ptr_1).mrarity as
                                                           libc::c_int /
                                                           2 as libc::c_int))
                                         > (*e_ptr_1).rarity as libc::c_int) {
                                    /* Hack -- mark the item as an ego */
                                    (*o_ptr).name2b = j_1 as s16b;
                                    /* Success */
                                    ret = 1 as libc::c_int as bool_;
                                    break ;
                                }
                            }
                        }
                    }
                }
            }
            i += 1
        }
    }
    rnfree(ok_ego as vptr,
           (max_e_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    /* Return */
    return ret;
}
/*
 * Charge a new stick.
 */
#[no_mangle]
pub unsafe extern "C" fn charge_stick(mut o_ptr: *mut object_type) {
    (*o_ptr).pval =
        exec_lua(format(b"return get_stick_charges(%d)\x00" as *const u8 as
                            *const libc::c_char,
                        (*o_ptr).pval2 as libc::c_int));
}
/*
 * Apply magic to an item known to be a "weapon"
 *
 * Hack -- note special base damage dice boosting
 * Hack -- note special processing for weapon/digger
 * Hack -- note special rating boost for dragon scale mail
 */
unsafe extern "C" fn a_m_aux_1(mut o_ptr: *mut object_type,
                               mut level: libc::c_int,
                               mut power: libc::c_int) {
    let mut tohit1: libc::c_int =
        Rand_div(5 as libc::c_int) + 1 as libc::c_int +
            m_bonus(5 as libc::c_int, level) as libc::c_int;
    let mut todam1: libc::c_int =
        Rand_div(5 as libc::c_int) + 1 as libc::c_int +
            m_bonus(5 as libc::c_int, level) as libc::c_int;
    let mut tohit2: libc::c_int =
        m_bonus(10 as libc::c_int, level) as libc::c_int;
    let mut todam2: libc::c_int =
        m_bonus(10 as libc::c_int, level) as libc::c_int;
    artifact_bias = 0 as libc::c_int;
    /* Very good */
    if power > 1 as libc::c_int {
        /* Make ego item */
        if Rand_div(RANDART_WEAPON) == 1 as libc::c_int &&
               (*o_ptr).tval as libc::c_int != 46 as libc::c_int {
            create_artifact(o_ptr, 0 as libc::c_int as bool_,
                            1 as libc::c_int as bool_);
        } else { make_ego_item(o_ptr, 1 as libc::c_int as bool_); }
    } else if power < -(1 as libc::c_int) {
        /* Make ego item */
        make_ego_item(o_ptr, 0 as libc::c_int as bool_);
    }
    /* Good */
    if power > 0 as libc::c_int {
        /* Enchant */
        (*o_ptr).to_h = ((*o_ptr).to_h as libc::c_int + tohit1) as s16b;
        (*o_ptr).to_d = ((*o_ptr).to_d as libc::c_int + todam1) as s16b;
        /* Very good */
        if power > 1 as libc::c_int {
            /* Enchant again */
            (*o_ptr).to_h = ((*o_ptr).to_h as libc::c_int + tohit2) as s16b;
            (*o_ptr).to_d = ((*o_ptr).to_d as libc::c_int + todam2) as s16b
        }
    } else if power < 0 as libc::c_int {
        /* Cursed */
        /* Penalize */
        (*o_ptr).to_h = ((*o_ptr).to_h as libc::c_int - tohit1) as s16b;
        (*o_ptr).to_d = ((*o_ptr).to_d as libc::c_int - todam1) as s16b;
        /* Very cursed */
        if power < -(1 as libc::c_int) {
            /* Penalize again */
            (*o_ptr).to_h = ((*o_ptr).to_h as libc::c_int - tohit2) as s16b;
            (*o_ptr).to_d = ((*o_ptr).to_d as libc::c_int - todam2) as s16b
        }
        /* Cursed (if "bad") */
        if ((*o_ptr).to_h as libc::c_int + (*o_ptr).to_d as libc::c_int) <
               0 as libc::c_int {
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                    byte_hack
        }
    }
    /* Some special cases */
    if process_hooks(66 as libc::c_int,
                     b"(O,d,d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, o_ptr, level, power) != 0 {
        return
    }
    match (*o_ptr).tval as libc::c_int {
        46 => {
            /* Good */
            if power > 0 as libc::c_int {
                (*o_ptr).to_a =
                    ((*o_ptr).to_a as libc::c_int +
                         (Rand_div(5 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            /* Very good */
            if power > 1 as libc::c_int {
                (*o_ptr).to_a =
                    ((*o_ptr).to_a as libc::c_int +
                         (Rand_div(5 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            /* Bad */
            if power < 0 as libc::c_int {
                (*o_ptr).to_a =
                    ((*o_ptr).to_a as libc::c_int -
                         (Rand_div(5 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
            /* Very bad */
            if power < -(1 as libc::c_int) {
                (*o_ptr).to_a =
                    ((*o_ptr).to_a as libc::c_int -
                         (Rand_div(5 as libc::c_int) + 1 as libc::c_int)) as
                        s16b
            }
        }
        6 => {
            if (*o_ptr).name2 as libc::c_int == 4 as libc::c_int ||
                   (*o_ptr).name2b as libc::c_int == 4 as libc::c_int {
                let mut gf: [libc::c_int; 2] = [0; 2];
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < 2 as libc::c_int {
                    let mut k: libc::c_int = 0 as libc::c_int;
                    gf[i as usize] = 0 as libc::c_int;
                    while k == 0 {
                        gf[i as usize] = Rand_div(111 as libc::c_int);
                        k =
                            lookup_kind(104 as libc::c_int, gf[i as usize]) as
                                libc::c_int
                    }
                    i += 1
                }
                (*o_ptr).pval =
                    gf[0 as libc::c_int as usize] +
                        (gf[1 as libc::c_int as usize] << 16 as libc::c_int);
                (*o_ptr).pval3 =
                    Rand_div(6 as libc::c_int) +
                        (Rand_div(6 as libc::c_int) << 16 as libc::c_int);
                (*o_ptr).pval2 =
                    (Rand_div(70 as libc::c_int) + 1 as libc::c_int +
                         ((Rand_div(70 as libc::c_int) + 1 as libc::c_int) <<
                              8 as libc::c_int)) as s16b
            } else {
                (*o_ptr).art_flags5 =
                    ((*o_ptr).art_flags5 as libc::c_long |
                         (0x800 as libc::c_long | 0x10000 as libc::c_long)) as
                        u32b
            }
        }
        18 | 17 | 16 => {
            if power == 1 as libc::c_int && (*o_ptr).name2 == 0 {
                if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                       30 as libc::c_int {
                    /* Exploding missile */
                    let mut power_0: [libc::c_int; 27] =
                        [1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
                         4 as libc::c_int, 5 as libc::c_int,
                         12 as libc::c_int, 15 as libc::c_int,
                         16 as libc::c_int, 20 as libc::c_int,
                         21 as libc::c_int, 22 as libc::c_int,
                         23 as libc::c_int, 24 as libc::c_int,
                         26 as libc::c_int, 27 as libc::c_int,
                         28 as libc::c_int, 30 as libc::c_int,
                         31 as libc::c_int, 33 as libc::c_int,
                         34 as libc::c_int, 35 as libc::c_int,
                         40 as libc::c_int, 63 as libc::c_int,
                         66 as libc::c_int, 73 as libc::c_int,
                         78 as libc::c_int, 81 as libc::c_int];
                    (*o_ptr).pval2 =
                        power_0[Rand_div(27 as libc::c_int) as usize] as s16b
                }
            }
        }
        _ => { }
    };
}
unsafe extern "C" fn dragon_resist(mut o_ptr: *mut object_type) {
    loop  {
        artifact_bias = 0 as libc::c_int;
        if Rand_div(4 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
            random_resistance(o_ptr, 0 as libc::c_int as bool_,
                              Rand_div(14 as libc::c_int) + 1 as libc::c_int +
                                  4 as libc::c_int);
        } else {
            random_resistance(o_ptr, 0 as libc::c_int as bool_,
                              Rand_div(22 as libc::c_int) + 1 as libc::c_int +
                                  16 as libc::c_int);
        }
        if !(Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                 1 as libc::c_int) {
            break ;
        }
    };
}
/*
 * Apply magic to an item known to be "armor"
 *
 * Hack -- note special processing for crown/helm
 * Hack -- note special processing for robe of permanence
 */
unsafe extern "C" fn a_m_aux_2(mut o_ptr: *mut object_type,
                               mut level: libc::c_int,
                               mut power: libc::c_int) {
    let mut toac1: libc::c_int =
        Rand_div(5 as libc::c_int) + 1 as libc::c_int +
            m_bonus(5 as libc::c_int, level) as libc::c_int;
    let mut toac2: libc::c_int =
        m_bonus(10 as libc::c_int, level) as libc::c_int;
    artifact_bias = 0 as libc::c_int;
    /* Very good */
    if power > 1 as libc::c_int {
        /* Make ego item */
        if Rand_div(RANDART_ARMOR) == 1 as libc::c_int {
            create_artifact(o_ptr, 0 as libc::c_int as bool_,
                            1 as libc::c_int as bool_);
        } else { make_ego_item(o_ptr, 1 as libc::c_int as bool_); }
    } else if power < -(1 as libc::c_int) {
        /* Make ego item */
        make_ego_item(o_ptr, 0 as libc::c_int as bool_);
    }
    /* Good */
    if power > 0 as libc::c_int {
        /* Enchant */
        (*o_ptr).to_a = ((*o_ptr).to_a as libc::c_int + toac1) as s16b;
        /* Very good */
        if power > 1 as libc::c_int {
            /* Enchant again */
            (*o_ptr).to_a = ((*o_ptr).to_a as libc::c_int + toac2) as s16b
        }
    } else if power < 0 as libc::c_int {
        /* Cursed */
        /* Penalize */
        (*o_ptr).to_a = ((*o_ptr).to_a as libc::c_int - toac1) as s16b;
        /* Very cursed */
        if power < -(1 as libc::c_int) {
            /* Penalize again */
            (*o_ptr).to_a = ((*o_ptr).to_a as libc::c_int - toac2) as s16b
        }
        /* Cursed (if "bad") */
        if ((*o_ptr).to_a as libc::c_int) < 0 as libc::c_int {
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                    byte_hack
        }
    }
    /* Analyze type */
    if process_hooks(66 as libc::c_int,
                     b"(O,d,d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, o_ptr, level, power) != 0 {
        return
    } /* No cursed elven cloaks...? */
    match (*o_ptr).tval as libc::c_int {
        35 => {
            if (*o_ptr).sval as libc::c_int == 2 as libc::c_int {
                (*o_ptr).pval = Rand_div(4 as libc::c_int) + 1 as libc::c_int
            } else if (*o_ptr).sval as libc::c_int == 100 as libc::c_int {
                let mut mimic: s32b = 0;
                call_lua(b"find_random_mimic_shape\x00" as *const u8 as
                             *const libc::c_char,
                         b"(d,d)\x00" as *const u8 as *const libc::c_char,
                         b"d\x00" as *const u8 as *const libc::c_char, level,
                         1 as libc::c_int, &mut mimic as *mut s32b);
                (*o_ptr).pval2 = mimic as s16b
            }
        }
        38 => {
            /* Rating boost */
            rating = (rating as libc::c_int + 30 as libc::c_int) as s16b;
            /* Mention the item */
            if cheat_peek as libc::c_int != 0 ||
                   (*p_ptr).precognition as libc::c_int != 0 {
                object_mention(o_ptr);
            }
        }
        34 => {
            if (*o_ptr).sval as libc::c_int == 6 as libc::c_int {
                /* Rating boost */
                rating = (rating as libc::c_int + 5 as libc::c_int) as s16b;
                /* Mention the item */
                if cheat_peek as libc::c_int != 0 ||
                       (*p_ptr).precognition as libc::c_int != 0 {
                    object_mention(o_ptr);
                }
                dragon_resist(o_ptr);
            }
        }
        32 => {
            if (*o_ptr).sval as libc::c_int == 7 as libc::c_int {
                /* Rating boost */
                rating = (rating as libc::c_int + 5 as libc::c_int) as s16b;
                /* Mention the item */
                if cheat_peek as libc::c_int != 0 ||
                       (*p_ptr).precognition as libc::c_int != 0 {
                    object_mention(o_ptr);
                }
                dragon_resist(o_ptr);
            }
        }
        _ => { }
    };
}
/*
 * Apply magic to an item known to be a "ring" or "amulet"
 *
 * Hack -- note special rating boost for ring of speed
 * Hack -- note special rating boost for amulet of the magi
 * Hack -- note special "pval boost" code for ring of speed
 * Hack -- note that some items must be cursed (or blessed)
 */
unsafe extern "C" fn a_m_aux_3(mut o_ptr: *mut object_type,
                               mut level: libc::c_int,
                               mut power: libc::c_int) {
    artifact_bias = 0 as libc::c_int;
    /* Very good */
    if power > 1 as libc::c_int {
        /* Make ego item */
        if Rand_div(RANDART_JEWEL) == 1 as libc::c_int {
            create_artifact(o_ptr, 0 as libc::c_int as bool_,
                            1 as libc::c_int as bool_);
        } else { make_ego_item(o_ptr, 1 as libc::c_int as bool_); }
    } else if power < -(1 as libc::c_int) {
        /* Make ego item */
        make_ego_item(o_ptr, 0 as libc::c_int as bool_);
    }
    /* Apply magic (good or bad) according to type */
    if process_hooks(66 as libc::c_int,
                     b"(O,d,d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, o_ptr, level, power) != 0 {
        return
    }
    match (*o_ptr).tval as libc::c_int {
        45 => {
            /* Analyze */
            match (*o_ptr).sval as libc::c_int {
                49 => {
                    /* Strength, Constitution, Dexterity, Intelligence */
                    /* Stat bonus */
                    (*o_ptr).pval = m_bonus(3 as libc::c_int, level) as s32b;
                    if (*o_ptr).pval < 1 as libc::c_int {
                        (*o_ptr).pval = 1 as libc::c_int
                    }
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse pval */
                        (*o_ptr).pval = 0 as libc::c_int - (*o_ptr).pval
                    }
                }
                59 => {
                    /* Critical hits */
                    /* Stat bonus */
                    (*o_ptr).pval = m_bonus(10 as libc::c_int, level) as s32b;
                    if (*o_ptr).pval < 1 as libc::c_int {
                        (*o_ptr).pval = 1 as libc::c_int
                    }
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse pval */
                        (*o_ptr).pval = 0 as libc::c_int - (*o_ptr).pval
                    }
                }
                24 | 27 | 26 | 25 => {
                    /* Stat bonus */
                    (*o_ptr).pval =
                        1 as libc::c_int +
                            m_bonus(5 as libc::c_int, level) as libc::c_int;
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse pval */
                        (*o_ptr).pval = 0 as libc::c_int - (*o_ptr).pval
                    }
                }
                31 => {
                    /* Ring of Speed! */
                    /* Base speed (1 to 10) */
                    (*o_ptr).pval =
                        Rand_div(5 as libc::c_int) + 1 as libc::c_int +
                            m_bonus(5 as libc::c_int, level) as libc::c_int;
                    /* Super-charge the ring */
                    while Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                        (*o_ptr).pval += 1
                    }
                    /* Cursed Ring */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse pval */
                        (*o_ptr).pval = 0 as libc::c_int - (*o_ptr).pval
                    } else {
                        /* Rating boost */
                        rating =
                            (rating as libc::c_int + 25 as libc::c_int) as
                                s16b;
                        /* Mention the item */
                        if cheat_peek as libc::c_int != 0 ||
                               (*p_ptr).precognition as libc::c_int != 0 {
                            object_mention(o_ptr);
                        }
                    }
                }
                48 => {
                    loop  {
                        random_resistance(o_ptr, 0 as libc::c_int as bool_,
                                          Rand_div(20 as libc::c_int) +
                                              1 as libc::c_int +
                                              18 as libc::c_int);
                        if !(Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                                 1 as libc::c_int) {
                            break ;
                        }
                    }
                    /* Bonus to armor class */
                    (*o_ptr).to_a =
                        (10 as libc::c_int +
                             (Rand_div(5 as libc::c_int) + 1 as libc::c_int) +
                             m_bonus(10 as libc::c_int, level) as libc::c_int)
                            as s16b;
                    rating =
                        (rating as libc::c_int + 5 as libc::c_int) as s16b
                }
                23 => {
                    /* Searching */
                    /* Bonus to searching */
                    (*o_ptr).pval =
                        1 as libc::c_int +
                            m_bonus(5 as libc::c_int, level) as libc::c_int;
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse pval */
                        (*o_ptr).pval = 0 as libc::c_int - (*o_ptr).pval
                    }
                }
                18 | 17 | 19 => {
                    /* Flames, Acid, Ice */
                    /* Bonus to armor class */
                    (*o_ptr).to_a =
                        (5 as libc::c_int +
                             (Rand_div(5 as libc::c_int) + 1 as libc::c_int) +
                             m_bonus(10 as libc::c_int, level) as libc::c_int)
                            as s16b
                }
                2 | 3 => {
                    /* Weakness, Stupidity */
                    /* Cursed */
                    (*o_ptr).ident =
                        ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int)
                            as byte_hack;
                    /* Penalize */
                    (*o_ptr).pval =
                        0 as libc::c_int -
                            (1 as libc::c_int +
                                 m_bonus(5 as libc::c_int, level) as
                                     libc::c_int)
                }
                0 => {
                    /* WOE, Stupidity */
                    /* Cursed */
                    (*o_ptr).ident =
                        ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int)
                            as byte_hack;
                    /* Penalize */
                    (*o_ptr).to_a =
                        (0 as libc::c_int -
                             (5 as libc::c_int +
                                  m_bonus(10 as libc::c_int, level) as
                                      libc::c_int)) as s16b;
                    (*o_ptr).pval =
                        0 as libc::c_int -
                            (1 as libc::c_int +
                                 m_bonus(5 as libc::c_int, level) as
                                     libc::c_int)
                }
                29 => {
                    /* Ring of damage */
                    /* Bonus to damage */
                    (*o_ptr).to_d =
                        (5 as libc::c_int +
                             (Rand_div(8 as libc::c_int) + 1 as libc::c_int) +
                             m_bonus(10 as libc::c_int, level) as libc::c_int)
                            as s16b;
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse bonus */
                        (*o_ptr).to_d =
                            (0 as libc::c_int - (*o_ptr).to_d as libc::c_int)
                                as s16b
                    }
                }
                28 => {
                    /* Ring of Accuracy */
                    /* Bonus to hit */
                    (*o_ptr).to_h =
                        (5 as libc::c_int +
                             (Rand_div(8 as libc::c_int) + 1 as libc::c_int) +
                             m_bonus(10 as libc::c_int, level) as libc::c_int)
                            as s16b;
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse tohit */
                        (*o_ptr).to_h =
                            (0 as libc::c_int - (*o_ptr).to_h as libc::c_int)
                                as s16b
                    }
                }
                16 => {
                    /* Ring of Protection */
                    /* Bonus to armor class */
                    (*o_ptr).to_a =
                        (5 as libc::c_int +
                             (Rand_div(8 as libc::c_int) + 1 as libc::c_int) +
                             m_bonus(10 as libc::c_int, level) as libc::c_int)
                            as s16b;
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse toac */
                        (*o_ptr).to_a =
                            (0 as libc::c_int - (*o_ptr).to_a as libc::c_int)
                                as s16b
                    }
                }
                30 => {
                    /* Ring of Slaying */
                    /* Bonus to damage and to hit */
                    (*o_ptr).to_d =
                        (Rand_div(7 as libc::c_int) + 1 as libc::c_int +
                             m_bonus(10 as libc::c_int, level) as libc::c_int)
                            as s16b;
                    (*o_ptr).to_h =
                        (Rand_div(7 as libc::c_int) + 1 as libc::c_int +
                             m_bonus(10 as libc::c_int, level) as libc::c_int)
                            as s16b;
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse bonuses */
                        (*o_ptr).to_h =
                            (0 as libc::c_int - (*o_ptr).to_h as libc::c_int)
                                as s16b;
                        (*o_ptr).to_d =
                            (0 as libc::c_int - (*o_ptr).to_d as libc::c_int)
                                as s16b
                    }
                }
                _ => { }
            }
        }
        40 => {
            /* Analyze */
            match (*o_ptr).sval as libc::c_int {
                23 | 25 => {
                    /* Amulet of Trickery */
                    (*o_ptr).pval =
                        1 as libc::c_int +
                            m_bonus(3 as libc::c_int, level) as libc::c_int;
                    /* Mention the item */
                    if cheat_peek as libc::c_int != 0 ||
                           (*p_ptr).precognition as libc::c_int != 0 {
                        object_mention(o_ptr);
                    }
                }
                24 => {
                    (*o_ptr).pval =
                        1 as libc::c_int +
                            m_bonus(2 as libc::c_int, level) as libc::c_int;
                    (*o_ptr).to_a =
                        (1 as libc::c_int +
                             m_bonus(4 as libc::c_int, level) as libc::c_int)
                            as s16b;
                    (*o_ptr).to_h =
                        (1 as libc::c_int +
                             m_bonus(5 as libc::c_int, level) as libc::c_int)
                            as s16b;
                    (*o_ptr).to_d =
                        (1 as libc::c_int +
                             m_bonus(5 as libc::c_int, level) as libc::c_int)
                            as s16b;
                    /* Mention the item */
                    if cheat_peek as libc::c_int != 0 ||
                           (*p_ptr).precognition as libc::c_int != 0 {
                        object_mention(o_ptr);
                    }
                }
                6 | 7 | 28 | 26 => {
                    /* Amulet of wisdom/charisma */
                    (*o_ptr).pval =
                        1 as libc::c_int +
                            m_bonus(5 as libc::c_int, level) as libc::c_int;
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse bonuses */
                        (*o_ptr).pval = 0 as libc::c_int - (*o_ptr).pval
                    }
                }
                17 => {
                    /* Amulet of the Serpents */
                    (*o_ptr).pval =
                        1 as libc::c_int +
                            m_bonus(5 as libc::c_int, level) as libc::c_int;
                    (*o_ptr).to_a =
                        (1 as libc::c_int +
                             m_bonus(6 as libc::c_int, level) as libc::c_int)
                            as s16b;
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse bonuses */
                        (*o_ptr).pval = 0 as libc::c_int - (*o_ptr).pval
                    }
                }
                13 | 14 => {
                    if power < 0 as libc::c_int {
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack
                    }
                }
                15 => {
                    if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        random_resistance(o_ptr, 0 as libc::c_int as bool_,
                                          Rand_div(34 as libc::c_int) +
                                              1 as libc::c_int +
                                              4 as libc::c_int);
                    }
                    if Rand_div(5 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        (*o_ptr).art_flags2 =
                            ((*o_ptr).art_flags2 as libc::c_long |
                                 0x100000 as libc::c_long) as u32b
                    }
                }
                5 => {
                    /* Amulet of searching */
                    (*o_ptr).pval =
                        Rand_div(5 as libc::c_int) + 1 as libc::c_int +
                            m_bonus(5 as libc::c_int, level) as libc::c_int;
                    /* Cursed */
                    if power < 0 as libc::c_int {
                        /* Cursed */
                        (*o_ptr).ident =
                            ((*o_ptr).ident as libc::c_int |
                                 0x40 as libc::c_int) as byte_hack;
                        /* Reverse bonuses */
                        (*o_ptr).pval = 0 as libc::c_int - (*o_ptr).pval
                    }
                }
                8 => {
                    /* Amulet of the Magi -- never cursed */
                    (*o_ptr).pval =
                        1 as libc::c_int +
                            m_bonus(3 as libc::c_int, level) as libc::c_int;
                    if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        (*o_ptr).art_flags3 =
                            ((*o_ptr).art_flags3 as libc::c_long |
                                 0x10000 as libc::c_long) as u32b
                    }
                    /* Boost the rating */
                    rating =
                        (rating as libc::c_int + 25 as libc::c_int) as s16b;
                    /* Mention the item */
                    if cheat_peek as libc::c_int != 0 ||
                           (*p_ptr).precognition as libc::c_int != 0 {
                        object_mention(o_ptr);
                    }
                }
                0 => {
                    /* Amulet of Doom -- always cursed */
                    /* Cursed */
                    (*o_ptr).ident =
                        ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int)
                            as byte_hack;
                    /* Penalize */
                    (*o_ptr).pval =
                        0 as libc::c_int -
                            (Rand_div(5 as libc::c_int) + 1 as libc::c_int +
                                 m_bonus(5 as libc::c_int, level) as
                                     libc::c_int);
                    (*o_ptr).to_a =
                        (0 as libc::c_int -
                             (Rand_div(5 as libc::c_int) + 1 as libc::c_int +
                                  m_bonus(5 as libc::c_int, level) as
                                      libc::c_int)) as s16b
                }
                _ => { }
            }
        }
        _ => { }
    };
}
/*
 * Apply magic to an item known to be "boring"
 *
 * Hack -- note the special code for various items
 */
unsafe extern "C" fn a_m_aux_4(mut o_ptr: *mut object_type,
                               mut level: libc::c_int,
                               mut power: libc::c_int) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut bonus_lvl: s32b = 0;
    let mut max_lvl: s32b = 0;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    /* Very good */
    if power > 1 as libc::c_int {
        /* Make ego item */
        if Rand_div(RANDART_JEWEL) == 1 as libc::c_int &&
               (*o_ptr).tval as libc::c_int == 39 as libc::c_int {
            create_artifact(o_ptr, 0 as libc::c_int as bool_,
                            1 as libc::c_int as bool_);
        } else { make_ego_item(o_ptr, 1 as libc::c_int as bool_); }
    } else if power < -(1 as libc::c_int) {
        /* Make ego item */
        make_ego_item(o_ptr, 0 as libc::c_int as bool_);
    }
    /* Apply magic (good or bad) according to type */
    if process_hooks(66 as libc::c_int,
                     b"(O,d,d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, o_ptr, level, power) != 0 {
        return
    }
    match (*o_ptr).tval as libc::c_int {
        111 => {
            /* Randomize random books */
            if (*o_ptr).sval as libc::c_int == 255 as libc::c_int {
                let mut i: libc::c_int = 0 as libc::c_int;
                /* Only random ones */
                if Rand_div(100 as libc::c_int) < 75 as libc::c_int {
                    i =
                        exec_lua(format(b"return get_random_spell(SKILL_MAGIC, %d)\x00"
                                            as *const u8 as
                                            *const libc::c_char, level))
                } else {
                    i =
                        exec_lua(format(b"return get_random_spell(SKILL_SPIRITUALITY, %d)\x00"
                                            as *const u8 as
                                            *const libc::c_char, level))
                }
                /* Use globe of light(or the first one) */
                if i == -(1 as libc::c_int) {
                    (*o_ptr).pval = 0 as libc::c_int
                } else { (*o_ptr).pval = i }
            }
        }
        39 => {
            object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                         &mut esp);
            /* Hack -- random fuel */
            if f4 as libc::c_long & 0x10000000 as libc::c_long != 0 {
                if (*k_info.offset((*o_ptr).k_idx as isize)).pval2 >
                       0 as libc::c_int {
                    (*o_ptr).timeout =
                        (Rand_div((*k_info.offset((*o_ptr).k_idx as
                                                      isize)).pval2) +
                             1 as libc::c_int) as s16b
                }
            }
        }
        9 => {
            /* Hack -- choose a monster */
            let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
            let mut r_idx: libc::c_int =
                get_mon_num(dun_level as libc::c_int) as libc::c_int;
            r_ptr = &mut *r_info.offset(r_idx as isize) as *mut monster_race;
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 {
                (*o_ptr).pval2 = r_idx as s16b
            } else { (*o_ptr).pval2 = 2 as libc::c_int as s16b }
            (*o_ptr).pval3 = 0 as libc::c_int
        }
        10 => {
            /* Hack -- choose a monster */
            let mut r_ptr_0: *mut monster_race =
                0 as *mut monster_race; /* Blue fire-lizard */
            let mut r_idx_0: libc::c_int = 0;
            let mut count: libc::c_int = 0 as libc::c_int;
            let mut OK: bool_ = 0 as libc::c_int as bool_;
            while OK == 0 && count < 1000 as libc::c_int {
                r_idx_0 =
                    get_mon_num(dun_level as libc::c_int) as libc::c_int;
                r_ptr_0 =
                    &mut *r_info.offset(r_idx_0 as isize) as
                        *mut monster_race;
                if (*r_ptr_0).flags9 & 0x10 as libc::c_int as libc::c_uint !=
                       0 {
                    (*o_ptr).pval2 = r_idx_0 as s16b;
                    OK = 1 as libc::c_int as bool_
                }
                count += 1
            }
            if count == 1000 as libc::c_int {
                (*o_ptr).pval2 = 940 as libc::c_int as s16b
            }
            r_ptr_0 =
                &mut *r_info.offset((*o_ptr).pval2 as isize) as
                    *mut monster_race;
            (*o_ptr).weight =
                (*r_ptr_0).weight +
                    Rand_div((*r_ptr_0).weight) / 100 as libc::c_int +
                    1 as libc::c_int;
            (*o_ptr).pval =
                (*r_ptr_0).weight * 3 as libc::c_int +
                    Rand_div((*r_ptr_0).weight) + 1 as libc::c_int
        }
        99 => {
            /* Hack -- choose a monster */
            let mut r_ptr_1: *mut monster_race = 0 as *mut monster_race;
            let mut r_idx_1: libc::c_int =
                get_mon_num(dun_level as libc::c_int) as libc::c_int;
            r_ptr_1 =
                &mut *r_info.offset(r_idx_1 as isize) as *mut monster_race;
            if (*r_ptr_1).flags1 & 0x20000 as libc::c_int as libc::c_uint == 0
               {
                (*o_ptr).pval = r_idx_1
            } else { (*o_ptr).pval = 20 as libc::c_int }
            r_idx_1 = (*o_ptr).pval;
            r_ptr_1 =
                &mut *r_info.offset(r_idx_1 as isize) as *mut monster_race;
            (*o_ptr).pval3 =
                maxroll((*r_ptr_1).hdice as s16b, (*r_ptr_1).hside as s16b);
            (*o_ptr).pval2 = (*o_ptr).pval2;
            (*o_ptr).exp = 0 as libc::c_int;
            (*o_ptr).elevel = (*r_ptr_1).level
        }
        65 => {
            /* Decide the spell, pval == -1 means to bypass spell selection */
            if (*o_ptr).pval != -(1 as libc::c_int) {
                let mut spl: libc::c_int =
                    exec_lua(b"return get_random_stick(TV_WAND, dun_level)\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char);
                if spl == -(1 as libc::c_int) {
                    spl =
                        exec_lua(b"return find_spell(\'Manathrust\')\x00" as
                                     *const u8 as *const libc::c_char as
                                     *mut libc::c_char)
                }
                (*o_ptr).pval2 = spl as s16b
            } else if (*k_ptr).pval == -(1 as libc::c_int) {
                (*o_ptr).pval2 = (*k_ptr).pval2 as s16b
            }
            /* Is the spell predefined by the object kind? */
            /* Ok now get a base level */
            call_lua(b"get_stick_base_level\x00" as *const u8 as
                         *const libc::c_char,
                     b"(d,d,d)\x00" as *const u8 as *const libc::c_char,
                     b"d\x00" as *const u8 as *const libc::c_char,
                     65 as libc::c_int, dun_level as libc::c_int,
                     (*o_ptr).pval2 as libc::c_int,
                     &mut bonus_lvl as *mut s32b);
            call_lua(b"get_stick_max_level\x00" as *const u8 as
                         *const libc::c_char,
                     b"(d,d,d)\x00" as *const u8 as *const libc::c_char,
                     b"d\x00" as *const u8 as *const libc::c_char,
                     65 as libc::c_int, dun_level as libc::c_int,
                     (*o_ptr).pval2 as libc::c_int,
                     &mut max_lvl as *mut s32b);
            (*o_ptr).pval3 =
                (max_lvl << 16 as libc::c_int) +
                    (bonus_lvl & 0xffff as libc::c_int);
            /* Hack -- charge wands */
            charge_stick(o_ptr);
        }
        55 => {
            /* Decide the spell, pval == -1 means to bypass spell selection */
            if (*o_ptr).pval != -(1 as libc::c_int) {
                let mut spl_0: libc::c_int =
                    exec_lua(b"return get_random_stick(TV_STAFF, dun_level)\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char);
                if spl_0 == -(1 as libc::c_int) {
                    spl_0 =
                        exec_lua(b"return find_spell(\'Globe of Light\')\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char)
                }
                (*o_ptr).pval2 = spl_0 as s16b
            } else if (*k_ptr).pval == -(1 as libc::c_int) {
                (*o_ptr).pval2 = (*k_ptr).pval2 as s16b
            }
            /* Is the spell predefined by the object kind? */
            /* Ok now get a base level */
            call_lua(b"get_stick_base_level\x00" as *const u8 as
                         *const libc::c_char,
                     b"(d,d,d)\x00" as *const u8 as *const libc::c_char,
                     b"d\x00" as *const u8 as *const libc::c_char,
                     55 as libc::c_int, dun_level as libc::c_int,
                     (*o_ptr).pval2 as libc::c_int,
                     &mut bonus_lvl as *mut s32b);
            call_lua(b"get_stick_max_level\x00" as *const u8 as
                         *const libc::c_char,
                     b"(d,d,d)\x00" as *const u8 as *const libc::c_char,
                     b"d\x00" as *const u8 as *const libc::c_char,
                     55 as libc::c_int, dun_level as libc::c_int,
                     (*o_ptr).pval2 as libc::c_int,
                     &mut max_lvl as *mut s32b);
            (*o_ptr).pval3 =
                (max_lvl << 16 as libc::c_int) +
                    (bonus_lvl & 0xffff as libc::c_int);
            /* Hack -- charge staffs */
            charge_stick(o_ptr);
        }
        7 => {
            /* Hack -- skip ruined chests */
            if !((*k_info.offset((*o_ptr).k_idx as isize)).level as
                     libc::c_int <= 0 as libc::c_int) {
                /* Pick a trap */
                place_trap_object(o_ptr);
                /* Hack - set pval2 to the number of objects in it */
                if (*o_ptr).pval != 0 {
                    (*o_ptr).pval2 =
                        ((*o_ptr).sval as libc::c_int % 4 as libc::c_int *
                             2 as libc::c_int) as s16b
                }
            }
        }
        71 => {
            if (*o_ptr).sval as libc::c_int == 3 as libc::c_int {
                /* Rating boost */
                rating = (rating as libc::c_int + 25 as libc::c_int) as s16b;
                /*  Mention the item */
                if cheat_peek as libc::c_int != 0 ||
                       (*p_ptr).precognition as libc::c_int != 0 {
                    object_mention(o_ptr);
                }
            }
        }
        72 => {
            if (*o_ptr).sval as libc::c_int == 1 as libc::c_int {
                let mut mimic: s32b = 0;
                call_lua(b"find_random_mimic_shape\x00" as *const u8 as
                             *const libc::c_char,
                         b"(d,d)\x00" as *const u8 as *const libc::c_char,
                         b"d\x00" as *const u8 as *const libc::c_char, level,
                         0 as libc::c_int, &mut mimic as *mut s32b);
                (*o_ptr).pval2 = mimic as s16b
            }
        }
        14 => {
            if !((*o_ptr).sval as libc::c_int != 60 as libc::c_int) {
                if (*o_ptr).name2 as libc::c_int == 130 as libc::c_int ||
                       (*o_ptr).name2b as libc::c_int == 130 as libc::c_int {
                    match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
                        1 => { (*o_ptr).pval2 = 1 as libc::c_int as s16b }
                        2 => { (*o_ptr).pval2 = 5 as libc::c_int as s16b }
                        3 => { (*o_ptr).pval2 = 4 as libc::c_int as s16b }
                        4 => { (*o_ptr).pval2 = 3 as libc::c_int as s16b }
                        _ => { }
                    }
                }
            }
        }
        12 | _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn trap_hack(mut o_ptr: *mut object_type) {
    if (*o_ptr).tval as libc::c_int != 46 as libc::c_int { return }
    match (*o_ptr).sval as libc::c_int {
        4 | 5 | 6 => {
            (*o_ptr).to_h = 0 as libc::c_int as s16b;
            (*o_ptr).to_d = 0 as libc::c_int as s16b
        }
        _ => { }
    };
}
/* Add a random glag to the ego item */
#[no_mangle]
pub unsafe extern "C" fn add_random_ego_flag(mut o_ptr: *mut object_type,
                                             mut fego: libc::c_int,
                                             mut limit_blows: *mut bool_) {
    if fego as libc::c_long & 0x1 as libc::c_long != 0 {
        /* Make a random sustain */
        match Rand_div(6 as libc::c_int) + 1 as libc::c_int {
            1 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x1 as libc::c_long) as u32b
            }
            2 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x2 as libc::c_long) as u32b
            }
            3 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x4 as libc::c_long) as u32b
            }
            4 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x8 as libc::c_long) as u32b
            }
            5 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x10 as libc::c_long) as u32b
            }
            6 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x20 as libc::c_long) as u32b
            }
            _ => { }
        }
    }
    if fego as libc::c_long & 0x2 as libc::c_long != 0 {
        /* Make a random resist, equal probabilities */
        match Rand_div(11 as libc::c_int) + 1 as libc::c_int {
            1 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x1000000 as libc::c_long) as u32b
            }
            2 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x2000000 as libc::c_long) as u32b
            }
            3 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x4000000 as libc::c_long) as u32b
            }
            4 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x8000000 as libc::c_long) as u32b
            }
            5 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x10000000 as libc::c_long) as u32b
            }
            6 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x20000000 as libc::c_long) as u32b
            }
            7 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x40000000 as libc::c_long) as u32b
            }
            8 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x80000000 as libc::c_long) as u32b
            }
            9 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x100000 as libc::c_long) as u32b
            }
            10 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x800000 as libc::c_long) as u32b
            }
            11 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x400000 as libc::c_long) as u32b
            }
            _ => { }
        }
    }
    if fego as libc::c_long & 0x4 as libc::c_long != 0 {
        /* Choose an ability */
        match Rand_div(8 as libc::c_int) + 1 as libc::c_int {
            1 => {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x1000 as libc::c_long) as u32b
            }
            2 => {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x2000 as libc::c_long) as u32b
            }
            3 => {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x4000 as libc::c_long) as u32b
            }
            4 => {
                (*o_ptr).art_esp =
                    ((*o_ptr).art_esp as libc::c_long |
                         0x80000000 as libc::c_long) as u32b
            }
            5 => {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x10000 as libc::c_long) as u32b
            }
            6 => {
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x20000 as libc::c_long) as u32b
            }
            7 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x4000 as libc::c_long) as u32b
            }
            8 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x8000 as libc::c_long) as u32b
            }
            _ => { }
        }
    }
    if fego as libc::c_long & 0x8 as libc::c_long != 0 {
        /* Make an acid/elec/fire/cold/poison resist */
        random_resistance(o_ptr, 0 as libc::c_int as bool_,
                          Rand_div(14 as libc::c_int) + 1 as libc::c_int +
                              4 as libc::c_int);
    }
    if fego as libc::c_long & 0x10 as libc::c_long != 0 {
        /* Make an acid/elec/fire/cold resist */
        random_resistance(o_ptr, 0 as libc::c_int as bool_,
                          Rand_div(12 as libc::c_int) + 1 as libc::c_int +
                              4 as libc::c_int);
    }
    if fego as libc::c_long & 0x20 as libc::c_long != 0 {
        /* Make a high resist */
        random_resistance(o_ptr, 0 as libc::c_int as bool_,
                          Rand_div(22 as libc::c_int) + 1 as libc::c_int +
                              16 as libc::c_int);
    }
    if fego as libc::c_long & 0x40 as libc::c_long != 0 {
        /* Make any resist */
        random_resistance(o_ptr, 0 as libc::c_int as bool_,
                          Rand_div(34 as libc::c_int) + 1 as libc::c_int +
                              4 as libc::c_int);
    }
    if fego as libc::c_long & 0x80 as libc::c_long != 0 {
        /* Make "dragon resist" */
        dragon_resist(o_ptr);
    }
    if fego as libc::c_long & 0x100 as libc::c_long != 0 {
        /* Make a Weapon of Slaying */
        if Rand_div(3 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
            /* double damage */
            (*o_ptr).dd =
                ((*o_ptr).dd as libc::c_int * 2 as libc::c_int) as byte_hack
        } else {
            loop  {
                (*o_ptr).dd = (*o_ptr).dd.wrapping_add(1);
                if !(Rand_div((*o_ptr).dd as s32b) + 1 as libc::c_int ==
                         1 as libc::c_int) {
                    break ;
                }
            }
            loop  {
                (*o_ptr).ds = (*o_ptr).ds.wrapping_add(1);
                if !(Rand_div((*o_ptr).ds as s32b) + 1 as libc::c_int ==
                         1 as libc::c_int) {
                    break ;
                }
            }
        }
        if Rand_div(5 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x8000000 as libc::c_long) as u32b
        }
        if (*o_ptr).tval as libc::c_int == 23 as libc::c_int &&
               Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
            (*o_ptr).art_flags1 =
                ((*o_ptr).art_flags1 as libc::c_long |
                     0x2000000 as libc::c_long) as u32b
        }
    }
    if fego as libc::c_long & 0x200 as libc::c_long != 0 {
        /* Increase damage dice */
        (*o_ptr).dd = (*o_ptr).dd.wrapping_add(1)
    }
    if fego as libc::c_long & 0x400 as libc::c_long != 0 {
        /* Increase damage dice size */
        (*o_ptr).ds = (*o_ptr).ds.wrapping_add(1)
    }
    if fego as libc::c_long & 0x80000000 as libc::c_long != 0 {
        /* Swap this flag */
        *limit_blows = (*limit_blows == 0) as libc::c_int as bool_
    }
    if fego as libc::c_long & 0x800 as libc::c_long != 0 {
        /* Increase pval */
        (*o_ptr).pval += 1
    }
    if fego as libc::c_long & 0x1000 as libc::c_long != 0 {
        /* Increase pval */
        (*o_ptr).pval +=
            m_bonus(2 as libc::c_int, dun_level as libc::c_int) as libc::c_int
    }
    if fego as libc::c_long & 0x2000 as libc::c_long != 0 {
        /* Increase pval */
        (*o_ptr).pval +=
            m_bonus(3 as libc::c_int, dun_level as libc::c_int) as libc::c_int
    }
    if fego as libc::c_long & 0x4000 as libc::c_long != 0 {
        /* Increase pval */
        (*o_ptr).pval +=
            m_bonus(5 as libc::c_int, dun_level as libc::c_int) as libc::c_int
    }
    if fego as libc::c_long & 0x8000 as libc::c_long != 0 {
        /* Increase ac */
        (*o_ptr).to_a += 1
    }
    if fego as libc::c_long & 0x10000 as libc::c_long != 0 {
        /* Increase ac */
        (*o_ptr).to_a =
            ((*o_ptr).to_a as libc::c_int +
                 m_bonus(2 as libc::c_int, dun_level as libc::c_int) as
                     libc::c_int) as s16b
    }
    if fego as libc::c_long & 0x20000 as libc::c_long != 0 {
        /* Increase ac */
        (*o_ptr).to_a =
            ((*o_ptr).to_a as libc::c_int +
                 m_bonus(3 as libc::c_int, dun_level as libc::c_int) as
                     libc::c_int) as s16b
    }
    if fego as libc::c_long & 0x40000 as libc::c_long != 0 {
        /* Increase ac */
        (*o_ptr).to_a =
            ((*o_ptr).to_a as libc::c_int +
                 m_bonus(5 as libc::c_int, dun_level as libc::c_int) as
                     libc::c_int) as s16b
    }
    if fego as libc::c_long & 0x80000 as libc::c_long != 0 {
        /* Increase to hit */
        (*o_ptr).to_h += 1
    }
    if fego as libc::c_long & 0x100000 as libc::c_long != 0 {
        /* Increase to hit */
        (*o_ptr).to_h =
            ((*o_ptr).to_h as libc::c_int +
                 m_bonus(2 as libc::c_int, dun_level as libc::c_int) as
                     libc::c_int) as s16b
    }
    if fego as libc::c_long & 0x200000 as libc::c_long != 0 {
        /* Increase to hit */
        (*o_ptr).to_h =
            ((*o_ptr).to_h as libc::c_int +
                 m_bonus(3 as libc::c_int, dun_level as libc::c_int) as
                     libc::c_int) as s16b
    }
    if fego as libc::c_long & 0x400000 as libc::c_long != 0 {
        /* Increase to hit */
        (*o_ptr).to_h =
            ((*o_ptr).to_h as libc::c_int +
                 m_bonus(5 as libc::c_int, dun_level as libc::c_int) as
                     libc::c_int) as s16b
    }
    if fego as libc::c_long & 0x800000 as libc::c_long != 0 {
        /* Increase to dam */
        (*o_ptr).to_d += 1
    }
    if fego as libc::c_long & 0x1000000 as libc::c_long != 0 {
        /* Increase to dam */
        (*o_ptr).to_d =
            ((*o_ptr).to_d as libc::c_int +
                 m_bonus(2 as libc::c_int, dun_level as libc::c_int) as
                     libc::c_int) as s16b
    }
    if fego as libc::c_long & 0x2000000 as libc::c_long != 0 {
        /* Increase to dam */
        (*o_ptr).to_d =
            ((*o_ptr).to_d as libc::c_int +
                 m_bonus(3 as libc::c_int, dun_level as libc::c_int) as
                     libc::c_int) as s16b
    }
    if fego as libc::c_long & 0x4000000 as libc::c_long != 0 {
        /* Increase to dam */
        (*o_ptr).to_d =
            ((*o_ptr).to_d as libc::c_int +
                 m_bonus(5 as libc::c_int, dun_level as libc::c_int) as
                     libc::c_int) as s16b
    }
    if fego as libc::c_long & 0x8000000 as libc::c_long != 0 {
        /* Add a random pval-affected ability */
		/* This might cause boots with + to blows */
        match Rand_div(6 as libc::c_int) + 1 as libc::c_int {
            1 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x100 as libc::c_long) as u32b
            }
            2 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x200 as libc::c_long) as u32b
            }
            3 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x400 as libc::c_long) as u32b
            }
            4 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x800 as libc::c_long) as u32b
            }
            5 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x1000 as libc::c_long) as u32b
            }
            6 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x2000 as libc::c_long) as u32b
            }
            _ => { }
        }
    }
    if fego as libc::c_long & 0x10000000 as libc::c_long != 0 {
        /* Add a random stat */
        match Rand_div(6 as libc::c_int) + 1 as libc::c_int {
            1 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x1 as libc::c_long) as u32b
            }
            2 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x2 as libc::c_long) as u32b
            }
            3 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x4 as libc::c_long) as u32b
            }
            4 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x8 as libc::c_long) as u32b
            }
            5 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x10 as libc::c_long) as u32b
            }
            6 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x20 as libc::c_long) as u32b
            }
            _ => { }
        }
    }
    if fego as libc::c_long & 0x20000000 as libc::c_long != 0 {
        /* Add a random stat and sustain it */
        match Rand_div(6 as libc::c_int) + 1 as libc::c_int {
            1 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x1 as libc::c_long) as u32b;
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x1 as libc::c_long) as u32b
            }
            2 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x2 as libc::c_long) as u32b;
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x2 as libc::c_long) as u32b
            }
            3 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x4 as libc::c_long) as u32b;
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x4 as libc::c_long) as u32b
            }
            4 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x8 as libc::c_long) as u32b;
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x8 as libc::c_long) as u32b
            }
            5 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x10 as libc::c_long) as u32b;
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x10 as libc::c_long) as u32b
            }
            6 => {
                (*o_ptr).art_flags1 =
                    ((*o_ptr).art_flags1 as libc::c_long |
                         0x20 as libc::c_long) as u32b;
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x20 as libc::c_long) as u32b
            }
            _ => { }
        }
    }
    if fego as libc::c_long & 0x40000000 as libc::c_long != 0 {
        /* Give a random immunity */
        match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
            1 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x400 as libc::c_long) as u32b;
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x400000 as libc::c_long) as u32b
            }
            2 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x100 as libc::c_long) as u32b;
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x100000 as libc::c_long) as u32b
            }
            3 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x200 as libc::c_long) as u32b;
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x200000 as libc::c_long) as u32b
            }
            4 => {
                (*o_ptr).art_flags2 =
                    ((*o_ptr).art_flags2 as libc::c_long |
                         0x800 as libc::c_long) as u32b;
                (*o_ptr).art_flags3 =
                    ((*o_ptr).art_flags3 as libc::c_long |
                         0x800000 as libc::c_long) as u32b
            }
            _ => { }
        }
    };
}
/*
 * Complete the "creation" of an object by applying "magic" to the item
 *
 * This includes not only rolling for random bonuses, but also putting the
 * finishing touches on ego-items and artifacts, giving charges to wands and
 * staffs, giving fuel to lites, and placing traps on chests.
 *
 * In particular, note that "Instant Artifacts", if "created" by an external
 * routine, must pass through this function to complete the actual creation.
 *
 * The base "chance" of the item being "good" increases with the "level"
 * parameter, which is usually derived from the dungeon level, being equal
 * to the level plus 10, up to a maximum of 75.  If "good" is true, then
 * the object is guaranteed to be "good".  If an object is "good", then
 * the chance that the object will be "great" (ego-item or artifact), also
 * increases with the "level", being equal to half the level, plus 5, up to
 * a maximum of 20.  If "great" is true, then the object is guaranteed to be
 * "great".  At dungeon level 65 and below, 15/100 objects are "great".
 *
 * If the object is not "good", there is a chance it will be "cursed", and
 * if it is "cursed", there is a chance it will be "broken".  These chances
 * are related to the "good" / "great" chances above.
 *
 * Otherwise "normal" rings and amulets will be "good" half the time and
 * "cursed" half the time, unless the ring/amulet is always good or cursed.
 *
 * If "okay" is true, and the object is going to be "great", then there is
 * a chance that an artifact will be created.  This is true even if both the
 * "good" and "great" arguments are false.  As a total hack, if "great" is
 * true, then the item gets 3 extra "attempts" to become an artifact.
 */
#[no_mangle]
pub static mut hack_apply_magic_power: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn apply_magic(mut o_ptr: *mut object_type,
                                     mut lev: libc::c_int, mut okay: bool_,
                                     mut good: bool_, mut great: bool_) {
    let mut i: libc::c_int = 0;
    let mut rolls: libc::c_int = 0;
    let mut f1: libc::c_int = 0;
    let mut f2: libc::c_int = 0;
    let mut power: libc::c_int = 0;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    /* Aply luck */
    lev += luck(-(7 as libc::c_int), 7 as libc::c_int);
    /* Spell in it ? no ! */
    if (*k_ptr).flags5 as libc::c_long & 0x800 as libc::c_long != 0 {
        (*o_ptr).pval2 = -(1 as libc::c_int) as s16b
    }
    /* Important to do before all else, be sure to have the basic obvious flags set */
    (*o_ptr).art_oflags1 = (*k_ptr).oflags1;
    (*o_ptr).art_oflags2 = (*k_ptr).oflags2;
    (*o_ptr).art_oflags3 = (*k_ptr).oflags3;
    (*o_ptr).art_oflags4 = (*k_ptr).oflags4;
    (*o_ptr).art_oflags5 = (*k_ptr).oflags5;
    (*o_ptr).art_oesp = (*k_ptr).oesp;
    /* No need to touch normal artifacts */
    if (*k_ptr).flags3 as libc::c_long & 0x8000 as libc::c_long != 0 {
        /* Ahah! we tried to trick us !! */
        if (*k_ptr).artifact as libc::c_int != 0 ||
               (*k_ptr).flags4 as libc::c_long & 0x400 as libc::c_long != 0 &&
                   *k_allow_special.offset((*o_ptr).k_idx as isize) == 0 {
            object_prep(o_ptr,
                        lookup_kind((*k_ptr).btval as libc::c_int,
                                    (*k_ptr).bsval as libc::c_int) as
                            libc::c_int);
            if wizard != 0 {
                msg_print(b"We\'ve been tricked!\x00" as *const u8 as
                              *const libc::c_char);
            }
        } else {
            /* Arg I hate so much to do that ... but I see no other way */
            if (*o_ptr).tval as libc::c_int == 65 as libc::c_int ||
                   (*o_ptr).tval as libc::c_int == 55 as libc::c_int {
                let mut base_lvl: s32b = 0;
                let mut max_lvl: s32b = 0;
                /* Is the spell predefined by the object kind? */
                if (*k_ptr).pval == -(1 as libc::c_int) {
                    (*o_ptr).pval2 = (*k_ptr).pval2 as s16b
                }
                /* Determine a base and a max level */
                call_lua(b"get_stick_base_level\x00" as *const u8 as
                             *const libc::c_char,
                         b"(d,d,d)\x00" as *const u8 as *const libc::c_char,
                         b"d\x00" as *const u8 as *const libc::c_char,
                         (*o_ptr).tval as libc::c_int,
                         dun_level as libc::c_int,
                         (*o_ptr).pval2 as libc::c_int,
                         &mut base_lvl as *mut s32b);
                call_lua(b"get_stick_max_level\x00" as *const u8 as
                             *const libc::c_char,
                         b"(d,d,d)\x00" as *const u8 as *const libc::c_char,
                         b"d\x00" as *const u8 as *const libc::c_char,
                         (*o_ptr).tval as libc::c_int,
                         dun_level as libc::c_int,
                         (*o_ptr).pval2 as libc::c_int,
                         &mut max_lvl as *mut s32b);
                (*o_ptr).pval3 =
                    (max_lvl << 16 as libc::c_int) +
                        (base_lvl & 0xffff as libc::c_int);
                /* Hack -- charge wands */
                charge_stick(o_ptr);
            }
            (*k_ptr).artifact = 1 as libc::c_int as bool_;
            if cheat_peek as libc::c_int != 0 ||
                   (*p_ptr).precognition as libc::c_int != 0 {
                object_mention(o_ptr);
            }
        }
        return
    }
    /* Maximum "level" for various things */
    if lev > 128 as libc::c_int - 1 as libc::c_int {
        lev = 128 as libc::c_int - 1 as libc::c_int
    }
    /* Base chance of being "good" */
    f1 =
        lev + 10 as libc::c_int +
            luck(-(15 as libc::c_int), 15 as libc::c_int);
    /* Maximal chance of being "good" */
    if f1 > 75 as libc::c_int { f1 = 75 as libc::c_int }
    /* Base chance of being "great" */
    f2 = f1 / 2 as libc::c_int;
    /* Maximal chance of being "great" */
    if f2 > 20 as libc::c_int { f2 = 20 as libc::c_int }
    /* Assume normal */
    power = 0 as libc::c_int;
    /* Roll for "good" */
    if good as libc::c_int != 0 || Rand_div(100 as libc::c_int) < f1 {
        /* Assume "good" */
        power = 1 as libc::c_int;
        /* Roll for "great" */
        if great as libc::c_int != 0 || Rand_div(100 as libc::c_int) < f2 {
            power = 2 as libc::c_int
        }
    } else if Rand_div(100 as libc::c_int) < f1 {
        /* Roll for "cursed" */
        /* Assume "cursed" */
        power = -(1 as libc::c_int);
        /* Roll for "broken" */
        if Rand_div(100 as libc::c_int) < f2 { power = -(2 as libc::c_int) }
    }
    /* Mega hack */
    if hack_apply_magic_power != 0 {
        if hack_apply_magic_power == -(99 as libc::c_int) {
            power = 0 as libc::c_int
        } else { power = hack_apply_magic_power }
    }
    hack_apply_magic_power = 0 as libc::c_int;
    /* Assume no rolls */
    rolls = 0 as libc::c_int;
    /* Get one roll if excellent */
    if power >= 2 as libc::c_int { rolls = 1 as libc::c_int }
    /* Hack -- Get four rolls if forced great */
    if great != 0 { rolls = 4 as libc::c_int }
    /* Hack -- Get no rolls if not allowed */
    if okay == 0 || (*o_ptr).name1 as libc::c_int != 0 {
        rolls = 0 as libc::c_int
    }
    /* Roll for artifacts if allowed */
    i = 0 as libc::c_int;
    while i < rolls {
        /* Roll for an artifact */
        if make_artifact(o_ptr) != 0 { break ; }
        i += 1
    }
    /* Mega hack -- to lazy to do it properly with hooks :) */
    if (*o_ptr).name1 as libc::c_int == 13 as libc::c_int &&
           ((*quest.offset(17 as libc::c_int as isize)).status as libc::c_int)
               < 1 as libc::c_int {
        (*o_ptr).name1 = 0 as libc::c_int as byte_hack;
        (*o_ptr).name2 = 0 as libc::c_int as s16b;
        (*o_ptr).name2b = 0 as libc::c_int as s16b;
        object_prep(o_ptr,
                    lookup_kind(45 as libc::c_int, 53 as libc::c_int) as
                        libc::c_int);
    }
    /* Hack -- analyze artifacts */
    if (*o_ptr).name1 != 0 {
        let mut a_ptr: *mut artifact_type =
            &mut *a_info.offset((*o_ptr).name1 as isize) as
                *mut artifact_type;
        /* Hack -- Mark the artifact as "created" */
        (*a_ptr).cur_num = 1 as libc::c_int as byte_hack;
        /* Extract the other fields */
        (*o_ptr).pval = (*a_ptr).pval as s32b;
        (*o_ptr).ac = (*a_ptr).ac;
        (*o_ptr).dd = (*a_ptr).dd;
        (*o_ptr).ds = (*a_ptr).ds;
        (*o_ptr).to_a = (*a_ptr).to_a;
        (*o_ptr).to_h = (*a_ptr).to_h;
        (*o_ptr).to_d = (*a_ptr).to_d;
        (*o_ptr).weight = (*a_ptr).weight as s32b;
        (*o_ptr).number = 1 as libc::c_int as byte_hack;
        /* Hack -- extract the "cursed" flag */
        if (*a_ptr).flags3 as libc::c_long & 0x20000000 as libc::c_long != 0 {
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                    byte_hack
        }
        /* Mega-Hack -- increase the rating */
        rating = (rating as libc::c_int + 10 as libc::c_int) as s16b;
        /* Mega-Hack -- increase the rating again */
        if (*a_ptr).cost as libc::c_long > 50000 as libc::c_long {
            rating = (rating as libc::c_int + 10 as libc::c_int) as s16b
        }
        /* Set the good item flag */
        good_item_flag = 1 as libc::c_int as bool_;
        /* Cheat -- peek at the item */
        if cheat_peek as libc::c_int != 0 ||
               (*p_ptr).precognition as libc::c_int != 0 {
            object_mention(o_ptr);
        }
        /* Spell in it ? no ! */
        if (*a_ptr).flags5 as libc::c_long & 0x800 as libc::c_long != 0 {
            (*o_ptr).pval2 = -(1 as libc::c_int) as s16b
        }
        /* Give a basic exp/exp level to an artifact that needs it */
        if (*a_ptr).flags4 as libc::c_long & 0x100 as libc::c_long != 0 {
            (*o_ptr).elevel =
                ((*k_ptr).level as libc::c_int / 10 as libc::c_int +
                     1 as libc::c_int) as byte_hack;
            (*o_ptr).exp =
                player_exp[((*o_ptr).elevel as libc::c_int - 1 as libc::c_int)
                               as usize]
        }
        /* Done */
        return
    }
    /* Apply magic */
    match (*o_ptr).tval as libc::c_int {
        102 => { finalize_randart(o_ptr, lev); }
        21 | 22 | 6 | 23 | 24 | 15 | 19 | 16 | 17 | 18 | 46 => {
            if power != 0 { a_m_aux_1(o_ptr, lev, power); }
        }
        115 => {
            /* UGLY, but needed, depending of sval teh demon stuff are eitehr weapon or armor */
            if (*o_ptr).sval as libc::c_int == 55 as libc::c_int {
                if power != 0 { a_m_aux_1(o_ptr, lev, power); }
            } else if power != 0 { a_m_aux_2(o_ptr, lev, power); }
        }
        38 | 37 | 36 | 34 | 32 | 33 | 35 | 31 | 30 => {
            a_m_aux_2(o_ptr, lev, power);
        }
        45 | 40 => {
            if power == 0 && Rand_div(100 as libc::c_int) < 50 as libc::c_int
               {
                power = -(1 as libc::c_int)
            }
            a_m_aux_3(o_ptr, lev, power);
        }
        _ => { a_m_aux_4(o_ptr, lev, power); }
    }
    if (*o_ptr).art_name != 0 {
        rating = (rating as libc::c_int + 40 as libc::c_int) as s16b
    } else if (*o_ptr).name2 != 0 {
        let mut e_ptr: *mut ego_item_type = 0 as *mut ego_item_type;
        let mut j: libc::c_int = 0;
        let mut limit_blows: bool_ = 0 as libc::c_int as bool_;
        let mut f1_0: u32b = 0;
        let mut f2_0: u32b = 0;
        let mut f3: u32b = 0;
        let mut f4: u32b = 0;
        let mut f5: u32b = 0;
        let mut esp: u32b = 0;
        let mut e_idx: s16b = 0;
        e_idx = (*o_ptr).name2;
        loop 
             /* Hack -- analyze ego-items */
             /* Ok now, THAT is truly ugly */
             {
            e_ptr = &mut *e_info.offset(e_idx as isize) as *mut ego_item_type;
            /* Hack -- extra powers */
            j = 0 as libc::c_int;
            while j < 5 as libc::c_int {
                /* Rarity check */
                if Rand_div(100 as libc::c_int) <
                       (*e_ptr).rar[j as usize] as libc::c_int {
                    (*o_ptr).art_flags1 |= (*e_ptr).flags1[j as usize];
                    (*o_ptr).art_flags2 |= (*e_ptr).flags2[j as usize];
                    (*o_ptr).art_flags3 |= (*e_ptr).flags3[j as usize];
                    (*o_ptr).art_flags4 |= (*e_ptr).flags4[j as usize];
                    (*o_ptr).art_flags5 |= (*e_ptr).flags5[j as usize];
                    (*o_ptr).art_esp |= (*e_ptr).esp[j as usize];
                    (*o_ptr).art_oflags1 |= (*e_ptr).oflags1[j as usize];
                    (*o_ptr).art_oflags2 |= (*e_ptr).oflags2[j as usize];
                    (*o_ptr).art_oflags3 |= (*e_ptr).oflags3[j as usize];
                    (*o_ptr).art_oflags4 |= (*e_ptr).oflags4[j as usize];
                    (*o_ptr).art_oflags5 |= (*e_ptr).oflags5[j as usize];
                    (*o_ptr).art_oesp |= (*e_ptr).oesp[j as usize];
                    add_random_ego_flag(o_ptr,
                                        (*e_ptr).fego[j as usize] as
                                            libc::c_int, &mut limit_blows);
                }
                j += 1
            }
            /* No insane number of blows */
            if limit_blows as libc::c_int != 0 &&
                   (*o_ptr).art_flags1 as libc::c_long &
                       0x2000 as libc::c_long != 0 {
                if (*o_ptr).pval > 2 as libc::c_int {
                    (*o_ptr).pval =
                        Rand_div(2 as libc::c_int) + 1 as libc::c_int
                }
            }
            /* get flags */
            object_flags(o_ptr, &mut f1_0, &mut f2_0, &mut f3, &mut f4,
                         &mut f5, &mut esp);
            /* Hack -- acquire "cursed" flag */
            if f3 as libc::c_long & 0x20000000 as libc::c_long != 0 {
                (*o_ptr).ident =
                    ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                        byte_hack
            }
            /* Hack -- obtain bonuses */
            if (*e_ptr).max_to_h as libc::c_int > 0 as libc::c_int {
                (*o_ptr).to_h =
                    ((*o_ptr).to_h as libc::c_int +
                         (Rand_div((*e_ptr).max_to_h as s32b) +
                              1 as libc::c_int)) as s16b
            }
            if ((*e_ptr).max_to_h as libc::c_int) < 0 as libc::c_int {
                (*o_ptr).to_h =
                    ((*o_ptr).to_h as libc::c_int -
                         (Rand_div(-((*e_ptr).max_to_h as libc::c_int)) +
                              1 as libc::c_int)) as s16b
            }
            if (*e_ptr).max_to_d as libc::c_int > 0 as libc::c_int {
                (*o_ptr).to_d =
                    ((*o_ptr).to_d as libc::c_int +
                         (Rand_div((*e_ptr).max_to_d as s32b) +
                              1 as libc::c_int)) as s16b
            }
            if ((*e_ptr).max_to_d as libc::c_int) < 0 as libc::c_int {
                (*o_ptr).to_d =
                    ((*o_ptr).to_d as libc::c_int -
                         (Rand_div(-((*e_ptr).max_to_d as libc::c_int)) +
                              1 as libc::c_int)) as s16b
            }
            if (*e_ptr).max_to_a as libc::c_int > 0 as libc::c_int {
                (*o_ptr).to_a =
                    ((*o_ptr).to_a as libc::c_int +
                         (Rand_div((*e_ptr).max_to_a as s32b) +
                              1 as libc::c_int)) as s16b
            }
            if ((*e_ptr).max_to_a as libc::c_int) < 0 as libc::c_int {
                (*o_ptr).to_a =
                    ((*o_ptr).to_a as libc::c_int -
                         (Rand_div(-((*e_ptr).max_to_a as libc::c_int)) +
                              1 as libc::c_int)) as s16b
            }
            /* Hack -- obtain pval */
            if (*e_ptr).max_pval > 0 as libc::c_int {
                (*o_ptr).pval +=
                    Rand_div((*e_ptr).max_pval) + 1 as libc::c_int
            }
            if (*e_ptr).max_pval < 0 as libc::c_int {
                (*o_ptr).pval -=
                    Rand_div(-(*e_ptr).max_pval) + 1 as libc::c_int
            }
            /* Hack -- apply rating bonus */
            rating =
                (rating as libc::c_int + (*e_ptr).rating as libc::c_int) as
                    s16b;
            if !((*o_ptr).name2b as libc::c_int != 0 &&
                     (*o_ptr).name2b as libc::c_int != e_idx as libc::c_int) {
                break ;
            }
            e_idx = (*o_ptr).name2b
        }
        /* Spell in it ? no ! */
        if f5 as libc::c_long & 0x800 as libc::c_long != 0 {
            /* Mega hack, mage staves of spell cannot SPELL_CONTAIN */
            if (*o_ptr).name2 as libc::c_int == 4 as libc::c_int ||
                   (*o_ptr).name2b as libc::c_int == 4 as libc::c_int {
                (*o_ptr).art_flags5 =
                    ((*o_ptr).art_flags5 as libc::c_long &
                         !(0x800 as libc::c_long)) as u32b
            } else { (*o_ptr).pval2 = -(1 as libc::c_int) as s16b }
        }
        /* Cheat -- describe the item */
        if cheat_peek as libc::c_int != 0 ||
               (*p_ptr).precognition as libc::c_int != 0 {
            object_mention(o_ptr);
        }
    }
    /* Examine real objects */
    if (*o_ptr).k_idx != 0 {
        let mut f1_1: u32b = 0;
        let mut f2_1: u32b = 0;
        let mut f3_0: u32b = 0;
        let mut f4_0: u32b = 0;
        let mut f5_0: u32b = 0;
        let mut esp_0: u32b = 0;
        let mut k_ptr_0: *mut object_kind =
            &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
        /* Hack -- acquire "cursed" flag */
        if (*k_ptr_0).flags3 as libc::c_long & 0x20000000 as libc::c_long != 0
           {
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                    byte_hack
        }
        /* Extract some flags */
        object_flags(o_ptr, &mut f1_1, &mut f2_1, &mut f3_0, &mut f4_0,
                     &mut f5_0, &mut esp_0);
        /* Hack give a basic exp/exp level to an object that needs it */
        if f4_0 as libc::c_long & 0x100 as libc::c_long != 0 {
            (*o_ptr).elevel =
                ((*k_ptr_0).level as libc::c_int / 10 as libc::c_int +
                     1 as libc::c_int) as byte_hack;
            (*o_ptr).exp =
                player_exp[((*o_ptr).elevel as libc::c_int - 1 as libc::c_int)
                               as usize]
        }
        /* Spell in it ? no ! */
        if f5_0 as libc::c_long & 0x800 as libc::c_long != 0 {
            /* Mega hack, mage staves of spell cannot SPELL_CONTAIN */
            if (*o_ptr).name2 as libc::c_int == 4 as libc::c_int ||
                   (*o_ptr).name2b as libc::c_int == 4 as libc::c_int {
                (*o_ptr).art_flags5 =
                    ((*o_ptr).art_flags5 as libc::c_long &
                         !(0x800 as libc::c_long)) as u32b
            } else { (*o_ptr).pval2 = -(1 as libc::c_int) as s16b }
        }
        /* Hacccccccckkkkk attack ! :) -- To prevent som ugly crashs */
        if (*o_ptr).tval as libc::c_int == 6 as libc::c_int &&
               (*o_ptr).sval as libc::c_int == 1 as libc::c_int &&
               (*o_ptr).pval < 0 as libc::c_int {
            (*o_ptr).pval = 0 as libc::c_int
        }
        /* Hack, cant be done in a_m_aux4 because the ego flags are not yet in place */
        if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
            /* Set the max mana and the current mana */
            (*o_ptr).pval2 =
                if f4_0 as libc::c_long & 0x2000 as libc::c_long != 0 {
                    ((*o_ptr).sval as libc::c_int) * 2 as libc::c_int
                } else { (*o_ptr).sval as libc::c_int } as s16b;
            (*o_ptr).timeout = (*o_ptr).pval2
        }
        /* Remove some unnecessary stuff hack */
        if (*o_ptr).tval as libc::c_int == 46 as libc::c_int {
            trap_hack(o_ptr);
        }
    };
}
/* The themed objects to use */
static mut match_theme: obj_theme =
    obj_theme{treasure: 0, combat: 0, magic: 0, tools: 0,};
/*
 * XXX XXX XXX It relies on the fact that obj_theme is a four byte structure
 * for its efficient operation. A horrendous hack, I'd say.
 */
#[no_mangle]
pub unsafe extern "C" fn init_match_theme(mut theme: obj_theme) {
    /* Save the theme */
    match_theme = theme;
}
/*
 * Ditto XXX XXX XXX
 */
unsafe extern "C" fn theme_changed(mut theme: obj_theme) -> bool_ {
    /* Any of the themes has been changed */
    if theme.treasure as libc::c_int != match_theme.treasure as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    if theme.combat as libc::c_int != match_theme.combat as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    if theme.magic as libc::c_int != match_theme.magic as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    if theme.tools as libc::c_int != match_theme.tools as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* No changes */
    return 0 as libc::c_int as bool_;
}
/*
 * Maga-Hack -- match certain types of object only.
 */
#[no_mangle]
pub unsafe extern "C" fn kind_is_theme(mut k_idx: libc::c_int) -> bool_ {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(k_idx as isize) as *mut object_kind;
    let mut prob: s32b = 0 as libc::c_int;
    /*
	 * Paranoia -- Prevent accidental "(Nothing)"
	 * that are results of uninitialised theme structs.
	 *
	 * Caution: Junks go into the allocation table.
	 */
    if match_theme.treasure as libc::c_int + match_theme.combat as libc::c_int
           + match_theme.magic as libc::c_int +
           match_theme.tools as libc::c_int == 0 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Pick probability to use */
    match (*k_ptr).tval as libc::c_int {
        1 | 2 | 11 | 9 | 10 => {
            /*
			 * Degree of junk is defined in terms of the other
			 * 4 quantities XXX XXX XXX
			 * The type of prob should be *signed* as well as
			 * larger than theme components, or we would see
			 * unexpected, well, junks.
			 */
            prob =
                100 as libc::c_int -
                    (match_theme.treasure as libc::c_int +
                         match_theme.combat as libc::c_int +
                         match_theme.magic as libc::c_int +
                         match_theme.tools as libc::c_int)
        }
        7 => { prob = match_theme.treasure as s32b }
        33 => { prob = match_theme.treasure as s32b }
        38 => { prob = match_theme.treasure as s32b }
        40 => { prob = match_theme.treasure as s32b }
        45 => { prob = match_theme.treasure as s32b }
        16 => { prob = match_theme.combat as s32b }
        17 => { prob = match_theme.combat as s32b }
        18 => { prob = match_theme.combat as s32b }
        15 => { prob = match_theme.combat as s32b }
        19 => { prob = match_theme.combat as s32b }
        21 => { prob = match_theme.combat as s32b }
        22 => { prob = match_theme.combat as s32b }
        23 => { prob = match_theme.combat as s32b }
        24 => { prob = match_theme.combat as s32b }
        31 => { prob = match_theme.combat as s32b }
        32 => { prob = match_theme.combat as s32b }
        34 => { prob = match_theme.combat as s32b }
        36 => { prob = match_theme.combat as s32b }
        37 => { prob = match_theme.combat as s32b }
        6 => { prob = match_theme.magic as s32b }
        55 => { prob = match_theme.magic as s32b }
        65 => { prob = match_theme.magic as s32b }
        66 => { prob = match_theme.magic as s32b }
        67 => { prob = match_theme.magic as s32b }
        70 => { prob = match_theme.magic as s32b }
        8 => { prob = match_theme.magic as s32b }
        71 => { prob = match_theme.magic as s32b }
        72 => { prob = match_theme.magic as s32b }
        4 => { prob = match_theme.magic as s32b }
        102 => { prob = match_theme.magic as s32b }
        104 => { prob = match_theme.magic as s32b }
        105 => { prob = match_theme.magic as s32b }
        111 => { prob = match_theme.magic as s32b }
        112 => { prob = match_theme.magic as s32b }
        113 => { prob = match_theme.magic as s32b }
        114 => { prob = match_theme.magic as s32b }
        115 => { prob = match_theme.magic as s32b }
        39 => { prob = match_theme.tools as s32b }
        35 => { prob = match_theme.tools as s32b }
        30 => { prob = match_theme.tools as s32b }
        5 => { prob = match_theme.tools as s32b }
        20 => { prob = match_theme.tools as s32b }
        77 => { prob = match_theme.tools as s32b }
        80 => { prob = match_theme.tools as s32b }
        12 => { prob = match_theme.tools as s32b }
        14 => { prob = match_theme.tools as s32b }
        46 => { prob = match_theme.tools as s32b }
        _ => { }
    }
    /* Roll to see if it can be made */
    if Rand_div(100 as libc::c_int) < prob {
        return 1 as libc::c_int as bool_
    }
    /* Not a match */
    return 0 as libc::c_int as bool_;
}
/*
 * Determine if an object must not be generated.
 */
#[no_mangle]
pub static mut kind_is_legal_special: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub unsafe extern "C" fn kind_is_legal(mut k_idx: libc::c_int) -> bool_ {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(k_idx as isize) as *mut object_kind;
    if kind_is_theme(k_idx) == 0 { return 0 as libc::c_int as bool_ }
    if (*k_ptr).flags4 as libc::c_long & 0x400 as libc::c_long != 0 {
        if *k_allow_special.offset(k_idx as isize) != 0 {
            return 1 as libc::c_int as bool_
        } else { return 0 as libc::c_int as bool_ }
    }
    /* No 2 times the same normal artifact */
    if (*k_ptr).flags3 as libc::c_long & 0x8000 as libc::c_long != 0 &&
           (*k_ptr).artifact as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*k_ptr).tval as libc::c_int == 9 as libc::c_int {
        if (*k_ptr).sval as libc::c_int != 4 as libc::c_int &&
               (*k_ptr).sval as libc::c_int != 2 as libc::c_int &&
               (*k_ptr).sval as libc::c_int != 3 as libc::c_int &&
               (*k_ptr).sval as libc::c_int != 1 as libc::c_int {
            return 1 as libc::c_int as bool_
        } else { return 0 as libc::c_int as bool_ }
    }
    if (*k_ptr).tval as libc::c_int == 99 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Used only for the Nazgul rings */
    if (*k_ptr).tval as libc::c_int == 45 as libc::c_int &&
           (*k_ptr).sval as libc::c_int == 5 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Are we forced to one tval ? */
    if kind_is_legal_special != -(1 as libc::c_int) &&
           kind_is_legal_special != (*k_ptr).tval as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Assume legal */
    return 1 as libc::c_int as bool_;
}
/*
 * Hack -- determine if a template is "good"
 */
#[no_mangle]
pub unsafe extern "C" fn kind_is_good(mut k_idx: libc::c_int) -> bool_ {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(k_idx as isize) as *mut object_kind;
    if kind_is_legal(k_idx) == 0 { return 0 as libc::c_int as bool_ }
    /* Analyze the item type */
    match (*k_ptr).tval as libc::c_int {
        37 | 36 | 38 | 34 | 35 | 30 | 31 | 32 | 33 => {
            /* Armor -- Good unless damaged */
            if ((*k_ptr).to_a as libc::c_int) < 0 as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            return 1 as libc::c_int as bool_
        }
        19 | 23 | 24 | 21 | 22 | 20 | 46 | 6 | 15 => {
            /* Weapons -- Good unless damaged */
            if ((*k_ptr).to_h as libc::c_int) < 0 as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            if ((*k_ptr).to_d as libc::c_int) < 0 as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            return 1 as libc::c_int as bool_
        }
        18 | 17 => {
            /* Ammo -- Arrows/Bolts are good */
            return 1 as libc::c_int as bool_
        }
        67 => {
            /* Rods - Silver and better are good */
            if (*k_ptr).sval as libc::c_int >= 100 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            return 0 as libc::c_int as bool_
        }
        66 => {
            /* Expensive rod tips are good */
            /* Probing is not good, but Recall is*/
            if (*k_ptr).cost >= 4500 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            return 0 as libc::c_int as bool_
        }
        111 => {
            /* The Tomes are good */
            if (*k_ptr).sval as libc::c_int <= 49 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            return 0 as libc::c_int as bool_
        }
        45 => {
            /* Rings -- Rings of Speed are good */
            if (*k_ptr).sval as libc::c_int == 31 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            return 0 as libc::c_int as bool_
        }
        40 => {
            /* Amulets -- Some are good */
            if (*k_ptr).sval as libc::c_int == 8 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            if (*k_ptr).sval as libc::c_int == 25 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            if (*k_ptr).sval as libc::c_int == 24 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            if (*k_ptr).sval as libc::c_int == 23 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            if (*k_ptr).sval as libc::c_int == 15 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            if (*k_ptr).sval as libc::c_int == 9 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            if (*k_ptr).sval as libc::c_int == 22 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            return 0 as libc::c_int as bool_
        }
        _ => { }
    }
    /* Assume not good */
    return 0 as libc::c_int as bool_;
}
/*
* Determine if template is suitable for building a randart -- dsb
*/
#[no_mangle]
pub unsafe extern "C" fn kind_is_artifactable(mut k_idx: libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(k_idx as isize) as *mut object_kind;
    if kind_is_good(k_idx) != 0 {
        /* We consider the item artifactable if there is at least one
		* randart power in ra_info that could be added to this item. */
        i = 0 as libc::c_int;
        while i < max_ra_idx as libc::c_int {
            let mut ra_ptr: *mut randart_part_type =
                &mut *ra_info.offset(i as isize) as *mut randart_part_type;
            j = 0 as libc::c_int;
            while j < 20 as libc::c_int {
                if !((*ra_ptr).tval[j as usize] as libc::c_int !=
                         (*k_ptr).tval as libc::c_int) {
                    if !((*ra_ptr).min_sval[j as usize] as libc::c_int >
                             (*k_ptr).sval as libc::c_int) {
                        if !(((*ra_ptr).max_sval[j as usize] as libc::c_int) <
                                 (*k_ptr).sval as libc::c_int) {
                            /* Winner */
                            return 1 as libc::c_int as bool_
                        }
                    }
                }
                j += 1
            }
            i += 1
        }
    }
    /* No match. Too bad. */
    return 0 as libc::c_int as bool_;
}
/*
 * Attempt to make an object (normal or good/great)
 *
 * This routine plays nasty games to generate the "special artifacts".
 *
 * This routine uses "object_level" for the "generation level".
 *
 * We assume that the given object has been "wiped".
 *
 * To Watch: The allocation table caching is heavily relies on
 * an assumption that the SPECIAL_GENE objects should only be created
 * through the forge--object_prep()--apply_magic() sequence and
 * get_obj_num() should never be called for that purpose XXX XXX XXX
 */
#[no_mangle]
pub unsafe extern "C" fn make_object(mut j_ptr: *mut object_type,
                                     mut good: bool_, mut great: bool_,
                                     mut theme: obj_theme) -> bool_ {
    let mut invprob: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    /* Chance of "special object" */
    invprob =
        if good as libc::c_int != 0 {
            (10 as libc::c_int) - luck(-(9 as libc::c_int), 9 as libc::c_int)
        } else { 1000 as libc::c_int };
    /* Base level for the object */
    base =
        if good as libc::c_int != 0 {
            (object_level as libc::c_int) + 10 as libc::c_int
        } else { object_level as libc::c_int };
    /* Generate a special object, or a normal object */
    if Rand_div(invprob) != 0 as libc::c_int ||
           make_artifact_special(j_ptr) == 0 {
        let mut k_idx: libc::c_int = 0;
        /* See if the theme has been changed XXX XXX XXX */
        if theme_changed(theme) != 0 {
            /* Select items based on "theme" */
            init_match_theme(theme);
            /* Invalidate the cached allocation table */
            alloc_kind_table_valid = 0 as libc::c_int as bool_
        }
        /* Good objects */
        if good != 0 {
            /* Activate restriction */
            get_obj_num_hook =
                Some(kind_is_good as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_);
            /* Prepare allocation table */
            get_obj_num_prep();
        } else if alloc_kind_table_valid == 0 {
            /* Normal objects -- only when the cache is invalidated */
            /* Activate normal restriction */
            get_obj_num_hook =
                Some(kind_is_legal as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_);
            /* Prepare allocation table */
            get_obj_num_prep();
            /* The table is synchronised */
            alloc_kind_table_valid = 1 as libc::c_int as bool_
        }
        /* Pick a random object */
        k_idx = get_obj_num(base) as libc::c_int;
        /* Good objects */
        if good != 0 {
            /* Restore normal restriction */
            get_obj_num_hook =
                Some(kind_is_legal as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_);
            /* Prepare allocation table */
            get_obj_num_prep();
            /* The table is synchronised */
            alloc_kind_table_valid = 1 as libc::c_int as bool_
        }
        /* Handle failure */
        if k_idx == 0 { return 0 as libc::c_int as bool_ }
        /* Prepare the object */
        object_prep(j_ptr, k_idx);
    }
    /* Apply magic (allow artifacts) */
    apply_magic(j_ptr, object_level as libc::c_int, 1 as libc::c_int as bool_,
                good, great);
    /* Hack -- generate multiple spikes/missiles */
    match (*j_ptr).tval as libc::c_int {
        5 | 16 | 17 | 18 => {
            (*j_ptr).number =
                damroll(6 as libc::c_int as s16b, 7 as libc::c_int as s16b) as
                    byte_hack
        }
        _ => { }
    }
    /* hack, no multiple artifacts */
    if (*j_ptr).tval as libc::c_int == 102 as libc::c_int ||
           (if (*j_ptr).name1 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) != 0 ||
           (if (*j_ptr).art_name as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) != 0 ||
           (if (*k_info.offset((*j_ptr).k_idx as isize)).flags3 as
                   libc::c_long & 0x8000 as libc::c_long != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) != 0 {
        (*j_ptr).number = 1 as libc::c_int as byte_hack
    }
    /* Notice "okay" out-of-depth objects */
    if (*j_ptr).ident as libc::c_int & 0x40 as libc::c_int == 0 &&
           (*k_info.offset((*j_ptr).k_idx as isize)).level as libc::c_int >
               dun_level as libc::c_int {
        /* Rating increase */
        rating =
            (rating as libc::c_int +
                 ((*k_info.offset((*j_ptr).k_idx as isize)).level as
                      libc::c_int - dun_level as libc::c_int)) as s16b;
        /* Cheat -- peek at items */
        if cheat_peek as libc::c_int != 0 ||
               (*p_ptr).precognition as libc::c_int != 0 {
            object_mention(j_ptr);
        }
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Attempt to place an object (normal or good/great) at the given location.
 *
 * This routine plays nasty games to generate the "special artifacts".
 *
 * This routine uses "object_level" for the "generation level".
 *
 * This routine requires a clean floor grid destination.
 */
#[no_mangle]
pub unsafe extern "C" fn place_object(mut y: libc::c_int, mut x: libc::c_int,
                                      mut good: bool_, mut great: bool_,
                                      mut where_0: libc::c_int) {
    let mut o_idx: s16b = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
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
    /* Paranoia -- check bounds */
    if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
             y < cur_hgt as libc::c_int - 1 as libc::c_int &&
             x < cur_wid as libc::c_int - 1 as libc::c_int) {
        return
    }
    /* Require clean floor space */
    if !((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                             isize)).flags1 as libc::c_long &
             0x10 as libc::c_long != 0 &&
             (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                 0xaf as libc::c_int &&
             (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int ==
                 0 as libc::c_int &&
             (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                 isize)).flags1 as libc::c_long &
                 0x40 as libc::c_long == 0) {
        return
    }
    /* Get local object */
    q_ptr = &mut forge;
    /* Wipe the object */
    object_wipe(q_ptr);
    /* Make an object (if possible) */
    if make_object(q_ptr, good, great,
                   (*d_info.offset(dungeon_type as isize)).objs) == 0 {
        return
    }
    if where_0 == 3 as libc::c_int {
        (*q_ptr).found = 3 as libc::c_int as byte_hack;
        (*q_ptr).found_aux1 = dungeon_type as s16b;
        (*q_ptr).found_aux2 =
            if dungeon_type as libc::c_int == 0 as libc::c_int {
                (*(*wild_map.offset((*p_ptr).wilderness_y as
                                        isize)).offset((*p_ptr).wilderness_x
                                                           as isize)).feat
            } else { dun_level as libc::c_int } as s16b
    } else if where_0 == 2 as libc::c_int {
        (*q_ptr).found = 2 as libc::c_int as byte_hack;
        (*q_ptr).found_aux1 = dungeon_type as s16b;
        (*q_ptr).found_aux2 =
            if dungeon_type as libc::c_int == 0 as libc::c_int {
                (*(*wild_map.offset((*p_ptr).wilderness_y as
                                        isize)).offset((*p_ptr).wilderness_x
                                                           as isize)).feat
            } else { dun_level as libc::c_int } as s16b
    } else if where_0 == 4 as libc::c_int {
        (*q_ptr).found = 4 as libc::c_int as byte_hack
    } else if where_0 == 5 as libc::c_int {
        (*q_ptr).found = 5 as libc::c_int as byte_hack
    }
    /* Make an object */
    o_idx = o_pop();
    /* Success */
    if o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(o_idx as isize) as *mut object_type;
        /* Structure Copy */
        object_copy(o_ptr, q_ptr);
        /* Location */
        (*o_ptr).iy = y as byte_hack;
        (*o_ptr).ix = x as byte_hack;
        /* Acquire grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Build a stack */
        (*o_ptr).next_o_idx = (*c_ptr).o_idx;
        /* Place the object */
        (*c_ptr).o_idx = o_idx;
        /* Notice */
        note_spot(y, x);
        /* Redraw */
        lite_spot(y, x);
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
/* Object array overflow */
/* Hack -- Preserve artifacts */
/* Number of "gold" entries */
/*
 * Make a treasure object
 *
 * The location must be a legal, clean, floor grid.
 */
#[no_mangle]
pub unsafe extern "C" fn make_gold(mut j_ptr: *mut object_type) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut base: s32b = 0;
    /* Hack -- Pick a Treasure variety */
    i =
        (Rand_div(object_level as libc::c_int + 2 as libc::c_int) +
             1 as libc::c_int + 2 as libc::c_int) / 2 as libc::c_int -
            1 as libc::c_int;
    /* Apply "extra" magic */
    if Rand_div(20 as libc::c_int) == 0 as libc::c_int {
        i +=
            Rand_div(object_level as libc::c_int + 1 as libc::c_int) +
                1 as libc::c_int
    }
    /* Hack -- Creeping Coins only generate "themselves" */
    if coin_type != 0 { i = coin_type as libc::c_int }
    /* Do not create "illegal" Treasure Types */
    if i >= 18 as libc::c_int { i = 18 as libc::c_int - 1 as libc::c_int }
    /* Prepare a gold object */
    object_prep(j_ptr, 480 as libc::c_int + i);
    /* Hack -- Base coin cost */
    base = (*k_info.offset((480 as libc::c_int + i) as isize)).cost;
    /* Determine how much the treasure is "worth" */
    (*j_ptr).pval =
        (base as libc::c_long +
             8 as libc::c_long *
                 (Rand_div(base) + 1 as libc::c_int) as libc::c_long +
             (Rand_div(8 as libc::c_int) + 1 as libc::c_int) as libc::c_long)
            as s32b;
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Places a treasure (Gold or Gems) at given location
 *
 * The location must be a legal, clean, floor grid.
 */
#[no_mangle]
pub unsafe extern "C" fn place_gold(mut y: libc::c_int, mut x: libc::c_int) {
    let mut o_idx: s16b = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
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
    /* Paranoia -- check bounds */
    if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
             y < cur_hgt as libc::c_int - 1 as libc::c_int &&
             x < cur_wid as libc::c_int - 1 as libc::c_int) {
        return
    }
    /* Require clean floor space */
    if !((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                             isize)).flags1 as libc::c_long &
             0x10 as libc::c_long != 0 &&
             (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                 0xaf as libc::c_int &&
             (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int ==
                 0 as libc::c_int &&
             (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                 isize)).flags1 as libc::c_long &
                 0x40 as libc::c_long == 0) {
        return
    }
    /* Get local object */
    q_ptr = &mut forge;
    /* Wipe the object */
    object_wipe(q_ptr);
    /* Make some gold */
    if make_gold(q_ptr) == 0 { return }
    /* Make an object */
    o_idx = o_pop();
    /* Success */
    if o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(o_idx as isize) as *mut object_type;
        /* Copy the object */
        object_copy(o_ptr, q_ptr);
        /* Save location */
        (*o_ptr).iy = y as byte_hack;
        (*o_ptr).ix = x as byte_hack;
        /* Acquire grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Build a stack */
        (*o_ptr).next_o_idx = (*c_ptr).o_idx;
        /* Place the object */
        (*c_ptr).o_idx = o_idx;
        /* Notice */
        note_spot(y, x);
        /* Redraw */
        lite_spot(y, x);
    };
}
/*
 * Let an object fall to the ground at or near a location.
 *
 * The initial location is assumed to be "in_bounds()".
 *
 * This function takes a parameter "chance".  This is the percentage
 * chance that the item will "disappear" instead of drop.  If the object
 * has been thrown, then this is the chance of disappearance on contact.
 *
 * Hack -- this function uses "chance" to determine if it should produce
 * some form of "description" of the drop event (under the player).
 *
 * We check several locations to see if we can find a location at which
 * the object can combine, stack, or be placed.  Artifacts will try very
 * hard to be placed, including "teleporting" to a useful grid if needed.
 */
#[no_mangle]
pub unsafe extern "C" fn drop_near(mut j_ptr: *mut object_type,
                                   mut chance: libc::c_int,
                                   mut y: libc::c_int, mut x: libc::c_int)
 -> s16b {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut bs: libc::c_int = 0;
    let mut bn: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut bx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    let mut o_idx: s16b = 0 as libc::c_int as s16b;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    let mut plural: bool_ = 0 as libc::c_int as bool_;
    /* Extract plural */
    if (*j_ptr).number as libc::c_int != 1 as libc::c_int {
        plural = 1 as libc::c_int as bool_
    }
    /* Describe object */
    object_desc(o_name.as_mut_ptr(), j_ptr, 0 as libc::c_int,
                0 as libc::c_int);
    /* Handle normal "breakage" */
    if !((*j_ptr).art_name as libc::c_int != 0 ||
             ((*j_ptr).tval as libc::c_int == 102 as libc::c_int ||
                  (if (*j_ptr).name1 as libc::c_int != 0 {
                       1 as libc::c_int
                   } else { 0 as libc::c_int }) != 0 ||
                  (if (*j_ptr).art_name as libc::c_int != 0 {
                       1 as libc::c_int
                   } else { 0 as libc::c_int }) != 0 ||
                  (if (*k_info.offset((*j_ptr).k_idx as isize)).flags3 as
                          libc::c_long & 0x8000 as libc::c_long != 0 {
                       1 as libc::c_int
                   } else { 0 as libc::c_int }) != 0)) &&
           Rand_div(100 as libc::c_int) < chance {
        /* Message */
        msg_format(b"The %s disappear%s.\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   if plural as libc::c_int != 0 {
                       b"\x00" as *const u8 as *const libc::c_char
                   } else { b"s\x00" as *const u8 as *const libc::c_char });
        /* Debug */
        if wizard != 0 {
            msg_print(b"(breakage)\x00" as *const u8 as *const libc::c_char);
        }
        /* Failure */
        return 0 as libc::c_int as s16b
    }
    /* Score */
    bs = -(1 as libc::c_int);
    /* Picker */
    bn = 0 as libc::c_int;
    /* Default */
    by = y;
    bx = x;
    /* Scan local grids */
    dy = -(3 as libc::c_int);
    while dy <= 3 as libc::c_int {
        /* Scan local grids */
        dx = -(3 as libc::c_int);
        while dx <= 3 as libc::c_int {
            let mut comb: bool_ = 0 as libc::c_int as bool_;
            /* Calculate actual distance */
            d = dy * dy + dx * dx;
            /* Ignore distant grids */
            if !(d > 10 as libc::c_int) {
                /* Location */
                ty = y + dy;
                tx = x + dx;
                /* Skip illegal grids */
                if ty > 0 as libc::c_int && tx > 0 as libc::c_int &&
                       ty < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       tx < cur_wid as libc::c_int - 1 as libc::c_int {
                    /* Require line of sight */
                    if !(los(y, x, ty, tx) == 0) {
                        /* Obtain grid */
                        c_ptr =
                            &mut *(*cave.as_mut_ptr().offset(ty as
                                                                 isize)).offset(tx
                                                                                    as
                                                                                    isize)
                                as *mut cave_type;
                        /* Require floor space (or shallow terrain) -KMW- */
                        if !((*f_info.offset((*c_ptr).feat as isize)).flags1
                                 as libc::c_long & 0x10 as libc::c_long == 0)
                           {
                            /* No traps */
                            if !((*c_ptr).t_idx != 0) {
                                /* No objects */
                                k = 0 as libc::c_int;
                                /* Scan objects in that grid */
                                this_o_idx = (*c_ptr).o_idx;
                                while this_o_idx != 0 {
                                    let mut o_ptr: *mut object_type =
                                        0 as *mut object_type;
                                    /* Acquire object */
                                    o_ptr =
                                        &mut *o_list.offset(this_o_idx as
                                                                isize) as
                                            *mut object_type;
                                    /* Acquire next object */
                                    next_o_idx = (*o_ptr).next_o_idx;
                                    /* Check for possible combination */
                                    if object_similar(o_ptr, j_ptr) != 0 {
                                        comb = 1 as libc::c_int as bool_
                                    }
                                    /* Count objects */
                                    k += 1;
                                    this_o_idx = next_o_idx
                                }
                                /* Add new object */
                                if comb == 0 { k += 1 }
                                /* No stacking (allow combining) */
                                if !(testing_stack == 0 &&
                                         k > 1 as libc::c_int) {
                                    /* Paranoia */
                                    if !(k > 23 as libc::c_int) {
                                        /* Calculate score */
                                        s =
                                            1000 as libc::c_int -
                                                (d + k * 5 as libc::c_int);
                                        /* Skip bad values */
                                        if !(s < bs) {
                                            /* New best value */
                                            if s > bs {
                                                bn = 0 as libc::c_int
                                            }
                                            /* Apply the randomizer to equivalent values */
                                            bn += 1;
                                            if !(bn >= 2 as libc::c_int &&
                                                     Rand_div(bn) !=
                                                         0 as libc::c_int) {
                                                /* Keep score */
                                                bs = s;
                                                /* Track it */
                                                by = ty;
                                                bx = tx;
                                                /* Okay */
                                                flag =
                                                    1 as libc::c_int as bool_
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            dx += 1
        }
        dy += 1
    }
    /* Handle lack of space */
    if flag == 0 &&
           !((*j_ptr).tval as libc::c_int == 102 as libc::c_int ||
                 (if (*j_ptr).name1 as libc::c_int != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0 ||
                 (if (*j_ptr).art_name as libc::c_int != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0 ||
                 (if (*k_info.offset((*j_ptr).k_idx as isize)).flags3 as
                         libc::c_long & 0x8000 as libc::c_long != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0 ||
                 (*j_ptr).art_name as libc::c_int != 0) {
        /* Message */
        msg_format(b"The %s disappear%s.\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   if plural as libc::c_int != 0 {
                       b"\x00" as *const u8 as *const libc::c_char
                   } else { b"s\x00" as *const u8 as *const libc::c_char });
        /* Debug */
        if wizard != 0 {
            msg_print(b"(no floor space)\x00" as *const u8 as
                          *const libc::c_char);
        }
        /* Failure */
        return 0 as libc::c_int as s16b
    }
    /* Find a grid */
    i = 0 as libc::c_int;
    while flag == 0 {
        /* Bounce around */
        if i < 1000 as libc::c_int {
            ty =
                by +
                    Rand_div(1 as libc::c_int + 1 as libc::c_int +
                                 1 as libc::c_int) - 1 as libc::c_int;
            tx =
                bx +
                    Rand_div(1 as libc::c_int + 1 as libc::c_int +
                                 1 as libc::c_int) - 1 as libc::c_int
        } else {
            /* Random locations */
            ty = Rand_div(cur_hgt as s32b);
            tx = Rand_div(cur_wid as s32b)
        }
        /* Grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(ty as isize)).offset(tx as isize)
                as *mut cave_type;
        /* Require floor space (or shallow terrain) -KMW- */
        if !((*c_ptr).feat as libc::c_int != 0x1 as libc::c_int &&
                 (*c_ptr).feat as libc::c_int != 0x54 as libc::c_int &&
                 (*c_ptr).feat as libc::c_int != 0x59 as libc::c_int &&
                 (*c_ptr).feat as libc::c_int != 0x58 as libc::c_int &&
                 (*c_ptr).feat as libc::c_int != 0x56 as libc::c_int) {
            /* Bounce to that location */
            by = ty;
            bx = tx;
            /* Require floor space */
            if (*f_info.offset((*cave[by as usize].offset(bx as isize)).feat
                                   as isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[by as usize].offset(bx as isize)).feat as
                       libc::c_int != 0xaf as libc::c_int &&
                   (*cave[by as usize].offset(bx as isize)).o_idx as
                       libc::c_int == 0 as libc::c_int &&
                   (*f_info.offset((*cave[by as
                                              usize].offset(bx as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x40 as libc::c_long == 0 {
                /* Okay */
                flag = 1 as libc::c_int as bool_
            }
        }
        i += 1
    }
    /* Grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(by as isize)).offset(bx as isize) as
            *mut cave_type;
    /* Scan objects in that grid for combination */
    this_o_idx = (*c_ptr).o_idx;
    while this_o_idx != 0 {
        let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr_0 =
            &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr_0).next_o_idx;
        /* Check for combination */
        if object_similar(o_ptr_0, j_ptr) != 0 {
            /* Combine the items */
            object_absorb(o_ptr_0, j_ptr);
            /* Success */
            done = 1 as libc::c_int as bool_;
            break ;
        } else { this_o_idx = next_o_idx }
    }
    /* Get new object */
    if done == 0 { o_idx = o_pop() }
    /* Failure */
    if done == 0 && o_idx == 0 {
        /* Message */
        msg_format(b"The %s disappear%s.\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   if plural as libc::c_int != 0 {
                       b"\x00" as *const u8 as *const libc::c_char
                   } else { b"s\x00" as *const u8 as *const libc::c_char });
        /* Debug */
        if wizard != 0 {
            msg_print(b"(too many objects)\x00" as *const u8 as
                          *const libc::c_char);
        }
        /* Hack -- Preserve artifacts */
        if (*j_ptr).name1 != 0 {
            (*a_info.offset((*j_ptr).name1 as isize)).cur_num =
                0 as libc::c_int as byte_hack
        } else if (*k_info.offset((*j_ptr).k_idx as isize)).flags3 as
                      libc::c_long & 0x8000 as libc::c_long != 0 {
            (*k_info.offset((*j_ptr).k_idx as isize)).artifact =
                0 as libc::c_int as bool_
        } else if (*j_ptr).tval as libc::c_int == 102 as libc::c_int {
            random_artifacts[(*j_ptr).sval as usize].generated =
                0 as libc::c_int as byte_hack
        }
        /* Failure */
        return 0 as libc::c_int as s16b
    }
    /* Stack */
    if done == 0 {
        /* Structure copy */
        object_copy(&mut *o_list.offset(o_idx as isize), j_ptr);
        /* Access new object */
        j_ptr = &mut *o_list.offset(o_idx as isize) as *mut object_type;
        /* Locate */
        (*j_ptr).iy = by as byte_hack;
        (*j_ptr).ix = bx as byte_hack;
        /* No monster */
        (*j_ptr).held_m_idx = 0 as libc::c_int as s16b;
        /* Build a stack */
        (*j_ptr).next_o_idx = (*c_ptr).o_idx;
        /* Place the object */
        (*c_ptr).o_idx = o_idx;
        /* Success */
        done = 1 as libc::c_int as bool_
    }
    /* Note the spot */
    note_spot(by, bx);
    /* Draw the spot */
    lite_spot(by, bx);
    /* Mega-Hack -- no message if "dropped" by player */
	/* Message when an object falls under the player */
    if chance != 0 && by == (*p_ptr).py as libc::c_int &&
           bx == (*p_ptr).px as libc::c_int {
        msg_print(b"You feel something roll beneath your feet.\x00" as
                      *const u8 as *const libc::c_char);
        /* Sound */
        sound(4 as libc::c_int);
    }
    /* XXX XXX XXX */
    /* Result */
    return o_idx;
}
/*
 * Scatter some "great" objects near the player
 */
#[no_mangle]
pub unsafe extern "C" fn acquirement(mut y1: libc::c_int, mut x1: libc::c_int,
                                     mut num: libc::c_int, mut great: bool_,
                                     mut known: bool_) {
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
    loop 
         /* Acquirement */
         {
        let fresh1 = num;
        num = num - 1;
        if !(fresh1 != 0) { break ; }
        /* Get local object */
        i_ptr = &mut object_type_body;
        /* Wipe the object */
        object_wipe(i_ptr);
        /* Make a good (or great) object (if possible) */
        if make_object(i_ptr, 1 as libc::c_int as bool_, great,
                       (*d_info.offset(dungeon_type as isize)).objs) == 0 {
            continue ;
        }
        if known != 0 { object_aware(i_ptr); object_known(i_ptr); }
        /* Drop the object */
        drop_near(i_ptr, -(1 as libc::c_int), y1, x1);
    };
}
/*
 * Hack -- instantiate a trap
 *
 * XXX XXX XXX This routine should be redone to reflect trap "level".
 * That is, it does not make sense to have spiked pits at 50 feet.
 * Actually, it is not this routine, but the "trap instantiation"
 * code, which should also check for "trap doors" on quest levels.
 */
#[no_mangle]
pub unsafe extern "C" fn pick_trap(mut y: libc::c_int, mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Paranoia */
    if (*c_ptr).t_idx as libc::c_int == 0 as libc::c_int ||
           (*c_ptr).info as libc::c_int & 0x100 as libc::c_int != 0 {
        return
    }
    /* Activate the trap */
    (*c_ptr).info =
        ((*c_ptr).info as libc::c_int | 0x100 as libc::c_int) as u16b;
    /* Notice and redraw */
    note_spot(y, x);
    lite_spot(y, x);
}
/*
 * Describe the charges on an item in the inventory.
 */
#[no_mangle]
pub unsafe extern "C" fn inven_item_charges(mut item: libc::c_int) {
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    /* Require staff/wand */
    if (*o_ptr).tval as libc::c_int != 55 as libc::c_int &&
           (*o_ptr).tval as libc::c_int != 65 as libc::c_int {
        return
    }
    /* Require known item */
    if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
             (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                 libc::c_int != 0 &&
                 (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                     libc::c_int != 0) {
        return
    }
    /* Multiple charges */
    if (*o_ptr).pval != 1 as libc::c_int {
        /* Print a message */
        msg_format(b"You have %d charges remaining.\x00" as *const u8 as
                       *const libc::c_char, (*o_ptr).pval);
    } else {
        /* Single charge */
        /* Print a message */
        msg_format(b"You have %d charge remaining.\x00" as *const u8 as
                       *const libc::c_char, (*o_ptr).pval);
    };
}
/*
 * Describe an item in the inventory.
 */
#[no_mangle]
pub unsafe extern "C" fn inven_item_describe(mut item: libc::c_int) {
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Get a description */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Print a message */
    msg_format(b"You have %s.\x00" as *const u8 as *const libc::c_char,
               o_name.as_mut_ptr());
}
/*
 * Increase the "number" of an item in the inventory
 */
#[no_mangle]
pub unsafe extern "C" fn inven_item_increase(mut item: libc::c_int,
                                             mut num: libc::c_int) {
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    /* Apply */
    num += (*o_ptr).number as libc::c_int;
    /* Bounds check */
    if num > 255 as libc::c_int {
        num = 255 as libc::c_int
    } else if num < 0 as libc::c_int { num = 0 as libc::c_int }
    /* Un-apply */
    num -= (*o_ptr).number as libc::c_int;
    /* Change the number and weight */
    if num != 0 {
        /* Add the number */
        (*o_ptr).number = ((*o_ptr).number as libc::c_int + num) as byte_hack;
        /* Recalculate bonuses */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Recalculate mana XXX */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x20 as libc::c_long) as u32b;
        /* Combine the pack */
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
    };
}
/*
 * Erase an inventory slot if it has no more items
 */
#[no_mangle]
pub unsafe extern "C" fn inven_item_optimize(mut item: libc::c_int) -> bool_ {
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    /* Only optimize real items */
    if (*o_ptr).k_idx == 0 { return 0 as libc::c_int as bool_ }
    /* Only optimize empty items */
    if (*o_ptr).number != 0 { return 0 as libc::c_int as bool_ }
    /* The item is in the pack */
    if item < 24 as libc::c_int {
        let mut i: libc::c_int = 0;
        /* One less item */
        inven_cnt -= 1;
        /* Slide everything down */
        i = item;
        while i < 23 as libc::c_int {
            /* Structure copy */
            (*p_ptr).inventory[i as usize] =
                (*p_ptr).inventory[(i + 1 as libc::c_int) as usize];
            i += 1
        }
        /* Erase the "final" slot */
        object_wipe(&mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize));
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x1 as libc::c_long) as u32b
    } else {
        /* The item is being wielded */
        /* One less item */
        equip_cnt -= 1;
        if (*o_ptr).name1 != 0 {
            takeoff_set((*p_ptr).inventory[item as usize].name1 as s16b,
                        (*a_info.offset((*p_ptr).inventory[item as
                                                               usize].name1 as
                                            isize)).set);
        }
        object_wipe(&mut *(*p_ptr).inventory.as_mut_ptr().offset(item as
                                                                     isize));
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as u32b;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x20 as libc::c_long) as u32b;
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x2 as libc::c_long) as u32b
    }
    return 1 as libc::c_int as bool_;
}
/* Take care of item sets*/
/* Erase the empty slot */
/* Recalculate bonuses */
/* Recalculate torch */
/* Recalculate mana XXX */
/* Window stuff */
/*
 * Describe the charges on an item on the floor.
 */
#[no_mangle]
pub unsafe extern "C" fn floor_item_charges(mut item: libc::c_int) {
    let mut o_ptr: *mut object_type =
        &mut *o_list.offset(item as isize) as *mut object_type;
    /* Require staff/wand */
    if (*o_ptr).tval as libc::c_int != 55 as libc::c_int &&
           (*o_ptr).tval as libc::c_int != 65 as libc::c_int {
        return
    }
    /* Require known item */
    if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
             (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                 libc::c_int != 0 &&
                 (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                     libc::c_int != 0) {
        return
    }
    /* Multiple charges */
    if (*o_ptr).pval != 1 as libc::c_int {
        /* Print a message */
        msg_format(b"There are %d charges remaining.\x00" as *const u8 as
                       *const libc::c_char, (*o_ptr).pval);
    } else {
        /* Single charge */
        /* Print a message */
        msg_format(b"There is %d charge remaining.\x00" as *const u8 as
                       *const libc::c_char, (*o_ptr).pval);
    };
}
/*
 * Describe an item in the inventory.
 */
#[no_mangle]
pub unsafe extern "C" fn floor_item_describe(mut item: libc::c_int) {
    let mut o_ptr: *mut object_type =
        &mut *o_list.offset(item as isize) as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Get a description */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Print a message */
    msg_format(b"You see %s.\x00" as *const u8 as *const libc::c_char,
               o_name.as_mut_ptr());
}
/*
 * Increase the "number" of an item on the floor
 */
#[no_mangle]
pub unsafe extern "C" fn floor_item_increase(mut item: libc::c_int,
                                             mut num: libc::c_int) {
    let mut o_ptr: *mut object_type =
        &mut *o_list.offset(item as isize) as *mut object_type;
    /* Apply */
    num += (*o_ptr).number as libc::c_int;
    /* Bounds check */
    if num > 255 as libc::c_int {
        num = 255 as libc::c_int
    } else if num < 0 as libc::c_int { num = 0 as libc::c_int }
    /* Un-apply */
    num -= (*o_ptr).number as libc::c_int;
    /* Change the number */
    (*o_ptr).number = ((*o_ptr).number as libc::c_int + num) as byte_hack;
}
/*
 * Optimize an item on the floor (destroy "empty" items)
 */
#[no_mangle]
pub unsafe extern "C" fn floor_item_optimize(mut item: libc::c_int) {
    let mut o_ptr: *mut object_type =
        &mut *o_list.offset(item as isize) as *mut object_type;
    /* Paranoia -- be sure it exists */
    if (*o_ptr).k_idx == 0 { return }
    /* Only optimize empty items */
    if (*o_ptr).number != 0 { return }
    /* Delete the object */
    delete_object_idx(item);
}
/*
 * Increase stack size for item, describe and optimize.
 */
#[no_mangle]
pub unsafe extern "C" fn inc_stack_size(mut item: libc::c_int,
                                        mut delta: libc::c_int) {
    inc_stack_size_ex(item, delta, OPTIMIZE, DESCRIBE);
}
/*
 * Increase stack size for item.
 */
#[no_mangle]
pub unsafe extern "C" fn inc_stack_size_ex(mut item: libc::c_int,
                                           mut delta: libc::c_int,
                                           mut opt: optimize_flag,
                                           mut desc: describe_flag) {
    /* Pack item? */
    if item >= 0 as libc::c_int {
        inven_item_increase(item, delta);
        if desc as libc::c_uint == DESCRIBE as libc::c_int as libc::c_uint {
            inven_item_describe(item);
        }
        if opt as libc::c_uint == OPTIMIZE as libc::c_int as libc::c_uint {
            inven_item_optimize(item);
        }
    } else {
        /* Floor item? */
        floor_item_increase(0 as libc::c_int - item, delta);
        if desc as libc::c_uint == DESCRIBE as libc::c_int as libc::c_uint {
            floor_item_describe(0 as libc::c_int - item);
        }
        if opt as libc::c_uint == OPTIMIZE as libc::c_int as libc::c_uint {
            floor_item_optimize(0 as libc::c_int - item);
        }
    };
}
/*
 * Check if we have space for an item in the pack without overflow
 */
#[no_mangle]
pub unsafe extern "C" fn inven_carry_okay(mut o_ptr: *mut object_type)
 -> bool_ {
    let mut j: libc::c_int = 0;
    if (*o_ptr).tval as libc::c_int == 100 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Empty slot? */
    if (inven_cnt as libc::c_int) < 23 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Similar slot? */
    j = 0 as libc::c_int;
    while j < 23 as libc::c_int {
        let mut j_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(j as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*j_ptr).k_idx == 0) {
            /* Check if the two items can be combined */
            if object_similar(j_ptr, o_ptr) != 0 {
                return 1 as libc::c_int as bool_
            }
        }
        j += 1
    }
    /* Nope */
    return 0 as libc::c_int as bool_;
}
/*
 * Add an item to the players inventory, and return the slot used.
 *
 * If the new item can combine with an existing item in the inventory,
 * it will do so, using "object_similar()" and "object_absorb()", otherwise,
 * the item will be placed into the "proper" location in the inventory.
 *
 * This function can be used to "over-fill" the player's pack, but only
 * once, and such an action must trigger the "overflow" code immediately.
 * Note that when the pack is being "over-filled", the new item must be
 * placed into the "overflow" slot, and the "overflow" must take place
 * before the pack is reordered, but (optionally) after the pack is
 * combined.  This may be tricky.  See "dungeon.c" for info.
 *
 * Note that this code must remove any location/stack information
 * from the object once it is placed into the inventory.
 *
 * The "final" flag tells this function to bypass the "combine"
 * and "reorder" code until later.
 */
#[no_mangle]
pub unsafe extern "C" fn inven_carry(mut o_ptr: *mut object_type,
                                     mut final_0: bool_) -> s16b {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = -(1 as libc::c_int);
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    /* Not final */
    if final_0 == 0 {
        /* Check for combining */
        j = 0 as libc::c_int;
        while j < 23 as libc::c_int {
            j_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(j as isize) as
                    *mut object_type;
            /* Skip non-objects */
            if !((*j_ptr).k_idx == 0) {
                /* Hack -- track last item */
                n = j;
                /* Check if the two items can be combined */
                if object_similar(j_ptr, o_ptr) != 0 {
                    /* Combine the items */
                    object_absorb(j_ptr, o_ptr);
                    /* Recalculate bonuses */
                    (*p_ptr).update =
                        ((*p_ptr).update as libc::c_long |
                             0x1 as libc::c_long) as u32b;
                    /* Window stuff */
                    (*p_ptr).window =
                        ((*p_ptr).window as libc::c_long |
                             0x1 as libc::c_long) as u32b;
                    /* Success */
                    return j as s16b
                }
            }
            j += 1
        }
    }
    /* Paranoia */
    if inven_cnt as libc::c_int > 23 as libc::c_int {
        return -(1 as libc::c_int) as s16b
    }
    /* Find an empty slot */
    j = 0 as libc::c_int;
    while j <= 23 as libc::c_int {
        j_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(j as isize) as
                *mut object_type;
        /* Use it if found */
        if (*j_ptr).k_idx == 0 { break ; }
        j += 1
    }
    /* Use that slot */
    i = j;
    /* Hack -- pre-reorder the pack */
    if final_0 == 0 && i < 23 as libc::c_int {
        let mut o_value: s32b = 0;
        let mut j_value: s32b = 0;
        /* Get the "value" of the item */
        o_value = object_value(o_ptr);
        let mut current_block_21: u64;
        /* Scan every occupied slot */
        j = 0 as libc::c_int;
        while j < 23 as libc::c_int {
            j_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(j as isize) as
                    *mut object_type;
            /* Use empty slots */
            if (*j_ptr).k_idx == 0 { break ; }
            /* Objects sort by decreasing type */
            if (*o_ptr).tval as libc::c_int > (*j_ptr).tval as libc::c_int {
                break ;
            }
            if !(((*o_ptr).tval as libc::c_int) <
                     (*j_ptr).tval as libc::c_int) {
                /* Non-aware (flavored) items always come last */
                if !((*k_info.offset((*o_ptr).k_idx as isize)).aware == 0) {
                    if (*k_info.offset((*j_ptr).k_idx as isize)).aware == 0 {
                        break ;
                    }
                    /* Objects sort by increasing sval */
                    if ((*o_ptr).sval as libc::c_int) <
                           (*j_ptr).sval as libc::c_int {
                        break ;
                    }
                    if !((*o_ptr).sval as libc::c_int >
                             (*j_ptr).sval as libc::c_int) {
                        /* Unidentified objects always come last */
                        if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int
                               != 0 ||
                               (*k_info.offset((*o_ptr).k_idx as
                                                   isize)).easy_know as
                                   libc::c_int != 0 &&
                                   (*k_info.offset((*o_ptr).k_idx as
                                                       isize)).aware as
                                       libc::c_int != 0 {
                            if !((*j_ptr).ident as libc::c_int &
                                     0x8 as libc::c_int != 0 ||
                                     (*k_info.offset((*j_ptr).k_idx as
                                                         isize)).easy_know as
                                         libc::c_int != 0 &&
                                         (*k_info.offset((*j_ptr).k_idx as
                                                             isize)).aware as
                                             libc::c_int != 0) {
                                break ;
                            }
                            /* Hack:  otherwise identical rods sort by
			increasing recharge time --dsb */
                            if (*o_ptr).tval as libc::c_int ==
                                   67 as libc::c_int {
                                if (*o_ptr).timeout as libc::c_int >
                                       (*j_ptr).timeout as libc::c_int {
                                    break ;
                                }
                                if ((*o_ptr).timeout as libc::c_int) <
                                       (*j_ptr).timeout as libc::c_int {
                                    current_block_21 = 2719512138335094285;
                                } else {
                                    current_block_21 = 17784502470059252271;
                                }
                            } else {
                                current_block_21 = 17784502470059252271;
                            }
                            match current_block_21 {
                                2719512138335094285 => { }
                                _ => {
                                    /* Determine the "value" of the pack item */
                                    j_value = object_value(j_ptr);
                                    /* Objects sort by decreasing value */
                                    if o_value > j_value { break ; }
                                    (o_value) < j_value;
                                }
                            }
                        }
                    }
                }
            }
            j += 1
        }
        /* Use that slot */
        i = j;
        /* Slide objects */
        k = n;
        while k >= i {
            /* Hack -- Slide the item */
            object_copy(&mut *(*p_ptr).inventory.as_mut_ptr().offset((k +
                                                                          1 as
                                                                              libc::c_int)
                                                                         as
                                                                         isize),
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset(k as
                                                                         isize));
            k -= 1
        }
        /* Wipe the empty slot */
        object_wipe(&mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize));
    }
    /* Acquire a copy of the item */
    object_copy(&mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize),
                o_ptr);
    /* Access new object */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
            *mut object_type;
    /* Clean out unused fields */
    (*o_ptr).ix = 0 as libc::c_int as byte_hack;
    (*o_ptr).iy = (*o_ptr).ix;
    (*o_ptr).next_o_idx = 0 as libc::c_int as s16b;
    (*o_ptr).held_m_idx = 0 as libc::c_int as s16b;
    /* Count the items */
    inven_cnt += 1;
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Combine and Reorder pack */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Return the slot */
    return i as s16b;
}
/*
 * Take off (some of) a non-cursed equipment item
 *
 * Note that only one item at a time can be wielded per slot.
 *
 * Note that taking off an item when "full" may cause that item
 * to fall to the ground.
 *
 * Return the inventory slot into which the item is placed.
 */
#[no_mangle]
pub unsafe extern "C" fn inven_takeoff(mut item: libc::c_int,
                                       mut amt: libc::c_int,
                                       mut force_drop: bool_) -> s16b {
    let mut slot: libc::c_int = 0;
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
    let mut act: cptr = 0 as *const libc::c_char;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Get the item to take off */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    /* Paranoia */
    if amt <= 0 as libc::c_int { return -(1 as libc::c_int) as s16b }
    /* Verify */
    if amt > (*o_ptr).number as libc::c_int {
        amt = (*o_ptr).number as libc::c_int
    }
    /* Get local object */
    q_ptr = &mut forge;
    /* Obtain a local object */
    object_copy(q_ptr, o_ptr);
    /* Modify quantity */
    (*q_ptr).number = amt as byte_hack;
    /* Describe the object */
    object_desc(o_name.as_mut_ptr(), q_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Took off weapon */
    if item == 24 as libc::c_int {
        act = b"You were wielding\x00" as *const u8 as *const libc::c_char
    } else if item == 27 as libc::c_int {
        act = b"You were holding\x00" as *const u8 as *const libc::c_char
    } else if item == 36 as libc::c_int {
        act = b"You were holding\x00" as *const u8 as *const libc::c_char
    } else if item == 50 as libc::c_int {
        act =
            b"You were carrying in your quiver\x00" as *const u8 as
                *const libc::c_char
    } else if item == 51 as libc::c_int {
        act = b"You were using\x00" as *const u8 as *const libc::c_char
    } else {
        /* Took off bow */
        /* Took off light */
        /* Took off ammo */
        /* Took off tool */
        /* Took off something */
        act = b"You were wearing\x00" as *const u8 as *const libc::c_char
    }
    /* Modify, Optimize */
    inc_stack_size_ex(item, -amt, OPTIMIZE, NO_DESCRIBE);
    if item == 49 as libc::c_int &&
           get_skill(8 as libc::c_int) as libc::c_int != 0 {
        /* Drop the monster */
        (*o_ptr).pval2 = 0 as libc::c_int as s16b;
        msg_print(b"You carefully drop the poor monster on the floor.\x00" as
                      *const u8 as *const libc::c_char);
        drop_near(q_ptr, 0 as libc::c_int, (*p_ptr).py as libc::c_int,
                  (*p_ptr).px as libc::c_int);
        slot = -(1 as libc::c_int)
    } else if force_drop != 0 {
        drop_near(q_ptr, 0 as libc::c_int, (*p_ptr).py as libc::c_int,
                  (*p_ptr).px as libc::c_int);
        slot = -(1 as libc::c_int)
    } else {
        /* Carry the object */
        slot = inven_carry(q_ptr, 0 as libc::c_int as bool_) as libc::c_int
    }
    /* Message */
    msg_format(b"%s %s (%c).\x00" as *const u8 as *const libc::c_char, act,
               o_name.as_mut_ptr(), index_to_label(slot) as libc::c_int);
    /* Return slot */
    return slot as s16b;
}
/*
 * Drop (some of) a non-cursed inventory/equipment item
 *
 * The object will be dropped "near" the current location
 */
#[no_mangle]
pub unsafe extern "C" fn inven_drop(mut item: libc::c_int,
                                    mut amt: libc::c_int, mut dy: libc::c_int,
                                    mut dx: libc::c_int, mut silent: bool_) {
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
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Access original object */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    /* Error check */
    if amt <= 0 as libc::c_int { return }
    /* Not too many */
    if amt > (*o_ptr).number as libc::c_int {
        amt = (*o_ptr).number as libc::c_int
    }
    /* Take off equipment */
    if item >= 24 as libc::c_int {
        /* Take off first */
        item =
            inven_takeoff(item, amt, 0 as libc::c_int as bool_) as
                libc::c_int;
        /* Access original object */
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
                *mut object_type
    }
    if item > -(1 as libc::c_int) {
        /* Get local object */
        q_ptr = &mut forge;
        /* Obtain local object */
        object_copy(q_ptr, o_ptr);
        /*
		 * Hack -- If rods or wands are dropped, the total maximum timeout or 
		 * charges need to be allocated between the two stacks.  If all the items 
		 * are being dropped, it makes for a neater message to leave the original 
		 * stack's pval alone. -LM-
		 */
        if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
            if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
                (*q_ptr).pval =
                    (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int;
                if amt < (*o_ptr).number as libc::c_int {
                    (*o_ptr).pval -= (*q_ptr).pval
                }
            }
        }
        /* Modify quantity */
        (*q_ptr).number = amt as byte_hack;
        /* Describe local object */
        object_desc(o_name.as_mut_ptr(), q_ptr, 1 as libc::c_int,
                    3 as libc::c_int);
        /* Message */
        if silent == 0 {
            msg_format(b"You drop %s (%c).\x00" as *const u8 as
                           *const libc::c_char, o_name.as_mut_ptr(),
                       index_to_label(item) as libc::c_int);
        }
        /* Drop it near the player */
        drop_near(q_ptr, 0 as libc::c_int, dy, dx);
        /* Modify, Describe, Optimize */
        inc_stack_size(item, -amt);
    };
}
/*
 * Combine items in the pack
 *
 * Note special handling of the "overflow" slot
 */
#[no_mangle]
pub unsafe extern "C" fn combine_pack() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    /* Combine the pack (backwards) */
    i = 23 as libc::c_int;
    while i > 0 as libc::c_int {
        /* Get the item */
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip empty items */
        if !((*o_ptr).k_idx == 0) {
            /* Scan the items above that item */
            j = 0 as libc::c_int;
            while j < i {
                /* Get the item */
                j_ptr =
                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(j as isize)
                        as *mut object_type;
                /* Skip empty items */
                if !((*j_ptr).k_idx == 0) {
                    /* Can we drop "o_ptr" onto "j_ptr"? */
                    if object_similar(j_ptr, o_ptr) != 0 {
                        /* Take note */
                        flag = 1 as libc::c_int as bool_;
                        /* Add together the item counts */
                        object_absorb(j_ptr, o_ptr);
                        /* One object is gone */
                        inven_cnt -= 1;
                        /* Slide everything down */
                        k = i;
                        while k < 23 as libc::c_int {
                            /* Structure copy */
                            (*p_ptr).inventory[k as usize] =
                                (*p_ptr).inventory[(k + 1 as libc::c_int) as
                                                       usize];
                            k += 1
                        }
                        /* Erase the "final" slot */
                        object_wipe(&mut *(*p_ptr).inventory.as_mut_ptr().offset(k
                                                                                     as
                                                                                     isize));
                        /* Window stuff */
                        (*p_ptr).window =
                            ((*p_ptr).window as libc::c_long |
                                 0x1 as libc::c_long) as u32b;
                        break ;
                    }
                }
                j += 1
            }
        }
        i -= 1
    }
    /* Message */
    if flag != 0 {
        msg_print(b"You combine some items in your pack.\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/*
 * Reorder items in the pack
 *
 * Note special handling of the "overflow" slot
 */
#[no_mangle]
pub unsafe extern "C" fn reorder_pack() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut o_value: s32b = 0;
    let mut j_value: s32b = 0;
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
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    /* Re-order the pack (forwards) */
    i = 0 as libc::c_int;
    while i < 23 as libc::c_int {
        /* Mega-Hack -- allow "proper" over-flow */
        if i == 23 as libc::c_int &&
               inven_cnt as libc::c_int == 23 as libc::c_int {
            break ;
        }
        /* Get the item */
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip empty slots */
        if !((*o_ptr).k_idx == 0) {
            /* Get the "value" of the item */
            o_value = object_value(o_ptr);
            let mut current_block_5: u64;
            /* Scan every occupied slot */
            j = 0 as libc::c_int;
            while j < 23 as libc::c_int {
                /* Get the item already there */
                j_ptr =
                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(j as isize)
                        as *mut object_type;
                /* Use empty slots */
                if (*j_ptr).k_idx == 0 { break ; }
                /* Objects sort by decreasing type */
                if (*o_ptr).tval as libc::c_int > (*j_ptr).tval as libc::c_int
                   {
                    break ;
                }
                if !(((*o_ptr).tval as libc::c_int) <
                         (*j_ptr).tval as libc::c_int) {
                    /* Non-aware (flavored) items always come last */
                    if !((*k_info.offset((*o_ptr).k_idx as isize)).aware == 0)
                       {
                        if (*k_info.offset((*j_ptr).k_idx as isize)).aware ==
                               0 {
                            break ;
                        }
                        /* Objects sort by increasing sval */
                        if ((*o_ptr).sval as libc::c_int) <
                               (*j_ptr).sval as libc::c_int {
                            break ;
                        }
                        if !((*o_ptr).sval as libc::c_int >
                                 (*j_ptr).sval as libc::c_int) {
                            /* Unidentified objects always come last */
                            if (*o_ptr).ident as libc::c_int &
                                   0x8 as libc::c_int != 0 ||
                                   (*k_info.offset((*o_ptr).k_idx as
                                                       isize)).easy_know as
                                       libc::c_int != 0 &&
                                       (*k_info.offset((*o_ptr).k_idx as
                                                           isize)).aware as
                                           libc::c_int != 0 {
                                if !((*j_ptr).ident as libc::c_int &
                                         0x8 as libc::c_int != 0 ||
                                         (*k_info.offset((*j_ptr).k_idx as
                                                             isize)).easy_know
                                             as libc::c_int != 0 &&
                                             (*k_info.offset((*j_ptr).k_idx as
                                                                 isize)).aware
                                                 as libc::c_int != 0) {
                                    break ;
                                }
                                /* Hack:  otherwise identical rods sort by
			increasing recharge time --dsb */
                                if (*o_ptr).tval as libc::c_int ==
                                       67 as libc::c_int {
                                    if (*o_ptr).timeout as libc::c_int >
                                           (*j_ptr).timeout as libc::c_int {
                                        break ;
                                    }
                                    if ((*o_ptr).timeout as libc::c_int) <
                                           (*j_ptr).timeout as libc::c_int {
                                        current_block_5 = 2968425633554183086;
                                    } else {
                                        current_block_5 =
                                            14648156034262866959;
                                    }
                                } else {
                                    current_block_5 = 14648156034262866959;
                                }
                                match current_block_5 {
                                    2968425633554183086 => { }
                                    _ => {
                                        /* Determine the "value" of the pack item */
                                        j_value = object_value(j_ptr);
                                        /* Objects sort by decreasing value */
                                        if o_value > j_value { break ; }
                                        (o_value) < j_value;
                                    }
                                }
                            }
                        }
                    }
                }
                j += 1
            }
            /* Never move down */
            if !(j >= i) {
                /* Take note */
                flag = 1 as libc::c_int as bool_;
                /* Get local object */
                q_ptr = &mut forge;
                /* Save a copy of the moving item */
                object_copy(q_ptr,
                            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as
                                                                             isize));
                /* Slide the objects */
                k = i;
                while k > j {
                    /* Slide the item */
                    object_copy(&mut *(*p_ptr).inventory.as_mut_ptr().offset(k
                                                                                 as
                                                                                 isize),
                                &mut *(*p_ptr).inventory.as_mut_ptr().offset((k
                                                                                  -
                                                                                  1
                                                                                      as
                                                                                      libc::c_int)
                                                                                 as
                                                                                 isize));
                    k -= 1
                }
                /* Insert the moving item */
                object_copy(&mut *(*p_ptr).inventory.as_mut_ptr().offset(j as
                                                                             isize),
                            q_ptr);
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long | 0x1 as libc::c_long) as
                        u32b
            }
        }
        i += 1
    }
    /* Message */
    if flag != 0 {
        msg_print(b"You reorder some items in your pack.\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/*
 * Hack -- display an object kind in the current window
 *
 * Include list of usable spells for readible books
 */
#[no_mangle]
pub unsafe extern "C" fn display_koff(mut k_idx: libc::c_int) {
    let mut y: libc::c_int = 0;
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
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Erase the window */
    y = 0 as libc::c_int;
    while y < (*Term).hgt as libc::c_int {
        /* Erase the line */
        Term_erase(0 as libc::c_int, y, 255 as libc::c_int);
        y += 1
    }
    /* No info */
    if k_idx == 0 { return }
    /* Get local object */
    q_ptr = &mut forge;
    /* Prepare the object */
    object_wipe(q_ptr);
    /* Prepare the object */
    object_prep(q_ptr, k_idx);
    /* Describe */
    object_desc_store(o_name.as_mut_ptr(), q_ptr, 0 as libc::c_int,
                      0 as libc::c_int);
    /* Mention the object name */
    Term_putstr(0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int),
                1 as libc::c_int as byte_hack, o_name.as_mut_ptr() as cptr);
}
/*
 * Let the floor carry an object
 */
#[no_mangle]
pub unsafe extern "C" fn floor_carry(mut y: libc::c_int, mut x: libc::c_int,
                                     mut j_ptr: *mut object_type) -> s16b {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut o_idx: s16b = 0;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    /* Scan objects in that grid for combination */
    this_o_idx = (*cave[y as usize].offset(x as isize)).o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Check for combination */
        if object_similar(o_ptr, j_ptr) != 0 {
            /* Combine the items */
            object_absorb(o_ptr, j_ptr);
            /* Result */
            return this_o_idx
        }
        /* Count objects */
        n += 1;
        this_o_idx = next_o_idx
    }
    /* The stack is already too large */
    if n > 23 as libc::c_int { return 0 as libc::c_int as s16b }
    /* Make an object */
    o_idx = o_pop();
    /* Success */
    if o_idx != 0 {
        let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr_0 = &mut *o_list.offset(o_idx as isize) as *mut object_type;
        /* Structure Copy */
        object_copy(o_ptr_0, j_ptr);
        /* Location */
        (*o_ptr_0).iy = y as byte_hack;
        (*o_ptr_0).ix = x as byte_hack;
        /* Forget monster */
        (*o_ptr_0).held_m_idx = 0 as libc::c_int as s16b;
        /* Build a stack */
        (*o_ptr_0).next_o_idx = (*cave[y as usize].offset(x as isize)).o_idx;
        /* Place the object */
        (*cave[y as usize].offset(x as isize)).o_idx = o_idx;
        /* Notice */
        note_spot(y, x);
        /* Redraw */
        lite_spot(y, x);
    }
    /* Result */
    return o_idx;
}
/*
 * Notice a decaying object in the pack
 */
#[no_mangle]
pub unsafe extern "C" fn pack_decay(mut item: libc::c_int) {
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*o_ptr).pval2 as isize) as *mut monster_race;
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
    let mut amt: libc::c_int = (*o_ptr).number as libc::c_int;
    let mut m_type: s16b = 0;
    let mut wt: s32b = 0;
    let mut known: byte_hack = (*o_ptr).name1;
    let mut gone: byte_hack = 1 as libc::c_int as byte_hack;
    let mut desc: [libc::c_char; 80] = [0; 80];
    /* Player notices each decaying object */
    object_desc(desc.as_mut_ptr(), o_ptr, 1 as libc::c_int, 3 as libc::c_int);
    msg_format(b"You feel %s decompose.\x00" as *const u8 as
                   *const libc::c_char, desc.as_mut_ptr());
    /* Get local object */
    i_ptr = &mut object_type_body;
    /* Obtain local object */
    object_copy(i_ptr, o_ptr);
    /* Remember what creature we were */
    m_type = (*o_ptr).pval2;
    /* and how much we weighed */
    wt = (*r_ptr).weight;
    /* Get rid of decayed object */
    inc_stack_size_ex(item, -amt, OPTIMIZE, NO_DESCRIBE);
    if (*i_ptr).tval as libc::c_int == 9 as libc::c_int {
        /* Monster must have a skull for its head to become one */
        if (*i_ptr).sval as libc::c_int == 3 as libc::c_int {
            /* Replace the head with a skull */
            object_prep(i_ptr,
                        lookup_kind(9 as libc::c_int, 4 as libc::c_int) as
                            libc::c_int);
            (*i_ptr).weight =
                wt / 60 as libc::c_int + Rand_div(wt) / 600 as libc::c_int;
            /* Stay here */
            gone = 0 as libc::c_int as byte_hack
        }
        /* Monster must have a skeleton for its corpse to become one */
        if (*i_ptr).sval as libc::c_int == 1 as libc::c_int &&
               (*r_ptr).flags3 & 0x2 as libc::c_int as libc::c_uint != 0 {
            /* Replace the corpse with a skeleton */
            object_prep(i_ptr,
                        lookup_kind(9 as libc::c_int, 2 as libc::c_int) as
                            libc::c_int);
            (*i_ptr).weight =
                wt / 4 as libc::c_int + Rand_div(wt) / 40 as libc::c_int;
            /* Stay here */
            gone = 0 as libc::c_int as byte_hack
        }
        /* Don't restore if the item is gone */
        if gone == 0 {
            (*i_ptr).number = amt as byte_hack;
            (*i_ptr).pval2 = m_type;
            /* Should become "The skull of Farmer Maggot", not "A skull" */
            if known != 0 {
                object_aware(i_ptr);
                /* Named skeletons are artifacts */
                (*i_ptr).name1 = 201 as libc::c_int as byte_hack
            }
            inven_carry(i_ptr, 1 as libc::c_int as bool_);
        }
    };
}
/*
 *	Decay an object on the floor
 */
#[no_mangle]
pub unsafe extern "C" fn floor_decay(mut item: libc::c_int) {
    let mut o_ptr: *mut object_type =
        &mut *o_list.offset(item as isize) as *mut object_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*o_ptr).pval2 as isize) as *mut monster_race;
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
    let mut amt: libc::c_int = (*o_ptr).number as libc::c_int;
    let mut m_type: s16b = 0;
    let mut wt: s32b = 0;
    let mut known: byte_hack = (*o_ptr).name1;
    /* Assume we disappear */
    let mut gone: byte_hack = 1 as libc::c_int as byte_hack;
    let mut x: byte_hack = (*o_ptr).ix;
    let mut y: byte_hack = (*o_ptr).iy;
    /* Maybe the player sees it */
    let mut visible: bool_ =
        ((*cave[(*o_ptr).iy as usize].offset((*o_ptr).ix as isize)).info as
             libc::c_int & 0x10 as libc::c_int != 0 as libc::c_int) as
            libc::c_int as bool_;
    let mut desc: [libc::c_char; 80] = [0; 80];
    if visible != 0 {
        /* Player notices each decaying object */
        object_desc(desc.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                    3 as libc::c_int);
        msg_format(b"You see %s decompose.\x00" as *const u8 as
                       *const libc::c_char, desc.as_mut_ptr());
    }
    /* Get local object */
    i_ptr = &mut object_type_body;
    /* Obtain local object */
    object_copy(i_ptr, o_ptr);
    /* Remember what creature we were */
    m_type = (*o_ptr).pval2;
    /* and how much we weighed */
    wt = (*r_ptr).weight;
    floor_item_increase(item, -amt);
    floor_item_optimize(item);
    if (*i_ptr).tval as libc::c_int == 9 as libc::c_int {
        /* Monster must have a skull for its head to become one */
        if (*i_ptr).sval as libc::c_int == 3 as libc::c_int {
            /* Replace the head with a skull */
            object_prep(i_ptr,
                        lookup_kind(9 as libc::c_int, 4 as libc::c_int) as
                            libc::c_int);
            (*i_ptr).weight =
                wt / 60 as libc::c_int + Rand_div(wt) / 600 as libc::c_int;
            /* Stay here */
            gone = 0 as libc::c_int as byte_hack
        }
        /* Monster must have a skeleton for its corpse to become one */
        if (*i_ptr).sval as libc::c_int == 1 as libc::c_int &&
               (*r_ptr).flags3 & 0x2 as libc::c_int as libc::c_uint != 0 {
            /* Replace the corpse with a skeleton */
            object_prep(i_ptr,
                        lookup_kind(9 as libc::c_int, 2 as libc::c_int) as
                            libc::c_int);
            (*i_ptr).weight =
                wt / 4 as libc::c_int + Rand_div(wt) / 40 as libc::c_int;
            /* Stay here */
            gone = 0 as libc::c_int as byte_hack
        }
        /* Don't restore if the item is gone */
        if gone == 0 {
            (*i_ptr).number = amt as byte_hack;
            (*i_ptr).pval2 = m_type;
            /* Should become "The skull of Farmer Maggot", not "A skull" */
            if known != 0 {
                object_aware(i_ptr);
                /* Named skeletons are artifacts */
                (*i_ptr).name1 = 201 as libc::c_int as byte_hack
            }
            floor_carry(y as libc::c_int, x as libc::c_int, i_ptr);
        }
    };
}
/* Return the item be it on the floor or in inven */
#[no_mangle]
pub unsafe extern "C" fn get_object(mut item: libc::c_int)
 -> *mut object_type {
    /* Get the item (in the pack) */
    if item >= 0 as libc::c_int {
        return &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
                   *mut object_type
    } else {
        /* Get the item (on the floor) */
        return &mut *o_list.offset((0 as libc::c_int - item) as isize) as
                   *mut object_type
    };
}
