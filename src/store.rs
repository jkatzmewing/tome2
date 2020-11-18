use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut adj_chr_gold: [byte_hack; 0];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_cmd: s16b;
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut command_rep: s16b;
    #[no_mangle]
    static mut command_new: s16b;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut turn: s32b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut use_bigtile: bool_;
    #[no_mangle]
    static mut show_store_graph: bool_;
    #[no_mangle]
    static mut auto_haggle: bool_;
    #[no_mangle]
    static mut show_weights: bool_;
    #[no_mangle]
    static mut cheat_peek: bool_;
    #[no_mangle]
    static mut speak_unique: bool_;
    #[no_mangle]
    static mut rating: s16b;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut town_info: *mut town_type;
    #[no_mangle]
    static mut alloc_kind_table_valid: bool_;
    #[no_mangle]
    static mut misc_to_attr: [byte_hack; 256];
    #[no_mangle]
    static mut misc_to_char: [libc::c_char; 256];
    #[no_mangle]
    static mut tval_to_attr: [byte_hack; 128];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut st_info: *mut store_info_type;
    #[no_mangle]
    static mut st_name: *mut libc::c_char;
    #[no_mangle]
    static mut ba_info: *mut store_action_type;
    #[no_mangle]
    static mut ow_info: *mut owner_type;
    #[no_mangle]
    static mut ow_name: *mut libc::c_char;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut get_obj_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut max_st_idx: u16b;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut process_hooks_return: [hook_return; 20];
    #[no_mangle]
    fn process_hooks_ret(h_idx: libc::c_int, ret: *mut libc::c_char,
                         fmt: *mut libc::c_char, _: ...) -> bool_;
    #[no_mangle]
    fn forget_view();
    #[no_mangle]
    fn do_cmd_inven();
    #[no_mangle]
    fn do_cmd_equip();
    #[no_mangle]
    fn do_cmd_wield();
    #[no_mangle]
    fn do_cmd_takeoff();
    #[no_mangle]
    fn do_cmd_destroy();
    #[no_mangle]
    fn do_cmd_observe();
    #[no_mangle]
    fn do_cmd_uninscribe();
    #[no_mangle]
    fn do_cmd_inscribe();
    #[no_mangle]
    fn do_cmd_query_symbol();
    #[no_mangle]
    fn do_cmd_redraw();
    #[no_mangle]
    fn do_cmd_change_name();
    #[no_mangle]
    fn do_cmd_message_one();
    #[no_mangle]
    fn do_cmd_messages();
    #[no_mangle]
    fn do_cmd_options();
    #[no_mangle]
    fn do_cmd_pref();
    #[no_mangle]
    fn do_cmd_macros();
    #[no_mangle]
    fn do_cmd_visuals();
    #[no_mangle]
    fn do_cmd_colors();
    #[no_mangle]
    fn do_cmd_note();
    #[no_mangle]
    fn do_cmd_version();
    #[no_mangle]
    fn do_cmd_feeling();
    #[no_mangle]
    fn do_cmd_load_screen();
    #[no_mangle]
    fn do_cmd_save_screen();
    #[no_mangle]
    fn do_cmd_knowledge();
    #[no_mangle]
    fn do_cmd_browse_aux(o_ptr: *mut object_type);
    #[no_mangle]
    fn do_cmd_browse();
    #[no_mangle]
    fn do_cmd_help();
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    fn get_item_letter_color(o_ptr: *mut object_type) -> byte_hack;
    #[no_mangle]
    fn inc_stack_size(item: libc::c_int, delta: libc::c_int);
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn init_match_theme(theme: obj_theme);
    #[no_mangle]
    fn kind_is_legal(k_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_flags_known(o_ptr: *mut object_type, f1: *mut u32b,
                          f2: *mut u32b, f3: *mut u32b, f4: *mut u32b,
                          f5: *mut u32b, esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn object_desc_store(buf: *mut libc::c_char, o_ptr: *mut object_type,
                         pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn object_out_desc(o_ptr: *mut object_type, fff: *mut FILE,
                       trim_down: bool_, wait_for_it: bool_) -> bool_;
    #[no_mangle]
    fn index_to_label(i: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn inven_carry_okay(o_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn inven_carry(o_ptr: *mut object_type, final_0: bool_) -> s16b;
    #[no_mangle]
    fn atol(__nptr: *const libc::c_char) -> libc::c_long;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_draw(x: libc::c_int, y: libc::c_int, a: byte_hack,
                 c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_user(n: libc::c_int) -> errr;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn toggle_inven_equip();
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_obj_num_prep() -> errr;
    #[no_mangle]
    fn get_obj_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_value(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn object_value_real(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn object_similar(o_ptr: *mut object_type, j_ptr: *mut object_type)
     -> bool_;
    #[no_mangle]
    fn object_absorb(o_ptr: *mut object_type, j_ptr: *mut object_type);
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
    fn msg_print(msg: cptr);
    #[no_mangle]
    static mut request_command_ignore_keymaps: [libc::c_char; 0];
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn is_state(s_ptr: *mut store_type, state: libc::c_int) -> bool_;
    #[no_mangle]
    fn c_prt(attr: byte_hack, str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn notice_stuff();
    #[no_mangle]
    fn bldg_process_command(s_ptr: *mut store_type, i: libc::c_int) -> bool_;
    #[no_mangle]
    fn repeat_check();
    #[no_mangle]
    fn request_command(shopping: libc::c_int);
    #[no_mangle]
    fn show_building(s_ptr: *mut store_type);
    #[no_mangle]
    fn clear_from(row: libc::c_int);
    #[no_mangle]
    fn quark_add(str: cptr) -> s16b;
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_quantity(prompt: cptr, max: s32b) -> s32b;
    #[no_mangle]
    fn repeat_push(what: libc::c_int);
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn repeat_pull(what: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn get_skill_scale(skill: libc::c_int, scale: u32b) -> s16b;
    #[no_mangle]
    fn test_object_wish(name: *mut libc::c_char, o_ptr: *mut object_type,
                        forge: *mut object_type, what: *mut libc::c_char)
     -> bool_;
    #[no_mangle]
    fn clean_wish_name(buf: *mut libc::c_char, name: *mut libc::c_char);
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
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
pub union hook_return {
    pub num: s32b,
    pub str_0: cptr,
    pub o_ptr: *mut object_type,
    pub m_ptr: *mut monster_type,
}
/* File: store.c */
/* Purpose: Store commands */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
static mut comment_1: [cptr; 6] =
    [b"Okay.\x00" as *const u8 as *const libc::c_char,
     b"Fine.\x00" as *const u8 as *const libc::c_char,
     b"Accepted!\x00" as *const u8 as *const libc::c_char,
     b"Agreed!\x00" as *const u8 as *const libc::c_char,
     b"Done!\x00" as *const u8 as *const libc::c_char,
     b"Taken!\x00" as *const u8 as *const libc::c_char];
static mut comment_2a: [cptr; 2] =
    [b"You try my patience.  %s is final.\x00" as *const u8 as
         *const libc::c_char,
     b"My patience grows thin.  %s is final.\x00" as *const u8 as
         *const libc::c_char];
static mut comment_2b: [cptr; 12] =
    [b"I can take no less than %s gold pieces.\x00" as *const u8 as
         *const libc::c_char,
     b"I will accept no less than %s gold pieces.\x00" as *const u8 as
         *const libc::c_char,
     b"Ha!  No less than %s gold pieces.\x00" as *const u8 as
         *const libc::c_char,
     b"You knave!  No less than %s gold pieces.\x00" as *const u8 as
         *const libc::c_char,
     b"That\'s a pittance!  I want %s gold pieces.\x00" as *const u8 as
         *const libc::c_char,
     b"That\'s an insult!  I want %s gold pieces.\x00" as *const u8 as
         *const libc::c_char,
     b"As if!  How about %s gold pieces?\x00" as *const u8 as
         *const libc::c_char,
     b"My gosh!  How about %s gold pieces?\x00" as *const u8 as
         *const libc::c_char,
     b"May the fleas of 1000 orcs molest you!  Try %s gold pieces.\x00" as
         *const u8 as *const libc::c_char,
     b"May your most favourite weapons rust!  Try %s gold pieces.\x00" as
         *const u8 as *const libc::c_char,
     b"May Morgoth find you tasty!  Perhaps %s gold pieces?\x00" as *const u8
         as *const libc::c_char,
     b"Your mother was an Ogre!  Perhaps %s gold pieces?\x00" as *const u8 as
         *const libc::c_char];
static mut comment_3a: [cptr; 2] =
    [b"You try my patience.  %s is final.\x00" as *const u8 as
         *const libc::c_char,
     b"My patience grows thin.  %s is final.\x00" as *const u8 as
         *const libc::c_char];
static mut comment_3b: [cptr; 12] =
    [b"Perhaps %s gold pieces?\x00" as *const u8 as *const libc::c_char,
     b"How about %s gold pieces?\x00" as *const u8 as *const libc::c_char,
     b"I will pay no more than %s gold pieces.\x00" as *const u8 as
         *const libc::c_char,
     b"I can afford no more than %s gold pieces.\x00" as *const u8 as
         *const libc::c_char,
     b"Be reasonable.  How about %s gold pieces?\x00" as *const u8 as
         *const libc::c_char,
     b"I\'ll buy it as scrap for %s gold pieces.\x00" as *const u8 as
         *const libc::c_char,
     b"That is too much!  How about %s gold pieces?\x00" as *const u8 as
         *const libc::c_char,
     b"That looks war surplus!  Say %s gold pieces?\x00" as *const u8 as
         *const libc::c_char,
     b"Never!  %s is more like it.\x00" as *const u8 as *const libc::c_char,
     b"That\'s an insult!  %s is more like it.\x00" as *const u8 as
         *const libc::c_char,
     b"%s gold pieces and be thankful for it!\x00" as *const u8 as
         *const libc::c_char,
     b"%s gold pieces and not a copper more!\x00" as *const u8 as
         *const libc::c_char];
static mut comment_4a: [cptr; 4] =
    [b"Enough!  You have abused me once too often!\x00" as *const u8 as
         *const libc::c_char,
     b"Arghhh!  I have had enough abuse for one day!\x00" as *const u8 as
         *const libc::c_char,
     b"That does it!  You shall waste my time no more!\x00" as *const u8 as
         *const libc::c_char,
     b"This is getting nowhere!  I\'m going to Londis!\x00" as *const u8 as
         *const libc::c_char];
static mut comment_4b: [cptr; 4] =
    [b"Leave my store!\x00" as *const u8 as *const libc::c_char,
     b"Get out of my sight!\x00" as *const u8 as *const libc::c_char,
     b"Begone, you scoundrel!\x00" as *const u8 as *const libc::c_char,
     b"Out, out, out!\x00" as *const u8 as *const libc::c_char];
static mut comment_5: [cptr; 8] =
    [b"Try again.\x00" as *const u8 as *const libc::c_char,
     b"Ridiculous!\x00" as *const u8 as *const libc::c_char,
     b"You will have to do better than that!\x00" as *const u8 as
         *const libc::c_char,
     b"Do you wish to do business or not?\x00" as *const u8 as
         *const libc::c_char,
     b"You\'ve got to be kidding!\x00" as *const u8 as *const libc::c_char,
     b"You\'d better be kidding!\x00" as *const u8 as *const libc::c_char,
     b"You try my patience.\x00" as *const u8 as *const libc::c_char,
     b"Hmmm, nice weather we\'re having.\x00" as *const u8 as
         *const libc::c_char];
static mut comment_6: [cptr; 4] =
    [b"I must have heard you wrong.\x00" as *const u8 as *const libc::c_char,
     b"I\'m sorry, I missed that.\x00" as *const u8 as *const libc::c_char,
     b"I\'m sorry, what was that?\x00" as *const u8 as *const libc::c_char,
     b"Sorry, what was that again?\x00" as *const u8 as *const libc::c_char];
/*
 * Successful haggle.
 */
unsafe extern "C" fn say_comment_1() {
    let mut rumour: [libc::c_char; 80] = [0; 80];
    msg_print(comment_1[Rand_div(6 as libc::c_int) as usize]);
    if Rand_div(8 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int &&
           speak_unique as libc::c_int != 0 {
        msg_print(b"The shopkeeper whispers something into your ear:\x00" as
                      *const u8 as *const libc::c_char);
        get_rnd_line(b"rumors.txt\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, rumour.as_mut_ptr());
        msg_print(rumour.as_mut_ptr() as cptr);
    };
}
/*
 * Continue haggling (player is buying)
 */
unsafe extern "C" fn say_comment_2(mut value: s32b,
                                   mut annoyed: libc::c_int) {
    let mut tmp_val: [libc::c_char; 80] = [0; 80];
    /* Prepare a string to insert */
    strnfmt(tmp_val.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"%ld\x00" as *const u8 as *const libc::c_char,
            value as libc::c_long);
    /* Final offer */
    if annoyed > 0 as libc::c_int {
        /* Formatted message */
        msg_format(comment_2a[Rand_div(2 as libc::c_int) as usize],
                   tmp_val.as_mut_ptr());
    } else {
        /* Normal offer */
        /* Formatted message */
        msg_format(comment_2b[Rand_div(12 as libc::c_int) as usize],
                   tmp_val.as_mut_ptr());
    };
}
/*
 * Continue haggling (player is selling)
 */
unsafe extern "C" fn say_comment_3(mut value: s32b,
                                   mut annoyed: libc::c_int) {
    let mut tmp_val: [libc::c_char; 80] = [0; 80];
    /* Prepare a string to insert */
    strnfmt(tmp_val.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"%ld\x00" as *const u8 as *const libc::c_char,
            value as libc::c_long);
    /* Final offer */
    if annoyed > 0 as libc::c_int {
        /* Formatted message */
        msg_format(comment_3a[Rand_div(2 as libc::c_int) as usize],
                   tmp_val.as_mut_ptr());
    } else {
        /* Normal offer */
        /* Formatted message */
        msg_format(comment_3b[Rand_div(12 as libc::c_int) as usize],
                   tmp_val.as_mut_ptr());
    };
}
/*
 * Kick 'da bum out.					-RAK-
 */
unsafe extern "C" fn say_comment_4() {
    msg_print(comment_4a[Rand_div(4 as libc::c_int) as usize]);
    msg_print(comment_4b[Rand_div(4 as libc::c_int) as usize]);
}
/*
 * You are insulting me
 */
unsafe extern "C" fn say_comment_5() {
    msg_print(comment_5[Rand_div(8 as libc::c_int) as usize]);
}
/*
 * That makes no sense.
 */
unsafe extern "C" fn say_comment_6() {
    msg_print(comment_6[Rand_div(5 as libc::c_int) as usize]);
}
/*
 * Messages for reacting to purchase prices.
 */
static mut comment_7a: [cptr; 4] =
    [b"Arrgghh!\x00" as *const u8 as *const libc::c_char,
     b"You moron!\x00" as *const u8 as *const libc::c_char,
     b"You hear someone sobbing...\x00" as *const u8 as *const libc::c_char,
     b"The shopkeeper howls in agony!\x00" as *const u8 as
         *const libc::c_char];
static mut comment_7b: [cptr; 4] =
    [b"Darn!\x00" as *const u8 as *const libc::c_char,
     b"You fiend!\x00" as *const u8 as *const libc::c_char,
     b"The shopkeeper yells at you.\x00" as *const u8 as *const libc::c_char,
     b"The shopkeeper glares at you.\x00" as *const u8 as
         *const libc::c_char];
static mut comment_7c: [cptr; 4] =
    [b"Cool!\x00" as *const u8 as *const libc::c_char,
     b"You\'ve made my day!\x00" as *const u8 as *const libc::c_char,
     b"The shopkeeper giggles.\x00" as *const u8 as *const libc::c_char,
     b"The shopkeeper laughs loudly.\x00" as *const u8 as
         *const libc::c_char];
static mut comment_7d: [cptr; 4] =
    [b"Yippee!\x00" as *const u8 as *const libc::c_char,
     b"I think I\'ll retire!\x00" as *const u8 as *const libc::c_char,
     b"The shopkeeper jumps for joy.\x00" as *const u8 as *const libc::c_char,
     b"The shopkeeper smiles gleefully.\x00" as *const u8 as
         *const libc::c_char];
/*
 * Let a shop-keeper React to a purchase
 *
 * We paid "price", it was worth "value", and we thought it was worth "guess"
 */
unsafe extern "C" fn purchase_analyze(mut price: s32b, mut value: s32b,
                                      mut guess: s32b) {
    /* Item was worthless, but we bought it */
    if value <= 0 as libc::c_int && price > value {
        /* Comment */
        msg_print(comment_7a[Rand_div(4 as libc::c_int) as usize]);
        /* Sound */
        sound(17 as libc::c_int);
    } else if value < guess && price > value {
        /* Item was cheaper than we thought, and we paid more than necessary */
        /* Comment */
        msg_print(comment_7b[Rand_div(4 as libc::c_int) as usize]);
        /* Sound */
        sound(18 as libc::c_int);
    } else if value > guess && value < 4 as libc::c_int * guess &&
                  price < value {
        /* Item was a good bargain, and we got away with it */
        /* Comment */
        msg_print(comment_7c[Rand_div(4 as libc::c_int) as usize]);
        /* Sound */
        sound(19 as libc::c_int);
    } else if value > guess && price < value {
        /* Item was a great bargain, and we got away with it */
        /* Comment */
        msg_print(comment_7d[Rand_div(4 as libc::c_int) as usize]);
        /* Sound */
        sound(20 as libc::c_int);
    };
}
/*
 * We store the current "store number" here so everyone can access it
 */
static mut cur_store_num: libc::c_int = 7 as libc::c_int;
/*
 * We store the current "store page" here so everyone can access it
 */
static mut store_top: libc::c_int = 0 as libc::c_int;
/*
 * We store the current "store pointer" here so everyone can access it
 */
static mut st_ptr: *mut store_type =
    0 as *const store_type as *mut store_type;
/*
 * We store the current "owner type" here so everyone can access it
 */
static mut ot_ptr: *mut owner_type =
    0 as *const owner_type as *mut owner_type;
/*
 * Determine the price of an item (qty one) in a store.
 *
 * This function takes into account the player's charisma, and the
 * shop-keepers friendliness, and the shop-keeper's base greed, but
 * never lets a shop-keeper lose money in a transaction.
 *
 * The "greed" value should exceed 100 when the player is "buying" the
 * item, and should be less than 100 when the player is "selling" it.
 *
 * Hack -- the black market always charges twice as much as it should.
 *
 * Charisma adjustment runs from 80 to 130
 * Racial adjustment runs from 95 to 130
 *
 * Since greed/charisma/racial adjustments are centered at 100, we need
 * to adjust (by 200) to extract a usable multiplier.  Note that the
 * "greed" value is always something (?).
 */
unsafe extern "C" fn price_item(mut o_ptr: *mut object_type,
                                mut greed: libc::c_int, mut flip: bool_)
 -> s32b {
    let mut factor: libc::c_int = 0;
    let mut adjust: libc::c_int = 0;
    let mut price: s32b = 0;
    /* Get the value of one of the items */
    price = object_value(o_ptr);
    /* Worthless items */
    if price <= 0 as libc::c_int { return 0 as libc::c_long as s32b }
    /* Compute the racial factor */
    if is_state(st_ptr, 1 as libc::c_int) != 0 {
        factor = (*ot_ptr).costs[1 as libc::c_int as usize] as libc::c_int
    } else if is_state(st_ptr, 0 as libc::c_int) != 0 {
        factor = (*ot_ptr).costs[0 as libc::c_int as usize] as libc::c_int
    } else {
        factor = (*ot_ptr).costs[2 as libc::c_int as usize] as libc::c_int
    }
    /* Add in the charisma factor */
    factor +=
        *adj_chr_gold.as_mut_ptr().offset((*p_ptr).stat_ind[5 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int;
    /* Shop is buying */
    if flip != 0 {
        /* Mega Hack^3 */
        match (*o_ptr).tval as libc::c_int {
            16 | 17 | 18 => { price /= 5 as libc::c_int }
            _ => { }
        }
        /* Adjust for greed */
        adjust = 100 as libc::c_int + (300 as libc::c_int - (greed + factor));
        /* Never get "silly" */
        if adjust > 100 as libc::c_int { adjust = 100 as libc::c_int }
        /* Mega-Hack -- Black market sucks */
        if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as libc::c_long
               & 0x80 as libc::c_long != 0 {
            price = price / 2 as libc::c_int
        }
    } else {
        /* Shop is selling */
        /* Adjust for greed */
        adjust = 100 as libc::c_int + (greed + factor - 300 as libc::c_int);
        if adjust < 100 as libc::c_int { adjust = 100 as libc::c_int }
        if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as libc::c_long
               & 0x80 as libc::c_long != 0 {
            price = price * 2 as libc::c_int
        }
    }
    /* Never get "silly" */
    /* Mega-Hack -- Black market sucks */
    /* Compute the final price (with rounding) */
    price =
        (((price * adjust) as libc::c_long + 50 as libc::c_long) /
             100 as libc::c_long) as s32b;
    /* Note -- Never become "free" */
    if price as libc::c_long <= 0 as libc::c_long {
        return 1 as libc::c_long as s32b
    }
    /* Return the price */
    return price;
}
/*
 * Special "mass production" computation
 */
unsafe extern "C" fn mass_roll(mut num: libc::c_int, mut max: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num { t += Rand_div(max); i += 1 }
    return t;
}
/*
 * Certain "cheap" objects should be created in "piles"
 * Some objects can be sold at a "discount" (in small piles)
 */
unsafe extern "C" fn mass_produce(mut o_ptr: *mut object_type) {
    let mut size: libc::c_int = 1 as libc::c_int;
    let mut discount: libc::c_int = 0 as libc::c_int;
    let mut cost: s32b = object_value(o_ptr);
    /* Analyze the type */
    match (*o_ptr).tval as libc::c_int {
        80 | 77 | 39 => {
            /* Food, Flasks, and Lites */
            if cost as libc::c_long <= 5 as libc::c_long {
                size += mass_roll(3 as libc::c_int, 5 as libc::c_int)
            }
            if cost as libc::c_long <= 20 as libc::c_long {
                size += mass_roll(3 as libc::c_int, 5 as libc::c_int)
            }
        }
        71 | 72 | 70 => {
            if cost as libc::c_long <= 60 as libc::c_long {
                size += mass_roll(3 as libc::c_int, 5 as libc::c_int)
            }
            if cost as libc::c_long <= 240 as libc::c_long {
                size += mass_roll(1 as libc::c_int, 5 as libc::c_int)
            }
        }
        112 | 113 | 114 | 115 | 111 => {
            if cost as libc::c_long <= 50 as libc::c_long {
                size += mass_roll(2 as libc::c_int, 3 as libc::c_int)
            }
            if cost as libc::c_long <= 500 as libc::c_long {
                size += mass_roll(1 as libc::c_int, 3 as libc::c_int)
            }
        }
        36 | 37 | 34 | 31 | 30 | 35 | 32 | 33 | 23 | 24 | 22 | 21 | 20 | 19 =>
        {
            if !((*o_ptr).name2 != 0) {
                if cost as libc::c_long <= 10 as libc::c_long {
                    size += mass_roll(3 as libc::c_int, 5 as libc::c_int)
                }
                if cost as libc::c_long <= 100 as libc::c_long {
                    size += mass_roll(3 as libc::c_int, 5 as libc::c_int)
                }
            }
        }
        5 | 16 | 17 | 18 => {
            if cost as libc::c_long <= 5 as libc::c_long {
                size += mass_roll(5 as libc::c_int, 5 as libc::c_int)
            }
            if cost as libc::c_long <= 50 as libc::c_long {
                size += mass_roll(5 as libc::c_int, 5 as libc::c_int)
            }
            if cost as libc::c_long <= 500 as libc::c_long {
                size += mass_roll(5 as libc::c_int, 5 as libc::c_int)
            }
        }
        66 | 65 | 55 => {
            /* Because many rods (and a few wands and staffs) are useful mainly
		 * in quantity, the Black Market will occasionally have a bunch of
		 * one kind. -LM- */
            if (cost as libc::c_long) < 1601 as libc::c_long {
                size += mass_roll(1 as libc::c_int, 5 as libc::c_int)
            } else if (cost as libc::c_long) < 3201 as libc::c_long {
                size += mass_roll(1 as libc::c_int, 3 as libc::c_int)
            }
        }
        _ => { }
    }
    /* Pick a discount */
    if cost < 5 as libc::c_int {
        discount = 0 as libc::c_int
    } else if Rand_div(25 as libc::c_int) == 0 as libc::c_int {
        discount = 25 as libc::c_int
    } else if Rand_div(150 as libc::c_int) == 0 as libc::c_int {
        discount = 50 as libc::c_int
    } else if Rand_div(300 as libc::c_int) == 0 as libc::c_int {
        discount = 75 as libc::c_int
    } else if Rand_div(500 as libc::c_int) == 0 as libc::c_int {
        discount = 90 as libc::c_int
    }
    if (*o_ptr).art_name != 0 {
        if cheat_peek as libc::c_int != 0 && discount != 0 {
            msg_print(b"No discount on random artifacts.\x00" as *const u8 as
                          *const libc::c_char);
        }
        discount = 0 as libc::c_int
    }
    /* Save the discount */
    (*o_ptr).discount = discount as byte_hack;
    /* Save the total pile size */
    (*o_ptr).number =
        (size - size * discount / 100 as libc::c_int) as byte_hack;
}
/*
 * Determine if a store item can "absorb" another item
 *
 * See "object_similar()" for the same function for the "player"
 */
unsafe extern "C" fn store_object_similar(mut o_ptr: *mut object_type,
                                          mut j_ptr: *mut object_type)
 -> bool_ {
    /* Hack -- Identical items cannot be stacked */
    if o_ptr == j_ptr { return 0 as libc::c_int as bool_ }
    /* Different objects cannot be stacked */
    if (*o_ptr).k_idx as libc::c_int != (*j_ptr).k_idx as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Different charges (etc) cannot be stacked, unless wands or rods. */
    if (*o_ptr).pval != (*j_ptr).pval &&
           (*o_ptr).tval as libc::c_int != 65 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Require many identical values */
    if (*o_ptr).pval2 as libc::c_int != (*j_ptr).pval2 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).pval3 != (*j_ptr).pval3 { return 0 as libc::c_int as bool_ }
    /* Require many identical values */
    if (*o_ptr).to_h as libc::c_int != (*j_ptr).to_h as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).to_d as libc::c_int != (*j_ptr).to_d as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).to_a as libc::c_int != (*j_ptr).to_a as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Require identical "artifact" names */
    if (*o_ptr).name1 as libc::c_int != (*j_ptr).name1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Require identical "ego-item" names */
    if (*o_ptr).name2 as libc::c_int != (*j_ptr).name2 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Require identical "ego-item" names */
    if (*o_ptr).name2b as libc::c_int != (*j_ptr).name2b as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Random artifacts don't stack !*/
    if (*o_ptr).art_name as libc::c_int != 0 ||
           (*j_ptr).art_name as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Identical art_flags! */
    if (*o_ptr).art_flags1 != (*j_ptr).art_flags1 ||
           (*o_ptr).art_flags2 != (*j_ptr).art_flags2 ||
           (*o_ptr).art_flags3 != (*j_ptr).art_flags3 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Never stack "powerful" items */
    if (*o_ptr).xtra1 as libc::c_int != 0 ||
           (*j_ptr).xtra1 as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).tval as libc::c_int == 39 as libc::c_int {
        /* Require identical "turns of light" */
        if (*o_ptr).timeout as libc::c_int != (*j_ptr).timeout as libc::c_int
           {
            return 0 as libc::c_int as bool_
        }
    } else if (*o_ptr).timeout as libc::c_int != 0 ||
                  (*j_ptr).timeout as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Never stack recharging items */
    /* Require many identical values */
    if (*o_ptr).ac as libc::c_int != (*j_ptr).ac as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).dd as libc::c_int != (*j_ptr).dd as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).ds as libc::c_int != (*j_ptr).ds as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Never stack chests */
    if (*o_ptr).tval as libc::c_int == 7 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Require matching discounts */
    if (*o_ptr).discount as libc::c_int != (*j_ptr).discount as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* They match, so they must be similar */
    return 1 as libc::c_int as bool_;
}
/*
 * Allow a store item to absorb another item
 */
unsafe extern "C" fn store_object_absorb(mut o_ptr: *mut object_type,
                                         mut j_ptr: *mut object_type) {
    let mut total: libc::c_int =
        (*o_ptr).number as libc::c_int + (*j_ptr).number as libc::c_int;
    /* Combine quantity, lose excess items */
    (*o_ptr).number =
        if total > 99 as libc::c_int { 99 as libc::c_int } else { total } as
            byte_hack;
    /* Hack -- if wands are stacking, combine the charges. -LM- */
    if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
        (*o_ptr).pval += (*j_ptr).pval
    };
}
/*
 * Check to see if the shop will be carrying too many objects	-RAK-
 * Note that the shop, just like a player, will not accept things
 * it cannot hold.	Before, one could "nuke" potions this way.
 */
unsafe extern "C" fn store_check_num(mut o_ptr: *mut object_type) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    /* Free space is always usable */
    if ((*st_ptr).stock_num as libc::c_int) <
           (*st_ptr).stock_size as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* The "home" acts like the player */
    if cur_store_num == 7 as libc::c_int ||
           (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as libc::c_long
               & 0x400 as libc::c_long != 0 {
        /* Check all the items */
        i = 0 as libc::c_int;
        while i < (*st_ptr).stock_num as libc::c_int {
            /* Get the existing item */
            j_ptr =
                &mut *(*st_ptr).stock.offset(i as isize) as *mut object_type;
            /* Can the new object be combined with the old one? */
            if object_similar(j_ptr, o_ptr) != 0 {
                return 1 as libc::c_int as bool_
            }
            i += 1
        }
    } else {
        /* Normal stores do special stuff */
        /* Check all the items */
        i = 0 as libc::c_int;
        while i < (*st_ptr).stock_num as libc::c_int {
            /* Get the existing item */
            j_ptr =
                &mut *(*st_ptr).stock.offset(i as isize) as *mut object_type;
            /* Can the new object be combined with the old one? */
            if store_object_similar(j_ptr, o_ptr) != 0 {
                return 1 as libc::c_int as bool_
            }
            i += 1
        }
    }
    /* But there was no room at the inn... */
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn is_blessed(mut o_ptr: *mut object_type) -> bool_ {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    object_flags_known(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                       &mut esp);
    if f3 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/*
 * Determine if the current store will purchase the given item
 *
 * Note that a shop-keeper must refuse to buy "worthless" items
 */
unsafe extern "C" fn store_will_buy(mut o_ptr: *mut object_type) -> bool_ {
    /* Hack -- The Home is simple */
    if cur_store_num == 7 as libc::c_int { return 1 as libc::c_int as bool_ }
    if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as libc::c_long &
           0x400 as libc::c_long != 0 {
        return 1 as libc::c_int as bool_
    }
    /* XXX XXX XXX Ignore "worthless" items */
    if object_value(o_ptr) <= 0 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Lua can define things to buy */
    if process_hooks_ret(48 as libc::c_int,
                         b"d\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"(d,s,O)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         (*st_ptr).st_idx as libc::c_int,
                         st_name.offset((*st_info.offset((*st_ptr).st_idx as
                                                             isize)).name as
                                            isize), o_ptr) != 0 {
        return process_hooks_return[0 as libc::c_int as usize].num as bool_
    }
    /* Assume not okay */
    return 0 as libc::c_int as bool_;
}
/*
 * Add the item "o_ptr" to the inventory of the "Home"
 *
 * In all cases, return the slot (or -1) where the object was placed
 *
 * Note that this is a hacked up version of "inven_carry()".
 *
 * Also note that it may not correctly "adapt" to "knowledge" bacoming
 * known, the player may have to pick stuff up and drop it again.
 */
unsafe extern "C" fn home_carry(mut o_ptr: *mut object_type) -> libc::c_int {
    let mut slot: libc::c_int = 0;
    let mut value: s32b = 0;
    let mut j_value: s32b = 0;
    let mut i: libc::c_int = 0;
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    /* Check each existing item (try to combine) */
    slot = 0 as libc::c_int;
    while slot < (*st_ptr).stock_num as libc::c_int {
        /* Get the existing item */
        j_ptr =
            &mut *(*st_ptr).stock.offset(slot as isize) as *mut object_type;
        /* The home acts just like the player */
        if object_similar(j_ptr, o_ptr) != 0 {
            /* Save the new number of items */
            object_absorb(j_ptr, o_ptr);
            /* All done */
            return slot
        }
        slot += 1
    }
    /* No space? */
    if (*st_ptr).stock_num as libc::c_int >=
           (*st_ptr).stock_size as libc::c_int {
        return -(1 as libc::c_int)
    }
    /* Determine the "value" of the item */
    value = object_value(o_ptr);
    let mut current_block_13: u64;
    /* Check existing slots to see if we must "slide" */
    slot = 0 as libc::c_int;
    while slot < (*st_ptr).stock_num as libc::c_int {
        /* Get that item */
        j_ptr =
            &mut *(*st_ptr).stock.offset(slot as isize) as *mut object_type;
        /* Objects sort by decreasing type */
        if (*o_ptr).tval as libc::c_int > (*j_ptr).tval as libc::c_int {
            break ;
        }
        if !(((*o_ptr).tval as libc::c_int) < (*j_ptr).tval as libc::c_int) {
            /* Can happen in the home */
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
                    /* Objects in the home can be unknown */
                    if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0
                           ||
                           (*k_info.offset((*o_ptr).k_idx as isize)).easy_know
                               as libc::c_int != 0 &&
                               (*k_info.offset((*o_ptr).k_idx as isize)).aware
                                   as libc::c_int != 0 {
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
                        /*
		 * Hack:  otherwise identical rods sort by
		 * increasing recharge time --dsb
		 */
                        if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
                            if ((*o_ptr).timeout as libc::c_int) <
                                   (*j_ptr).timeout as libc::c_int {
                                break ;
                            }
                            if (*o_ptr).timeout as libc::c_int >
                                   (*j_ptr).timeout as libc::c_int {
                                current_block_13 = 2979737022853876585;
                            } else {
                                current_block_13 = 14648156034262866959;
                            }
                        } else { current_block_13 = 14648156034262866959; }
                        match current_block_13 {
                            2979737022853876585 => { }
                            _ => {
                                /* Objects sort by decreasing value */
                                j_value = object_value(j_ptr);
                                if value > j_value { break ; }
                                (value) < j_value;
                            }
                        }
                    }
                }
            }
        }
        slot += 1
    }
    /* Slide the others up */
    i = (*st_ptr).stock_num as libc::c_int;
    while i > slot {
        *(*st_ptr).stock.offset(i as isize) =
            *(*st_ptr).stock.offset((i - 1 as libc::c_int) as isize);
        i -= 1
    }
    /* More stuff now */
    (*st_ptr).stock_num = (*st_ptr).stock_num.wrapping_add(1);
    /* Insert the new item */
    *(*st_ptr).stock.offset(slot as isize) = *o_ptr;
    /* Return the location */
    return slot;
}
/*
 * Add the item "o_ptr" to a real stores inventory.
 *
 * If the item is "worthless", it is thrown away (except in the home).
 *
 * If the item cannot be combined with an object already in the inventory,
 * make a new slot for it, and calculate its "per item" price.	Note that
 * this price will be negative, since the price will not be "fixed" yet.
 * Adding an item to a "fixed" price stack will not change the fixed price.
 *
 * In all cases, return the slot (or -1) where the object was placed
 */
unsafe extern "C" fn store_carry(mut o_ptr: *mut object_type) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut value: s32b = 0;
    let mut j_value: s32b = 0;
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    /* Evaluate the object */
    value = object_value(o_ptr);
    /* Cursed/Worthless items "disappear" when sold */
    if value <= 0 as libc::c_int { return -(1 as libc::c_int) }
    /* All store items are fully *identified* */
    (*o_ptr).ident =
        ((*o_ptr).ident as libc::c_int | 0x20 as libc::c_int) as byte_hack;
    /* Erase the inscription */
    (*o_ptr).note = 0 as libc::c_int as u16b;
    /* Check each existing item (try to combine) */
    slot = 0 as libc::c_int;
    while slot < (*st_ptr).stock_num as libc::c_int {
        /* Get the existing item */
        j_ptr =
            &mut *(*st_ptr).stock.offset(slot as isize) as *mut object_type;
        /* Can the existing items be incremented? */
        if store_object_similar(j_ptr, o_ptr) != 0 {
            /* Hack -- extra items disappear */
            store_object_absorb(j_ptr, o_ptr);
            /* All done */
            return slot
        }
        slot += 1
    }
    /* No space? */
    if (*st_ptr).stock_num as libc::c_int >=
           (*st_ptr).stock_size as libc::c_int {
        return -(1 as libc::c_int)
    }
    let mut current_block_17: u64;
    /* Check existing slots to see if we must "slide" */
    slot = 0 as libc::c_int;
    while slot < (*st_ptr).stock_num as libc::c_int {
        /* Get that item */
        j_ptr =
            &mut *(*st_ptr).stock.offset(slot as isize) as *mut object_type;
        /* Objects sort by decreasing type */
        if (*o_ptr).tval as libc::c_int > (*j_ptr).tval as libc::c_int {
            break ;
        }
        if !(((*o_ptr).tval as libc::c_int) < (*j_ptr).tval as libc::c_int) {
            /* Objects sort by increasing sval */
            if ((*o_ptr).sval as libc::c_int) < (*j_ptr).sval as libc::c_int {
                break ;
            }
            if !((*o_ptr).sval as libc::c_int > (*j_ptr).sval as libc::c_int)
               {
                /*
		 * Hack:  otherwise identical rods sort by
		 * increasing recharge time --dsb
		 */
                if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
                    if ((*o_ptr).timeout as libc::c_int) <
                           (*j_ptr).timeout as libc::c_int {
                        break ;
                    }
                    if (*o_ptr).timeout as libc::c_int >
                           (*j_ptr).timeout as libc::c_int {
                        current_block_17 = 9606288038608642794;
                    } else { current_block_17 = 6669252993407410313; }
                } else { current_block_17 = 6669252993407410313; }
                match current_block_17 {
                    9606288038608642794 => { }
                    _ => {
                        /* Evaluate that slot */
                        j_value = object_value(j_ptr);
                        /* Objects sort by decreasing value */
                        if value > j_value { break ; }
                        (value) < j_value;
                    }
                }
            }
        }
        slot += 1
    }
    /* Slide the others up */
    i = (*st_ptr).stock_num as libc::c_int;
    while i > slot {
        *(*st_ptr).stock.offset(i as isize) =
            *(*st_ptr).stock.offset((i - 1 as libc::c_int) as isize);
        i -= 1
    }
    /* More stuff now */
    (*st_ptr).stock_num = (*st_ptr).stock_num.wrapping_add(1);
    /* Insert the new item */
    *(*st_ptr).stock.offset(slot as isize) = *o_ptr;
    /* Return the location */
    return slot;
}
/*
 * Increase, by a given amount, the number of a certain item
 * in a certain store.	This can result in zero items.
 */
unsafe extern "C" fn store_item_increase(mut item: libc::c_int,
                                         mut num: libc::c_int) {
    let mut cnt: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Get the item */
    o_ptr = &mut *(*st_ptr).stock.offset(item as isize) as *mut object_type;
    /* Verify the number */
    cnt = (*o_ptr).number as libc::c_int + num;
    if cnt > 255 as libc::c_int {
        cnt = 255 as libc::c_int
    } else if cnt < 0 as libc::c_int { cnt = 0 as libc::c_int }
    num = cnt - (*o_ptr).number as libc::c_int;
    /* Save the new number */
    (*o_ptr).number = ((*o_ptr).number as libc::c_int + num) as byte_hack;
}
/*
 * Remove a slot if it is empty
 */
unsafe extern "C" fn store_item_optimize(mut item: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Get the item */
    o_ptr = &mut *(*st_ptr).stock.offset(item as isize) as *mut object_type;
    /* Must exist */
    if (*o_ptr).k_idx == 0 { return }
    /* Must have no items */
    if (*o_ptr).number != 0 { return }
    /* One less item */
    (*st_ptr).stock_num = (*st_ptr).stock_num.wrapping_sub(1);
    /* Slide everyone */
    j = item;
    while j < (*st_ptr).stock_num as libc::c_int {
        *(*st_ptr).stock.offset(j as isize) =
            *(*st_ptr).stock.offset((j + 1 as libc::c_int) as isize);
        j += 1
    }
    /* Nuke the final slot */
    object_wipe(&mut *(*st_ptr).stock.offset(j as isize));
}
/*
 * This function will keep 'crap' out of the black market.
 * Crap is defined as any item that is "available" elsewhere
 * Based on a suggestion by "Lee Vogt" <lvogt@cig.mcel.mot.com>
 */
unsafe extern "C" fn black_market_crap(mut o_ptr: *mut object_type) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* Ego items are never crap */
    if (*o_ptr).name2 != 0 { return 0 as libc::c_int as bool_ }
    /* Good items are never crap */
    if (*o_ptr).to_a as libc::c_int > 0 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).to_h as libc::c_int > 0 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).to_d as libc::c_int > 0 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Check all stores */
    i = 0 as libc::c_int;
    while i < max_st_idx as libc::c_int {
        if !(i == 7 as libc::c_int) {
            if !((*st_info.offset(i as isize)).flags1 as libc::c_long &
                     0x400 as libc::c_long != 0) {
                /* Check every item in the store */
                j = 0 as libc::c_int;
                while j <
                          (*(*town_info.offset((*p_ptr).town_num as
                                                   isize)).store.offset(i as
                                                                            isize)).stock_num
                              as libc::c_int {
                    let mut j_ptr: *mut object_type =
                        &mut *(*(*town_info.offset((*p_ptr).town_num as
                                                       isize)).store.offset(i
                                                                                as
                                                                                isize)).stock.offset(j
                                                                                                         as
                                                                                                         isize)
                            as *mut object_type;
                    /* Duplicate item "type", assume crappy */
                    if (*o_ptr).k_idx as libc::c_int ==
                           (*j_ptr).k_idx as libc::c_int {
                        return 1 as libc::c_int as bool_
                    }
                    j += 1
                }
            }
        }
        i += 1
    }
    /* Assume okay */
    return 0 as libc::c_int as bool_;
}
/*
 * Attempt to delete (some of) a random item from the store
 * Hack -- we attempt to "maintain" piles of items when possible.
 */
unsafe extern "C" fn store_delete() {
    let mut what: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    /* Pick a random slot */
    what = Rand_div((*st_ptr).stock_num as s32b);
    /* Determine how many items are here */
    num = (*(*st_ptr).stock.offset(what as isize)).number as libc::c_int;
    /* Hack -- sometimes, only destroy half the items */
    if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        num = (num + 1 as libc::c_int) / 2 as libc::c_int
    }
    /* Hack -- sometimes, only destroy a single item */
    if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        num = 1 as libc::c_int
    }
    /* Hack -- decrement the maximum timeouts and total charges of rods and wands. -LM- */
    if (*(*st_ptr).stock.offset(what as isize)).tval as libc::c_int ==
           65 as libc::c_int {
        let ref mut fresh0 = (*(*st_ptr).stock.offset(what as isize)).pval;
        *fresh0 -=
            num * (*(*st_ptr).stock.offset(what as isize)).pval /
                (*(*st_ptr).stock.offset(what as isize)).number as libc::c_int
    }
    /* Actually destroy (part of) the item */
    store_item_increase(what, -num);
    store_item_optimize(what);
}
/* Analyze store flags and return a level */
#[no_mangle]
pub unsafe extern "C" fn return_level() -> libc::c_int {
    let mut sti_ptr: *mut store_info_type =
        &mut *st_info.offset((*st_ptr).st_idx as isize) as
            *mut store_info_type;
    let mut level: libc::c_int = 0;
    if (*sti_ptr).flags1 as libc::c_long & 0x100 as libc::c_long != 0 {
        level = 0 as libc::c_int
    } else {
        level =
            1 as libc::c_int +
                Rand_div(1 as libc::c_int + 5 as libc::c_int -
                             1 as libc::c_int)
    }
    if (*sti_ptr).flags1 as libc::c_long & 0x1 as libc::c_long != 0 {
        level += dun_level as libc::c_int
    }
    if (*sti_ptr).flags1 as libc::c_long & 0x2 as libc::c_long != 0 {
        level += 5 as libc::c_int + Rand_div(5 as libc::c_int)
    }
    if (*sti_ptr).flags1 as libc::c_long & 0x4 as libc::c_long != 0 {
        level += 25 as libc::c_int + Rand_div(25 as libc::c_int)
    }
    if (*sti_ptr).flags1 as libc::c_long & 0x8 as libc::c_long != 0 {
        level += 45 as libc::c_int + Rand_div(45 as libc::c_int)
    }
    if (*sti_ptr).flags1 as libc::c_long & 0x80 as libc::c_long != 0 {
        level += (*p_ptr).lev as libc::c_int
    }
    return level;
}
/* Is it an ok object ? */
static mut store_tval: libc::c_int = 0 as libc::c_int;
static mut store_level: libc::c_int = 0 as libc::c_int;
/*
 * Hack -- determine if a template is "good"
 */
unsafe extern "C" fn kind_is_storeok(mut k_idx: libc::c_int) -> bool_ {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(k_idx as isize) as *mut object_kind;
    if (*k_info.offset(k_idx as isize)).flags3 as libc::c_long &
           0x8000 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*k_info.offset(k_idx as isize)).flags3 as libc::c_long &
           0x800 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    if kind_is_legal(k_idx) == 0 { return 0 as libc::c_int as bool_ }
    if (*k_ptr).tval as libc::c_int != store_tval {
        return 0 as libc::c_int as bool_
    }
    if ((*k_ptr).level as libc::c_int) < store_level / 2 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    return 1 as libc::c_int as bool_;
}
/*
 * Creates a random item and gives it to a store
 * This algorithm needs to be rethought.  A lot.
 *
 * Note -- the "level" given to "obj_get_num()" is a "favored"
 * level, that is, there is a much higher chance of getting
 * items with a level approaching that of the given level...
 *
 * Should we check for "permission" to have the given item?
 */
unsafe extern "C" fn store_create() {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut tries: libc::c_int = 0;
    let mut level: libc::c_int = 0 as libc::c_int;
    let mut chance: libc::c_int = 0;
    let mut item: libc::c_int = 0;
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
    let mut obj_all_done: bool_ = 0 as libc::c_int as bool_;
    /* Paranoia -- no room left */
    if (*st_ptr).stock_num as libc::c_int >=
           (*st_ptr).stock_size as libc::c_int {
        return
    }
    let mut current_block_50: u64;
    /* Hack -- consider up to four items */
    tries = 0 as libc::c_int;
    while tries < 4 as libc::c_int {
        obj_all_done = 0 as libc::c_int as bool_;
        /* Lua can define things to buy */
        if process_hooks_ret(47 as libc::c_int,
                             b"O\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                             b"(d,s,d)\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             (*st_ptr).st_idx as libc::c_int,
                             st_name.offset((*st_info.offset((*st_ptr).st_idx
                                                                 as
                                                                 isize)).name
                                                as isize), return_level()) !=
               0 {
            obj_all_done = 1 as libc::c_int as bool_;
            q_ptr = process_hooks_return[0 as libc::c_int as usize].o_ptr;
            current_block_50 = 5330834795799507926;
        } else if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as
                      libc::c_long & 0x80 as libc::c_long != 0 {
            let mut theme: obj_theme =
                obj_theme{treasure: 0, combat: 0, magic: 0, tools: 0,};
            /* Black Market */
            /* No themes */
            theme.treasure = 100 as libc::c_int as byte_hack;
            theme.combat = 100 as libc::c_int as byte_hack;
            theme.magic = 100 as libc::c_int as byte_hack;
            theme.tools = 100 as libc::c_int as byte_hack;
            init_match_theme(theme);
            /*
			 * Even in Black Markets, illegal objects can be
			 * problematic -- Oxymoron?
			 */
            get_obj_num_hook =
                Some(kind_is_legal as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_);
            /* Rebuild the allocation table */
            get_obj_num_prep();
            /* Pick a level for object/magic */
            level = return_level();
            /* Random item (usually of given level) */
            i = get_obj_num(level) as libc::c_int;
            /* Invalidate the cached allocation table */
            alloc_kind_table_valid = 0 as libc::c_int as bool_;
            /* Handle failure */
            if i == 0 {
                current_block_50 = 7502529970979898288;
            } else { current_block_50 = 5330834795799507926; }
        } else {
            /* Normal Store */
            /* Hack -- Pick an item to sell */
            item =
                Rand_div((*st_info.offset((*st_ptr).st_idx as
                                              isize)).table_num as s32b);
            i =
                (*st_info.offset((*st_ptr).st_idx as
                                     isize)).table[item as
                                                       usize][0 as libc::c_int
                                                                  as usize] as
                    libc::c_int;
            chance =
                (*st_info.offset((*st_ptr).st_idx as
                                     isize)).table[item as
                                                       usize][1 as libc::c_int
                                                                  as usize] as
                    libc::c_int;
            /* Don't allow k_info artifacts */
            if i <= 10000 as libc::c_int &&
                   (*k_info.offset(i as isize)).flags3 as libc::c_long &
                       0x8000 as libc::c_long != 0 {
                current_block_50 = 7502529970979898288;
            } else if !(Rand_div(100 as libc::c_int) < chance) {
                current_block_50 = 7502529970979898288;
            } else {
                /* Does it passes the rarity check ? */
                /* Hack -- fake level for apply_magic() */
                level = return_level();
                /* Hack -- i > 10000 means it's a tval and all svals are allowed */
                if i > 10000 as libc::c_int {
                    let mut theme_0: obj_theme =
                        obj_theme{treasure: 0,
                                  combat: 0,
                                  magic: 0,
                                  tools: 0,};
                    /* No themes */
                    theme_0.treasure = 100 as libc::c_int as byte_hack;
                    theme_0.combat = 100 as libc::c_int as byte_hack;
                    theme_0.magic = 100 as libc::c_int as byte_hack;
                    theme_0.tools = 100 as libc::c_int as byte_hack;
                    init_match_theme(theme_0);
                    /* Activate restriction */
                    get_obj_num_hook =
                        Some(kind_is_storeok as
                                 unsafe extern "C" fn(_: libc::c_int)
                                     -> bool_);
                    store_tval = i - 10000 as libc::c_int;
                    /* Do we forbid too shallow items ? */
                    if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as
                           libc::c_long & 0x200 as libc::c_long != 0 {
                        store_level = level
                    } else { store_level = 0 as libc::c_int }
                    /* Prepare allocation table */
                    get_obj_num_prep();
                    /* Get it ! */
                    i = get_obj_num(level) as libc::c_int;
                    /* Invalidate the cached allocation table */
                    alloc_kind_table_valid = 0 as libc::c_int as bool_
                }
                if i == 0 {
                    current_block_50 = 7502529970979898288;
                } else { current_block_50 = 5330834795799507926; }
            }
        }
        match current_block_50 {
            5330834795799507926 =>
            /* Only if not already done */
            {
                if obj_all_done == 0 {
                    /* Don't allow k_info artifacts */
                    if (*k_info.offset(i as isize)).flags3 as libc::c_long &
                           0x8000 as libc::c_long != 0 {
                        current_block_50 = 7502529970979898288;
                    } else if (*k_info.offset(i as isize)).flags3 as
                                  libc::c_long & 0x800 as libc::c_long != 0 {
                        current_block_50 = 7502529970979898288;
                    } else {
                        /* Don't allow artifacts */
                        /* Get local object */
                        q_ptr = &mut forge;
                        /* Create a new object of the chosen kind */
                        object_prep(q_ptr, i);
                        /* Apply some "low-level" magic (no artifacts) */
                        apply_magic(q_ptr, level, 0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_,
                                    0 as libc::c_int as bool_);
                        /* Hack -- Charge lite's */
                        if (*q_ptr).tval as libc::c_int == 39 as libc::c_int {
                            let mut f1: u32b = 0;
                            let mut f2: u32b = 0;
                            let mut f3: u32b = 0;
                            let mut f4: u32b = 0;
                            let mut f5: u32b = 0;
                            let mut esp: u32b = 0;
                            object_flags(q_ptr, &mut f1, &mut f2, &mut f3,
                                         &mut f4, &mut f5, &mut esp);
                            if f4 as libc::c_long & 0x10000000 as libc::c_long
                                   != 0 {
                                (*q_ptr).timeout =
                                    (*k_info.offset((*q_ptr).k_idx as
                                                        isize)).pval2 as s16b
                            }
                        }
                        current_block_50 = 1724319918354933278;
                    }
                } else { current_block_50 = 1724319918354933278; }
                match current_block_50 {
                    7502529970979898288 => { }
                    _ => {
                        /* The item is "known" */
                        object_known(q_ptr);
                        /* Mark it storebought */
                        (*q_ptr).ident =
                            ((*q_ptr).ident as libc::c_int |
                                 0x10 as libc::c_int) as byte_hack;
                        /* Mega-Hack -- no chests in stores */
                        if !((*q_ptr).tval as libc::c_int == 7 as libc::c_int)
                           {
                            /* Prune the black market */
                            if (*st_info.offset((*st_ptr).st_idx as
                                                    isize)).flags1 as
                                   libc::c_long & 0x80 as libc::c_long != 0 {
                                /* Hack -- No "crappy" items */
                                if black_market_crap(q_ptr) != 0 {
                                    current_block_50 = 7502529970979898288;
                                } else if object_value(q_ptr) <
                                              10 as libc::c_int {
                                    current_block_50 = 7502529970979898288;
                                } else {
                                    current_block_50 = 4888910987971495881;
                                }
                            } else if object_value(q_ptr) <= 0 as libc::c_int
                             {
                                current_block_50 = 7502529970979898288;
                            } else { current_block_50 = 4888910987971495881; }
                            match current_block_50 {
                                7502529970979898288 => { }
                                _ => {
                                    /* Hack -- No "cheap" items */
                                    /* Prune normal stores */
                                    /* No "worthless" items */
                                    /* Mass produce and/or Apply discount */
                                    mass_produce(q_ptr);
                                    /* The charges an wands are per each, so multiply to get correct number */
                                    if obj_all_done == 0 &&
                                           (*q_ptr).tval as libc::c_int ==
                                               65 as libc::c_int {
                                        (*q_ptr).pval *=
                                            (*q_ptr).number as libc::c_int
                                    }
                                    /* Attempt to carry the (known) item */
                                    store_carry(q_ptr);
                                    break ;
                                }
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        tries += 1
    };
}
/*
 * Eliminate need to bargain if player has haggled well in the past
 */
unsafe extern "C" fn noneedtobargain(mut minprice: s32b) -> bool_ {
    let mut good: s32b = (*st_ptr).good_buy as s32b;
    let mut bad: s32b = (*st_ptr).bad_buy as s32b;
    /* Cheap items are "boring" */
    if (minprice as libc::c_long) < 10 as libc::c_long {
        return 1 as libc::c_int as bool_
    }
    /* Perfect haggling */
    if good == 32767 as libc::c_int { return 1 as libc::c_int as bool_ }
    /* Reward good haggles, punish bad haggles, notice price */
    if good >
           3 as libc::c_int * bad +
               (5 as libc::c_int + minprice / 50 as libc::c_int) {
        return 1 as libc::c_int as bool_
    }
    /* Return the flag */
    return 0 as libc::c_int as bool_;
}
/*
 * Update the bargain info
 */
unsafe extern "C" fn updatebargain(mut price: s32b, mut minprice: s32b) {
    /* Hack -- auto-haggle */
    if auto_haggle != 0 { return }
    /* Cheap items are "boring" */
    if (minprice as libc::c_long) < 10 as libc::c_long { return }
    /* Count the successful haggles */
    if price == minprice {
        /* Just count the good haggles */
        if ((*st_ptr).good_buy as libc::c_int) < 32767 as libc::c_int {
            (*st_ptr).good_buy += 1
        }
    } else if ((*st_ptr).bad_buy as libc::c_int) < 32767 as libc::c_int {
        (*st_ptr).bad_buy += 1
    };
}
/* Count the failed haggles */
/* Just count the bad haggles */
/*
 * Re-displays a single store entry
 */
unsafe extern "C" fn display_entry(mut pos: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut cur_col: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut x: s32b = 0;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut maxwid: libc::c_int = 75 as libc::c_int;
    /* Get the item */
    o_ptr = &mut *(*st_ptr).stock.offset(pos as isize) as *mut object_type;
    /* Get the "offset" */
    i = pos % 12 as libc::c_int;
    /* Label it, clear the line --(-- */
    strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
            b"%c) \x00" as *const u8 as *const libc::c_char, i + 'a' as i32);
    c_prt(get_item_letter_color(o_ptr), out_val.as_mut_ptr() as cptr,
          i + 6 as libc::c_int, 0 as libc::c_int);
    cur_col = 3 as libc::c_int;
    if show_store_graph != 0 {
        let mut a: byte_hack =
            if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
                random_artifacts[(*o_ptr).sval as usize].attr as libc::c_int
            } else if (*k_info.offset((*o_ptr).k_idx as isize)).flavor as
                          libc::c_int != 0 {
                misc_to_attr[(*k_info.offset((*o_ptr).k_idx as isize)).flavor
                                 as usize] as libc::c_int
            } else {
                (*k_info.offset((*o_ptr).k_idx as isize)).x_attr as
                    libc::c_int
            } as byte_hack;
        let mut c: libc::c_char =
            if (*k_info.offset((*o_ptr).k_idx as isize)).flavor as libc::c_int
                   != 0 {
                misc_to_char[(*k_info.offset((*o_ptr).k_idx as isize)).flavor
                                 as usize] as libc::c_int
            } else {
                (*k_info.offset((*o_ptr).k_idx as isize)).x_char as
                    libc::c_int
            } as libc::c_char;
        if (*o_ptr).k_idx == 0 { c = ' ' as i32 as libc::c_char }
        Term_draw(cur_col, i + 6 as libc::c_int, a, c);
        if use_bigtile != 0 {
            cur_col += 1;
            if a as libc::c_int & 0x80 as libc::c_int != 0 {
                Term_draw(cur_col, i + 6 as libc::c_int,
                          255 as libc::c_int as byte_hack,
                          255 as libc::c_int as libc::c_char);
            }
        }
        cur_col += 2 as libc::c_int
    }
    /* Describe an item in the home */
    if cur_store_num == 7 as libc::c_int ||
           (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as libc::c_long
               & 0x400 as libc::c_long != 0 {
        maxwid = 75 as libc::c_int;
        /* Leave room for weights, if necessary -DRS- */
        if show_weights != 0 { maxwid -= 10 as libc::c_int }
        /* Describe the object */
        object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                    3 as libc::c_int);
        o_name[maxwid as usize] = '\u{0}' as i32 as libc::c_char;
        c_put_str(tval_to_attr[(*o_ptr).tval as usize],
                  o_name.as_mut_ptr() as cptr, i + 6 as libc::c_int, cur_col);
        /* Show weights */
        if show_weights != 0 {
            /* Only show the weight of an individual item */
            let mut wgt: libc::c_int = (*o_ptr).weight;
            strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"%3d.%d lb\x00" as *const u8 as *const libc::c_char,
                    wgt / 10 as libc::c_int, wgt % 10 as libc::c_int);
            put_str(out_val.as_mut_ptr() as cptr, i + 6 as libc::c_int,
                    68 as libc::c_int);
        }
    } else {
        /* Describe an item (fully) in a store */
        let mut color: byte_hack = 1 as libc::c_int as byte_hack;
        /* Must leave room for the "price" */
        maxwid = 65 as libc::c_int;
        /* Leave room for weights, if necessary -DRS- */
        if show_weights != 0 { maxwid -= 7 as libc::c_int }
        /* Describe the object (fully) */
        object_desc_store(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                          3 as libc::c_int);
        o_name[maxwid as usize] = '\u{0}' as i32 as libc::c_char;
        c_put_str(tval_to_attr[(*o_ptr).tval as usize],
                  o_name.as_mut_ptr() as cptr, i + 6 as libc::c_int, cur_col);
        /* Show weights */
        if show_weights != 0 {
            /* Only show the weight of an individual item */
            let mut wgt_0: libc::c_int = (*o_ptr).weight;
            strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"%3d.%d\x00" as *const u8 as *const libc::c_char,
                    wgt_0 / 10 as libc::c_int, wgt_0 % 10 as libc::c_int);
            put_str(out_val.as_mut_ptr() as cptr, i + 6 as libc::c_int,
                    61 as libc::c_int);
        }
        /* Display a "fixed" cost */
        if (*o_ptr).ident as libc::c_int & 0x2 as libc::c_int != 0 {
            /* Extract the "minimum" price */
            x =
                price_item(o_ptr, (*ot_ptr).min_inflate as libc::c_int,
                           0 as libc::c_int as bool_);
            /* Can we buy one ? */
            if x > (*p_ptr).au { color = 8 as libc::c_int as byte_hack }
            /* Actually draw the price (not fixed) */
            strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"%9ld F\x00" as *const u8 as *const libc::c_char,
                    x as libc::c_long);
            c_put_str(color, out_val.as_mut_ptr() as cptr,
                      i + 6 as libc::c_int, 68 as libc::c_int);
        } else if auto_haggle != 0 {
            /* Display a "taxed" cost */
            /* Extract the "minimum" price */
            x =
                price_item(o_ptr, (*ot_ptr).min_inflate as libc::c_int,
                           0 as libc::c_int as bool_);
            /* Hack -- Apply Sales Tax if needed */
            if noneedtobargain(x) == 0 { x += x / 10 as libc::c_int }
            /* Can we buy one ? */
            if x > (*p_ptr).au { color = 8 as libc::c_int as byte_hack }
            /* Actually draw the price (with tax) */
            strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"%9ld  \x00" as *const u8 as *const libc::c_char,
                    x as libc::c_long);
            c_put_str(color, out_val.as_mut_ptr() as cptr,
                      i + 6 as libc::c_int, 68 as libc::c_int);
        } else {
            /* Display a "haggle" cost */
            /* Extrect the "maximum" price */
            x =
                price_item(o_ptr, (*ot_ptr).max_inflate as libc::c_int,
                           0 as libc::c_int as bool_);
            if x > (*p_ptr).au { color = 8 as libc::c_int as byte_hack }
            strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"%9ld  \x00" as *const u8 as *const libc::c_char,
                    x as libc::c_long);
            c_put_str(color, out_val.as_mut_ptr() as cptr,
                      i + 6 as libc::c_int, 68 as libc::c_int);
        }
    };
}
/* Can we buy one ? */
/* Actually draw the price (not fixed) */
/*
 * Displays a store's inventory 		-RAK-
 * All prices are listed as "per individual object".  -BEN-
 */
unsafe extern "C" fn display_inventory() {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    /* Display the next 12 items */
    k = 0 as libc::c_int;
    while k < 12 as libc::c_int {
        /* Do not display "dead" items */
        if store_top + k >= (*st_ptr).stock_num as libc::c_int { break ; }
        /* Display that line */
        display_entry(store_top + k);
        k += 1
    }
    /* Erase the extra lines and the "more" prompt */
    i = k;
    while i < 13 as libc::c_int {
        prt(b"\x00" as *const u8 as *const libc::c_char, i + 6 as libc::c_int,
            0 as libc::c_int);
        i += 1
    }
    /* Assume "no current page" */
    put_str(b"         \x00" as *const u8 as *const libc::c_char,
            5 as libc::c_int, 20 as libc::c_int);
    /* Visual reminder of "more items" */
    if (*st_ptr).stock_num as libc::c_int > 12 as libc::c_int {
        /* Show "more" reminder (after the last item) */
        prt(b"-more-\x00" as *const u8 as *const libc::c_char,
            k + 6 as libc::c_int, 3 as libc::c_int);
        /* Indicate the "current page" */
        put_str(format(b"(Page %d) \x00" as *const u8 as *const libc::c_char,
                       store_top / 12 as libc::c_int + 1 as libc::c_int) as
                    cptr, 5 as libc::c_int, 20 as libc::c_int);
    };
}
/*
 * Displays players gold					-RAK-
 */
#[no_mangle]
pub unsafe extern "C" fn store_prt_gold() {
    let mut out_val: [libc::c_char; 64] = [0; 64];
    prt(b"Gold Remaining: \x00" as *const u8 as *const libc::c_char,
        19 as libc::c_int, 53 as libc::c_int);
    strnfmt(out_val.as_mut_ptr(), 64 as libc::c_int as uint_hack,
            b"%9ld\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).au as libc::c_long);
    prt(out_val.as_mut_ptr() as cptr, 19 as libc::c_int, 68 as libc::c_int);
}
/*
 * Displays store (after clearing screen)		-RAK-
 */
#[no_mangle]
pub unsafe extern "C" fn display_store() {
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* Clear screen */
    Term_clear();
    /* The "Home" is special */
    if cur_store_num == 7 as libc::c_int {
        put_str(b"Your Home\x00" as *const u8 as *const libc::c_char,
                3 as libc::c_int, 30 as libc::c_int);
        /* Label the item descriptions */
        put_str(b"Item Description\x00" as *const u8 as *const libc::c_char,
                5 as libc::c_int, 3 as libc::c_int);
        /* If showing weights, show label */
        if show_weights != 0 {
            put_str(b"Weight\x00" as *const u8 as *const libc::c_char,
                    5 as libc::c_int, 70 as libc::c_int);
        }
    } else if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as
                  libc::c_long & 0x400 as libc::c_long != 0 {
        let mut store_name: cptr =
            st_name.offset((*st_info.offset(cur_store_num as isize)).name as
                               isize) as cptr;
        /* Show the name of the store */
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%s\x00" as *const u8 as *const libc::c_char, store_name);
        prt(buf.as_mut_ptr() as cptr, 3 as libc::c_int, 30 as libc::c_int);
        /* Label the item descriptions */
        put_str(b"Item Description\x00" as *const u8 as *const libc::c_char,
                5 as libc::c_int, 3 as libc::c_int);
        /* If showing weights, show label */
        if show_weights != 0 {
            put_str(b"Weight\x00" as *const u8 as *const libc::c_char,
                    5 as libc::c_int, 70 as libc::c_int);
        }
    } else {
        /* Normal stores */
        let mut store_name_0: cptr =
            st_name.offset((*st_info.offset(cur_store_num as isize)).name as
                               isize) as cptr;
        let mut owner_name: cptr =
            ow_name.offset((*ot_ptr).name as isize) as cptr;
        /* Put the owner name and race */
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%s\x00" as *const u8 as *const libc::c_char, owner_name);
        put_str(buf.as_mut_ptr() as cptr, 3 as libc::c_int,
                10 as libc::c_int);
        /* Show the max price in the store (above prices) */
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%s (%ld)\x00" as *const u8 as *const libc::c_char,
                store_name_0, (*ot_ptr).max_cost as libc::c_long);
        prt(buf.as_mut_ptr() as cptr, 3 as libc::c_int, 50 as libc::c_int);
        /* Label the item descriptions */
        put_str(b"Item Description\x00" as *const u8 as *const libc::c_char,
                5 as libc::c_int, 3 as libc::c_int);
        /* If showing weights, show label */
        if show_weights != 0 {
            put_str(b"Weight\x00" as *const u8 as *const libc::c_char,
                    5 as libc::c_int, 60 as libc::c_int);
        }
        /* Label the asking price (in stores) */
        put_str(b"Price\x00" as *const u8 as *const libc::c_char,
                5 as libc::c_int, 72 as libc::c_int);
    }
    /* Display the current gold */
    store_prt_gold();
    /* Draw in the inventory */
    display_inventory();
}
/*
 * Get the ID of a store item and return its value	-RAK-
 */
unsafe extern "C" fn get_stock(mut com_val: *mut libc::c_int, mut pmt: cptr,
                               mut i: libc::c_int, mut j: libc::c_int)
 -> libc::c_int {
    let mut command: libc::c_char = 0;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    /* Get the item index */
    if repeat_pull(com_val) != 0 {
        /* Verify the item */
        if *com_val >= i && *com_val <= j {
            /* Success */
            return 1 as libc::c_int
        }
    }
    /* Paranoia XXX XXX XXX */
    msg_print(0 as cptr);
    /* Assume failure */
    *com_val = -(1 as libc::c_int);
    /* Build the prompt */
    strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
            b"(Items %c-%c, ESC to exit) %s\x00" as *const u8 as
                *const libc::c_char, i + 'a' as i32, j + 'a' as i32, pmt);
    loop 
         /* Ask until done */
         {
        let mut k: libc::c_int = 0;
        /* Escape */
        if get_com(out_val.as_mut_ptr() as cptr, &mut command) == 0 {
            break ;
        }
        /* Convert */
        k =
            if *(*__ctype_b_loc()).offset(command as libc::c_int as isize) as
                   libc::c_int &
                   _ISlower as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                (command as libc::c_int) - 'a' as i32
            } else { -(1 as libc::c_int) };
        /* Legal responses */
        if k >= i && k <= j {
            *com_val = k;
            break ;
        } else {
            /* Oops */
            bell();
        }
    }
    /* Clear the prompt */
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
    /* Cancel */
    if command as libc::c_int == '\u{1b}' as i32 { return 0 as libc::c_int }
    repeat_push(*com_val);
    /* Success */
    return 1 as libc::c_int;
}
/*
 * Increase the insult counter and get angry if too many -RAK-
 */
unsafe extern "C" fn increase_insults() -> libc::c_int {
    /* Increase insults */
    (*st_ptr).insult_cur += 1;
    /* Become insulted */
    if (*st_ptr).insult_cur as libc::c_int >
           (*ot_ptr).insult_max as libc::c_int {
        /* Complain */
        say_comment_4();
        /* Reset insults */
        (*st_ptr).insult_cur = 0 as libc::c_int as s16b;
        (*st_ptr).good_buy = 0 as libc::c_int as s16b;
        (*st_ptr).bad_buy = 0 as libc::c_int as s16b;
        /* Open tomorrow */
        (*st_ptr).store_open =
            turn + 25000 as libc::c_int +
                (Rand_div(25000 as libc::c_int) + 1 as libc::c_int);
        /* Closed */
        return 1 as libc::c_int
    }
    /* Not closed */
    return 0 as libc::c_int;
}
/*
 * Decrease insults 				-RAK-
 */
unsafe extern "C" fn decrease_insults() {
    /* Decrease insults */
    if (*st_ptr).insult_cur != 0 { (*st_ptr).insult_cur -= 1 };
}
/*
 * Have insulted while haggling 			-RAK-
 */
unsafe extern "C" fn haggle_insults() -> libc::c_int {
    /* Increase insults */
    if increase_insults() != 0 { return 1 as libc::c_int }
    /* Display and flush insult */
    say_comment_5();
    /* Still okay */
    return 0 as libc::c_int;
}
/*
 * Mega-Hack -- Enable "increments"
 */
static mut allow_inc: bool_ = 0 as libc::c_int as bool_;
/*
 * Mega-Hack -- Last "increment" during haggling
 */
static mut last_inc: s32b = 0 as libc::c_long as s32b;
/*
 * Get a haggle
 */
unsafe extern "C" fn get_haggle(mut pmt: cptr, mut poffer: *mut s32b,
                                mut price: s32b, mut final_0: libc::c_int)
 -> libc::c_int {
    let mut i: s32b = 0;
    let mut p: cptr = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    /* Clear old increment if necessary */
    if allow_inc == 0 { last_inc = 0 as libc::c_long as s32b }
    /* Final offer */
    if final_0 != 0 {
        strnfmt(buf.as_mut_ptr(), 128 as libc::c_int as uint_hack,
                b"%s [accept] \x00" as *const u8 as *const libc::c_char, pmt);
    } else if last_inc < 0 as libc::c_int {
        strnfmt(buf.as_mut_ptr(), 128 as libc::c_int as uint_hack,
                b"%s [-%ld] \x00" as *const u8 as *const libc::c_char, pmt,
                if last_inc < 0 as libc::c_int { -last_inc } else { last_inc }
                    as libc::c_long);
    } else if last_inc > 0 as libc::c_int {
        strnfmt(buf.as_mut_ptr(), 128 as libc::c_int as uint_hack,
                b"%s [+%ld] \x00" as *const u8 as *const libc::c_char, pmt,
                if last_inc < 0 as libc::c_int { -last_inc } else { last_inc }
                    as libc::c_long);
    } else {
        /* Old (negative) increment, and not final */
        /* Old (positive) increment, and not final */
        /* Normal haggle */
        strnfmt(buf.as_mut_ptr(), 128 as libc::c_int as uint_hack,
                b"%s \x00" as *const u8 as *const libc::c_char, pmt);
    }
    /* Paranoia XXX XXX XXX */
    msg_print(0 as cptr);
    loop 
         /* Ask until done */
         /* Default */
         {
        strcpy(out_val.as_mut_ptr(),
               b"\x00" as *const u8 as *const libc::c_char);
        /* Ask the user for a response */
        if get_string(buf.as_mut_ptr() as cptr, out_val.as_mut_ptr(),
                      32 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        /* Skip leading spaces */
        p = out_val.as_mut_ptr() as cptr;
        while *p as libc::c_int == ' ' as i32 {
            /* loop */
            p = p.offset(1)
        }
        /* Empty response */
        if *p as libc::c_int == '\u{0}' as i32 {
            /* Accept current price */
            if final_0 != 0 {
                *poffer = price;
                last_inc = 0 as libc::c_long as s32b;
                break ;
            } else if allow_inc as libc::c_int != 0 && last_inc != 0 {
                *poffer += last_inc;
                break ;
            }
        } else {
            /* Use previous increment */
            /* Normal response */
            /* Extract a number */
            i = atol(p) as s32b;
            /* Handle "incremental" number */
            if *p as libc::c_int == '+' as i32 ||
                   *p as libc::c_int == '-' as i32 {
                /* Allow increments */
                if allow_inc != 0 {
                    /* Use the given "increment" */
                    *poffer += i;
                    last_inc = i;
                    break ;
                }
            } else {
                /* Handle normal number */
                /* Use the given "number" */
                *poffer = i;
                last_inc = 0 as libc::c_long as s32b;
                break ;
            }
        }
        /* Warning */
        msg_print(b"Invalid response.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
    }
    /* Success */
    return 1 as libc::c_int;
}
/*
 * Receive an offer (from the player)
 *
 * Return TRUE if offer is NOT okay
 */
unsafe extern "C" fn receive_offer(mut pmt: cptr, mut poffer: *mut s32b,
                                   mut last_offer: s32b,
                                   mut factor: libc::c_int, mut price: s32b,
                                   mut final_0: libc::c_int) -> bool_ {
    loop 
         /* Haggle till done */
         /* Get a haggle (or cancel) */
         {
        if get_haggle(pmt, poffer, price, final_0) == 0 {
            return 1 as libc::c_int as bool_
        }
        /* Acceptable offer */
        if *poffer * factor >= last_offer * factor { break ; }
        /* Insult, and check for kicked out */
        if haggle_insults() != 0 { return 1 as libc::c_int as bool_ }
        /* Reject offer (correctly) */
        *poffer = last_offer
    }
    /* Success */
    return 0 as libc::c_int as bool_;
}
/*
 * Haggling routine 				-RAK-
 *
 * Return TRUE if purchase is NOT successful
 */
unsafe extern "C" fn purchase_haggle(mut o_ptr: *mut object_type,
                                     mut price: *mut s32b) -> bool_ {
    let mut cur_ask: s32b = 0;
    let mut final_ask: s32b = 0;
    let mut last_offer: s32b = 0;
    let mut offer: s32b = 0;
    let mut x1: s32b = 0;
    let mut x2: s32b = 0;
    let mut x3: s32b = 0;
    let mut min_per: s32b = 0;
    let mut max_per: s32b = 0;
    let mut flag: libc::c_int = 0;
    let mut loop_flag: libc::c_int = 0;
    let mut noneed: libc::c_int = 0;
    let mut annoyed: libc::c_int = 0 as libc::c_int;
    let mut final_0: libc::c_int = 0 as libc::c_int;
    let mut cancel: bool_ = 0 as libc::c_int as bool_;
    let mut pmt: cptr = b"Asking\x00" as *const u8 as *const libc::c_char;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    *price = 0 as libc::c_int;
    /* Extract the starting offer and the final offer */
    cur_ask =
        price_item(o_ptr, (*ot_ptr).max_inflate as libc::c_int,
                   0 as libc::c_int as bool_);
    final_ask =
        price_item(o_ptr, (*ot_ptr).min_inflate as libc::c_int,
                   0 as libc::c_int as bool_);
    /* Determine if haggling is necessary */
    noneed = noneedtobargain(final_ask) as libc::c_int;
    /* No need to haggle */
    if noneed != 0 || auto_haggle as libc::c_int != 0 {
        /* No need to haggle */
        if noneed != 0 {
            /* Message summary */
            msg_print(b"You eventually agree upon the price.\x00" as *const u8
                          as *const libc::c_char);
            msg_print(0 as cptr);
        } else {
            /* No haggle option */
            /* Message summary */
            msg_print(b"You quickly agree upon the price.\x00" as *const u8 as
                          *const libc::c_char);
            msg_print(0 as cptr);
            final_ask += final_ask / 10 as libc::c_int
        }
        /* Apply Sales Tax */
        /* Final price */
        cur_ask = final_ask;
        /* Go to final offer */
        pmt = b"Final Offer\x00" as *const u8 as *const libc::c_char;
        final_0 = 1 as libc::c_int
    }
    /* Haggle for the whole pile */
    cur_ask *= (*o_ptr).number as libc::c_int;
    final_ask *= (*o_ptr).number as libc::c_int;
    /* Haggle parameters */
    min_per = (*ot_ptr).haggle_per as s32b;
    max_per = min_per * 3 as libc::c_int;
    /* Mega-Hack -- artificial "last offer" value */
    last_offer = object_value(o_ptr) * (*o_ptr).number as libc::c_int;
    last_offer =
        ((last_offer *
              (200 as libc::c_int - (*ot_ptr).max_inflate as libc::c_int)) as
             libc::c_long / 100 as libc::c_long) as s32b;
    if last_offer <= 0 as libc::c_int { last_offer = 1 as libc::c_int }
    /* No offer yet */
    offer = 0 as libc::c_int;
    /* No incremental haggling yet */
    allow_inc = 0 as libc::c_int as bool_;
    /* Haggle until done */
    flag = 0 as libc::c_int;
    while flag == 0 {
        loop_flag = 1 as libc::c_int;
        while flag == 0 && loop_flag != 0 {
            strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"%s :  %ld\x00" as *const u8 as *const libc::c_char, pmt,
                    cur_ask as libc::c_long);
            put_str(out_val.as_mut_ptr() as cptr, 1 as libc::c_int,
                    0 as libc::c_int);
            cancel =
                receive_offer(b"What do you offer? \x00" as *const u8 as
                                  *const libc::c_char, &mut offer, last_offer,
                              1 as libc::c_int, cur_ask, final_0);
            if cancel != 0 {
                flag = 1 as libc::c_int
            } else if offer > cur_ask {
                say_comment_6();
                offer = last_offer
            } else if offer == cur_ask {
                flag = 1 as libc::c_int;
                *price = offer
            } else { loop_flag = 0 as libc::c_int }
        }
        if flag == 0 {
            x1 =
                100 as libc::c_int * (offer - last_offer) /
                    (cur_ask - last_offer);
            if x1 < min_per {
                if haggle_insults() != 0 {
                    flag = 1 as libc::c_int;
                    cancel = 1 as libc::c_int as bool_
                }
            } else if x1 > max_per {
                x1 = x1 * 3 as libc::c_int / 4 as libc::c_int;
                if x1 < max_per { x1 = max_per }
            }
            x2 =
                x1 - 2 as libc::c_int +
                    Rand_div(1 as libc::c_int + (x1 + 2 as libc::c_int) -
                                 (x1 - 2 as libc::c_int));
            x3 =
                (((cur_ask - offer) * x2) as libc::c_long /
                     100 as libc::c_long + 1 as libc::c_int as libc::c_long)
                    as s32b;
            /* don't let the price go up */
            if x3 < 0 as libc::c_int { x3 = 0 as libc::c_int }
            cur_ask -= x3;
            /* Too little */
            if cur_ask < final_ask {
                final_0 = 1 as libc::c_int;
                cur_ask = final_ask;
                pmt = b"Final Offer\x00" as *const u8 as *const libc::c_char;
                annoyed += 1;
                if annoyed > 3 as libc::c_int {
                    increase_insults();
                    cancel = 1 as libc::c_int as bool_;
                    flag = 1 as libc::c_int
                }
            } else if offer >= cur_ask {
                flag = 1 as libc::c_int;
                *price = offer
            }
            if flag == 0 {
                last_offer = offer;
                allow_inc = 1 as libc::c_int as bool_;
                prt(b"\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int, 0 as libc::c_int);
                strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                        b"Your last offer: %ld\x00" as *const u8 as
                            *const libc::c_char, last_offer as libc::c_long);
                put_str(out_val.as_mut_ptr() as cptr, 1 as libc::c_int,
                        39 as libc::c_int);
                say_comment_2(cur_ask, annoyed);
            }
        }
    }
    /* Cancel */
    if cancel != 0 { return 1 as libc::c_int as bool_ }
    /* Update bargaining info */
    updatebargain(*price, final_ask);
    /* Do not cancel */
    return 0 as libc::c_int as bool_;
}
/*
 * Haggling routine 				-RAK-
 *
 * Return TRUE if purchase is NOT successful
 */
unsafe extern "C" fn sell_haggle(mut o_ptr: *mut object_type,
                                 mut price: *mut s32b) -> bool_ {
    let mut purse: s32b = 0;
    let mut cur_ask: s32b = 0;
    let mut final_ask: s32b = 0;
    let mut last_offer: s32b = 0 as libc::c_int;
    let mut offer: s32b = 0 as libc::c_int;
    let mut x1: s32b = 0;
    let mut x2: s32b = 0;
    let mut x3: s32b = 0;
    let mut min_per: s32b = 0;
    let mut max_per: s32b = 0;
    let mut flag: libc::c_int = 0;
    let mut loop_flag: libc::c_int = 0;
    let mut noneed: libc::c_int = 0;
    let mut annoyed: libc::c_int = 0 as libc::c_int;
    let mut final_0: libc::c_int = 0 as libc::c_int;
    let mut cancel: bool_ = 0 as libc::c_int as bool_;
    let mut pmt: cptr = b"Offer\x00" as *const u8 as *const libc::c_char;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    *price = 0 as libc::c_int;
    /* Obtain the starting offer and the final offer */
    cur_ask =
        price_item(o_ptr, (*ot_ptr).max_inflate as libc::c_int,
                   1 as libc::c_int as bool_);
    final_ask =
        price_item(o_ptr, (*ot_ptr).min_inflate as libc::c_int,
                   1 as libc::c_int as bool_);
    /* Determine if haggling is necessary */
    noneed = noneedtobargain(final_ask) as libc::c_int;
    /* Get the owner's payout limit */
    purse = (*ot_ptr).max_cost as s32b;
    /* No need to haggle */
    if noneed != 0 || auto_haggle as libc::c_int != 0 || final_ask >= purse {
        /* No reason to haggle */
        if final_ask >= purse {
            /* Message */
            msg_print(b"You instantly agree upon the price.\x00" as *const u8
                          as *const libc::c_char);
            msg_print(0 as cptr);
            /* Offer full purse */
            final_ask = purse
        } else if noneed != 0 {
            /* No need to haggle */
            /* Message */
            msg_print(b"You eventually agree upon the price.\x00" as *const u8
                          as *const libc::c_char);
            msg_print(0 as cptr);
        } else {
            /* No haggle option */
            /* Message summary */
            msg_print(b"You quickly agree upon the price.\x00" as *const u8 as
                          *const libc::c_char);
            msg_print(0 as cptr);
            final_ask -= final_ask / 10 as libc::c_int
        }
        /* Apply Sales Tax */
        /* Final price */
        cur_ask = final_ask;
        /* Final offer */
        final_0 = 1 as libc::c_int;
        pmt = b"Final Offer\x00" as *const u8 as *const libc::c_char
    }
    /* Haggle for the whole pile */
    cur_ask *= (*o_ptr).number as libc::c_int;
    final_ask *= (*o_ptr).number as libc::c_int;
    /* XXX XXX XXX Display commands */
    /* Haggling parameters */
    min_per = (*ot_ptr).haggle_per as s32b;
    max_per = min_per * 3 as libc::c_int;
    /* Mega-Hack -- artificial "last offer" value */
    last_offer = object_value(o_ptr) * (*o_ptr).number as libc::c_int;
    last_offer =
        ((last_offer * (*ot_ptr).max_inflate as libc::c_int) as libc::c_long /
             100 as libc::c_long) as s32b;
    /* No offer yet */
    offer = 0 as libc::c_int;
    /* No incremental haggling yet */
    allow_inc = 0 as libc::c_int as bool_;
    /* Haggle */
    flag = 0 as libc::c_int;
    while flag == 0 {
        loop  {
            loop_flag = 1 as libc::c_int;
            strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"%s :  %ld\x00" as *const u8 as *const libc::c_char, pmt,
                    cur_ask as libc::c_long);
            put_str(out_val.as_mut_ptr() as cptr, 1 as libc::c_int,
                    0 as libc::c_int);
            cancel =
                receive_offer(b"What price do you ask? \x00" as *const u8 as
                                  *const libc::c_char, &mut offer, last_offer,
                              -(1 as libc::c_int), cur_ask, final_0);
            if cancel != 0 {
                flag = 1 as libc::c_int
            } else if offer < cur_ask {
                say_comment_6();
                /* rejected, reset offer for incremental haggling */
                offer = last_offer
            } else if offer == cur_ask {
                flag = 1 as libc::c_int;
                *price = offer
            } else { loop_flag = 0 as libc::c_int }
            /* Stop */
            if flag != 0 || loop_flag == 0 { break ; }
        }
        if flag == 0 {
            x1 =
                100 as libc::c_int * (last_offer - offer) /
                    (last_offer - cur_ask);
            if x1 < min_per {
                if haggle_insults() != 0 {
                    flag = 1 as libc::c_int;
                    cancel = 1 as libc::c_int as bool_
                }
            } else if x1 > max_per {
                x1 = x1 * 3 as libc::c_int / 4 as libc::c_int;
                if x1 < max_per { x1 = max_per }
            }
            x2 =
                x1 - 2 as libc::c_int +
                    Rand_div(1 as libc::c_int + (x1 + 2 as libc::c_int) -
                                 (x1 - 2 as libc::c_int));
            x3 =
                (((offer - cur_ask) * x2) as libc::c_long /
                     100 as libc::c_long + 1 as libc::c_int as libc::c_long)
                    as s32b;
            /* don't let the price go down */
            if x3 < 0 as libc::c_int { x3 = 0 as libc::c_int }
            cur_ask += x3;
            if cur_ask > final_ask {
                cur_ask = final_ask;
                final_0 = 1 as libc::c_int;
                pmt = b"Final Offer\x00" as *const u8 as *const libc::c_char;
                annoyed += 1;
                if annoyed > 3 as libc::c_int {
                    flag = 1 as libc::c_int;
                    increase_insults();
                }
            } else if offer <= cur_ask {
                flag = 1 as libc::c_int;
                *price = offer
            }
            if flag == 0 {
                last_offer = offer;
                allow_inc = 1 as libc::c_int as bool_;
                prt(b"\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int, 0 as libc::c_int);
                strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                        b"Your last bid %ld\x00" as *const u8 as
                            *const libc::c_char, last_offer as libc::c_long);
                put_str(out_val.as_mut_ptr() as cptr, 1 as libc::c_int,
                        39 as libc::c_int);
                say_comment_3(cur_ask, annoyed);
            }
        }
    }
    /* Cancel */
    if cancel != 0 { return 1 as libc::c_int as bool_ }
    /* Update bargaining info */
    updatebargain(*price, final_ask);
    /* Do not cancel */
    return 0 as libc::c_int as bool_;
}
/*
 * Will the owner retire?
 */
unsafe extern "C" fn retire_owner_p() -> bool_ {
    let mut sti_ptr: *mut store_info_type =
        &mut *st_info.offset((*(*town_info.offset((*p_ptr).town_num as
                                                      isize)).store.offset(cur_store_num
                                                                               as
                                                                               isize)).st_idx
                                 as isize) as *mut store_info_type;
    if (*sti_ptr).owners[0 as libc::c_int as usize] as libc::c_int ==
           (*sti_ptr).owners[1 as libc::c_int as usize] as libc::c_int &&
           (*sti_ptr).owners[0 as libc::c_int as usize] as libc::c_int ==
               (*sti_ptr).owners[2 as libc::c_int as usize] as libc::c_int &&
           (*sti_ptr).owners[0 as libc::c_int as usize] as libc::c_int ==
               (*sti_ptr).owners[3 as libc::c_int as usize] as libc::c_int {
        /* there is no other owner */
        return 0 as libc::c_int as bool_
    }
    if Rand_div(21 as libc::c_int) != 0 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    return 1 as libc::c_int as bool_;
}
/*
 * Stole an item from a store                   -DG-
 */
#[no_mangle]
pub unsafe extern "C" fn store_stole() {
    let mut i: libc::c_int = 0;
    let mut amt: libc::c_int = 0;
    let mut item: libc::c_int = 0;
    let mut item_new: libc::c_int = 0;
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
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    if cur_store_num == 7 as libc::c_int {
        msg_print(b"You can\'t steal from your home!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Empty? */
    if (*st_ptr).stock_num as libc::c_int <= 0 as libc::c_int {
        msg_print(b"There is no item to steal.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Find the number of objects on this and following pages */
    i = (*st_ptr).stock_num as libc::c_int - store_top;
    /* And then restrict it to the current page */
    if i > 12 as libc::c_int { i = 12 as libc::c_int }
    /* Prompt */
    strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
            b"Which item do you want to steal? \x00" as *const u8 as
                *const libc::c_char);
    /* Get the item number to be bought */
    if get_stock(&mut item, out_val.as_mut_ptr() as cptr, 0 as libc::c_int,
                 i - 1 as libc::c_int) == 0 {
        return
    }
    /* Get the actual index */
    item = item + store_top;
    /* Get the actual item */
    o_ptr = &mut *(*st_ptr).stock.offset(item as isize) as *mut object_type;
    /* Assume the player wants just one of them */
    amt = 1 as libc::c_int;
    /* Get local object */
    j_ptr = &mut forge;
    /* Get a copy of the object */
    object_copy(j_ptr, o_ptr);
    /* Modify quantity */
    (*j_ptr).number = amt as byte_hack;
    /* Hack -- require room in pack */
    if inven_carry_okay(j_ptr) == 0 {
        msg_print(b"You cannot carry that many different items.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Find out how many the player wants */
    if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
        /* Get a quantity */
        amt = get_quantity(0 as cptr, (*o_ptr).number as s32b);
        /* Allow user abort */
        if amt <= 0 as libc::c_int { return }
    }
    /* Get local object */
    j_ptr = &mut forge;
    /* Get desired object */
    object_copy(j_ptr, o_ptr);
    /* Modify quantity */
    (*j_ptr).number = amt as byte_hack;
    /* Hack -- require room in pack */
    if inven_carry_okay(j_ptr) == 0 {
        msg_print(b"You cannot carry that many items.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Player tries to stole it */
    if Rand_div(40 as libc::c_int -
                    (*p_ptr).stat_ind[3 as libc::c_int as usize] as
                        libc::c_int +
                    (*j_ptr).weight * amt /
                        (5 as libc::c_int +
                             get_skill_scale(40 as libc::c_int,
                                             15 as libc::c_int as u32b) as
                                 libc::c_int) -
                    get_skill_scale(40 as libc::c_int,
                                    15 as libc::c_int as u32b) as libc::c_int)
           <= 10 as libc::c_int {
        /* Hack -- buying an item makes you aware of it */
        object_aware(j_ptr);
        /* Be aware of how you found it */
        (*j_ptr).found = 8 as libc::c_int as byte_hack;
        (*j_ptr).found_aux1 = (*st_ptr).st_idx as s16b;
        /* Hack -- clear the "fixed" flag from the item */
        (*j_ptr).ident =
            ((*j_ptr).ident as libc::c_int & !(0x2 as libc::c_int)) as
                byte_hack;
        /* "Hot" merchandise can't be sold back.  It doesn't make sense
		   to be able to sell back to a guy what you just stole from him.
		   Also, without the discount one could fairly easily macro himself
		   an infinite money supply */
        (*j_ptr).discount = 100 as libc::c_int as byte_hack;
        if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
            (*j_ptr).pval =
                (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int;
            (*o_ptr).pval -= (*j_ptr).pval
        }
        /* Describe the transaction */
        object_desc(o_name.as_mut_ptr(), j_ptr, 1 as libc::c_int,
                    3 as libc::c_int);
        /* Message */
        msg_format(b"You steal %s.\x00" as *const u8 as *const libc::c_char,
                   o_name.as_mut_ptr());
        /* Erase the inscription */
        (*j_ptr).note = 0 as libc::c_int as u16b;
        /* Give it to the player */
        item_new =
            inven_carry(j_ptr, 0 as libc::c_int as bool_) as libc::c_int;
        /* Describe the final result */
        object_desc(o_name.as_mut_ptr(),
                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(item_new as
                                                                     isize),
                    1 as libc::c_int, 3 as libc::c_int);
        /* Message */
        msg_format(b"You have %s (%c).\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   index_to_label(item_new) as libc::c_int);
        /* Handle stuff */
        handle_stuff();
        /* Note how many slots the store used to have */
        i = (*st_ptr).stock_num as libc::c_int;
        /* Remove the bought items from the store */
        store_item_increase(item, -amt);
        store_item_optimize(item);
        /* Store is empty */
        if (*st_ptr).stock_num as libc::c_int == 0 as libc::c_int {
            /* Shuffle */
            if retire_owner_p() != 0 {
                /* Message */
                msg_print(b"The shopkeeper retires.\x00" as *const u8 as
                              *const libc::c_char);
                /* Shuffle the store */
                store_shuffle(cur_store_num);
            } else {
                /* Maintain */
                /* Message */
                msg_print(b"The shopkeeper brings out some new stock.\x00" as
                              *const u8 as *const libc::c_char);
            }
            /* New inventory */
            i = 0 as libc::c_int;
            while i < 10 as libc::c_int {
                /* Maintain the store */
                store_maint((*p_ptr).town_num as libc::c_int, cur_store_num);
                i += 1
            }
            /* Start over */
            store_top = 0 as libc::c_int;
            /* Redraw everything */
            display_inventory();
        } else if (*st_ptr).stock_num as libc::c_int != i {
            /* The item is gone */
            /* Pick the correct screen */
            if store_top >= (*st_ptr).stock_num as libc::c_int {
                store_top -= 12 as libc::c_int
            }
            /* Redraw everything */
            display_inventory();
        } else {
            /* Item is still here */
            /* Redraw the item */
            display_entry(item);
        }
    } else {
        /* Complain */
        say_comment_4();
        /* Reset insults */
        (*st_ptr).insult_cur = 0 as libc::c_int as s16b;
        (*st_ptr).good_buy = 0 as libc::c_int as s16b;
        (*st_ptr).bad_buy = 0 as libc::c_int as s16b;
        /* Kicked out for a LONG time */
        (*st_ptr).store_open =
            turn + 500000 as libc::c_int +
                (Rand_div(500000 as libc::c_int) + 1 as libc::c_int)
    };
}
/*
 * Buy an item from a store 			-RAK-
 */
#[no_mangle]
pub unsafe extern "C" fn store_purchase() {
    let mut i: libc::c_int = 0;
    let mut amt: libc::c_int = 1 as libc::c_int;
    let mut choice: libc::c_int = 0;
    let mut item: libc::c_int = 0;
    let mut item_new: libc::c_int = 0;
    let mut price: s32b = 0;
    let mut best: s32b = 0;
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
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    /* Museum? */
    if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as libc::c_long &
           0x400 as libc::c_long != 0 {
        msg_print(b"You cannot take items from the museum!\x00" as *const u8
                      as *const libc::c_char);
        return
    }
    /* Empty? */
    if (*st_ptr).stock_num as libc::c_int <= 0 as libc::c_int {
        if cur_store_num == 7 as libc::c_int {
            msg_print(b"Your home is empty.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            msg_print(b"I am currently out of stock.\x00" as *const u8 as
                          *const libc::c_char);
        }
        return
    }
    /* Find the number of objects on this and following pages */
    i = (*st_ptr).stock_num as libc::c_int - store_top;
    /* And then restrict it to the current page */
    if i > 12 as libc::c_int { i = 12 as libc::c_int }
    /* Prompt */
    if cur_store_num == 7 as libc::c_int {
        strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                b"Which item do you want to take? \x00" as *const u8 as
                    *const libc::c_char);
    } else {
        strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                b"Which item are you interested in? \x00" as *const u8 as
                    *const libc::c_char);
    }
    /* Get the item number to be bought */
    if get_stock(&mut item, out_val.as_mut_ptr() as cptr, 0 as libc::c_int,
                 i - 1 as libc::c_int) == 0 {
        return
    }
    /* Get the actual index */
    item = item + store_top;
    /* Get the actual item */
    o_ptr = &mut *(*st_ptr).stock.offset(item as isize) as *mut object_type;
    /* Get local object */
    j_ptr = &mut forge;
    /* Get a copy of one object to determine the price */
    object_copy(j_ptr, o_ptr);
    /* Modify quantity */
    (*j_ptr).number = 1 as libc::c_int as byte_hack;
    /* Hack -- If a wand, allocate the number of charges of one wand */
    if (*j_ptr).tval as libc::c_int == 65 as libc::c_int {
        (*j_ptr).pval = (*o_ptr).pval / (*o_ptr).number as libc::c_int
    }
    /* Hack -- require room in pack */
    if inven_carry_okay(j_ptr) == 0 {
        msg_print(b"You cannot carry that many different items.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Determine the "best" price (per item) */
    best =
        price_item(j_ptr, (*ot_ptr).min_inflate as libc::c_int,
                   0 as libc::c_int as bool_);
    /* Find out how many the player wants */
    if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
        let mut q: s32b = 0;
        /* Hack -- note cost of "fixed" items */
        if cur_store_num != 7 as libc::c_int &&
               (*o_ptr).ident as libc::c_int & 0x2 as libc::c_int != 0 {
            msg_format(b"That costs %ld gold per item.\x00" as *const u8 as
                           *const libc::c_char, best as libc::c_long);
        }
        /* How many can we buy ? 99 if price is 0*/
        if cur_store_num == 7 as libc::c_int {
            q = 99 as libc::c_int
        } else if best == 0 as libc::c_int {
            q = 99 as libc::c_int
        } else if auto_haggle != 0 {
            q = (*p_ptr).au / (best + best / 10 as libc::c_int)
        } else { q = (*p_ptr).au / best }
        if ((*o_ptr).number as libc::c_int) < q {
            q = (*o_ptr).number as s32b
        }
        /* None ? ahh too bad */
        if q == 0 {
            msg_print(b"You do not have enough gold to buy one.\x00" as
                          *const u8 as *const libc::c_char);
            return
        }
        /* Get a quantity */
        amt = get_quantity(0 as cptr, q);
        /* Allow user abort */
        if amt <= 0 as libc::c_int { return }
    }
    /* Get local object */
    j_ptr = &mut forge;
    /* Get desired object */
    object_copy(j_ptr, o_ptr);
    /* Modify quantity */
    (*j_ptr).number = amt as byte_hack;
    /* Hack -- If a rod or wand, allocate total maximum timeouts or charges
	 * between those purchased and left on the shelf. -LM-
	 */
    if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
        (*j_ptr).pval = (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int
    }
    /* Hack -- require room in pack */
    if inven_carry_okay(j_ptr) == 0 {
        msg_print(b"You cannot carry that many items.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Attempt to buy it */
    if cur_store_num != 7 as libc::c_int {
        /* Fixed price, quick buy */
        if (*o_ptr).ident as libc::c_int & 0x2 as libc::c_int != 0 {
            /* Assume accept */
            choice = 0 as libc::c_int;
            /* Go directly to the "best" deal */
            price = best * (*j_ptr).number as libc::c_int
        } else {
            /* Haggle for it */
            /* Describe the object (fully) */
            object_desc_store(o_name.as_mut_ptr(), j_ptr, 1 as libc::c_int,
                              3 as libc::c_int);
            msg_format(b"Buying %s (%c).\x00" as *const u8 as
                           *const libc::c_char, o_name.as_mut_ptr(),
                       item + 'a' as i32);
            msg_print(0 as cptr);
            choice = purchase_haggle(j_ptr, &mut price) as libc::c_int;
            if (*st_ptr).store_open >= turn { return }
        }
        /* Message */
        /* Haggle for a final price */
        /* Hack -- Got kicked out */
        /* Player wants it */
        if choice == 0 as libc::c_int {
            /* Fix the item price (if "correctly" haggled) */
            if price == best * (*j_ptr).number as libc::c_int {
                (*o_ptr).ident =
                    ((*o_ptr).ident as libc::c_int | 0x2 as libc::c_int) as
                        byte_hack
            }
            /* Player can afford it */
            if (*p_ptr).au >= price {
                /* Say "okay" */
                say_comment_1();
                /* Make a sound */
                sound(26 as libc::c_int);
                /* Be happy */
                decrease_insults();
                /* Spend the money */
                (*p_ptr).au -= price;
                /* Update the display */
                store_prt_gold();
                /* Hack -- buying an item makes you aware of it */
                object_aware(j_ptr);
                /* Be aware of how you found it */
                (*j_ptr).found = 7 as libc::c_int as byte_hack;
                (*j_ptr).found_aux1 = (*st_ptr).st_idx as s16b;
                /* Hack -- clear the "fixed" flag from the item */
                (*j_ptr).ident =
                    ((*j_ptr).ident as libc::c_int & !(0x2 as libc::c_int)) as
                        byte_hack;
                /* Describe the transaction */
                object_desc(o_name.as_mut_ptr(), j_ptr, 1 as libc::c_int,
                            3 as libc::c_int);
                /* Message */
                msg_format(b"You bought %s for %ld gold.\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr(),
                           price as libc::c_long);
                /* Erase the inscription */
                (*j_ptr).note = 0 as libc::c_int as u16b;
                /* Hack -- If a rod or wand, allocate total maximum
				 * timeouts or charges between those picked up and 
				 * those left behind. -LM-
				 */
                if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
                    (*j_ptr).pval =
                        (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int;
                    (*o_ptr).pval -= (*j_ptr).pval
                }
                /* Give it to the player */
                item_new =
                    inven_carry(j_ptr, 0 as libc::c_int as bool_) as
                        libc::c_int;
                /* Describe the final result */
                object_desc(o_name.as_mut_ptr(),
                            &mut *(*p_ptr).inventory.as_mut_ptr().offset(item_new
                                                                             as
                                                                             isize),
                            1 as libc::c_int, 3 as libc::c_int);
                /* Message */
                msg_format(b"You have %s (%c).\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr(),
                           index_to_label(item_new) as libc::c_int);
                /* Handle stuff */
                handle_stuff();
                /* Note how many slots the store used to have */
                i = (*st_ptr).stock_num as libc::c_int;
                /* Remove the bought items from the store */
                store_item_increase(item, -amt);
                store_item_optimize(item);
                /* Store is empty */
                if (*st_ptr).stock_num as libc::c_int == 0 as libc::c_int {
                    /* Shuffle */
                    if retire_owner_p() != 0 {
                        /* Message */
                        msg_print(b"The shopkeeper retires.\x00" as *const u8
                                      as *const libc::c_char);
                        /* Shuffle the store */
                        store_shuffle(cur_store_num);
                    } else {
                        /* Maintain */
                        /* Message */
                        msg_print(b"The shopkeeper brings out some new stock.\x00"
                                      as *const u8 as *const libc::c_char);
                    }
                    /* New inventory */
                    i = 0 as libc::c_int;
                    while i < 10 as libc::c_int {
                        /* Maintain the store */
                        store_maint((*p_ptr).town_num as libc::c_int,
                                    cur_store_num);
                        i += 1
                    }
                    /* Start over */
                    store_top = 0 as libc::c_int
                } else if (*st_ptr).stock_num as libc::c_int != i {
                    /* The item is gone */
                    /* Pick the correct screen */
                    if store_top >= (*st_ptr).stock_num as libc::c_int {
                        store_top -= 12 as libc::c_int
                    }
                }
                /* Redraw everything */
                display_inventory();
            } else {
                /* Player cannot afford it */
                /* Simple message (no insult) */
                msg_print(b"You do not have enough gold.\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
    } else {
        /* Home is much easier */
        /* Hack -- If a rod or wand, allocate total maximum
		 * timeouts or charges between those picked up and 
		 * those left behind. -LM-
		 */
        if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
            (*j_ptr).pval =
                (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int;
            (*o_ptr).pval -= (*j_ptr).pval
        }
        item_new =
            inven_carry(j_ptr, 0 as libc::c_int as bool_) as libc::c_int;
        object_desc(o_name.as_mut_ptr(),
                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(item_new as
                                                                     isize),
                    1 as libc::c_int, 3 as libc::c_int);
        msg_format(b"You have %s (%c).\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   index_to_label(item_new) as libc::c_int);
        handle_stuff();
        i = (*st_ptr).stock_num as libc::c_int;
        store_item_increase(item, -amt);
        store_item_optimize(item);
        if i == (*st_ptr).stock_num as libc::c_int {
            /* Give it to the player */
            /* Describe just the result */
            /* Message */
            /* Handle stuff */
            /* Take note if we take the last one */
            /* Remove the items from the home */
            /* Redraw the item */
            display_entry(item);
        } else {
            /* The item is gone */
            /* Nothing left */
            if (*st_ptr).stock_num as libc::c_int == 0 as libc::c_int {
                store_top = 0 as libc::c_int
            } else if store_top >= (*st_ptr).stock_num as libc::c_int {
                store_top -= 12 as libc::c_int
            }
            display_inventory();
        }
    };
}
/* Nothing left on that screen */
/* Redraw everything */
/* Hack -- Item is still here */
/*
 * Sell an item to the store (or home)
 */
#[no_mangle]
pub unsafe extern "C" fn store_sell() {
    let mut choice: libc::c_int = 0;
    let mut item: libc::c_int = 0;
    let mut item_pos: libc::c_int = 0;
    let mut amt: libc::c_int = 0;
    let mut price: s32b = 0;
    let mut value: s32b = 0;
    let mut dummy: s32b = 0;
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
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut museum: bool_ =
        if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as libc::c_long
               & 0x400 as libc::c_long != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    /* Prepare a prompt */
    if cur_store_num == 7 as libc::c_int {
        q = b"Drop which item? \x00" as *const u8 as *const libc::c_char
    } else if museum != 0 {
        q = b"Donate which item?\x00" as *const u8 as *const libc::c_char
    } else {
        q = b"Sell which item? \x00" as *const u8 as *const libc::c_char
    }
    /* Only allow items the store will buy */
    item_tester_hook =
        Some(store_will_buy as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    if cur_store_num == 7 as libc::c_int {
        s =
            b"You have nothing to drop.\x00" as *const u8 as
                *const libc::c_char
    } else if museum != 0 {
        s =
            b"You have nothing to donate.\x00" as *const u8 as
                *const libc::c_char
    } else {
        s =
            b"You have nothing that I want.\x00" as *const u8 as
                *const libc::c_char
    }
    if get_item(&mut item, q, s, 0x1 as libc::c_int | 0x2 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Hack -- Cannot remove cursed items */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        if item >= 24 as libc::c_int {
            /* Oops */
            msg_print(b"Hmmm, it seems to be cursed.\x00" as *const u8 as
                          *const libc::c_char);
            /* Nope */
            return
        } else {
            if f4 as libc::c_long & 0x40000000 as libc::c_long != 0 {
                /* Oops */
                msg_print(b"Hmmm, you seem to be unable to drop it.\x00" as
                              *const u8 as *const libc::c_char);
                /* Nope */
                return
            }
        }
    }
    /* Assume one item */
    amt = 1 as libc::c_int;
    /* Find out how many the player wants (letter means "all") */
    if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
        /* Get a quantity */
        amt = get_quantity(0 as cptr, (*o_ptr).number as s32b);
        /* Allow user abort */
        if amt <= 0 as libc::c_int { return }
    }
    /* Get local object */
    q_ptr = &mut forge;
    /* Get a copy of the object */
    object_copy(q_ptr, o_ptr);
    /* Modify quantity */
    (*q_ptr).number = amt as byte_hack;
    /* Hack -- If a rod or wand, allocate total maximum
	 * timeouts or charges to those being sold. -LM-
	 */
    if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
        (*q_ptr).pval = (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int
    }
    /* Get a full description */
    object_desc(o_name.as_mut_ptr(), q_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Remove any inscription for stores */
    if cur_store_num != 7 as libc::c_int && museum == 0 {
        (*q_ptr).note = 0 as libc::c_int as u16b
    }
    /* Is there room in the store (or the home?) */
    if store_check_num(q_ptr) == 0 {
        if cur_store_num == 7 as libc::c_int {
            msg_print(b"Your home is full.\x00" as *const u8 as
                          *const libc::c_char);
        } else if museum != 0 {
            msg_print(b"The museum is full.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            msg_print(b"I have not the room in my store to keep it.\x00" as
                          *const u8 as *const libc::c_char);
        }
        return
    }
    /* Real store */
    if cur_store_num != 7 as libc::c_int && museum == 0 {
        /* Describe the transaction */
        msg_format(b"Selling %s (%c).\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   index_to_label(item) as libc::c_int);
        msg_print(0 as cptr);
        /* Haggle for it */
        choice = sell_haggle(q_ptr, &mut price) as libc::c_int;
        /* Kicked out */
        if (*st_ptr).store_open >= turn { return }
        /* Sold... */
        if choice == 0 as libc::c_int {
            /* Say "okay" */
            say_comment_1();
            /* Make a sound */
            sound(27 as libc::c_int);
            /* Be happy */
            decrease_insults();
            /* Get some money */
            (*p_ptr).au += price;
            /* Update the display */
            store_prt_gold();
            /* Get the "apparent" value */
            dummy = object_value(q_ptr) * (*q_ptr).number as libc::c_int;
            /* Identify original item */
            object_aware(o_ptr);
            object_known(o_ptr);
            /* Combine / Reorder the pack (later) */
            (*p_ptr).notice =
                ((*p_ptr).notice as libc::c_long |
                     (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long |
                     (0x1 as libc::c_long | 0x2 as libc::c_long |
                          0x8 as libc::c_long)) as u32b;
            /* Get local object */
            q_ptr = &mut forge;
            /* Get a copy of the object */
            object_copy(q_ptr, o_ptr);
            /* Modify quantity */
            (*q_ptr).number = amt as byte_hack;
            /*
			 * Hack -- If a rod or wand, let the shopkeeper know just 
			 * how many charges he really paid for. -LM-
			 */
            if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
                (*q_ptr).pval =
                    (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int
            }
            /* Get the "actual" value */
            value = object_value(q_ptr) * (*q_ptr).number as libc::c_int;
            /* Get the description all over again */
            object_desc(o_name.as_mut_ptr(), q_ptr, 1 as libc::c_int,
                        3 as libc::c_int);
            /* Describe the result (in message buffer) */
            msg_format(b"You sold %s for %ld gold.\x00" as *const u8 as
                           *const libc::c_char, o_name.as_mut_ptr(),
                       price as libc::c_long);
            /* Analyze the prices (and comment verbally) */
            purchase_analyze(price, value, dummy);
            /*
			 * Hack -- Allocate charges between those wands or rods sold 
			 * and retained, unless all are being sold. -LM-
			 */
            if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
                (*q_ptr).pval =
                    (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int;
                if (*o_ptr).number as libc::c_int > amt {
                    (*o_ptr).pval -= (*q_ptr).pval
                }
            }
            /* Take the item from the player, describe the result */
            inc_stack_size(item, -amt);
            /* Handle stuff */
            handle_stuff();
            /* The store gets that (known) item */
            item_pos = store_carry(q_ptr);
            /* Re-display if item is now in store */
            if item_pos >= 0 as libc::c_int {
                store_top = item_pos / 12 as libc::c_int * 12 as libc::c_int;
                display_inventory();
            }
        }
    } else if museum != 0 {
        let mut o2_name: [libc::c_char; 80] = [0; 80];
        object_desc(o2_name.as_mut_ptr(), q_ptr, 1 as libc::c_int,
                    0 as libc::c_int);
        msg_print(b"Once you donate something, you cannot take it back.\x00"
                      as *const u8 as *const libc::c_char);
        if get_check(format(b"Do you really want to donate %s?\x00" as
                                *const u8 as *const libc::c_char,
                            o2_name.as_mut_ptr()) as cptr) == 0 {
            return
        }
        /* Player is at museum */
        /* Identify it */
        object_aware(q_ptr);
        object_known(q_ptr);
        (*q_ptr).ident =
            ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int) as
                byte_hack;
        /*
		 * Hack -- Allocate charges between those wands or rods sold 
		 * and retained, unless all are being sold. -LM-
		 */
        if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
            (*q_ptr).pval =
                (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int;
            if (*o_ptr).number as libc::c_int > amt {
                (*o_ptr).pval -= (*q_ptr).pval
            }
        }
        /* Describe */
        msg_format(b"You donate %s (%c).\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   index_to_label(item) as libc::c_int);
        choice = 0 as libc::c_int;
        /* Take it from the players inventory */
        inc_stack_size(item, -amt);
        /* Handle stuff */
        handle_stuff();
        /* Let the home carry it */
        item_pos = home_carry(q_ptr);
        /* Update store display */
        if item_pos >= 0 as libc::c_int {
            store_top = item_pos / 12 as libc::c_int * 12 as libc::c_int;
            display_inventory();
        }
    } else {
        /* Player is at home */
        /* Describe */
        msg_format(b"You drop %s (%c).\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   index_to_label(item) as libc::c_int);
        if (*o_ptr).tval as libc::c_int == 65 as libc::c_int {
            (*q_ptr).pval =
                (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int;
            if (*o_ptr).number as libc::c_int > amt {
                (*o_ptr).pval -= (*q_ptr).pval
            }
        }
        inc_stack_size(item, -amt);
        handle_stuff();
        item_pos = home_carry(q_ptr);
        if item_pos >= 0 as libc::c_int {
            store_top = item_pos / 12 as libc::c_int * 12 as libc::c_int;
            display_inventory();
        }
    };
}
/*
		 * Hack -- Allocate charges between those wands or rods sold 
		 * and retained, unless all are being sold. -LM-
		 */
/* Take it from the players inventory */
/* Handle stuff */
/* Let the home carry it */
/* Update store display */
/*
 * Examine an item in a store			   -JDL-
 */
#[no_mangle]
pub unsafe extern "C" fn store_examine() {
    let mut i: libc::c_int = 0;
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    /* Empty? */
    if (*st_ptr).stock_num as libc::c_int <= 0 as libc::c_int {
        if cur_store_num == 7 as libc::c_int {
            msg_print(b"Your home is empty.\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as
                      libc::c_long & 0x400 as libc::c_long != 0 {
            msg_print(b"The museum is empty.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            msg_print(b"I am currently out of stock.\x00" as *const u8 as
                          *const libc::c_char);
        }
        return
    }
    /* Find the number of objects on this and following pages */
    i = (*st_ptr).stock_num as libc::c_int - store_top;
    /* And then restrict it to the current page */
    if i > 12 as libc::c_int { i = 12 as libc::c_int }
    /* Prompt */
    strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
            b"Which item do you want to examine? \x00" as *const u8 as
                *const libc::c_char);
    /* Get the item number to be examined */
    if get_stock(&mut item, out_val.as_mut_ptr() as cptr, 0 as libc::c_int,
                 i - 1 as libc::c_int) == 0 {
        return
    }
    /* Get the actual index */
    item = item + store_top;
    /* Get the actual item */
    o_ptr = &mut *(*st_ptr).stock.offset(item as isize) as *mut object_type;
    /* Debug hack */
    if wizard != 0 {
        drop_near(o_ptr, -(1 as libc::c_int), (*p_ptr).py as libc::c_int,
                  (*p_ptr).px as libc::c_int);
    }
    /* Require full knowledge */
    if (*o_ptr).ident as libc::c_int & 0x20 as libc::c_int == 0 {
        /* This can only happen in the home */
        msg_print(b"You have no special knowledge about that item.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Description */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Describe */
    msg_format(b"Examining %s...\x00" as *const u8 as *const libc::c_char,
               o_name.as_mut_ptr());
    /* Show the object's powers. */
    if object_out_desc(o_ptr, 0 as *mut FILE, 0 as libc::c_int as bool_,
                       1 as libc::c_int as bool_) == 0 {
        msg_print(b"You see nothing special.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Show spell listing for instruments, daemonwear and spellbooks. */
    if (*o_ptr).tval as libc::c_int == 14 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 115 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 111 as libc::c_int {
        do_cmd_browse_aux(o_ptr);
    };
}
/*
 * Hack -- set this to leave the store
 */
static mut leave_store: bool_ = 0 as libc::c_int as bool_;
/*
 * Process a command in a store
 *
 * Note that we must allow the use of a few "special" commands
 * in the stores which are not allowed in the dungeon, and we
 * must disable some commands which are allowed in the dungeon
 * but not in the stores, to prevent chaos.
 */
unsafe extern "C" fn store_process_command() -> bool_ {
    let mut validcmd: bool_ = 0 as libc::c_int as bool_;
    let mut i: libc::c_int = 0;
    let mut ba_ptr: *mut store_action_type = 0 as *mut store_action_type;
    let mut recreate: bool_ = 0 as libc::c_int as bool_;
    /* Handle repeating the last command */
    repeat_check();
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        ba_ptr =
            &mut *ba_info.offset(*(*st_info.offset((*st_ptr).st_idx as
                                                       isize)).actions.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)
                                     as isize) as *mut store_action_type;
        if (*ba_ptr).letter != 0 {
            if (*ba_ptr).letter as libc::c_int == command_cmd as libc::c_int {
                validcmd = 1 as libc::c_int as bool_;
                break ;
            }
        }
        if (*ba_ptr).letter_aux != 0 {
            if (*ba_ptr).letter_aux as libc::c_int ==
                   command_cmd as libc::c_int {
                validcmd = 1 as libc::c_int as bool_;
                break ;
            }
        }
        i += 1
    }
    if validcmd != 0 {
        recreate = bldg_process_command(st_ptr, i)
    } else {
        let mut current_block_59: u64;
        /* Parse the command */
        match command_cmd as libc::c_int {
            27 => {
                /* Leave */
                leave_store = 1 as libc::c_int as bool_;
                current_block_59 = 15622658527355336244;
            }
            32 => {
                /* Browse */
                if (*st_ptr).stock_num as libc::c_int <= 12 as libc::c_int {
                    msg_print(b"Entire inventory is shown.\x00" as *const u8
                                  as *const libc::c_char);
                } else {
                    store_top += 12 as libc::c_int;
                    if store_top >= (*st_ptr).stock_num as libc::c_int {
                        store_top = 0 as libc::c_int
                    }
                    display_inventory();
                }
                current_block_59 = 15622658527355336244;
            }
            45 => {
                /* Browse backwards */
                if (*st_ptr).stock_num as libc::c_int <= 12 as libc::c_int {
                    msg_print(b"Entire inventory is shown.\x00" as *const u8
                                  as *const libc::c_char);
                } else {
                    store_top -= 12 as libc::c_int;
                    if store_top < 0 as libc::c_int {
                        store_top =
                            ((*st_ptr).stock_num as libc::c_int -
                                 1 as libc::c_int) / 12 as libc::c_int *
                                12 as libc::c_int
                    }
                    display_inventory();
                }
                current_block_59 = 14072441030219150333;
            }
            18 => { current_block_59 = 14072441030219150333; }
            13 => { current_block_59 = 15622658527355336244; }
            119 => {
                /* ** Inventory Commands ***/
                /* Wear/wield equipment */
                do_cmd_wield();
                current_block_59 = 15622658527355336244;
            }
            116 => {
                /* Take off equipment */
                do_cmd_takeoff();
                current_block_59 = 15622658527355336244;
            }
            107 => {
                /* Destroy an item */
                do_cmd_destroy();
                current_block_59 = 15622658527355336244;
            }
            101 => {
                /* Equipment list */
                do_cmd_equip();
                current_block_59 = 15622658527355336244;
            }
            105 => {
                /* Inventory list */
                do_cmd_inven();
                current_block_59 = 15622658527355336244;
            }
            73 => {
                /* ** Various commands ***/
                /* Identify an object */
                do_cmd_observe();
                current_block_59 = 15622658527355336244;
            }
            9 => {
                /* Hack -- toggle windows */
                toggle_inven_equip();
                current_block_59 = 15622658527355336244;
            }
            98 => {
                /* ** Use various objects ***/
                /* Browse a book */
                do_cmd_browse();
                current_block_59 = 15622658527355336244;
            }
            123 => {
                /* Inscribe an object */
                do_cmd_inscribe();
                current_block_59 = 15622658527355336244;
            }
            125 => {
                /* Uninscribe an object */
                do_cmd_uninscribe();
                current_block_59 = 15622658527355336244;
            }
            63 => {
                /* ** Help and Such ***/
                /* Help */
                do_cmd_help();
                current_block_59 = 15622658527355336244;
            }
            47 => {
                /* Identify symbol */
                do_cmd_query_symbol();
                current_block_59 = 15622658527355336244;
            }
            67 => {
                /* Character description */
                do_cmd_change_name();
                display_store();
                current_block_59 = 15622658527355336244;
            }
            33 => {
                /* ** System Commands ***/
                /* Hack -- User interface */
                Term_user(0 as libc::c_int);
                current_block_59 = 15622658527355336244;
            }
            34 => {
                /* Single line from a pref file */
                do_cmd_pref();
                current_block_59 = 15622658527355336244;
            }
            64 => {
                /* Interact with macros */
                do_cmd_macros();
                current_block_59 = 15622658527355336244;
            }
            37 => {
                /* Interact with visuals */
                do_cmd_visuals();
                current_block_59 = 15622658527355336244;
            }
            38 => {
                /* Interact with colors */
                do_cmd_colors();
                current_block_59 = 15622658527355336244;
            }
            61 => {
                /* Interact with options */
                do_cmd_options();
                current_block_59 = 15622658527355336244;
            }
            58 => {
                /* ** Misc Commands ***/
                /* Take notes */
                do_cmd_note();
                current_block_59 = 15622658527355336244;
            }
            86 => {
                /* Version info */
                do_cmd_version();
                current_block_59 = 15622658527355336244;
            }
            6 => {
                /* Repeat level feeling */
                do_cmd_feeling();
                current_block_59 = 15622658527355336244;
            }
            15 => {
                /* Show previous message */
                do_cmd_message_one();
                current_block_59 = 15622658527355336244;
            }
            16 => {
                /* Show previous messages */
                do_cmd_messages();
                current_block_59 = 15622658527355336244;
            }
            126 | 124 => {
                /* Check artifacts, uniques etc. */
                do_cmd_knowledge();
                current_block_59 = 15622658527355336244;
            }
            40 => {
                /* Load "screen dump" */
                do_cmd_load_screen();
                current_block_59 = 15622658527355336244;
            }
            41 => {
                /* Save "screen dump" */
                do_cmd_save_screen();
                current_block_59 = 15622658527355336244;
            }
            _ => {
                /* Hack -- Unknown command */
                if (*st_ptr).st_idx as libc::c_int == 7 as libc::c_int {
                    msg_print(b"That command does not work in this home.\x00"
                                  as *const u8 as *const libc::c_char);
                } else {
                    msg_print(b"That command does not work in this store.\x00"
                                  as *const u8 as *const libc::c_char);
                }
                current_block_59 = 15622658527355336244;
            }
        }
        match current_block_59 {
            14072441030219150333 =>
            /* Redraw */
            {
                do_cmd_redraw();
                display_store();
            }
            _ => { }
        }
    }
    return recreate;
}
/*
 * Enter a store, and interact with it.
 *
 * Note that we use the standard "request_command()" function
 * to get a command, allowing us to use "command_arg" and all
 * command macros and other nifty stuff, but we use the special
 * "shopping" argument, to force certain commands to be converted
 * into other commands, normally, we convert "p" (pray) and "m"
 * (cast magic) into "g" (get), and "s" (search) into "d" (drop).
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_store() {
    let mut which: libc::c_int = 0;
    let mut maintain_num: libc::c_int = 0;
    let mut tmp_chr: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut recreate: bool_ = 0 as libc::c_int as bool_;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Access the player grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                             isize)).offset((*p_ptr).px as
                                                                isize) as
            *mut cave_type;
    /* Verify a store */
    if (*c_ptr).feat as libc::c_int != 0x4a as libc::c_int {
        msg_print(b"You see no store here.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Extract the store code */
    which = (*c_ptr).special as libc::c_int;
    /* Hack -- Check the "locked doors" */
    if (*(*town_info.offset((*p_ptr).town_num as
                                isize)).store.offset(which as
                                                         isize)).store_open >=
           turn {
        msg_print(b"The doors are locked.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Calculate the number of store maintainances since the last visit */
    maintain_num =
        ((turn -
              (*(*town_info.offset((*p_ptr).town_num as
                                       isize)).store.offset(which as
                                                                isize)).last_visit)
             as libc::c_long /
             (10 as libc::c_long * 1000 as libc::c_int as libc::c_long)) as
            libc::c_int;
    /* Maintain the store max. 10 times */
    if maintain_num > 10 as libc::c_int { maintain_num = 10 as libc::c_int }
    if maintain_num != 0 {
        /* Maintain the store */
        i = 0 as libc::c_int;
        while i < maintain_num {
            store_maint((*p_ptr).town_num as libc::c_int, which);
            i += 1
        }
        /* Save the visit */
        (*(*town_info.offset((*p_ptr).town_num as
                                 isize)).store.offset(which as
                                                          isize)).last_visit =
            turn
    }
    /* Forget the lite */
	/* forget_lite(); */
    /* Forget the view */
    forget_view();
    /* Hack -- Character is in "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* No command argument */
    command_arg = 0 as libc::c_int as s16b;
    /* No repeated command */
    command_rep = 0 as libc::c_int as s16b;
    /* No automatic command */
    command_new = 0 as libc::c_int as s16b;
    /* Save the store number */
    cur_store_num = which;
    /* Save the store and owner pointers */
    st_ptr =
        &mut *(*town_info.offset((*p_ptr).town_num as
                                     isize)).store.offset(cur_store_num as
                                                              isize) as
            *mut store_type;
    ot_ptr =
        &mut *ow_info.offset((*st_ptr).owner as isize) as *mut owner_type;
    /* Start at the beginning */
    store_top = 0 as libc::c_int;
    /* Display the store */
    display_store();
    /* Mega-Hack -- Ignore keymaps on store action letters */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        let mut ba_ptr: *mut store_action_type =
            &mut *ba_info.offset(*(*st_info.offset((*st_ptr).st_idx as
                                                       isize)).actions.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)
                                     as isize) as *mut store_action_type;
        *request_command_ignore_keymaps.as_mut_ptr().offset((2 as libc::c_int
                                                                 * i) as
                                                                isize) =
            (*ba_ptr).letter;
        *request_command_ignore_keymaps.as_mut_ptr().offset((2 as libc::c_int
                                                                 * i +
                                                                 1 as
                                                                     libc::c_int)
                                                                as isize) =
            (*ba_ptr).letter_aux;
        i += 1
    }
    /* Do not leave */
    leave_store = 0 as libc::c_int as bool_;
    /* Interact with player */
    while leave_store == 0 {
        /* Hack -- Clear line 1 */
        prt(b"\x00" as *const u8 as *const libc::c_char, 1 as libc::c_int,
            0 as libc::c_int);
        /* Hack -- Check the charisma */
        tmp_chr = (*p_ptr).stat_use[5 as libc::c_int as usize] as libc::c_int;
        /* Clear */
        clear_from(21 as libc::c_int);
        /* Basic commands */
        c_prt(11 as libc::c_int as byte_hack,
              b" ESC.\x00" as *const u8 as *const libc::c_char,
              22 as libc::c_int, 0 as libc::c_int);
        prt(b") Exit.\x00" as *const u8 as *const libc::c_char,
            22 as libc::c_int, 4 as libc::c_int);
        /* Browse if necessary */
        if (*st_ptr).stock_num as libc::c_int > 12 as libc::c_int {
            c_prt(11 as libc::c_int as byte_hack,
                  b" SPACE\x00" as *const u8 as *const libc::c_char,
                  23 as libc::c_int, 0 as libc::c_int);
            prt(b") Next page\x00" as *const u8 as *const libc::c_char,
                23 as libc::c_int, 6 as libc::c_int);
        }
        /* Prompt */
        prt(b"You may: \x00" as *const u8 as *const libc::c_char,
            21 as libc::c_int, 0 as libc::c_int);
        /* Show the commands */
        show_building(st_ptr);
        /* Get a command */
        request_command(1 as libc::c_int);
        /* Process the command */
        if store_process_command() != 0 {
            recreate = 1 as libc::c_int as bool_
        }
        /* Hack -- Character is still in "icky" mode */
        character_icky = 1 as libc::c_int as bool_;
        /* Notice stuff */
        notice_stuff();
        /* Handle stuff */
        handle_stuff();
        /* XXX XXX XXX Pack Overflow */
        if (*p_ptr).inventory[23 as libc::c_int as usize].k_idx != 0 {
            let mut item: libc::c_int = 23 as libc::c_int;
            let mut o_ptr: *mut object_type =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
                    *mut object_type;
            /* Hack -- Flee from the store */
            if cur_store_num != 7 as libc::c_int {
                /* Message */
                msg_print(b"Your pack is so full that you flee the store...\x00"
                              as *const u8 as *const libc::c_char);
                /* Leave */
                leave_store = 1 as libc::c_int as bool_
            } else if store_check_num(o_ptr) == 0 {
                /* Hack -- Flee from the home */
                /* Message */
                msg_print(b"Your pack is so full that you flee your home...\x00"
                              as *const u8 as *const libc::c_char);
                /* Leave */
                leave_store = 1 as libc::c_int as bool_
            } else {
                /* Hack -- Drop items into the home */
                let mut item_pos: libc::c_int = 0;
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
                /* Give a message */
                msg_print(b"Your pack overflows!\x00" as *const u8 as
                              *const libc::c_char);
                /* Get local object */
                q_ptr = &mut forge;
                /* Grab a copy of the item */
                object_copy(q_ptr, o_ptr);
                /* Describe it */
                object_desc(o_name.as_mut_ptr(), q_ptr, 1 as libc::c_int,
                            3 as libc::c_int);
                /* Message */
                msg_format(b"You drop %s (%c).\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr(),
                           index_to_label(item) as libc::c_int);
                /* Remove it from the players inventory */
                inc_stack_size(item, -(255 as libc::c_int));
                /* Handle stuff */
                handle_stuff();
                /* Let the home carry it */
                item_pos = home_carry(q_ptr);
                /* Redraw the home */
                if item_pos >= 0 as libc::c_int {
                    store_top =
                        item_pos / 12 as libc::c_int * 12 as libc::c_int;
                    display_inventory();
                }
            }
        }
        /* Hack -- Redisplay store prices if charisma changes */
        if tmp_chr !=
               (*p_ptr).stat_use[5 as libc::c_int as usize] as libc::c_int {
            display_inventory();
        }
        /* Hack -- get kicked out of the store */
        if (*st_ptr).store_open >= turn {
            leave_store = 1 as libc::c_int as bool_
        }
    }
    /* Free turn XXX XXX XXX */
    energy_use = 0 as libc::c_int;
    /* Recreate the level only when needed */
    if recreate != 0 {
        /* Reinit wilderness to activate quests ... */
        (*p_ptr).oldpx = (*p_ptr).px;
        (*p_ptr).oldpy = (*p_ptr).py;
        (*p_ptr).leaving = 1 as libc::c_int as bool_
    }
    /* Hack -- Character is no longer in "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
    /* Hack -- Cancel automatic command */
    command_new = 0 as libc::c_int as s16b;
    /* Mega-Hack -- Clear the 'ignore-keymaps' list */
    memset(request_command_ignore_keymaps.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, 12 as libc::c_int as libc::c_ulong);
    /* Flush messages XXX XXX XXX */
    msg_print(0 as cptr);
    /* Clear the screen */
    Term_clear();
    /* Update everything */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x200000 as libc::c_long)) as u32b;
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as u32b;
    /* Redraw entire screen */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x2000000 as libc::c_long | 0x1000000 as libc::c_long)) as u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Shuffle one of the stores.
 */
#[no_mangle]
pub unsafe extern "C" fn store_shuffle(mut which: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* Ignore home */
    if which == 7 as libc::c_int { return }
    /* Ignoer Museum */
    if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as libc::c_long &
           0x400 as libc::c_long != 0 {
        return
    }
    /* Save the store index */
    cur_store_num = which;
    /* Activate that store */
    st_ptr =
        &mut *(*town_info.offset((*p_ptr).town_num as
                                     isize)).store.offset(cur_store_num as
                                                              isize) as
            *mut store_type;
    /* Pick a new owner */
    j = (*st_ptr).owner as libc::c_int;
    while j == (*st_ptr).owner as libc::c_int {
        (*st_ptr).owner =
            (*st_info.offset((*st_ptr).st_idx as
                                 isize)).owners[Rand_div(4 as libc::c_int) as
                                                    usize]
    }
    /* Activate the new owner */
    ot_ptr =
        &mut *ow_info.offset((*st_ptr).owner as isize) as *mut owner_type;
    /* Reset the owner data */
    (*st_ptr).insult_cur = 0 as libc::c_int as s16b;
    (*st_ptr).store_open = 0 as libc::c_int;
    (*st_ptr).good_buy = 0 as libc::c_int as s16b;
    (*st_ptr).bad_buy = 0 as libc::c_int as s16b;
    /* Hack -- discount all the items */
    i = 0 as libc::c_int;
    while i < (*st_ptr).stock_num as libc::c_int {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Get the item */
        o_ptr = &mut *(*st_ptr).stock.offset(i as isize) as *mut object_type;
        /* Hack -- Sell all old items for "half price" */
        if (*o_ptr).art_name == 0 {
            (*o_ptr).discount = 50 as libc::c_int as byte_hack
        }
        /* Hack -- Items are no longer "fixed price" */
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int & !(0x2 as libc::c_int)) as
                byte_hack;
        /* Mega-Hack -- Note that the item is "on sale" */
        (*o_ptr).note =
            quark_add(b"on sale\x00" as *const u8 as *const libc::c_char) as
                u16b;
        i += 1
    };
}
/*
 * Maintain the inventory at the stores.
 */
#[no_mangle]
pub unsafe extern "C" fn store_maint(mut town_num: libc::c_int,
                                     mut store_num: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut tries: libc::c_int = 100 as libc::c_int;
    let mut old_rating: libc::c_int = rating as libc::c_int;
    cur_store_num = store_num;
    /* Ignore home */
    if store_num == 7 as libc::c_int { return }
    /* Activate that store */
    st_ptr =
        &mut *(*town_info.offset(town_num as
                                     isize)).store.offset(store_num as isize)
            as *mut store_type;
    /* Ignoer Museum */
    if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as libc::c_long &
           0x400 as libc::c_long != 0 {
        return
    }
    /* Activate the owner */
    ot_ptr =
        &mut *ow_info.offset((*st_ptr).owner as isize) as *mut owner_type;
    /* Store keeper forgives the player */
    (*st_ptr).insult_cur = 0 as libc::c_int as s16b;
    /* Mega-Hack -- prune the black market */
    if (*st_info.offset((*st_ptr).st_idx as isize)).flags1 as libc::c_long &
           0x80 as libc::c_long != 0 {
        /* Destroy crappy black market items */
        j = (*st_ptr).stock_num as libc::c_int - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            let mut o_ptr: *mut object_type =
                &mut *(*st_ptr).stock.offset(j as isize) as *mut object_type;
            /* Destroy crappy items */
            if black_market_crap(o_ptr) != 0 {
                /* Destroy the item */
                store_item_increase(j,
                                    0 as libc::c_int -
                                        (*o_ptr).number as libc::c_int);
                store_item_optimize(j);
            }
            j -= 1
        }
    }
    /* Choose the number of slots to keep */
    j = (*st_ptr).stock_num as libc::c_int;
    /* Sell a few items */
    j = j - (Rand_div(9 as libc::c_int) + 1 as libc::c_int);
    /* Never keep more than "STORE_MAX_KEEP" slots */
    if j > 18 as libc::c_int { j = 18 as libc::c_int }
    /* Always "keep" at least "STORE_MIN_KEEP" items */
    if j < 6 as libc::c_int { j = 6 as libc::c_int }
    /* Hack -- prevent "underflow" */
    if j < 0 as libc::c_int { j = 0 as libc::c_int }
    /* Destroy objects until only "j" slots are left */
    while (*st_ptr).stock_num as libc::c_int > j { store_delete(); }
    /* Choose the number of slots to fill */
    j = (*st_ptr).stock_num as libc::c_int;
    /* Buy some more items */
    j = j + (Rand_div(9 as libc::c_int) + 1 as libc::c_int);
    /* Never keep more than "STORE_MAX_KEEP" slots */
    if j > 18 as libc::c_int { j = 18 as libc::c_int }
    /* Always "keep" at least "STORE_MIN_KEEP" items */
    if j < 6 as libc::c_int { j = 6 as libc::c_int }
    /* Hack -- prevent "overflow" */
    if j >= (*st_ptr).stock_size as libc::c_int {
        j = (*st_ptr).stock_size as libc::c_int - 1 as libc::c_int
    }
    /* Acquire some new items */
    while ((*st_ptr).stock_num as libc::c_int) < j && tries != 0 {
        store_create();
        tries -= 1
    }
    /* Hack -- Restore the rating */
    rating = old_rating as s16b;
}
/*
 * Initialize the stores
 */
#[no_mangle]
pub unsafe extern "C" fn store_init(mut town_num: libc::c_int,
                                    mut store_num: libc::c_int) {
    let mut k: libc::c_int = 0;
    cur_store_num = store_num;
    /* Activate that store */
    st_ptr =
        &mut *(*town_info.offset(town_num as
                                     isize)).store.offset(store_num as isize)
            as *mut store_type;
    /* Pick an owner */
    (*st_ptr).owner =
        (*st_info.offset((*st_ptr).st_idx as
                             isize)).owners[Rand_div(4 as libc::c_int) as
                                                usize];
    /* Activate the new owner */
    ot_ptr =
        &mut *ow_info.offset((*st_ptr).owner as isize) as *mut owner_type;
    /* Initialize the store */
    (*st_ptr).store_open = 0 as libc::c_int;
    (*st_ptr).insult_cur = 0 as libc::c_int as s16b;
    (*st_ptr).good_buy = 0 as libc::c_int as s16b;
    (*st_ptr).bad_buy = 0 as libc::c_int as s16b;
    /* Nothing in stock */
    (*st_ptr).stock_num = 0 as libc::c_int as byte_hack;
    /*
	 * MEGA-HACK - Last visit to store is
	 * BEFORE player birth to enable store restocking
	 */
    (*st_ptr).last_visit =
        (-(100 as libc::c_long) * 1000 as libc::c_int as libc::c_long) as
            s32b;
    /* Clear any old items */
    k = 0 as libc::c_int;
    while k < (*st_ptr).stock_size as libc::c_int {
        object_wipe(&mut *(*st_ptr).stock.offset(k as isize));
        k += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn move_to_black_market(mut o_ptr: *mut object_type) {
    st_ptr =
        &mut *(*town_info.offset((*p_ptr).town_num as
                                     isize)).store.offset(6 as libc::c_int as
                                                              isize) as
            *mut store_type;
    (*o_ptr).ident =
        ((*o_ptr).ident as libc::c_int | 0x10 as libc::c_int) as byte_hack;
    store_carry(o_ptr);
    object_wipe(o_ptr);
    /* Don't leave a bogus object behind... */
}
/*
 * Enter the home, and interact with it from the dungeon (trump magic).
 *
 * Note that we use the standard "request_command()" function
 * to get a command, allowing us to use "command_arg" and all
 * command macros and other nifty stuff, but we use the special
 * "shopping" argument, to force certain commands to be converted
 * into other commands, normally, we convert "p" (pray) and "m"
 * (cast magic) into "g" (get), and "s" (search) into "d" (drop).
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_home_trump() {
    let mut which: libc::c_int = 0;
    let mut maintain_num: libc::c_int = 0;
    let mut tmp_chr: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut town_num: libc::c_int = 0;
    /* Extract the store code */
    which = 7 as libc::c_int;
    if (*p_ptr).town_num != 0 {
        town_num = (*p_ptr).town_num as libc::c_int
    } else { town_num = 1 as libc::c_int }
    /* Hack -- Check the "locked doors" */
    if (*(*town_info.offset(town_num as
                                isize)).store.offset(which as
                                                         isize)).store_open >=
           turn {
        msg_print(b"The doors are locked.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Calculate the number of store maintainances since the last visit */
    maintain_num =
        ((turn -
              (*(*town_info.offset(town_num as
                                       isize)).store.offset(which as
                                                                isize)).last_visit)
             as libc::c_long /
             (10 as libc::c_long * 1000 as libc::c_int as libc::c_long)) as
            libc::c_int;
    /* Maintain the store max. 10 times */
    if maintain_num > 10 as libc::c_int { maintain_num = 10 as libc::c_int }
    if maintain_num != 0 {
        /* Maintain the store */
        i = 0 as libc::c_int;
        while i < maintain_num { store_maint(town_num, which); i += 1 }
        /* Save the visit */
        (*(*town_info.offset(town_num as
                                 isize)).store.offset(which as
                                                          isize)).last_visit =
            turn
    }
    /* Forget the lite */
	/* forget_lite(); */
    /* Forget the view */
    forget_view();
    /* Hack -- Character is in "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* No command argument */
    command_arg = 0 as libc::c_int as s16b;
    /* No repeated command */
    command_rep = 0 as libc::c_int as s16b;
    /* No automatic command */
    command_new = 0 as libc::c_int as s16b;
    /* Save the store number */
    cur_store_num = which;
    /* Save the store and owner pointers */
    st_ptr =
        &mut *(*town_info.offset(town_num as
                                     isize)).store.offset(cur_store_num as
                                                              isize) as
            *mut store_type;
    ot_ptr =
        &mut *ow_info.offset((*st_ptr).owner as isize) as *mut owner_type;
    /* Start at the beginning */
    store_top = 0 as libc::c_int;
    /* Display the store */
    display_store();
    /* Mega-Hack -- Ignore keymaps on store action letters */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        let mut ba_ptr: *mut store_action_type =
            &mut *ba_info.offset(*(*st_info.offset((*st_ptr).st_idx as
                                                       isize)).actions.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)
                                     as isize) as *mut store_action_type;
        *request_command_ignore_keymaps.as_mut_ptr().offset((2 as libc::c_int
                                                                 * i) as
                                                                isize) =
            (*ba_ptr).letter;
        *request_command_ignore_keymaps.as_mut_ptr().offset((2 as libc::c_int
                                                                 * i +
                                                                 1 as
                                                                     libc::c_int)
                                                                as isize) =
            (*ba_ptr).letter_aux;
        i += 1
    }
    /* Do not leave */
    leave_store = 0 as libc::c_int as bool_;
    /* Interact with player */
    while leave_store == 0 {
        /* Hack -- Clear line 1 */
        prt(b"\x00" as *const u8 as *const libc::c_char, 1 as libc::c_int,
            0 as libc::c_int);
        /* Hack -- Check the charisma */
        tmp_chr = (*p_ptr).stat_use[5 as libc::c_int as usize] as libc::c_int;
        /* Clear */
        clear_from(21 as libc::c_int);
        /* Basic commands */
        prt(b" ESC) Exit from Building.\x00" as *const u8 as
                *const libc::c_char, 22 as libc::c_int, 0 as libc::c_int);
        /* Browse if necessary */
        if (*st_ptr).stock_num as libc::c_int > 12 as libc::c_int {
            prt(b" SPACE) Next page of stock\x00" as *const u8 as
                    *const libc::c_char, 23 as libc::c_int, 0 as libc::c_int);
        }
        /* Home commands */
        if cur_store_num == 7 as libc::c_int {
            prt(b" g) Get an item.\x00" as *const u8 as *const libc::c_char,
                22 as libc::c_int, 31 as libc::c_int);
            prt(b" d) Drop an item.\x00" as *const u8 as *const libc::c_char,
                23 as libc::c_int, 31 as libc::c_int);
        } else {
            /* Shop commands XXX XXX XXX */
            prt(b" p) Purchase an item.\x00" as *const u8 as
                    *const libc::c_char, 22 as libc::c_int,
                31 as libc::c_int);
            prt(b" s) Sell an item.\x00" as *const u8 as *const libc::c_char,
                23 as libc::c_int, 31 as libc::c_int);
        }
        /* Add in the eXamine option */
        prt(b" x) eXamine an item.\x00" as *const u8 as *const libc::c_char,
            22 as libc::c_int, 56 as libc::c_int);
        /* Prompt */
        prt(b"You may: \x00" as *const u8 as *const libc::c_char,
            21 as libc::c_int, 0 as libc::c_int);
        /* Get a command */
        request_command(1 as libc::c_int);
        /* Process the command */
        store_process_command();
        /* Hack -- Character is still in "icky" mode */
        character_icky = 1 as libc::c_int as bool_;
        /* Notice stuff */
        notice_stuff();
        /* Handle stuff */
        handle_stuff();
        /* XXX XXX XXX Pack Overflow */
        if (*p_ptr).inventory[23 as libc::c_int as usize].k_idx != 0 {
            let mut item: libc::c_int = 23 as libc::c_int;
            let mut o_ptr: *mut object_type =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
                    *mut object_type;
            /* Hack -- Flee from the store */
            if cur_store_num != 7 as libc::c_int {
                /* Message */
                msg_print(b"Your pack is so full that you flee the store...\x00"
                              as *const u8 as *const libc::c_char);
                /* Leave */
                leave_store = 1 as libc::c_int as bool_
            } else if store_check_num(o_ptr) == 0 {
                /* Hack -- Flee from the home */
                /* Message */
                msg_print(b"Your pack is so full that you flee your home...\x00"
                              as *const u8 as *const libc::c_char);
                /* Leave */
                leave_store = 1 as libc::c_int as bool_
            } else {
                /* Hack -- Drop items into the home */
                let mut item_pos: libc::c_int = 0;
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
                /* Give a message */
                msg_print(b"Your pack overflows!\x00" as *const u8 as
                              *const libc::c_char);
                /* Get local object */
                q_ptr = &mut forge;
                /* Grab a copy of the item */
                object_copy(q_ptr, o_ptr);
                /* Describe it */
                object_desc(o_name.as_mut_ptr(), q_ptr, 1 as libc::c_int,
                            3 as libc::c_int);
                /* Message */
                msg_format(b"You drop %s (%c).\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr(),
                           index_to_label(item) as libc::c_int);
                /* Remove it from the players inventory */
                inc_stack_size(item, -(255 as libc::c_int));
                /* Handle stuff */
                handle_stuff();
                /* Let the home carry it */
                item_pos = home_carry(q_ptr);
                /* Redraw the home */
                if item_pos >= 0 as libc::c_int {
                    store_top =
                        item_pos / 12 as libc::c_int * 12 as libc::c_int;
                    display_inventory();
                }
            }
        }
        /* Hack -- Redisplay store prices if charisma changes */
        if tmp_chr !=
               (*p_ptr).stat_use[5 as libc::c_int as usize] as libc::c_int {
            display_inventory();
        }
        /* Hack -- get kicked out of the store */
        if (*st_ptr).store_open >= turn {
            leave_store = 1 as libc::c_int as bool_
        }
    }
    /* Hack -- Character is no longer in "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
    /* Hack -- Cancel automatic command */
    command_new = 0 as libc::c_int as s16b;
    /* Mega-Hack -- Clear the 'ignore-keymaps' list */
    memset(request_command_ignore_keymaps.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, 12 as libc::c_int as libc::c_ulong);
    /* Flush messages XXX XXX XXX */
    msg_print(0 as cptr);
    /* Clear the screen */
    Term_clear();
    /* Update everything */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x200000 as libc::c_long)) as u32b;
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as u32b;
    /* Redraw entire screen */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x2000000 as libc::c_long | 0x1000000 as libc::c_long)) as u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
}
unsafe extern "C" fn pay_for_requested_item(mut value: libc::c_int,
                                            mut q_ptr: *mut object_type) {
    msg_format(b"It\'ll cost %i gold pieces. \x00" as *const u8 as
                   *const libc::c_char, value);
    if get_check(b"Do you wish to pay?\x00" as *const u8 as
                     *const libc::c_char) != 0 {
        if (*p_ptr).au < value {
            msg_print(b"You don\'t have enough money for it.\x00" as *const u8
                          as *const libc::c_char);
        } else if store_carry(q_ptr) != -(1 as libc::c_int) {
            msg_print(b"The item has arrived in the Black Market.\x00" as
                          *const u8 as *const libc::c_char);
            (*p_ptr).au -= value;
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x100 as libc::c_long) as
                    u32b
        } else {
            msg_print(b"There isn\'t enough room for it.\x00" as *const u8 as
                          *const libc::c_char);
        }
    };
}
/*
 * Request item for merchants
 */
#[no_mangle]
pub unsafe extern "C" fn store_request_item() {
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut name: [libc::c_char; 80] = [0; 80];
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
    let mut q_ptr: *mut object_type = &mut forge;
    let mut ost_ptr: *mut store_type = st_ptr;
    /* Get the Black Market */
    st_ptr =
        &mut *(*town_info.offset((*p_ptr).town_num as
                                     isize)).store.offset(6 as libc::c_int as
                                                              isize) as
            *mut store_type;
    /* Make an empty string */
    buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    /* Ask for the wish */
    if get_string(b"Request what? \x00" as *const u8 as *const libc::c_char,
                  buf.as_mut_ptr(), 80 as libc::c_int) == 0 {
        st_ptr = ost_ptr;
        return
    }
    clean_wish_name(buf.as_mut_ptr(), name.as_mut_ptr());
    if test_object_wish(name.as_mut_ptr(), q_ptr, &mut forge,
                        b"request\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char) != 0 {
        let mut value: libc::c_int =
            object_value_real(q_ptr) * 5 as libc::c_int;
        /* Pay for the delivery */
        pay_for_requested_item(value, q_ptr);
        /* Don't search any more */
        st_ptr = ost_ptr;
        return
    }
    st_ptr = ost_ptr;
}
