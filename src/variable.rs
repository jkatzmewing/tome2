use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn text_out_to_screen(a: byte_hack, str: cptr);
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
pub struct header {
    pub info_num: u16b,
    pub name_size: u32b,
    pub text_size: u32b,
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
pub struct randart_gen_type {
    pub chance: libc::c_int,
    pub dd: libc::c_int,
    pub ds: libc::c_int,
    pub plus: libc::c_int,
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
pub struct vault_type {
    pub name: u32b,
    pub text: u32b,
    pub typ: byte_hack,
    pub rat: byte_hack,
    pub hgt: byte_hack,
    pub wid: byte_hack,
    pub lvl: s16b,
    pub dun_type: byte_hack,
    pub mon: [s16b; 10],
    pub item: [libc::c_int; 3],
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
pub struct owner_type {
    pub name: u32b,
    pub max_cost: s16b,
    pub max_inflate: byte_hack,
    pub min_inflate: byte_hack,
    pub haggle_per: byte_hack,
    pub insult_max: byte_hack,
    pub races: [[u32b; 2]; 2],
    pub classes: [[u32b; 2]; 2],
    pub costs: [s16b; 3],
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
pub struct store_action_type {
    pub name: u32b,
    pub costs: [s16b; 3],
    pub letter: libc::c_char,
    pub letter_aux: libc::c_char,
    pub action: s16b,
    pub action_restr: s16b,
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
pub struct meta_class_type {
    pub name: [libc::c_char; 80],
    pub color: byte_hack,
    pub classes: *mut s16b,
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
pub struct rune_spell {
    pub name: [libc::c_char; 30],
    pub type_0: s16b,
    pub rune2: s16b,
    pub mana: s16b,
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
pub struct hist_type {
    pub info: s32b,
    pub roll: byte_hack,
    pub chart: s16b,
    pub next: s16b,
    pub bonus: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct set_type {
    pub name: u32b,
    pub desc: u32b,
    pub num: byte_hack,
    pub num_use: byte_hack,
    pub arts: [C2RustUnnamed_3; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub present: bool_,
    pub a_idx: s16b,
    pub pval: [s16b; 6],
    pub flags1: [u32b; 6],
    pub flags2: [u32b; 6],
    pub flags3: [u32b; 6],
    pub flags4: [u32b; 6],
    pub flags5: [u32b; 6],
    pub esp: [u32b; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_comm {
    pub comm: cptr,
    pub descrip: cptr,
    pub key: s16b,
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
pub struct spell_type {
    pub name: cptr,
    pub skill_level: byte_hack,
    pub mana: byte_hack,
    pub mana_max: byte_hack,
    pub fail: s16b,
    pub level: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct school_type {
    pub name: cptr,
    pub skill: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timer_type {
    pub next: *mut timer_type,
    pub enabled: bool_,
    pub delay: s32b,
    pub countdown: s32b,
    pub callback: cptr,
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
/* File: variable.c */
/* Purpose: Angband variables */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Hack -- Link a copyright message into the executable
 */
#[no_mangle]
pub static mut copyright: [cptr; 5] =
    [b"Copyright (c) 1989 James E. Wilson, Robert A. Keoneke\x00" as *const u8
         as *const libc::c_char, b"\x00" as *const u8 as *const libc::c_char,
     b"This software may be copied and distributed for educational, research,\x00"
         as *const u8 as *const libc::c_char,
     b"and not for profit purposes provided that this copyright and statement\x00"
         as *const u8 as *const libc::c_char,
     b"are included in all such copies.\x00" as *const u8 as
         *const libc::c_char];
#[no_mangle]
pub static mut max_macrotrigger: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut macro_template: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut macro_modifier_chr: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut macro_modifier_name: [*mut libc::c_char; 12] =
    [0 as *const libc::c_char as *mut libc::c_char; 12];
#[no_mangle]
pub static mut macro_trigger_name: [*mut libc::c_char; 200] =
    [0 as *const libc::c_char as *mut libc::c_char; 200];
#[no_mangle]
pub static mut macro_trigger_keycode: [[*mut libc::c_char; 200]; 2] =
    [[0 as *const libc::c_char as *mut libc::c_char; 200]; 2];
/*
 * Executable version
 */
#[no_mangle]
pub static mut version_major: byte_hack = 0;
#[no_mangle]
pub static mut version_minor: byte_hack = 0;
#[no_mangle]
pub static mut version_patch: byte_hack = 0;
#[no_mangle]
pub static mut version_extra: byte_hack = 0 as libc::c_int as byte_hack;
/*
 * Savefile version
 */
#[no_mangle]
pub static mut sf_major: byte_hack = 0;
/* Savefile's "version_major" */
#[no_mangle]
pub static mut sf_minor: byte_hack = 0;
/* Savefile's "version_minor" */
#[no_mangle]
pub static mut sf_patch: byte_hack = 0;
/* Savefile's "version_patch" */
#[no_mangle]
pub static mut sf_extra: byte_hack = 0;
/* Savefile's "version_extra" */
#[no_mangle]
pub static mut vernum: u32b = 0;
/*
 * Savefile information
 */
#[no_mangle]
pub static mut sf_xtra: u32b = 0;
/* Operating system info */
#[no_mangle]
pub static mut sf_when: u32b = 0;
/* Time when savefile created */
#[no_mangle]
pub static mut sf_lives: u16b = 0;
/* Number of past "lives" with this file */
#[no_mangle]
pub static mut sf_saves: u16b = 0;
/* Number of "saves" during this life */
/*
 * Run-time arguments
 */
#[no_mangle]
pub static mut arg_fiddle: bool_ = 0;
/* Command arg -- Request fiddle mode */
#[no_mangle]
pub static mut arg_wizard: bool_ = 0;
/* Command arg -- Request wizard mode */
#[no_mangle]
pub static mut arg_sound: bool_ = 0;
/* Command arg -- Request special sounds */
#[no_mangle]
pub static mut arg_graphics: bool_ = 0;
/* Command arg -- Request graphics mode */
#[no_mangle]
pub static mut arg_force_original: bool_ = 0;
/* Command arg -- Request original keyset */
#[no_mangle]
pub static mut arg_force_roguelike: bool_ = 0;
/* Command arg -- Request roguelike keyset */
#[no_mangle]
pub static mut arg_bigtile: bool_ = 0 as libc::c_int as bool_;
/* Command arg -- Request big tile mode */
/*
 * Various things
 */
#[no_mangle]
pub static mut character_generated: bool_ = 0;
/* The character exists */
#[no_mangle]
pub static mut character_dungeon: bool_ = 0;
/* The character has a dungeon */
#[no_mangle]
pub static mut character_loaded: bool_ = 0;
/* The character was loaded from a savefile */
#[no_mangle]
pub static mut character_saved: bool_ = 0;
/* The character was just saved to a savefile */
#[no_mangle]
pub static mut character_icky: bool_ = 0;
/* The game is in an icky full screen mode */
#[no_mangle]
pub static mut character_xtra: bool_ = 0;
/* The game is in an icky startup mode */
#[no_mangle]
pub static mut seed_flavor: u32b = 0;
/* Hack -- consistent object colors */
#[no_mangle]
pub static mut command_cmd: s16b = 0;
/* Current "Angband Command" */
#[no_mangle]
pub static mut command_arg: s16b = 0;
/* Gives argument of current command */
#[no_mangle]
pub static mut command_rep: s16b = 0;
/* Gives repetition of current command */
#[no_mangle]
pub static mut command_dir: s16b = 0;
/* Gives direction of current command */
#[no_mangle]
pub static mut command_wrk: s16b = 0;
/* See "cmd1.c" */
#[no_mangle]
pub static mut command_new: s16b = 0;
/* Command chaining from inven/equip view */
#[no_mangle]
pub static mut energy_use: s32b = 0;
/* Energy use this turn */
#[no_mangle]
pub static mut create_up_stair: bool_ = 0;
/* Auto-create "up stairs" */
#[no_mangle]
pub static mut create_down_stair: bool_ = 0;
/* Auto-create "down stairs" */
#[no_mangle]
pub static mut create_up_shaft: bool_ = 0;
/* Auto-create "up shaft" */
#[no_mangle]
pub static mut create_down_shaft: bool_ = 0;
/* Auto-create "down shaft" */
#[no_mangle]
pub static mut msg_flag: bool_ = 0;
/* Used in msg_print() for "buffering" */
#[no_mangle]
pub static mut alive: bool_ = 0;
/* True if game is running */
#[no_mangle]
pub static mut death: bool_ = 0;
/* True if player has died */
#[no_mangle]
pub static mut running: s16b = 0;
/* Current counter for running, if any */
#[no_mangle]
pub static mut resting: s16b = 0;
/* Current counter for resting, if any */
#[no_mangle]
pub static mut cur_hgt: s16b = 0;
/* Current dungeon height */
#[no_mangle]
pub static mut cur_wid: s16b = 0;
/* Current dungeon width */
#[no_mangle]
pub static mut dun_level: s16b = 0;
/* Current dungeon level */
#[no_mangle]
pub static mut old_dun_level: s16b = 0;
/* Old dungeon level */
#[no_mangle]
pub static mut num_repro: s16b = 0;
/* Current reproducer count */
#[no_mangle]
pub static mut object_level: s16b = 0;
/* Current object creation level */
#[no_mangle]
pub static mut monster_level: s16b = 0;
/* Current monster creation level */
#[no_mangle]
pub static mut turn: s32b = 0;
/* Current game turn */
#[no_mangle]
pub static mut old_turn: s32b = 0;
/* Turn when level began (feelings) */
#[no_mangle]
pub static mut wizard: bool_ = 0;
/* Is the player currently in Wizard mode? */
#[no_mangle]
pub static mut use_sound: bool_ = 0;
/* The "sound" mode is enabled */
#[no_mangle]
pub static mut use_graphics: bool_ = 0;
/* The "graphics" mode is enabled */
#[no_mangle]
pub static mut use_bigtile: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub static mut graphics_mode: byte_hack = 0;
/* Current graphics mode */
#[no_mangle]
pub static mut total_winner: u16b = 0;
/* Semi-Hack -- Game has been won */
#[no_mangle]
pub static mut has_won: u16b = 0;
/* Semi-Hack -- Game has been won */
#[no_mangle]
pub static mut noscore: u16b = 0;
/* Track various "cheating" conditions */
#[no_mangle]
pub static mut signal_count: s16b = 0;
/* Hack -- Count interupts */
#[no_mangle]
pub static mut inkey_base: bool_ = 0;
/* See the "inkey()" function */
#[no_mangle]
pub static mut inkey_xtra: bool_ = 0;
/* See the "inkey()" function */
#[no_mangle]
pub static mut inkey_scan: bool_ = 0;
/* See the "inkey()" function */
#[no_mangle]
pub static mut inkey_flag: bool_ = 0;
/* See the "inkey()" function */
#[no_mangle]
pub static mut coin_type: s16b = 0;
/* Hack -- force coin type */
#[no_mangle]
pub static mut opening_chest: bool_ = 0;
/* Hack -- prevent chest generation */
#[no_mangle]
pub static mut shimmer_monsters: bool_ = 0;
/* Hack -- optimize multi-hued monsters */
#[no_mangle]
pub static mut shimmer_objects: bool_ = 0;
/* Hack -- optimize multi-hued objects */
#[no_mangle]
pub static mut repair_monsters: bool_ = 0;
/* Hack -- optimize detect monsters */
#[no_mangle]
pub static mut repair_objects: bool_ = 0;
/* Hack -- optimize detect objects */
#[no_mangle]
pub static mut inven_nxt: s16b = 0;
/* Hack -- unused */
#[no_mangle]
pub static mut hack_mind: bool_ = 0;
#[no_mangle]
pub static mut hack_corruption: bool_ = 0;
#[no_mangle]
pub static mut artifact_bias: libc::c_int = 0;
#[no_mangle]
pub static mut is_autosave: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub static mut inven_cnt: s16b = 0;
/* Number of items in inventory */
#[no_mangle]
pub static mut equip_cnt: s16b = 0;
/* Number of items in equipment */
#[no_mangle]
pub static mut o_max: s16b = 1 as libc::c_int as s16b;
/* Number of allocated objects */
#[no_mangle]
pub static mut o_cnt: s16b = 0 as libc::c_int as s16b;
/* Number of live objects */
#[no_mangle]
pub static mut m_max: s16b = 1 as libc::c_int as s16b;
/* Number of allocated monsters */
#[no_mangle]
pub static mut m_cnt: s16b = 0 as libc::c_int as s16b;
/* Number of live monsters */
#[no_mangle]
pub static mut hack_m_idx: s16b = 0 as libc::c_int as s16b;
/* Hack -- see "process_monsters()" */
#[no_mangle]
pub static mut hack_m_idx_ii: s16b = 0 as libc::c_int as s16b;
#[no_mangle]
pub static mut multi_rew: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub static mut summon_kin_type: libc::c_char = 0;
/* Hack, by Julian Lighton: summon 'relatives' */
#[no_mangle]
pub static mut total_friends: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut total_friend_levels: s32b = 0 as libc::c_int;
#[no_mangle]
pub static mut leaving_quest: libc::c_int = 0 as libc::c_int;
/*
 * Hack - the destination file for text_out_to_file.
 */
#[no_mangle]
pub static mut text_out_file: *mut FILE = 0 as *const FILE as *mut FILE;
/*
 * Hack -- function hook to output (colored) text to the
 * screen or to a file.
 */
#[no_mangle]
pub static mut text_out_hook:
           Option<unsafe extern "C" fn(_: byte_hack, _: cptr) -> ()> =
    unsafe {
        Some(text_out_to_screen as
                 unsafe extern "C" fn(_: byte_hack, _: cptr) -> ())
    };
/*
 * Hack -- Where to wrap the text when using text_out().  Use the default
 * value (for example the screen width) when 'text_out_wrap' is 0.
 */
#[no_mangle]
pub static mut text_out_wrap: libc::c_int = 0 as libc::c_int;
/*
 * Hack -- Indentation for the text when using text_out().
 */
#[no_mangle]
pub static mut text_out_indent: libc::c_int = 0 as libc::c_int;
/*
 * The "highscore" file descriptor, if available.
 */
#[no_mangle]
pub static mut highscore_fd: libc::c_int = -(1 as libc::c_int);
/*
 * Software options (set via the '=' command).  See "tables.c"
 */
/* Option Set 1 -- User Interface */
#[no_mangle]
pub static mut rogue_like_commands: bool_ = 0;
/* Rogue-like commands */
#[no_mangle]
pub static mut quick_messages: bool_ = 0;
/* Activate quick messages */
#[no_mangle]
pub static mut other_query_flag: bool_ = 0;
/* Prompt for various information */
#[no_mangle]
pub static mut carry_query_flag: bool_ = 0;
/* Prompt before picking things up */
#[no_mangle]
pub static mut use_old_target: bool_ = 0;
/* Use old target by default */
#[no_mangle]
pub static mut always_pickup: bool_ = 0;
/* Pick things up by default */
#[no_mangle]
pub static mut prompt_pickup_heavy: bool_ = 0;
/* Don't pick up the corpses */
#[no_mangle]
pub static mut always_repeat: bool_ = 0;
/* Repeat obvious commands */
#[no_mangle]
pub static mut depth_in_feet: bool_ = 0;
/* Show dungeon level in feet */
#[no_mangle]
pub static mut stack_force_notes: bool_ = 0;
/* Merge inscriptions when stacking */
#[no_mangle]
pub static mut stack_force_costs: bool_ = 0;
/* Merge discounts when stacking */
#[no_mangle]
pub static mut show_labels: bool_ = 0;
/* Show labels in object listings */
#[no_mangle]
pub static mut show_weights: bool_ = 0;
/* Show weights in object listings */
#[no_mangle]
pub static mut show_choices: bool_ = 0;
/* Show choices in certain sub-windows */
#[no_mangle]
pub static mut show_details: bool_ = 0;
/* Show details in certain sub-windows */
#[no_mangle]
pub static mut ring_bell: bool_ = 0;
/* Ring the bell (on errors, etc) */
#[no_mangle]
pub static mut show_inven_graph: bool_ = 0;
/* Show graphics in inventory */
#[no_mangle]
pub static mut show_equip_graph: bool_ = 0;
/* Show graphics in equip list */
#[no_mangle]
pub static mut show_store_graph: bool_ = 0;
/* Show graphics in store */
/* Option Set 2 -- Disturbance */
#[no_mangle]
pub static mut find_ignore_stairs: bool_ = 0;
/* Run past stairs */
#[no_mangle]
pub static mut find_ignore_doors: bool_ = 0;
/* Run through open doors */
#[no_mangle]
pub static mut find_cut: bool_ = 0;
/* Run past known corners */
#[no_mangle]
pub static mut find_examine: bool_ = 0;
/* Run into potential corners */
#[no_mangle]
pub static mut disturb_move: bool_ = 0;
/* Disturb whenever any monster moves */
#[no_mangle]
pub static mut disturb_near: bool_ = 0;
/* Disturb whenever viewable monster moves */
#[no_mangle]
pub static mut disturb_panel: bool_ = 0;
/* Disturb whenever map panel changes */
#[no_mangle]
pub static mut disturb_detect: bool_ = 0;
/* Disturb whenever leaving trap-detected area */
#[no_mangle]
pub static mut disturb_state: bool_ = 0;
/* Disturn whenever player state changes */
#[no_mangle]
pub static mut disturb_minor: bool_ = 0;
/* Disturb whenever boring things happen */
#[no_mangle]
pub static mut disturb_other: bool_ = 0;
/* Disturb whenever various things happen */
#[no_mangle]
pub static mut alert_hitpoint: bool_ = 0;
/* Alert user to critical hitpoints */
#[no_mangle]
pub static mut alert_failure: bool_ = 0;
/* Alert user to various failures */
#[no_mangle]
pub static mut last_words: bool_ = 0;
/* Get last words upon dying */
#[no_mangle]
pub static mut speak_unique: bool_ = 0;
/* Speaking uniques + shopkeepers */
#[no_mangle]
pub static mut small_levels: bool_ = 0;
/* Allow unusually small dungeon levels */
#[no_mangle]
pub static mut empty_levels: bool_ = 0;
/* Allow empty 'arena' levels */
#[no_mangle]
pub static mut always_small_level: bool_ = 0;
/* Small levels */
#[no_mangle]
pub static mut player_symbols: bool_ = 0;
/* Use varying symbols for the player char */
#[no_mangle]
pub static mut plain_descriptions: bool_ = 0;
/* Plain object descriptions */
#[no_mangle]
pub static mut stupid_monsters: bool_ = 0;
/* Monsters use old AI */
#[no_mangle]
pub static mut auto_destroy: bool_ = 0;
/* Known worthless items are destroyed without confirmation */
#[no_mangle]
pub static mut confirm_stairs: bool_ = 0;
/* Prompt before staircases... */
#[no_mangle]
pub static mut wear_confirm: bool_ = 0;
/* Confirm before putting on known cursed items */
#[no_mangle]
pub static mut disturb_pets: bool_ = 0;
/* Pets moving nearby disturb us */
/* Option Set 3 -- Game-Play */
#[no_mangle]
pub static mut auto_haggle: bool_ = 0;
/* Auto-haggle in stores */
#[no_mangle]
pub static mut auto_scum: bool_ = 0;
/* Auto-scum for good levels */
#[no_mangle]
pub static mut stack_allow_items: bool_ = 0;
/* Allow weapons and armor to stack */
#[no_mangle]
pub static mut stack_allow_wands: bool_ = 0;
/* Allow wands/staffs/rods to stack */
#[no_mangle]
pub static mut expand_look: bool_ = 0;
/* Expand the power of the look command */
#[no_mangle]
pub static mut expand_list: bool_ = 0;
/* Expand the power of the list commands */
#[no_mangle]
pub static mut view_perma_grids: bool_ = 0;
/* Map remembers all perma-lit grids */
#[no_mangle]
pub static mut view_torch_grids: bool_ = 0;
/* Map remembers all torch-lit grids */
#[no_mangle]
pub static mut monster_lite: bool_ = 0;
/* Allow some monsters to carry light */
#[no_mangle]
pub static mut dungeon_align: bool_ = 0;
/* Generate dungeons with aligned rooms */
#[no_mangle]
pub static mut dungeon_stair: bool_ = 0;
/* Generate dungeons with connected stairs */
#[no_mangle]
pub static mut flow_by_sound: bool_ = 0;
/* Monsters track new player location */
#[no_mangle]
pub static mut track_follow: bool_ = 0;
/* Monsters follow the player */
#[no_mangle]
pub static mut track_target: bool_ = 0;
/* Monsters target the player */
#[no_mangle]
pub static mut smart_learn: bool_ = 0;
/* Monsters learn from their mistakes */
#[no_mangle]
pub static mut smart_cheat: bool_ = 0;
/* Monsters exploit player weaknesses */
/* Option Set 4 -- Efficiency */
#[no_mangle]
pub static mut view_reduce_lite: bool_ = 0;
/* Reduce lite-radius when running */
#[no_mangle]
pub static mut view_reduce_view: bool_ = 0;
/* Reduce view-radius in town */
#[no_mangle]
pub static mut avoid_abort: bool_ = 0;
/* Avoid checking for user abort */
#[no_mangle]
pub static mut avoid_shimmer: bool_ = 0;
/* Avoid processing extra shimmering */
#[no_mangle]
pub static mut avoid_other: bool_ = 0;
/* Avoid processing special colors */
#[no_mangle]
pub static mut flush_failure: bool_ = 0;
/* Flush input on any failure */
#[no_mangle]
pub static mut flush_disturb: bool_ = 0;
/* Flush input on disturbance */
#[no_mangle]
pub static mut flush_command: bool_ = 0;
/* Flush input before every command */
#[no_mangle]
pub static mut fresh_before: bool_ = 0;
/* Flush output before normal commands */
#[no_mangle]
pub static mut fresh_after: bool_ = 0;
/* Flush output after normal commands */
#[no_mangle]
pub static mut fresh_message: bool_ = 0;
/* Flush output after all messages */
#[no_mangle]
pub static mut hilite_player: bool_ = 0;
/* Hilite the player with the cursor */
#[no_mangle]
pub static mut view_yellow_lite: bool_ = 0;
/* Use special colors for torch-lit grids */
#[no_mangle]
pub static mut view_bright_lite: bool_ = 0;
/* Use special colors for 'viewable' grids */
#[no_mangle]
pub static mut view_granite_lite: bool_ = 0;
/* Use special colors for wall grids (slow) */
#[no_mangle]
pub static mut view_special_lite: bool_ = 0;
/* Use special colors for floor grids (slow) */
/* Option set 5 -- Testing */
#[no_mangle]
pub static mut testing_stack: bool_ = 0;
/* Test the stacking code */
#[no_mangle]
pub static mut testing_carry: bool_ = 0;
/* Test the carrying code */
/* Cheating options */
#[no_mangle]
pub static mut cheat_peek: bool_ = 0;
/* Peek into object creation */
#[no_mangle]
pub static mut cheat_hear: bool_ = 0;
/* Peek into monster creation */
#[no_mangle]
pub static mut cheat_room: bool_ = 0;
/* Peek into dungeon creation */
#[no_mangle]
pub static mut cheat_xtra: bool_ = 0;
/* Peek into something else */
#[no_mangle]
pub static mut cheat_know: bool_ = 0;
/* Know complete monster info */
#[no_mangle]
pub static mut cheat_live: bool_ = 0;
/* Allow player to avoid death */
/* Special options */
#[no_mangle]
pub static mut hitpoint_warn: byte_hack = 0;
/* Hitpoint warning (0 to 9) */
#[no_mangle]
pub static mut delay_factor: byte_hack = 0;
/* Delay factor (0 to 9) */
#[no_mangle]
pub static mut autosave_l: bool_ = 0;
/* Autosave before entering new levels */
#[no_mangle]
pub static mut autosave_t: bool_ = 0;
/* Timed autosave */
#[no_mangle]
pub static mut autosave_freq: s16b = 0;
/* Autosave frequency */
/*
 * Dungeon variables
 */
#[no_mangle]
pub static mut feeling: s16b = 0;
/* Most recent feeling */
#[no_mangle]
pub static mut rating: s16b = 0;
/* Level's current rating */
#[no_mangle]
pub static mut good_item_flag: bool_ = 0;
/* True if "Artifact" on this level */
#[no_mangle]
pub static mut closing_flag: bool_ = 0;
/* Dungeon is closing */
/*
 * Dungeon size info
 */
#[no_mangle]
pub static mut max_panel_rows: s16b = 0;
#[no_mangle]
pub static mut max_panel_cols: s16b = 0;
#[no_mangle]
pub static mut panel_row_min: s16b = 0;
#[no_mangle]
pub static mut panel_row_max: s16b = 0;
#[no_mangle]
pub static mut panel_col_min: s16b = 0;
#[no_mangle]
pub static mut panel_col_max: s16b = 0;
#[no_mangle]
pub static mut panel_col_prt: s16b = 0;
#[no_mangle]
pub static mut panel_row_prt: s16b = 0;
/*
 * Dungeon graphics info
 * Why the first two are byte and the rest s16b???
 */
#[no_mangle]
pub static mut feat_wall_outer: byte_hack = 0x3a as libc::c_int as byte_hack;
/* Outer wall of rooms */
#[no_mangle]
pub static mut feat_wall_inner: byte_hack = 0x39 as libc::c_int as byte_hack;
/* Inner wall of rooms */
#[no_mangle]
pub static mut floor_type: [s16b; 100] = [0; 100];
/* Dungeon floor */
#[no_mangle]
pub static mut fill_type: [s16b; 100] = [0; 100];
/* Dungeon filler */
/*
 * Targetting variables
 */
#[no_mangle]
pub static mut target_who: s16b = 0;
#[no_mangle]
pub static mut target_col: s16b = 0;
#[no_mangle]
pub static mut target_row: s16b = 0;
/*
 * Health bar variable -DRS-
 */
#[no_mangle]
pub static mut health_who: s16b = 0;
/*
 * Monster race to track
 */
#[no_mangle]
pub static mut monster_race_idx: s16b = 0;
#[no_mangle]
pub static mut monster_ego_idx: s16b = 0;
/*
 * Object to track
 */
#[no_mangle]
pub static mut tracked_object: *mut object_type =
    0 as *const object_type as *mut object_type;
/*
 * User info
 */
#[no_mangle]
pub static mut player_uid: libc::c_int = 0;
/*
 * Current player's character name
 */
#[no_mangle]
pub static mut player_name: [libc::c_char; 32] = [0; 32];
/*
 * Stripped version of "player_name"
 */
#[no_mangle]
pub static mut player_base: [libc::c_char; 32] = [0; 32];
/*
 * What killed the player
 */
#[no_mangle]
pub static mut died_from: [libc::c_char; 80] = [0; 80];
/*
 * Hack -- Textual "history" for the Player
 */
#[no_mangle]
pub static mut history: [[libc::c_char; 60]; 4] = [[0; 60]; 4];
/*
 * Buffer to hold the current savefile name
 */
#[no_mangle]
pub static mut savefile: [libc::c_char; 1024] = [0; 1024];
/*
 * Array of grids lit by player lite (see "cave.c")
 */
#[no_mangle]
pub static mut lite_n: s16b = 0;
#[no_mangle]
pub static mut lite_y: [s16b; 1536] = [0; 1536];
#[no_mangle]
pub static mut lite_x: [s16b; 1536] = [0; 1536];
/*
 * Array of grids viewable to the player (see "cave.c")
 */
#[no_mangle]
pub static mut view_n: s16b = 0;
#[no_mangle]
pub static mut view_y: [byte_hack; 1536] = [0; 1536];
#[no_mangle]
pub static mut view_x: [byte_hack; 1536] = [0; 1536];
/*
 * Array of grids for use by various functions (see "cave.c")
 */
#[no_mangle]
pub static mut temp_n: s16b = 0;
#[no_mangle]
pub static mut temp_y: [byte_hack; 16384] = [0; 16384];
#[no_mangle]
pub static mut temp_x: [byte_hack; 16384] = [0; 16384];
/*
 * Number of active macros.
 */
#[no_mangle]
pub static mut macro__num: s16b = 0;
/*
 * Array of macro patterns [MACRO_MAX]
 */
#[no_mangle]
pub static mut macro__pat: *mut cptr = 0 as *const cptr as *mut cptr;
/*
 * Array of macro actions [MACRO_MAX]
 */
#[no_mangle]
pub static mut macro__act: *mut cptr = 0 as *const cptr as *mut cptr;
/*
 * Array of macro types [MACRO_MAX]
 */
#[no_mangle]
pub static mut macro__cmd: *mut bool_ = 0 as *const bool_ as *mut bool_;
/*
 * Current macro action [1024]
 */
#[no_mangle]
pub static mut macro__buf: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The number of quarks
 */
#[no_mangle]
pub static mut quark__num: s16b = 0;
/*
 * The pointers to the quarks [QUARK_MAX]
 */
#[no_mangle]
pub static mut quark__str: *mut cptr = 0 as *const cptr as *mut cptr;
/*
 * The next "free" index to use
 */
#[no_mangle]
pub static mut message__next: u16b = 0;
/*
 * The index of the oldest message (none yet)
 */
#[no_mangle]
pub static mut message__last: u16b = 0;
/*
 * The next "free" offset
 */
#[no_mangle]
pub static mut message__head: u16b = 0;
/*
 * The offset to the oldest used char (none yet)
 */
#[no_mangle]
pub static mut message__tail: u16b = 0;
/*
 * The array of offsets, by index [MESSAGE_MAX]
 */
#[no_mangle]
pub static mut message__ptr: *mut u16b = 0 as *const u16b as *mut u16b;
/*
 * The array of colors, by index [MESSAGE_MAX]
 */
#[no_mangle]
pub static mut message__color: *mut byte_hack =
    0 as *const byte_hack as *mut byte_hack;
/*
 * The array of type, by index [MESSAGE_MAX]
 */
#[no_mangle]
pub static mut message__type: *mut byte_hack =
    0 as *const byte_hack as *mut byte_hack;
/*
 * The array of message counts, by index [MESSAGE_MAX]
 */
#[no_mangle]
pub static mut message__count: *mut u16b = 0 as *const u16b as *mut u16b;
/*
 * The array of chars, by offset [MESSAGE_BUF]
 */
#[no_mangle]
pub static mut message__buf: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The array of normal options
 */
#[no_mangle]
pub static mut option_flag: [u32b; 8] = [0; 8];
#[no_mangle]
pub static mut option_mask: [u32b; 8] = [0; 8];
/*
 * The array of window options
 */
#[no_mangle]
pub static mut window_flag: [u32b; 8] = [0; 8];
#[no_mangle]
pub static mut window_mask: [u32b; 8] = [0; 8];
/*
 * The array of window pointers
 */
#[no_mangle]
pub static mut angband_term: [*mut term; 8] =
    [0 as *const term as *mut term; 8];
/*
 * Standard window names
 */
#[no_mangle]
pub static mut angband_term_name: [[libc::c_char; 80]; 8] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 80],
                                  &mut [libc::c_char; 80]>(b"ToME\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 80],
                                  &mut [libc::c_char; 80]>(b"Mirror\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 80],
                                  &mut [libc::c_char; 80]>(b"Recall\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 80],
                                  &mut [libc::c_char; 80]>(b"Choice\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 80],
                                  &mut [libc::c_char; 80]>(b"Term-4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 80],
                                  &mut [libc::c_char; 80]>(b"Term-5\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 80],
                                  &mut [libc::c_char; 80]>(b"Term-6\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 80],
                                  &mut [libc::c_char; 80]>(b"Term-7\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")]
    };
/*
 * Global table of color definitions
 */
#[no_mangle]
pub static mut angband_color_table: [[byte_hack; 4]; 256] =
    [[0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
      0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0xff as libc::c_int as byte_hack,
      0xff as libc::c_int as byte_hack, 0xff as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0x80 as libc::c_int as byte_hack,
      0x80 as libc::c_int as byte_hack, 0x80 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0xff as libc::c_int as byte_hack,
      0x80 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0xc0 as libc::c_int as byte_hack,
      0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
      0x80 as libc::c_int as byte_hack, 0x40 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
      0 as libc::c_int as byte_hack, 0xff as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0x80 as libc::c_int as byte_hack,
      0x40 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0x40 as libc::c_int as byte_hack,
      0x40 as libc::c_int as byte_hack, 0x40 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0xc0 as libc::c_int as byte_hack,
      0xc0 as libc::c_int as byte_hack, 0xc0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0xff as libc::c_int as byte_hack,
      0 as libc::c_int as byte_hack, 0xff as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0xff as libc::c_int as byte_hack,
      0xff as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0xff as libc::c_int as byte_hack,
      0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
      0xff as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
      0xff as libc::c_int as byte_hack, 0xff as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0xc0 as libc::c_int as byte_hack,
      0x80 as libc::c_int as byte_hack, 0x40 as libc::c_int as byte_hack],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4],
     [0; 4], [0; 4], [0; 4], [0; 4], [0; 4], [0; 4]];
/*
 * Standard sound names
 */
#[no_mangle]
pub static mut angband_sound_name: [[libc::c_char; 16]; 65] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"hit\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"miss\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"flee\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"drop\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"kill\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"level\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"death\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"study\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"teleport\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"shoot\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"quaff\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"zap\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"walk\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"tpother\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"hitwall\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"eat\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"store1\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"store2\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"store3\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"store4\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"dig\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"opendoor\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"shutdoor\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"tplevel\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"scroll\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"buy\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"sell\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"warn\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"rocket\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"n_kill\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"u_kill\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"quest\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"heal\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"x_heal\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"bite\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"claw\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"m_spell\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"summon\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"breath\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"ball\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"m_heal\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"atkspell\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"evil\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"touch\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"sting\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"crush\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"slime\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"wail\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"winner\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"fire\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"acid\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"elec\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"cold\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"illegal\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"fail\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"wakeup\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"invuln\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"fall\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"pain\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"destitem\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"moan\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"show\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"unused\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 16],
                                  &mut [libc::c_char; 16]>(b"explode\x00\x00\x00\x00\x00\x00\x00\x00\x00")]
    };
/*
 * The array of "cave grids" [MAX_WID][MAX_HGT].
 * Not completely allocated, that would be inefficient
 * Not completely hardcoded, that would overflow memory
 */
#[no_mangle]
pub static mut cave: [*mut cave_type; 66] =
    [0 as *const cave_type as *mut cave_type; 66];
/*
 * The array of dungeon items [max_o_idx]
 */
#[no_mangle]
pub static mut o_list: *mut object_type =
    0 as *const object_type as *mut object_type;
/*
 * The array of dungeon monsters [max_m_idx]
 */
#[no_mangle]
pub static mut m_list: *mut monster_type =
    0 as *const monster_type as *mut monster_type;
/*
 * The array of to keep monsters [max_m_idx]
 */
#[no_mangle]
pub static mut km_list: *mut monster_type =
    0 as *const monster_type as *mut monster_type;
/*
 * Maximum number of towns
 */
#[no_mangle]
pub static mut max_towns: u16b = 0;
#[no_mangle]
pub static mut max_real_towns: u16b = 0;
/*
 * The towns [max_towns]
 */
#[no_mangle]
pub static mut town_info: *mut town_type =
    0 as *const town_type as *mut town_type;
/*
 * The size of "alloc_kind_table" (at most max_k_idx * 4)
 */
#[no_mangle]
pub static mut alloc_kind_size: s16b = 0;
/*
 * The entries in the "kind allocator table"
 */
#[no_mangle]
pub static mut alloc_kind_table: *mut alloc_entry =
    0 as *const alloc_entry as *mut alloc_entry;
/*
 * The flag to tell if alloc_kind_table contains valid entries
 * for normal (i.e. kind_is_legal) object allocation
 */
#[no_mangle]
pub static mut alloc_kind_table_valid: bool_ = 0 as libc::c_int as bool_;
/*
 * The size of "alloc_race_table" (at most max_r_idx)
 */
#[no_mangle]
pub static mut alloc_race_size: s16b = 0;
/*
 * The entries in the "race allocator table"
 */
#[no_mangle]
pub static mut alloc_race_table: *mut alloc_entry =
    0 as *const alloc_entry as *mut alloc_entry;
/*
 * Specify attr/char pairs for visual special effects
 * Be sure to use "index & 0x7F" to avoid illegal access
 */
#[no_mangle]
pub static mut misc_to_attr: [byte_hack; 256] = [0; 256];
#[no_mangle]
pub static mut misc_to_char: [libc::c_char; 256] = [0; 256];
/*
 * Specify attr/char pairs for inventory items (by tval)
 * Be sure to use "index & 0x7F" to avoid illegal access
 */
#[no_mangle]
pub static mut tval_to_attr: [byte_hack; 128] = [0; 128];
#[no_mangle]
pub static mut tval_to_char: [libc::c_char; 128] = [0; 128];
/*
 * Keymaps for each "mode" associated with each keypress.
 */
#[no_mangle]
pub static mut keymap_act: [[cptr; 256]; 2] =
    [[0 as *const libc::c_char; 256]; 2];
/* ** Player information ***/
/*
 * Static player info record
 */
#[no_mangle]
pub static mut p_body: player_type =
    player_type{lives: 0,
                oldpy: 0,
                oldpx: 0,
                py: 0,
                px: 0,
                psex: 0,
                prace: 0,
                pracem: 0,
                pclass: 0,
                pspec: 0,
                mimic_form: 0,
                mimic_level: 0,
                oops: 0,
                inventory:
                    [object_type{k_idx: 0,
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
                                 found_aux4: 0,}; 52],
                hitdie: 0,
                expfact: 0,
                maximize: 0,
                preserve: 0,
                special: 0,
                allow_one_death: 0,
                age: 0,
                ht: 0,
                wt: 0,
                sc: 0,
                au: 0,
                max_exp: 0,
                exp: 0,
                exp_frac: 0,
                lev: 0,
                town_num: 0,
                arena_number: 0,
                inside_arena: 0,
                inside_quest: 0,
                exit_bldg: 0,
                wilderness_x: 0,
                wilderness_y: 0,
                wild_mode: 0,
                old_wild_mode: 0,
                mhp: 0,
                chp: 0,
                chp_frac: 0,
                hp_mod: 0,
                msp: 0,
                csp: 0,
                csp_frac: 0,
                msane: 0,
                csane: 0,
                csane_frac: 0,
                grace: 0,
                pgod: 0,
                praying: 0,
                melkor_sacrifice: 0,
                max_plv: 0,
                stat_max: [0; 6],
                stat_cur: [0; 6],
                luck_cur: 0,
                luck_max: 0,
                luck_base: 0,
                speed_factor: 0,
                fast: 0,
                lightspeed: 0,
                slow: 0,
                blind: 0,
                paralyzed: 0,
                confused: 0,
                afraid: 0,
                image: 0,
                poisoned: 0,
                cut: 0,
                stun: 0,
                protevil: 0,
                protgood: 0,
                protundead: 0,
                invuln: 0,
                hero: 0,
                shero: 0,
                shield: 0,
                shield_power: 0,
                shield_opt: 0,
                shield_power_opt: 0,
                shield_power_opt2: 0,
                blessed: 0,
                tim_invis: 0,
                tim_infra: 0,
                oppose_acid: 0,
                oppose_elec: 0,
                oppose_fire: 0,
                oppose_cold: 0,
                oppose_pois: 0,
                oppose_ld: 0,
                oppose_cc: 0,
                oppose_ss: 0,
                oppose_nex: 0,
                rush: 0,
                tim_esp: 0,
                tim_wraith: 0,
                tim_ffall: 0,
                tim_fly: 0,
                tim_fire_aura: 0,
                tim_poison: 0,
                tim_thunder: 0,
                tim_thunder_p1: 0,
                tim_thunder_p2: 0,
                tim_project: 0,
                tim_project_dam: 0,
                tim_project_gf: 0,
                tim_project_rad: 0,
                tim_project_flag: 0,
                tim_roots: 0,
                tim_roots_ac: 0,
                tim_roots_dam: 0,
                resist_magic: 0,
                tim_invisible: 0,
                tim_inv_pow: 0,
                tim_mimic: 0,
                tim_lite: 0,
                tim_regen: 0,
                tim_regen_pow: 0,
                holy: 0,
                walk_water: 0,
                tim_mental_barrier: 0,
                strike: 0,
                meditation: 0,
                tim_reflect: 0,
                tim_res_time: 0,
                tim_deadly: 0,
                prob_travel: 0,
                disrupt_shield: 0,
                parasite: 0,
                parasite_r_idx: 0,
                loan: 0,
                loan_time: 0,
                absorb_soul: 0,
                tim_magic_breath: 0,
                tim_water_breath: 0,
                immov_cntr: 0,
                chaos_patron: 0,
                recall_dungeon: 0,
                word_recall: 0,
                energy: 0,
                food: 0,
                confusing: 0,
                searching: 0,
                new_spells: 0,
                old_spells: 0,
                xtra_spells: 0,
                old_cumber_armor: 0,
                old_cumber_glove: 0,
                old_heavy_wield: 0,
                old_heavy_shoot: 0,
                old_icky_wield: 0,
                old_lite: 0,
                old_view: 0,
                old_food_aux: 0,
                cumber_armor: 0,
                cumber_glove: 0,
                heavy_wield: 0,
                heavy_shoot: 0,
                icky_wield: 0,
                immovable: 0,
                cur_lite: 0,
                notice: 0,
                update: 0,
                redraw: 0,
                window: 0,
                stat_use: [0; 6],
                stat_top: [0; 6],
                stat_add: [0; 6],
                stat_ind: [0; 6],
                stat_cnt: [0; 6],
                stat_los: [0; 6],
                immune_acid: 0,
                immune_elec: 0,
                immune_fire: 0,
                immune_cold: 0,
                immune_neth: 0,
                resist_acid: 0,
                resist_elec: 0,
                resist_fire: 0,
                resist_cold: 0,
                resist_pois: 0,
                resist_conf: 0,
                resist_sound: 0,
                resist_lite: 0,
                resist_dark: 0,
                resist_chaos: 0,
                resist_disen: 0,
                resist_shard: 0,
                resist_nexus: 0,
                resist_blind: 0,
                resist_neth: 0,
                resist_fear: 0,
                resist_continuum: 0,
                sensible_fire: 0,
                sensible_lite: 0,
                reflect: 0,
                sh_fire: 0,
                sh_elec: 0,
                wraith_form: 0,
                anti_magic: 0,
                anti_tele: 0,
                sustain_str: 0,
                sustain_int: 0,
                sustain_wis: 0,
                sustain_dex: 0,
                sustain_con: 0,
                sustain_chr: 0,
                aggravate: 0,
                teleport: 0,
                exp_drain: 0,
                drain_mana: 0,
                drain_life: 0,
                magical_breath: 0,
                water_breath: 0,
                climb: 0,
                fly: 0,
                ffall: 0,
                lite: 0,
                free_act: 0,
                see_inv: 0,
                regenerate: 0,
                hold_life: 0,
                telepathy: 0,
                slow_digest: 0,
                bless_blade: 0,
                xtra_might: 0,
                impact: 0,
                auto_id: 0,
                invis: 0,
                dis_to_h: 0,
                dis_to_d: 0,
                dis_to_a: 0,
                dis_ac: 0,
                to_l: 0,
                to_m: 0,
                to_s: 0,
                to_h: 0,
                to_d: 0,
                to_h_melee: 0,
                to_d_melee: 0,
                to_h_ranged: 0,
                to_d_ranged: 0,
                to_a: 0,
                ac: 0,
                antimagic: 0,
                antimagic_dis: 0,
                see_infra: 0,
                skill_dis: 0,
                skill_dev: 0,
                skill_sav: 0,
                skill_stl: 0,
                skill_srh: 0,
                skill_fos: 0,
                skill_thn: 0,
                skill_thb: 0,
                skill_tht: 0,
                skill_dig: 0,
                num_blow: 0,
                num_fire: 0,
                xtra_crit: 0,
                throw_mult: 0,
                tval_xtra: 0,
                tval_ammo: 0,
                pspeed: 0,
                mimic_extra: 0,
                antimagic_extra: 0,
                druid_extra: 0,
                druid_extra2: 0,
                druid_extra3: 0,
                music_extra: 0,
                music_extra2: 0,
                necro_extra: 0,
                necro_extra2: 0,
                race_extra1: 0,
                race_extra2: 0,
                race_extra3: 0,
                race_extra4: 0,
                race_extra5: 0,
                race_extra6: 0,
                race_extra7: 0,
                dodge_chance: 0,
                maintain_sum: 0,
                spellbinder_num: 0,
                spellbinder: [0; 4],
                spellbinder_trigger: 0,
                mimic_name: 0 as *const libc::c_char,
                tactic: 0,
                movement: 0,
                companion_killed: 0,
                no_mortal: 0,
                black_breath: 0,
                precognition: 0,
                xtra_f1: 0,
                xtra_f2: 0,
                xtra_f3: 0,
                xtra_f4: 0,
                xtra_f5: 0,
                xtra_esp: 0,
                corruptions: 0 as *const bool_ as *mut bool_,
                pet_follow_distance: 0,
                pet_open_doors: 0,
                pet_pickup_items: 0,
                control: 0,
                control_dir: 0,
                body_monster: 0,
                disembodied: 0,
                body_parts: [0; 28],
                astral: 0,
                powers: 0 as *const bool_ as *mut bool_,
                powers_mod: [0; 62],
                skill_points: 0,
                skill_last_level: 0,
                melee_style: 0,
                use_piercing_shots: 0,
                help: help_info{enabled: 0, help1: 0,},
                did_nothing: 0,
                leaving: 0,};
/*
 * Pointer to the player info
 */
#[no_mangle]
pub static mut p_ptr: *mut player_type =
    unsafe { &p_body as *const player_type as *mut player_type };
/*
 * Pointer to the player tables
 * (sex, race, race mod, class, magic)
 */
#[no_mangle]
pub static mut sp_ptr: *mut player_sex =
    0 as *const player_sex as *mut player_sex;
#[no_mangle]
pub static mut rp_ptr: *mut player_race =
    0 as *const player_race as *mut player_race;
#[no_mangle]
pub static mut rmp_ptr: *mut player_race_mod =
    0 as *const player_race_mod as *mut player_race_mod;
#[no_mangle]
pub static mut cp_ptr: *mut player_class =
    0 as *const player_class as *mut player_class;
#[no_mangle]
pub static mut spp_ptr: *mut player_spec =
    0 as *const player_spec as *mut player_spec;
/*
 * More spell info
 */
#[no_mangle]
pub static mut alchemist_known_egos: [u32b; 32] = [0; 32];
#[no_mangle]
pub static mut alchemist_known_artifacts: [u32b; 6] = [0; 6];
#[no_mangle]
pub static mut alchemist_gained: u32b = 0;
/*
 * Calculated base hp values for player at each level,
 * store them so that drain life + restore life does not
 * affect hit points.  Also prevents shameless use of backup
 * savefiles for hitpoint acquirement.
 */
#[no_mangle]
pub static mut player_hp: [s16b; 50] = [0; 50];
/*
 * The alchemy recipe arrays
 */
#[no_mangle]
pub static mut al_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut alchemist_recipes: *mut alchemist_recipe =
    0 as *const alchemist_recipe as *mut alchemist_recipe;
#[no_mangle]
pub static mut al_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut a_select_flags: *mut artifact_select_flag =
    0 as *const artifact_select_flag as *mut artifact_select_flag;
/*
 * The vault generation arrays
 */
#[no_mangle]
pub static mut v_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut v_info: *mut vault_type =
    0 as *const vault_type as *mut vault_type;
#[no_mangle]
pub static mut v_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut v_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The terrain feature arrays
 */
#[no_mangle]
pub static mut f_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut f_info: *mut feature_type =
    0 as *const feature_type as *mut feature_type;
#[no_mangle]
pub static mut f_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut f_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The object kind arrays
 */
#[no_mangle]
pub static mut k_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut k_info: *mut object_kind =
    0 as *const object_kind as *mut object_kind;
#[no_mangle]
pub static mut k_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut k_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The artifact arrays
 */
#[no_mangle]
pub static mut a_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut a_info: *mut artifact_type =
    0 as *const artifact_type as *mut artifact_type;
#[no_mangle]
pub static mut a_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut a_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The item set arrays
 */
#[no_mangle]
pub static mut set_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut set_info: *mut set_type =
    0 as *const set_type as *mut set_type;
#[no_mangle]
pub static mut set_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut set_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The ego-item arrays
 */
#[no_mangle]
pub static mut e_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut e_info: *mut ego_item_type =
    0 as *const ego_item_type as *mut ego_item_type;
#[no_mangle]
pub static mut e_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut e_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The randart arrays
 */
#[no_mangle]
pub static mut ra_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut ra_info: *mut randart_part_type =
    0 as *const randart_part_type as *mut randart_part_type;
#[no_mangle]
pub static mut ra_gen: [randart_gen_type; 30] =
    [randart_gen_type{chance: 0, dd: 0, ds: 0, plus: 0,}; 30];
/* jk */
/* the trap-arrays */
#[no_mangle]
pub static mut t_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut t_info: *mut trap_type =
    0 as *const trap_type as *mut trap_type;
#[no_mangle]
pub static mut t_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut t_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The monster race arrays
 */
#[no_mangle]
pub static mut r_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut r_info: *mut monster_race =
    0 as *const monster_race as *mut monster_race;
#[no_mangle]
pub static mut r_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut r_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The monster ego race arrays
 */
#[no_mangle]
pub static mut re_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut re_info: *mut monster_ego =
    0 as *const monster_ego as *mut monster_ego;
#[no_mangle]
pub static mut re_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * The dungeon types arrays
 */
#[no_mangle]
pub static mut d_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut d_info: *mut dungeon_info_type =
    0 as *const dungeon_info_type as *mut dungeon_info_type;
#[no_mangle]
pub static mut d_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut d_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * Player abilities arrays
 */
#[no_mangle]
pub static mut ab_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut ab_info: *mut ability_type =
    0 as *const ability_type as *mut ability_type;
#[no_mangle]
pub static mut ab_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ab_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * Player skills arrays
 */
#[no_mangle]
pub static mut s_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut s_info: *mut skill_type =
    0 as *const skill_type as *mut skill_type;
#[no_mangle]
pub static mut s_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut s_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * Player race arrays
 */
#[no_mangle]
pub static mut rp_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut race_info: *mut player_race =
    0 as *const player_race as *mut player_race;
#[no_mangle]
pub static mut rp_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut rp_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * Player mod race arrays
 */
#[no_mangle]
pub static mut rmp_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut race_mod_info: *mut player_race_mod =
    0 as *const player_race_mod as *mut player_race_mod;
#[no_mangle]
pub static mut rmp_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut rmp_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * Player class arrays
 */
#[no_mangle]
pub static mut c_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut class_info: *mut player_class =
    0 as *const player_class as *mut player_class;
#[no_mangle]
pub static mut c_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut c_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut meta_class_info: *mut meta_class_type =
    0 as *const meta_class_type as *mut meta_class_type;
/*
 * The wilderness features arrays
 */
#[no_mangle]
pub static mut wf_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut wf_info: *mut wilderness_type_info =
    0 as *const wilderness_type_info as *mut wilderness_type_info;
#[no_mangle]
pub static mut wf_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut wf_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut wildc2i: [libc::c_int; 256] = [0; 256];
/*
 * The store/building types arrays
 */
#[no_mangle]
pub static mut st_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut st_info: *mut store_info_type =
    0 as *const store_info_type as *mut store_info_type;
#[no_mangle]
pub static mut st_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* char *st_text; */
/*
 * The building actions types arrays
 */
#[no_mangle]
pub static mut ba_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut ba_info: *mut store_action_type =
    0 as *const store_action_type as *mut store_action_type;
#[no_mangle]
pub static mut ba_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* char *ba_text; */
/*
 * The owner types arrays
 */
#[no_mangle]
pub static mut ow_head: *mut header = 0 as *const header as *mut header;
#[no_mangle]
pub static mut ow_info: *mut owner_type =
    0 as *const owner_type as *mut owner_type;
#[no_mangle]
pub static mut ow_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* char *ow_text; */
/*
 * The dungeon types arrays
 */
/*
 * Hack -- The special Angband "System Suffix"
 * This variable is used to choose an appropriate "pref-xxx" file
 */
#[no_mangle]
pub static mut ANGBAND_SYS: cptr =
    b"xxx\x00" as *const u8 as *const libc::c_char;
/*
 * Hack -- The special Angband "Keyboard Suffix"
 * This variable is used to choose an appropriate macro-trigger definition
 */
#[no_mangle]
pub static mut ANGBAND_KEYBOARD: cptr =
    b"0\x00" as *const u8 as *const libc::c_char;
/*
 * Hack -- The special Angband "Graphics Suffix"
 * This variable is used to choose an appropriate "graf-xxx" file
 */
#[no_mangle]
pub static mut ANGBAND_GRAF: cptr =
    b"old\x00" as *const u8 as *const libc::c_char;
/*
 * Path name: The main "lib" directory
 * This variable is not actually used anywhere in the code
 */
#[no_mangle]
pub static mut ANGBAND_DIR: cptr = 0 as *const libc::c_char;
/*
 * High score files (binary)
 * These files may be portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_APEX: cptr = 0 as *const libc::c_char;
/*
 * Core lua system
 * These files are portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_CORE: cptr = 0 as *const libc::c_char;
/*
 * Textual dungeon level definition files
 * These files are portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_DNGN: cptr = 0 as *const libc::c_char;
/*
 * Binary image files for the "*_info" arrays (binary)
 * These files are not portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_DATA: cptr = 0 as *const libc::c_char;
/*
 * Textual template files for the "*_info" arrays (ascii)
 * These files are portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_EDIT: cptr = 0 as *const libc::c_char;
/*
 * Various extra files (ascii)
 * These files may be portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_FILE: cptr = 0 as *const libc::c_char;
/*
 * Help files (normal) for the online help (ascii)
 * These files are portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_HELP: cptr = 0 as *const libc::c_char;
/*
 * Help files (spoilers) for the online help (ascii)
 * These files are portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_INFO: cptr = 0 as *const libc::c_char;
/*
 * Modules, those subdirectories are half-mirrors of lib/
 */
#[no_mangle]
pub static mut ANGBAND_DIR_MODULES: cptr = 0 as *const libc::c_char;
/*
 * Patches, contains one subdir per patch with a patch.lua file
 * in it and a patch_init() function in it
 */
#[no_mangle]
pub static mut ANGBAND_DIR_PATCH: cptr = 0 as *const libc::c_char;
/*
 * Textual template files for the plot files (ascii)
 * These files are portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_NOTE: cptr = 0 as *const libc::c_char;
/*
 * Savefiles for current characters (binary)
 * These files are portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_SAVE: cptr = 0 as *const libc::c_char;
/*
 * Scripts.
 * These files are portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_SCPT: cptr = 0 as *const libc::c_char;
/*
 * Default "preference" files (ascii)
 * These files are rarely portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_PREF: cptr = 0 as *const libc::c_char;
/*
 * User "preference" files (ascii)
 * These files are rarely portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_USER: cptr = 0 as *const libc::c_char;
/*
 * Various extra files (binary)
 * These files are rarely portable between platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_XTRA: cptr = 0 as *const libc::c_char;
/*
 * Cmovie files of entire games (ascii)
 * Apart from possible newline things, likely portable btw platforms
 */
#[no_mangle]
pub static mut ANGBAND_DIR_CMOV: cptr = 0 as *const libc::c_char;
/*
 * Some variables values are created on the fly XXX XXX
 */
#[no_mangle]
pub static mut pref_tmp_value: [libc::c_char; 8] = [0; 8];
/*
 * Total Hack -- allow all items to be listed (even empty ones)
 * This is only used by "do_cmd_inven_e()" and is cleared there.
 */
#[no_mangle]
pub static mut item_tester_full: bool_ = 0;
/*
 * Here is a "pseudo-hook" used during calls to "get_item()" and
 * "show_inven()" and "show_equip()", and the choice window routines.
 */
#[no_mangle]
pub static mut item_tester_tval: byte_hack = 0;
/*
 * Here is a "hook" used during calls to "get_item()" and
 * "show_inven()" and "show_equip()", and the choice window routines.
 */
#[no_mangle]
pub static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_> =
    None;
/*
 * Current "comp" function for ang_sort()
 */
#[no_mangle]
pub static mut ang_sort_comp:
           Option<unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                       _: libc::c_int) -> bool_> =
    None;
/*
 * Current "swap" function for ang_sort()
 */
#[no_mangle]
pub static mut ang_sort_swap:
           Option<unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                       _: libc::c_int) -> ()> =
    None;
/*
 * Hack -- function hooks to restrict "get_mon_num_prep()" function
 */
#[no_mangle]
pub static mut get_mon_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> =
    None;
#[no_mangle]
pub static mut get_mon_num2_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> =
    None;
/*
 * Hack -- function hook to restrict "get_obj_num_prep()" function
 */
#[no_mangle]
pub static mut get_obj_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> =
    None;
/* Hack, monk armour */
#[no_mangle]
pub static mut monk_armour_aux: bool_ = 0;
#[no_mangle]
pub static mut monk_notify_aux: bool_ = 0;
#[no_mangle]
pub static mut easy_open: bool_ = 1 as libc::c_int as bool_;
#[no_mangle]
pub static mut easy_disarm: bool_ = 1 as libc::c_int as bool_;
#[no_mangle]
pub static mut easy_tunnel: bool_ = 0 as libc::c_int as bool_;
/*
 * Maximum size of the wilderness map
 */
#[no_mangle]
pub static mut max_wild_x: u16b = 0;
#[no_mangle]
pub static mut max_wild_y: u16b = 0;
/*
 * Wilderness map
 */
#[no_mangle]
pub static mut wild_map: *mut *mut wilderness_map =
    0 as *const *mut wilderness_map as *mut *mut wilderness_map;
/*
 * Maximum number of skills in s_info.txt
 */
#[no_mangle]
pub static mut old_max_s_idx: u16b = 0 as libc::c_int as u16b;
#[no_mangle]
pub static mut max_s_idx: u16b = 0;
/*
 * Maximum number of abilities in ab_info.txt
 */
#[no_mangle]
pub static mut max_ab_idx: u16b = 0;
/*
 * Maximum number of monsters in r_info.txt
 */
#[no_mangle]
pub static mut max_r_idx: u16b = 0;
/*
 * Maximum number of ego monsters in re_info.txt
 */
#[no_mangle]
pub static mut max_re_idx: u16b = 0;
/*
 * Maximum number of items in k_info.txt
 */
#[no_mangle]
pub static mut max_k_idx: u16b = 0;
/*
 * Maximum number of vaults in v_info.txt
 */
#[no_mangle]
pub static mut max_v_idx: u16b = 0;
/*
 * Maximum number of terrain features in f_info.txt
 */
#[no_mangle]
pub static mut max_f_idx: u16b = 0;
/*
 * Maximum number of alchemist recipies in al_info.txt
 */
#[no_mangle]
pub static mut max_al_idx: u16b = 0;
/*
 * Maximum number of artifacts in a_info.txt
 */
#[no_mangle]
pub static mut max_a_idx: u16b = 0;
/*
 * Maximum number of ego-items in e_info.txt
 */
#[no_mangle]
pub static mut max_e_idx: u16b = 0;
/*
 * Maximum number of randarts in ra_info.txt
 */
#[no_mangle]
pub static mut max_ra_idx: u16b = 0;
/*
 * Maximum number of dungeon types in d_info.txt
 */
#[no_mangle]
pub static mut max_d_idx: u16b = 0;
/*
 * Maximum number of stores types in st_info.txt
 */
#[no_mangle]
pub static mut max_st_idx: u16b = 0;
/*
 * Item sets
 */
#[no_mangle]
pub static mut max_set_idx: s16b = 1 as libc::c_int as s16b;
/*
 * Maximum number of players info in p_info.txt
 */
#[no_mangle]
pub static mut max_rp_idx: u16b = 0;
#[no_mangle]
pub static mut max_rmp_idx: u16b = 0;
#[no_mangle]
pub static mut max_c_idx: u16b = 0;
#[no_mangle]
pub static mut max_mc_idx: u16b = 0;
/*
 * Maximum number of actions types in ba_info.txt
 */
#[no_mangle]
pub static mut max_ba_idx: u16b = 0;
/*
 * Maximum number of owner types in ow_info.txt
 */
#[no_mangle]
pub static mut max_ow_idx: u16b = 0;
/*
 * Maximum number of objects in the level
 */
#[no_mangle]
pub static mut max_o_idx: u16b = 0;
/*
 * Maximum number of monsters in the level
 */
#[no_mangle]
pub static mut max_m_idx: u16b = 0;
/*
 * Maximum number of traps in tr_info.txt
 */
#[no_mangle]
pub static mut max_t_idx: u16b = 0;
/*
 * Maximum number of wilderness features in wf_info.txt
 */
#[no_mangle]
pub static mut max_wf_idx: u16b = 0;
/*
 * Flags for initialization
 */
#[no_mangle]
pub static mut init_flags: libc::c_int = 0;
/* True if on an ambush */
#[no_mangle]
pub static mut ambush_flag: bool_ = 0;
/* True if on fated level */
#[no_mangle]
pub static mut fate_flag: bool_ = 0;
/* No breeders */
#[no_mangle]
pub static mut no_breeds: s16b = 0;
/* Carried monsters can't take the damage if this is them which attack the player */
#[no_mangle]
pub static mut carried_monster_hit: bool_ = 0 as libc::c_int as bool_;
/*
 * Random artifacts.
 */
#[no_mangle]
pub static mut random_artifacts: [random_artifact; 84] =
    [random_artifact{name_full: [0; 80],
                     name_short: [0; 80],
                     level: 0,
                     attr: 0,
                     cost: 0,
                     activation: 0,
                     timeout: 0,
                     generated: 0,}; 84];
/* These three used to be constants but now are set by modules */
#[no_mangle]
pub static mut RANDART_WEAPON: s32b = 0;
#[no_mangle]
pub static mut RANDART_ARMOR: s32b = 0;
#[no_mangle]
pub static mut RANDART_JEWEL: s32b = 0;
/*
 * Current bounties. An array of tuples of two, with the first being the
 * r_idx of the monster, and the second the monster's worth.
 */
#[no_mangle]
pub static mut bounties: [[s16b; 2]; 24] = [[0; 2]; 24];
/*
 * Spell description
 */
#[no_mangle]
pub static mut info_spell: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub static mut spell_txt: [libc::c_char; 50] = [0; 50];
/*
 * Random spells.
 */
#[no_mangle]
pub static mut random_spells: [random_spell; 100] =
    [random_spell{desc: [0; 30],
                  name: [0; 30],
                  mana: 0,
                  fail: 0,
                  proj_flags: 0,
                  GF: 0,
                  radius: 0,
                  dam_sides: 0,
                  dam_dice: 0,
                  level: 0,
                  untried: 0,}; 100];
#[no_mangle]
pub static mut spell_num: s16b = 0;
/*
 * Runecrafter's selfmade spells.
 */
#[no_mangle]
pub static mut rune_spells: [rune_spell; 100] =
    [rune_spell{name: [0; 30], type_0: 0, rune2: 0, mana: 0,}; 100];
#[no_mangle]
pub static mut rune_num: s16b = 0;
/*
 * Fate.
 */
#[no_mangle]
pub static mut fates: [fate; 200] =
    [fate{fate: 0,
          level: 0,
          serious: 0,
          o_idx: 0,
          e_idx: 0,
          a_idx: 0,
          v_idx: 0,
          r_idx: 0,
          count: 0,
          time: 0,
          know: 0,
          icky: 0,}; 200];
/*
 * Which dungeon ?
 * 0 = Wilderness
 * 1 = Mirkwood
 * 2 = Mordor
 * 3 = Angband
 * 4 = Barrow Downs
 * 5 = Mount Doom
 * 6 = Nether Realm
 * etc. (see d_info.txt)
 */
#[no_mangle]
pub static mut dungeon_type: byte_hack = 0;
#[no_mangle]
pub static mut max_dlv: *mut s16b = 0 as *const s16b as *mut s16b;
/*
 * Number of total bounties the player had had.
 */
#[no_mangle]
pub static mut total_bounties: u32b = 0;
/* The Doppleganger index in m_list */
#[no_mangle]
pub static mut doppleganger: s16b = 0;
/* To allow wilderness encounters */
#[no_mangle]
pub static mut generate_encounter: bool_ = 0;
/* Permanent dungeons ? */
#[no_mangle]
pub static mut permanent_levels: bool_ = 0;
/* Autoroler */
#[no_mangle]
pub static mut autoroll: bool_ = 0;
/* Point based */
#[no_mangle]
pub static mut point_based: bool_ = 0;
/* Maximize, preserve, special levels, ironman_rooms */
#[no_mangle]
pub static mut maximize: bool_ = 0;
#[no_mangle]
pub static mut preserve: bool_ = 0;
#[no_mangle]
pub static mut special_lvls: bool_ = 0;
#[no_mangle]
pub static mut ironman_rooms: bool_ = 0;
/* In inventory option window, just erase the letters,
 * rather that displaying the list without the invalid
 * selections */
#[no_mangle]
pub static mut inventory_no_move: bool_ = 0;
/* Notes patch */
#[no_mangle]
pub static mut take_notes: bool_ = 0;
#[no_mangle]
pub static mut auto_notes: bool_ = 0;
/*
 * Such an ugly hack ...
 */
#[no_mangle]
pub static mut m_allow_special: *mut bool_ = 0 as *const bool_ as *mut bool_;
#[no_mangle]
pub static mut k_allow_special: *mut bool_ = 0 as *const bool_ as *mut bool_;
#[no_mangle]
pub static mut a_allow_special: *mut bool_ = 0 as *const bool_ as *mut bool_;
/*
 * Gives a random object to newly created characters
 */
#[no_mangle]
pub static mut rand_birth: bool_ = 0;
/*
 * Fast autoroller
 */
#[no_mangle]
pub static mut fast_autoroller: bool_ = 0;
/*
 * Which monsters are allowed ?
 */
#[no_mangle]
pub static mut joke_monsters: bool_ = 0;
/*
 * How will mana staf & weapons of life act
 */
#[no_mangle]
pub static mut munchkin_multipliers: bool_ = 1 as libc::c_int as bool_;
/*
 * Center view
 */
#[no_mangle]
pub static mut center_player: bool_ = 0 as libc::c_int as bool_;
/*
 * Plots
 */
#[no_mangle]
pub static mut plots: [s16b; 7] = [0; 7];
/*
 * Random quest
 */
#[no_mangle]
pub static mut random_quests: [random_quest; 99] =
    [random_quest{type_0: 0, r_idx: 0, done: 0,}; 99];
/*
 * Show exp left
 */
#[no_mangle]
pub static mut exp_need: bool_ = 0;
/*
 * Auto load old colors;
 */
#[no_mangle]
pub static mut autoload_old_colors: bool_ = 0;
/*
 * Fated ?
 */
#[no_mangle]
pub static mut fate_option: bool_ = 0;
/*
 * Special levels
 */
#[no_mangle]
pub static mut special_lvl: [*mut bool_; 128] =
    [0 as *const bool_ as *mut bool_; 128];
#[no_mangle]
pub static mut generate_special_feeling: bool_ = 0 as libc::c_int as bool_;
/*
 * Auto more
 */
#[no_mangle]
pub static mut auto_more: bool_ = 0;
/*
 * Dungeon flags
 */
#[no_mangle]
pub static mut dungeon_flags1: u32b = 0;
#[no_mangle]
pub static mut dungeon_flags2: u32b = 0;
/*
 * The last character displayed
 */
#[no_mangle]
pub static mut previous_char: birther =
    birther{sex: 0,
            race: 0,
            rmod: 0,
            pclass: 0,
            spec: 0,
            quests: 0,
            god: 0,
            grace: 0,
            god_favor: 0,
            age: 0,
            wt: 0,
            ht: 0,
            sc: 0,
            au: 0,
            stat: [0; 6],
            luck: 0,
            chaos_patron: 0,
            weapon: 0,
            history: [[0; 60]; 4],
            quick_ok: 0,};
/*
 * Race histories
 */
#[no_mangle]
pub static mut bg: *mut hist_type = 0 as *const hist_type as *mut hist_type;
#[no_mangle]
pub static mut max_bg_idx: libc::c_int = 0;
/*
 * Powers
 */
#[no_mangle]
pub static mut power_max: s16b = 62 as libc::c_int as s16b;
#[no_mangle]
pub static mut powers_type: *mut power_type =
    0 as *const power_type as *mut power_type;
/*
 * Variable savefile stuff
 */
#[no_mangle]
pub static mut extra_savefile_parts: s32b = 0 as libc::c_int;
/*
 * Quests
 */
#[no_mangle]
pub static mut max_q_idx: s16b = 26 as libc::c_int as s16b;
#[no_mangle]
pub static mut quest: *mut quest_type =
    0 as *const quest_type as *mut quest_type;
/*
 * Display the player as a special symbol when in bad health ?
 */
#[no_mangle]
pub static mut player_char_health: bool_ = 0;
/*
 * The spell list of schools
 */
#[no_mangle]
pub static mut max_spells: s16b = 0;
#[no_mangle]
pub static mut school_spells: *mut spell_type =
    0 as *const spell_type as *mut spell_type;
#[no_mangle]
pub static mut max_schools: s16b = 0;
#[no_mangle]
pub static mut schools: *mut school_type =
    0 as *const school_type as *mut school_type;
/*
 * Lasting spell effects
 */
#[no_mangle]
pub static mut project_time: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut project_time_effect: s32b = 0 as libc::c_int;
#[no_mangle]
pub static mut effects: [effect_type; 128] =
    [effect_type{time: 0, dam: 0, type_0: 0, cy: 0, cx: 0, rad: 0, flags: 0,};
        128];
/*
 * General skills set
 */
#[no_mangle]
pub static mut gen_skill_basem: [libc::c_char; 200] = [0; 200];
#[no_mangle]
pub static mut gen_skill_base: [u32b; 200] = [0; 200];
#[no_mangle]
pub static mut gen_skill_modm: [libc::c_char; 200] = [0; 200];
#[no_mangle]
pub static mut gen_skill_mod: [s16b; 200] = [0; 200];
/*
 * Display stats as linear
 */
#[no_mangle]
pub static mut linear_stats: bool_ = 0;
/*
 * Table of "cli" macros.
 */
#[no_mangle]
pub static mut cli_info: *mut cli_comm =
    0 as *const cli_comm as *mut cli_comm;
#[no_mangle]
pub static mut cli_total: libc::c_int = 0 as libc::c_int;
/*
 * max_bact, only used so that lua scripts can add new bacts without worrying about the numbers
 */
#[no_mangle]
pub static mut max_bact: libc::c_int = 54 as libc::c_int;
/*
 * Max corruptions
 */
#[no_mangle]
pub static mut max_corruptions: s16b = 0 as libc::c_int as s16b;
/*
 * Ingame contextual help
 */
#[no_mangle]
pub static mut option_ingame_help: bool_ = 1 as libc::c_int as bool_;
/*
 * Automatizer enabled status
 */
#[no_mangle]
pub static mut automatizer_enabled: bool_ = 0 as libc::c_int as bool_;
/*
 * Location of the last teleportation thath affected the level
 */
#[no_mangle]
pub static mut last_teleportation_y: s16b = -(1 as libc::c_int) as s16b;
#[no_mangle]
pub static mut last_teleportation_x: s16b = -(1 as libc::c_int) as s16b;
/*
 * The current game module
 */
#[no_mangle]
pub static mut game_module: cptr = 0 as *const libc::c_char;
#[no_mangle]
pub static mut VERSION_MAJOR: s32b = 0;
#[no_mangle]
pub static mut VERSION_MINOR: s32b = 0;
#[no_mangle]
pub static mut VERSION_PATCH: s32b = 0;
/*
 * Some module info
 */
#[no_mangle]
pub static mut max_plev: s32b = 50 as libc::c_int;
#[no_mangle]
pub static mut DUNGEON_DEATH: s32b = 28 as libc::c_int;
/*
 * Gods
 */
#[no_mangle]
pub static mut deity_info: *mut deity_type =
    0 as *const deity_type as *mut deity_type;
#[no_mangle]
pub static mut max_gods: s32b = 6 as libc::c_int;
/*
 * Timers
 */
#[no_mangle]
pub static mut gl_timers: *mut timer_type =
    0 as *const timer_type as *mut timer_type;
/* *
 * Get the version string.
 */
#[no_mangle]
pub unsafe extern "C" fn get_version_string() -> *const libc::c_char {
    static mut version_str: [libc::c_char; 80] = [0; 80];
    static mut initialized: bool_ = 0 as libc::c_int as bool_;
    if initialized == 0 {
        sprintf(version_str.as_mut_ptr(),
                b"%s %ld.%ld.%ld%s\x00" as *const u8 as *const libc::c_char,
                game_module, VERSION_MAJOR as libc::c_long,
                VERSION_MINOR as libc::c_long, VERSION_PATCH as libc::c_long,
                b" (ah)\x00" as *const u8 as *const libc::c_char);
        initialized = 1 as libc::c_int as bool_
    }
    return version_str.as_mut_ptr();
}
