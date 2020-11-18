use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut testing_carry: bool_;
    #[no_mangle]
    static mut testing_stack: bool_;
    #[no_mangle]
    static mut fate_option: bool_;
    #[no_mangle]
    static mut always_small_level: bool_;
    #[no_mangle]
    static mut joke_monsters: bool_;
    #[no_mangle]
    static mut fast_autoroller: bool_;
    #[no_mangle]
    static mut auto_notes: bool_;
    #[no_mangle]
    static mut take_notes: bool_;
    #[no_mangle]
    static mut ironman_rooms: bool_;
    #[no_mangle]
    static mut point_based: bool_;
    #[no_mangle]
    static mut autoroll: bool_;
    #[no_mangle]
    static mut preserve: bool_;
    #[no_mangle]
    static mut maximize: bool_;
    #[no_mangle]
    static mut inventory_no_move: bool_;
    #[no_mangle]
    static mut linear_stats: bool_;
    #[no_mangle]
    static mut player_char_health: bool_;
    #[no_mangle]
    static mut auto_more: bool_;
    #[no_mangle]
    static mut autoload_old_colors: bool_;
    #[no_mangle]
    static mut exp_need: bool_;
    #[no_mangle]
    static mut option_ingame_help: bool_;
    #[no_mangle]
    static mut center_player: bool_;
    #[no_mangle]
    static mut view_special_lite: bool_;
    #[no_mangle]
    static mut view_granite_lite: bool_;
    #[no_mangle]
    static mut view_bright_lite: bool_;
    #[no_mangle]
    static mut view_yellow_lite: bool_;
    #[no_mangle]
    static mut hilite_player: bool_;
    #[no_mangle]
    static mut fresh_message: bool_;
    #[no_mangle]
    static mut fresh_after: bool_;
    #[no_mangle]
    static mut fresh_before: bool_;
    #[no_mangle]
    static mut flush_command: bool_;
    #[no_mangle]
    static mut flush_disturb: bool_;
    #[no_mangle]
    static mut flush_failure: bool_;
    #[no_mangle]
    static mut avoid_other: bool_;
    #[no_mangle]
    static mut avoid_shimmer: bool_;
    #[no_mangle]
    static mut avoid_abort: bool_;
    #[no_mangle]
    static mut view_reduce_view: bool_;
    #[no_mangle]
    static mut view_reduce_lite: bool_;
    #[no_mangle]
    static mut empty_levels: bool_;
    #[no_mangle]
    static mut small_levels: bool_;
    #[no_mangle]
    static mut stupid_monsters: bool_;
    #[no_mangle]
    static mut smart_cheat: bool_;
    #[no_mangle]
    static mut smart_learn: bool_;
    #[no_mangle]
    static mut plain_descriptions: bool_;
    #[no_mangle]
    static mut player_symbols: bool_;
    #[no_mangle]
    static mut flow_by_sound: bool_;
    #[no_mangle]
    static mut dungeon_stair: bool_;
    #[no_mangle]
    static mut dungeon_align: bool_;
    #[no_mangle]
    static mut monster_lite: bool_;
    #[no_mangle]
    static mut view_torch_grids: bool_;
    #[no_mangle]
    static mut view_perma_grids: bool_;
    #[no_mangle]
    static mut expand_list: bool_;
    #[no_mangle]
    static mut expand_look: bool_;
    #[no_mangle]
    static mut stack_allow_wands: bool_;
    #[no_mangle]
    static mut stack_allow_items: bool_;
    #[no_mangle]
    static mut auto_scum: bool_;
    #[no_mangle]
    static mut auto_haggle: bool_;
    #[no_mangle]
    static mut easy_tunnel: bool_;
    #[no_mangle]
    static mut easy_disarm: bool_;
    #[no_mangle]
    static mut easy_open: bool_;
    #[no_mangle]
    static mut disturb_pets: bool_;
    #[no_mangle]
    static mut confirm_stairs: bool_;
    #[no_mangle]
    static mut wear_confirm: bool_;
    #[no_mangle]
    static mut auto_destroy: bool_;
    #[no_mangle]
    static mut speak_unique: bool_;
    #[no_mangle]
    static mut last_words: bool_;
    #[no_mangle]
    static mut alert_failure: bool_;
    #[no_mangle]
    static mut alert_hitpoint: bool_;
    #[no_mangle]
    static mut disturb_other: bool_;
    #[no_mangle]
    static mut disturb_minor: bool_;
    #[no_mangle]
    static mut disturb_state: bool_;
    #[no_mangle]
    static mut disturb_detect: bool_;
    #[no_mangle]
    static mut disturb_panel: bool_;
    #[no_mangle]
    static mut disturb_near: bool_;
    #[no_mangle]
    static mut disturb_move: bool_;
    #[no_mangle]
    static mut find_examine: bool_;
    #[no_mangle]
    static mut find_cut: bool_;
    #[no_mangle]
    static mut find_ignore_doors: bool_;
    #[no_mangle]
    static mut find_ignore_stairs: bool_;
    #[no_mangle]
    static mut ring_bell: bool_;
    #[no_mangle]
    static mut show_details: bool_;
    #[no_mangle]
    static mut show_choices: bool_;
    #[no_mangle]
    static mut show_store_graph: bool_;
    #[no_mangle]
    static mut show_equip_graph: bool_;
    #[no_mangle]
    static mut show_inven_graph: bool_;
    #[no_mangle]
    static mut show_weights: bool_;
    #[no_mangle]
    static mut show_labels: bool_;
    #[no_mangle]
    static mut stack_force_costs: bool_;
    #[no_mangle]
    static mut stack_force_notes: bool_;
    #[no_mangle]
    static mut depth_in_feet: bool_;
    #[no_mangle]
    static mut always_repeat: bool_;
    #[no_mangle]
    static mut prompt_pickup_heavy: bool_;
    #[no_mangle]
    static mut always_pickup: bool_;
    #[no_mangle]
    static mut use_old_target: bool_;
    #[no_mangle]
    static mut carry_query_flag: bool_;
    #[no_mangle]
    static mut other_query_flag: bool_;
    #[no_mangle]
    static mut quick_messages: bool_;
    #[no_mangle]
    static mut rogue_like_commands: bool_;
    #[no_mangle]
    fn quest_evil_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    static mut plots: [s16b; 7];
    #[no_mangle]
    fn quest_haunted_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_dragons_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_wolves_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_ultra_evil_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_ultra_good_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_thrain_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_shroom_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_one_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_between_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_invasion_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_nirnaeth_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_eol_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_narsil_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_poison_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_spider_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_wight_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_troll_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_nazgul_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_hobbit_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_random_describe(fff: *mut FILE) -> bool_;
    #[no_mangle]
    fn quest_random_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_thieves_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_morgoth_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_sauron_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_necro_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn quest_null_hook(q: libc::c_int) -> bool_;
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
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
pub type s16b = libc::c_short;
pub type s32b = libc::c_int;
pub type u32b = libc::c_uint;
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
pub struct player_sex {
    pub title: cptr,
    pub winner: cptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct martial_arts {
    pub desc: cptr,
    pub min_level: libc::c_int,
    pub chance: libc::c_int,
    pub dd: libc::c_int,
    pub ds: libc::c_int,
    pub effect: s16b,
    pub power: s16b,
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
pub struct deity_type {
    pub name: cptr,
    pub desc: [[libc::c_char; 80]; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tactic_info_type {
    pub to_hit: s16b,
    pub to_dam: s16b,
    pub to_ac: s16b,
    pub to_stealth: s16b,
    pub to_disarm: s16b,
    pub to_saving: s16b,
    pub name: cptr,
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
pub struct move_info_type {
    pub to_speed: s16b,
    pub to_search: s16b,
    pub to_stealth: s16b,
    pub to_percep: s16b,
    pub name: cptr,
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
pub struct flags_group {
    pub name: [libc::c_char; 30],
    pub color: byte_hack,
    pub price: byte_hack,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags4: u32b,
    pub esp: u32b,
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
pub struct monster_power {
    pub power: u32b,
    pub name: cptr,
    pub mana: libc::c_int,
    pub great: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tval_desc {
    pub tval: libc::c_int,
    pub desc: cptr,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gf_name_type {
    pub gf: libc::c_int,
    pub name: cptr,
}
/* File: tables.c */
/* Purpose: Angband Tables */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Global array for looping through the "keypad directions"
 */
#[no_mangle]
pub static mut ddd: [s16b; 9] =
    [2 as libc::c_int as s16b, 8 as libc::c_int as s16b,
     6 as libc::c_int as s16b, 4 as libc::c_int as s16b,
     3 as libc::c_int as s16b, 1 as libc::c_int as s16b,
     9 as libc::c_int as s16b, 7 as libc::c_int as s16b,
     5 as libc::c_int as s16b];
/*
 * Global arrays for converting "keypad direction" into offsets
 */
#[no_mangle]
pub static mut ddx: [s16b; 10] =
    [0 as libc::c_int as s16b, -(1 as libc::c_int) as s16b,
     0 as libc::c_int as s16b, 1 as libc::c_int as s16b,
     -(1 as libc::c_int) as s16b, 0 as libc::c_int as s16b,
     1 as libc::c_int as s16b, -(1 as libc::c_int) as s16b,
     0 as libc::c_int as s16b, 1 as libc::c_int as s16b];
#[no_mangle]
pub static mut ddy: [s16b; 10] =
    [0 as libc::c_int as s16b, 1 as libc::c_int as s16b,
     1 as libc::c_int as s16b, 1 as libc::c_int as s16b,
     0 as libc::c_int as s16b, 0 as libc::c_int as s16b,
     0 as libc::c_int as s16b, -(1 as libc::c_int) as s16b,
     -(1 as libc::c_int) as s16b, -(1 as libc::c_int) as s16b];
/*
 * Global arrays for optimizing "ddx[ddd[i]]" and "ddy[ddd[i]]"
 */
#[no_mangle]
pub static mut ddx_ddd: [s16b; 9] =
    [0 as libc::c_int as s16b, 0 as libc::c_int as s16b,
     1 as libc::c_int as s16b, -(1 as libc::c_int) as s16b,
     1 as libc::c_int as s16b, -(1 as libc::c_int) as s16b,
     1 as libc::c_int as s16b, -(1 as libc::c_int) as s16b,
     0 as libc::c_int as s16b];
#[no_mangle]
pub static mut ddy_ddd: [s16b; 9] =
    [1 as libc::c_int as s16b, -(1 as libc::c_int) as s16b,
     0 as libc::c_int as s16b, 0 as libc::c_int as s16b,
     1 as libc::c_int as s16b, 1 as libc::c_int as s16b,
     -(1 as libc::c_int) as s16b, -(1 as libc::c_int) as s16b,
     0 as libc::c_int as s16b];
/*
* Global array for converting numbers to uppercase hexadecimal digit
 * This array can also be used to convert a number to an octal digit
 */
#[no_mangle]
pub static mut hexsym: [libc::c_char; 16] =
    ['0' as i32 as libc::c_char, '1' as i32 as libc::c_char,
     '2' as i32 as libc::c_char, '3' as i32 as libc::c_char,
     '4' as i32 as libc::c_char, '5' as i32 as libc::c_char,
     '6' as i32 as libc::c_char, '7' as i32 as libc::c_char,
     '8' as i32 as libc::c_char, '9' as i32 as libc::c_char,
     'A' as i32 as libc::c_char, 'B' as i32 as libc::c_char,
     'C' as i32 as libc::c_char, 'D' as i32 as libc::c_char,
     'E' as i32 as libc::c_char, 'F' as i32 as libc::c_char];
/*
 * Stat Table (INT/WIS) -- Number of half-spells per level
 */
#[no_mangle]
pub static mut adj_mag_study: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack];
/*
 * Stat Table (INT/WIS) -- extra half-mana-points per level
 */
#[no_mangle]
pub static mut adj_mag_mana: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     13 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 16 as libc::c_int as byte_hack,
     16 as libc::c_int as byte_hack, 17 as libc::c_int as byte_hack,
     17 as libc::c_int as byte_hack, 18 as libc::c_int as byte_hack];
/*
 * Stat Table (INT/WIS) -- Minimum failure rate (percentage)
 */
#[no_mangle]
pub static mut adj_mag_fail: [byte_hack; 38] =
    [99 as libc::c_int as byte_hack, 99 as libc::c_int as byte_hack,
     99 as libc::c_int as byte_hack, 99 as libc::c_int as byte_hack,
     99 as libc::c_int as byte_hack, 50 as libc::c_int as byte_hack,
     30 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack];
/*
 * Stat Table (INT/WIS) -- Various things
 */
#[no_mangle]
pub static mut adj_mag_stat: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     13 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 16 as libc::c_int as byte_hack,
     17 as libc::c_int as byte_hack, 18 as libc::c_int as byte_hack,
     19 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack];
/*
 * Stat Table (CHR) -- payment percentages
 */
#[no_mangle]
pub static mut adj_chr_gold: [byte_hack; 38] =
    [130 as libc::c_int as byte_hack, 125 as libc::c_int as byte_hack,
     122 as libc::c_int as byte_hack, 120 as libc::c_int as byte_hack,
     118 as libc::c_int as byte_hack, 116 as libc::c_int as byte_hack,
     114 as libc::c_int as byte_hack, 112 as libc::c_int as byte_hack,
     110 as libc::c_int as byte_hack, 108 as libc::c_int as byte_hack,
     106 as libc::c_int as byte_hack, 104 as libc::c_int as byte_hack,
     103 as libc::c_int as byte_hack, 102 as libc::c_int as byte_hack,
     101 as libc::c_int as byte_hack, 100 as libc::c_int as byte_hack,
     99 as libc::c_int as byte_hack, 98 as libc::c_int as byte_hack,
     97 as libc::c_int as byte_hack, 96 as libc::c_int as byte_hack,
     95 as libc::c_int as byte_hack, 94 as libc::c_int as byte_hack,
     93 as libc::c_int as byte_hack, 92 as libc::c_int as byte_hack,
     91 as libc::c_int as byte_hack, 90 as libc::c_int as byte_hack,
     89 as libc::c_int as byte_hack, 88 as libc::c_int as byte_hack,
     87 as libc::c_int as byte_hack, 86 as libc::c_int as byte_hack,
     85 as libc::c_int as byte_hack, 84 as libc::c_int as byte_hack,
     83 as libc::c_int as byte_hack, 82 as libc::c_int as byte_hack,
     81 as libc::c_int as byte_hack, 80 as libc::c_int as byte_hack,
     79 as libc::c_int as byte_hack, 78 as libc::c_int as byte_hack];
/*
 * Stat Table (INT) -- Magic devices
 */
#[no_mangle]
pub static mut adj_int_dev: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     13 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 16 as libc::c_int as byte_hack,
     17 as libc::c_int as byte_hack, 18 as libc::c_int as byte_hack,
     19 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack];
/*
 * Stat Table (WIS) -- Saving throw
 */
#[no_mangle]
pub static mut adj_wis_sav: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     12 as libc::c_int as byte_hack, 13 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     16 as libc::c_int as byte_hack, 17 as libc::c_int as byte_hack,
     18 as libc::c_int as byte_hack, 19 as libc::c_int as byte_hack];
/*
 * Stat Table (DEX) -- disarming
 */
#[no_mangle]
pub static mut adj_dex_dis: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack];
/*
 * Stat Table (INT) -- disarming
 */
#[no_mangle]
pub static mut adj_int_dis: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     12 as libc::c_int as byte_hack, 13 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     16 as libc::c_int as byte_hack, 17 as libc::c_int as byte_hack,
     18 as libc::c_int as byte_hack, 19 as libc::c_int as byte_hack,
     19 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack];
/*
 * Stat Table (DEX) -- bonus to ac (plus 128)
 */
#[no_mangle]
pub static mut adj_dex_ta: [byte_hack; 38] =
    [(128 as libc::c_int + -(4 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(3 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(2 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(1 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 5 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 6 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 7 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 8 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 9 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 9 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 10 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 11 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 12 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 13 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 14 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 15 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 15 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 16 as libc::c_int) as byte_hack];
/*
 * Stat Table (STR) -- bonus to dam (plus 128)
 */
#[no_mangle]
pub static mut adj_str_td: [byte_hack; 38] =
    [(128 as libc::c_int + -(2 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(2 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(1 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(1 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 5 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 5 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 6 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 7 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 8 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 9 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 10 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 11 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 12 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 13 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 14 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 15 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 16 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 18 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 20 as libc::c_int) as byte_hack];
/*
 * Stat Table (DEX) -- bonus to hit (plus 128)
 */
#[no_mangle]
pub static mut adj_dex_th: [byte_hack; 38] =
    [(128 as libc::c_int + -(3 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(2 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(2 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(1 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(1 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 5 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 6 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 7 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 8 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 9 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 9 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 10 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 11 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 12 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 13 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 14 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 15 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 15 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 16 as libc::c_int) as byte_hack];
/*
 * Stat Table (STR) -- bonus to hit (plus 128)
 */
#[no_mangle]
pub static mut adj_str_th: [byte_hack; 38] =
    [(128 as libc::c_int + -(3 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(2 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(1 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(1 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 5 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 6 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 7 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 8 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 9 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 10 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 11 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 12 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 13 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 14 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 15 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 15 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 16 as libc::c_int) as byte_hack];
/*
 * Stat Table (STR) -- weight limit in deca-pounds
 */
#[no_mangle]
pub static mut adj_str_wgt: [byte_hack; 38] =
    [5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     13 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 16 as libc::c_int as byte_hack,
     17 as libc::c_int as byte_hack, 18 as libc::c_int as byte_hack,
     19 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack,
     22 as libc::c_int as byte_hack, 24 as libc::c_int as byte_hack,
     26 as libc::c_int as byte_hack, 28 as libc::c_int as byte_hack,
     30 as libc::c_int as byte_hack, 31 as libc::c_int as byte_hack,
     31 as libc::c_int as byte_hack, 32 as libc::c_int as byte_hack,
     32 as libc::c_int as byte_hack, 33 as libc::c_int as byte_hack,
     33 as libc::c_int as byte_hack, 34 as libc::c_int as byte_hack,
     34 as libc::c_int as byte_hack, 35 as libc::c_int as byte_hack,
     35 as libc::c_int as byte_hack, 36 as libc::c_int as byte_hack,
     36 as libc::c_int as byte_hack, 37 as libc::c_int as byte_hack,
     37 as libc::c_int as byte_hack, 38 as libc::c_int as byte_hack,
     38 as libc::c_int as byte_hack, 39 as libc::c_int as byte_hack];
/*
 * Stat Table (STR) -- weapon weight limit in pounds
 */
#[no_mangle]
pub static mut adj_str_hold: [byte_hack; 38] =
    [4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     12 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     16 as libc::c_int as byte_hack, 18 as libc::c_int as byte_hack,
     20 as libc::c_int as byte_hack, 22 as libc::c_int as byte_hack,
     24 as libc::c_int as byte_hack, 26 as libc::c_int as byte_hack,
     28 as libc::c_int as byte_hack, 30 as libc::c_int as byte_hack,
     30 as libc::c_int as byte_hack, 35 as libc::c_int as byte_hack,
     40 as libc::c_int as byte_hack, 45 as libc::c_int as byte_hack,
     50 as libc::c_int as byte_hack, 55 as libc::c_int as byte_hack,
     60 as libc::c_int as byte_hack, 65 as libc::c_int as byte_hack,
     70 as libc::c_int as byte_hack, 80 as libc::c_int as byte_hack,
     80 as libc::c_int as byte_hack, 80 as libc::c_int as byte_hack,
     80 as libc::c_int as byte_hack, 80 as libc::c_int as byte_hack,
     90 as libc::c_int as byte_hack, 90 as libc::c_int as byte_hack,
     90 as libc::c_int as byte_hack, 90 as libc::c_int as byte_hack,
     90 as libc::c_int as byte_hack, 100 as libc::c_int as byte_hack,
     100 as libc::c_int as byte_hack, 100 as libc::c_int as byte_hack];
/*
 * Stat Table (STR) -- digging value
 */
#[no_mangle]
pub static mut adj_str_dig: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack,
     25 as libc::c_int as byte_hack, 30 as libc::c_int as byte_hack,
     35 as libc::c_int as byte_hack, 40 as libc::c_int as byte_hack,
     45 as libc::c_int as byte_hack, 50 as libc::c_int as byte_hack,
     55 as libc::c_int as byte_hack, 60 as libc::c_int as byte_hack,
     65 as libc::c_int as byte_hack, 70 as libc::c_int as byte_hack,
     75 as libc::c_int as byte_hack, 80 as libc::c_int as byte_hack,
     85 as libc::c_int as byte_hack, 90 as libc::c_int as byte_hack,
     95 as libc::c_int as byte_hack, 100 as libc::c_int as byte_hack,
     100 as libc::c_int as byte_hack, 100 as libc::c_int as byte_hack];
/*
 * Stat Table (STR) -- help index into the "blow" table
 */
#[no_mangle]
pub static mut adj_str_blow: [byte_hack; 38] =
    [3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     13 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 16 as libc::c_int as byte_hack,
     17 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack,
     30 as libc::c_int as byte_hack, 40 as libc::c_int as byte_hack,
     50 as libc::c_int as byte_hack, 60 as libc::c_int as byte_hack,
     70 as libc::c_int as byte_hack, 80 as libc::c_int as byte_hack,
     90 as libc::c_int as byte_hack, 100 as libc::c_int as byte_hack,
     110 as libc::c_int as byte_hack, 120 as libc::c_int as byte_hack,
     130 as libc::c_int as byte_hack, 140 as libc::c_int as byte_hack,
     150 as libc::c_int as byte_hack, 160 as libc::c_int as byte_hack,
     170 as libc::c_int as byte_hack, 180 as libc::c_int as byte_hack,
     190 as libc::c_int as byte_hack, 200 as libc::c_int as byte_hack,
     210 as libc::c_int as byte_hack, 220 as libc::c_int as byte_hack,
     230 as libc::c_int as byte_hack, 240 as libc::c_int as byte_hack];
/*
 * Stat Table (DEX) -- index into the "blow" table
 */
#[no_mangle]
pub static mut adj_dex_blow: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 16 as libc::c_int as byte_hack,
     18 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack,
     20 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack];
/*
 * Stat Table (DEX) -- chance of avoiding "theft" and "falling"
 */
#[no_mangle]
pub static mut adj_dex_safe: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack,
     25 as libc::c_int as byte_hack, 30 as libc::c_int as byte_hack,
     35 as libc::c_int as byte_hack, 40 as libc::c_int as byte_hack,
     45 as libc::c_int as byte_hack, 50 as libc::c_int as byte_hack,
     60 as libc::c_int as byte_hack, 70 as libc::c_int as byte_hack,
     80 as libc::c_int as byte_hack, 90 as libc::c_int as byte_hack,
     100 as libc::c_int as byte_hack, 100 as libc::c_int as byte_hack,
     100 as libc::c_int as byte_hack, 100 as libc::c_int as byte_hack,
     100 as libc::c_int as byte_hack, 100 as libc::c_int as byte_hack,
     100 as libc::c_int as byte_hack, 100 as libc::c_int as byte_hack];
/*
 * Stat Table (CON) -- base regeneration rate
 */
#[no_mangle]
pub static mut adj_con_fix: [byte_hack; 38] =
    [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     0 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack];
/*
 * Stat Table (CON) -- extra half-hitpoints per level (plus 128)
 */
#[no_mangle]
pub static mut adj_con_mhp: [byte_hack; 38] =
    [(128 as libc::c_int + -(5 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(3 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(2 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + -(1 as libc::c_int)) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 0 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 1 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 2 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 3 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 4 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 5 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 6 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 7 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 8 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 9 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 10 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 11 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 12 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 13 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 14 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 15 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 16 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 18 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 20 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 22 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 25 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 26 as libc::c_int) as byte_hack,
     (128 as libc::c_int + 27 as libc::c_int) as byte_hack];
/*
 * This table is used to help calculate the number of blows the player can
 * make in a single round of attacks (one player turn) with a normal weapon.
 *
 * This number ranges from a single blow/round for weak players to up to six
 * blows/round for powerful warriors.
 *
 * Note that certain artifacts and ego-items give "bonus" blows/round.
 *
 * First, from the player class, we extract some values:
 *
 *    Warrior --> num = 6; mul = 5; div = MAX(30, weapon_weight);
 *    Mage    --> num = 4; mul = 2; div = MAX(40, weapon_weight);
 *    Priest  --> num = 5; mul = 3; div = MAX(35, weapon_weight);
 *    Rogue   --> num = 5; mul = 3; div = MAX(30, weapon_weight);
 *    Ranger  --> num = 5; mul = 4; div = MAX(35, weapon_weight);
 *    Paladin --> num = 5; mul = 4; div = MAX(30, weapon_weight);
 *
 * To get "P", we look up the relevant "adj_str_blow[]" (see above),
 * multiply it by "mul", and then divide it by "div", rounding down.
 *
 * To get "D", we look up the relevant "adj_dex_blow[]" (see above),
 * note especially column 6 (DEX 18/101) and 11 (DEX 18/150).
 *
 * The player gets "blows_table[P][D]" blows/round, as shown below,
 * up to a maximum of "num" blows/round, plus any "bonus" blows/round.
 */
#[no_mangle]
pub static mut blows_table: [[byte_hack; 12]; 12] =
    [[1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
      1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
      1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
      2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
      2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
      2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack],
     [1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
      1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
      2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
      3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack],
     [1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
      2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
      3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack],
     [1 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
      2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack],
     [1 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
      2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack],
     [2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
      3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack],
     [2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
      3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack],
     [2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack],
     [3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      6 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack],
     [3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      6 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack],
     [3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
      6 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack],
     [3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
      5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
      6 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
      6 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack]];
#[no_mangle]
pub static mut arena_monsters: [s16b; 29] =
    [30 as libc::c_int as s16b, 43 as libc::c_int as s16b,
     102 as libc::c_int as s16b, 118 as libc::c_int as s16b,
     126 as libc::c_int as s16b, 149 as libc::c_int as s16b,
     173 as libc::c_int as s16b, 183 as libc::c_int as s16b,
     188 as libc::c_int as s16b, 191 as libc::c_int as s16b,
     216 as libc::c_int as s16b, 230 as libc::c_int as s16b,
     238 as libc::c_int as s16b, 244 as libc::c_int as s16b,
     255 as libc::c_int as s16b, 262 as libc::c_int as s16b,
     293 as libc::c_int as s16b, 297 as libc::c_int as s16b,
     321 as libc::c_int as s16b, 349 as libc::c_int as s16b,
     372 as libc::c_int as s16b, 401 as libc::c_int as s16b,
     415 as libc::c_int as s16b, 454 as libc::c_int as s16b,
     464 as libc::c_int as s16b, 485 as libc::c_int as s16b,
     538 as libc::c_int as s16b, 631 as libc::c_int as s16b,
     641 as libc::c_int as s16b];
/*
 * This table allows quick conversion from "speed" to "energy"
 * The basic function WAS ((S>=110) ? (S-110) : (100 / (120-S)))
 * Note that table access is *much* quicker than computation.
 *
 * Note that the table has been changed at high speeds.  From
 * "Slow (-40)" to "Fast (+30)" is pretty much unchanged, but
 * at speeds above "Fast (+30)", one approaches an asymptotic
 * effective limit of 50 energy per turn.  This means that it
 * is relatively easy to reach "Fast (+30)" and get about 40
 * energy per turn, but then speed becomes very "expensive",
 * and you must get all the way to "Fast (+50)" to reach the
 * point of getting 45 energy per turn.  After that point,
 * further increases in speed are more or less pointless,
 * except to balance out heavy inventory.
 *
 * Note that currently the fastest monster is "Fast (+30)".
 *
 * It should be possible to lower the energy threshold from
 * 100 units to 50 units, though this may interact badly with
 * the (compiled out) small random energy boost code.  It may
 * also tend to cause more "clumping" at high speeds.
 */
#[no_mangle]
pub static mut extract_energy: [byte_hack; 300] =
    [1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     12 as libc::c_int as byte_hack, 13 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     16 as libc::c_int as byte_hack, 17 as libc::c_int as byte_hack,
     18 as libc::c_int as byte_hack, 19 as libc::c_int as byte_hack,
     20 as libc::c_int as byte_hack, 21 as libc::c_int as byte_hack,
     22 as libc::c_int as byte_hack, 23 as libc::c_int as byte_hack,
     24 as libc::c_int as byte_hack, 25 as libc::c_int as byte_hack,
     26 as libc::c_int as byte_hack, 27 as libc::c_int as byte_hack,
     28 as libc::c_int as byte_hack, 29 as libc::c_int as byte_hack,
     30 as libc::c_int as byte_hack, 31 as libc::c_int as byte_hack,
     32 as libc::c_int as byte_hack, 33 as libc::c_int as byte_hack,
     34 as libc::c_int as byte_hack, 35 as libc::c_int as byte_hack,
     36 as libc::c_int as byte_hack, 36 as libc::c_int as byte_hack,
     37 as libc::c_int as byte_hack, 37 as libc::c_int as byte_hack,
     38 as libc::c_int as byte_hack, 38 as libc::c_int as byte_hack,
     39 as libc::c_int as byte_hack, 39 as libc::c_int as byte_hack,
     40 as libc::c_int as byte_hack, 40 as libc::c_int as byte_hack,
     40 as libc::c_int as byte_hack, 41 as libc::c_int as byte_hack,
     41 as libc::c_int as byte_hack, 41 as libc::c_int as byte_hack,
     42 as libc::c_int as byte_hack, 42 as libc::c_int as byte_hack,
     42 as libc::c_int as byte_hack, 43 as libc::c_int as byte_hack,
     43 as libc::c_int as byte_hack, 43 as libc::c_int as byte_hack,
     44 as libc::c_int as byte_hack, 44 as libc::c_int as byte_hack,
     44 as libc::c_int as byte_hack, 44 as libc::c_int as byte_hack,
     45 as libc::c_int as byte_hack, 45 as libc::c_int as byte_hack,
     45 as libc::c_int as byte_hack, 45 as libc::c_int as byte_hack,
     45 as libc::c_int as byte_hack, 46 as libc::c_int as byte_hack,
     46 as libc::c_int as byte_hack, 46 as libc::c_int as byte_hack,
     46 as libc::c_int as byte_hack, 46 as libc::c_int as byte_hack,
     47 as libc::c_int as byte_hack, 47 as libc::c_int as byte_hack,
     47 as libc::c_int as byte_hack, 47 as libc::c_int as byte_hack,
     47 as libc::c_int as byte_hack, 48 as libc::c_int as byte_hack,
     48 as libc::c_int as byte_hack, 48 as libc::c_int as byte_hack,
     48 as libc::c_int as byte_hack, 48 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack,
     49 as libc::c_int as byte_hack, 49 as libc::c_int as byte_hack];
/*
 * Base experience levels, may be adjusted up for race and/or class
 */
#[no_mangle]
pub static mut player_exp: [s32b; 50] =
    [10 as libc::c_int, 25 as libc::c_int, 45 as libc::c_int,
     70 as libc::c_int, 100 as libc::c_int, 140 as libc::c_int,
     200 as libc::c_int, 280 as libc::c_int, 380 as libc::c_int,
     500 as libc::c_int, 650 as libc::c_int, 850 as libc::c_int,
     1100 as libc::c_int, 1400 as libc::c_int, 1800 as libc::c_int,
     2300 as libc::c_int, 2900 as libc::c_int, 3600 as libc::c_int,
     4400 as libc::c_int, 5400 as libc::c_int, 6800 as libc::c_int,
     8400 as libc::c_int, 10200 as libc::c_int, 12500 as libc::c_int,
     17500 as libc::c_int, 25000 as libc::c_int,
     35000 as libc::c_long as s32b, 50000 as libc::c_long as s32b,
     75000 as libc::c_long as s32b, 100000 as libc::c_long as s32b,
     150000 as libc::c_long as s32b, 200000 as libc::c_long as s32b,
     275000 as libc::c_long as s32b, 350000 as libc::c_long as s32b,
     450000 as libc::c_long as s32b, 550000 as libc::c_long as s32b,
     700000 as libc::c_long as s32b, 850000 as libc::c_long as s32b,
     1000000 as libc::c_long as s32b, 1250000 as libc::c_long as s32b,
     1500000 as libc::c_long as s32b, 1800000 as libc::c_long as s32b,
     2100000 as libc::c_long as s32b, 2400000 as libc::c_long as s32b,
     2700000 as libc::c_long as s32b, 3000000 as libc::c_long as s32b,
     3500000 as libc::c_long as s32b, 4000000 as libc::c_long as s32b,
     4500000 as libc::c_long as s32b, 5000000 as libc::c_long as s32b];
/*
 * Player Sexes
 *
 *      Title,
 *      Winner
 */
#[no_mangle]
pub static mut sex_info: [player_sex; 3] =
    [{
         let mut init =
             player_sex{title:
                            b"Female\x00" as *const u8 as *const libc::c_char,
                        winner:
                            b"Queen\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             player_sex{title:
                            b"Male\x00" as *const u8 as *const libc::c_char,
                        winner:
                            b"King\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             player_sex{title:
                            b"Neuter\x00" as *const u8 as *const libc::c_char,
                        winner:
                            b"Ruler\x00" as *const u8 as
                                *const libc::c_char,};
         init
     }];
/*
 * Hack -- the "basic" color names (see "TERM_xxx")
 */
#[no_mangle]
pub static mut color_names: [cptr; 16] =
    [b"Dark\x00" as *const u8 as *const libc::c_char,
     b"White\x00" as *const u8 as *const libc::c_char,
     b"Slate\x00" as *const u8 as *const libc::c_char,
     b"Orange\x00" as *const u8 as *const libc::c_char,
     b"Red\x00" as *const u8 as *const libc::c_char,
     b"Green\x00" as *const u8 as *const libc::c_char,
     b"Blue\x00" as *const u8 as *const libc::c_char,
     b"Umber\x00" as *const u8 as *const libc::c_char,
     b"Light Dark\x00" as *const u8 as *const libc::c_char,
     b"Light Slate\x00" as *const u8 as *const libc::c_char,
     b"Violet\x00" as *const u8 as *const libc::c_char,
     b"Yellow\x00" as *const u8 as *const libc::c_char,
     b"Light Red\x00" as *const u8 as *const libc::c_char,
     b"Light Green\x00" as *const u8 as *const libc::c_char,
     b"Light Blue\x00" as *const u8 as *const libc::c_char,
     b"Light Umber\x00" as *const u8 as *const libc::c_char];
/*
 * Abbreviations of healthy stats
 */
#[no_mangle]
pub static mut stat_names: [cptr; 6] =
    [b"STR\x00" as *const u8 as *const libc::c_char,
     b"INT\x00" as *const u8 as *const libc::c_char,
     b"WIS\x00" as *const u8 as *const libc::c_char,
     b"DEX\x00" as *const u8 as *const libc::c_char,
     b"CON\x00" as *const u8 as *const libc::c_char,
     b"CHR\x00" as *const u8 as *const libc::c_char];
/*
 * Abbreviations of damaged stats
 */
#[no_mangle]
pub static mut stat_names_reduced: [cptr; 6] =
    [b"Str\x00" as *const u8 as *const libc::c_char,
     b"Int\x00" as *const u8 as *const libc::c_char,
     b"Wis\x00" as *const u8 as *const libc::c_char,
     b"Dex\x00" as *const u8 as *const libc::c_char,
     b"Con\x00" as *const u8 as *const libc::c_char,
     b"Chr\x00" as *const u8 as *const libc::c_char];
/*
 * Certain "screens" always use the main screen, including News, Birth,
 * Dungeon, Tomb-stone, High-scores, Macros, Colors, Visuals, Options.
 *
 * Later, special flags may allow sub-windows to "steal" stuff from the
 * main window, including File dump (help), File dump (artifacts, uniques),
 * Character screen, Small scale map, Previous Messages, Store screen, etc.
 *
 * The "ctrl-i" (tab) command flips the "Display inven/equip" and "Display
 * equip/inven" flags for all windows.
 *
 * The "ctrl-g" command (or pseudo-command) should perhaps grab a snapshot
 * of the main screen into any interested windows.
 */
#[no_mangle]
pub static mut window_flag_desc: [cptr; 32] =
    [b"Display inven/equip\x00" as *const u8 as *const libc::c_char,
     b"Display equip/inven\x00" as *const u8 as *const libc::c_char,
     0 as cptr, b"Display character\x00" as *const u8 as *const libc::c_char,
     b"Show visible monsters\x00" as *const u8 as *const libc::c_char,
     0 as cptr, b"Display messages\x00" as *const u8 as *const libc::c_char,
     b"Display overhead view\x00" as *const u8 as *const libc::c_char,
     b"Display monster recall\x00" as *const u8 as *const libc::c_char,
     b"Display object recall\x00" as *const u8 as *const libc::c_char,
     0 as cptr, b"Display snap-shot\x00" as *const u8 as *const libc::c_char,
     0 as cptr, 0 as cptr,
     b"Display borg messages\x00" as *const u8 as *const libc::c_char,
     b"Display borg status\x00" as *const u8 as *const libc::c_char,
     0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr,
     0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr,
     0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr];
/*
 * Available Options
 *
 * Option Screen Sets:
 *
 *      Set 1: User Interface
 *      Set 2: Disturbance
 *      Set 3: Inventory
 *      Set 4: Game Play
 *      Set 5: ToME
 *      Set 6: Birth
 *
 * Note that bits 28-31 of set 0 are currently unused.
 */
#[no_mangle]
pub static mut option_info: [option_type; 98] =
    unsafe {
        [{
             let mut init =
                 option_type{o_var:
                                 &rogue_like_commands as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 0 as libc::c_int as byte_hack,
                             o_text:
                                 b"rogue_like_commands\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Rogue-like commands\x00" as *const u8 as
                                     *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &quick_messages as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 1 as libc::c_int as byte_hack,
                             o_text:
                                 b"quick_messages\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Activate quick messages\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &other_query_flag as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 2 as libc::c_int as byte_hack,
                             o_text:
                                 b"other_query_flag\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Prompt for various information\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &carry_query_flag as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 3 as libc::c_int as byte_hack,
                             o_text:
                                 b"carry_query_flag\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Prompt before picking things up\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &use_old_target as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 4 as libc::c_int as byte_hack,
                             o_text:
                                 b"use_old_target\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Use old target by default\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &always_pickup as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 5 as libc::c_int as byte_hack,
                             o_text:
                                 b"always_pickup\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Pick things up by default\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &prompt_pickup_heavy as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 6 as libc::c_int as byte_hack,
                             o_text:
                                 b"prompt_pickup_heavy\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Prompt before picking up heavy objects\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &always_repeat as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 7 as libc::c_int as byte_hack,
                             o_text:
                                 b"always_repeat\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Repeat obvious commands\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &depth_in_feet as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 8 as libc::c_int as byte_hack,
                             o_text:
                                 b"depth_in_feet\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Show dungeon level in feet\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &stack_force_notes as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 9 as libc::c_int as byte_hack,
                             o_text:
                                 b"stack_force_notes\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Merge inscriptions when stacking\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &stack_force_costs as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 10 as libc::c_int as byte_hack,
                             o_text:
                                 b"stack_force_costs\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Merge discounts when stacking\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &show_labels as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 11 as libc::c_int as byte_hack,
                             o_text:
                                 b"show_labels\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Show labels in object listings\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &show_weights as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 12 as libc::c_int as byte_hack,
                             o_text:
                                 b"show_weights\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Show weights in object listings\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &show_inven_graph as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 13 as libc::c_int as byte_hack,
                             o_text:
                                 b"show_inven_graph\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Show graphics in inventory list\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &show_equip_graph as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 14 as libc::c_int as byte_hack,
                             o_text:
                                 b"show_equip_graph\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Show graphics in equipment list\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &show_store_graph as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 15 as libc::c_int as byte_hack,
                             o_text:
                                 b"show_store_graph\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Show graphics in stores\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &show_choices as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 16 as libc::c_int as byte_hack,
                             o_text:
                                 b"show_choices\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Show choices in certain sub-windows\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &show_details as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 17 as libc::c_int as byte_hack,
                             o_text:
                                 b"show_details\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Show details in certain sub-windows\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &ring_bell as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 1 as libc::c_int as byte_hack,
                             o_bit: 18 as libc::c_int as byte_hack,
                             o_text:
                                 b"ring_bell\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Audible bell (on errors, etc)\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &find_ignore_stairs as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 0 as libc::c_int as byte_hack,
                             o_text:
                                 b"find_ignore_stairs\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Run past stairs\x00" as *const u8 as
                                     *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &find_ignore_doors as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 1 as libc::c_int as byte_hack,
                             o_text:
                                 b"find_ignore_doors\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Run through open doors\x00" as *const u8 as
                                     *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &find_cut as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 2 as libc::c_int as byte_hack,
                             o_text:
                                 b"find_cut\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Run past known corners\x00" as *const u8 as
                                     *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &find_examine as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 3 as libc::c_int as byte_hack,
                             o_text:
                                 b"find_examine\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Run into potential corners\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &disturb_move as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 4 as libc::c_int as byte_hack,
                             o_text:
                                 b"disturb_move\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Disturb whenever any monster moves\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &disturb_near as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 5 as libc::c_int as byte_hack,
                             o_text:
                                 b"disturb_near\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Disturb whenever viewable monster moves\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &disturb_panel as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 6 as libc::c_int as byte_hack,
                             o_text:
                                 b"disturb_panel\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Disturb whenever map panel changes\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &disturb_detect as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 21 as libc::c_int as byte_hack,
                             o_text:
                                 b"disturb_detect\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Disturb whenever leaving trap-detected area\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &disturb_state as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 7 as libc::c_int as byte_hack,
                             o_text:
                                 b"disturb_state\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Disturb whenever player state changes\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &disturb_minor as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 8 as libc::c_int as byte_hack,
                             o_text:
                                 b"disturb_minor\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Disturb whenever boring things happen\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &disturb_other as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 9 as libc::c_int as byte_hack,
                             o_text:
                                 b"disturb_other\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Disturb whenever random things happen\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &alert_hitpoint as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 10 as libc::c_int as byte_hack,
                             o_text:
                                 b"alert_hitpoint\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Alert user to critical hitpoints\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &alert_failure as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 11 as libc::c_int as byte_hack,
                             o_text:
                                 b"alert_failure\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Alert user to various failures\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &last_words as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 12 as libc::c_int as byte_hack,
                             o_text:
                                 b"last_words\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Get last words when the character dies\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &speak_unique as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 13 as libc::c_int as byte_hack,
                             o_text:
                                 b"speak_unique\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow shopkeepers and uniques to speak\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &auto_destroy as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 14 as libc::c_int as byte_hack,
                             o_text:
                                 b"auto_destroy\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"No query to destroy known worthless items\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &wear_confirm as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 15 as libc::c_int as byte_hack,
                             o_text:
                                 b"confirm_wear\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Confirm to wear/wield known cursed items\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &confirm_stairs as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 16 as libc::c_int as byte_hack,
                             o_text:
                                 b"confirm_stairs\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Prompt before exiting a dungeon level\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &disturb_pets as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 17 as libc::c_int as byte_hack,
                             o_text:
                                 b"disturb_pets\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Disturb when visible pets move\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &easy_open as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 18 as libc::c_int as byte_hack,
                             o_text:
                                 b"easy_open\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Automatically open doors\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &easy_disarm as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 19 as libc::c_int as byte_hack,
                             o_text:
                                 b"easy_disarm\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Automatically disarm traps\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &easy_tunnel as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 2 as libc::c_int as byte_hack,
                             o_bit: 20 as libc::c_int as byte_hack,
                             o_text:
                                 b"easy_tunnel\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Automatically tunnel walls\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &auto_haggle as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 0 as libc::c_int as byte_hack,
                             o_text:
                                 b"auto_haggle\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Auto-haggle in stores\x00" as *const u8 as
                                     *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &auto_scum as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 1 as libc::c_int as byte_hack,
                             o_text:
                                 b"auto_scum\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Auto-scum for good levels\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &stack_allow_items as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 2 as libc::c_int as byte_hack,
                             o_text:
                                 b"stack_allow_items\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow weapons and armour to stack\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &stack_allow_wands as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 3 as libc::c_int as byte_hack,
                             o_text:
                                 b"stack_allow_wands\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow wands/staffs/rods to stack\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &expand_look as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 4 as libc::c_int as byte_hack,
                             o_text:
                                 b"expand_look\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Expand the power of the look command\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &expand_list as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 5 as libc::c_int as byte_hack,
                             o_text:
                                 b"expand_list\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Expand the power of the list commands\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &view_perma_grids as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 6 as libc::c_int as byte_hack,
                             o_text:
                                 b"view_perma_grids\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Map remembers all perma-lit grids\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &view_torch_grids as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 7 as libc::c_int as byte_hack,
                             o_text:
                                 b"view_torch_grids\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Map remembers all torch-lit grids\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &monster_lite as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 19 as libc::c_int as byte_hack,
                             o_text:
                                 b"monster_lite\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow some monsters to carry light\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &dungeon_align as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 8 as libc::c_int as byte_hack,
                             o_text:
                                 b"dungeon_align\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Generate dungeons with aligned rooms\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &dungeon_stair as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 9 as libc::c_int as byte_hack,
                             o_text:
                                 b"dungeon_stair\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Generate dungeons with connected stairs\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &flow_by_sound as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 10 as libc::c_int as byte_hack,
                             o_text:
                                 b"flow_by_sound\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Monsters chase current location (v.slow)\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &player_symbols as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 12 as libc::c_int as byte_hack,
                             o_text:
                                 b"player_symbols\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Use special symbols for the player char\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &plain_descriptions as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 13 as libc::c_int as byte_hack,
                             o_text:
                                 b"plain_descriptions\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Plain object descriptions\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &smart_learn as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 14 as libc::c_int as byte_hack,
                             o_text:
                                 b"smart_learn\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Monsters learn from their mistakes\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &smart_cheat as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 15 as libc::c_int as byte_hack,
                             o_text:
                                 b"smart_cheat\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Monsters exploit players weaknesses\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &stupid_monsters as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 16 as libc::c_int as byte_hack,
                             o_text:
                                 b"stupid_monsters\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Monsters behave stupidly\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &small_levels as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 17 as libc::c_int as byte_hack,
                             o_text:
                                 b"small_levels\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow unusually small dungeon levels\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &empty_levels as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 3 as libc::c_int as byte_hack,
                             o_bit: 18 as libc::c_int as byte_hack,
                             o_text:
                                 b"empty_levels\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow empty \'arena\' levels\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &view_reduce_lite as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 0 as libc::c_int as byte_hack,
                             o_text:
                                 b"view_reduce_lite\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Reduce lite-radius when running\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &view_reduce_view as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 1 as libc::c_int as byte_hack,
                             o_text:
                                 b"view_reduce_view\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Reduce view-radius in town\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &avoid_abort as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 2 as libc::c_int as byte_hack,
                             o_text:
                                 b"avoid_abort\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Avoid checking for user abort\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &avoid_shimmer as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 17 as libc::c_int as byte_hack,
                             o_text:
                                 b"avoid_shimmer\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Avoid extra shimmering (fast)\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &avoid_other as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 3 as libc::c_int as byte_hack,
                             o_text:
                                 b"avoid_other\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Avoid processing special colors (fast)\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &flush_failure as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 4 as libc::c_int as byte_hack,
                             o_text:
                                 b"flush_failure\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Flush input on various failures\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &flush_disturb as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 5 as libc::c_int as byte_hack,
                             o_text:
                                 b"flush_disturb\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Flush input whenever disturbed\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &flush_command as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 6 as libc::c_int as byte_hack,
                             o_text:
                                 b"flush_command\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Flush input before every command\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &fresh_before as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 7 as libc::c_int as byte_hack,
                             o_text:
                                 b"fresh_before\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Flush output before every command\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &fresh_after as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 8 as libc::c_int as byte_hack,
                             o_text:
                                 b"fresh_after\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Flush output after every command\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &fresh_message as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 9 as libc::c_int as byte_hack,
                             o_text:
                                 b"fresh_message\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Flush output after every message\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &hilite_player as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 11 as libc::c_int as byte_hack,
                             o_text:
                                 b"hilite_player\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Hilite the player with the cursor\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &view_yellow_lite as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 12 as libc::c_int as byte_hack,
                             o_text:
                                 b"view_yellow_lite\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Use special colors for torch-lit grids\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &view_bright_lite as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 13 as libc::c_int as byte_hack,
                             o_text:
                                 b"view_bright_lite\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Use special colors for \'viewable\' grids\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &view_granite_lite as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 14 as libc::c_int as byte_hack,
                             o_text:
                                 b"view_granite_lite\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Use special colors for wall grids (slow)\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &view_special_lite as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 15 as libc::c_int as byte_hack,
                             o_text:
                                 b"view_special_lite\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Use special colors for floor grids (slow)\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &center_player as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 4 as libc::c_int as byte_hack,
                             o_bit: 16 as libc::c_int as byte_hack,
                             o_text:
                                 b"center_player\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Center the view on the player (very slow)\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &option_ingame_help as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 5 as libc::c_int as byte_hack,
                             o_bit: 1 as libc::c_int as byte_hack,
                             o_text:
                                 b"ingame_help\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Ingame contextual help\x00" as *const u8 as
                                     *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &exp_need as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 5 as libc::c_int as byte_hack,
                             o_bit: 2 as libc::c_int as byte_hack,
                             o_text:
                                 b"exp_need\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Show the experience needed for next level\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &autoload_old_colors as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 5 as libc::c_int as byte_hack,
                             o_bit: 3 as libc::c_int as byte_hack,
                             o_text:
                                 b"old_colors\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Use the old(Z) coloring scheme(reload the game)\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &auto_more as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 5 as libc::c_int as byte_hack,
                             o_bit: 4 as libc::c_int as byte_hack,
                             o_text:
                                 b"auto_more\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Automatically clear \'-more-\' prompts\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &player_char_health as *const bool_ as
                                     *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 5 as libc::c_int as byte_hack,
                             o_bit: 6 as libc::c_int as byte_hack,
                             o_text:
                                 b"player_char_health\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Player char represent his/her health\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &linear_stats as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 5 as libc::c_int as byte_hack,
                             o_bit: 7 as libc::c_int as byte_hack,
                             o_text:
                                 b"linear_stats\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Stats are represented in a linear way\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &inventory_no_move as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 5 as libc::c_int as byte_hack,
                             o_bit: 8 as libc::c_int as byte_hack,
                             o_text:
                                 b"inventory_no_move\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"In option windows, just omit the select char\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &maximize as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 1 as libc::c_int as byte_hack,
                             o_text:
                                 b"maximize\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Maximise stats\x00" as *const u8 as
                                     *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &preserve as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 2 as libc::c_int as byte_hack,
                             o_text:
                                 b"preserve\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Preserve artifacts\x00" as *const u8 as
                                     *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &autoroll as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 3 as libc::c_int as byte_hack,
                             o_text:
                                 b"autoroll\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Specify \'minimal\' stats\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &point_based as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 17 as libc::c_int as byte_hack,
                             o_text:
                                 b"point_based\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Generate character using a point system\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &ironman_rooms as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 6 as libc::c_int as byte_hack,
                             o_text:
                                 b"ironman_rooms\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Always generate very unusual rooms\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &take_notes as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 7 as libc::c_int as byte_hack,
                             o_text:
                                 b"take_notes\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow notes to be written to a file\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &auto_notes as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 8 as libc::c_int as byte_hack,
                             o_text:
                                 b"auto_notes\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Automatically note important events\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &fast_autoroller as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 10 as libc::c_int as byte_hack,
                             o_text:
                                 b"fast_autoroller\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Fast autoroller(NOT on multiuser systems)\x00"
                                     as *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &joke_monsters as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 14 as libc::c_int as byte_hack,
                             o_text:
                                 b"joke_monsters\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow use of some \'joke\' monsters\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &always_small_level as *const bool_ as
                                     *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 16 as libc::c_int as byte_hack,
                             o_text:
                                 b"always_small_level\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Always make small levels\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &fate_option as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 6 as libc::c_int as byte_hack,
                             o_bit: 18 as libc::c_int as byte_hack,
                             o_text:
                                 b"fate_option\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"You can receive fates, good or bad\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &testing_stack as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 7 as libc::c_int as byte_hack,
                             o_bit: 0 as libc::c_int as byte_hack,
                             o_text:
                                 b"testing_stack\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow objects to stack on floor\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var:
                                 &testing_carry as *const bool_ as *mut bool_,
                             o_norm: 1 as libc::c_int as byte_hack,
                             o_page: 7 as libc::c_int as byte_hack,
                             o_bit: 1 as libc::c_int as byte_hack,
                             o_text:
                                 b"testing_carry\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow monsters to carry objects\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: 0 as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 0 as libc::c_int as byte_hack,
                             o_bit: 0 as libc::c_int as byte_hack,
                             o_text: 0 as cptr,
                             o_desc: 0 as cptr,};
             init
         }]
    };
#[no_mangle]
pub static mut chaos_patrons: [cptr; 16] =
    [b"Slortar\x00" as *const u8 as *const libc::c_char,
     b"Mabelode\x00" as *const u8 as *const libc::c_char,
     b"Chardros\x00" as *const u8 as *const libc::c_char,
     b"Hionhurn\x00" as *const u8 as *const libc::c_char,
     b"Xiombarg\x00" as *const u8 as *const libc::c_char,
     b"Pyaray\x00" as *const u8 as *const libc::c_char,
     b"Balaan\x00" as *const u8 as *const libc::c_char,
     b"Arioch\x00" as *const u8 as *const libc::c_char,
     b"Eequor\x00" as *const u8 as *const libc::c_char,
     b"Narjhan\x00" as *const u8 as *const libc::c_char,
     b"Balo\x00" as *const u8 as *const libc::c_char,
     b"Khorne\x00" as *const u8 as *const libc::c_char,
     b"Slaanesh\x00" as *const u8 as *const libc::c_char,
     b"Nurgle\x00" as *const u8 as *const libc::c_char,
     b"Tzeentch\x00" as *const u8 as *const libc::c_char,
     b"Khaine\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub static mut chaos_stats: [libc::c_int; 16] =
    [4 as libc::c_int, 4 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
     4 as libc::c_int, 5 as libc::c_int, -(1 as libc::c_int),
     0 as libc::c_int, 5 as libc::c_int, 4 as libc::c_int, 1 as libc::c_int,
     0 as libc::c_int];
#[no_mangle]
pub static mut chaos_rewards: [[libc::c_int; 20]; 16] =
    [[22 as libc::c_int, 19 as libc::c_int, 20 as libc::c_int,
      14 as libc::c_int, 13 as libc::c_int, 32 as libc::c_int,
      32 as libc::c_int, 32 as libc::c_int, 16 as libc::c_int,
      0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 12 as libc::c_int,
      12 as libc::c_int, 1 as libc::c_int, 3 as libc::c_int, 5 as libc::c_int,
      4 as libc::c_int, 15 as libc::c_int, 15 as libc::c_int],
     [22 as libc::c_int, 19 as libc::c_int, 20 as libc::c_int,
      10 as libc::c_int, 9 as libc::c_int, 9 as libc::c_int,
      32 as libc::c_int, 32 as libc::c_int, 16 as libc::c_int,
      16 as libc::c_int, 0 as libc::c_int, 17 as libc::c_int,
      17 as libc::c_int, 12 as libc::c_int, 33 as libc::c_int,
      5 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int, 6 as libc::c_int,
      6 as libc::c_int],
     [22 as libc::c_int, 22 as libc::c_int, 18 as libc::c_int,
      21 as libc::c_int, 10 as libc::c_int, 9 as libc::c_int,
      32 as libc::c_int, 32 as libc::c_int, 23 as libc::c_int,
      33 as libc::c_int, 24 as libc::c_int, 25 as libc::c_int,
      25 as libc::c_int, 26 as libc::c_int, 3 as libc::c_int,
      5 as libc::c_int, 6 as libc::c_int, 6 as libc::c_int, 15 as libc::c_int,
      15 as libc::c_int],
     [22 as libc::c_int, 22 as libc::c_int, 19 as libc::c_int,
      20 as libc::c_int, 14 as libc::c_int, 32 as libc::c_int,
      32 as libc::c_int, 33 as libc::c_int, 23 as libc::c_int,
      24 as libc::c_int, 25 as libc::c_int, 25 as libc::c_int,
      17 as libc::c_int, 12 as libc::c_int, 12 as libc::c_int,
      5 as libc::c_int, 6 as libc::c_int, 6 as libc::c_int, 15 as libc::c_int,
      15 as libc::c_int],
     [8 as libc::c_int, 8 as libc::c_int, 21 as libc::c_int,
      14 as libc::c_int, 13 as libc::c_int, 32 as libc::c_int,
      0 as libc::c_int, 0 as libc::c_int, 16 as libc::c_int,
      16 as libc::c_int, 24 as libc::c_int, 26 as libc::c_int,
      3 as libc::c_int, 3 as libc::c_int, 35 as libc::c_int,
      12 as libc::c_int, 5 as libc::c_int, 1 as libc::c_int,
      15 as libc::c_int, 6 as libc::c_int],
     [22 as libc::c_int, 8 as libc::c_int, 21 as libc::c_int,
      10 as libc::c_int, 10 as libc::c_int, 32 as libc::c_int,
      32 as libc::c_int, 32 as libc::c_int, 16 as libc::c_int,
      0 as libc::c_int, 0 as libc::c_int, 34 as libc::c_int,
      17 as libc::c_int, 12 as libc::c_int, 12 as libc::c_int,
      5 as libc::c_int, 11 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int,
      7 as libc::c_int],
     [8 as libc::c_int, 18 as libc::c_int, 19 as libc::c_int,
      20 as libc::c_int, 14 as libc::c_int, 9 as libc::c_int,
      2 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 16 as libc::c_int,
      33 as libc::c_int, 17 as libc::c_int, 17 as libc::c_int,
      1 as libc::c_int, 1 as libc::c_int, 5 as libc::c_int, 3 as libc::c_int,
      6 as libc::c_int, 7 as libc::c_int, 15 as libc::c_int],
     [22 as libc::c_int, 21 as libc::c_int, 14 as libc::c_int,
      2 as libc::c_int, 10 as libc::c_int, 32 as libc::c_int,
      32 as libc::c_int, 32 as libc::c_int, 32 as libc::c_int,
      0 as libc::c_int, 0 as libc::c_int, 25 as libc::c_int,
      34 as libc::c_int, 17 as libc::c_int, 5 as libc::c_int,
      5 as libc::c_int, 3 as libc::c_int, 1 as libc::c_int, 4 as libc::c_int,
      15 as libc::c_int],
     [22 as libc::c_int, 8 as libc::c_int, 21 as libc::c_int,
      19 as libc::c_int, 14 as libc::c_int, 32 as libc::c_int,
      32 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
      16 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
      35 as libc::c_int, 17 as libc::c_int, 1 as libc::c_int,
      12 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int, 4 as libc::c_int,
      15 as libc::c_int],
     [22 as libc::c_int, 20 as libc::c_int, 19 as libc::c_int,
      19 as libc::c_int, 20 as libc::c_int, 32 as libc::c_int,
      32 as libc::c_int, 32 as libc::c_int, 0 as libc::c_int,
      0 as libc::c_int, 16 as libc::c_int, 17 as libc::c_int,
      17 as libc::c_int, 1 as libc::c_int, 15 as libc::c_int,
      3 as libc::c_int, 3 as libc::c_int, 5 as libc::c_int, 4 as libc::c_int,
      7 as libc::c_int],
     [22 as libc::c_int, 34 as libc::c_int, 19 as libc::c_int,
      20 as libc::c_int, 2 as libc::c_int, 12 as libc::c_int,
      13 as libc::c_int, 16 as libc::c_int, 0 as libc::c_int,
      32 as libc::c_int, 23 as libc::c_int, 25 as libc::c_int,
      5 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int,
      15 as libc::c_int, 14 as libc::c_int, 10 as libc::c_int,
      7 as libc::c_int, 15 as libc::c_int],
     [22 as libc::c_int, 18 as libc::c_int, 18 as libc::c_int,
      10 as libc::c_int, 10 as libc::c_int, 32 as libc::c_int,
      32 as libc::c_int, 32 as libc::c_int, 35 as libc::c_int,
      34 as libc::c_int, 0 as libc::c_int, 16 as libc::c_int,
      17 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int, 5 as libc::c_int,
      6 as libc::c_int, 6 as libc::c_int, 4 as libc::c_int, 7 as libc::c_int],
     [22 as libc::c_int, 21 as libc::c_int, 21 as libc::c_int,
      14 as libc::c_int, 13 as libc::c_int, 2 as libc::c_int,
      32 as libc::c_int, 32 as libc::c_int, 16 as libc::c_int,
      34 as libc::c_int, 0 as libc::c_int, 17 as libc::c_int,
      17 as libc::c_int, 3 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
      5 as libc::c_int, 12 as libc::c_int, 4 as libc::c_int,
      15 as libc::c_int],
     [22 as libc::c_int, 21 as libc::c_int, 18 as libc::c_int,
      14 as libc::c_int, 13 as libc::c_int, 2 as libc::c_int,
      32 as libc::c_int, 32 as libc::c_int, 32 as libc::c_int,
      0 as libc::c_int, 0 as libc::c_int, 16 as libc::c_int,
      17 as libc::c_int, 3 as libc::c_int, 12 as libc::c_int,
      12 as libc::c_int, 33 as libc::c_int, 5 as libc::c_int,
      4 as libc::c_int, 15 as libc::c_int],
     [22 as libc::c_int, 19 as libc::c_int, 20 as libc::c_int,
      14 as libc::c_int, 13 as libc::c_int, 2 as libc::c_int,
      32 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
      0 as libc::c_int, 16 as libc::c_int, 17 as libc::c_int,
      5 as libc::c_int, 4 as libc::c_int, 12 as libc::c_int,
      12 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
      15 as libc::c_int],
     [22 as libc::c_int, 18 as libc::c_int, 21 as libc::c_int,
      13 as libc::c_int, 2 as libc::c_int, 32 as libc::c_int,
      32 as libc::c_int, 26 as libc::c_int, 11 as libc::c_int,
      11 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
      12 as libc::c_int, 12 as libc::c_int, 35 as libc::c_int,
      3 as libc::c_int, 5 as libc::c_int, 4 as libc::c_int,
      6 as libc::c_int]];
/* Names used for random artifact name generation */
#[no_mangle]
pub static mut artifact_names_list: cptr =
    b"adanedhel\nadurant\naeglos\naegnor\naelin\naeluin\naerandir\naerin\nagarwaen\naglareb\naglarond\naglon\nainulindale\nainur\nalcarinque\naldaron\naldudenie\nalmaren\nalqualonde\naman\namandil\namarie\namarth\namlach\namon\namras\namrod\nanach\nanar\nanarion\nancalagon\nancalimon\nanarrima\nandor\nandram\nandroth\nanduin\nandunie\nanfauglir\nanfauglith\nangainor\nangband\nanghabar\nanglachel\nangrenost\nangrim\nangrist\nangrod\nanguirel\nannael\nannatar\nannon\nannuminas\napanonar\naradan\naragorn\naraman\naranel\naranruth\naranwe\naras\naratan\naratar\narathorn\narda\nard-galen\naredhel\nar-feiniel\nargonath\narien\narmenelos\narminas\narnor\naros\narossiach\narthad\narvernien\narwen\nascar\nastaldo\natalante\natanamir\natanatari\natani\naule\navallone\navari\navathar\nbalan\nbalar\nbalrog\nbarad\nbaragund\nbarahir\nbaran\nbaranduin\nbar\nbauglir\nbeleg\nbelegaer\nbelegost\nbelegund\nbeleriand\nbelfalas\nbelthil\nbelthronding\nbeor\nberaid\nbereg\nberen\nboromir\nboron\nbragollach\nbrandir\nbregolas\nbregor\nbrethil\nbrilthor\nbrithiach\nbrithombar\nbrithon\ncabed\ncalacirya\ncalaquendi\ncalenardhon\ncalion\ncamlost\ncaragdur\ncaranthir\ncarcharoth\ncardolan\ncarnil\nceleborn\ncelebrant\ncelebrimbor\ncelebrindal\ncelebros\ncelegorm\ncelon\ncirdan\ncirith\ncirth\nciryatan\nciryon\ncoimas\ncorollaire\ncrissaegrim\ncuarthal\ncuivienen\nculurien\ncurufin\ncurufinwe\ncurunir\ncuthalion\ndaedeloth\ndaeron\ndagnir\ndagor\ndagorlad\ndairuin\ndanwedh\ndelduwath\ndenethor\ndimbar\ndimrost\ndinen\ndior\ndirnen\ndolmed\ndoriath\ndorlas\ndorthonion\ndraugluin\ndrengist\nduath\nduinath\nduilwen\ndunedain\ndungortheb\nearendil\nearendur\nearnil\nearnur\nearrame\nearwen\nechor\nechoriath\necthelion\nedain\nedrahil\neglador\neglarest\neglath\neilinel\neithel\nekkaia\nelbereth\neldalie\neldalieva\neldamar\neldar\neledhwen\nelemmire\nelende\nelendil\nelendur\nelenna\nelentari\nelenwe\nelerrina\nelleth\nelmoth\nelostirion\nelrond\nelros\nelu\neluchil\nelured\nelurin\nelwe\nelwing\nemeldir\nendor\nengrin\nengwar\neol\neonwe\nephel\nerchamion\nereb\nered\nerech\neregion\nereinion\nerellont\neressea\neriador\neru\nesgalduin\neste\nestel\nestolad\nethir\nezellohar\nfaelivrin\nfalas\nfalathar\nfalathrim\nfalmari\nfaroth\nfauglith\nfeanor\nfeanturi\nfelagund\nfinarfin\nfinduilas\nfingolfin\nfingon\nfinwe\nfirimar\nformenos\nfornost\nfrodo\nfuin\nfuinur\ngabilgathol\ngalad\ngaladriel\ngalathilion\ngaldor\ngalen\ngalvorn\ngandalf\ngaurhoth\ngelion\ngelmir\ngelydh\ngil\ngildor\ngiliath\nginglith\ngirith\nglaurung\nglingal\nglirhuin\ngloredhel\nglorfindel\ngolodhrim\ngondolin\ngondor\ngonnhirrim\ngorgoroth\ngorlim\ngorthaur\ngorthol\ngothmog\nguilin\nguinar\nguldur\ngundor\ngurthang\ngwaith\ngwareth\ngwindor\nhadhodrond\nhador\nhaladin\nhaldad\nhaldan\nhaldar\nhaldir\nhaleth\nhalmir\nhandir\nharad\nhareth\nhathaldir\nhathol\nhaudh\nhelcar\nhelcaraxe\nhelevorn\nhelluin\nherumor\nherunumen\nhildorien\nhimlad\nhimring\nhirilorn\nhisilome\nhithaeglir\nhithlum\nhollin\nhuan\nhunthor\nhuor\nhurin\nhyarmendacil\nhyarmentir\niant\niaur\nibun\nidril\nilluin\nilmare\nilmen\niluvatar\nimlach\nimladris\nindis\ningwe\nirmo\nisil\nisildur\nistari\nithil\nivrin\nkelvar\nkementari\nladros\nlaiquendi\nlalaith\nlamath\nlammoth\nlanthir\nlaurelin\nleithian\nlegolin\nlembas\nlenwe\nlinaewen\nlindon\nlindorie\nloeg\nlomelindi\nlomin\nlomion\nlorellin\nlorien\nlorindol\nlosgar\nlothlann\nlothlorien\nluin\nluinil\nlumbar\nluthien\nmablung\nmaedhros\nmaeglin\nmaglor\nmagor\nmahanaxar\nmahtan\nmaiar\nmalduin\nmalinalda\nmandos\nmanwe\nmardil\nmelian\nmelkor\nmenegroth\nmeneldil\nmenelmacar\nmeneltarma\nminas\nminastir\nmindeb\nmindolluin\nmindon\nminyatur\nmirdain\nmiriel\nmithlond\nmithrandir\nmithrim\nmordor\nmorgoth\nmorgul\nmoria\nmoriquendi\nmormegil\nmorwen\nnahar\nnaeramarth\nnamo\nnandor\nnargothrond\nnarog\nnarsil\nnarsilion\nnarya\nnauglamir\nnaugrim\nndengin\nneithan\nneldoreth\nnenar\nnenning\nnenuial\nnenya\nnerdanel\nnessa\nnevrast\nnibin\nnienna\nnienor\nnimbrethil\nnimloth\nnimphelos\nnimrais\nnimras\nningloron\nniniel\nninniach\nninquelote\nniphredil\nnirnaeth\nnivrim\nnoegyth\nnogrod\nnoldolante\nnoldor\nnumenor\nnurtale\nobel\nohtar\noiolosse\noiomure\nolorin\nolvar\nolwe\nondolinde\norfalch\normal\norocarni\norodreth\norodruin\norome\noromet\northanc\nosgiliath\nosse\nossiriand\npalantir\npelargir\npelori\nperiannath\nquendi\nquenta\nquenya\nradagast\nradhruin\nragnor\nramdal\nrana\nrathloriel\nrauros\nregion\nrerir\nrhovanion\nrhudaur\nrhun\nrhunen\nrian\nringil\nringwil\nromenna\nrudh\nrumil\nsaeros\nsalmar\nsaruman\nsauron\nserech\nseregon\nserinde\nshelob\nsilmarien\nsilmaril\nsilpion\nsindar\nsingollo\nsirion\nsoronume\nsul\nsulimo\ntalath\ntaniquetil\ntar\ntaras\ntarn\ntathren\ntaur\ntauron\nteiglin\ntelchar\ntelemnar\nteleri\ntelperion\ntelumendil\nthalion\nthalos\nthangorodrim\nthargelion\nthingol\nthoronath\nthorondor\nthranduil\nthuringwethil\ntilion\ntintalle\ntinuviel\ntirion\ntirith\ntol\ntulkas\ntumhalad\ntumladen\ntuna\ntuor\nturambar\nturgon\nturin\nuial\nuilos\nuinen\nulairi\nulmo\nulumuri\numanyar\numarth\numbar\nungoliant\nurthel\nuruloki\nutumno\nvaire\nvalacirca\nvalandil\nvalaquenta\nvalar\nvalaraukar\nvalaroma\nvalier\nvalimar\nvalinor\nvalinoreva\nvalmar\nvana\nvanyar\nvarda\nvasa\nvilya\nvingilot\nvinyamar\nvoronwe\nwethrin\nwilwarin\nyavanna\n\x00"
        as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut ma_blows: [martial_arts; 17] =
    [{
         let mut init =
             martial_arts{desc:
                              b"You punch %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 1 as libc::c_int,
                          chance: 0 as libc::c_int,
                          dd: 2 as libc::c_int,
                          ds: 4 as libc::c_int,
                          effect: 0 as libc::c_int as s16b,
                          power: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You kick %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 2 as libc::c_int,
                          chance: 0 as libc::c_int,
                          dd: 2 as libc::c_int,
                          ds: 6 as libc::c_int,
                          effect: 0 as libc::c_int as s16b,
                          power: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You strike %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 3 as libc::c_int,
                          chance: 0 as libc::c_int,
                          dd: 2 as libc::c_int,
                          ds: 7 as libc::c_int,
                          effect: 0 as libc::c_int as s16b,
                          power: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hit %s with your knee.\x00" as *const u8
                                  as *const libc::c_char,
                          min_level: 5 as libc::c_int,
                          chance: 5 as libc::c_int,
                          dd: 4 as libc::c_int,
                          ds: 3 as libc::c_int,
                          effect: 0x1 as libc::c_int as s16b,
                          power: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hit %s with your elbow.\x00" as *const u8
                                  as *const libc::c_char,
                          min_level: 7 as libc::c_int,
                          chance: 5 as libc::c_int,
                          dd: 2 as libc::c_int,
                          ds: 8 as libc::c_int,
                          effect: 0 as libc::c_int as s16b,
                          power: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You butt %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 9 as libc::c_int,
                          chance: 10 as libc::c_int,
                          dd: 4 as libc::c_int,
                          ds: 5 as libc::c_int,
                          effect: 0 as libc::c_int as s16b,
                          power: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You kick %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 11 as libc::c_int,
                          chance: 10 as libc::c_int,
                          dd: 6 as libc::c_int,
                          ds: 4 as libc::c_int,
                          effect: 0x2 as libc::c_int as s16b,
                          power: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You uppercut %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 13 as libc::c_int,
                          chance: 12 as libc::c_int,
                          dd: 8 as libc::c_int,
                          ds: 4 as libc::c_int,
                          effect: 0x8 as libc::c_int as s16b,
                          power: 6 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You double-kick %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 16 as libc::c_int,
                          chance: 15 as libc::c_int,
                          dd: 10 as libc::c_int,
                          ds: 4 as libc::c_int,
                          effect: 0x8 as libc::c_int as s16b,
                          power: 8 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hit %s with a Cat\'s Claw.\x00" as
                                  *const u8 as *const libc::c_char,
                          min_level: 20 as libc::c_int,
                          chance: 20 as libc::c_int,
                          dd: 10 as libc::c_int,
                          ds: 5 as libc::c_int,
                          effect: 0 as libc::c_int as s16b,
                          power: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hit %s with a jump kick.\x00" as *const u8
                                  as *const libc::c_char,
                          min_level: 25 as libc::c_int,
                          chance: 25 as libc::c_int,
                          dd: 10 as libc::c_int,
                          ds: 6 as libc::c_int,
                          effect: 0x8 as libc::c_int as s16b,
                          power: 10 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hit %s with an Eagle\'s Claw.\x00" as
                                  *const u8 as *const libc::c_char,
                          min_level: 29 as libc::c_int,
                          chance: 25 as libc::c_int,
                          dd: 12 as libc::c_int,
                          ds: 6 as libc::c_int,
                          effect: 0 as libc::c_int as s16b,
                          power: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hit %s with a circle kick.\x00" as
                                  *const u8 as *const libc::c_char,
                          min_level: 33 as libc::c_int,
                          chance: 30 as libc::c_int,
                          dd: 12 as libc::c_int,
                          ds: 8 as libc::c_int,
                          effect: 0x8 as libc::c_int as s16b,
                          power: 10 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hit %s with an Iron Fist.\x00" as
                                  *const u8 as *const libc::c_char,
                          min_level: 37 as libc::c_int,
                          chance: 35 as libc::c_int,
                          dd: 16 as libc::c_int,
                          ds: 8 as libc::c_int,
                          effect: 0x8 as libc::c_int as s16b,
                          power: 10 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hit %s with a flying kick.\x00" as
                                  *const u8 as *const libc::c_char,
                          min_level: 41 as libc::c_int,
                          chance: 35 as libc::c_int,
                          dd: 16 as libc::c_int,
                          ds: 10 as libc::c_int,
                          effect: 0x8 as libc::c_int as s16b,
                          power: 12 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hit %s with a Dragon Fist.\x00" as
                                  *const u8 as *const libc::c_char,
                          min_level: 45 as libc::c_int,
                          chance: 35 as libc::c_int,
                          dd: 20 as libc::c_int,
                          ds: 10 as libc::c_int,
                          effect: 0x8 as libc::c_int as s16b,
                          power: 16 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hit %s with a Crushing Blow.\x00" as
                                  *const u8 as *const libc::c_char,
                          min_level: 48 as libc::c_int,
                          chance: 35 as libc::c_int,
                          dd: 20 as libc::c_int,
                          ds: 12 as libc::c_int,
                          effect: 0x8 as libc::c_int as s16b,
                          power: 18 as libc::c_int as s16b,};
         init
     }];
/*
 *   cptr    desc;      A verbose attack description
 *   int     min_level; Minimum level to use
 *   int     chance;    Chance of 'success
 *   int     dd;        Damage dice
 *   int     ds;        Damage sides
 *   s16b    effect;    Special effects
 *   s16b    power;     Special effects power
 */
#[no_mangle]
pub static mut bear_blows: [martial_arts; 8] =
    [{
         let mut init =
             martial_arts{desc:
                              b"You claw %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 1 as libc::c_int,
                          chance: 0 as libc::c_int,
                          dd: 3 as libc::c_int,
                          ds: 4 as libc::c_int,
                          effect: 0x8 as libc::c_int as s16b,
                          power: 4 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You swat %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 4 as libc::c_int,
                          chance: 0 as libc::c_int,
                          dd: 4 as libc::c_int,
                          ds: 4 as libc::c_int,
                          effect: 0x4 as libc::c_int as s16b,
                          power: 20 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You bite %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 9 as libc::c_int,
                          chance: 2 as libc::c_int,
                          dd: 4 as libc::c_int,
                          ds: 4 as libc::c_int,
                          effect: 0x4 as libc::c_int as s16b,
                          power: 30 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hug %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 15 as libc::c_int,
                          chance: 5 as libc::c_int,
                          dd: 6 as libc::c_int,
                          ds: 4 as libc::c_int,
                          effect: 0x10 as libc::c_int as s16b,
                          power: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You swat and rake %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 25 as libc::c_int,
                          chance: 10 as libc::c_int,
                          dd: 6 as libc::c_int,
                          ds: 5 as libc::c_int,
                          effect:
                              (0x8 as libc::c_int | 0x4 as libc::c_int) as
                                  s16b,
                          power: 10 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You hug and claw %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 30 as libc::c_int,
                          chance: 15 as libc::c_int,
                          dd: 6 as libc::c_int,
                          ds: 6 as libc::c_int,
                          effect:
                              (0x10 as libc::c_int | 0x4 as libc::c_int) as
                                  s16b,
                          power: 60 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You double swat %s.\x00" as *const u8 as
                                  *const libc::c_char,
                          min_level: 35 as libc::c_int,
                          chance: 20 as libc::c_int,
                          dd: 9 as libc::c_int,
                          ds: 7 as libc::c_int,
                          effect:
                              (0x8 as libc::c_int | 0x4 as libc::c_int) as
                                  s16b,
                          power: 20 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             martial_arts{desc:
                              b"You double swat and rake %s.\x00" as *const u8
                                  as *const libc::c_char,
                          min_level: 40 as libc::c_int,
                          chance: 25 as libc::c_int,
                          dd: 10 as libc::c_int,
                          ds: 10 as libc::c_int,
                          effect:
                              (0x8 as libc::c_int | 0x4 as libc::c_int) as
                                  s16b,
                          power: 25 as libc::c_int as s16b,};
         init
     }];
#[no_mangle]
pub static mut mindcraft_powers: [magic_power; 12] =
    [{
         let mut init =
             magic_power{min_lev: 1 as libc::c_int,
                         mana_cost: 1 as libc::c_int,
                         fail: 15 as libc::c_int,
                         name:
                             b"Precognition\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Detect monsters, traps and level layout and lights up at higher levels.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 2 as libc::c_int,
                         mana_cost: 1 as libc::c_int,
                         fail: 20 as libc::c_int,
                         name:
                             b"Neural Blast\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Blast the minds of your foes.\x00" as *const u8
                                 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 3 as libc::c_int,
                         mana_cost: 2 as libc::c_int,
                         fail: 25 as libc::c_int,
                         name:
                             b"Minor Displacement\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Short distance teleportation\x00" as *const u8
                                 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 7 as libc::c_int,
                         mana_cost: 6 as libc::c_int,
                         fail: 35 as libc::c_int,
                         name:
                             b"Major Displacement\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Teleport you and others at high levels.\x00" as
                                 *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 9 as libc::c_int,
                         mana_cost: 7 as libc::c_int,
                         fail: 50 as libc::c_int,
                         name:
                             b"Domination\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Charm monsters\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 11 as libc::c_int,
                         mana_cost: 7 as libc::c_int,
                         fail: 30 as libc::c_int,
                         name:
                             b"Pulverise\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Fires a bolt of pure sound.\x00" as *const u8
                                 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 13 as libc::c_int,
                         mana_cost: 12 as libc::c_int,
                         fail: 50 as libc::c_int,
                         name:
                             b"Character Armour\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Sets up physical/elemental shield.\x00" as
                                 *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 15 as libc::c_int,
                         mana_cost: 12 as libc::c_int,
                         fail: 60 as libc::c_int,
                         name:
                             b"Psychometry\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Identifies objects.\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 18 as libc::c_int,
                         mana_cost: 10 as libc::c_int,
                         fail: 45 as libc::c_int,
                         name:
                             b"Mind Wave\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Projects psi waves to crush the minds of your foes.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 23 as libc::c_int,
                         mana_cost: 15 as libc::c_int,
                         fail: 50 as libc::c_int,
                         name:
                             b"Adrenaline Channeling\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Heals you, cures you and speeds you.\x00" as
                                 *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 25 as libc::c_int,
                         mana_cost: 10 as libc::c_int,
                         fail: 40 as libc::c_int,
                         name:
                             b"Psychic Drain\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Drain your foes\' life into your mana reserves\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 28 as libc::c_int,
                         mana_cost: 20 as libc::c_int,
                         fail: 45 as libc::c_int,
                         name:
                             b"Telekinetic Wave\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Powerful wave of pure telekinetic forces.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     }];
#[no_mangle]
pub static mut necro_powers: [magic_power; 6] =
    [{
         let mut init =
             magic_power{min_lev: 1 as libc::c_int,
                         mana_cost: 2 as libc::c_int,
                         fail: 10 as libc::c_int,
                         name:
                             b"Horrify\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Calls upon the darkness to stun and scare your foes.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 5 as libc::c_int,
                         mana_cost: 6 as libc::c_int,
                         fail: 20 as libc::c_int,
                         name:
                             b"Raise Dead\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Brings back your foes in the form of various undead.  Also, can heal monsters.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 12 as libc::c_int,
                         mana_cost: 20 as libc::c_int,
                         fail: 25 as libc::c_int,
                         name:
                             b"Necromantic Teeth\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Conjures a temporary vampiric weapon.\x00" as
                                 *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 20 as libc::c_int,
                         mana_cost: 10 as libc::c_int,
                         fail: 25 as libc::c_int,
                         name:
                             b"Absorb Soul\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Gives back some life for each kill.\x00" as
                                 *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 30 as libc::c_int,
                         mana_cost: 15 as libc::c_int,
                         fail: 20 as libc::c_int,
                         name:
                             b"Vampirism\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Drain the life of your foes into your own.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 35 as libc::c_int,
                         mana_cost: 100 as libc::c_int,
                         fail: 25 as libc::c_int,
                         name:
                             b"Death\x00" as *const u8 as *const libc::c_char,
                         desc:
                             b"Instantly kills your opponent and you, turning yourself into an undead.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     }];
#[no_mangle]
pub static mut mimic_powers: [magic_power; 5] =
    [{
         let mut init =
             magic_power{min_lev: 1 as libc::c_int,
                         mana_cost: 2 as libc::c_int,
                         fail: 0 as libc::c_int,
                         name:
                             b"Mimic\x00" as *const u8 as *const libc::c_char,
                         desc:
                             b"Lets you use the powers of a Cloak of Mimicry.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 10 as libc::c_int,
                         mana_cost: 6 as libc::c_int,
                         fail: 20 as libc::c_int,
                         name:
                             b"Invisibility\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Hides you from the sight of mortals.\x00" as
                                 *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 25 as libc::c_int,
                         mana_cost: 20 as libc::c_int,
                         fail: 25 as libc::c_int,
                         name:
                             b"Legs Mimicry\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Temporarily provides a new pair of legs.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 30 as libc::c_int,
                         mana_cost: 40 as libc::c_int,
                         fail: 30 as libc::c_int,
                         name:
                             b"Wall Mimicry\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Temporarily lets you walk in walls, and ONLY in walls.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 35 as libc::c_int,
                         mana_cost: 100 as libc::c_int,
                         fail: 40 as libc::c_int,
                         name:
                             b"Arms Mimicry\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Temporarily provides a new pair of arms.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     }];
#[no_mangle]
pub static mut symbiotic_powers: [magic_power; 9] =
    [{
         let mut init =
             magic_power{min_lev: 1 as libc::c_int,
                         mana_cost: 1 as libc::c_int,
                         fail: 0 as libc::c_int,
                         name:
                             b"Hypnotise\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Hypnotise a non-moving pet to allow you to enter symbiosis(wear) with it.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 1 as libc::c_int,
                         mana_cost: 1 as libc::c_int,
                         fail: 0 as libc::c_int,
                         name:
                             b"Release\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Release an hypnotised pet.\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 3 as libc::c_int,
                         mana_cost: 2 as libc::c_int,
                         fail: 10 as libc::c_int,
                         name:
                             b"Charm Never-Moving\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Tries to charm a never-moving monster.\x00" as
                                 *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 5 as libc::c_int,
                         mana_cost: 5 as libc::c_int,
                         fail: 20 as libc::c_int,
                         name:
                             b"Life Share\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Evens out your life with your symbiote.\x00" as
                                 *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 10 as libc::c_int,
                         mana_cost: 10 as libc::c_int,
                         fail: 20 as libc::c_int,
                         name:
                             b"Use Minor Powers\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Allows you to use some of the powers of your symbiote.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 15 as libc::c_int,
                         mana_cost: 14 as libc::c_int,
                         fail: 25 as libc::c_int,
                         name:
                             b"Heal Symbiote\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Heals your symbiotic monster.\x00" as *const u8
                                 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 25 as libc::c_int,
                         mana_cost: 30 as libc::c_int,
                         fail: 40 as libc::c_int,
                         name:
                             b"Use major powers\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Allows you to use all the powers of your symbiote.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 30 as libc::c_int,
                         mana_cost: 35 as libc::c_int,
                         fail: 40 as libc::c_int,
                         name:
                             b"Summon Never-Moving Pet\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Summons a never-moving pet.\x00" as *const u8
                                 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             magic_power{min_lev: 40 as libc::c_int,
                         mana_cost: 60 as libc::c_int,
                         fail: 70 as libc::c_int,
                         name:
                             b"Force Symbiosis\x00" as *const u8 as
                                 *const libc::c_char,
                         desc:
                             b"Allows you to use all the powers of a monster in your line of sight.\x00"
                                 as *const u8 as *const libc::c_char,};
         init
     }];
/*
 * Textual translation of your god's "niceness".
 */
#[no_mangle]
pub static mut deity_niceness: [cptr; 10] =
    [b"a lovable deity\x00" as *const u8 as *const libc::c_char,
     b"a friendly deity\x00" as *const u8 as *const libc::c_char,
     b"an easygoing deity\x00" as *const u8 as *const libc::c_char,
     b"a forgiving deity\x00" as *const u8 as *const libc::c_char,
     b"an uncaring deity\x00" as *const u8 as *const libc::c_char,
     b"a wary deity\x00" as *const u8 as *const libc::c_char,
     b"an unforgiving deity\x00" as *const u8 as *const libc::c_char,
     b"an impatient deity\x00" as *const u8 as *const libc::c_char,
     b"a wrathful deity\x00" as *const u8 as *const libc::c_char,
     b"an easily angered deity\x00" as *const u8 as *const libc::c_char];
/*
 * Textual translation of your standing with your god.
 */
#[no_mangle]
pub static mut deity_standing: [cptr; 11] =
    [b"cursed\x00" as *const u8 as *const libc::c_char,
     b"persecuted\x00" as *const u8 as *const libc::c_char,
     b"punished\x00" as *const u8 as *const libc::c_char,
     b"despised\x00" as *const u8 as *const libc::c_char,
     b"disliked\x00" as *const u8 as *const libc::c_char,
     b"watched\x00" as *const u8 as *const libc::c_char,
     b"unnoticed\x00" as *const u8 as *const libc::c_char,
     b"noticed\x00" as *const u8 as *const libc::c_char,
     b"rewarded\x00" as *const u8 as *const libc::c_char,
     b"favored\x00" as *const u8 as *const libc::c_char,
     b"championed\x00" as *const u8 as *const libc::c_char];
/*
 * Name and description (max. 10 lines) of the gods.
 * Only the first four lines are printed at birth. 
 */
#[no_mangle]
pub static mut deity_info_init: [deity_type; 6] =
    unsafe {
        [{
             let mut init =
                 deity_type{name:
                                b"Nobody\x00" as *const u8 as
                                    *const libc::c_char,
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Atheist\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],};
             init
         },
         {
             let mut init =
                 deity_type{name:
                                b"Eru Iluvatar\x00" as *const u8 as
                                    *const libc::c_char,
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"He is the supreme god, he created the world, and most of its inhabitants.\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],};
             init
         },
         {
             let mut init =
                 deity_type{name:
                                b"Manwe Sulimo\x00" as *const u8 as
                                    *const libc::c_char,
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"He is the king of the Valar, most powerful of them after Melkor.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],};
             init
         },
         {
             let mut init =
                 deity_type{name:
                                b"Tulkas\x00" as *const u8 as
                                    *const libc::c_char,
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"He is the last of the Valar that came to the world, and the fiercest fighter.\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],};
             init
         },
         {
             let mut init =
                 deity_type{name:
                                b"Melkor Bauglir\x00" as *const u8 as
                                    *const libc::c_char,
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"He is the most powerful of the Valar. He became corrupted and he\'s now \x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"the greatest threat of Arda, he is also known as Morgoth, the dark enemy.\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],};
             init
         },
         {
             let mut init =
                 deity_type{name:
                                b"Yavanna Kementari\x00" as *const u8 as
                                    *const libc::c_char,
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"She is the Vala of nature, protectress of the great forests of Middle-earth.\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],};
             init
         }]
    };
/* jk - to hit, to dam, to ac, to stealth, to disarm, to saving throw */
/* this concept is taken from Adom, where Thomas Biskup thought it out, */
/* as far as I know. */
#[no_mangle]
pub static mut tactic_info: [tactic_info_type; 9] =
    [{
         let mut init =
             tactic_info_type{to_hit: -(10 as libc::c_int) as s16b,
                              to_dam: -(10 as libc::c_int) as s16b,
                              to_ac: 15 as libc::c_int as s16b,
                              to_stealth: 3 as libc::c_int as s16b,
                              to_disarm: 15 as libc::c_int as s16b,
                              to_saving: 14 as libc::c_int as s16b,
                              name:
                                  b"coward\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             tactic_info_type{to_hit: -(8 as libc::c_int) as s16b,
                              to_dam: -(8 as libc::c_int) as s16b,
                              to_ac: 10 as libc::c_int as s16b,
                              to_stealth: 2 as libc::c_int as s16b,
                              to_disarm: 9 as libc::c_int as s16b,
                              to_saving: 9 as libc::c_int as s16b,
                              name:
                                  b"meek\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             tactic_info_type{to_hit: -(4 as libc::c_int) as s16b,
                              to_dam: -(4 as libc::c_int) as s16b,
                              to_ac: 5 as libc::c_int as s16b,
                              to_stealth: 1 as libc::c_int as s16b,
                              to_disarm: 5 as libc::c_int as s16b,
                              to_saving: 5 as libc::c_int as s16b,
                              name:
                                  b"wary\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             tactic_info_type{to_hit: -(2 as libc::c_int) as s16b,
                              to_dam: -(2 as libc::c_int) as s16b,
                              to_ac: 2 as libc::c_int as s16b,
                              to_stealth: 1 as libc::c_int as s16b,
                              to_disarm: 2 as libc::c_int as s16b,
                              to_saving: 2 as libc::c_int as s16b,
                              name:
                                  b"careful\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             tactic_info_type{to_hit: 0 as libc::c_int as s16b,
                              to_dam: 0 as libc::c_int as s16b,
                              to_ac: 0 as libc::c_int as s16b,
                              to_stealth: 0 as libc::c_int as s16b,
                              to_disarm: 0 as libc::c_int as s16b,
                              to_saving: 0 as libc::c_int as s16b,
                              name:
                                  b"normal\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             tactic_info_type{to_hit: 2 as libc::c_int as s16b,
                              to_dam: 2 as libc::c_int as s16b,
                              to_ac: -(2 as libc::c_int) as s16b,
                              to_stealth: -(1 as libc::c_int) as s16b,
                              to_disarm: -(2 as libc::c_int) as s16b,
                              to_saving: -(3 as libc::c_int) as s16b,
                              name:
                                  b"confident\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             tactic_info_type{to_hit: 4 as libc::c_int as s16b,
                              to_dam: 4 as libc::c_int as s16b,
                              to_ac: -(5 as libc::c_int) as s16b,
                              to_stealth: -(2 as libc::c_int) as s16b,
                              to_disarm: -(5 as libc::c_int) as s16b,
                              to_saving: -(7 as libc::c_int) as s16b,
                              name:
                                  b"aggressive\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             tactic_info_type{to_hit: 6 as libc::c_int as s16b,
                              to_dam: 6 as libc::c_int as s16b,
                              to_ac: -(10 as libc::c_int) as s16b,
                              to_stealth: -(3 as libc::c_int) as s16b,
                              to_disarm: -(11 as libc::c_int) as s16b,
                              to_saving: -(12 as libc::c_int) as s16b,
                              name:
                                  b"furious\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     },
     {
         let mut init =
             tactic_info_type{to_hit: 8 as libc::c_int as s16b,
                              to_dam: 12 as libc::c_int as s16b,
                              to_ac: -(25 as libc::c_int) as s16b,
                              to_stealth: -(5 as libc::c_int) as s16b,
                              to_disarm: -(18 as libc::c_int) as s16b,
                              to_saving: -(18 as libc::c_int) as s16b,
                              name:
                                  b"berserker\x00" as *const u8 as
                                      *const libc::c_char,};
         init
     }];
/*
 * Random artifact activations.
 */
#[no_mangle]
pub static mut activation_info: [activation; 51] =
    unsafe {
        [{
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"death\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 127 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"ruination\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 128 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"destruction\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 129 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"stupidity\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 130 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"weakness\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 131 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"unhealth\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 132 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"ugliness\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 133 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"clumsiness\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 134 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"naivete\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 135 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"stat loss\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 136 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"huge stat loss\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 137 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"experience loss\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 138 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"huge experience loss\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 139 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"teleportation\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 125 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"monster summoning\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 5 as libc::c_int as u32b,
                            spell: 140 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"paralyzation\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 141 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"hallucination\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 100 as libc::c_int as u32b,
                            spell: 142 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"poisoning\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 143 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"hunger\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 144 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"stun\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 145 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cuts\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 146 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"paranoia\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 147 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"confusion\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 148 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"blindness\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 149 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"pet summoning\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1010 as libc::c_int as u32b,
                            spell: 150 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure paralyzation\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 5000 as libc::c_int as u32b,
                            spell: 151 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure hallucination\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 152 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure poison\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 153 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure hunger\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 154 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure stun\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 155 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure cut\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 156 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure fear\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 157 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure confusion\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 158 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure blindness\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 159 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure light wounds\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 500 as libc::c_int as u32b,
                            spell: 81 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure serious wounds\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 750 as libc::c_int as u32b,
                            spell: 82 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure critical wounds\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 86 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"curing\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1100 as libc::c_int as u32b,
                            spell: 160 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"genocide\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 5000 as libc::c_int as u32b,
                            spell: 57 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"mass genocide\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 10000 as libc::c_int as u32b,
                            spell: 58 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"restoration\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 2000 as libc::c_int as u32b,
                            spell: 85 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"light\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 111 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"darkness\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 161 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"teleportation\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 1000 as libc::c_int as u32b,
                            spell: 125 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"level teleportation\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 500 as libc::c_int as u32b,
                            spell: 162 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"acquirement\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 30000 as libc::c_int as u32b,
                            spell: 163 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"something weird\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 50 as libc::c_int as u32b,
                            spell: 164 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"aggravation\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 0 as libc::c_int as u32b,
                            spell: 165 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"corruption\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 100 as libc::c_int as u32b,
                            spell: 166 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"cure insanity\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 2000 as libc::c_int as u32b,
                            spell: 167 as libc::c_int as s16b,};
             init
         },
         {
             let mut init =
                 activation{desc:
                                *::std::mem::transmute::<&[u8; 80],
                                                         &mut [libc::c_char; 80]>(b"light absortion\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            cost: 800 as libc::c_int as u32b,
                            spell: 169 as libc::c_int as s16b,};
             init
         }]
    };
/*
 * Possible movement type.
 */
#[no_mangle]
pub static mut move_info: [move_info_type; 9] =
    [{
         let mut init =
             move_info_type{to_speed: -(10 as libc::c_int) as s16b,
                            to_search: 17 as libc::c_int as s16b,
                            to_stealth: 4 as libc::c_int as s16b,
                            to_percep: 20 as libc::c_int as s16b,
                            name:
                                b"slug-like\x00" as *const u8 as
                                    *const libc::c_char,};
         init
     },
     {
         let mut init =
             move_info_type{to_speed: -(8 as libc::c_int) as s16b,
                            to_search: 12 as libc::c_int as s16b,
                            to_stealth: 4 as libc::c_int as s16b,
                            to_percep: 16 as libc::c_int as s16b,
                            name:
                                b"very slow\x00" as *const u8 as
                                    *const libc::c_char,};
         init
     },
     {
         let mut init =
             move_info_type{to_speed: -(6 as libc::c_int) as s16b,
                            to_search: 8 as libc::c_int as s16b,
                            to_stealth: 3 as libc::c_int as s16b,
                            to_percep: 10 as libc::c_int as s16b,
                            name:
                                b"slow\x00" as *const u8 as
                                    *const libc::c_char,};
         init
     },
     {
         let mut init =
             move_info_type{to_speed: -(3 as libc::c_int) as s16b,
                            to_search: 4 as libc::c_int as s16b,
                            to_stealth: 2 as libc::c_int as s16b,
                            to_percep: 6 as libc::c_int as s16b,
                            name:
                                b"leisurely\x00" as *const u8 as
                                    *const libc::c_char,};
         init
     },
     {
         let mut init =
             move_info_type{to_speed: 0 as libc::c_int as s16b,
                            to_search: 0 as libc::c_int as s16b,
                            to_stealth: 0 as libc::c_int as s16b,
                            to_percep: 0 as libc::c_int as s16b,
                            name:
                                b"normal\x00" as *const u8 as
                                    *const libc::c_char,};
         init
     },
     {
         let mut init =
             move_info_type{to_speed: 1 as libc::c_int as s16b,
                            to_search: -(4 as libc::c_int) as s16b,
                            to_stealth: -(1 as libc::c_int) as s16b,
                            to_percep: -(4 as libc::c_int) as s16b,
                            name:
                                b"brisk\x00" as *const u8 as
                                    *const libc::c_char,};
         init
     },
     {
         let mut init =
             move_info_type{to_speed: 2 as libc::c_int as s16b,
                            to_search: -(6 as libc::c_int) as s16b,
                            to_stealth: -(4 as libc::c_int) as s16b,
                            to_percep: -(8 as libc::c_int) as s16b,
                            name:
                                b"fast\x00" as *const u8 as
                                    *const libc::c_char,};
         init
     },
     {
         let mut init =
             move_info_type{to_speed: 3 as libc::c_int as s16b,
                            to_search: -(10 as libc::c_int) as s16b,
                            to_stealth: -(7 as libc::c_int) as s16b,
                            to_percep: -(14 as libc::c_int) as s16b,
                            name:
                                b"very fast\x00" as *const u8 as
                                    *const libc::c_char,};
         init
     },
     {
         let mut init =
             move_info_type{to_speed: 4 as libc::c_int as s16b,
                            to_search: -(16 as libc::c_int) as s16b,
                            to_stealth: -(10 as libc::c_int) as s16b,
                            to_percep: -(20 as libc::c_int) as s16b,
                            name:
                                b"running\x00" as *const u8 as
                                    *const libc::c_char,};
         init
     }];
/*
 * Possible inscriptions type.
 */
#[no_mangle]
pub static mut inscription_info: [inscription_info_type; 8] =
    unsafe {
        [{
             let mut init =
                 inscription_info_type{text:
                                           *::std::mem::transmute::<&[u8; 40],
                                                                    &mut [libc::c_char; 40]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                       when: 0 as libc::c_int as byte_hack,
                                       know: 1 as libc::c_int as bool_,
                                       mana: 0 as libc::c_int as byte_hack,};
             init
         },
         {
             let mut init =
                 inscription_info_type{text:
                                           *::std::mem::transmute::<&[u8; 40],
                                                                    &mut [libc::c_char; 40]>(b"ure nimir\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                       when:
                                           (0x1 as libc::c_int |
                                                0x2 as libc::c_int |
                                                0x4 as libc::c_int) as
                                               byte_hack,
                                       know: 0 as libc::c_int as bool_,
                                       mana: 30 as libc::c_int as byte_hack,};
             init
         },
         {
             let mut init =
                 inscription_info_type{text:
                                           *::std::mem::transmute::<&[u8; 40],
                                                                    &mut [libc::c_char; 40]>(b"lomi gimli\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                       when:
                                           (0x1 as libc::c_int |
                                                0x2 as libc::c_int |
                                                0x4 as libc::c_int) as
                                               byte_hack,
                                       know: 0 as libc::c_int as bool_,
                                       mana: 10 as libc::c_int as byte_hack,};
             init
         },
         {
             let mut init =
                 inscription_info_type{text:
                                           *::std::mem::transmute::<&[u8; 40],
                                                                    &mut [libc::c_char; 40]>(b"dulgi bawiba\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                       when:
                                           (0x1 as libc::c_int |
                                                0x2 as libc::c_int |
                                                0x4 as libc::c_int) as
                                               byte_hack,
                                       know: 0 as libc::c_int as bool_,
                                       mana: 40 as libc::c_int as byte_hack,};
             init
         },
         {
             let mut init =
                 inscription_info_type{text:
                                           *::std::mem::transmute::<&[u8; 40],
                                                                    &mut [libc::c_char; 40]>(b"pedo mellon a minno\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                       when: 0x4 as libc::c_int as byte_hack,
                                       know: 0 as libc::c_int as bool_,
                                       mana: 8 as libc::c_int as byte_hack,};
             init
         },
         {
             let mut init =
                 inscription_info_type{text:
                                           *::std::mem::transmute::<&[u8; 40],
                                                                    &mut [libc::c_char; 40]>(b"Baruk Khazad! Khazad aimenu!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                       when: 0x1 as libc::c_int as byte_hack,
                                       know: 0 as libc::c_int as bool_,
                                       mana:
                                           100 as libc::c_int as byte_hack,};
             init
         },
         {
             let mut init =
                 inscription_info_type{text:
                                           *::std::mem::transmute::<&[u8; 40],
                                                                    &mut [libc::c_char; 40]>(b"dunna hrassa\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                       when: 0x4 as libc::c_int as byte_hack,
                                       know: 0 as libc::c_int as bool_,
                                       mana: 50 as libc::c_int as byte_hack,};
             init
         },
         {
             let mut init =
                 inscription_info_type{text:
                                           *::std::mem::transmute::<&[u8; 40],
                                                                    &mut [libc::c_char; 40]>(b"burz ghash ronk\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                       when:
                                           (0x1 as libc::c_int |
                                                0x2 as libc::c_int |
                                                0x4 as libc::c_int) as
                                               byte_hack,
                                       know: 0 as libc::c_int as bool_,
                                       mana: 60 as libc::c_int as byte_hack,};
             init
         }]
    };
/*
 * Inscriptions for pseudo-id
 */
#[no_mangle]
pub static mut sense_desc: [cptr; 11] =
    [b"whoops\x00" as *const u8 as *const libc::c_char,
     b"cursed\x00" as *const u8 as *const libc::c_char,
     b"average\x00" as *const u8 as *const libc::c_char,
     b"good\x00" as *const u8 as *const libc::c_char,
     b"good\x00" as *const u8 as *const libc::c_char,
     b"excellent\x00" as *const u8 as *const libc::c_char,
     b"worthless\x00" as *const u8 as *const libc::c_char,
     b"terrible\x00" as *const u8 as *const libc::c_char,
     b"special\x00" as *const u8 as *const libc::c_char,
     b"broken\x00" as *const u8 as *const libc::c_char,
     b"\x00" as *const u8 as *const libc::c_char];
/*
 * Flag groups used for art creation, level gaining weapons, ...
 * -----
 * Name,
 * Price,
 * Flags 1,
 * Flags 2,
 * Flags 3,
 * Flags 4,
 * ESP,
 */
#[no_mangle]
pub static mut flags_groups: [flags_group; 12] =
    unsafe {
        [{
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Fire\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 12 as libc::c_int as byte_hack,
                             price: 1 as libc::c_int as byte_hack,
                             flags1:
                                 (0x40000 as libc::c_long |
                                      0x40000000 as libc::c_long) as u32b,
                             flags2: 0x40000 as libc::c_long as u32b,
                             flags3:
                                 (0x1 as libc::c_long | 0x2000 as libc::c_long
                                      | 0x400000 as libc::c_long) as u32b,
                             flags4: 0 as libc::c_int as u32b,
                             esp: 0 as libc::c_int as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Cold\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 1 as libc::c_int as byte_hack,
                             price: 1 as libc::c_int as byte_hack,
                             flags1:
                                 (0x800000 as libc::c_long |
                                      0x80000 as libc::c_long |
                                      0x80000000 as libc::c_long) as u32b,
                             flags2:
                                 (0x80000 as libc::c_long |
                                      0x40 as libc::c_long) as u32b,
                             flags3:
                                 (0x10000 as libc::c_long |
                                      0x800000 as libc::c_long) as u32b,
                             flags4: 0 as libc::c_int as u32b,
                             esp: 0 as libc::c_int as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Acid\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 5 as libc::c_int as byte_hack,
                             price: 3 as libc::c_int as byte_hack,
                             flags1:
                                 (0x10000 as libc::c_long |
                                      0x4000000 as libc::c_long |
                                      0x800 as libc::c_long |
                                      0x10000000 as libc::c_long) as u32b,
                             flags2: 0x10000 as libc::c_long as u32b,
                             flags3: 0x100000 as libc::c_long as u32b,
                             flags4: 0 as libc::c_int as u32b,
                             esp: 0 as libc::c_int as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Lightning\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 14 as libc::c_int as byte_hack,
                             price: 1 as libc::c_int as byte_hack,
                             flags1:
                                 (0x20000 as libc::c_long |
                                      0x20000000 as libc::c_long) as u32b,
                             flags2: 0x20000 as libc::c_long as u32b,
                             flags3:
                                 (0x200000 as libc::c_long |
                                      0x2 as libc::c_long |
                                      0x4000000 as libc::c_long) as u32b,
                             flags4: 0 as libc::c_int as u32b,
                             esp: 0 as libc::c_int as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Poison\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 13 as libc::c_int as byte_hack,
                             price: 2 as libc::c_int as byte_hack,
                             flags1:
                                 (0x20 as libc::c_long |
                                      0x8000 as libc::c_long |
                                      0x10000 as libc::c_long |
                                      0x8000000 as libc::c_long) as u32b,
                             flags2:
                                 (0x20 as libc::c_long |
                                      0x100000 as libc::c_long) as u32b,
                             flags3: 0x2000000 as libc::c_long as u32b,
                             flags4: 0 as libc::c_int as u32b,
                             esp:
                                 (0x2 as libc::c_long | 0x8 as libc::c_long)
                                     as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Air\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 6 as libc::c_int as byte_hack,
                             price: 5 as libc::c_int as byte_hack,
                             flags1:
                                 (0x4 as libc::c_long | 0x100 as libc::c_long
                                      | 0x400 as libc::c_long |
                                      0x1000 as libc::c_long) as u32b,
                             flags2:
                                 (0x400000 as libc::c_long |
                                      0x800000 as libc::c_long |
                                      0x1000000 as libc::c_long |
                                      0x4 as libc::c_long) as u32b,
                             flags3:
                                 (0x1000 as libc::c_long |
                                      0x4000 as libc::c_long |
                                      0x10000000 as libc::c_long) as u32b,
                             flags4: 0 as libc::c_int as u32b,
                             esp: 0x200 as libc::c_long as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Earth\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 15 as libc::c_int as byte_hack,
                             price: 5 as libc::c_int as byte_hack,
                             flags1:
                                 (0x1 as libc::c_long | 0x10 as libc::c_long |
                                      0x800 as libc::c_long |
                                      0x2000 as libc::c_long |
                                      0x200000 as libc::c_long |
                                      0x400000 as libc::c_long |
                                      0x4000000 as libc::c_long) as u32b,
                             flags2:
                                 (0x1 as libc::c_long | 0x10 as libc::c_long |
                                      0x4000 as libc::c_long |
                                      0x200000 as libc::c_long |
                                      0x8000000 as libc::c_long) as u32b,
                             flags3: 0x20000 as libc::c_long as u32b,
                             flags4: 0 as libc::c_int as u32b,
                             esp:
                                 (0x2 as libc::c_long | 0x8 as libc::c_long)
                                     as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Mind\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 11 as libc::c_int as byte_hack,
                             price: 7 as libc::c_int as byte_hack,
                             flags1:
                                 (0x2 as libc::c_long | 0x200 as libc::c_long)
                                     as u32b,
                             flags2:
                                 (0x2 as libc::c_long |
                                      0x2000000 as libc::c_long |
                                      0x200000 as libc::c_long) as u32b,
                             flags3: 0 as libc::c_int as u32b,
                             flags4: 0 as libc::c_int as u32b,
                             esp:
                                 (0x1 as libc::c_long | 0x2 as libc::c_long |
                                      0x8 as libc::c_long |
                                      0x80 as libc::c_long |
                                      0x800 as libc::c_long |
                                      0x1000 as libc::c_long |
                                      0x10 as libc::c_long) as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Shield\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 4 as libc::c_int as byte_hack,
                             price: 7 as libc::c_int as byte_hack,
                             flags1: 0x8 as libc::c_long as u32b,
                             flags2:
                                 (0x8 as libc::c_long | 0x40 as libc::c_long |
                                      0x2000 as libc::c_long |
                                      0x8000 as libc::c_long |
                                      0x4000000 as libc::c_long |
                                      0x20000000 as libc::c_long) as u32b,
                             flags3: 0x20000 as libc::c_long as u32b,
                             flags4: 0 as libc::c_int as u32b,
                             esp: 0 as libc::c_int as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Chaos\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 10 as libc::c_int as byte_hack,
                             price: 7 as libc::c_int as byte_hack,
                             flags1:
                                 (0x4000 as libc::c_long |
                                      0x4000000 as libc::c_long) as u32b,
                             flags2:
                                 (0x40000000 as libc::c_long |
                                      0x80000000 as libc::c_long) as u32b,
                             flags3: 0x20000 as libc::c_long as u32b,
                             flags4: 0 as libc::c_int as u32b,
                             esp: 0x80000000 as libc::c_long as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Magic\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 14 as libc::c_int as byte_hack,
                             price: 10 as libc::c_int as byte_hack,
                             flags1:
                                 (0x40 as libc::c_long | 0x80 as libc::c_long)
                                     as u32b,
                             flags2:
                                 (0x40000000 as libc::c_long |
                                      0x80000000 as libc::c_long) as u32b,
                             flags3: 0x40 as libc::c_long as u32b,
                             flags4:
                                 (0x2 as libc::c_long | 0x10 as libc::c_long |
                                      0x200 as libc::c_long) as u32b,
                             esp: 0 as libc::c_int as u32b,};
             init
         },
         {
             let mut init =
                 flags_group{name:
                                 *::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"Antimagic\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                             color: 8 as libc::c_int as byte_hack,
                             price: 10 as libc::c_int as byte_hack,
                             flags1:
                                 (0x8000 as libc::c_long |
                                      0x4000 as libc::c_long |
                                      0x2000 as libc::c_long |
                                      0x1000 as libc::c_long) as u32b,
                             flags2:
                                 (0x80 as libc::c_long |
                                      0x2000 as libc::c_long |
                                      0x4000 as libc::c_long |
                                      0x8000 as libc::c_long) as u32b,
                             flags3:
                                 (0x20 as libc::c_long | 0x10 as libc::c_long
                                      | 0x4000 as libc::c_long) as u32b,
                             flags4:
                                 (0x100000 as libc::c_long |
                                      0x80000 as libc::c_long) as u32b,
                             esp: 0 as libc::c_int as u32b,};
             init
         }]
    };
/* Powers */
#[no_mangle]
pub static mut powers_type_init: [power_type; 62] =
    [{
         let mut init =
             power_type{name:
                            b"spit acid\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can spit acid.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to spit acid.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to spit acid.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 9 as libc::c_int as byte_hack,
                        cost: 9 as libc::c_int as byte_hack,
                        stat: 3 as libc::c_int as byte_hack,
                        diff: 15 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"fire breath\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can breath fire.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to breathe fire.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to breathe fire.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 20 as libc::c_int as byte_hack,
                        cost: 10 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 18 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"hypnotic gaze\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"Your gaze is hypnotic.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"Your eyes look mesmerising...\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"Your eyes look uninteresting.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        level: 12 as libc::c_int as byte_hack,
                        cost: 12 as libc::c_int as byte_hack,
                        stat: 5 as libc::c_int as byte_hack,
                        diff: 18 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"telekinesis\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You are telekinetic.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to move objects telekinetically.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to move objects telekinetically.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 9 as libc::c_int as byte_hack,
                        cost: 9 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 14 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"teleport\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can teleport at will.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the power of teleportation at will.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the power of teleportation at will.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 7 as libc::c_int as byte_hack,
                        cost: 7 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 15 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"mind blast\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can mind blast your enemies.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You gain the power of Mind Blast.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the power of Mind Blast.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 5 as libc::c_int as byte_hack,
                        cost: 3 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 15 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"emit radiation\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can emit hard radiation at will.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You start emitting hard radiation.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You stop emitting hard radiation.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 15 as libc::c_int as byte_hack,
                        cost: 15 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 14 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"vampiric drain\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can drain life from a foe.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You become vampiric.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"You are no longer vampiric.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        level: 4 as libc::c_int as byte_hack,
                        cost: 5 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 9 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"smell metal\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can smell nearby precious metal.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You smell a metallic odour.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"You no longer smell a metallic odour.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 3 as libc::c_int as byte_hack,
                        cost: 2 as libc::c_int as byte_hack,
                        stat: 1 as libc::c_int as byte_hack,
                        diff: 12 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"smell monsters\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can smell nearby monsters.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You smell filthy monsters.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"You no longer smell filthy monsters.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 5 as libc::c_int as byte_hack,
                        cost: 4 as libc::c_int as byte_hack,
                        stat: 1 as libc::c_int as byte_hack,
                        diff: 15 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"blink\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        desc_text:
                            b"You can teleport yourself short distances.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You gain the power of minor teleportation.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the power of minor teleportation.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 3 as libc::c_int as byte_hack,
                        cost: 3 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 12 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"eat rock\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can consume solid rock.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"The walls look delicious.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"The walls look unappetising.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        level: 8 as libc::c_int as byte_hack,
                        cost: 12 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 18 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"swap position\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can switch locations with another being.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You feel like walking a mile in someone else\'s shoes.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You feel like staying in your own shoes.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 15 as libc::c_int as byte_hack,
                        cost: 12 as libc::c_int as byte_hack,
                        stat: 3 as libc::c_int as byte_hack,
                        diff: 16 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"shriek\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        desc_text:
                            b"You can emit a horrible shriek.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"Your vocal cords get much tougher.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"Your vocal cords get much weaker.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 4 as libc::c_int as byte_hack,
                        cost: 4 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 6 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"illuminate\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can emit bright light.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You can light up rooms with your presence.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You can no longer light up rooms with your presence.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 3 as libc::c_int as byte_hack,
                        cost: 2 as libc::c_int as byte_hack,
                        stat: 1 as libc::c_int as byte_hack,
                        diff: 10 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"detect curses\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can feel the danger of evil magic.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You can feel evil magic.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"You can no longer feel evil magic.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 7 as libc::c_int as byte_hack,
                        cost: 14 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 14 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"berserk\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        desc_text:
                            b"You can drive yourself into a berserk frenzy.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You feel a controlled rage.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"You no longer feel a controlled rage.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 8 as libc::c_int as byte_hack,
                        cost: 8 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 14 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"polymorph\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can polymorph yourself at will.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"Your body seems mutable.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"Your body seems stable.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        level: 18 as libc::c_int as byte_hack,
                        cost: 20 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 18 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"Midas touch\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can turn ordinary items to gold.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You gain the Midas touch.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"You lose the Midas touch.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        level: 10 as libc::c_int as byte_hack,
                        cost: 5 as libc::c_int as byte_hack,
                        stat: 1 as libc::c_int as byte_hack,
                        diff: 12 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"grow mold\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can cause mold to grow near you.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You feel a sudden affinity for mold.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You feel a sudden dislike for mold.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 1 as libc::c_int as byte_hack,
                        cost: 6 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 14 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"resist elements\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can harden yourself to the ravages of the elements.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You feel like you can protect yourself.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You feel like you might be vulnerable.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 10 as libc::c_int as byte_hack,
                        cost: 12 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 12 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"earthquake\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can bring down the dungeon around your ears.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to wreck the dungeon.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to wreck the dungeon.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 12 as libc::c_int as byte_hack,
                        cost: 12 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 16 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"eat magic\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can consume magic energy for your own use.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"Your magic items look delicious.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"Your magic items no longer look delicious.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 17 as libc::c_int as byte_hack,
                        cost: 1 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 15 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"weigh magic\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can feel the strength of the magics affecting you.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You feel you can better understand the magic around you.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You no longer sense magic.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        level: 6 as libc::c_int as byte_hack,
                        cost: 6 as libc::c_int as byte_hack,
                        stat: 1 as libc::c_int as byte_hack,
                        diff: 10 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"sterilise\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can cause mass impotence.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You can give everything around you a headache.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You hear a massed sigh of relief.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 20 as libc::c_int as byte_hack,
                        cost: 40 as libc::c_int as byte_hack,
                        stat: 5 as libc::c_int as byte_hack,
                        diff: 18 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"panic hit\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can run for your life after hitting something.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You suddenly understand how thieves feel.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You no longer feel jumpy.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        level: 10 as libc::c_int as byte_hack,
                        cost: 12 as libc::c_int as byte_hack,
                        stat: 3 as libc::c_int as byte_hack,
                        diff: 14 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"dazzle\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        desc_text:
                            b"You can emit confusing, blinding radiation.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to emit dazzling lights.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to emit dazzling lights.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 7 as libc::c_int as byte_hack,
                        cost: 15 as libc::c_int as byte_hack,
                        stat: 5 as libc::c_int as byte_hack,
                        diff: 8 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"spear of darkness\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can create a spear of darkness.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"An illusory spear of darkness appears in your hand.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"The spear of darkness disappear.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 7 as libc::c_int as byte_hack,
                        cost: 10 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 9 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"recall\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        desc_text:
                            b"You can travel between towns and the depths.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You feel briefly homesick, but it passes.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You feel briefly homesick.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        level: 17 as libc::c_int as byte_hack,
                        cost: 50 as libc::c_int as byte_hack,
                        stat: 1 as libc::c_int as byte_hack,
                        diff: 16 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"banish evil\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can send evil creatures directly to the Nether Realm.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You feel a holy wrath fill you.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You no longer feel a holy wrath.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 25 as libc::c_int as byte_hack,
                        cost: 25 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 18 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"cold touch\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can freeze things with a touch.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"Your hands get very cold.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"Your hands warm up.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        level: 2 as libc::c_int as byte_hack,
                        cost: 2 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 11 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"throw object\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can hurl objects with great force.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"Your throwing arm feels much stronger.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"Your throwing arm feels much weaker.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 1 as libc::c_int as byte_hack,
                        cost: 10 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 6 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"find secret passages\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can use secret passages.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You suddenly notice lots of hidden ways.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You no longer can use hidden ways.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 15 as libc::c_int as byte_hack,
                        cost: 15 as libc::c_int as byte_hack,
                        stat: 3 as libc::c_int as byte_hack,
                        diff: 12 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"detect doors and traps\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can detect hidden doors and traps.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You develop an affinity for traps.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You no longer can detect hidden doors and traps.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 5 as libc::c_int as byte_hack,
                        cost: 3 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 10 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"create food\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can create food.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"Your cooking skills greatly improve.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"Your cooking skills return to a normal level.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 15 as libc::c_int as byte_hack,
                        cost: 10 as libc::c_int as byte_hack,
                        stat: 1 as libc::c_int as byte_hack,
                        diff: 10 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"remove fear\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can embolden yourself.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You feel your fears lessening.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"You feel your fears growing again.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 3 as libc::c_int as byte_hack,
                        cost: 5 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 8 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"set explosive rune\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can set explosive runes.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You suddenly understand how explosive runes work.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You suddenly forget how explosive runes work.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 25 as libc::c_int as byte_hack,
                        cost: 35 as libc::c_int as byte_hack,
                        stat: 1 as libc::c_int as byte_hack,
                        diff: 15 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"stone to mud\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can destroy walls.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You can destroy walls.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        lose_text:
                            b"You cannot destroy walls anymore.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 20 as libc::c_int as byte_hack,
                        cost: 10 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 12 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"poison dart\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can throw poisoned darts.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You get an infinite supply of poisoned darts.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose your infinite supply of poisoned darts.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 12 as libc::c_int as byte_hack,
                        cost: 8 as libc::c_int as byte_hack,
                        stat: 3 as libc::c_int as byte_hack,
                        diff: 14 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"magic missile\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can cast magic missiles.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You suddenly understand the basics of magic.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You forget the basics of magic.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 2 as libc::c_int as byte_hack,
                        cost: 2 as libc::c_int as byte_hack,
                        stat: 1 as libc::c_int as byte_hack,
                        diff: 9 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"grow trees\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can grow trees.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You feel an affinity for trees.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You no longer feel an affinity for trees.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 2 as libc::c_int as byte_hack,
                        cost: 6 as libc::c_int as byte_hack,
                        stat: 5 as libc::c_int as byte_hack,
                        diff: 3 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"cold breath\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can breath cold.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to breathe cold.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to breathe cold.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 20 as libc::c_int as byte_hack,
                        cost: 10 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 18 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"chaos breath\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can breath chaos.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to breathe chaos.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to breathe chaos.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 20 as libc::c_int as byte_hack,
                        cost: 10 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 18 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"elemental breath\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can breath the elements.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to breathe the elements.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to breathe the elements.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 20 as libc::c_int as byte_hack,
                        cost: 10 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 18 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"change the world\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can wreck the world around you.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to wreck the world.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to wreck the world.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 1 as libc::c_int as byte_hack,
                        cost: 30 as libc::c_int as byte_hack,
                        stat: 5 as libc::c_int as byte_hack,
                        diff: 6 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"scare monster\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can scare monsters.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to scare monsters.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to scare monsters.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 4 as libc::c_int as byte_hack,
                        cost: 3 as libc::c_int as byte_hack,
                        stat: 1 as libc::c_int as byte_hack,
                        diff: 3 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"restore life\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can restore lost life forces.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to restore your life force.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to restore your life force.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 30 as libc::c_int as byte_hack,
                        cost: 30 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 18 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"summon monsters\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can call upon monsters.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to call upon monsters.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to call upon monsters.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 0 as libc::c_int as byte_hack,
                        cost: 0 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 0 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"necromantic powers\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can use the foul necromantic magic.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to use the foul necromantic magic.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to use the foul necromantic magic.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 0 as libc::c_int as byte_hack,
                        cost: 0 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 0 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"Rohan Knight\'s Powers\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can use rohir powers.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to use rohir powers.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to use rohir powers.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 0 as libc::c_int as byte_hack,
                        cost: 0 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 0 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"Thunderlord\'s Powers\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can use thunderlords powers.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to use thunderlords powers.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to use thunderlords powers.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 0 as libc::c_int as byte_hack,
                        cost: 0 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 0 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"Death Mold\'s Powers\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can use the foul deathmold magic.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to use the foul deathmold magic.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to use the foul deathmold magic.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 0 as libc::c_int as byte_hack,
                        cost: 0 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 0 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"Hypnotise Pet\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can mystify pets.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to mystify pets.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to mystify pets.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 0 as libc::c_int as byte_hack,
                        cost: 0 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 0 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"Awaken Hypnotised Pet\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can wake up a pet.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You gain the ability to wake up a pet.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You lose the ability to wake up a pet.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 0 as libc::c_int as byte_hack,
                        cost: 0 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 0 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"Incarnate\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can incarnate into a body.\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You feel the need to get a body.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You no longer feel the need for a new body.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 0 as libc::c_int as byte_hack,
                        cost: 0 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 0 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"magic map\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can sense what is beyond walls.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You feel you can sense what is beyond walls.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You no longer can sense what is beyond walls.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 7 as libc::c_int as byte_hack,
                        cost: 10 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 15 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"lay trap\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can lay monster traps.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You suddenly understand how rogues work.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You no longer understand how rogues work.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 1 as libc::c_int as byte_hack,
                        cost: 1 as libc::c_int as byte_hack,
                        stat: 3 as libc::c_int as byte_hack,
                        diff: 1 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"Merchant abilities\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can request items and get loans.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"From now on you can use the merchant abilities.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You can no longer use the merchant abilities.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 0 as libc::c_int as byte_hack,
                        cost: 0 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 0 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"turn pet into companion\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can turn a pet into a companion.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You suddenly gain authority over your pets.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You can no longer convert pets into companions.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 2 as libc::c_int as byte_hack,
                        cost: 10 as libc::c_int as byte_hack,
                        stat: 5 as libc::c_int as byte_hack,
                        diff: 10 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"turn into a bear\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can turn into a bear.\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        gain_text:
                            b"You suddenly gain beorning powers.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You can no longer shapeshift into a bear.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 2 as libc::c_int as byte_hack,
                        cost: 5 as libc::c_int as byte_hack,
                        stat: 4 as libc::c_int as byte_hack,
                        diff: 5 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"sense dodge success\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can sense your dodging success chance.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You suddenly can sense your dodging success chance.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You can no longer sense your dodging success chance.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 0 as libc::c_int as byte_hack,
                        cost: 0 as libc::c_int as byte_hack,
                        stat: 0 as libc::c_int as byte_hack,
                        diff: 0 as libc::c_int as byte_hack,};
         init
     },
     {
         let mut init =
             power_type{name:
                            b"turn into a Balrog\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                        desc_text:
                            b"You can turn into a Balrog at will.\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        gain_text:
                            b"You feel the fire of Udun burning in you.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        lose_text:
                            b"You no longer feel the fire of Udun in you.\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                        level: 35 as libc::c_int as byte_hack,
                        cost: 80 as libc::c_int as byte_hack,
                        stat: 2 as libc::c_int as byte_hack,
                        diff: 25 as libc::c_int as byte_hack,};
         init
     }];
/*
 * The Quests
 */
// Initialized in run_static_initializers
#[no_mangle]
pub static mut quest_init_tome: [quest_type; 26] =
    [quest_type{silent: 0,
                dynamic_desc: 0,
                name: [0; 40],
                desc: [[0; 80]; 10],
                status: 0,
                level: 0,
                plot: 0 as *mut s16b,
                type_0: 0,
                init: None,
                data: [0; 4],
                gen_desc: None,}; 26];
/* List of powers for Symbiants/Powers */
#[no_mangle]
pub static mut monster_powers: [monster_power; 96] =
    [{
         let mut init =
             monster_power{power: 0x1 as libc::c_int as u32b,
                           name:
                               b"Aggravate Monster\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 1 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x2 as libc::c_int as u32b,
                           name:
                               b"Multiply\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 10 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x4 as libc::c_int as u32b,
                           name:
                               b"Summon Animal\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x8 as libc::c_int as u32b,
                           name:
                               b"Fire a Rocket\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 40 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x10 as libc::c_int as u32b,
                           name:
                               b"Light Arrow\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 1 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x20 as libc::c_int as u32b,
                           name:
                               b"Minor Arrow\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 3 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x40 as libc::c_int as u32b,
                           name:
                               b"Major Arrow\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 7 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x80 as libc::c_int as u32b,
                           name:
                               b"Great Arrow\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 9 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x100 as libc::c_int as u32b,
                           name:
                               b"Breathe Acid\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 10 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x200 as libc::c_int as u32b,
                           name:
                               b"Breathe Lightning\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 10 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x400 as libc::c_int as u32b,
                           name:
                               b"Breathe Fire\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 10 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x800 as libc::c_int as u32b,
                           name:
                               b"Breathe Cold\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 10 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x1000 as libc::c_int as u32b,
                           name:
                               b"Breathe Poison\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 15 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x2000 as libc::c_int as u32b,
                           name:
                               b"Breathe Nether\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x4000 as libc::c_int as u32b,
                           name:
                               b"Breathe Light\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x8000 as libc::c_int as u32b,
                           name:
                               b"Breathe Dark\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x10000 as libc::c_int as u32b,
                           name:
                               b"Breathe Confusion\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 15 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x20000 as libc::c_int as u32b,
                           name:
                               b"Breathe Sound\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x40000 as libc::c_int as u32b,
                           name:
                               b"Breathe Chaos\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x80000 as libc::c_int as u32b,
                           name:
                               b"Breathe Disenchantment\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x100000 as libc::c_int as u32b,
                           name:
                               b"Breathe Nexus\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x200000 as libc::c_int as u32b,
                           name:
                               b"Breathe Time\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x400000 as libc::c_int as u32b,
                           name:
                               b"Breathe Inertia\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x800000 as libc::c_int as u32b,
                           name:
                               b"Breathe Gravity\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x1000000 as libc::c_int as u32b,
                           name:
                               b"Breathe Shards\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x2000000 as libc::c_int as u32b,
                           name:
                               b"Breathe Plasma\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x4000000 as libc::c_int as u32b,
                           name:
                               b"Breathe Force\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x8000000 as libc::c_int as u32b,
                           name:
                               b"Breathe Mana\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 40 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x10000000 as libc::c_int as u32b,
                           name:
                               b"Nuke Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x20000000 as libc::c_int as u32b,
                           name:
                               b"Breathe Nuke\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 40 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x40000000 as libc::c_int as u32b,
                           name:
                               b"Chaos Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x80000000 as libc::c_uint,
                           name:
                               b"Breathe Disintegration\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 40 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x1 as libc::c_int as u32b,
                           name:
                               b"Acid Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 8 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x2 as libc::c_int as u32b,
                           name:
                               b"Lightning Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 8 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x4 as libc::c_int as u32b,
                           name:
                               b"Fire Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 8 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x8 as libc::c_int as u32b,
                           name:
                               b"Cold Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 8 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x10 as libc::c_int as u32b,
                           name:
                               b"Poison Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x20 as libc::c_int as u32b,
                           name:
                               b"Nether Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x40 as libc::c_int as u32b,
                           name:
                               b"Water Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x80 as libc::c_int as u32b,
                           name:
                               b"Mana Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 50 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x100 as libc::c_int as u32b,
                           name:
                               b"Darkness Ball\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0 as libc::c_int as u32b,
                           name:
                               b"(none)\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 0 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0 as libc::c_int as u32b,
                           name:
                               b"(none)\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 0 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0 as libc::c_int as u32b,
                           name:
                               b"(none)\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 0 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x1000 as libc::c_int as u32b,
                           name:
                               b"Cause Light Wounds\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x2000 as libc::c_int as u32b,
                           name:
                               b"Cause Medium Wounds\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x4000 as libc::c_int as u32b,
                           name:
                               b"Cause Critical Wounds\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 35 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x8000 as libc::c_int as u32b,
                           name:
                               b"Cause Mortal Wounds\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 45 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x10000 as libc::c_int as u32b,
                           name:
                               b"Acid Bolt\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 5 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x20000 as libc::c_int as u32b,
                           name:
                               b"Lightning Bolt\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 5 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x40000 as libc::c_int as u32b,
                           name:
                               b"Fire Bolt\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 5 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x80000 as libc::c_int as u32b,
                           name:
                               b"Cold Bolt\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 5 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x100000 as libc::c_int as u32b,
                           name:
                               b"Poison Bolt\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 10 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x200000 as libc::c_int as u32b,
                           name:
                               b"Nether Bolt\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 15 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x400000 as libc::c_int as u32b,
                           name:
                               b"Water Bolt\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x800000 as libc::c_int as u32b,
                           name:
                               b"Mana Bolt\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 25 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x1000000 as libc::c_int as u32b,
                           name:
                               b"Plasma Bolt\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x2000000 as libc::c_int as u32b,
                           name:
                               b"Ice Bolt\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x4000000 as libc::c_int as u32b,
                           name:
                               b"Magic Missile\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 1 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x8000000 as libc::c_int as u32b,
                           name:
                               b"Scare\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 4 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x10000000 as libc::c_int as u32b,
                           name:
                               b"Blindness\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 6 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x20000000 as libc::c_int as u32b,
                           name:
                               b"Confusion\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 7 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x40000000 as libc::c_int as u32b,
                           name:
                               b"Slowness\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 10 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x80000000 as libc::c_uint,
                           name:
                               b"Paralyse\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 10 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x1 as libc::c_int as u32b,
                           name:
                               b"Haste Self\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 50 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x2 as libc::c_int as u32b,
                           name:
                               b"Hand of Doom\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x4 as libc::c_int as u32b,
                           name:
                               b"Healing\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 60 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x8 as libc::c_int as u32b,
                           name:
                               b"Summon Animals\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 60 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x10 as libc::c_int as u32b,
                           name:
                               b"Phase Door\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 2 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x20 as libc::c_int as u32b,
                           name:
                               b"Teleport\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 10 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x40 as libc::c_int as u32b,
                           name:
                               b"Teleport To\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x80 as libc::c_int as u32b,
                           name:
                               b"Teleport Away\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x100 as libc::c_int as u32b,
                           name:
                               b"Teleport Level\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 20 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x200 as libc::c_int as u32b,
                           name:
                               b"Darkness\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 3 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x400 as libc::c_int as u32b,
                           name:
                               b"Create Traps\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 10 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0 as libc::c_int as u32b,
                           name:
                               b"(none)\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 0 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x1000 as libc::c_int as u32b,
                           name:
                               b"Raise the Dead\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 400 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0 as libc::c_int as u32b,
                           name:
                               b"(none)\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 0 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0 as libc::c_int as u32b,
                           name:
                               b"(none)\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 0 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x8000 as libc::c_int as u32b,
                           name:
                               b"Summon Thunderlords\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 90 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x10000 as libc::c_int as u32b,
                           name:
                               b"Summon Kin\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 80 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x20000 as libc::c_int as u32b,
                           name:
                               b"Summon Greater Demons\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 90 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x40000 as libc::c_int as u32b,
                           name:
                               b"Summon Monster\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 50 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x80000 as libc::c_int as u32b,
                           name:
                               b"Summon Monsters\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 60 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x100000 as libc::c_int as u32b,
                           name:
                               b"Summon Ants\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x200000 as libc::c_int as u32b,
                           name:
                               b"Summon Spider\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 30 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x400000 as libc::c_int as u32b,
                           name:
                               b"Summon Hound\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 50 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x800000 as libc::c_int as u32b,
                           name:
                               b"Summon Hydra\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 40 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x1000000 as libc::c_int as u32b,
                           name:
                               b"Summon Angel\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 60 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x2000000 as libc::c_int as u32b,
                           name:
                               b"Summon Demon\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 60 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x4000000 as libc::c_int as u32b,
                           name:
                               b"Summon Undead\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 70 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x8000000 as libc::c_int as u32b,
                           name:
                               b"Summon Dragon\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 70 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x10000000 as libc::c_int as u32b,
                           name:
                               b"Summon High Undead\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 90 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x20000000 as libc::c_int as u32b,
                           name:
                               b"Summon High Dragon\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 90 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0x40000000 as libc::c_int as u32b,
                           name:
                               b"Summon Wraith\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 90 as libc::c_int,
                           great: 1 as libc::c_int as bool_,};
         init
     },
     {
         let mut init =
             monster_power{power: 0 as libc::c_int as u32b,
                           name:
                               b"(none)\x00" as *const u8 as
                                   *const libc::c_char,
                           mana: 0 as libc::c_int,
                           great: 0 as libc::c_int as bool_,};
         init
     }];
/* Tval descriptions */
#[no_mangle]
pub static mut tval_descs: [tval_desc; 53] =
    [{
         let mut init =
             tval_desc{tval: 4 as libc::c_int,
                       desc:
                           b"Essences contain primitive magic forces which users of the Alchemy skill can use to create powerful magic items from other magic items.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 6 as libc::c_int,
                       desc:
                           b"Mage Staves are the spellcaster\'s weapons of choice.  They all reduce spellcasting time to 80% of normal time and some will yield even greater powers.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 3 as libc::c_int,
                       desc: b"XXX\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 8 as libc::c_int,
                       desc:
                           b"Parchments can contain useful information ... or useless junk.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 10 as libc::c_int,
                       desc:
                           b"Eggs are laid by some monsters.  If they hatch in your inventory the monster will be your friend.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 12 as libc::c_int,
                       desc:
                           b"Tools can be digging implements, climbing equipment and such. They have their own slot in your inventory.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 14 as libc::c_int,
                       desc:
                           b"Musical instruments can be used with the Music skill to play magical songs. Some of them can also be activated.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 15 as libc::c_int,
                       desc:
                           b"Boomerangs can be used instead of bows or slings.  They are more like melee weapons than bows.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 16 as libc::c_int,
                       desc:
                           b"Shots are small, hard balls.  They are the standard ammunition for slings.  You can carry them in your quiver if you have a sling equipped.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 17 as libc::c_int,
                       desc:
                           b"Arrows are the standard ammunition for bows.  You can carry them in your quiver if you have a bow equipped.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 18 as libc::c_int,
                       desc:
                           b"Bolts are the standard ammunition for crossbows.  You can carry them in your quiver if you have a crossbow equipped.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 19 as libc::c_int,
                       desc:
                           b"Slings, bows and crossbows are used to attack monsters from a distance.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 20 as libc::c_int,
                       desc:
                           b"Tools can be digging implements, climbing equipment and such.  They have their own slot in your inventory.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 21 as libc::c_int,
                       desc:
                           b"Hafted weapons are melee weapons.  Eru followers can use them without penalties.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 23 as libc::c_int,
                       desc:
                           b"Swords are melee weapons.\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 24 as libc::c_int,
                       desc:
                           b"Axes are melee weapons.\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 22 as libc::c_int,
                       desc:
                           b"Polearms are melee weapons.\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 38 as libc::c_int,
                       desc:
                           b"Dragon armour is made from the scales of dead dragons. These mighty sets of armour usually yield great power to their wearer.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 39 as libc::c_int,
                       desc:
                           b"Lights allow you to read things and see from afar. Some of them need to be fueled but some do not.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 40 as libc::c_int,
                       desc:
                           b"Amulets are fine pieces of jewelry, usually imbued with arcane magics.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 45 as libc::c_int,
                       desc:
                           b"Rings are fine pieces of jewelry, usually imbued with arcane magics.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 46 as libc::c_int,
                       desc:
                           b"Trapping kits are used with the trapping ability to set deadly monster traps.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 55 as libc::c_int,
                       desc:
                           b"Staves are objects imbued with mystical powers.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 65 as libc::c_int,
                       desc:
                           b"Wands are like small staves and usually have a targeted effect.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 66 as libc::c_int,
                       desc:
                           b"Rod tips are the physical bindings of powerful spells.  Zap (attach) them to a rod to get a fully functional rod. Each spell takes some mana from the rod it is attached to to work.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 67 as libc::c_int,
                       desc:
                           b"Rods contain mana reserves used to cast spells in rod tips.  Zap (attach) a rod tip to them to get a fully functional rod. Each spell takes some mana from the rod it is attached to to work.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 70 as libc::c_int,
                       desc:
                           b"Scrolls are magical parchments imbued with magic spells. Some are good, some...are not.  When a scroll is read, its magic is released and the scroll is destroyed.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 71 as libc::c_int,
                       desc:
                           b"Potions are magical liquids.  Some of them are beneficial...some not.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 72 as libc::c_int,
                       desc:
                           b"Potions are magical liquids.  Some of them are beneficial...some not.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 77 as libc::c_int,
                       desc:
                           b"Flasks of oil can be used to refill lanterns.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 80 as libc::c_int,
                       desc:
                           b"Everybody needs to eat, even you.\x00" as
                               *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 99 as libc::c_int,
                       desc:
                           b"This monster seems to be hypnotised and friendly.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 102 as libc::c_int,
                       desc:
                           b"Those objects are only known of by rumours.  It is said that they can be activated for great or strange effects...\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 104 as libc::c_int,
                       desc:
                           b"Runes are used with the Runecraft skill to create brand new spells.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 105 as libc::c_int,
                       desc:
                           b"Runes are used with the Runecraft skill to create brand new spells.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 11 as libc::c_int,
                       desc:
                           b"Junk is usually worthless, though experienced archers can create ammo with them.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 1 as libc::c_int,
                       desc:
                           b"It looks dead...\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 2 as libc::c_int,
                       desc:
                           b"An empty bottle. Maybe an alchemist could refill it.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 5 as libc::c_int,
                       desc:
                           b"Spikes can be used to jam doors.\x00" as
                               *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 9 as libc::c_int,
                       desc:
                           b"It looks dead...\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 30 as libc::c_int,
                       desc:
                           b"Boots can help your armour rating.  Some of these are magical.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 31 as libc::c_int,
                       desc:
                           b"Handgear is used to protect hands, but nonmagical ones can sometimes hinder spellcasting.  Alchemists need gloves in order to do alchemy.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 32 as libc::c_int,
                       desc:
                           b"Headgear will protect your head.\x00" as
                               *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 33 as libc::c_int,
                       desc:
                           b"Headgear will protect your head.\x00" as
                               *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 34 as libc::c_int,
                       desc:
                           b"Shields will help improve your defence rating, but you cannot use them with two handed weapons.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 35 as libc::c_int,
                       desc:
                           b"Cloaks can shield you from damage.  Sometimes they also provide magical powers.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 36 as libc::c_int,
                       desc:
                           b"Soft armour is light, and will not hinder your combat much.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 37 as libc::c_int,
                       desc:
                           b"Hard armour provides much more protection than soft armour but also hinders combat much more.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 112 as libc::c_int,
                       desc:
                           b"This mystical book is used by symbiants to extend their symbiosis.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 113 as libc::c_int,
                       desc:
                           b"This song book is used by bards to play songs.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 114 as libc::c_int,
                       desc:
                           b"This mystical book is used by druids to call upon the powers of nature.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 115 as libc::c_int,
                       desc:
                           b"This unholy demon equipment is used with the Demonology skill to control the school of demon power.\x00"
                               as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc{tval: 0 as libc::c_int,
                       desc: b"\x00" as *const u8 as *const libc::c_char,};
         init
     }];
/*
 * List of the between exits
 *       s16b corresp;           Corresponding between gate
 *       bool_ dungeon;           Do we exit in a dungeon or in the wild ?
 *
 *       s16b wild_x, wild_y;    Wilderness spot to land onto
 *       s16b p_ptr->px, p_ptr->py;            Location of the map
 *
 *       s16b d_idx;             Dungeon to land onto
 *       s16b level;
 */
#[no_mangle]
pub static mut between_exits: [between_exit; 2] =
    [{
         let mut init =
             between_exit{corresp: 1 as libc::c_int as s16b,
                          dungeon: 0 as libc::c_int as bool_,
                          wild_x: 49 as libc::c_int as s16b,
                          wild_y: 11 as libc::c_int as s16b,
                          px: 119 as libc::c_int as s16b,
                          py: 25 as libc::c_int as s16b,
                          d_idx: 0 as libc::c_int as s16b,
                          level: 0 as libc::c_int as s16b,};
         init
     },
     {
         let mut init =
             between_exit{corresp: 0 as libc::c_int as s16b,
                          dungeon: 0 as libc::c_int as bool_,
                          wild_x: 60 as libc::c_int as s16b,
                          wild_y: 56 as libc::c_int as s16b,
                          px: 10 as libc::c_int as s16b,
                          py: 35 as libc::c_int as s16b,
                          d_idx: 0 as libc::c_int as s16b,
                          level: 0 as libc::c_int as s16b,};
         init
     }];
/*
 * Months
 */
#[no_mangle]
pub static mut month_day: [libc::c_int; 9] =
    [0 as libc::c_int, 1 as libc::c_int, 55 as libc::c_int,
     127 as libc::c_int, 181 as libc::c_int, 184 as libc::c_int,
     238 as libc::c_int, 310 as libc::c_int, 364 as libc::c_int];
#[no_mangle]
pub static mut month_name: [cptr; 9] =
    [b"Yestare\x00" as *const u8 as *const libc::c_char,
     b"Tuile\x00" as *const u8 as *const libc::c_char,
     b"Laire\x00" as *const u8 as *const libc::c_char,
     b"Yavie\x00" as *const u8 as *const libc::c_char,
     b"Enderi\x00" as *const u8 as *const libc::c_char,
     b"Quelle\x00" as *const u8 as *const libc::c_char,
     b"Hrive\x00" as *const u8 as *const libc::c_char,
     b"Coire\x00" as *const u8 as *const libc::c_char,
     b"Mettare\x00" as *const u8 as *const libc::c_char];
/*
 * max body parts
 */
#[no_mangle]
pub static mut max_body_part: [libc::c_int; 6] =
    [3 as libc::c_int, 1 as libc::c_int, 3 as libc::c_int, 6 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int];
/*
 * Description of GF_FOO
 */
#[no_mangle]
pub static mut gf_names: [gf_name_type; 93] =
    [{
         let mut init =
             gf_name_type{gf: 1 as libc::c_int,
                          name:
                              b"electricity\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 2 as libc::c_int,
                          name:
                              b"poison\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 3 as libc::c_int,
                          name:
                              b"acid\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 4 as libc::c_int,
                          name:
                              b"cold\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 5 as libc::c_int,
                          name:
                              b"fire\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 6 as libc::c_int,
                          name:
                              b"asphyxiating gas\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 7 as libc::c_int,
                          name:
                              b"corpse explosion\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 10 as libc::c_int,
                          name:
                              b"missile\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 11 as libc::c_int,
                          name:
                              b"arrow\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 12 as libc::c_int,
                          name:
                              b"plasma\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 13 as libc::c_int,
                          name:
                              b"a tidal wave\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 14 as libc::c_int,
                          name:
                              b"water\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 15 as libc::c_int,
                          name:
                              b"light\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 16 as libc::c_int,
                          name:
                              b"darkness\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 17 as libc::c_int,
                          name:
                              b"weak light\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 18 as libc::c_int,
                          name:
                              b"weak darkness\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 20 as libc::c_int,
                          name:
                              b"shards\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 21 as libc::c_int,
                          name:
                              b"sound\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 22 as libc::c_int,
                          name:
                              b"confusion\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 23 as libc::c_int,
                          name:
                              b"force\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 24 as libc::c_int,
                          name:
                              b"inertia\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 26 as libc::c_int,
                          name:
                              b"pure mana\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 27 as libc::c_int,
                          name:
                              b"meteor\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 28 as libc::c_int,
                          name:
                              b"ice\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 30 as libc::c_int,
                          name:
                              b"chaos\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 31 as libc::c_int,
                          name:
                              b"nether\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 32 as libc::c_int,
                          name:
                              b"disenchantment\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 33 as libc::c_int,
                          name:
                              b"nexus\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 34 as libc::c_int,
                          name:
                              b"time\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 35 as libc::c_int,
                          name:
                              b"gravity\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 40 as libc::c_int,
                          name:
                              b"wall destruction\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 41 as libc::c_int,
                          name:
                              b"door destruction\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 42 as libc::c_int,
                          name:
                              b"trap destruction\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 45 as libc::c_int,
                          name:
                              b"wall creation\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 46 as libc::c_int,
                          name:
                              b"door creation\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 47 as libc::c_int,
                          name:
                              b"trap creation\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 51 as libc::c_int,
                          name:
                              b"clone\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 52 as libc::c_int,
                          name:
                              b"polymorph\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 53 as libc::c_int,
                          name:
                              b"healing\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 54 as libc::c_int,
                          name:
                              b"speed\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 55 as libc::c_int,
                          name:
                              b"slowness\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 56 as libc::c_int,
                          name:
                              b"confusion\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 57 as libc::c_int,
                          name:
                              b"sleep\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 58 as libc::c_int,
                          name:
                              b"drain life\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 61 as libc::c_int,
                          name:
                              b"teleport away undead\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 62 as libc::c_int,
                          name:
                              b"teleport away evil\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 63 as libc::c_int,
                          name:
                              b"teleport away\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 64 as libc::c_int,
                          name:
                              b"scare undead\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 65 as libc::c_int,
                          name:
                              b"scare evil\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 66 as libc::c_int,
                          name:
                              b"scare\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 67 as libc::c_int,
                          name:
                              b"dispel undead\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 68 as libc::c_int,
                          name:
                              b"dispel evil\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 69 as libc::c_int,
                          name:
                              b"dispel\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 70 as libc::c_int,
                          name:
                              b"dispel demons\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 71 as libc::c_int,
                          name:
                              b"dispel living creatures\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 72 as libc::c_int,
                          name:
                              b"rocket\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 73 as libc::c_int,
                          name:
                              b"nuke\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 74 as libc::c_int,
                          name:
                              b"glyph creation\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 75 as libc::c_int,
                          name:
                              b"stasis\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 76 as libc::c_int,
                          name:
                              b"stone wall creation\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 77 as libc::c_int,
                          name:
                              b"death ray\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 78 as libc::c_int,
                          name:
                              b"stunning\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 79 as libc::c_int,
                          name:
                              b"holy fire\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 80 as libc::c_int,
                          name:
                              b"hellfire\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 81 as libc::c_int,
                          name:
                              b"disintegration\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 82 as libc::c_int,
                          name:
                              b"charming\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 83 as libc::c_int,
                          name:
                              b"undead control\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 84 as libc::c_int,
                          name:
                              b"animal control\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 85 as libc::c_int,
                          name:
                              b"psionic energy\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 86 as libc::c_int,
                          name:
                              b"psionic drain\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 87 as libc::c_int,
                          name:
                              b"telekinesis\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 88 as libc::c_int,
                          name:
                              b"door jamming\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 89 as libc::c_int,
                          name:
                              b"domination\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 90 as libc::c_int,
                          name:
                              b"dispel good\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 91 as libc::c_int,
                          name:
                              b"identification\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 92 as libc::c_int,
                          name:
                              b"raise dead\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 93 as libc::c_int,
                          name:
                              b"*identification*\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 94 as libc::c_int,
                          name:
                              b"destruction\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 95 as libc::c_int,
                          name:
                              b"stunning and confusion\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 96 as libc::c_int,
                          name:
                              b"stunning and damage\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 98 as libc::c_int,
                          name:
                              b"confusion and damage\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 99 as libc::c_int,
                          name:
                              b"*charming*\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 100 as libc::c_int,
                          name:
                              b"implosion\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 101 as libc::c_int,
                          name:
                              b"lava\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 102 as libc::c_int,
                          name:
                              b"fear\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 103 as libc::c_int,
                          name:
                              b"jumpgate creation\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 104 as libc::c_int,
                          name: b"\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 105 as libc::c_int,
                          name:
                              b"death\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 106 as libc::c_int,
                          name:
                              b"control demon\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 107 as libc::c_int,
                          name:
                              b"raise demon\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 108 as libc::c_int,
                          name:
                              b"*control demon*\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: 109 as libc::c_int,
                          name:
                              b"projected melee attacks\x00" as *const u8 as
                                  *const libc::c_char,};
         init
     },
     {
         let mut init =
             gf_name_type{gf: -(1 as libc::c_int), name: 0 as cptr,};
         init
     }];
unsafe extern "C" fn run_static_initializers() {
    quest_init_tome =
        [{
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 0 as libc::c_int as s16b,
                            plot: 0 as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_null_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Dol Guldur\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"The forest of Mirkwood is a very dangerous place to go, mainly due to\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"the activities of the Necromancer that lurks in Dol Guldur.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Find him, and free Mirkwood from his spells.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 1 as libc::c_int as s16b,
                            level: 70 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_necro_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Sauron\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"It is time to take the battle to Morgoth. But, before you can\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"reach it, you must find and kill Sauron.  Only after defeating\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"this powerful sorcerer will the stairs leading to Morgoth\'s\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"room be opened.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 99 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_sauron_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Morgoth\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Your final quest is the ultimate quest that has always been\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"required of you. You must enter the fetid depths of Angband, where\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Morgoth is waiting. Travel deep, and defeat this source of all our\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"problems.  Be prepared, be patient, and good luck. May the light\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"shine on you.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 100 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_morgoth_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Thieves!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"There are thieves robbing my people! They live in a small\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"burrow outside the city walls, but they get inside the walls\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"with a tunnel to a building here! Your task is to go into\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"the building and kill these ruffians.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 5 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_thieves_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 1 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Random Quest\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 5 as libc::c_int as s16b,
                            plot: 0 as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_random_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc:
                                Some(quest_random_describe as
                                         unsafe extern "C" fn(_: *mut FILE)
                                             -> bool_),};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Lost Hobbit\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Merton Proudfoot, a young hobbit, seems to have disappeared.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Last time anyone saw him was near the horrible maze to the south of Bree.\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 25 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_hobbit_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"The Dark Horseman\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"A dark-cloaked horseman has been spotted several times in town.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"He carries an aura of fear with him and people seem to get sick\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"wherever he goes.  Please do something, but be careful...\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 40 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_nazgul_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"The Trolls Glade\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"A group of Forest Trolls settled in an abandoned forest in the\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"south east of our town. They are killing our people.  You must\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"put an end to this!  It might be best to look for them at night.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Local hobbits claim that the mighty swords Orcrist and Glamdring\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"can be found there! Bring back one of them as a proof!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 30 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_troll_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"The Wight Grave\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"The Barrow-Downs hides many mysteries and dangers.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Lately many people, both men and hobbits, have disappeared there.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Please put an end to this threat!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 30 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_wight_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Spiders of Mirkwood\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Powers lurk deep within Mirkwood. Spiders have blocked the\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"path through the forest, and Thranduil\'s folk have been\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"unable to hold them off. It is your task to drive them\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"away. Be careful -- many traps have been laid by their\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"webs, and their venom is dangerous indeed.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 25 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_spider_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Poisoned Water\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"A curse has beset Lothlorien. All trees along the shorelines of Nimrodel\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"are withering away. We fear the blight could spread to the whole forest.\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"The cause seems to be an unknown poison. You are to go to the West and\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"travel along Celebrant and Nimrodel until you discover the source of\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"the poisoning.  Then you must destroy it and drop these potions on\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"the tainted water.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 30 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_poison_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"The Broken Sword\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"You have found Narsil, a broken sword. It is said that the sword that\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"was broken shall be reforged... Maybe it is this one.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"You should bring it to Aragorn at Minas Anor -- he would know.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 20 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_narsil_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Eol the Dark Elf\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"We have disturbing tidings. Eol the Dark Elf has come seeking his kin in\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Gondolin. We cannot let anyone pass the borders of the city without the\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"King\'s leave. Go forth to the eastern mountains and apprehend him. If\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"he resists, use whatever means possible to hinder him from reaching the\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"city. Be wary -- the mountain caves may have many hidden traps.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 30 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(4 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_eol_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Nirnaeth Arnoediad\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"The fortunes of war in the north turn against us.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Morgoth\'s treachery has driven our armies back nigh\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"to the city\'s walls. Go forth from the city gates\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"and clear a path for them to retreat. You need not\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"destroy the troll army, simply drive a path through.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 37 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(4 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_nirnaeth_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Invasion of Gondolin\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Morgoth is upon us! Dragons and Balrogs have poured over secret\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"ways of the Echoriath, and are looking for our city. They are\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"conducted by Maeglin! You must stop him or they will find us.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Do not let Maeglin get to the stairs or everything will be lost!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Go now, be brave.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 80 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(4 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_invasion_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"The Last Alliance\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"The armies of Morgoth are closing in on the last remaining strongholds\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"of resistance against him. We are too far apart to help each other.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"The arrival of our new Thunderlord allies has helped, but can only delay\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"the inevitable. We must be able to stand together and reinforce each other,\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"or both our kingdoms will fall separately. The Thunderlords have taught us\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"how to use the Void Jumpgates: we need you to open a Void Jumpgate in our\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"own city, and that of Gondolin.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Simply travel to Gondolin, but beware of rebel thunderlords.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 80 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(5 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_between_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"The One Ring\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Find the One Ring, then bring it to Mount Doom, in Mordor, to drop\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"it in the Great Fire where it was once forged.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"But beware: *NEVER* use it, or you will be corrupted.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Once it is destroyed you will be able to permanently defeat Sauron.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"The ring must be cast back into the fires of Mount Doom!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 99 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_one_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Mushroom supplies\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Farmer Maggot asked you to bring him back his mushrooms.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Do not harm his dogs.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 3 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_shroom_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"The prisoner of Dol Guldur\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"You keep hearing distress cries in the dark tower of\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Dol Guldur...\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Maybe there is someone being held prisoner and tortured!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 60 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_thrain_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Falling Toward Apotheosis\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"You must enter the Void where Melkor spirit lurks to destroy\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"him forever. Remember however that it is likely to be your own\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"death that awaits you.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 150 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_ultra_good_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Falling Toward Apotheosis\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"You must now launch an onslaught on Valinor itself to eliminate\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"once and for all any posible resistance to your dominance of Arda.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Remember however that it is likely to be your own death that awaits you.\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 150 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_ultra_evil_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Wolves!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"There are wolves pestering my people! They gather in a hut\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"on the edge of town and menace everyone nearby. Your task\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"is to go in there and clear them out.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 15 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_wolves_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Dragons!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"There are dragons pestering my people! They gather in a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"building on the edge of town and menace everyone nearby.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Your task is to go into the building and clear them out.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 25 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(4 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_dragons_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Haunted House!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"There are undead pestering my people! They gather in a hut\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"on the edge of town and menace everyone nearby. Your task\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"is to go into the building and clear out the beasts.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 45 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(5 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_haunted_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         },
         {
             let mut init =
                 quest_type{silent: 0 as libc::c_int as bool_,
                            dynamic_desc: 0 as libc::c_int as bool_,
                            name:
                                *::std::mem::transmute::<&[u8; 40],
                                                         &mut [libc::c_char; 40]>(b"Evil!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                            desc:
                                [*::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"We have burrowed too deep, and let out some creatures of\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"Morgoth\'s that threaten to kill us all! Your task is to save us\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"from them!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                 *::std::mem::transmute::<&[u8; 80],
                                                          &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")],
                            status: 0 as libc::c_int as s16b,
                            level: 60 as libc::c_int as s16b,
                            plot:
                                &mut *plots.as_mut_ptr().offset(6 as
                                                                    libc::c_int
                                                                    as isize)
                                    as *mut s16b,
                            type_0: 0 as libc::c_int as byte_hack,
                            init:
                                Some(quest_evil_init_hook as
                                         unsafe extern "C" fn(_: libc::c_int)
                                             -> bool_),
                            data: [0 as libc::c_int, 0 as libc::c_int, 0, 0],
                            gen_desc: None,};
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
