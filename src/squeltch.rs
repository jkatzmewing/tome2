use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut ANGBAND_DIR_USER: cptr;
    #[no_mangle]
    static mut automatizer_enabled: bool_;
    #[no_mangle]
    static mut hook_file: *mut FILE;
    #[no_mangle]
    fn show_file(name: cptr, what: cptr, line: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn file_exist(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn input_box(text: cptr, y: libc::c_int, x: libc::c_int,
                 buf: *mut libc::c_char, max: libc::c_int) -> bool_;
    #[no_mangle]
    fn draw_box(y: libc::c_int, x: libc::c_int, h: libc::c_int,
                w: libc::c_int);
    #[no_mangle]
    fn display_list(y: libc::c_int, x: libc::c_int, h: libc::c_int,
                    w: libc::c_int, title: cptr, list: *mut cptr,
                    max: libc::c_int, begin: libc::c_int, sel: libc::c_int,
                    sel_color: byte_hack);
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn display_message(x: libc::c_int, y: libc::c_int, split: libc::c_int,
                       color: byte_hack, t: cptr);
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn screen_save();
    #[no_mangle]
    fn screen_load();
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn c_prt(attr: byte_hack, str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn msg_box(text: cptr, y: libc::c_int, x: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn string_exec_lua(file: *mut libc::c_char) -> cptr;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn lua_gettop(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L_0: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_getglobal(L_0: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_call(L_0: *mut lua_State, nargs: libc::c_int,
                nresults: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolua_tag(L_0: *mut lua_State, type_0: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn tolua_getnumber(L_0: *mut lua_State, narg: libc::c_int,
                       def: libc::c_long) -> libc::c_long;
    #[no_mangle]
    fn tolua_pushnumber(L_0: *mut lua_State, value: libc::c_long);
    #[no_mangle]
    fn tolua_pushusertype(L_0: *mut lua_State, value: *mut libc::c_void,
                          tag: libc::c_int);
    /* File: squeltch.c */
    /* Purpose: Automatizer */
    /*
 * Copyright (c) 2002 DarkGod
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
    #[no_mangle]
    static mut L: *mut lua_State;
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
/*
 * The functions here use direct lua stack manipulation for calls instead of
 * exec_lua(format()) because string manipulations are too slow for such
 * functions
 */
/* Check the floor for "crap" */
#[no_mangle]
pub unsafe extern "C" fn squeltch_grid() {
    let mut oldtop: libc::c_int = 0;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    if automatizer_enabled == 0 { return }
    oldtop = lua_gettop(L);
    /* Scan the pile of objects */
    this_o_idx =
        (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).o_idx;
    while this_o_idx != 0 {
        /* Acquire object */
        let mut o_ptr: *mut object_type =
            &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* We've now seen one of these */
        if (*k_info.offset((*o_ptr).k_idx as isize)).flavor == 0 {
            object_aware(o_ptr);
        }
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Push the function */
        lua_settop(L, oldtop);
        lua_getglobal(L,
                      b"apply_rules\x00" as *const u8 as *const libc::c_char);
        /* Push the args */
        tolua_pushusertype(L, o_ptr as *mut libc::c_void,
                           tolua_tag(L,
                                     b"object_type\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char));
        tolua_pushnumber(L, -(this_o_idx as libc::c_int) as libc::c_long);
        /* Call the function */
        if lua_call(L, 2 as libc::c_int, 0 as libc::c_int) != 0 {
            cmsg_format(10 as libc::c_int as byte_hack,
                        b"ERROR in lua_call while calling \'apply_rules\'.\x00"
                            as *const u8 as *const libc::c_char);
            lua_settop(L, oldtop);
            return
        }
        this_o_idx = next_o_idx
    }
    lua_settop(L, oldtop);
}
/* Check the inventory for "crap" */
#[no_mangle]
pub unsafe extern "C" fn squeltch_inventory() {
    let mut oldtop: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut num_iter: libc::c_int = 0 as libc::c_int;
    let mut found: bool_ = 1 as libc::c_int as bool_;
    if automatizer_enabled == 0 { return }
    oldtop = lua_gettop(L);
    while found as libc::c_int != 0 &&
              {
                  let fresh0 = num_iter;
                  num_iter = num_iter + 1;
                  (fresh0) < 100 as libc::c_int
              } {
        /* Sometimes an index in the inventory is skipped */
        found = 0 as libc::c_int as bool_;
        i = 0 as libc::c_int;
        while i < 23 as libc::c_int {
            let mut o_ptr: *mut object_type =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                    *mut object_type;
            /* Push the function */
            lua_settop(L, oldtop);
            lua_getglobal(L,
                          b"apply_rules\x00" as *const u8 as
                              *const libc::c_char);
            /* Push the args */
            tolua_pushusertype(L, o_ptr as *mut libc::c_void,
                               tolua_tag(L,
                                         b"object_type\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char));
            tolua_pushnumber(L, i as libc::c_long);
            /* Call the function */
            if lua_call(L, 2 as libc::c_int, 1 as libc::c_int) != 0 {
                cmsg_format(10 as libc::c_int as byte_hack,
                            b"ERROR in lua_call while calling \'apply_rules\'.\x00"
                                as *const u8 as *const libc::c_char);
                lua_settop(L, oldtop);
                return
            }
            /* Did it return TRUE */
            if tolua_getnumber(L, -(lua_gettop(L) - oldtop),
                               0 as libc::c_int as libc::c_long) != 0 {
                found = 1 as libc::c_int as bool_;
                break ;
            } else { i += 1 }
        }
    }
    if num_iter >= 100 as libc::c_int {
        cmsg_format(10 as libc::c_int as byte_hack,
                    b"\'apply_rules\' ran too often.\x00" as *const u8 as
                        *const libc::c_char);
    }
    lua_settop(L, oldtop);
}
/* ********************* The interface **********************/
unsafe extern "C" fn get_rule_list(mut max: *mut libc::c_int) -> *mut cptr {
    let mut list: *mut cptr = 0 as *mut cptr;
    let mut i: libc::c_int = 0;
    *max =
        exec_lua(b"return __rules_max\x00" as *const u8 as *const libc::c_char
                     as *mut libc::c_char);
    list =
        memset(ralloc((*max as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (*max as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                    as libc::c_ulong)) as
            *mut cptr;
    i = 0 as libc::c_int;
    while i < *max {
        let ref mut fresh1 = *list.offset(i as isize);
        *fresh1 =
            string_exec_lua(format(b"return __rules[%d].table.args.name\x00"
                                       as *const u8 as *const libc::c_char,
                                   i));
        i += 1
    }
    return list;
}
static mut types_list: [cptr; 22] =
    [b"and\x00" as *const u8 as *const libc::c_char,
     b"or\x00" as *const u8 as *const libc::c_char,
     b"not\x00" as *const u8 as *const libc::c_char,
     b"name\x00" as *const u8 as *const libc::c_char,
     b"contain\x00" as *const u8 as *const libc::c_char,
     b"inscribed\x00" as *const u8 as *const libc::c_char,
     b"discount\x00" as *const u8 as *const libc::c_char,
     b"symbol\x00" as *const u8 as *const libc::c_char,
     b"state\x00" as *const u8 as *const libc::c_char,
     b"status\x00" as *const u8 as *const libc::c_char,
     b"tval\x00" as *const u8 as *const libc::c_char,
     b"sval\x00" as *const u8 as *const libc::c_char,
     b"race\x00" as *const u8 as *const libc::c_char,
     b"subrace\x00" as *const u8 as *const libc::c_char,
     b"class\x00" as *const u8 as *const libc::c_char,
     b"level\x00" as *const u8 as *const libc::c_char,
     b"skill\x00" as *const u8 as *const libc::c_char,
     b"ability\x00" as *const u8 as *const libc::c_char,
     b"inventory\x00" as *const u8 as *const libc::c_char,
     b"equipment\x00" as *const u8 as *const libc::c_char,
     b"comment\x00" as *const u8 as *const libc::c_char, 0 as cptr];
/* Create a new rule */
unsafe extern "C" fn automatizer_new_rule() -> libc::c_int {
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut begin: libc::c_int = 0 as libc::c_int;
    let mut sel: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    /* Get the number of types */
    max = 0 as libc::c_int;
    while !types_list[max as usize].is_null() { max += 1 }
    loop  {
        Term_clear();
        Term_get_size(&mut wid, &mut hgt);
        display_list(0 as libc::c_int, 0 as libc::c_int,
                     hgt - 1 as libc::c_int, 15 as libc::c_int,
                     b"Rule types\x00" as *const u8 as *const libc::c_char,
                     types_list.as_mut_ptr(), max, begin, sel,
                     13 as libc::c_int as byte_hack);
        exec_lua(format(b"auto_aux:display_desc(\'%s\')\x00" as *const u8 as
                            *const libc::c_char, types_list[sel as usize]));
        c_prt(11 as libc::c_int as byte_hack,
              b"Example:\x00" as *const u8 as *const libc::c_char,
              5 as libc::c_int, 17 as libc::c_int);
        exec_lua(format(b"xml.write_out_y = 6; xml.write_out_x = 16; xml.write_out_h = %d; xml.write_out_w = %d; xml.write_y = 0; xml.write_x = 0; xml:display_xml(auto_aux.types_desc[\'%s\'][2][1], \'\')\x00"
                            as *const u8 as *const libc::c_char,
                        hgt - 3 as libc::c_int - 5 as libc::c_int,
                        wid - 1 as libc::c_int - 15 as libc::c_int -
                            1 as libc::c_int, types_list[sel as usize]));
        c = inkey();
        if c as libc::c_int == '\u{1b}' as i32 { break ; }
        if c as libc::c_int == '8' as i32 {
            sel -= 1;
            if sel < 0 as libc::c_int {
                sel = max - 1 as libc::c_int;
                begin = max - hgt;
                if begin < 0 as libc::c_int { begin = 0 as libc::c_int }
            }
            if sel < begin { begin = sel }
        } else if c as libc::c_int == '2' as i32 {
            sel += 1;
            if sel >= max { sel = 0 as libc::c_int; begin = 0 as libc::c_int }
            if sel >= begin + hgt - 1 as libc::c_int { begin += 1 }
        } else if c as libc::c_int == '\r' as i32 { return sel }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn adjust_begin(mut begin: *mut libc::c_int,
                                  mut sel: *mut libc::c_int,
                                  mut max: libc::c_int,
                                  mut hgt: libc::c_int) {
    if *sel < 0 as libc::c_int {
        *sel = max - 1 as libc::c_int;
        *begin = *sel - hgt + 3 as libc::c_int;
        if *begin < 0 as libc::c_int { *begin = 0 as libc::c_int }
    }
    if *sel < *begin { *begin = *sel }
    if *sel >= max { *sel = 0 as libc::c_int; *begin = 0 as libc::c_int }
    if *sel >= *begin + hgt - 2 as libc::c_int { *begin += 1 };
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_automatizer() {
    let mut wid: libc::c_int = 0 as libc::c_int;
    let mut hgt: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut list: *mut cptr = 0 as *mut cptr;
    let mut max: libc::c_int = 0;
    let mut begin: libc::c_int = 0 as libc::c_int;
    let mut sel: libc::c_int = 0 as libc::c_int;
    let mut active: libc::c_int = 0 as libc::c_int;
    let mut keys: cptr = 0 as *const libc::c_char;
    let mut keys2: cptr = 0 as *const libc::c_char;
    let mut keys3: cptr = 0 as *const libc::c_char;
    Term_get_size(&mut wid, &mut hgt);
    if automatizer_enabled == 0 {
        if msg_box(b"Automatizer is currently disabled, enable it? (y/n)\x00"
                       as *const u8 as *const libc::c_char,
                   hgt / 2 as libc::c_int, wid / 2 as libc::c_int) as
               libc::c_int == 'y' as i32 {
            automatizer_enabled = 1 as libc::c_int as bool_
        } else { return }
    }
    screen_save();
    exec_lua(format(b"auto_aux:adjust_current(%d)\x00" as *const u8 as
                        *const libc::c_char, sel));
    loop 
         /* Error */
         {
        Term_clear();
        Term_get_size(&mut wid, &mut hgt);
        list = get_rule_list(&mut max);
        display_list(0 as libc::c_int, 0 as libc::c_int,
                     hgt - 1 as libc::c_int, 15 as libc::c_int,
                     b"Rules\x00" as *const u8 as *const libc::c_char, list,
                     max, begin, sel,
                     if active == 0 as libc::c_int {
                         13 as libc::c_int
                     } else { 5 as libc::c_int } as byte_hack);
        rnfree(list as vptr,
               (max as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                    as libc::c_ulong));
        draw_box(0 as libc::c_int, 15 as libc::c_int, hgt - 4 as libc::c_int,
                 wid - 1 as libc::c_int - 15 as libc::c_int);
        if active == 1 as libc::c_int {
            keys =
                b"#Bup#W/#Bdown#W/#Bleft#W/#Bright#W to navitage rule, #B9#W/#B3#W/#B7#W/#B1#W to scroll\x00"
                    as *const u8 as *const libc::c_char;
            keys2 =
                b"#Btab#W for switch, #Ba#Wdd clause, #Bd#Welete clause/rule\x00"
                    as *const u8 as *const libc::c_char;
            keys3 =
                b"#Bx#W to toggle english/xml, #G?#W for Automatizer help\x00"
                    as *const u8 as *const libc::c_char;
            exec_lua(b"xml.write_active = not nil\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char);
        } else {
            keys =
                b"#Bup#W/#Bdown#W to scroll, #Btab#W to switch to the rule window\x00"
                    as *const u8 as *const libc::c_char;
            keys2 =
                b"#Bu#W/#Bd#W to move rules, #Bn#Wew rule, #Br#Wename rule, #Bs#Wave rules\x00"
                    as *const u8 as *const libc::c_char;
            keys3 =
                b"#Rk#W to #rdisable#W the automatizer, #G?#W for Automatizer help\x00"
                    as *const u8 as *const libc::c_char;
            exec_lua(b"xml.write_active = nil\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char);
        }
        display_message(17 as libc::c_int, hgt - 3 as libc::c_int,
                        strlen(keys) as libc::c_int,
                        1 as libc::c_int as byte_hack, keys);
        display_message(17 as libc::c_int, hgt - 2 as libc::c_int,
                        strlen(keys2) as libc::c_int,
                        1 as libc::c_int as byte_hack, keys2);
        display_message(17 as libc::c_int, hgt - 1 as libc::c_int,
                        strlen(keys3) as libc::c_int,
                        1 as libc::c_int as byte_hack, keys3);
        if max != 0 {
            exec_lua(format(b"xml.write_out_y = 1; xml.write_out_x = 16; xml.write_out_h = %d; xml.write_out_w = %d; xml.write_y = 0; xml.write_x = 0; xml:display_xml(__rules[%d].table, \'\')\x00"
                                as *const u8 as *const libc::c_char,
                            hgt - 4 as libc::c_int - 1 as libc::c_int,
                            wid - 1 as libc::c_int - 15 as libc::c_int -
                                1 as libc::c_int, sel));
        }
        c = inkey();
        if c as libc::c_int == '\u{1b}' as i32 { break ; }
        if active == 0 as libc::c_int {
            if c as libc::c_int == '?' as i32 {
                screen_save();
                show_file(b"automat.txt\x00" as *const u8 as
                              *const libc::c_char,
                          b"Automatizer help\x00" as *const u8 as
                              *const libc::c_char, 0 as libc::c_int,
                          0 as libc::c_int);
                screen_load();
            } else if c as libc::c_int == '8' as i32 {
                if max == 0 { continue ; }
                sel -= 1;
                adjust_begin(&mut begin, &mut sel, max, hgt);
                exec_lua(format(b"auto_aux:adjust_current(%d)\x00" as
                                    *const u8 as *const libc::c_char, sel));
            } else if c as libc::c_int == '2' as i32 {
                if max == 0 { continue ; }
                sel += 1;
                adjust_begin(&mut begin, &mut sel, max, hgt);
                exec_lua(format(b"auto_aux:adjust_current(%d)\x00" as
                                    *const u8 as *const libc::c_char, sel));
            } else if c as libc::c_int == 'u' as i32 {
                if max == 0 { continue ; }
                sel =
                    exec_lua(format(b"return auto_aux:move_up(%d)\x00" as
                                        *const u8 as *const libc::c_char,
                                    sel));
                adjust_begin(&mut begin, &mut sel, max, hgt);
                exec_lua(format(b"auto_aux:adjust_current(%d)\x00" as
                                    *const u8 as *const libc::c_char, sel));
            } else if c as libc::c_int == 'd' as i32 {
                if max == 0 { continue ; }
                sel =
                    exec_lua(format(b"return auto_aux:move_down(%d)\x00" as
                                        *const u8 as *const libc::c_char,
                                    sel));
                adjust_begin(&mut begin, &mut sel, max, hgt);
                exec_lua(format(b"auto_aux:adjust_current(%d)\x00" as
                                    *const u8 as *const libc::c_char, sel));
            } else if c as libc::c_int == 'n' as i32 {
                let mut name: [libc::c_char; 20] =
                    ['\u{0}' as i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                let mut typ: libc::c_char = 0;
                sprintf(name.as_mut_ptr(),
                        b"No name\x00" as *const u8 as *const libc::c_char);
                if input_box(b"Name?\x00" as *const u8 as *const libc::c_char,
                             hgt / 2 as libc::c_int, wid / 2 as libc::c_int,
                             name.as_mut_ptr(),
                             (::std::mem::size_of::<[libc::c_char; 20]>() as
                                  libc::c_ulong).wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong)
                                 as libc::c_int) != 0 {
                    let mut styp: cptr =
                        b"nothing\x00" as *const u8 as *const libc::c_char;
                    typ =
                        msg_box(b"[D]estroy, [P]ickup, [I]nscribe, [N]othing rule?\x00"
                                    as *const u8 as *const libc::c_char,
                                hgt / 2 as libc::c_int,
                                wid / 2 as libc::c_int);
                    if typ as libc::c_int == 'd' as i32 ||
                           typ as libc::c_int == 'D' as i32 {
                        styp =
                            b"destroy\x00" as *const u8 as *const libc::c_char
                    } else if typ as libc::c_int == 'p' as i32 ||
                                  typ as libc::c_int == 'P' as i32 {
                        styp =
                            b"pickup\x00" as *const u8 as *const libc::c_char
                    } else if typ as libc::c_int == 'i' as i32 ||
                                  typ as libc::c_int == 'I' as i32 {
                        styp =
                            b"inscribe\x00" as *const u8 as
                                *const libc::c_char
                    }
                    exec_lua(format(b"auto_aux:new_rule(%d, \'%s\', \'%s\', \'\'); auto_aux:adjust_current(%d)\x00"
                                        as *const u8 as *const libc::c_char,
                                    sel, name.as_mut_ptr(), styp, sel));
                    active = 1 as libc::c_int
                }
            } else if c as libc::c_int == 's' as i32 {
                let mut name_0: [libc::c_char; 30] =
                    ['\u{0}' as i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0];
                sprintf(name_0.as_mut_ptr(),
                        b"automat.atm\x00" as *const u8 as
                            *const libc::c_char);
                if !(input_box(b"Save name?\x00" as *const u8 as
                                   *const libc::c_char,
                               hgt / 2 as libc::c_int, wid / 2 as libc::c_int,
                               name_0.as_mut_ptr(),
                               (::std::mem::size_of::<[libc::c_char; 30]>() as
                                    libc::c_ulong).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
                                   as libc::c_int) != 0) {
                    continue ;
                }
                let mut buf: [libc::c_char; 1025] = [0; 1025];
                let mut ch: libc::c_char = 0;
                /* Build the filename */
                path_build(buf.as_mut_ptr(), 1024 as libc::c_int,
                           ANGBAND_DIR_USER, name_0.as_mut_ptr() as cptr);
                /* File type is "TEXT" */
                if file_exist(buf.as_mut_ptr()) != 0 {
                    c_put_str(1 as libc::c_int as byte_hack,
                              b"File exists, continue?[y/n]\x00" as *const u8
                                  as *const libc::c_char,
                              hgt / 2 as libc::c_int,
                              wid / 2 as libc::c_int - 14 as libc::c_int);
                    ch = inkey();
                    if ch as libc::c_int != 'Y' as i32 &&
                           ch as libc::c_int != 'y' as i32 {
                        continue ;
                    }
                }
                /* Open the non-existing file */
                hook_file =
                    my_fopen(buf.as_mut_ptr() as cptr,
                             b"w\x00" as *const u8 as *const libc::c_char);
                /* Invalid file */
                if hook_file.is_null() {
                    /* Message */
                    c_put_str(1 as libc::c_int as byte_hack,
                              b"Saving rules failed!       \x00" as *const u8
                                  as *const libc::c_char,
                              hgt / 2 as libc::c_int,
                              wid / 2 as libc::c_int - 14 as libc::c_int);
                    inkey();
                } else {
                    exec_lua(b"auto_aux:save_ruleset()\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char);
                    my_fclose(hook_file);
                    /* Overwrite query message */
                    c_put_str(1 as libc::c_int as byte_hack,
                              b"Saved rules in file        \x00" as *const u8
                                  as *const libc::c_char,
                              hgt / 2 as libc::c_int,
                              wid / 2 as libc::c_int - 14 as libc::c_int);
                    inkey();
                }
            } else if c as libc::c_int == 'r' as i32 {
                let mut name_1: [libc::c_char; 20] = [0; 20];
                if max == 0 { continue ; }
                sprintf(name_1.as_mut_ptr(),
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        string_exec_lua(format(b"return __rules[%d].table.args.name\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               sel)));
                if input_box(b"New name?\x00" as *const u8 as
                                 *const libc::c_char, hgt / 2 as libc::c_int,
                             wid / 2 as libc::c_int, name_1.as_mut_ptr(),
                             15 as libc::c_int) != 0 {
                    exec_lua(format(b"auto_aux:rename_rule(%d, \'%s\')\x00" as
                                        *const u8 as *const libc::c_char, sel,
                                    name_1.as_mut_ptr()));
                }
            } else if c as libc::c_int == 'k' as i32 {
                automatizer_enabled = 0 as libc::c_int as bool_;
                break ;
            } else if c as libc::c_int == '\t' as i32 {
                if max == 0 { continue ; }
                active = 1 as libc::c_int
            } else if c as libc::c_int == 'x' as i32 {
                exec_lua(b"xml.display_english = not xml.display_english\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
            }
        } else {
            if !(active == 1 as libc::c_int) { continue ; }
            if c as libc::c_int == '?' as i32 {
                screen_save();
                show_file(b"automat.txt\x00" as *const u8 as
                              *const libc::c_char,
                          b"Automatizer help\x00" as *const u8 as
                              *const libc::c_char, 0 as libc::c_int,
                          0 as libc::c_int);
                screen_load();
            } else if c as libc::c_int == '8' as i32 {
                exec_lua(b"auto_aux:go_up()\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char);
            } else if c as libc::c_int == '2' as i32 {
                exec_lua(b"auto_aux:go_down()\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char);
            } else if c as libc::c_int == '6' as i32 {
                exec_lua(b"auto_aux:go_right()\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char);
            } else if c as libc::c_int == '4' as i32 {
                exec_lua(format(b"auto_aux:go_left(%d)\x00" as *const u8 as
                                    *const libc::c_char, sel));
            } else if c as libc::c_int == '9' as i32 {
                exec_lua(b"auto_aux:scroll_up()\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char);
            } else if c as libc::c_int == '3' as i32 {
                exec_lua(b"auto_aux:scroll_down()\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char);
            } else if c as libc::c_int == '7' as i32 {
                exec_lua(b"auto_aux:scroll_left()\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char);
            } else if c as libc::c_int == '1' as i32 {
                exec_lua(b"auto_aux:scroll_right()\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char);
            } else if c as libc::c_int == 'a' as i32 {
                let mut s: libc::c_int = automatizer_new_rule();
                if s == -(1 as libc::c_int) { continue ; }
                exec_lua(format(b"auto_aux:add_child(\'%s\')\x00" as *const u8
                                    as *const libc::c_char,
                                types_list[s as usize]));
            } else if c as libc::c_int == 'd' as i32 {
                if max != 0 {
                    let mut new_sel: libc::c_int = 0;
                    new_sel =
                        exec_lua(format(b"return auto_aux:del_self(%d)\x00" as
                                            *const u8 as *const libc::c_char,
                                        sel));
                    if sel != new_sel && new_sel >= 0 as libc::c_int {
                        sel = new_sel;
                        adjust_begin(&mut begin, &mut sel, max, hgt);
                        exec_lua(format(b"auto_aux:adjust_current(%d)\x00" as
                                            *const u8 as *const libc::c_char,
                                        sel));
                    } else if new_sel == -(1 as libc::c_int) {
                        active = 0 as libc::c_int
                    }
                }
            } else if c as libc::c_int == '\t' as i32 {
                active = 0 as libc::c_int
            } else if c as libc::c_int == 'x' as i32 {
                exec_lua(b"xml.display_english = not xml.display_english\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
            }
        }
    }
    /* Recalculate the rules */
    exec_lua(b"auto_aux.regen_ruleset()\x00" as *const u8 as
                 *const libc::c_char as *mut libc::c_char);
    screen_load();
}
/* Add a new rule in an easy way */
#[no_mangle]
pub static mut automatizer_create: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub unsafe extern "C" fn automatizer_add_rule(mut o_ptr: *mut object_type,
                                              mut destroy: bool_) {
    let mut ch: libc::c_char = 0;
    let mut do_status: bool_ = 0 as libc::c_int as bool_;
    let mut type_0: cptr = b"destroy\x00" as *const u8 as *const libc::c_char;
    if destroy == 0 {
        type_0 = b"pickup\x00" as *const u8 as *const libc::c_char
    }
    while !(get_com(format(b"%s all of the same [T]ype, [F]amily or [N]ame, also use [S]tatus (%s)? \x00"
                               as *const u8 as *const libc::c_char,
                           if destroy as libc::c_int != 0 {
                               b"Destroy\x00" as *const u8 as
                                   *const libc::c_char
                           } else {
                               b"Pickup\x00" as *const u8 as
                                   *const libc::c_char
                           },
                           if do_status as libc::c_int != 0 {
                               b"Yes\x00" as *const u8 as *const libc::c_char
                           } else {
                               b"No\x00" as *const u8 as *const libc::c_char
                           }) as cptr, &mut ch) == 0) {
        if ch as libc::c_int == 'S' as i32 || ch as libc::c_int == 's' as i32
           {
            do_status = (do_status == 0) as libc::c_int as bool_
        } else if ch as libc::c_int == 'T' as i32 ||
                      ch as libc::c_int == 't' as i32 {
            call_lua(b"easy_add_rule\x00" as *const u8 as *const libc::c_char,
                     b"(s,s,d,O)\x00" as *const u8 as *const libc::c_char,
                     b"\x00" as *const u8 as *const libc::c_char, type_0,
                     b"tsval\x00" as *const u8 as *const libc::c_char,
                     do_status as libc::c_int, o_ptr);
            break ;
        } else if ch as libc::c_int == 'F' as i32 ||
                      ch as libc::c_int == 'f' as i32 {
            call_lua(b"easy_add_rule\x00" as *const u8 as *const libc::c_char,
                     b"(s,s,d,O)\x00" as *const u8 as *const libc::c_char,
                     b"\x00" as *const u8 as *const libc::c_char, type_0,
                     b"tval\x00" as *const u8 as *const libc::c_char,
                     do_status as libc::c_int, o_ptr);
            break ;
        } else {
            if !(ch as libc::c_int == 'N' as i32 ||
                     ch as libc::c_int == 'n' as i32) {
                continue ;
            }
            call_lua(b"easy_add_rule\x00" as *const u8 as *const libc::c_char,
                     b"(s,s,d,O)\x00" as *const u8 as *const libc::c_char,
                     b"\x00" as *const u8 as *const libc::c_char, type_0,
                     b"name\x00" as *const u8 as *const libc::c_char,
                     do_status as libc::c_int, o_ptr);
            break ;
        }
    };
}
