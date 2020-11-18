use ::libc;
extern "C" {
    #[no_mangle]
    static mut ddd: [s16b; 9];
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut ddx_ddd: [s16b; 9];
    #[no_mangle]
    static mut ddy_ddd: [s16b; 9];
    #[no_mangle]
    static mut arena_monsters: [s16b; 29];
    #[no_mangle]
    static mut character_dungeon: bool_;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut old_dun_level: s16b;
    #[no_mangle]
    static mut object_level: s16b;
    #[no_mangle]
    static mut monster_level: s16b;
    #[no_mangle]
    static mut turn: s32b;
    #[no_mangle]
    static mut old_turn: s32b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut o_max: s16b;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut view_perma_grids: bool_;
    #[no_mangle]
    static mut auto_scum: bool_;
    #[no_mangle]
    static mut dungeon_align: bool_;
    #[no_mangle]
    static mut dungeon_stair: bool_;
    #[no_mangle]
    static mut cheat_peek: bool_;
    #[no_mangle]
    static mut cheat_hear: bool_;
    #[no_mangle]
    static mut cheat_room: bool_;
    #[no_mangle]
    static mut cheat_xtra: bool_;
    #[no_mangle]
    static mut small_levels: bool_;
    #[no_mangle]
    static mut empty_levels: bool_;
    #[no_mangle]
    static mut always_small_level: bool_;
    #[no_mangle]
    static mut feeling: s16b;
    #[no_mangle]
    static mut rating: s16b;
    #[no_mangle]
    static mut good_item_flag: bool_;
    #[no_mangle]
    static mut max_panel_rows: s16b;
    #[no_mangle]
    static mut max_panel_cols: s16b;
    #[no_mangle]
    static mut panel_row_min: s16b;
    #[no_mangle]
    static mut panel_row_max: s16b;
    #[no_mangle]
    static mut panel_col_min: s16b;
    #[no_mangle]
    static mut panel_col_max: s16b;
    #[no_mangle]
    static mut feat_wall_outer: byte_hack;
    #[no_mangle]
    static mut feat_wall_inner: byte_hack;
    #[no_mangle]
    static mut floor_type: [s16b; 100];
    #[no_mangle]
    static mut fill_type: [s16b; 100];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut km_list: *mut monster_type;
    #[no_mangle]
    static mut town_info: *mut town_type;
    #[no_mangle]
    static mut alloc_kind_table_valid: bool_;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut v_info: *mut vault_type;
    #[no_mangle]
    static mut v_text: *mut libc::c_char;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut get_mon_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut get_obj_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_v_idx: u16b;
    #[no_mangle]
    static mut max_d_idx: u16b;
    #[no_mangle]
    static mut max_o_idx: u16b;
    #[no_mangle]
    static mut max_m_idx: u16b;
    #[no_mangle]
    static mut init_flags: libc::c_int;
    #[no_mangle]
    static mut ambush_flag: bool_;
    #[no_mangle]
    static mut fate_flag: bool_;
    #[no_mangle]
    static mut fates: [fate; 200];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut ironman_rooms: bool_;
    #[no_mangle]
    static mut m_allow_special: *mut bool_;
    #[no_mangle]
    static mut k_allow_special: *mut bool_;
    #[no_mangle]
    static mut a_allow_special: *mut bool_;
    #[no_mangle]
    static mut special_lvl: [*mut bool_; 128];
    #[no_mangle]
    static mut generate_special_feeling: bool_;
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut effects: [effect_type; 128];
    #[no_mangle]
    static mut last_teleportation_y: s16b;
    #[no_mangle]
    static mut last_teleportation_x: s16b;
    #[no_mangle]
    static mut game_module: cptr;
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
    fn wiz_lite();
    #[no_mangle]
    fn wiz_lite_extra();
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn place_floor(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn place_filler(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn scatter(yp: *mut libc::c_int, xp: *mut libc::c_int, y: libc::c_int,
               x: libc::c_int, d: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn string_make(str: cptr) -> cptr;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    static mut Rand_quick: bool_;
    #[no_mangle]
    static mut Rand_value: u32b;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn randnor(mean: libc::c_int, stand: libc::c_int) -> s16b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_branch() -> libc::c_int;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn town_gen(t_idx: libc::c_int);
    #[no_mangle]
    fn delete_object(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn delete_monster(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn place_trap(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn place_monster(y: libc::c_int, x: libc::c_int, slp: bool_, grp: bool_)
     -> bool_;
    #[no_mangle]
    fn place_object(y: libc::c_int, x: libc::c_int, good: bool_, great: bool_,
                    where_0: libc::c_int);
    #[no_mangle]
    fn place_gold(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn place_monster_aux(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         slp: bool_, grp: bool_, status: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn get_mon_num_prep() -> errr;
    #[no_mangle]
    fn get_mon_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn m_bonus(max: libc::c_int, level: libc::c_int) -> s16b;
    #[no_mangle]
    fn get_dungeon_special(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn get_dungeon_save(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn m_pop() -> s16b;
    #[no_mangle]
    fn get_pos_player(dis: libc::c_int, ny: *mut libc::c_int,
                      nx: *mut libc::c_int);
    #[no_mangle]
    fn wipe_m_list();
    #[no_mangle]
    fn wipe_o_list();
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn apply_magic(o_ptr: *mut object_type, lev: libc::c_int, okay: bool_,
                   good: bool_, great: bool_);
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn o_pop() -> s16b;
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn place_monster_one(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         ego: libc::c_int, slp: bool_, status: libc::c_int)
     -> s16b;
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn create_artifact(o_ptr: *mut object_type, a_scroll: bool_,
                       get_name: bool_) -> bool_;
    #[no_mangle]
    fn kind_is_legal(k_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_obj_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn get_obj_num_prep() -> errr;
    #[no_mangle]
    fn kind_is_artifactable(k_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn alloc_monster(dis: libc::c_int, slp: bool_) -> bool_;
    #[no_mangle]
    fn get_fbranch() -> libc::c_int;
    #[no_mangle]
    fn set_mon_num_hook();
    #[no_mangle]
    fn get_dungeon_generator(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn wilderness_gen_small();
    #[no_mangle]
    fn wilderness_gen(refresh: libc::c_int);
    #[no_mangle]
    fn process_dungeon_file(name: cptr, yval: *mut libc::c_int,
                            xval: *mut libc::c_int, ymax: libc::c_int,
                            xmax: libc::c_int, init: bool_, full: bool_)
     -> errr;
    #[no_mangle]
    fn player_place(y: libc::c_int, x: libc::c_int) -> s16b;
    #[no_mangle]
    fn get_level_flags();
    #[no_mangle]
    fn load_dungeon(ext: *mut libc::c_char) -> bool_;
}
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
pub struct level_generator_type {
    pub name: cptr,
    pub generator: Option<unsafe extern "C" fn(_: cptr) -> bool_>,
    pub default_stairs: bool_,
    pub default_monsters: bool_,
    pub default_objects: bool_,
    pub default_miscs: bool_,
    pub next: *mut level_generator_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coord {
    pub y: byte_hack,
    pub x: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dun_data {
    pub cent_n: libc::c_int,
    pub cent: [coord; 100],
    pub door_n: libc::c_int,
    pub door: [coord; 200],
    pub wall_n: libc::c_int,
    pub wall: [coord; 500],
    pub tunn_n: libc::c_int,
    pub tunn: [coord; 900],
    pub row_rooms: libc::c_int,
    pub col_rooms: libc::c_int,
    pub room_map: [[bool_; 18]; 6],
    pub crowded: bool_,
}
static mut level_generators: *mut level_generator_type =
    0 as *const level_generator_type as *mut level_generator_type;
/*
 * Add a new generator
 */
#[no_mangle]
pub unsafe extern "C" fn add_level_generator(mut name: cptr,
                                             mut generator:
                                                 Option<unsafe extern "C" fn()
                                                            -> bool_>,
                                             mut stairs: bool_,
                                             mut monsters: bool_,
                                             mut objects: bool_,
                                             mut miscs: bool_) {
    let mut g: *mut level_generator_type = 0 as *mut level_generator_type;
    g =
        memset(ralloc(::std::mem::size_of::<level_generator_type>() as
                          libc::c_ulong) as *mut libc::c_char as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<level_generator_type>() as libc::c_ulong)
            as *mut level_generator_type;
    (*g).name = string_make(name);
    (*g).generator =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> bool_>,
                                Option<unsafe extern "C" fn(_: cptr)
                                           -> bool_>>(generator);
    (*g).default_stairs = stairs;
    (*g).default_monsters = monsters;
    (*g).default_objects = objects;
    (*g).default_miscs = miscs;
    (*g).next = level_generators;
    level_generators = g;
}
/*
 * Dungeon generation data -- see "cave_gen()"
 */
static mut dun: *mut dun_data = 0 as *const dun_data as *mut dun_data;
/*
 * ???
 */
static mut template_race: libc::c_int = 0;
/*
 * Array of room types depths
 */
static mut roomdep: [s16b; 13] =
    [0 as libc::c_int as s16b, 1 as libc::c_int as s16b,
     1 as libc::c_int as s16b, 3 as libc::c_int as s16b,
     3 as libc::c_int as s16b, 5 as libc::c_int as s16b,
     5 as libc::c_int as s16b, 5 as libc::c_int as s16b,
     10 as libc::c_int as s16b, 1 as libc::c_int as s16b,
     3 as libc::c_int as s16b, 10 as libc::c_int as s16b,
     10 as libc::c_int as s16b];
/*
 * Always picks a correct direction
 */
unsafe extern "C" fn correct_dir(mut rdir: *mut libc::c_int,
                                 mut cdir: *mut libc::c_int,
                                 mut y1: libc::c_int, mut x1: libc::c_int,
                                 mut y2: libc::c_int, mut x2: libc::c_int) {
    /* Extract vertical and horizontal directions */
    *rdir =
        if y1 == y2 {
            0 as libc::c_int
        } else if y1 < y2 { 1 as libc::c_int } else { -(1 as libc::c_int) };
    *cdir =
        if x1 == x2 {
            0 as libc::c_int
        } else if x1 < x2 { 1 as libc::c_int } else { -(1 as libc::c_int) };
    /* Never move diagonally */
    if *rdir != 0 && *cdir != 0 {
        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
            *rdir = 0 as libc::c_int
        } else { *cdir = 0 as libc::c_int }
    };
}
/*
 * Pick a random direction
 */
unsafe extern "C" fn rand_dir(mut rdir: *mut libc::c_int,
                              mut cdir: *mut libc::c_int) {
    /* Pick a random direction */
    let mut i: libc::c_int = Rand_div(4 as libc::c_int);
    /* Extract the dy/dx components */
    *rdir = ddy_ddd[i as usize] as libc::c_int;
    *cdir = ddx_ddd[i as usize] as libc::c_int;
}
/*
 * Convert existing terrain type to "up stairs"
 */
unsafe extern "C" fn place_up_stairs(mut y: libc::c_int, mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Create up stairs */
    if Rand_div(3 as libc::c_int) != 0 as libc::c_int ||
           dungeon_flags2 as libc::c_long & 0x2 as libc::c_long != 0 {
        cave_set_feat(y, x, 0x6 as libc::c_int);
    } else { cave_set_feat(y, x, 0xe as libc::c_int); }
    (*c_ptr).special = 0 as libc::c_int as s16b;
}
/*
 * Convert existing terrain type to "down stairs" with dungeon changing.
 */
unsafe extern "C" fn place_magical_stairs(mut y: libc::c_int,
                                          mut x: libc::c_int,
                                          mut next: byte_hack) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Create up stairs */
    cave_set_feat(y, x, 0x7 as libc::c_int);
    (*c_ptr).special = next as s16b;
}
/*
 * Convert existing terrain type to "down stairs"
 */
unsafe extern "C" fn place_down_stairs(mut y: libc::c_int,
                                       mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /*
	 * Create down stairs
	 * All thoses tests are necesary because a shaft can jump up to 4 levels
	 */
    if dun_level as libc::c_int + 4 as libc::c_int >
           (*d_info.offset(dungeon_type as isize)).maxdepth as libc::c_int ||
           Rand_div(3 as libc::c_int) != 0 as libc::c_int ||
           dungeon_flags2 as libc::c_long & 0x2 as libc::c_long != 0 {
        cave_set_feat(y, x, 0x7 as libc::c_int);
    } else { cave_set_feat(y, x, 0xd as libc::c_int); }
    (*c_ptr).special = 0 as libc::c_int as s16b;
}
/*
 * Helper function for place_new_way. Determine if y, x is one of
 * floor features of the current dungeon
 */
unsafe extern "C" fn is_safe_floor(mut y: libc::c_int, mut x: libc::c_int)
 -> bool_ {
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    let mut feat: byte_hack = (*cave[y as usize].offset(x as isize)).feat;
    /* One of the legal floor types */
    if feat as libc::c_int == (*d_ptr).floor1 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    if feat as libc::c_int == (*d_ptr).floor2 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    if feat as libc::c_int == (*d_ptr).floor3 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume non-floor */
    return 0 as libc::c_int as bool_;
}
/*
 * Place a way to next / previoous level on flat places
 */
#[no_mangle]
pub unsafe extern "C" fn place_new_way(mut y: *mut libc::c_int,
                                       mut x: *mut libc::c_int) {
    let mut xx: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut ok: bool_ = 0;
    let mut i: libc::c_int = 0;
    let mut way_n: libc::c_int = 0;
    let mut way_x: [byte_hack; 198] = [0; 198];
    let mut way_y: [byte_hack; 198] = [0; 198];
    loop 
         /* Find valid location XXX XXX XXX */
         /* A way on vertical edge */
         {
        if Rand_div(cur_hgt as libc::c_int + cur_wid as libc::c_int) <
               cur_hgt as libc::c_int {
            /* Pick a random grid */
            *y =
                Rand_div(cur_hgt as libc::c_int - 2 as libc::c_int) +
                    1 as libc::c_int;
            yy = *y;
            *x =
                1 as libc::c_int +
                    Rand_div(2 as libc::c_int) *
                        (cur_wid as libc::c_int - 3 as libc::c_int);
            xx = *x;
            /* Pick a direction */
            if xx == 1 as libc::c_int {
                /* Left */
                x0 = 1 as libc::c_int;
                y0 = 0 as libc::c_int;
                /* Sides */
                x1 = 0 as libc::c_int;
                y1 = -(1 as libc::c_int);
                x2 = 0 as libc::c_int;
                y2 = 1 as libc::c_int
            } else {
                /* Right */
                x0 = -(1 as libc::c_int);
                y0 = 0 as libc::c_int;
                /* Sides */
                x1 = 0 as libc::c_int;
                y1 = -(1 as libc::c_int);
                x2 = 0 as libc::c_int;
                y2 = 1 as libc::c_int
            }
        } else {
            /* A way on horizontal edge */
            /* Pick a random grid */
            *x =
                Rand_div(cur_wid as libc::c_int - 2 as libc::c_int) +
                    1 as libc::c_int;
            xx = *x;
            *y =
                1 as libc::c_int +
                    Rand_div(2 as libc::c_int) *
                        (cur_hgt as libc::c_int - 3 as libc::c_int);
            yy = *y;
            if yy == 1 as libc::c_int {
                /* Down */
                x0 = 0 as libc::c_int;
                y0 = 1 as libc::c_int;
                /* Sides */
                x1 = -(1 as libc::c_int);
                y1 = 0 as libc::c_int;
                x2 = 1 as libc::c_int;
                y2 = 0 as libc::c_int
            } else {
                /* Up */
                x0 = 0 as libc::c_int;
                y0 = -(1 as libc::c_int);
                /* Sides */
                x1 = -(1 as libc::c_int);
                y1 = 0 as libc::c_int;
                x2 = 1 as libc::c_int;
                y2 = 0 as libc::c_int
            }
        }
        /* Pick a direction */
        /* Look at the starting location */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(yy as isize)).offset(xx as isize)
                as *mut cave_type;
        /* Reject locations inside vaults */
        if (*c_ptr).info as libc::c_int & 0x4 as libc::c_int != 0 {
            continue ;
        }
        /* Reject permanent features */
        if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
               0x40 as libc::c_long != 0 &&
               (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long
                   & 0x10 as libc::c_long != 0 {
            continue ;
        }
        /* Reject room walls */
        if (*c_ptr).info as libc::c_int & 0x8 as libc::c_int != 0 &&
               (*c_ptr).feat as libc::c_int == feat_wall_outer as libc::c_int
           {
            continue ;
        }
        /* Look at a neighbouring edge */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset((yy + y1) as
                                                 isize)).offset((xx + x1) as
                                                                    isize) as
                *mut cave_type;
        /* Reject two adjacent ways */
        if (*c_ptr).feat as libc::c_int == 0xb3 as libc::c_int ||
               (*c_ptr).feat as libc::c_int == 0xb4 as libc::c_int {
            continue ;
        }
        /* Look at the other neighbouring edge */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset((yy + y2) as
                                                 isize)).offset((xx + x2) as
                                                                    isize) as
                *mut cave_type;
        /* Reject two adjacent ways */
        if (*c_ptr).feat as libc::c_int == 0xb3 as libc::c_int ||
               (*c_ptr).feat as libc::c_int == 0xb4 as libc::c_int {
            continue ;
        }
        /* Look ahead */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset((yy + y0) as
                                                 isize)).offset((xx + x0) as
                                                                    isize) as
                *mut cave_type;
        /* Reject two adjacent ways -- relatively rare, but this can happen */
        if (*c_ptr).feat as libc::c_int == 0xb3 as libc::c_int ||
               (*c_ptr).feat as libc::c_int == 0xb4 as libc::c_int {
            continue ;
        }
        /* Reset counter */
        way_n = 0 as libc::c_int;
        /* Assume bad location */
        ok = 0 as libc::c_int as bool_;
        /* Check if it connects to current dungeon */
        while yy > 0 as libc::c_int && xx > 0 as libc::c_int &&
                  yy < cur_hgt as libc::c_int - 1 as libc::c_int &&
                  xx < cur_wid as libc::c_int - 1 as libc::c_int {
            /* Check grids ahead */
            if is_safe_floor(yy + y0, xx + x0) != 0 {
                ok = 1 as libc::c_int as bool_
            }
            /* Check side grids */
            if is_safe_floor(yy + y1, xx + x1) != 0 {
                ok = 1 as libc::c_int as bool_
            }
            if is_safe_floor(yy + y2, xx + x2) != 0 {
                ok = 1 as libc::c_int as bool_
            }
            /* Connected */
            if ok != 0 { break ; }
            /* Access grid (ahead) */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset((yy + y0) as
                                                     isize)).offset((xx + x0)
                                                                        as
                                                                        isize)
                    as *mut cave_type;
            /* Avoid opening vaults */
            if (*c_ptr).feat as libc::c_int == 0x3e as libc::c_int {
                /* Comment this out if you find any problems... */
                ok = 1 as libc::c_int as bool_;
                break ;
            } else {
                /* Paranoia */
                if (*c_ptr).feat as libc::c_int == 0x3f as libc::c_int {
                    break ;
                }
                /*
			 * Hack -- Avoid digging room corner
			 *
			 * CAVEAT: Can't handle situations like this:
			 *
			 *     .....########
			 *     .....########
			 *     ######.....>#
			 *     #############
			 *     .....#
			 *     .....#
			 */
                if (*c_ptr).info as libc::c_int & 0x8 as libc::c_int != 0 {
                    let mut c1_ptr: *mut cave_type =
                        &mut *(*cave.as_mut_ptr().offset((yy + y0 + y1) as
                                                             isize)).offset((xx
                                                                                 +
                                                                                 x0
                                                                                 +
                                                                                 x1)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    let mut c2_ptr: *mut cave_type =
                        &mut *(*cave.as_mut_ptr().offset((yy + y0 + y2) as
                                                             isize)).offset((xx
                                                                                 +
                                                                                 x0
                                                                                 +
                                                                                 x2)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Bend the way */
                    if (*c1_ptr).info as libc::c_int & 0x8 as libc::c_int != 0
                           &&
                           (*c2_ptr).info as libc::c_int & 0x8 as libc::c_int
                               == 0 {
                        way_x[way_n as usize] = (xx + x1) as byte_hack;
                        way_y[way_n as usize] = (yy + y1) as byte_hack;
                        way_n += 1;
                        way_x[way_n as usize] = (xx + x1 + x0) as byte_hack;
                        way_y[way_n as usize] = (yy + y1 + y0) as byte_hack;
                        way_n += 1
                    } else if (*c2_ptr).info as libc::c_int &
                                  0x8 as libc::c_int != 0 &&
                                  (*c1_ptr).info as libc::c_int &
                                      0x8 as libc::c_int == 0 {
                        way_x[way_n as usize] = (xx + x2) as byte_hack;
                        way_y[way_n as usize] = (yy + y2) as byte_hack;
                        way_n += 1;
                        way_x[way_n as usize] = (xx + x2 + x0) as byte_hack;
                        way_y[way_n as usize] = (yy + y2 + y0) as byte_hack;
                        way_n += 1
                    } else {
                        way_x[way_n as usize] = (xx + x0) as byte_hack;
                        way_y[way_n as usize] = (yy + y0) as byte_hack;
                        way_n += 1
                    }
                    ok = 1 as libc::c_int as bool_;
                    break ;
                } else {
                    /* Bend the way -- the other direction */
                    /* Remember the location */
                    way_x[way_n as usize] = (xx + x0) as byte_hack;
                    way_y[way_n as usize] = (yy + y0) as byte_hack;
                    way_n += 1;
                    /* Advance */
                    xx += x0;
                    yy += y0
                }
            }
        }
        /* Accept connected corridor */
        if ok != 0 { break ; }
    }
    /* Actually dig a corridor connecting the way to dungeon */
    i = 0 as libc::c_int;
    while i < way_n {
        /* Dig */
        place_floor(way_y[i as usize] as libc::c_int,
                    way_x[i as usize] as libc::c_int);
        i += 1
    };
}
/*
 * Returns random co-ordinates for player/monster/object
 */
#[no_mangle]
pub unsafe extern "C" fn new_player_spot(mut branch: libc::c_int) -> bool_ {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut max_attempts: libc::c_int = 5000 as libc::c_int;
    /* Place the player */
    if dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long != 0 {
        place_new_way(&mut y, &mut x);
    } else {
        loop  {
            let fresh0 = max_attempts;
            max_attempts = max_attempts - 1;
            if !(fresh0 != 0) { break ; }
            /* Pick a legal spot */
            y =
                1 as libc::c_int +
                    Rand_div(1 as libc::c_int +
                                 (cur_hgt as libc::c_int - 2 as libc::c_int) -
                                 1 as libc::c_int);
            x =
                1 as libc::c_int +
                    Rand_div(1 as libc::c_int +
                                 (cur_wid as libc::c_int - 2 as libc::c_int) -
                                 1 as libc::c_int);
            /* Must be a "naked" floor grid */
            if !((*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                     as isize)).flags1 as libc::c_long &
                     0x10 as libc::c_long != 0 &&
                     (*cave[y as usize].offset(x as isize)).feat as
                         libc::c_int != 0xaf as libc::c_int &&
                     (*f_info.offset((*cave[y as
                                                usize].offset(x as
                                                                  isize)).feat
                                         as isize)).flags1 as libc::c_long &
                         0x40 as libc::c_long == 0 &&
                     (*cave[y as usize].offset(x as isize)).o_idx as
                         libc::c_int == 0 as libc::c_int &&
                     (*cave[y as usize].offset(x as isize)).m_idx as
                         libc::c_int == 0 as libc::c_int) {
                continue ;
            }
            /* Refuse to start on anti-teleport grids */
            if !((*cave[y as usize].offset(x as isize)).info as libc::c_int &
                     0x4 as libc::c_int != 0) {
                break ;
            }
        }
    }
    /* Should be -1, actually if we failed... */
    if max_attempts < 1 as libc::c_int { return 0 as libc::c_int as bool_ }
    /* Save the new player grid */
    (*p_ptr).py = y as s16b;
    (*p_ptr).px = x as s16b;
    /* XXX XXX XXX */
    if dungeon_stair as libc::c_int != 0 &&
           dungeon_flags2 as libc::c_long & 0x20 as libc::c_long == 0 &&
           dun_level as libc::c_int != 0 &&
           (is_quest(dun_level as libc::c_int) == 0 ||
                (old_dun_level as libc::c_int) < dun_level as libc::c_int) &&
           branch == 0 {
        if (old_dun_level as libc::c_int) < dun_level as libc::c_int {
            place_up_stairs((*p_ptr).py as libc::c_int,
                            (*p_ptr).px as libc::c_int);
            if dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long != 0
               {
                cave_set_feat((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int,
                              0xb4 as libc::c_int);
            }
        } else {
            place_down_stairs((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int);
            if dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long != 0
               {
                cave_set_feat((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int,
                              0xb3 as libc::c_int);
            }
        }
    }
    return 1 as libc::c_int as bool_;
}
/*
 * Count the number of walls adjacent to the given grid.
 *
 * Note -- Assumes "in_bounds(y, x)"
 *
 * We count only granite walls and permanent walls.
 */
unsafe extern "C" fn next_to_walls(mut y: libc::c_int, mut x: libc::c_int)
 -> libc::c_int {
    let mut k: libc::c_int = 0 as libc::c_int;
    if (*f_info.offset((*cave[(y + 1 as libc::c_int) as
                                  usize].offset(x as isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x20 as libc::c_long != 0 {
        k += 1
    }
    if (*f_info.offset((*cave[(y - 1 as libc::c_int) as
                                  usize].offset(x as isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x20 as libc::c_long != 0 {
        k += 1
    }
    if (*f_info.offset((*cave[y as
                                  usize].offset((x + 1 as libc::c_int) as
                                                    isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x20 as libc::c_long != 0 {
        k += 1
    }
    if (*f_info.offset((*cave[y as
                                  usize].offset((x - 1 as libc::c_int) as
                                                    isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x20 as libc::c_long != 0 {
        k += 1
    }
    return k;
}
/*
 * Convert existing terrain type to rubble
 */
unsafe extern "C" fn place_rubble(mut y: libc::c_int, mut x: libc::c_int) {
    /* Create rubble */
    cave_set_feat(y, x, 0x31 as libc::c_int);
}
/*
 * Place an altar at the given location
 */
unsafe extern "C" fn place_altar(mut y: libc::c_int, mut x: libc::c_int) {
    if Rand_div(100 as libc::c_int) < 10 as libc::c_int {
        cave_set_feat(y, x, 164 as libc::c_int);
    };
}
/*
 * Place a fountain at the given location
 */
unsafe extern "C" fn place_fountain(mut y: libc::c_int, mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut svals: [libc::c_int; 82] = [0; 82];
    let mut maxsval: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    /* List of usable svals */
    k = 1 as libc::c_int;
    while k < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(k as isize) as *mut object_kind;
        if ((*k_ptr).tval as libc::c_int == 71 as libc::c_int ||
                (*k_ptr).tval as libc::c_int == 72 as libc::c_int) &&
               (*k_ptr).level as libc::c_int <= dun_level as libc::c_int &&
               (*k_ptr).flags4 as libc::c_long & 0x10000 as libc::c_long != 0
           {
            if (*k_ptr).tval as libc::c_int == 72 as libc::c_int {
                svals[maxsval as usize] =
                    (*k_ptr).sval as libc::c_int + 63 as libc::c_int
            } else { svals[maxsval as usize] = (*k_ptr).sval as libc::c_int }
            maxsval += 1
        }
        k += 1
    }
    if maxsval == 0 as libc::c_int { return }
    /* Place the fountain */
    if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) < 30 as libc::c_int {
        cave_set_feat(y, x, 0xf as libc::c_int);
        (*c_ptr).special2 = 0 as libc::c_int as s16b
    } else {
        cave_set_feat(y, x, 0x2 as libc::c_int);
        (*c_ptr).special2 =
            damroll(3 as libc::c_int as s16b, 4 as libc::c_int as s16b) as
                s16b
    }
    (*c_ptr).special = svals[Rand_div(maxsval) as usize] as s16b;
}
/*
 * Place a between gate at the given location
 */
unsafe extern "C" fn place_between(mut y: libc::c_int, mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut c1_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut gx: libc::c_int = 0;
    let mut gy: libc::c_int = 0;
    loop  {
        /* Location */
        gy = Rand_div(cur_hgt as s32b);
        gx = Rand_div(cur_wid as s32b);
        /* Require "naked" floor grid */
        if (*f_info.offset((*cave[gy as usize].offset(gx as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[gy as usize].offset(gx as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*f_info.offset((*cave[gy as usize].offset(gx as isize)).feat
                                   as isize)).flags1 as libc::c_long &
                   0x40 as libc::c_long == 0 &&
               (*cave[gy as usize].offset(gx as isize)).o_idx as libc::c_int
                   == 0 as libc::c_int &&
               (*cave[gy as usize].offset(gx as isize)).m_idx as libc::c_int
                   == 0 as libc::c_int {
            break ;
        }
    }
    /* Access the target grid */
    c1_ptr =
        &mut *(*cave.as_mut_ptr().offset(gy as isize)).offset(gx as isize) as
            *mut cave_type;
    /* Place a pair of between gates */
    cave_set_feat(y, x, 0xa0 as libc::c_int);
    (*c_ptr).special = (gx + (gy << 8 as libc::c_int)) as s16b;
    cave_set_feat(gy, gx, 0xa0 as libc::c_int);
    (*c1_ptr).special = (x + (y << 8 as libc::c_int)) as s16b;
}
/*
 * Place an up/down staircase at given location
 */
unsafe extern "C" fn place_random_stairs(mut y: libc::c_int,
                                         mut x: libc::c_int) {
    /* Paranoia */
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
    /* Choose a staircase */
    if dun_level == 0 {
        place_down_stairs(y, x);
    } else if is_quest(dun_level as libc::c_int) != 0 &&
                  dun_level as libc::c_int > 1 as libc::c_int {
        place_up_stairs(y, x);
    } else if dun_level as libc::c_int >=
                  (*d_info.offset(dungeon_type as isize)).maxdepth as
                      libc::c_int {
        if (*d_info.offset(dungeon_type as isize)).next != 0 {
            place_magical_stairs(y, x,
                                 (*d_info.offset(dungeon_type as
                                                     isize)).next);
        } else { place_up_stairs(y, x); }
    } else if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        place_down_stairs(y, x);
    } else { place_up_stairs(y, x); };
}
/*
 * Place a locked door at the given location
 */
unsafe extern "C" fn place_locked_door(mut y: libc::c_int,
                                       mut x: libc::c_int) {
    /* Create locked door */
    cave_set_feat(y, x,
                  0x20 as libc::c_int +
                      (Rand_div(7 as libc::c_int) + 1 as libc::c_int));
}
/*
 * Place a secret door at the given location
 */
unsafe extern "C" fn place_secret_door(mut y: libc::c_int,
                                       mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Vaults */
    if (*c_ptr).info as libc::c_int & 0x4 as libc::c_int != 0 {
        (*c_ptr).mimic = 0x39 as libc::c_int as byte_hack
    } else if (*c_ptr).info as libc::c_int & 0x8 as libc::c_int != 0 {
        /* Ordinary room -- use current outer or inner wall */
        /* Determine if it's inner or outer XXX XXX XXX */
        if (*cave[(y - 1 as libc::c_int) as usize].offset(x as isize)).info as
               libc::c_int & 0x8 as libc::c_int != 0 &&
               (*cave[(y + 1 as libc::c_int) as
                          usize].offset(x as isize)).info as libc::c_int &
                   0x8 as libc::c_int != 0 &&
               (*cave[y as
                          usize].offset((x - 1 as libc::c_int) as isize)).info
                   as libc::c_int & 0x8 as libc::c_int != 0 &&
               (*cave[y as
                          usize].offset((x + 1 as libc::c_int) as isize)).info
                   as libc::c_int & 0x8 as libc::c_int != 0 {
            (*c_ptr).mimic = feat_wall_inner
        } else { (*c_ptr).mimic = feat_wall_outer }
    } else {
        (*c_ptr).mimic =
            fill_type[Rand_div(100 as libc::c_int) as usize] as byte_hack
    }
    /* Create secret door */
    cave_set_feat(y, x, 0x30 as libc::c_int);
}
/*
 * Place a random type of door at the given location
 */
unsafe extern "C" fn place_random_door(mut y: libc::c_int,
                                       mut x: libc::c_int) {
    let mut tmp: libc::c_int = 0;
    /* Choose an object */
    tmp = Rand_div(1000 as libc::c_int);
    /* Open doors (300/1000) */
    if tmp < 300 as libc::c_int {
        /* Create open door */
        cave_set_feat(y, x, 0x4 as libc::c_int);
    } else if tmp < 400 as libc::c_int {
        /* Broken doors (100/1000) */
        /* Create broken door */
        cave_set_feat(y, x, 0x5 as libc::c_int);
    } else if tmp < 600 as libc::c_int {
        /* Secret doors (200/1000) */
        /* Create secret door */
        place_secret_door(y, x);
    } else if tmp < 900 as libc::c_int {
        /* Closed doors (300/1000) */
        /* Create closed door */
        cave_set_feat(y, x, 0x20 as libc::c_int + 0 as libc::c_int);
    } else if tmp < 999 as libc::c_int {
        /* Locked doors (99/1000) */
        /* Create locked door */
        cave_set_feat(y, x,
                      0x20 as libc::c_int +
                          (Rand_div(7 as libc::c_int) + 1 as libc::c_int));
    } else {
        /* Stuck doors (1/1000) */
        /* Create jammed door */
        cave_set_feat(y, x,
                      0x20 as libc::c_int + 0x8 as libc::c_int +
                          Rand_div(8 as libc::c_int) as byte_hack as
                              libc::c_int);
    };
}
/*
 * Places some staircases near walls
 */
unsafe extern "C" fn alloc_stairs(mut feat: libc::c_int, mut num: libc::c_int,
                                  mut walls: libc::c_int,
                                  mut branch: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    /* Place "num" stairs */
    cnt = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num || cnt < 1 as libc::c_int && num > 1 as libc::c_int {
        let mut current_block_29: u64;
        /* Try several times, then decrease "walls" */
        j = 0 as libc::c_int;
        while j <= 5000 as libc::c_int {
            if dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long != 0
               {
                place_new_way(&mut y, &mut x);
            } else {
                /* Pick a random grid */
                y = Rand_div(cur_hgt as s32b);
                x = Rand_div(cur_wid as s32b);
                /* Require "naked" floor grid */
                if !((*f_info.offset((*cave[y as
                                                usize].offset(x as
                                                                  isize)).feat
                                         as isize)).flags1 as libc::c_long &
                         0x10 as libc::c_long != 0 &&
                         (*cave[y as usize].offset(x as isize)).feat as
                             libc::c_int != 0xaf as libc::c_int &&
                         (*f_info.offset((*cave[y as
                                                    usize].offset(x as
                                                                      isize)).feat
                                             as isize)).flags1 as libc::c_long
                             & 0x40 as libc::c_long == 0 &&
                         (*cave[y as usize].offset(x as isize)).o_idx as
                             libc::c_int == 0 as libc::c_int &&
                         (*cave[y as usize].offset(x as isize)).m_idx as
                             libc::c_int == 0 as libc::c_int) {
                    current_block_29 = 16658872821858055392;
                } else if next_to_walls(y, x) < walls {
                    current_block_29 = 16658872821858055392;
                } else { current_block_29 = 12209867499936983673; }
                match current_block_29 {
                    12209867499936983673 => { }
                    _ => { j += 1; continue ; }
                }
            }
            /* Require a certain number of adjacent walls */
            /* Town -- must go down */
            if dun_level == 0 {
                /* Clear previous contents, add down stairs */
                if dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long
                       != 0 {
                    cave_set_feat(y, x, 0xb3 as libc::c_int);
                } else if Rand_div(3 as libc::c_int) == 0 as libc::c_int &&
                              dungeon_flags2 as libc::c_long &
                                  0x2 as libc::c_long == 0 {
                    cave_set_feat(y, x, 0xd as libc::c_int);
                } else { cave_set_feat(y, x, 0x7 as libc::c_int); }
            } else if is_quest(dun_level as libc::c_int) != 0 &&
                          dun_level as libc::c_int >= 1 as libc::c_int ||
                          dun_level as libc::c_int >=
                              (*d_info.offset(dungeon_type as isize)).maxdepth
                                  as libc::c_int &&
                              dungeon_flags1 as libc::c_long &
                                  0x8000 as libc::c_long == 0 {
                /* Quest -- must go up */
                /* Clear previous contents, add up stairs */
                if dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long
                       != 0 {
                    cave_set_feat(y, x, 0xb4 as libc::c_int);
                } else if Rand_div(3 as libc::c_int) == 0 as libc::c_int &&
                              dungeon_flags2 as libc::c_long &
                                  0x2 as libc::c_long == 0 {
                    cave_set_feat(y, x, 0xe as libc::c_int);
                } else { cave_set_feat(y, x, 0x6 as libc::c_int); }
            } else {
                /* Requested type */
                /* Clear previous contents, add stairs */
                cave_set_feat(y, x, feat);
            }
            (*cave[y as usize].offset(x as isize)).special = branch as s16b;
            /* Count the number of stairs we've actually managed to place. */
            cnt += 1;
            break ;
        }
        /* Require fewer walls */
        if walls != 0 { walls -= 1 }
        i += 1
    };
}
/*
 * Allocates some objects (using "place" and "type")
 */
unsafe extern "C" fn alloc_object(mut set: libc::c_int, mut typ: libc::c_int,
                                  mut num: libc::c_int) {
    let mut y: libc::c_int = 1 as libc::c_int;
    let mut x: libc::c_int = 1 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut dummy: libc::c_int = 0 as libc::c_int;
    /* Place some objects */
    k = 0 as libc::c_int;
    while k < num {
        /* Pick a "legal" spot */
        while dummy < 5000 as libc::c_int {
            let mut room: bool_ = 0;
            dummy += 1;
            /* Location */
            y = Rand_div(cur_hgt as s32b);
            x = Rand_div(cur_wid as s32b);
            /* Require "naked" floor grid */
            if !((*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                     as isize)).flags1 as libc::c_long &
                     0x10 as libc::c_long != 0 &&
                     (*cave[y as usize].offset(x as isize)).feat as
                         libc::c_int != 0xaf as libc::c_int &&
                     (*f_info.offset((*cave[y as
                                                usize].offset(x as
                                                                  isize)).feat
                                         as isize)).flags1 as libc::c_long &
                         0x40 as libc::c_long == 0 &&
                     (*cave[y as usize].offset(x as isize)).o_idx as
                         libc::c_int == 0 as libc::c_int &&
                     (*cave[y as usize].offset(x as isize)).m_idx as
                         libc::c_int == 0 as libc::c_int) {
                continue ;
            }
            /* Check for "room" */
            room =
                if (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x8 as libc::c_int != 0 {
                    1 as libc::c_int
                } else { 0 as libc::c_int } as bool_;
            /* Require corridor? */
            if set == 1 as libc::c_int && room as libc::c_int != 0 {
                continue ;
            }
            /* Require room? */
            if !(set == 2 as libc::c_int && room == 0) { break ; }
        }
        if dummy >= 5000 as libc::c_int {
            if cheat_room != 0 {
                msg_format(b"Warning! Could not place object, type : %d!\x00"
                               as *const u8 as *const libc::c_char, typ);
            }
            return
        }
        /* Place something */
        match typ {
            1 => { place_rubble(y, x); }
            3 => { place_trap(y, x); }
            4 => { place_gold(y, x); }
            5 => {
                place_object(y, x, 0 as libc::c_int as bool_,
                             0 as libc::c_int as bool_, 2 as libc::c_int);
            }
            6 => { place_altar(y, x); }
            7 => { place_between(y, x); }
            8 => { place_fountain(y, x); }
            _ => { }
        }
        k += 1
    };
}
/*
 * The following functions create a rectangle (e.g. outer wall of rooms)
 */
#[no_mangle]
pub unsafe extern "C" fn build_rectangle(mut y1: libc::c_int,
                                         mut x1: libc::c_int,
                                         mut y2: libc::c_int,
                                         mut x2: libc::c_int,
                                         mut feat: libc::c_int,
                                         mut info: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Top and bottom boundaries */
    x = x1;
    while x <= x2 {
        cave_set_feat(y1, x, feat);
        let ref mut fresh1 = (*cave[y1 as usize].offset(x as isize)).info;
        *fresh1 = (*fresh1 as libc::c_int | info) as u16b;
        cave_set_feat(y2, x, feat);
        let ref mut fresh2 = (*cave[y2 as usize].offset(x as isize)).info;
        *fresh2 = (*fresh2 as libc::c_int | info) as u16b;
        x += 1
    }
    /* Top and bottom boundaries */
    y = y1;
    while y <= y2 {
        cave_set_feat(y, x1, feat);
        let ref mut fresh3 = (*cave[y as usize].offset(x1 as isize)).info;
        *fresh3 = (*fresh3 as libc::c_int | info) as u16b;
        cave_set_feat(y, x2, feat);
        let ref mut fresh4 = (*cave[y as usize].offset(x2 as isize)).info;
        *fresh4 = (*fresh4 as libc::c_int | info) as u16b;
        y += 1
    };
}
/*
 * Place water through the dungeon using recursive fractal algorithm
 *
 * Why do those good at math and/or algorithms tend *not* to 
 * place any spaces around binary operators? I've been always
 * wondering. This seems almost a unversal phenomenon...
 * Tried to make those conform to the rule, but there may still
 * some left untouched...
 */
unsafe extern "C" fn recursive_river(mut x1: libc::c_int, mut y1: libc::c_int,
                                     mut x2: libc::c_int, mut y2: libc::c_int,
                                     mut feat1: libc::c_int,
                                     mut feat2: libc::c_int,
                                     mut width: libc::c_int) {
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut changex: libc::c_int = 0;
    let mut changey: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    length = distance(x1, y1, x2, y2);
    if length > 4 as libc::c_int {
        /*
		 * Divide path in half and call routine twice.
		 * There is a small chance of splitting the river
		 */
        dx = (x2 - x1) / 2 as libc::c_int;
        dy = (y2 - y1) / 2 as libc::c_int;
        if dy != 0 as libc::c_int {
            /* perturbation perpendicular to path */
            changex =
                (Rand_div(abs(dy)) + 1 as libc::c_int) * 2 as libc::c_int -
                    abs(dy)
        } else { changex = 0 as libc::c_int }
        if dx != 0 as libc::c_int {
            /* perturbation perpendicular to path */
            changey =
                (Rand_div(abs(dx)) + 1 as libc::c_int) * 2 as libc::c_int -
                    abs(dx)
        } else { changey = 0 as libc::c_int }
        /* construct river out of two smaller ones */
        recursive_river(x1, y1, x1 + dx + changex, y1 + dy + changey, feat1,
                        feat2, width);
        recursive_river(x1 + dx + changex, y1 + dy + changey, x2, y2, feat1,
                        feat2, width);
        /* Split the river some of the time -junctions look cool */
        if width > 0 as libc::c_int &&
               Rand_div(50 as libc::c_int) == 0 as libc::c_int {
            recursive_river(x1 + dx + changex, y1 + dy + changey,
                            x1 + 8 as libc::c_int * (dx + changex),
                            y1 + 8 as libc::c_int * (dy + changey), feat1,
                            feat2, width - 1 as libc::c_int);
        }
    } else {
        /* Actually build the river */
        l = 0 as libc::c_int;
        while l < length {
            x = x1 + l * (x2 - x1) / length;
            y = y1 + l * (y2 - y1) / length;
            ty = y - width - 1 as libc::c_int;
            while ty <= y + width + 1 as libc::c_int {
                tx = x - width - 1 as libc::c_int;
                while tx <= x + width + 1 as libc::c_int {
                    if ty > 0 as libc::c_int && tx > 0 as libc::c_int &&
                           ty < cur_hgt as libc::c_int - 1 as libc::c_int &&
                           tx < cur_wid as libc::c_int - 1 as libc::c_int {
                        if !((*cave[ty as usize].offset(tx as isize)).feat as
                                 libc::c_int == feat1) {
                            if !((*cave[ty as usize].offset(tx as isize)).feat
                                     as libc::c_int == feat2) {
                                if !(distance(ty, tx, y, x) >
                                         width +
                                             Rand_div(1 as libc::c_int +
                                                          1 as libc::c_int +
                                                          1 as libc::c_int) -
                                             1 as libc::c_int) {
                                    /* Do not convert permanent features */
                                    if !((*f_info.offset((*cave[ty as
                                                                    usize].offset(tx
                                                                                      as
                                                                                      isize)).feat
                                                             as isize)).flags1
                                             as libc::c_long &
                                             0x40 as libc::c_long != 0) {
                                        /*
					 * Clear previous contents, add feature
					 * The border mainly gets feat2, while the center
					 * gets feat1
					 */
                                        if distance(ty, tx, y, x) > width {
                                            cave_set_feat(ty, tx, feat2);
                                        } else {
                                            cave_set_feat(ty, tx, feat1);
                                        }
                                        /* Lava terrain glows */
                                        if feat1 == 0x55 as libc::c_int ||
                                               feat1 == 0x56 as libc::c_int {
                                            let ref mut fresh5 =
                                                (*cave[ty as
                                                           usize].offset(tx as
                                                                             isize)).info;
                                            *fresh5 =
                                                (*fresh5 as libc::c_int |
                                                     0x2 as libc::c_int) as
                                                    u16b
                                        }
                                        /* Hack -- don't teleport here */
                                        let ref mut fresh6 =
                                            (*cave[ty as
                                                       usize].offset(tx as
                                                                         isize)).info;
                                        *fresh6 =
                                            (*fresh6 as libc::c_int |
                                                 0x4 as libc::c_int) as u16b
                                    }
                                }
                            }
                        }
                    }
                    tx += 1
                }
                ty += 1
            }
            l += 1
        }
    };
}
/*
 * Places water through dungeon.
 */
unsafe extern "C" fn add_river(mut feat1: libc::c_int,
                               mut feat2: libc::c_int) {
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y1: libc::c_int = 0 as libc::c_int;
    let mut x1: libc::c_int = 0 as libc::c_int;
    let mut wid: libc::c_int = 0;
    /* Hack -- Choose starting point */
    y2 =
        Rand_div(cur_hgt as libc::c_int / 2 as libc::c_int - 2 as libc::c_int)
            + 1 as libc::c_int + cur_hgt as libc::c_int / 2 as libc::c_int;
    x2 =
        Rand_div(cur_wid as libc::c_int / 2 as libc::c_int - 2 as libc::c_int)
            + 1 as libc::c_int + cur_wid as libc::c_int / 2 as libc::c_int;
    /* Hack -- Choose ending point somewhere on boundary */
    match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
        1 => {
            /* top boundary */
            x1 =
                Rand_div(cur_wid as libc::c_int - 2 as libc::c_int) +
                    1 as libc::c_int + 1 as libc::c_int;
            y1 = 1 as libc::c_int
        }
        2 => {
            /* left boundary */
            x1 = 1 as libc::c_int;
            y1 =
                Rand_div(cur_hgt as libc::c_int - 2 as libc::c_int) +
                    1 as libc::c_int + 1 as libc::c_int
        }
        3 => {
            /* right boundary */
            x1 = cur_wid as libc::c_int - 1 as libc::c_int;
            y1 =
                Rand_div(cur_hgt as libc::c_int - 2 as libc::c_int) +
                    1 as libc::c_int + 1 as libc::c_int
        }
        4 => {
            /* bottom boundary */
            x1 =
                Rand_div(cur_wid as libc::c_int - 2 as libc::c_int) +
                    1 as libc::c_int + 1 as libc::c_int;
            y1 = cur_hgt as libc::c_int - 1 as libc::c_int
        }
        _ => { }
    }
    wid = Rand_div(2 as libc::c_int) + 1 as libc::c_int;
    recursive_river(x1, y1, x2, y2, feat1, feat2, wid);
}
/*
 * Places "streamers" of rock through dungeon
 *
 * Note that their are actually six different terrain features used
 * to represent streamers.  Three each of magma and quartz, one for
 * basic vein, one with hidden gold, and one with known gold.  The
 * hidden gold types are currently unused.
 */
unsafe extern "C" fn build_streamer(mut feat: libc::c_int,
                                    mut chance: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut dummy: libc::c_int = 0 as libc::c_int;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Hack -- Choose starting point */
    y =
        cur_hgt as libc::c_int / 2 as libc::c_int +
            Rand_div(1 as libc::c_int + 10 as libc::c_int + 10 as libc::c_int)
            - 10 as libc::c_int;
    x =
        cur_wid as libc::c_int / 2 as libc::c_int +
            Rand_div(1 as libc::c_int + 15 as libc::c_int + 15 as libc::c_int)
            - 15 as libc::c_int;
    /* Choose a random compass direction */
    dir = ddd[Rand_div(8 as libc::c_int) as usize] as libc::c_int;
    /* Place streamer into dungeon */
    while dummy < 5000 as libc::c_int {
        dummy += 1;
        /* One grid per density */
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            let mut d: libc::c_int = 2 as libc::c_int;
            loop 
                 /* Pick a nearby grid */
                 {
                ty = y + Rand_div(1 as libc::c_int + d + d) - d;
                tx = x + Rand_div(1 as libc::c_int + d + d) - d;
                if ty >= 0 as libc::c_int && tx >= 0 as libc::c_int &&
                       ty < cur_hgt as libc::c_int &&
                       tx < cur_wid as libc::c_int {
                    break ;
                }
            }
            /* Access the grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(ty as
                                                     isize)).offset(tx as
                                                                        isize)
                    as *mut cave_type;
            /* Only convert "granite" walls */
            if !((*c_ptr).feat as libc::c_int !=
                     feat_wall_inner as libc::c_int &&
                     (*c_ptr).feat as libc::c_int !=
                         feat_wall_outer as libc::c_int &&
                     (*c_ptr).feat as libc::c_int !=
                         (*d_info.offset(dungeon_type as isize)).fill_type1 as
                             libc::c_int &&
                     (*c_ptr).feat as libc::c_int !=
                         (*d_info.offset(dungeon_type as isize)).fill_type2 as
                             libc::c_int &&
                     (*c_ptr).feat as libc::c_int !=
                         (*d_info.offset(dungeon_type as isize)).fill_type3 as
                             libc::c_int) {
                /* Clear mimic feature to avoid nasty consequences */
                (*c_ptr).mimic = 0 as libc::c_int as byte_hack;
                /* Clear previous contents, add proper vein type */
                cave_set_feat(ty, tx, feat);
                /* Hack -- Add some (known) treasure */
                if Rand_div(chance) == 0 as libc::c_int {
                    cave_set_feat(ty, tx,
                                  (*c_ptr).feat as libc::c_int +
                                      0x4 as libc::c_int);
                }
            }
            i += 1
        }
        if dummy >= 5000 as libc::c_int {
            if cheat_room != 0 {
                msg_print(b"Warning! Could not place streamer!\x00" as
                              *const u8 as *const libc::c_char);
            }
            return
        }
        /* Advance the streamer */
        y += ddy[dir as usize] as libc::c_int;
        x += ddx[dir as usize] as libc::c_int;
        /* Quit before leaving the dungeon */
        if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
                 y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                 x < cur_wid as libc::c_int - 1 as libc::c_int) {
            break ;
        }
    };
}
/*
 * Place streams of water, lava, & trees -KMW-
 * This routine varies the placement based on dungeon level
 * otherwise is similar to build_streamer
 */
unsafe extern "C" fn build_streamer2(mut feat: libc::c_int,
                                     mut killwall: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut mid: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut poolchance: libc::c_int = 0;
    let mut poolsize: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    poolchance = Rand_div(10 as libc::c_int) + 1 as libc::c_int;
    /* Hack -- Choose starting point */
    y =
        cur_hgt as libc::c_int / 2 as libc::c_int +
            Rand_div(1 as libc::c_int + 10 as libc::c_int + 10 as libc::c_int)
            - 10 as libc::c_int;
    x =
        cur_wid as libc::c_int / 2 as libc::c_int +
            Rand_div(1 as libc::c_int + 15 as libc::c_int + 15 as libc::c_int)
            - 15 as libc::c_int;
    /* Choose a random compass direction */
    dir = ddd[Rand_div(8 as libc::c_int) as usize] as libc::c_int;
    /* Place streamer into dungeon */
    if poolchance > 2 as libc::c_int {
        loop  {
            let mut current_block_10: u64;
            /* One grid per density */
            i = 0 as libc::c_int;
            while i < 8 as libc::c_int + 1 as libc::c_int {
                let mut d: libc::c_int = 1 as libc::c_int;
                loop 
                     /* Pick a nearby grid */
                     {
                    ty = y + Rand_div(1 as libc::c_int + d + d) - d;
                    tx = x + Rand_div(1 as libc::c_int + d + d) - d;
                    if ty > 0 as libc::c_int && tx > 0 as libc::c_int &&
                           ty < cur_hgt as libc::c_int - 1 as libc::c_int &&
                           tx < cur_wid as libc::c_int - 1 as libc::c_int {
                        break ;
                    }
                }
                /* Access grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(ty as
                                                         isize)).offset(tx as
                                                                            isize)
                        as *mut cave_type;
                /* Never convert vaults */
                if !((*c_ptr).info as libc::c_int & 0x4 as libc::c_int != 0) {
                    /* Reject permanent features */
                    if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                             libc::c_long & 0x40 as libc::c_long != 0 &&
                             (*f_info.offset((*c_ptr).feat as isize)).flags1
                                 as libc::c_long & 0x10 as libc::c_long != 0)
                       {
                        /* Avoid converting walls when told so */
                        if killwall == 0 as libc::c_int {
                            if (*f_info.offset((*c_ptr).feat as isize)).flags1
                                   as libc::c_long & 0x20 as libc::c_long != 0
                               {
                                current_block_10 = 5720623009719927633;
                            } else { current_block_10 = 4495394744059808450; }
                        } else { current_block_10 = 4495394744059808450; }
                        match current_block_10 {
                            5720623009719927633 => { }
                            _ => {
                                /* Clear mimic feature to avoid nasty consequences */
                                (*c_ptr).mimic =
                                    0 as libc::c_int as byte_hack;
                                /* Clear previous contents, add proper vein type */
                                cave_set_feat(ty, tx, feat);
                            }
                        }
                    }
                }
                i += 1
            }
            /* Advance the streamer */
            y += ddy[dir as usize] as libc::c_int;
            x += ddx[dir as usize] as libc::c_int;
            /* Change direction */
            if Rand_div(20 as libc::c_int) == 0 as libc::c_int {
                dir = ddd[Rand_div(8 as libc::c_int) as usize] as libc::c_int
            }
            /* Stop at dungeon edge */
            if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
                     y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                     x < cur_wid as libc::c_int - 1 as libc::c_int) {
                break ;
            }
        }
    } else if feat == 0xbb as libc::c_int || feat == 0x55 as libc::c_int {
        poolsize =
            5 as libc::c_int +
                (Rand_div(10 as libc::c_int) + 1 as libc::c_int);
        mid = poolsize / 2 as libc::c_int;
        /* Create pool */
        /* One grid per density */
        i = 0 as libc::c_int;
        while i < poolsize {
            let mut current_block_25: u64;
            j = 0 as libc::c_int;
            while j < poolsize {
                tx = x + j;
                ty = y + i;
                if ty > 0 as libc::c_int && tx > 0 as libc::c_int &&
                       ty < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       tx < cur_wid as libc::c_int - 1 as libc::c_int {
                    if i < mid {
                        if j < mid {
                            if (i + j + 1 as libc::c_int) < mid {
                                current_block_25 = 7245201122033322888;
                            } else { current_block_25 = 6476622998065200121; }
                        } else if j > mid + i {
                            current_block_25 = 7245201122033322888;
                        } else { current_block_25 = 6476622998065200121; }
                    } else if j < mid {
                        if i > mid + j {
                            current_block_25 = 7245201122033322888;
                        } else { current_block_25 = 6476622998065200121; }
                    } else if i + j >
                                  mid * 3 as libc::c_int - 1 as libc::c_int {
                        current_block_25 = 7245201122033322888;
                    } else { current_block_25 = 6476622998065200121; }
                    match current_block_25 {
                        7245201122033322888 => { }
                        _ =>
                        /* Only convert non-permanent features */
                        {
                            if !((*f_info.offset((*cave[ty as
                                                            usize].offset(tx
                                                                              as
                                                                              isize)).feat
                                                     as isize)).flags1 as
                                     libc::c_long & 0x40 as libc::c_long != 0)
                               {
                                /* Clear mimic feature to avoid nasty consequences */
                                (*cave[ty as usize].offset(tx as isize)).mimic
                                    = 0 as libc::c_int as byte_hack;
                                /* Clear previous contents, add proper vein type */
                                cave_set_feat(ty, tx, feat);
                            }
                        }
                    }
                }
                j += 1
            }
            i += 1
        }
    };
}
/*
 * Build a destroyed level
 */
unsafe extern "C" fn destroy_level() {
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Note destroyed levels */
    if cheat_room as libc::c_int != 0 ||
           (*p_ptr).precognition as libc::c_int != 0 {
        msg_print(b"Destroyed Level\x00" as *const u8 as *const libc::c_char);
    }
    /* Drop a few epi-centers (usually about two) */
    n = 0 as libc::c_int;
    while n < Rand_div(5 as libc::c_int) + 1 as libc::c_int {
        /* Pick an epi-center */
        x1 =
            5 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_wid as libc::c_int - 1 as libc::c_int -
                                  5 as libc::c_int) - 5 as libc::c_int);
        y1 =
            5 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_hgt as libc::c_int - 1 as libc::c_int -
                                  5 as libc::c_int) - 5 as libc::c_int);
        /* Big area of affect */
        y = y1 - 15 as libc::c_int;
        while y <= y1 + 15 as libc::c_int {
            x = x1 - 15 as libc::c_int;
            while x <= x1 + 15 as libc::c_int {
                /* Skip illegal grids */
                if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                       y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       x < cur_wid as libc::c_int - 1 as libc::c_int {
                    /* Extract the distance */
                    k = distance(y1, x1, y, x);
                    /* Stay in the circle of death */
                    if !(k >= 16 as libc::c_int) {
                        /* Delete the monster (if any) */
                        delete_monster(y, x);
                        /* Destroy valid grids */
                        if cave_valid_bold(y, x) != 0 {
                            /* Delete objects */
                            delete_object(y, x);
                            /* Access the grid */
                            c_ptr =
                                &mut *(*cave.as_mut_ptr().offset(y as
                                                                     isize)).offset(x
                                                                                        as
                                                                                        isize)
                                    as *mut cave_type;
                            /* Wall (or floor) type */
                            t = Rand_div(200 as libc::c_int);
                            /* Granite */
                            if t < 20 as libc::c_int {
                                /* Create granite wall */
                                cave_set_feat(y, x, 0x38 as libc::c_int);
                            } else if t < 60 as libc::c_int {
                                /* Quartz */
                                /* Create quartz vein */
                                cave_set_feat(y, x, 0x33 as libc::c_int);
                            } else if t < 90 as libc::c_int {
                                /* Magma */
                                /* Create magma vein */
                                cave_set_feat(y, x, 0x32 as libc::c_int);
                            } else if t < 110 as libc::c_int {
                                /* Sand */
                                /* Create sand vein */
                                cave_set_feat(y, x, 0x62 as libc::c_int);
                            } else {
                                /* Floor */
                                /* Create floor */
                                place_floor(y, x);
                            }
                            /* No longer part of a room or vault */
                            (*c_ptr).info =
                                ((*c_ptr).info as libc::c_int &
                                     !(0x8 as libc::c_int |
                                           0x4 as libc::c_int)) as u16b;
                            /* No longer illuminated or known */
                            (*c_ptr).info =
                                ((*c_ptr).info as libc::c_int &
                                     !(0x1 as libc::c_int |
                                           0x2 as libc::c_int)) as u16b
                        }
                    }
                }
                x += 1
            }
            y += 1
        }
        n += 1
    };
}
/*
 * Function that sees if a square is a floor (Includes range checking)
 */
unsafe extern "C" fn get_is_floor(mut x: libc::c_int, mut y: libc::c_int)
 -> bool_ {
    /* Out of bounds */
    if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
             y < cur_hgt as libc::c_int - 1 as libc::c_int &&
             x < cur_wid as libc::c_int - 1 as libc::c_int) {
        return 0 as libc::c_int as bool_
    }
    /* Do the real check: */
    if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x10 as libc::c_long != 0 {
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Tunnel around a room if it will cut off part of a cave system
 */
unsafe extern "C" fn check_room_boundary(mut x1: libc::c_int,
                                         mut y1: libc::c_int,
                                         mut x2: libc::c_int,
                                         mut y2: libc::c_int) {
    let mut count: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut old_is_floor: bool_ = 0;
    let mut new_is_floor: bool_ = 0;
    /* Avoid doing this in irrelevant places -- pelpel */
    if dungeon_flags1 as libc::c_long & 0x800 as libc::c_long == 0 { return }
    /* Initialize */
    count = 0 as libc::c_int;
    old_is_floor = get_is_floor(x1 - 1 as libc::c_int, y1);
    /*
	 * Count the number of floor-wall boundaries around the room
	 * Note: diagonal squares are ignored since the player can move diagonally
	 * to bypass these if needed.
	 */
    /* Above the top boundary */
    x = x1;
    while x <= x2 {
        new_is_floor = get_is_floor(x, y1 - 1 as libc::c_int);
        /* increment counter if they are different */
        if new_is_floor as libc::c_int != old_is_floor as libc::c_int {
            count += 1
        }
        old_is_floor = new_is_floor;
        x += 1
    }
    /* Right boundary */
    y = y1;
    while y <= y2 {
        new_is_floor = get_is_floor(x2 + 1 as libc::c_int, y);
        /* increment counter if they are different */
        if new_is_floor as libc::c_int != old_is_floor as libc::c_int {
            count += 1
        }
        old_is_floor = new_is_floor;
        y += 1
    }
    /* Bottom boundary*/
    x = x2;
    while x >= x1 {
        new_is_floor = get_is_floor(x, y2 + 1 as libc::c_int);
        /* Increment counter if they are different */
        if new_is_floor as libc::c_int != old_is_floor as libc::c_int {
            count += 1
        }
        old_is_floor = new_is_floor;
        x -= 1
    }
    /* Left boundary */
    y = y2;
    while y >= y1 {
        new_is_floor = get_is_floor(x1 - 1 as libc::c_int, y);
        /* Increment counter if they are different */
        if new_is_floor as libc::c_int != old_is_floor as libc::c_int {
            count += 1
        }
        old_is_floor = new_is_floor;
        y -= 1
    }
    /* If all the same, or only one connection exit */
    if count == 0 as libc::c_int || count == 2 as libc::c_int { return }
    /* Tunnel around the room so to prevent problems with caves */
    y = y1;
    while y <= y2 {
        x = x1;
        while x <= x2 {
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                place_floor(y, x);
            }
            x += 1
        }
        y += 1
    };
}
/*
 * Create up to "num" objects near the given coordinates
 * Only really called by some of the "vault" routines.
 */
unsafe extern "C" fn vault_objects(mut y: libc::c_int, mut x: libc::c_int,
                                   mut num: libc::c_int) {
    let mut dummy: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = y;
    let mut k: libc::c_int = x;
    /* Attempt to place 'num' objects */
    while num > 0 as libc::c_int {
        /* Try up to 11 spots looking for empty space */
        i = 0 as libc::c_int;
        while i < 11 as libc::c_int {
            /* Pick a random location */
            while dummy < 5000 as libc::c_int {
                j =
                    y +
                        Rand_div(1 as libc::c_int + 2 as libc::c_int +
                                     2 as libc::c_int) - 2 as libc::c_int;
                k =
                    x +
                        Rand_div(1 as libc::c_int + 3 as libc::c_int +
                                     3 as libc::c_int) - 3 as libc::c_int;
                dummy += 1;
                if j > 0 as libc::c_int && k > 0 as libc::c_int &&
                       j < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       k < cur_wid as libc::c_int - 1 as libc::c_int {
                    break ;
                }
            }
            if dummy >= 5000 as libc::c_int {
                if cheat_room != 0 {
                    msg_print(b"Warning! Could not place vault object!\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
            /* Require "clean" floor space */
            if !((*f_info.offset((*cave[j as usize].offset(k as isize)).feat
                                     as isize)).flags1 as libc::c_long &
                     0x10 as libc::c_long != 0 &&
                     (*cave[j as usize].offset(k as isize)).feat as
                         libc::c_int != 0xaf as libc::c_int &&
                     (*cave[j as usize].offset(k as isize)).o_idx as
                         libc::c_int == 0 as libc::c_int &&
                     (*f_info.offset((*cave[j as
                                                usize].offset(k as
                                                                  isize)).feat
                                         as isize)).flags1 as libc::c_long &
                         0x40 as libc::c_long == 0) {
                i += 1
            } else {
                /* Place an item */
                if Rand_div(100 as libc::c_int) < 75 as libc::c_int {
                    place_object(j, k, 0 as libc::c_int as bool_,
                                 0 as libc::c_int as bool_, 2 as libc::c_int);
                } else {
                    /* Place gold */
                    place_gold(j, k);
                }
                break ;
            }
        }
        num -= 1
    };
}
/*
 * Place a trap with a given displacement of point
 */
unsafe extern "C" fn vault_trap_aux(mut y: libc::c_int, mut x: libc::c_int,
                                    mut yd: libc::c_int,
                                    mut xd: libc::c_int) {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut y1: libc::c_int = y;
    let mut x1: libc::c_int = x;
    let mut dummy: libc::c_int = 0 as libc::c_int;
    /* Place traps */
    count = 0 as libc::c_int;
    while count <= 5 as libc::c_int {
        /* Get a location */
        while dummy < 5000 as libc::c_int {
            y1 = y + Rand_div(1 as libc::c_int + yd + yd) - yd;
            x1 = x + Rand_div(1 as libc::c_int + xd + xd) - xd;
            dummy += 1;
            if y1 > 0 as libc::c_int && x1 > 0 as libc::c_int &&
                   y1 < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x1 < cur_wid as libc::c_int - 1 as libc::c_int {
                break ;
            }
        }
        if dummy >= 5000 as libc::c_int {
            if cheat_room != 0 {
                msg_print(b"Warning! Could not place vault trap!\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
        /* Require "naked" floor grids */
        if !((*f_info.offset((*cave[y1 as usize].offset(x1 as isize)).feat as
                                 isize)).flags1 as libc::c_long &
                 0x10 as libc::c_long != 0 &&
                 (*cave[y1 as usize].offset(x1 as isize)).feat as libc::c_int
                     != 0xaf as libc::c_int &&
                 (*f_info.offset((*cave[y1 as usize].offset(x1 as isize)).feat
                                     as isize)).flags1 as libc::c_long &
                     0x40 as libc::c_long == 0 &&
                 (*cave[y1 as usize].offset(x1 as isize)).o_idx as libc::c_int
                     == 0 as libc::c_int &&
                 (*cave[y1 as usize].offset(x1 as isize)).m_idx as libc::c_int
                     == 0 as libc::c_int) {
            count += 1
        } else {
            /* Place the trap */
            place_trap(y1, x1);
            break ;
        }
    };
}
/*
 * Place some traps with a given displacement of given location
 */
unsafe extern "C" fn vault_traps(mut y: libc::c_int, mut x: libc::c_int,
                                 mut yd: libc::c_int, mut xd: libc::c_int,
                                 mut num: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num { vault_trap_aux(y, x, yd, xd); i += 1 };
}
/*
 * Hack -- Place some sleeping monsters near the given location
 */
unsafe extern "C" fn vault_monsters(mut y1: libc::c_int, mut x1: libc::c_int,
                                    mut num: libc::c_int) {
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Try to summon "num" monsters "near" the given location */
    k = 0 as libc::c_int;
    while k < num {
        /* Try nine locations */
        i = 0 as libc::c_int;
        while i < 9 as libc::c_int {
            let mut d: libc::c_int = 1 as libc::c_int;
            /* Pick a nearby location */
            scatter(&mut y, &mut x, y1, x1, d);
            /* Require "empty" floor grids */
            if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                       != 0xaf as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
                   !(y == (*p_ptr).py as libc::c_int &&
                         x == (*p_ptr).px as libc::c_int) {
                /* Place the monster (allow groups) */
                monster_level =
                    (dun_level as libc::c_int + 2 as libc::c_int) as s16b;
                place_monster(y, x, 1 as libc::c_int as bool_,
                              1 as libc::c_int as bool_);
                monster_level = dun_level
            }
            i += 1
        }
        k += 1
    };
}
/*
 * Allocate the space needed by a room in the room_map array.
 *
 * width, height represent the size of the room (0...x-1) by (0...y-1).
 * crowded is used to denote a monset nest.
 * by0, bx0 are the positions in the room_map array given to the build_type'x'
 * function.
 * cx, cy are the returned center of the allocated room in coordinates for
 * cave.feat and cave.info etc.
 */
#[no_mangle]
pub unsafe extern "C" fn room_alloc(mut width: libc::c_int,
                                    mut height: libc::c_int,
                                    mut crowded: bool_, mut by0: libc::c_int,
                                    mut bx0: libc::c_int,
                                    mut cx: *mut libc::c_int,
                                    mut cy: *mut libc::c_int) -> bool_ {
    let mut temp: libc::c_int = 0;
    let mut eby: libc::c_int = 0;
    let mut ebx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut bx: libc::c_int = 0;
    /* Calculate number of room_map squares to allocate */
    /* Total number along width */
    temp = (width - 1 as libc::c_int) / 11 as libc::c_int + 1 as libc::c_int;
    ebx = bx0 + temp;
    while bx0 > 0 as libc::c_int && ebx > (*dun).col_rooms {
        bx0 -= 1;
        ebx -= 1
    }
    if ebx > (*dun).col_rooms { return 0 as libc::c_int as bool_ }
    /* Total number along height */
    temp = (height - 1 as libc::c_int) / 11 as libc::c_int + 1 as libc::c_int;
    eby = by0 + temp;
    while by0 > 0 as libc::c_int && eby > (*dun).row_rooms {
        by0 -= 1;
        eby -= 1
    }
    /* Never run off the screen */
    if eby > (*dun).row_rooms { return 0 as libc::c_int as bool_ }
    /* Verify open space */
    by = by0;
    while by < eby {
        bx = bx0;
        while bx < ebx {
            if (*dun).room_map[by as usize][bx as usize] != 0 {
                return 0 as libc::c_int as bool_
            }
            bx += 1
        }
        by += 1
    }
    /*
	 * It is *extremely* important that the following calculation
	 * be *exactly* correct to prevent memory errors XXX XXX XXX
	 */
    /* Acquire the location of the room */
    *cy = (by0 + eby) * 11 as libc::c_int / 2 as libc::c_int;
    *cx = (bx0 + ebx) * 11 as libc::c_int / 2 as libc::c_int;
    /* Save the room location */
    if (*dun).cent_n < 100 as libc::c_int {
        (*dun).cent[(*dun).cent_n as usize].y = *cy as byte_hack;
        (*dun).cent[(*dun).cent_n as usize].x = *cx as byte_hack;
        (*dun).cent_n += 1
    }
    /* Reserve some blocks */
    by = by0;
    while by < eby {
        bx = bx0;
        while bx < ebx {
            (*dun).room_map[by as usize][bx as usize] =
                1 as libc::c_int as bool_;
            bx += 1
        }
        by += 1
    }
    /* Count "crowded" rooms */
    if crowded != 0 { (*dun).crowded = 1 as libc::c_int as bool_ }
    /*
	 * Hack -- See if room will cut off a cavern.
	 * If so, fix by tunneling outside the room in such a way as
	 * to conect the caves.
	 */
    check_room_boundary(*cx - width / 2 as libc::c_int - 1 as libc::c_int,
                        *cy - height / 2 as libc::c_int - 1 as libc::c_int,
                        *cx + width / 2 as libc::c_int + 1 as libc::c_int,
                        *cy + height / 2 as libc::c_int + 1 as libc::c_int);
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Room building routines.
 *
 * Room types:
 *   1 -- normal
 *   2 -- overlapping
 *   3 -- cross shaped
 *   4 -- large room with features
 *   5 -- monster nests
 *   6 -- monster pits
 *   7 -- simple vaults
 *   8 -- greater vaults
 *   9 -- circular rooms
 */
/*
 * Type 1 -- normal rectangular rooms
 */
unsafe extern "C" fn build_type1(mut by0: libc::c_int, mut bx0: libc::c_int) {
    let mut info: u16b = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 1 as libc::c_int;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    let mut xval: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut xsize: libc::c_int = 0;
    let mut ysize: libc::c_int = 0;
    /* Pick a room size */
    y1 =
        1 as libc::c_int +
            Rand_div(1 as libc::c_int + 4 as libc::c_int - 1 as libc::c_int);
    x1 =
        1 as libc::c_int +
            Rand_div(1 as libc::c_int + 10 as libc::c_int - 1 as libc::c_int);
    y2 =
        1 as libc::c_int +
            Rand_div(1 as libc::c_int + 3 as libc::c_int - 1 as libc::c_int);
    x2 =
        1 as libc::c_int +
            Rand_div(1 as libc::c_int + 9 as libc::c_int - 1 as libc::c_int);
    xsize = x1 + x2;
    ysize = y1 + y2;
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc(xsize + 2 as libc::c_int, ysize + 2 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        return
    }
    /* Get corner values */
    y1 = yval - ysize / 2 as libc::c_int;
    x1 = xval - xsize / 2 as libc::c_int;
    y2 = y1 + ysize - 1 as libc::c_int;
    x2 = x1 + xsize - 1 as libc::c_int;
    info =
        if dun_level as libc::c_int <=
               Rand_div(25 as libc::c_int) + 1 as libc::c_int {
            (0x8 as libc::c_int) | 0x2 as libc::c_int
        } else { 0x8 as libc::c_int } as u16b;
    /* Place a full floor under the room */
    y = y1;
    while y <= y2 {
        x = x1;
        while x <= x2 {
            place_floor(y, x);
            let ref mut fresh7 = (*cave[y as usize].offset(x as isize)).info;
            *fresh7 = (*fresh7 as libc::c_int | info as libc::c_int) as u16b;
            x += 1
        }
        y += 1
    }
    /* Walls around the room */
    build_rectangle(y1 - 1 as libc::c_int, x1 - 1 as libc::c_int,
                    y2 + 1 as libc::c_int, x2 + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int, info as libc::c_int);
    /* Hack -- Occasional pillar room */
    if ysize > 2 as libc::c_int && xsize > 2 as libc::c_int {
        if Rand_div(20 as libc::c_int) == 0 as libc::c_int {
            y = y1;
            while y <= y2 {
                x = x1;
                while x <= x2 {
                    cave_set_feat(y, x, feat_wall_inner as libc::c_int);
                    x += 2 as libc::c_int
                }
                y += 2 as libc::c_int
            }
        } else if Rand_div(50 as libc::c_int) == 0 as libc::c_int {
            y = y1 + 2 as libc::c_int;
            while y <= y2 - 2 as libc::c_int {
                cave_set_feat(y, x1, feat_wall_inner as libc::c_int);
                cave_set_feat(y, x2, feat_wall_inner as libc::c_int);
                y += 2 as libc::c_int
            }
            x = x1 + 2 as libc::c_int;
            while x <= x2 - 2 as libc::c_int {
                cave_set_feat(y1, x, feat_wall_inner as libc::c_int);
                cave_set_feat(y2, x, feat_wall_inner as libc::c_int);
                x += 2 as libc::c_int
            }
        }
    };
}
/* Hack -- Occasional ragged-edge room */
/*
 * Type 2 -- Overlapping rectangular rooms
 */
unsafe extern "C" fn build_type2(mut by0: libc::c_int, mut bx0: libc::c_int) {
    let mut info: u16b = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    let mut xval: libc::c_int = 0;
    let mut y1a: libc::c_int = 0;
    let mut x1a: libc::c_int = 0;
    let mut y2a: libc::c_int = 0;
    let mut x2a: libc::c_int = 0;
    let mut y1b: libc::c_int = 0;
    let mut x1b: libc::c_int = 0;
    let mut y2b: libc::c_int = 0;
    let mut x2b: libc::c_int = 0;
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc(25 as libc::c_int, 11 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        return
    }
    /* Determine extents of the first room */
    y1a = yval - (Rand_div(4 as libc::c_int) + 1 as libc::c_int);
    y2a = yval + (Rand_div(3 as libc::c_int) + 1 as libc::c_int);
    x1a = xval - (Rand_div(14 as libc::c_int) + 1 as libc::c_int);
    x2a = xval + (Rand_div(6 as libc::c_int) + 1 as libc::c_int);
    /* Determine extents of the second room */
    y1b = yval - (Rand_div(3 as libc::c_int) + 1 as libc::c_int);
    y2b = yval + (Rand_div(4 as libc::c_int) + 1 as libc::c_int);
    x1b = xval - (Rand_div(6 as libc::c_int) + 1 as libc::c_int);
    x2b = xval + (Rand_div(14 as libc::c_int) + 1 as libc::c_int);
    info =
        if dun_level as libc::c_int <=
               Rand_div(25 as libc::c_int) + 1 as libc::c_int {
            (0x8 as libc::c_int) | 0x2 as libc::c_int
        } else { 0x8 as libc::c_int } as u16b;
    /* Place the walls around room "a" */
    build_rectangle(y1a - 1 as libc::c_int, x1a - 1 as libc::c_int,
                    y2a + 1 as libc::c_int, x2a + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int, info as libc::c_int);
    /* Place the walls around room "a" */
    build_rectangle(y1b - 1 as libc::c_int, x1b - 1 as libc::c_int,
                    y2b + 1 as libc::c_int, x2b + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int, info as libc::c_int);
    /* Replace the floor for room "a" */
    y = y1a;
    while y <= y2a {
        x = x1a;
        while x <= x2a {
            place_floor(y, x);
            let ref mut fresh8 = (*cave[y as usize].offset(x as isize)).info;
            *fresh8 = (*fresh8 as libc::c_int | info as libc::c_int) as u16b;
            x += 1
        }
        y += 1
    }
    /* Replace the floor for room "b" */
    y = y1b;
    while y <= y2b {
        x = x1b;
        while x <= x2b {
            place_floor(y, x);
            let ref mut fresh9 = (*cave[y as usize].offset(x as isize)).info;
            *fresh9 = (*fresh9 as libc::c_int | info as libc::c_int) as u16b;
            x += 1
        }
        y += 1
    };
}
/*
 * Type 3 -- Cross shaped rooms
 *
 * Builds a room at a row, column coordinate
 *
 * Room "a" runs north/south, and Room "b" runs east/east
 * So the "central pillar" runs from x1a,y1b to x2a,y2b.
 *
 * Note that currently, the "center" is always 3x3, but I think that
 * the code below will work (with "bounds checking") for 5x5, or even
 * for unsymetric values like 4x3 or 5x3 or 3x4 or 3x5, or even larger.
 */
unsafe extern "C" fn build_type3(mut by0: libc::c_int, mut bx0: libc::c_int) {
    let mut info: u16b = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut wy: libc::c_int = 0;
    let mut wx: libc::c_int = 0;
    let mut y1a: libc::c_int = 0;
    let mut x1a: libc::c_int = 0;
    let mut y2a: libc::c_int = 0;
    let mut x2a: libc::c_int = 0;
    let mut y1b: libc::c_int = 0;
    let mut x1b: libc::c_int = 0;
    let mut y2b: libc::c_int = 0;
    let mut x2b: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    let mut xval: libc::c_int = 0;
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc(25 as libc::c_int, 11 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        return
    }
    /* For now, always 3x3 */
    wy = 1 as libc::c_int;
    wx = wy;
    /* Pick max vertical size (at most 4) */
    dy =
        3 as libc::c_int +
            Rand_div(1 as libc::c_int + 4 as libc::c_int - 3 as libc::c_int);
    /* Pick max horizontal size (at most 11) */
    dx =
        3 as libc::c_int +
            Rand_div(1 as libc::c_int + 11 as libc::c_int - 3 as libc::c_int);
    /* Determine extents of the north/south room */
    y1a = yval - dy;
    y2a = yval + dy;
    x1a = xval - wx;
    x2a = xval + wx;
    /* Determine extents of the east/west room */
    y1b = yval - wy;
    y2b = yval + wy;
    x1b = xval - dx;
    x2b = xval + dx;
    info =
        if dun_level as libc::c_int <=
               Rand_div(25 as libc::c_int) + 1 as libc::c_int {
            (0x8 as libc::c_int) | 0x2 as libc::c_int
        } else { 0x8 as libc::c_int } as u16b;
    /* Place the walls around room "a" */
    build_rectangle(y1a - 1 as libc::c_int, x1a - 1 as libc::c_int,
                    y2a + 1 as libc::c_int, x2a + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int, info as libc::c_int);
    /* Place the walls around room "a" */
    build_rectangle(y1b - 1 as libc::c_int, x1b - 1 as libc::c_int,
                    y2b + 1 as libc::c_int, x2b + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int, info as libc::c_int);
    /* Replace the floor for room "a" */
    y = y1a;
    while y <= y2a {
        x = x1a;
        while x <= x2a {
            place_floor(y, x);
            let ref mut fresh10 = (*cave[y as usize].offset(x as isize)).info;
            *fresh10 =
                (*fresh10 as libc::c_int | info as libc::c_int) as u16b;
            x += 1
        }
        y += 1
    }
    /* Replace the floor for room "b" */
    y = y1b;
    while y <= y2b {
        x = x1b;
        while x <= x2b {
            place_floor(y, x);
            let ref mut fresh11 = (*cave[y as usize].offset(x as isize)).info;
            *fresh11 =
                (*fresh11 as libc::c_int | info as libc::c_int) as u16b;
            x += 1
        }
        y += 1
    }
    /* Special features (3/4) */
    match Rand_div(4 as libc::c_int) {
        1 => {
            /* Large solid middle pillar */
            y = y1b;
            while y <= y2b {
                x = x1a;
                while x <= x2a {
                    cave_set_feat(y, x, feat_wall_inner as libc::c_int);
                    x += 1
                }
                y += 1
            }
        }
        2 => {
            /* Inner treasure vault */
            /* Build the vault */
            build_rectangle(y1b, x1a, y2b, x2a,
                            feat_wall_inner as libc::c_int,
                            info as libc::c_int);
            /* Place a secret door on the inner room */
            match Rand_div(4 as libc::c_int) {
                0 => { place_secret_door(y1b, xval); }
                1 => { place_secret_door(y2b, xval); }
                2 => { place_secret_door(yval, x1a); }
                3 => { place_secret_door(yval, x2a); }
                _ => { }
            }
            /* Place a treasure in the vault */
            place_object(yval, xval, 0 as libc::c_int as bool_,
                         0 as libc::c_int as bool_, 2 as libc::c_int);
            /* Let's guard the treasure well */
            vault_monsters(yval, xval,
                           Rand_div(2 as libc::c_int) + 3 as libc::c_int);
            /* Traps naturally */
            vault_traps(yval, xval, 4 as libc::c_int, 4 as libc::c_int,
                        Rand_div(3 as libc::c_int) + 2 as libc::c_int);
        }
        3 => {
            /* Something else */
            /* Occasionally pinch the center shut */
            if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
                /* Pinch the east/west sides */
                y = y1b;
                while y <= y2b {
                    if !(y == yval) {
                        cave_set_feat(y, x1a - 1 as libc::c_int,
                                      feat_wall_inner as libc::c_int);
                        cave_set_feat(y, x2a + 1 as libc::c_int,
                                      feat_wall_inner as libc::c_int);
                    }
                    y += 1
                }
                /* Pinch the north/south sides */
                x = x1a;
                while x <= x2a {
                    if !(x == xval) {
                        cave_set_feat(y1b - 1 as libc::c_int, x,
                                      feat_wall_inner as libc::c_int);
                        cave_set_feat(y2b + 1 as libc::c_int, x,
                                      feat_wall_inner as libc::c_int);
                    }
                    x += 1
                }
                /* Sometimes shut using secret doors */
                if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
                    place_secret_door(yval, x1a - 1 as libc::c_int);
                    place_secret_door(yval, x2a + 1 as libc::c_int);
                    place_secret_door(y1b - 1 as libc::c_int, xval);
                    place_secret_door(y2b + 1 as libc::c_int, xval);
                }
            } else if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
                cave_set_feat(yval, xval, feat_wall_inner as libc::c_int);
                cave_set_feat(y1b, xval, feat_wall_inner as libc::c_int);
                cave_set_feat(y2b, xval, feat_wall_inner as libc::c_int);
                cave_set_feat(yval, x1a, feat_wall_inner as libc::c_int);
                cave_set_feat(yval, x2a, feat_wall_inner as libc::c_int);
            } else if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
                cave_set_feat(yval, xval, feat_wall_inner as libc::c_int);
            }
        }
        _ => { }
    };
}
/* Occasionally put a "plus" in the center */
/* Occasionally put a pillar in the center */
/*
 * Type 4 -- Large room with inner features
 *
 * Possible sub-types:
 *	1 - Just an inner room with one door
 *	2 - An inner room within an inner room
 *	3 - An inner room with pillar(s)
 *	4 - Inner room has a maze
 *	5 - A set of four inner rooms
 */
unsafe extern "C" fn build_type4(mut by0: libc::c_int, mut bx0: libc::c_int) {
    let mut info: u16b = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    let mut xval: libc::c_int = 0;
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc(25 as libc::c_int, 11 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        return
    }
    /* Large room */
    y1 = yval - 4 as libc::c_int;
    y2 = yval + 4 as libc::c_int;
    x1 = xval - 11 as libc::c_int;
    x2 = xval + 11 as libc::c_int;
    info =
        if dun_level as libc::c_int <=
               Rand_div(25 as libc::c_int) + 1 as libc::c_int {
            (0x8 as libc::c_int) | 0x2 as libc::c_int
        } else { 0x8 as libc::c_int } as u16b;
    /* Place a full floor under the room */
    y = y1 - 1 as libc::c_int;
    while y <= y2 + 1 as libc::c_int {
        x = x1 - 1 as libc::c_int;
        while x <= x2 + 1 as libc::c_int {
            place_floor(y, x);
            let ref mut fresh12 = (*cave[y as usize].offset(x as isize)).info;
            *fresh12 =
                (*fresh12 as libc::c_int | info as libc::c_int) as u16b;
            x += 1
        }
        y += 1
    }
    /* Outer Walls */
    build_rectangle(y1 - 1 as libc::c_int, x1 - 1 as libc::c_int,
                    y2 + 1 as libc::c_int, x2 + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int, info as libc::c_int);
    /* The inner room */
    y1 = y1 + 2 as libc::c_int;
    y2 = y2 - 2 as libc::c_int;
    x1 = x1 + 2 as libc::c_int;
    x2 = x2 - 2 as libc::c_int;
    /* The inner walls */
    build_rectangle(y1 - 1 as libc::c_int, x1 - 1 as libc::c_int,
                    y2 + 1 as libc::c_int, x2 + 1 as libc::c_int,
                    feat_wall_inner as libc::c_int, info as libc::c_int);
    /* Inner room variations */
    match Rand_div(5 as libc::c_int) + 1 as libc::c_int {
        1 => {
            /* Just an inner room with a monster */
            /* Place a secret door */
            match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
                1 => { place_secret_door(y1 - 1 as libc::c_int, xval); }
                2 => { place_secret_door(y2 + 1 as libc::c_int, xval); }
                3 => { place_secret_door(yval, x1 - 1 as libc::c_int); }
                4 => { place_secret_door(yval, x2 + 1 as libc::c_int); }
                _ => { }
            }
            /* Place a monster in the room */
            vault_monsters(yval, xval, 1 as libc::c_int);
        }
        2 => {
            /* Treasure Vault (with a door) */
            /* Place a secret door */
            match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
                1 => { place_secret_door(y1 - 1 as libc::c_int, xval); }
                2 => { place_secret_door(y2 + 1 as libc::c_int, xval); }
                3 => { place_secret_door(yval, x1 - 1 as libc::c_int); }
                4 => { place_secret_door(yval, x2 + 1 as libc::c_int); }
                _ => { }
            }
            /* Place another inner room */
            build_rectangle(yval - 1 as libc::c_int, xval - 1 as libc::c_int,
                            yval + 1 as libc::c_int, xval + 1 as libc::c_int,
                            feat_wall_inner as libc::c_int,
                            info as libc::c_int);
            /* Place a locked door on the inner room */
            match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
                1 => { place_locked_door(yval - 1 as libc::c_int, xval); }
                2 => { place_locked_door(yval + 1 as libc::c_int, xval); }
                3 => { place_locked_door(yval, xval - 1 as libc::c_int); }
                4 => { place_locked_door(yval, xval + 1 as libc::c_int); }
                _ => { }
            }
            /* Monsters to guard the "treasure" */
            vault_monsters(yval, xval,
                           Rand_div(3 as libc::c_int) + 1 as libc::c_int +
                               2 as libc::c_int);
            /* Object (80%) */
            if Rand_div(100 as libc::c_int) < 80 as libc::c_int {
                place_object(yval, xval, 0 as libc::c_int as bool_,
                             0 as libc::c_int as bool_, 2 as libc::c_int);
            } else {
                /* Stairs (20%) */
                place_random_stairs(yval, xval);
            }
            /* Traps to protect the treasure */
            vault_traps(yval, xval, 4 as libc::c_int, 10 as libc::c_int,
                        2 as libc::c_int +
                            (Rand_div(3 as libc::c_int) + 1 as libc::c_int));
        }
        3 => {
            /* Inner pillar(s). */
            /* Place a secret door */
            match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
                1 => { place_secret_door(y1 - 1 as libc::c_int, xval); }
                2 => { place_secret_door(y2 + 1 as libc::c_int, xval); }
                3 => { place_secret_door(yval, x1 - 1 as libc::c_int); }
                4 => { place_secret_door(yval, x2 + 1 as libc::c_int); }
                _ => { }
            }
            /* Large Inner Pillar */
            y = yval - 1 as libc::c_int;
            while y <= yval + 1 as libc::c_int {
                x = xval - 1 as libc::c_int;
                while x <= xval + 1 as libc::c_int {
                    cave_set_feat(y, x, feat_wall_inner as libc::c_int);
                    x += 1
                }
                y += 1
            }
            /* Occasionally, two more Large Inner Pillars */
            if Rand_div(2 as libc::c_int) == 0 as libc::c_int {
                tmp = Rand_div(2 as libc::c_int) + 1 as libc::c_int;
                y = yval - 1 as libc::c_int;
                while y <= yval + 1 as libc::c_int {
                    x = xval - 5 as libc::c_int - tmp;
                    while x <= xval - 3 as libc::c_int - tmp {
                        cave_set_feat(y, x, feat_wall_inner as libc::c_int);
                        x += 1
                    }
                    x = xval + 3 as libc::c_int + tmp;
                    while x <= xval + 5 as libc::c_int + tmp {
                        cave_set_feat(y, x, feat_wall_inner as libc::c_int);
                        x += 1
                    }
                    y += 1
                }
            }
            /* Occasionally, some Inner rooms */
            if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
                /* Long horizontal walls */
                x = xval - 5 as libc::c_int;
                while x <= xval + 5 as libc::c_int {
                    cave_set_feat(yval - 1 as libc::c_int, x,
                                  feat_wall_inner as libc::c_int);
                    cave_set_feat(yval + 1 as libc::c_int, x,
                                  feat_wall_inner as libc::c_int);
                    x += 1
                }
                /* Close off the left/right edges */
                cave_set_feat(yval, xval - 5 as libc::c_int,
                              feat_wall_inner as libc::c_int);
                cave_set_feat(yval, xval + 5 as libc::c_int,
                              feat_wall_inner as libc::c_int);
                /* Secret doors (random top/bottom) */
                place_secret_door(yval - 3 as libc::c_int +
                                      (Rand_div(2 as libc::c_int) +
                                           1 as libc::c_int) *
                                          2 as libc::c_int,
                                  xval - 3 as libc::c_int);
                place_secret_door(yval - 3 as libc::c_int +
                                      (Rand_div(2 as libc::c_int) +
                                           1 as libc::c_int) *
                                          2 as libc::c_int,
                                  xval + 3 as libc::c_int);
                /* Monsters */
                vault_monsters(yval, xval - 2 as libc::c_int,
                               Rand_div(2 as libc::c_int) + 1 as libc::c_int);
                vault_monsters(yval, xval + 2 as libc::c_int,
                               Rand_div(2 as libc::c_int) + 1 as libc::c_int);
                /* Objects */
                if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
                    place_object(yval, xval - 2 as libc::c_int,
                                 0 as libc::c_int as bool_,
                                 0 as libc::c_int as bool_, 2 as libc::c_int);
                }
                if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
                    place_object(yval, xval + 2 as libc::c_int,
                                 0 as libc::c_int as bool_,
                                 0 as libc::c_int as bool_, 2 as libc::c_int);
                }
            }
        }
        4 => {
            /* Maze inside. */
            /* Place a secret door */
            match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
                1 => { place_secret_door(y1 - 1 as libc::c_int, xval); }
                2 => { place_secret_door(y2 + 1 as libc::c_int, xval); }
                3 => { place_secret_door(yval, x1 - 1 as libc::c_int); }
                4 => { place_secret_door(yval, x2 + 1 as libc::c_int); }
                _ => { }
            }
            /* Maze (really a checkerboard) */
            y = y1;
            while y <= y2 {
                x = x1;
                while x <= x2 {
                    if 0x1 as libc::c_int & x + y != 0 {
                        cave_set_feat(y, x, feat_wall_inner as libc::c_int);
                    }
                    x += 1
                }
                y += 1
            }
            /* Monsters just love mazes. */
            vault_monsters(yval, xval - 5 as libc::c_int,
                           Rand_div(3 as libc::c_int) + 1 as libc::c_int);
            vault_monsters(yval, xval + 5 as libc::c_int,
                           Rand_div(3 as libc::c_int) + 1 as libc::c_int);
            /* Traps make them entertaining. */
            vault_traps(yval, xval - 3 as libc::c_int, 2 as libc::c_int,
                        8 as libc::c_int,
                        Rand_div(3 as libc::c_int) + 1 as libc::c_int);
            vault_traps(yval, xval + 3 as libc::c_int, 2 as libc::c_int,
                        8 as libc::c_int,
                        Rand_div(3 as libc::c_int) + 1 as libc::c_int);
            /* Mazes should have some treasure too. */
            vault_objects(yval, xval, 3 as libc::c_int);
        }
        5 => {
            /* Four small rooms. */
            /* Inner "cross" */
            y = y1;
            while y <= y2 {
                cave_set_feat(y, xval, feat_wall_inner as libc::c_int);
                y += 1
            }
            x = x1;
            while x <= x2 {
                cave_set_feat(yval, x, feat_wall_inner as libc::c_int);
                x += 1
            }
            /* Doors into the rooms */
            if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                let mut i: libc::c_int =
                    Rand_div(10 as libc::c_int) + 1 as libc::c_int;
                place_secret_door(y1 - 1 as libc::c_int, xval - i);
                place_secret_door(y1 - 1 as libc::c_int, xval + i);
                place_secret_door(y2 + 1 as libc::c_int, xval - i);
                place_secret_door(y2 + 1 as libc::c_int, xval + i);
            } else {
                let mut i_0: libc::c_int =
                    Rand_div(3 as libc::c_int) + 1 as libc::c_int;
                place_secret_door(yval + i_0, x1 - 1 as libc::c_int);
                place_secret_door(yval - i_0, x1 - 1 as libc::c_int);
                place_secret_door(yval + i_0, x2 + 1 as libc::c_int);
                place_secret_door(yval - i_0, x2 + 1 as libc::c_int);
            }
            /* Treasure, centered at the center of the cross */
            vault_objects(yval, xval,
                          2 as libc::c_int +
                              (Rand_div(2 as libc::c_int) +
                                   1 as libc::c_int));
            /* Gotta have some monsters. */
            vault_monsters(yval + 1 as libc::c_int, xval - 4 as libc::c_int,
                           Rand_div(4 as libc::c_int) + 1 as libc::c_int);
            vault_monsters(yval + 1 as libc::c_int, xval + 4 as libc::c_int,
                           Rand_div(4 as libc::c_int) + 1 as libc::c_int);
            vault_monsters(yval - 1 as libc::c_int, xval - 4 as libc::c_int,
                           Rand_div(4 as libc::c_int) + 1 as libc::c_int);
            vault_monsters(yval - 1 as libc::c_int, xval + 4 as libc::c_int,
                           Rand_div(4 as libc::c_int) + 1 as libc::c_int);
        }
        _ => { }
    };
}
/*
 * Determine if the given monster is appropriate for inclusion in
 * a monster nest or monster pit or the given type.
 *
 * None of the pits/nests are allowed to include "unique" monsters,
 * or monsters which can "multiply".
 *
 * Some of the pits/nests are asked to avoid monsters which can blink
 * away or which are invisible.  This is probably a hack.
 *
 * The old method made direct use of monster "names", which is bad.
 *
 * Note the use of Angband 2.7.9 monster race pictures in various places.
 */
/*
 * Helper function for "monster nest (jelly)"
 */
unsafe extern "C" fn vault_aux_jelly(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Also decline evil jellies (like death molds and shoggoths) */
    if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Require icky thing, jelly, mold, or mushroom */
    if strchr(b"ijm,\x00" as *const u8 as *const libc::c_char,
              (*r_ptr).d_char as libc::c_int).is_null() {
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Helper function for "monster nest (animal)"
 */
unsafe extern "C" fn vault_aux_animal(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Require "animal" flag */
    if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Helper function for "monster nest (undead)"
 */
unsafe extern "C" fn vault_aux_undead(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Require Undead */
    if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Helper function for "monster nest (chapel)"
 */
unsafe extern "C" fn vault_aux_chapel(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Require "priest" or Angel */
    if !((*r_ptr).d_char as libc::c_int == 'A' as i32 ||
             !strstr(r_name.offset((*r_ptr).name as isize),
                     b"riest\x00" as *const u8 as
                         *const libc::c_char).is_null()) {
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Helper function for "monster nest (kennel)"
 */
unsafe extern "C" fn vault_aux_kennel(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Require a Zephyr Hound or a dog */
    return ((*r_ptr).d_char as libc::c_int == 'Z' as i32 ||
                (*r_ptr).d_char as libc::c_int == 'C' as i32) as libc::c_int
               as bool_;
}
/*
 * Helper function for "monster nest (treasure)"
 */
unsafe extern "C" fn vault_aux_treasure(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Require "priest" or Angel */
    if !((*r_ptr).d_char as libc::c_int == '!' as i32 ||
             (*r_ptr).d_char as libc::c_int == '|' as i32 ||
             (*r_ptr).d_char as libc::c_int == '$' as i32 ||
             (*r_ptr).d_char as libc::c_int == '?' as i32 ||
             (*r_ptr).d_char as libc::c_int == '=' as i32) {
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Helper function for "monster nest (clone)"
 */
unsafe extern "C" fn vault_aux_clone(mut r_idx: libc::c_int) -> bool_ {
    return (r_idx == template_race) as libc::c_int as bool_;
}
/*
 * Helper function for "monster nest (symbol clone)"
 */
unsafe extern "C" fn vault_aux_symbol(mut r_idx: libc::c_int) -> bool_ {
    return ((*r_info.offset(r_idx as isize)).d_char as libc::c_int ==
                (*r_info.offset(template_race as isize)).d_char as libc::c_int
                &&
                (*r_info.offset(r_idx as isize)).flags1 &
                    0x1 as libc::c_int as libc::c_uint == 0) as libc::c_int as
               bool_;
}
/*
 * Helper function for "monster pit (orc)"
 */
unsafe extern "C" fn vault_aux_orc(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Require "o" monsters */
    if strchr(b"o\x00" as *const u8 as *const libc::c_char,
              (*r_ptr).d_char as libc::c_int).is_null() {
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Helper function for "monster pit (troll)"
 */
unsafe extern "C" fn vault_aux_troll(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Require "T" monsters */
    if strchr(b"T\x00" as *const u8 as *const libc::c_char,
              (*r_ptr).d_char as libc::c_int).is_null() {
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Helper function for "monster pit (giant)"
 */
unsafe extern "C" fn vault_aux_giant(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Require "P" monsters */
    if strchr(b"P\x00" as *const u8 as *const libc::c_char,
              (*r_ptr).d_char as libc::c_int).is_null() {
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Hack -- breath type for "vault_aux_dragon()"
 */
static mut vault_aux_dragon_mask4: u32b = 0;
/*
 * Helper function for "monster pit (dragon)"
 */
unsafe extern "C" fn vault_aux_dragon(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Require "d" or "D" monsters */
    if strchr(b"Dd\x00" as *const u8 as *const libc::c_char,
              (*r_ptr).d_char as libc::c_int).is_null() {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Require correct "breath attack" */
    if (*r_ptr).flags4 != vault_aux_dragon_mask4 {
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Helper function for "monster pit (demon)"
 */
unsafe extern "C" fn vault_aux_demon(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Decline unique monsters */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Hack -- Require "U" monsters */
    if strchr(b"U\x00" as *const u8 as *const libc::c_char,
              (*r_ptr).d_char as libc::c_int).is_null() {
        return 0 as libc::c_int as bool_
    }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Type 5 -- Monster nests
 *
 * A monster nest is a "big" room, with an "inner" room, containing
 * a "collection" of monsters of a given type strewn about the room.
 *
 * The monsters are chosen from a set of 64 randomly selected monster
 * races, to allow the nest creation to fail instead of having "holes".
 *
 * Note the use of the "get_mon_num_prep()" function, and the special
 * "get_mon_num_hook()" restriction function, to prepare the "monster
 * allocation table" in such a way as to optimize the selection of
 * "appropriate" non-unique monsters for the nest.
 *
 * Currently, a monster nest is one of
 *   a nest of "jelly" monsters   (Dungeon level 5 and deeper)
 *   a nest of "animal" monsters  (Dungeon level 30 and deeper)
 *   a nest of "undead" monsters  (Dungeon level 50 and deeper)
 *
 * Note that the "get_mon_num()" function may (rarely) fail, in which
 * case the nest will be empty, and will not affect the level rating.
 *
 * Note that "monster nests" will never contain "unique" monsters.
 */
unsafe extern "C" fn build_type5(mut by0: libc::c_int, mut bx0: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut xval: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut name: cptr = 0 as *const libc::c_char;
    let mut empty: bool_ = 0 as libc::c_int as bool_;
    let mut old_get_mon_num_hook:
            Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> = None;
    let mut what: [s16b; 64] = [0; 64];
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc(25 as libc::c_int, 11 as libc::c_int,
                  1 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        return
    }
    /* Large room */
    y1 = yval - 4 as libc::c_int;
    y2 = yval + 4 as libc::c_int;
    x1 = xval - 11 as libc::c_int;
    x2 = xval + 11 as libc::c_int;
    /* Place the floor area */
    y = y1;
    while y <= y2 {
        x = x1;
        while x <= x2 {
            place_floor(y, x);
            let ref mut fresh13 = (*cave[y as usize].offset(x as isize)).info;
            *fresh13 = (*fresh13 as libc::c_int | 0x8 as libc::c_int) as u16b;
            x += 1
        }
        y += 1
    }
    /* Place the outer walls */
    build_rectangle(y1 - 1 as libc::c_int, x1 - 1 as libc::c_int,
                    y2 + 1 as libc::c_int, x2 + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int, 0x8 as libc::c_int);
    /* Advance to the center room */
    y1 = y1 + 2 as libc::c_int;
    y2 = y2 - 2 as libc::c_int;
    x1 = x1 + 2 as libc::c_int;
    x2 = x2 - 2 as libc::c_int;
    /* The inner walls */
    build_rectangle(y1 - 1 as libc::c_int, x1 - 1 as libc::c_int,
                    y2 + 1 as libc::c_int, x2 + 1 as libc::c_int,
                    feat_wall_inner as libc::c_int, 0x8 as libc::c_int);
    /* Place a secret door */
    match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
        1 => { place_secret_door(y1 - 1 as libc::c_int, xval); }
        2 => { place_secret_door(y2 + 1 as libc::c_int, xval); }
        3 => { place_secret_door(yval, x1 - 1 as libc::c_int); }
        4 => { place_secret_door(yval, x2 + 1 as libc::c_int); }
        _ => { }
    }
    /* Hack -- Choose a nest type */
    tmp = Rand_div(dun_level as s32b) + 1 as libc::c_int;
    old_get_mon_num_hook = get_mon_num_hook;
    if tmp < 25 as libc::c_int &&
           Rand_div(2 as libc::c_int) != 0 as libc::c_int {
        loop  {
            template_race =
                Rand_div(max_r_idx as libc::c_int - 2 as libc::c_int) +
                    1 as libc::c_int;
            /* Reject uniques */
            if (*r_info.offset(template_race as isize)).flags1 &
                   0x1 as libc::c_int as libc::c_uint != 0 {
                continue ;
            }
            /* Reject OoD monsters in a loose fashion */
            if !((*r_info.offset(template_race as isize)).level as libc::c_int
                     + (Rand_div(5 as libc::c_int) + 1 as libc::c_int) >
                     dun_level as libc::c_int +
                         (Rand_div(5 as libc::c_int) + 1 as libc::c_int)) {
                break ;
            }
        }
        if dun_level as libc::c_int >=
               25 as libc::c_int +
                   (Rand_div(15 as libc::c_int) + 1 as libc::c_int) &&
               Rand_div(2 as libc::c_int) != 0 as libc::c_int {
            name = b"symbol clone\x00" as *const u8 as *const libc::c_char;
            get_mon_num_hook =
                Some(vault_aux_symbol as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_)
        } else {
            name = b"clone\x00" as *const u8 as *const libc::c_char;
            get_mon_num_hook =
                Some(vault_aux_clone as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_)
        }
    } else if tmp < 25 as libc::c_int {
        /* Monster nest (jelly) */
        /* Describe */
        name = b"jelly\x00" as *const u8 as *const libc::c_char;
        get_mon_num_hook =
            Some(vault_aux_jelly as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_)
    } else if tmp < 50 as libc::c_int {
        name = b"treasure\x00" as *const u8 as *const libc::c_char;
        get_mon_num_hook =
            Some(vault_aux_treasure as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_)
    } else if tmp < 65 as libc::c_int {
        if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
            name = b"kennel\x00" as *const u8 as *const libc::c_char;
            get_mon_num_hook =
                Some(vault_aux_kennel as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_)
        } else {
            /* Restrict to jelly */
            /* Monster nest (animal) */
            /* Describe */
            name = b"animal\x00" as *const u8 as *const libc::c_char;
            /* Restrict to animal */
            get_mon_num_hook =
                Some(vault_aux_animal as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_)
        }
    } else if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
        name = b"chapel\x00" as *const u8 as *const libc::c_char;
        get_mon_num_hook =
            Some(vault_aux_chapel as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_)
    } else {
        /* Monster nest (undead) */
        /* Describe */
        name = b"undead\x00" as *const u8 as *const libc::c_char;
        /* Restrict to undead */
        get_mon_num_hook =
            Some(vault_aux_undead as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_)
    }
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Pick some monster types */
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        /* Get a (hard) monster type */
        what[i as usize] =
            get_mon_num(dun_level as libc::c_int + 10 as libc::c_int);
        /* Notice failure */
        if what[i as usize] == 0 { empty = 1 as libc::c_int as bool_ }
        i += 1
    }
    /* Remove restriction */
    get_mon_num_hook = old_get_mon_num_hook;
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Oops */
    if empty != 0 { return }
    /* Describe */
    if cheat_room as libc::c_int != 0 ||
           (*p_ptr).precognition as libc::c_int != 0 {
        /* Room type */
        msg_format(b"Monster nest (%s)\x00" as *const u8 as
                       *const libc::c_char, name);
    }
    /* Increase the level rating */
    rating = (rating as libc::c_int + 10 as libc::c_int) as s16b;
    /* (Sometimes) Cause a "special feeling" (for "Monster Nests") */
    if dun_level as libc::c_int <= 40 as libc::c_int &&
           (Rand_div(dun_level as libc::c_int * dun_level as libc::c_int +
                         50 as libc::c_int) + 1 as libc::c_int) <
               300 as libc::c_int {
        good_item_flag = 1 as libc::c_int as bool_
    }
    /* Place some monsters */
    y = yval - 2 as libc::c_int;
    while y <= yval + 2 as libc::c_int {
        x = xval - 9 as libc::c_int;
        while x <= xval + 9 as libc::c_int {
            let mut r_idx: libc::c_int =
                what[Rand_div(64 as libc::c_int) as usize] as libc::c_int;
            /* Place that "random" monster (no groups) */
            place_monster_aux(y, x, r_idx, 0 as libc::c_int as bool_,
                              0 as libc::c_int as bool_, -(2 as libc::c_int));
            x += 1
        }
        y += 1
    };
}
/*
 * Type 6 -- Monster pits
 *
 * A monster pit is a "big" room, with an "inner" room, containing
 * a "collection" of monsters of a given type organized in the room.
 *
 * Monster types in the pit  (list out of date...)
 *   orc pit	(Dungeon Level 5 and deeper)
 *   troll pit	(Dungeon Level 20 and deeper)
 *   giant pit	(Dungeon Level 40 and deeper)
 *   dragon pit	(Dungeon Level 60 and deeper)
 *   demon pit	(Dungeon Level 80 and deeper)
 *
 * The inside room in a monster pit appears as shown below, where the
 * actual monsters in each location depend on the type of the pit
 *
 *   #####################
 *   #0000000000000000000#
 *   #0112233455543322110#
 *   #0112233467643322110#
 *   #0112233455543322110#
 *   #0000000000000000000#
 *   #####################
 *
 * Note that the monsters in the pit are now chosen by using "get_mon_num()"
 * to request 16 "appropriate" monsters, sorting them by level, and using
 * the "even" entries in this sorted list for the contents of the pit.
 *
 * Hack -- all of the "dragons" in a "dragon" pit must be the same "color",
 * which is handled by requiring a specific "breath" attack for all of the
 * dragons.  This may include "multi-hued" breath.  Note that "wyrms" may
 * be present in many of the dragon pits, if they have the proper breath.
 *
 * Note the use of the "get_mon_num_prep()" function, and the special
 * "get_mon_num_hook()" restriction function, to prepare the "monster
 * allocation table" in such a way as to optimize the selection of
 * "appropriate" non-unique monsters for the pit.
 *
 * Note that the "get_mon_num()" function may (rarely) fail, in which case
 * the pit will be empty, and will not effect the level rating.
 *
 * Note that "monster pits" will never contain "unique" monsters.
 */
unsafe extern "C" fn build_type6(mut by0: libc::c_int, mut bx0: libc::c_int) {
    let mut tmp: libc::c_int = 0;
    let mut what: [libc::c_int; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut xval: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    let mut empty: bool_ = 0 as libc::c_int as bool_;
    let mut name: cptr = 0 as *const libc::c_char;
    let mut old_get_mon_num_hook:
            Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> = None;
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc(25 as libc::c_int, 11 as libc::c_int,
                  1 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        return
    }
    /* Large room */
    y1 = yval - 4 as libc::c_int;
    y2 = yval + 4 as libc::c_int;
    x1 = xval - 11 as libc::c_int;
    x2 = xval + 11 as libc::c_int;
    /* Place the floor area */
    y = y1 - 1 as libc::c_int;
    while y <= y2 + 1 as libc::c_int {
        x = x1 - 1 as libc::c_int;
        while x <= x2 + 1 as libc::c_int {
            place_floor(y, x);
            let ref mut fresh14 = (*cave[y as usize].offset(x as isize)).info;
            *fresh14 = (*fresh14 as libc::c_int | 0x8 as libc::c_int) as u16b;
            x += 1
        }
        y += 1
    }
    /* Place the outer walls */
    build_rectangle(y1 - 1 as libc::c_int, x1 - 1 as libc::c_int,
                    y2 + 1 as libc::c_int, x2 + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int, 0x8 as libc::c_int);
    /* Advance to the center room */
    y1 = y1 + 2 as libc::c_int;
    y2 = y2 - 2 as libc::c_int;
    x1 = x1 + 2 as libc::c_int;
    x2 = x2 - 2 as libc::c_int;
    /* The inner walls */
    build_rectangle(y1 - 1 as libc::c_int, x1 - 1 as libc::c_int,
                    y2 + 1 as libc::c_int, x2 + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int, 0x8 as libc::c_int);
    /* Place a secret door */
    match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
        1 => { place_secret_door(y1 - 1 as libc::c_int, xval); }
        2 => { place_secret_door(y2 + 1 as libc::c_int, xval); }
        3 => { place_secret_door(yval, x1 - 1 as libc::c_int); }
        4 => { place_secret_door(yval, x2 + 1 as libc::c_int); }
        _ => { }
    }
    /* Choose a pit type */
    tmp = Rand_div(dun_level as s32b) + 1 as libc::c_int;
    old_get_mon_num_hook = get_mon_num_hook;
    /* Orc pit */
    if tmp < 20 as libc::c_int {
        /* Message */
        name = b"orc\x00" as *const u8 as *const libc::c_char;
        /* Restrict monster selection */
        get_mon_num_hook =
            Some(vault_aux_orc as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_)
    } else if tmp < 40 as libc::c_int {
        /* Troll pit */
        /* Message */
        name = b"troll\x00" as *const u8 as *const libc::c_char;
        /* Restrict monster selection */
        get_mon_num_hook =
            Some(vault_aux_troll as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_)
    } else if tmp < 55 as libc::c_int {
        /* Giant pit */
        /* Message */
        name = b"giant\x00" as *const u8 as *const libc::c_char;
        /* Restrict monster selection */
        get_mon_num_hook =
            Some(vault_aux_giant as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_)
    } else if tmp < 70 as libc::c_int {
        if Rand_div(4 as libc::c_int) + 1 as libc::c_int != 1 as libc::c_int {
            /* Message */
            name = b"ordered clones\x00" as *const u8 as *const libc::c_char;
            loop  {
                template_race =
                    Rand_div(max_r_idx as libc::c_int - 2 as libc::c_int) +
                        1 as libc::c_int;
                if !((*r_info.offset(template_race as isize)).flags1 &
                         0x1 as libc::c_int as libc::c_uint != 0 ||
                         (*r_info.offset(template_race as isize)).level as
                             libc::c_int +
                             (Rand_div(5 as libc::c_int) + 1 as libc::c_int) >
                             dun_level as libc::c_int +
                                 (Rand_div(5 as libc::c_int) +
                                      1 as libc::c_int)) {
                    break ;
                }
            }
            /* Restrict selection */
            get_mon_num_hook =
                Some(vault_aux_symbol as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_)
        } else {
            name = b"ordered chapel\x00" as *const u8 as *const libc::c_char;
            get_mon_num_hook =
                Some(vault_aux_chapel as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_)
        }
    } else if tmp < 80 as libc::c_int {
        /* Dragon pit */
        /* Pick dragon type */
        match Rand_div(6 as libc::c_int) {
            0 => {
                /* Black */
                /* Message */
                name = b"acid dragon\x00" as *const u8 as *const libc::c_char;
                /* Restrict dragon breath type */
                vault_aux_dragon_mask4 = 0x100 as libc::c_int as u32b
            }
            1 => {
                /* Blue */
                /* Message */
                name =
                    b"electric dragon\x00" as *const u8 as
                        *const libc::c_char;
                /* Restrict dragon breath type */
                vault_aux_dragon_mask4 = 0x200 as libc::c_int as u32b
            }
            2 => {
                /* Red */
                /* Message */
                name = b"fire dragon\x00" as *const u8 as *const libc::c_char;
                /* Restrict dragon breath type */
                vault_aux_dragon_mask4 = 0x400 as libc::c_int as u32b
            }
            3 => {
                /* White */
                /* Message */
                name = b"cold dragon\x00" as *const u8 as *const libc::c_char;
                /* Restrict dragon breath type */
                vault_aux_dragon_mask4 = 0x800 as libc::c_int as u32b
            }
            4 => {
                /* Green */
                /* Message */
                name =
                    b"poison dragon\x00" as *const u8 as *const libc::c_char;
                /* Restrict dragon breath type */
                vault_aux_dragon_mask4 = 0x1000 as libc::c_int as u32b
            }
            _ => {
                /* Multi-hued */
                /* Message */
                name =
                    b"multi-hued dragon\x00" as *const u8 as
                        *const libc::c_char;
                /* Restrict dragon breath type */
                vault_aux_dragon_mask4 =
                    (0x100 as libc::c_int | 0x200 as libc::c_int |
                         0x400 as libc::c_int | 0x800 as libc::c_int |
                         0x1000 as libc::c_int) as u32b
            }
        }
        /* Restrict monster selection */
        get_mon_num_hook =
            Some(vault_aux_dragon as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_)
    } else {
        /* Demon pit */
        /* Message */
        name = b"demon\x00" as *const u8 as *const libc::c_char;
        get_mon_num_hook =
            Some(vault_aux_demon as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_)
    }
    /* Restrict monster selection */
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Pick some monster types */
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        /* Get a (hard) monster type */
        what[i as usize] =
            get_mon_num(dun_level as libc::c_int + 10 as libc::c_int) as
                libc::c_int;
        /* Notice failure */
        if what[i as usize] == 0 { empty = 1 as libc::c_int as bool_ }
        i += 1
    }
    /* Remove restriction */
    get_mon_num_hook = old_get_mon_num_hook;
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Oops */
    if empty != 0 { return }
    /* XXX XXX XXX */
	/* Sort the entries */
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int - 1 as libc::c_int {
        /* Sort the entries */
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int - 1 as libc::c_int {
            let mut i1: libc::c_int = j;
            let mut i2: libc::c_int = j + 1 as libc::c_int;
            let mut p1: libc::c_int =
                (*r_info.offset(what[i1 as usize] as isize)).level as
                    libc::c_int;
            let mut p2: libc::c_int =
                (*r_info.offset(what[i2 as usize] as isize)).level as
                    libc::c_int;
            /* Bubble */
            if p1 > p2 {
                let mut tmp_0: libc::c_int = what[i1 as usize];
                what[i1 as usize] = what[i2 as usize];
                what[i2 as usize] = tmp_0
            }
            j += 1
        }
        i += 1
    }
    /* Select the entries */
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        /* Every other entry */
        what[i as usize] = what[(i * 2 as libc::c_int) as usize];
        i += 1
    }
    /* Message */
    if cheat_room as libc::c_int != 0 ||
           (*p_ptr).precognition as libc::c_int != 0 {
        /* Room type */
        msg_format(b"Monster pit (%s)\x00" as *const u8 as
                       *const libc::c_char, name);
        if cheat_hear as libc::c_int != 0 ||
               (*p_ptr).precognition as libc::c_int != 0 {
            /* Contents */
            i = 0 as libc::c_int;
            while i < 8 as libc::c_int {
                /* Message */
                msg_print(r_name.offset((*r_info.offset(what[i as usize] as
                                                            isize)).name as
                                            isize) as cptr);
                i += 1
            }
        }
    }
    /* Increase the level rating */
    rating = (rating as libc::c_int + 10 as libc::c_int) as s16b;
    /* (Sometimes) Cause a "special feeling" (for "Monster Pits") */
    if dun_level as libc::c_int <= 40 as libc::c_int &&
           (Rand_div(dun_level as libc::c_int * dun_level as libc::c_int +
                         50 as libc::c_int) + 1 as libc::c_int) <
               300 as libc::c_int {
        good_item_flag = 1 as libc::c_int as bool_
    }
    /* Top and bottom rows */
    x = xval - 9 as libc::c_int;
    while x <= xval + 9 as libc::c_int {
        place_monster_aux(yval - 2 as libc::c_int, x,
                          what[0 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(yval + 2 as libc::c_int, x,
                          what[0 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        x += 1
    }
    /* Middle columns */
    y = yval - 1 as libc::c_int;
    while y <= yval + 1 as libc::c_int {
        place_monster_aux(y, xval - 9 as libc::c_int,
                          what[0 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval + 9 as libc::c_int,
                          what[0 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval - 8 as libc::c_int,
                          what[1 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval + 8 as libc::c_int,
                          what[1 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval - 7 as libc::c_int,
                          what[1 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval + 7 as libc::c_int,
                          what[1 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval - 6 as libc::c_int,
                          what[2 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval + 6 as libc::c_int,
                          what[2 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval - 5 as libc::c_int,
                          what[2 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval + 5 as libc::c_int,
                          what[2 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval - 4 as libc::c_int,
                          what[3 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval + 4 as libc::c_int,
                          what[3 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval - 3 as libc::c_int,
                          what[3 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval + 3 as libc::c_int,
                          what[3 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval - 2 as libc::c_int,
                          what[4 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(y, xval + 2 as libc::c_int,
                          what[4 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        y += 1
    }
    /* Above/Below the center monster */
    x = xval - 1 as libc::c_int;
    while x <= xval + 1 as libc::c_int {
        place_monster_aux(yval + 1 as libc::c_int, x,
                          what[5 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        place_monster_aux(yval - 1 as libc::c_int, x,
                          what[5 as libc::c_int as usize],
                          0 as libc::c_int as bool_,
                          0 as libc::c_int as bool_, -(2 as libc::c_int));
        x += 1
    }
    /* Next to the center monster */
    place_monster_aux(yval, xval + 1 as libc::c_int,
                      what[6 as libc::c_int as usize],
                      0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                      -(2 as libc::c_int));
    place_monster_aux(yval, xval - 1 as libc::c_int,
                      what[6 as libc::c_int as usize],
                      0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                      -(2 as libc::c_int));
    /* Center monster */
    place_monster_aux(yval, xval, what[7 as libc::c_int as usize],
                      0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                      -(2 as libc::c_int));
}
/*
 * Hack -- fill in "vault" rooms
 */
unsafe extern "C" fn build_vault(mut yval: libc::c_int, mut xval: libc::c_int,
                                 mut ymax: libc::c_int, mut xmax: libc::c_int,
                                 mut data: cptr) {
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut bwy: [libc::c_int; 8] = [0; 8];
    let mut bwx: [libc::c_int; 8] = [0; 8];
    let mut i: libc::c_int = 0;
    let mut t: cptr = 0 as *const libc::c_char;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Clean the between gates arrays */
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        bwx[i as usize] = 9999 as libc::c_int;
        bwy[i as usize] = bwx[i as usize];
        i += 1
    }
    /* Place dungeon features and objects */
    t = data;
    dy = 0 as libc::c_int;
    while dy < ymax {
        dx = 0 as libc::c_int;
        while dx < xmax {
            /* Extract the location */
            x = xval - xmax / 2 as libc::c_int + dx;
            y = yval - ymax / 2 as libc::c_int + dy;
            /* Hack -- skip "non-grids" */
            if !(*t as libc::c_int == ' ' as i32) {
                /* Access the grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(y as
                                                         isize)).offset(x as
                                                                            isize)
                        as *mut cave_type;
                /* Lay down a floor */
                place_floor(y, x);
                /* Part of a vault */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int |
                         (0x8 as libc::c_int | 0x4 as libc::c_int)) as u16b;
                /* Analyze the grid */
                match *t as libc::c_int {
                    37 => {
                        /* Granite wall (outer) */
                        cave_set_feat(y, x, 0x3a as libc::c_int);
                    }
                    35 => {
                        /* Granite wall (inner) */
                        cave_set_feat(y, x, 0x39 as libc::c_int);
                    }
                    88 => {
                        /* Permanent wall (inner) */
                        cave_set_feat(y, x, 0x3d as libc::c_int);
                    }
                    42 => {
                        /* Treasure/trap */
                        if Rand_div(100 as libc::c_int) < 75 as libc::c_int {
                            place_object(y, x, 0 as libc::c_int as bool_,
                                         0 as libc::c_int as bool_,
                                         3 as libc::c_int);
                        } else { place_trap(y, x); }
                    }
                    43 => {
                        /* Secret doors */
                        place_secret_door(y, x);
                    }
                    94 => {
                        /* Trap */
                        place_trap(y, x);
                    }
                    71 => {
                        /* Glass wall */
                        cave_set_feat(y, x, 0xbc as libc::c_int);
                    }
                    73 => {
                        /* Illusion wall */
                        cave_set_feat(y, x, 0xbd as libc::c_int);
                    }
                    _ => { }
                }
            }
            dx += 1;
            t = t.offset(1)
        }
        dy += 1
    }
    /* Place dungeon monsters and objects */
    t = data;
    dy = 0 as libc::c_int;
    while dy < ymax {
        dx = 0 as libc::c_int;
        while dx < xmax {
            /* Extract the grid */
            x = xval - xmax / 2 as libc::c_int + dx;
            y = yval - ymax / 2 as libc::c_int + dy;
            /* Hack -- skip "non-grids" */
            if !(*t as libc::c_int == ' ' as i32) {
                /* Access the grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(y as
                                                         isize)).offset(x as
                                                                            isize)
                        as *mut cave_type;
                /* Analyze the symbol */
                match *t as libc::c_int {
                    38 => {
                        /* Monster */
                        monster_level =
                            (dun_level as libc::c_int + 5 as libc::c_int) as
                                s16b;
                        place_monster(y, x, 1 as libc::c_int as bool_,
                                      1 as libc::c_int as bool_);
                        monster_level = dun_level
                    }
                    64 => {
                        /* Meaner monster */
                        monster_level =
                            (dun_level as libc::c_int + 11 as libc::c_int) as
                                s16b;
                        place_monster(y, x, 1 as libc::c_int as bool_,
                                      1 as libc::c_int as bool_);
                        monster_level = dun_level
                    }
                    57 => {
                        /* Meaner monster, plus treasure */
                        monster_level =
                            (dun_level as libc::c_int + 9 as libc::c_int) as
                                s16b;
                        place_monster(y, x, 1 as libc::c_int as bool_,
                                      1 as libc::c_int as bool_);
                        monster_level = dun_level;
                        object_level =
                            (dun_level as libc::c_int + 7 as libc::c_int) as
                                s16b;
                        place_object(y, x, 1 as libc::c_int as bool_,
                                     0 as libc::c_int as bool_,
                                     3 as libc::c_int);
                        object_level = dun_level
                    }
                    56 => {
                        /* Nasty monster and treasure */
                        monster_level =
                            (dun_level as libc::c_int + 40 as libc::c_int) as
                                s16b;
                        place_monster(y, x, 1 as libc::c_int as bool_,
                                      1 as libc::c_int as bool_);
                        monster_level = dun_level;
                        object_level =
                            (dun_level as libc::c_int + 20 as libc::c_int) as
                                s16b;
                        place_object(y, x, 1 as libc::c_int as bool_,
                                     1 as libc::c_int as bool_,
                                     3 as libc::c_int);
                        object_level = dun_level
                    }
                    44 => {
                        /* Monster and/or object */
                        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                            monster_level =
                                (dun_level as libc::c_int + 3 as libc::c_int)
                                    as s16b;
                            place_monster(y, x, 1 as libc::c_int as bool_,
                                          1 as libc::c_int as bool_);
                            monster_level = dun_level
                        }
                        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                            object_level =
                                (dun_level as libc::c_int + 7 as libc::c_int)
                                    as s16b;
                            place_object(y, x, 0 as libc::c_int as bool_,
                                         0 as libc::c_int as bool_,
                                         3 as libc::c_int);
                            object_level = dun_level
                        }
                    }
                    112 => { cave_set_feat(y, x, 0x41 as libc::c_int); }
                    97 => { cave_set_feat(y, x, 0x42 as libc::c_int); }
                    98 => { cave_set_feat(y, x, 0x43 as libc::c_int); }
                    99 => { cave_set_feat(y, x, 0x44 as libc::c_int); }
                    100 => { cave_set_feat(y, x, 0x45 as libc::c_int); }
                    80 => { cave_set_feat(y, x, 0x46 as libc::c_int); }
                    66 => { cave_set_feat(y, x, 0x48 as libc::c_int); }
                    65 => {
                        object_level =
                            (dun_level as libc::c_int + 12 as libc::c_int) as
                                s16b;
                        place_object(y, x, 1 as libc::c_int as bool_,
                                     0 as libc::c_int as bool_,
                                     3 as libc::c_int);
                        object_level = dun_level
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                        /* Between gates */
                        /* Not found before */
                        if bwy[(*t as libc::c_int - '0' as i32) as usize] ==
                               9999 as libc::c_int {
                            cave_set_feat(y, x, 0xa0 as libc::c_int);
                            bwy[(*t as libc::c_int - '0' as i32) as usize] =
                                y;
                            bwx[(*t as libc::c_int - '0' as i32) as usize] = x
                        } else {
                            /* The second time */
                            cave_set_feat(y, x, 0xa0 as libc::c_int);
                            (*c_ptr).special =
                                (bwx[(*t as libc::c_int - '0' as i32) as
                                         usize] +
                                     (bwy[(*t as libc::c_int - '0' as i32) as
                                              usize] << 8 as libc::c_int)) as
                                    s16b;
                            (*cave[bwy[(*t as libc::c_int - '0' as i32) as
                                           usize] as
                                       usize].offset(bwx[(*t as libc::c_int -
                                                              '0' as i32) as
                                                             usize] as
                                                         isize)).special =
                                (x + (y << 8 as libc::c_int)) as s16b
                        }
                    }
                    _ => { }
                }
            }
            dx += 1;
            t = t.offset(1)
        }
        dy += 1
    };
}
/*
 * Type 7 -- simple vaults (see "v_info.txt")
 */
unsafe extern "C" fn build_type7(mut by0: libc::c_int, mut bx0: libc::c_int) {
    let mut v_ptr: *mut vault_type = 0 as *mut vault_type;
    let mut dummy: libc::c_int = 0 as libc::c_int;
    let mut xval: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    /* Pick a lesser vault */
    while dummy < 5000 as libc::c_int {
        dummy += 1;
        /* Access a random vault record */
        v_ptr =
            &mut *v_info.offset((Rand_div as
                                     unsafe extern "C" fn(_: s32b)
                                         -> s32b)(max_v_idx as s32b) as isize)
                as *mut vault_type;
        /* Accept the first lesser vault */
        if (*v_ptr).typ as libc::c_int == 7 as libc::c_int { break ; }
    }
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc((*v_ptr).wid as libc::c_int, (*v_ptr).hgt as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        if cheat_room != 0 {
            msg_print(b"Could not allocate this vault here\x00" as *const u8
                          as *const libc::c_char);
        }
        return
    }
    if dummy >= 5000 as libc::c_int {
        if cheat_room != 0 {
            msg_print(b"Warning! Could not place lesser vault!\x00" as
                          *const u8 as *const libc::c_char);
        }
        return
    }
    /* Message */
    if cheat_room as libc::c_int != 0 ||
           (*p_ptr).precognition as libc::c_int != 0 {
        msg_print(b"Lesser Vault\x00" as *const u8 as *const libc::c_char);
    }
    /* Boost the rating */
    rating = (rating as libc::c_int + (*v_ptr).rat as libc::c_int) as s16b;
    /* (Sometimes) Cause a special feeling */
    if dun_level as libc::c_int <= 50 as libc::c_int ||
           (Rand_div((dun_level as libc::c_int - 40 as libc::c_int) *
                         (dun_level as libc::c_int - 40 as libc::c_int) +
                         50 as libc::c_int) + 1 as libc::c_int) <
               400 as libc::c_int {
        good_item_flag = 1 as libc::c_int as bool_
    }
    /* Hack -- Build the vault */
    build_vault(yval, xval, (*v_ptr).hgt as libc::c_int,
                (*v_ptr).wid as libc::c_int,
                v_text.offset((*v_ptr).text as isize) as cptr);
}
/*
 * Type 8 -- greater vaults (see "v_info.txt")
 */
unsafe extern "C" fn build_type8(mut by0: libc::c_int, mut bx0: libc::c_int) {
    let mut v_ptr: *mut vault_type = 0 as *mut vault_type;
    let mut dummy: libc::c_int = 0 as libc::c_int;
    let mut xval: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    /* Pick a lesser vault */
    while dummy < 5000 as libc::c_int {
        dummy += 1;
        /* Access a random vault record */
        v_ptr =
            &mut *v_info.offset((Rand_div as
                                     unsafe extern "C" fn(_: s32b)
                                         -> s32b)(max_v_idx as s32b) as isize)
                as *mut vault_type;
        /* Accept the first greater vault */
        if (*v_ptr).typ as libc::c_int == 8 as libc::c_int { break ; }
    }
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc((*v_ptr).wid as libc::c_int, (*v_ptr).hgt as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        if cheat_room != 0 {
            msg_print(b"Could not allocate this vault here\x00" as *const u8
                          as *const libc::c_char);
        }
        return
    }
    if dummy >= 5000 as libc::c_int {
        if cheat_room != 0 {
            msg_print(b"Warning! Could not place greater vault!\x00" as
                          *const u8 as *const libc::c_char);
        }
        return
    }
    /* Message */
    if cheat_room as libc::c_int != 0 ||
           (*p_ptr).precognition as libc::c_int != 0 {
        msg_print(b"Greater Vault\x00" as *const u8 as *const libc::c_char);
    }
    /* Boost the rating */
    rating = (rating as libc::c_int + (*v_ptr).rat as libc::c_int) as s16b;
    /* (Sometimes) Cause a special feeling */
    if dun_level as libc::c_int <= 50 as libc::c_int ||
           (Rand_div((dun_level as libc::c_int - 40 as libc::c_int) *
                         (dun_level as libc::c_int - 40 as libc::c_int) +
                         50 as libc::c_int) + 1 as libc::c_int) <
               400 as libc::c_int {
        good_item_flag = 1 as libc::c_int as bool_
    }
    /* Hack -- Build the vault */
    build_vault(yval, xval, (*v_ptr).hgt as libc::c_int,
                (*v_ptr).wid as libc::c_int,
                v_text.offset((*v_ptr).text as isize) as cptr);
}
/*
 * DAG:
 * Build an vertical oval room.
 * For every grid in the possible square, check the distance.
 * If it's less than or == than the radius, make it a room square.
 * If its less, make it a normal grid. If it's == make it an outer
 * wall.
 */
unsafe extern "C" fn build_type9(mut by0: libc::c_int, mut bx0: libc::c_int) {
    let mut info: u16b = 0;
    let mut rad: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    rad = 2 as libc::c_int + Rand_div(8 as libc::c_int);
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc(rad * 2 as libc::c_int + 1 as libc::c_int,
                  rad * 2 as libc::c_int + 1 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut x0, &mut y0) == 0
       {
        return
    }
    info =
        if Rand_div(dun_level as s32b) + 1 as libc::c_int <= 5 as libc::c_int
           {
            (0x8 as libc::c_int) | 0x2 as libc::c_int
        } else { 0x8 as libc::c_int } as u16b;
    x = x0 - rad;
    while x <= x0 + rad {
        y = y0 - rad;
        while y <= y0 + rad {
            if distance(y0, x0, y, x) == rad {
                let ref mut fresh15 =
                    (*cave[y as usize].offset(x as isize)).info;
                *fresh15 =
                    (*fresh15 as libc::c_int | info as libc::c_int) as u16b;
                cave_set_feat(y, x, feat_wall_outer as libc::c_int);
            }
            if distance(y0, x0, y, x) < rad {
                let ref mut fresh16 =
                    (*cave[y as usize].offset(x as isize)).info;
                *fresh16 =
                    (*fresh16 as libc::c_int | info as libc::c_int) as u16b;
                place_floor(y, x);
            }
            y += 1
        }
        x += 1
    };
}
/*
 * Store routine for the fractal cave generator
 * this routine probably should be an inline function or a macro
 */
unsafe extern "C" fn store_height(mut x: libc::c_int, mut y: libc::c_int,
                                  mut x0: libc::c_int, mut y0: libc::c_int,
                                  mut val: byte_hack, mut xhsize: libc::c_int,
                                  mut yhsize: libc::c_int,
                                  mut cutoff: libc::c_int) {
    /* Only write to points that are "blank" */
    if (*cave[(y + y0 - yhsize) as
                  usize].offset((x + x0 - xhsize) as isize)).feat as
           libc::c_int != 255 as libc::c_int {
        return
    }
    /* If on boundary set val > cutoff so walls are not as square */
    if (x == 0 as libc::c_int || y == 0 as libc::c_int ||
            x == xhsize * 2 as libc::c_int || y == yhsize * 2 as libc::c_int)
           && val as libc::c_int <= cutoff {
        val = (cutoff + 1 as libc::c_int) as byte_hack
    }
    /* Store the value in height-map format */
	/* Meant to be temporary, hence no cave_set_feat */
    (*cave[(y + y0 - yhsize) as
               usize].offset((x + x0 - xhsize) as isize)).feat = val;
}
/*
 * Explanation of the plasma fractal algorithm:
 *
 * A grid of points is created with the properties of a 'height-map'
 * This is done by making the corners of the grid have a random value.
 * The grid is then subdivided into one with twice the resolution.
 * The new points midway between two 'known' points can be calculated
 * by taking the average value of the 'known' ones and randomly adding
 * or subtracting an amount proportional to the distance between those
 * points.  The final 'middle' points of the grid are then calculated
 * by averaging all four of the originally 'known' corner points.  An
 * random amount is added or subtracted from this to get a value of the
 * height at that point.  The scaling factor here is adjusted to the
 * slightly larger distance diagonally as compared to orthogonally.
 *
 * This is then repeated recursively to fill an entire 'height-map'
 * A rectangular map is done the same way, except there are different
 * scaling factors along the x and y directions.
 *
 * A hack to change the amount of correlation between points is done using
 * the grd variable.  If the current step size is greater than grd then
 * the point will be random, otherwise it will be calculated by the
 * above algorithm.  This makes a maximum distance at which two points on
 * the height map can affect each other.
 *
 * How fractal caves are made:
 *
 * When the map is complete, a cut-off value is used to create a cave.
 * Heights below this value are "floor", and heights above are "wall".
 * This also can be used to create lakes, by adding more height levels
 * representing shallow and deep water/ lava etc.
 *
 * The grd variable affects the width of passages.
 * The roug variable affects the roughness of those passages
 *
 * The tricky part is making sure the created cave is connected.  This
 * is done by 'filling' from the inside and only keeping the 'filled'
 * floor.  Walls bounding the 'filled' floor are also kept.  Everything
 * else is converted to the normal granite FEAT_WALL_EXTRA.
 */
/*
 * Note that this uses the cave.feat array in a very hackish way
 * the values are first set to zero, and then each array location
 * is used as a "heightmap"
 * The heightmap then needs to be converted back into the "feat" format.
 *
 * grd=level at which fractal turns on.  smaller gives more mazelike caves
 * roug=roughness level.  16=normal.  higher values make things more
 * convoluted small values are good for smooth walls.
 * size=length of the side of the square cave system.
 */
#[no_mangle]
pub unsafe extern "C" fn generate_hmap(mut y0: libc::c_int,
                                       mut x0: libc::c_int,
                                       mut xsiz: libc::c_int,
                                       mut ysiz: libc::c_int,
                                       mut grd: libc::c_int,
                                       mut roug: libc::c_int,
                                       mut cutoff: libc::c_int) {
    let mut xhsize: libc::c_int = 0;
    let mut yhsize: libc::c_int = 0;
    let mut xsize: libc::c_int = 0;
    let mut ysize: libc::c_int = 0;
    let mut maxsize: libc::c_int = 0;
    /*
	 * fixed point variables- these are stored as 256 x normal value
	 * this gives 8 binary places of fractional part + 8 places of normal part
	 */
    let mut xstep: u16b = 0;
    let mut xhstep: u16b = 0;
    let mut ystep: u16b = 0;
    let mut yhstep: u16b = 0;
    let mut i: u16b = 0;
    let mut j: u16b = 0;
    let mut diagsize: u16b = 0;
    let mut xxsize: u16b = 0;
    let mut yysize: u16b = 0;
    /* Redefine size so can change the value if out of range */
    xsize = xsiz;
    ysize = ysiz;
    /* Paranoia about size of the system of caves*/
    if xsize > 254 as libc::c_int { xsize = 254 as libc::c_int }
    if xsize < 4 as libc::c_int { xsize = 4 as libc::c_int }
    if ysize > 254 as libc::c_int { ysize = 254 as libc::c_int }
    if ysize < 4 as libc::c_int { ysize = 4 as libc::c_int }
    /* Get offsets to middle of array */
    xhsize = xsize / 2 as libc::c_int;
    yhsize = ysize / 2 as libc::c_int;
    /* Fix rounding problem */
    xsize = xhsize * 2 as libc::c_int;
    ysize = yhsize * 2 as libc::c_int;
    /*
	 * Scale factor for middle points:
	 * About sqrt(2)*256 - correct for a square lattice
	 * approximately correct for everything else.
	 */
    diagsize = 362 as libc::c_int as u16b;
    /* Maximum of xsize and ysize */
    maxsize = if xsize > ysize { xsize } else { ysize };
    /* Clear the section */
    i = 0 as libc::c_int as u16b;
    while i as libc::c_int <= xsize {
        j = 0 as libc::c_int as u16b;
        while j as libc::c_int <= ysize {
            let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
            /* Access the grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset((j as libc::c_int + y0 -
                                                      yhsize) as
                                                     isize)).offset((i as
                                                                         libc::c_int
                                                                         + x0
                                                                         -
                                                                         xhsize)
                                                                        as
                                                                        isize)
                    as *mut cave_type;
            /* 255 is a flag for "not done yet" */
            (*c_ptr).feat = 255 as libc::c_int as byte_hack;
            /* Clear icky flag because may be redoing the cave */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int & !(0x4 as libc::c_int)) as
                    u16b;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    /* Set the corner values just in case grd>size. */
    store_height(0 as libc::c_int, 0 as libc::c_int, x0, y0,
                 maxsize as byte_hack, xhsize, yhsize, cutoff);
    store_height(0 as libc::c_int, ysize, x0, y0, maxsize as byte_hack,
                 xhsize, yhsize, cutoff);
    store_height(xsize, 0 as libc::c_int, x0, y0, maxsize as byte_hack,
                 xhsize, yhsize, cutoff);
    store_height(xsize, ysize, x0, y0, maxsize as byte_hack, xhsize, yhsize,
                 cutoff);
    /* Set the middle square to be an open area. */
    store_height(xhsize, yhsize, x0, y0, 0 as libc::c_int as byte_hack,
                 xhsize, yhsize, cutoff);
    /* Initialise the step sizes */
    xhstep = (xsize * 256 as libc::c_int) as u16b;
    xstep = xhstep;
    yhstep = (ysize * 256 as libc::c_int) as u16b;
    ystep = yhstep;
    xxsize = (xsize * 256 as libc::c_int) as u16b;
    yysize = (ysize * 256 as libc::c_int) as u16b;
    /*
	 * Fill in the rectangle with fractal height data - like the
	 * 'plasma fractal' in fractint
	 */
    while xstep as libc::c_int / 256 as libc::c_int > 1 as libc::c_int ||
              ystep as libc::c_int / 256 as libc::c_int > 1 as libc::c_int {
        /* Halve the step sizes */
        xstep = xhstep;
        xhstep = (xhstep as libc::c_int / 2 as libc::c_int) as u16b;
        ystep = yhstep;
        yhstep = (yhstep as libc::c_int / 2 as libc::c_int) as u16b;
        /* Middle top to bottom */
        i = xhstep;
        while i as libc::c_int <=
                  xxsize as libc::c_int - xhstep as libc::c_int {
            j = 0 as libc::c_int as u16b;
            while j as libc::c_int <= yysize as libc::c_int {
                /* If greater than 'grid' level then is random */
                if xhstep as libc::c_int / 256 as libc::c_int > grd {
                    store_height(i as libc::c_int / 256 as libc::c_int,
                                 j as libc::c_int / 256 as libc::c_int, x0,
                                 y0,
                                 (Rand_div(maxsize) + 1 as libc::c_int) as
                                     byte_hack, xhsize, yhsize, cutoff);
                } else {
                    let mut l: *mut cave_type = 0 as *mut cave_type;
                    let mut r: *mut cave_type = 0 as *mut cave_type;
                    let mut val: byte_hack = 0;
                    /* Left point */
                    l =
                        &mut *(*cave.as_mut_ptr().offset((j as libc::c_int /
                                                              256 as
                                                                  libc::c_int
                                                              + y0 - yhsize)
                                                             as
                                                             isize)).offset(((i
                                                                                  as
                                                                                  libc::c_int
                                                                                  -
                                                                                  xhstep
                                                                                      as
                                                                                      libc::c_int)
                                                                                 /
                                                                                 256
                                                                                     as
                                                                                     libc::c_int
                                                                                 +
                                                                                 x0
                                                                                 -
                                                                                 xhsize)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Right point */
                    r =
                        &mut *(*cave.as_mut_ptr().offset((j as libc::c_int /
                                                              256 as
                                                                  libc::c_int
                                                              + y0 - yhsize)
                                                             as
                                                             isize)).offset(((i
                                                                                  as
                                                                                  libc::c_int
                                                                                  +
                                                                                  xhstep
                                                                                      as
                                                                                      libc::c_int)
                                                                                 /
                                                                                 256
                                                                                     as
                                                                                     libc::c_int
                                                                                 +
                                                                                 x0
                                                                                 -
                                                                                 xhsize)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Average of left and right points + random bit */
                    val =
                        (((*l).feat as libc::c_int + (*r).feat as libc::c_int)
                             / 2 as libc::c_int +
                             (Rand_div(xstep as libc::c_int /
                                           256 as libc::c_int) +
                                  1 as libc::c_int -
                                  xhstep as libc::c_int / 256 as libc::c_int)
                                 * roug / 16 as libc::c_int) as byte_hack;
                    store_height(i as libc::c_int / 256 as libc::c_int,
                                 j as libc::c_int / 256 as libc::c_int, x0,
                                 y0, val, xhsize, yhsize, cutoff);
                }
                j = (j as libc::c_int + ystep as libc::c_int) as u16b
            }
            i = (i as libc::c_int + xstep as libc::c_int) as u16b
        }
        /* Middle left to right */
        j = yhstep;
        while j as libc::c_int <=
                  yysize as libc::c_int - yhstep as libc::c_int {
            i = 0 as libc::c_int as u16b;
            while i as libc::c_int <= xxsize as libc::c_int {
                /* If greater than 'grid' level then is random */
                if xhstep as libc::c_int / 256 as libc::c_int > grd {
                    store_height(i as libc::c_int / 256 as libc::c_int,
                                 j as libc::c_int / 256 as libc::c_int, x0,
                                 y0,
                                 (Rand_div(maxsize) + 1 as libc::c_int) as
                                     byte_hack, xhsize, yhsize, cutoff);
                } else {
                    let mut u: *mut cave_type = 0 as *mut cave_type;
                    let mut d: *mut cave_type = 0 as *mut cave_type;
                    let mut val_0: byte_hack = 0;
                    /* Up point */
                    u =
                        &mut *(*cave.as_mut_ptr().offset(((j as libc::c_int -
                                                               yhstep as
                                                                   libc::c_int)
                                                              /
                                                              256 as
                                                                  libc::c_int
                                                              + y0 - yhsize)
                                                             as
                                                             isize)).offset((i
                                                                                 as
                                                                                 libc::c_int
                                                                                 /
                                                                                 256
                                                                                     as
                                                                                     libc::c_int
                                                                                 +
                                                                                 x0
                                                                                 -
                                                                                 xhsize)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Down point */
                    d =
                        &mut *(*cave.as_mut_ptr().offset(((j as libc::c_int +
                                                               yhstep as
                                                                   libc::c_int)
                                                              /
                                                              256 as
                                                                  libc::c_int
                                                              + y0 - yhsize)
                                                             as
                                                             isize)).offset((i
                                                                                 as
                                                                                 libc::c_int
                                                                                 /
                                                                                 256
                                                                                     as
                                                                                     libc::c_int
                                                                                 +
                                                                                 x0
                                                                                 -
                                                                                 xhsize)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Average of up and down points + random bit */
                    val_0 =
                        (((*u).feat as libc::c_int + (*d).feat as libc::c_int)
                             / 2 as libc::c_int +
                             (Rand_div(ystep as libc::c_int /
                                           256 as libc::c_int) +
                                  1 as libc::c_int -
                                  yhstep as libc::c_int / 256 as libc::c_int)
                                 * roug / 16 as libc::c_int) as byte_hack;
                    store_height(i as libc::c_int / 256 as libc::c_int,
                                 j as libc::c_int / 256 as libc::c_int, x0,
                                 y0, val_0, xhsize, yhsize, cutoff);
                }
                i = (i as libc::c_int + xstep as libc::c_int) as u16b
            }
            j = (j as libc::c_int + ystep as libc::c_int) as u16b
        }
        /* Center */
        i = xhstep;
        while i as libc::c_int <=
                  xxsize as libc::c_int - xhstep as libc::c_int {
            j = yhstep;
            while j as libc::c_int <=
                      yysize as libc::c_int - yhstep as libc::c_int {
                /* If greater than 'grid' level then is random */
                if xhstep as libc::c_int / 256 as libc::c_int > grd {
                    store_height(i as libc::c_int / 256 as libc::c_int,
                                 j as libc::c_int / 256 as libc::c_int, x0,
                                 y0,
                                 (Rand_div(maxsize) + 1 as libc::c_int) as
                                     byte_hack, xhsize, yhsize, cutoff);
                } else {
                    let mut ul: *mut cave_type = 0 as *mut cave_type;
                    let mut dl: *mut cave_type = 0 as *mut cave_type;
                    let mut ur: *mut cave_type = 0 as *mut cave_type;
                    let mut dr: *mut cave_type = 0 as *mut cave_type;
                    let mut val_1: byte_hack = 0;
                    /* Up-left point */
                    ul =
                        &mut *(*cave.as_mut_ptr().offset(((j as libc::c_int -
                                                               yhstep as
                                                                   libc::c_int)
                                                              /
                                                              256 as
                                                                  libc::c_int
                                                              + y0 - yhsize)
                                                             as
                                                             isize)).offset(((i
                                                                                  as
                                                                                  libc::c_int
                                                                                  -
                                                                                  xhstep
                                                                                      as
                                                                                      libc::c_int)
                                                                                 /
                                                                                 256
                                                                                     as
                                                                                     libc::c_int
                                                                                 +
                                                                                 x0
                                                                                 -
                                                                                 xhsize)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Down-left point */
                    dl =
                        &mut *(*cave.as_mut_ptr().offset(((j as libc::c_int +
                                                               yhstep as
                                                                   libc::c_int)
                                                              /
                                                              256 as
                                                                  libc::c_int
                                                              + y0 - yhsize)
                                                             as
                                                             isize)).offset(((i
                                                                                  as
                                                                                  libc::c_int
                                                                                  -
                                                                                  xhstep
                                                                                      as
                                                                                      libc::c_int)
                                                                                 /
                                                                                 256
                                                                                     as
                                                                                     libc::c_int
                                                                                 +
                                                                                 x0
                                                                                 -
                                                                                 xhsize)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Up-right point */
                    ur =
                        &mut *(*cave.as_mut_ptr().offset(((j as libc::c_int -
                                                               yhstep as
                                                                   libc::c_int)
                                                              /
                                                              256 as
                                                                  libc::c_int
                                                              + y0 - yhsize)
                                                             as
                                                             isize)).offset(((i
                                                                                  as
                                                                                  libc::c_int
                                                                                  +
                                                                                  xhstep
                                                                                      as
                                                                                      libc::c_int)
                                                                                 /
                                                                                 256
                                                                                     as
                                                                                     libc::c_int
                                                                                 +
                                                                                 x0
                                                                                 -
                                                                                 xhsize)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Down-right point */
                    dr =
                        &mut *(*cave.as_mut_ptr().offset(((j as libc::c_int +
                                                               yhstep as
                                                                   libc::c_int)
                                                              /
                                                              256 as
                                                                  libc::c_int
                                                              + y0 - yhsize)
                                                             as
                                                             isize)).offset(((i
                                                                                  as
                                                                                  libc::c_int
                                                                                  +
                                                                                  xhstep
                                                                                      as
                                                                                      libc::c_int)
                                                                                 /
                                                                                 256
                                                                                     as
                                                                                     libc::c_int
                                                                                 +
                                                                                 x0
                                                                                 -
                                                                                 xhsize)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /*
					 * average over all four corners + scale by diagsize to
					 * reduce the effect of the square grid on the shape
					 * of the fractal
					 */
                    val_1 =
                        (((*ul).feat as libc::c_int +
                              (*dl).feat as libc::c_int +
                              (*ur).feat as libc::c_int +
                              (*dr).feat as libc::c_int) / 4 as libc::c_int +
                             (Rand_div(xstep as libc::c_int /
                                           256 as libc::c_int) +
                                  1 as libc::c_int -
                                  xhstep as libc::c_int / 256 as libc::c_int)
                                 *
                                 (diagsize as libc::c_int / 16 as libc::c_int)
                                 / 256 as libc::c_int * roug) as byte_hack;
                    store_height(i as libc::c_int / 256 as libc::c_int,
                                 j as libc::c_int / 256 as libc::c_int, x0,
                                 y0, val_1, xhsize, yhsize, cutoff);
                }
                j = (j as libc::c_int + ystep as libc::c_int) as u16b
            }
            i = (i as libc::c_int + xstep as libc::c_int) as u16b
        }
    };
}
/*
 * Convert from height-map back to the normal Angband cave format
 */
unsafe extern "C" fn hack_isnt_wall(mut y: libc::c_int, mut x: libc::c_int,
                                    mut cutoff: libc::c_int) -> bool_ {
    /* Already done */
    if (*cave[y as usize].offset(x as isize)).info as libc::c_int &
           0x4 as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    } else {
        /* Show that have looked at this square */
        let ref mut fresh17 = (*cave[y as usize].offset(x as isize)).info;
        *fresh17 = (*fresh17 as libc::c_int | 0x4 as libc::c_int) as u16b;
        /* If less than cutoff then is a floor */
        if (*cave[y as usize].offset(x as isize)).feat as libc::c_int <=
               cutoff {
            place_floor(y, x);
            return 1 as libc::c_int as bool_
        } else {
            /* If greater than cutoff then is a wall */
            cave_set_feat(y, x, feat_wall_outer as libc::c_int);
            return 0 as libc::c_int as bool_
        }
    };
}
/*
 * Quick and nasty fill routine used to find the connected region
 * of floor in the middle of the cave
 */
unsafe extern "C" fn fill_hack(mut y0: libc::c_int, mut x0: libc::c_int,
                               mut y: libc::c_int, mut x: libc::c_int,
                               mut xsize: libc::c_int, mut ysize: libc::c_int,
                               mut cutoff: libc::c_int,
                               mut amount: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* check 8 neighbours +self (self is caught in the isnt_wall function) */
    i = -(1 as libc::c_int);
    while i <= 1 as libc::c_int {
        j = -(1 as libc::c_int);
        while j <= 1 as libc::c_int {
            /* If within bounds */
            if x + i > 0 as libc::c_int && x + i < xsize &&
                   y + j > 0 as libc::c_int && y + j < ysize {
                /* If not a wall or floor done before */
                if hack_isnt_wall(y + j + y0 - ysize / 2 as libc::c_int,
                                  x + i + x0 - xsize / 2 as libc::c_int,
                                  cutoff) != 0 {
                    /* then fill from the new point*/
                    fill_hack(y0, x0, y + j, x + i, xsize, ysize, cutoff,
                              amount);
                    /* keep tally of size of cave system */
                    *amount += 1
                }
            } else {
                /* Affect boundary */
                let ref mut fresh18 =
                    (*cave[(y0 + y + j - ysize / 2 as libc::c_int) as
                               usize].offset((x0 + x + i -
                                                  xsize / 2 as libc::c_int) as
                                                 isize)).info;
                *fresh18 =
                    (*fresh18 as libc::c_int | 0x4 as libc::c_int) as u16b
            }
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn generate_fracave(mut y0: libc::c_int,
                                          mut x0: libc::c_int,
                                          mut xsize: libc::c_int,
                                          mut ysize: libc::c_int,
                                          mut cutoff: libc::c_int,
                                          mut light: bool_, mut room: bool_)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut amount: libc::c_int = 0;
    let mut xhsize: libc::c_int = 0;
    let mut yhsize: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Offsets to middle from corner */
    xhsize = xsize / 2 as libc::c_int;
    yhsize = ysize / 2 as libc::c_int;
    /* Reset tally */
    amount = 0 as libc::c_int;
    /*
	 * Select region connected to center of cave system
	 * this gets rid of alot of isolated one-sqaures that
	 * can make teleport traps instadeaths...
	 */
    fill_hack(y0, x0, yhsize, xhsize, xsize, ysize, cutoff, &mut amount);
    /* If tally too small, try again */
    if amount < 10 as libc::c_int {
        /* Too small -- clear area and try again later */
        x = 0 as libc::c_int;
        while x <= xsize {
            y = 0 as libc::c_int;
            while y < ysize {
                place_filler(y0 + y - yhsize, x0 + x - xhsize);
                let ref mut fresh19 =
                    (*cave[(y0 + y - yhsize) as
                               usize].offset((x0 + x - xhsize) as
                                                 isize)).info;
                *fresh19 =
                    (*fresh19 as libc::c_int &
                         !(0x4 as libc::c_int | 0x8 as libc::c_int)) as u16b;
                y += 1
            }
            x += 1
        }
        return 0 as libc::c_int as bool_
    }
    /*
	 * Do boundaries -- check to see if they are next to a filled region
	 * If not then they are set to normal granite
	 * If so then they are marked as room walls
	 */
    i = 0 as libc::c_int;
    while i <= xsize {
        /* Access top boundary grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset((0 as libc::c_int + y0 - yhsize)
                                                 as
                                                 isize)).offset((i + x0 -
                                                                     xhsize)
                                                                    as isize)
                as *mut cave_type;
        /* Next to a 'filled' region? -- set to be room walls */
        if (*c_ptr).info as libc::c_int & 0x4 as libc::c_int != 0 {
            cave_set_feat(0 as libc::c_int + y0 - yhsize, i + x0 - xhsize,
                          feat_wall_outer as libc::c_int);
            if light != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as
                        u16b
            }
            if room != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x8 as libc::c_int) as
                        u16b
            } else {
                place_filler(0 as libc::c_int + y0 - yhsize, i + x0 - xhsize);
            }
        } else {
            /* Outside of the room -- set to be normal granite */
            place_filler(0 as libc::c_int + y0 - yhsize, i + x0 - xhsize);
        }
        /* Clear the icky flag -- don't need it any more */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int & !(0x4 as libc::c_int)) as u16b;
        /* Access bottom boundary grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset((ysize + y0 - yhsize) as
                                                 isize)).offset((i + x0 -
                                                                     xhsize)
                                                                    as isize)
                as *mut cave_type;
        /* Next to a 'filled' region? -- set to be room walls */
        if (*c_ptr).info as libc::c_int & 0x4 as libc::c_int != 0 {
            cave_set_feat(ysize + y0 - yhsize, i + x0 - xhsize,
                          feat_wall_outer as libc::c_int);
            if light != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as
                        u16b
            }
            if room != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x8 as libc::c_int) as
                        u16b
            } else { place_filler(ysize + y0 - yhsize, i + x0 - xhsize); }
        } else {
            /* Outside of the room -- set to be normal granite */
            place_filler(ysize + y0 - yhsize, i + x0 - xhsize);
        }
        /* Clear the icky flag -- don't need it any more */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int & !(0x4 as libc::c_int)) as u16b;
        i += 1
    }
    /* Do the left and right boundaries minus the corners (done above) */
    i = 1 as libc::c_int;
    while i < ysize {
        /* Access left boundary grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset((i + y0 - yhsize) as
                                                 isize)).offset((0 as
                                                                     libc::c_int
                                                                     + x0 -
                                                                     xhsize)
                                                                    as isize)
                as *mut cave_type;
        /* Next to a 'filled' region? -- set to be room walls */
        if (*c_ptr).info as libc::c_int & 0x4 as libc::c_int != 0 {
            cave_set_feat(i + y0 - yhsize, 0 as libc::c_int + x0 - xhsize,
                          feat_wall_outer as libc::c_int);
            if light != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as
                        u16b
            }
            if room != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x8 as libc::c_int) as
                        u16b
            } else {
                place_filler(i + y0 - yhsize, 0 as libc::c_int + x0 - xhsize);
            }
        } else {
            /* Outside of the room -- set to be normal granite */
            place_filler(i + y0 - yhsize, 0 as libc::c_int + x0 - xhsize);
        }
        /* Clear the icky flag -- don't need it any more */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int & !(0x4 as libc::c_int)) as u16b;
        /* Access left boundary grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset((i + y0 - yhsize) as
                                                 isize)).offset((xsize + x0 -
                                                                     xhsize)
                                                                    as isize)
                as *mut cave_type;
        /* Next to a 'filled' region? -- set to be room walls */
        if (*c_ptr).info as libc::c_int & 0x4 as libc::c_int != 0 {
            cave_set_feat(i + y0 - yhsize, xsize + x0 - xhsize,
                          feat_wall_outer as libc::c_int);
            if light != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as
                        u16b
            }
            if room != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x8 as libc::c_int) as
                        u16b
            } else { place_filler(i + y0 - yhsize, xsize + x0 - xhsize); }
        } else {
            /* Outside of the room -- set to be normal granite */
            place_filler(i + y0 - yhsize, xsize + x0 - xhsize);
        }
        /* Clear the icky flag -- don't need it any more */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int & !(0x4 as libc::c_int)) as u16b;
        i += 1
    }
    /*
	 * Do the rest: convert back to the normal format
	 * In other variants, may want to check to see if cave.feat< some value
	 * if so, set to be water:- this will make interesting pools etc.
	 * (I don't do this for standard Angband.)
	 */
    x = 1 as libc::c_int;
    while x < xsize {
        y = 1 as libc::c_int;
        while y < ysize {
            /* Access the grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset((y + y0 - yhsize) as
                                                     isize)).offset((x + x0 -
                                                                         xhsize)
                                                                        as
                                                                        isize)
                    as *mut cave_type;
            /* A floor grid to be converted */
            if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long
                   & 0x10 as libc::c_long != 0 &&
                   (*c_ptr).info as libc::c_int & 0x4 as libc::c_int != 0 {
                /* Clear the icky flag in the filled region */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int & !(0x4 as libc::c_int)) as
                        u16b;
                /* Set appropriate flags */
                if light != 0 {
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as
                            u16b
                }
                if room != 0 {
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x8 as libc::c_int) as
                            u16b
                }
            } else if (*c_ptr).feat as libc::c_int ==
                          feat_wall_outer as libc::c_int &&
                          (*c_ptr).info as libc::c_int & 0x4 as libc::c_int !=
                              0 {
                /* A wall grid to be convereted */
                /* Clear the icky flag in the filled region */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int & !(0x4 as libc::c_int)) as
                        u16b;
                /* Set appropriate flags */
                if light != 0 {
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as
                            u16b
                }
                if room != 0 {
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x8 as libc::c_int) as
                            u16b
                } else { place_filler(y + y0 - yhsize, x + x0 - xhsize); }
            } else {
                /* None of the above -- clear the unconnected regions */
                place_filler(y + y0 - yhsize, x + x0 - xhsize);
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int &
                         !(0x4 as libc::c_int | 0x8 as libc::c_int)) as u16b
            }
            y += 1
        }
        x += 1
    }
    /*
	 * XXX XXX XXX There is a slight problem when tunnels pierce the caves:
	 * Extra doors appear inside the system.  (Its not very noticeable though.)
	 * This can be removed by "filling" from the outside in.  This allows
	 * a separation from FEAT_WALL_OUTER with FEAT_WALL_INNER.  (Internal
	 * walls are  F.W.OUTER instead.)
	 * The extra effort for what seems to be only a minor thing (even
	 * non-existant if you think of the caves not as normal rooms, but as
	 * holes in the dungeon), doesn't seem worth it.
	 */
    return 1 as libc::c_int as bool_;
}
/*
 * Makes a cave system in the center of the dungeon
 */
unsafe extern "C" fn build_cavern() {
    let mut grd: libc::c_int = 0;
    let mut roug: libc::c_int = 0;
    let mut cutoff: libc::c_int = 0;
    let mut xsize: libc::c_int = 0;
    let mut ysize: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    let mut done: bool_ = 0;
    let mut light: bool_ = 0;
    let mut room: bool_ = 0;
    room = 0 as libc::c_int as bool_;
    done = room;
    light = done;
    if dun_level as libc::c_int <=
           Rand_div(25 as libc::c_int) + 1 as libc::c_int {
        light = 1 as libc::c_int as bool_
    }
    /* Make a cave the size of the dungeon */
    xsize = cur_wid as libc::c_int - 1 as libc::c_int;
    ysize = cur_hgt as libc::c_int - 1 as libc::c_int;
    x0 = xsize / 2 as libc::c_int;
    y0 = ysize / 2 as libc::c_int;
    /* Paranoia: make size even */
    xsize = x0 * 2 as libc::c_int;
    ysize = y0 * 2 as libc::c_int;
    while done == 0 {
        /* Testing values for these parameters: feel free to adjust */
        grd =
            (1 as libc::c_int) <<
                Rand_div(4 as libc::c_int) + 1 as libc::c_int +
                    4 as libc::c_int;
        /* Want average of about 16 */
        roug =
            (Rand_div(8 as libc::c_int) + 1 as libc::c_int) *
                (Rand_div(4 as libc::c_int) + 1 as libc::c_int);
        /* About size/2 */
        cutoff = xsize / 2 as libc::c_int;
        /* Make it */
        generate_hmap(y0, x0, xsize, ysize, grd, roug, cutoff);
        /* Convert to normal format+ clean up*/
        done = generate_fracave(y0, x0, xsize, ysize, cutoff, light, room)
    };
}
/*
 * Driver routine to create fractal cave system
 */
unsafe extern "C" fn build_type10(mut by0: libc::c_int,
                                  mut bx0: libc::c_int) {
    let mut grd: libc::c_int = 0;
    let mut roug: libc::c_int = 0;
    let mut cutoff: libc::c_int = 0;
    let mut xsize: libc::c_int = 0;
    let mut ysize: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut done: bool_ = 0;
    let mut light: bool_ = 0;
    let mut room: bool_ = 0;
    /* Get size: note 'Evenness'*/
    xsize =
        (Rand_div(22 as libc::c_int) + 1 as libc::c_int) * 2 as libc::c_int +
            6 as libc::c_int;
    ysize =
        (Rand_div(15 as libc::c_int) + 1 as libc::c_int) * 2 as libc::c_int +
            6 as libc::c_int;
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc(xsize + 1 as libc::c_int, ysize + 1 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut x0, &mut y0) == 0
       {
        return
    }
    done = 0 as libc::c_int as bool_;
    light = done;
    room = 1 as libc::c_int as bool_;
    if dun_level as libc::c_int <=
           Rand_div(25 as libc::c_int) + 1 as libc::c_int {
        light = 1 as libc::c_int as bool_
    }
    while done == 0 {
        /*
		 * Note: size must be even or there are rounding problems
		 * This causes the tunnels not to connect properly to the room
		 */
        /* Testing values for these parameters feel free to adjust */
        grd =
            (1 as libc::c_int) <<
                Rand_div(4 as libc::c_int) + 1 as libc::c_int;
        /* Want average of about 16 */
        roug =
            (Rand_div(8 as libc::c_int) + 1 as libc::c_int) *
                (Rand_div(4 as libc::c_int) + 1 as libc::c_int);
        /* About size/2 */
        cutoff =
            Rand_div(xsize / 4 as libc::c_int) + 1 as libc::c_int +
                (Rand_div(ysize / 4 as libc::c_int) + 1 as libc::c_int) +
                (Rand_div(xsize / 4 as libc::c_int) + 1 as libc::c_int) +
                (Rand_div(ysize / 4 as libc::c_int) + 1 as libc::c_int);
        /* Make it */
        generate_hmap(y0, x0, xsize, ysize, grd, roug, cutoff);
        /* Convert to normal format + clean up*/
        done = generate_fracave(y0, x0, xsize, ysize, cutoff, light, room)
    };
}
/*
 * Random vault generation from Z 2.5.1
 */
/*
 * Make a very small room centred at (x0, y0)
 *
 * This is used in crypts, and random elemental vaults.
 *
 * Note - this should be used only on allocated regions
 * within another room.
 */
unsafe extern "C" fn build_small_room(mut x0: libc::c_int,
                                      mut y0: libc::c_int) {
    build_rectangle(y0 - 1 as libc::c_int, x0 - 1 as libc::c_int,
                    y0 + 1 as libc::c_int, x0 + 1 as libc::c_int,
                    feat_wall_inner as libc::c_int, 0x8 as libc::c_int);
    /* Place a secret door on one side */
    match Rand_div(4 as libc::c_int) {
        0 => { place_secret_door(y0, x0 - 1 as libc::c_int); }
        1 => { place_secret_door(y0, x0 + 1 as libc::c_int); }
        2 => { place_secret_door(y0 - 1 as libc::c_int, x0); }
        3 => { place_secret_door(y0 + 1 as libc::c_int, x0); }
        _ => { }
    }
    /* Add inner open space */
    place_floor(y0, x0);
}
/*
 * Add a door to a location in a random vault
 *
 * Note that range checking has to be done in the calling routine.
 *
 * The doors must be INSIDE the allocated region.
 */
unsafe extern "C" fn add_door(mut x: libc::c_int, mut y: libc::c_int) {
    /* Need to have a wall in the center square */
    if (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
           feat_wall_outer as libc::c_int {
        return
    }
    /*
	 * Look at:
	 *  x#x
	 *  .#.
	 *  x#x
	 *
	 *  where x=don't care
	 *  .=floor, #=wall
	 */
    if get_is_floor(x, y - 1 as libc::c_int) as libc::c_int != 0 &&
           get_is_floor(x, y + 1 as libc::c_int) as libc::c_int != 0 &&
           (*cave[y as usize].offset((x - 1 as libc::c_int) as isize)).feat as
               libc::c_int == feat_wall_outer as libc::c_int &&
           (*cave[y as usize].offset((x + 1 as libc::c_int) as isize)).feat as
               libc::c_int == feat_wall_outer as libc::c_int {
        /* secret door */
        place_secret_door(y, x);
        /* set boundarys so don't get wide doors */
        place_filler(y, x - 1 as libc::c_int);
        place_filler(y, x + 1 as libc::c_int);
    }
    /*
	 * Look at:
	 *  x#x
	 *  .#.
	 *  x#x
	 *
	 *  where x = don't care
	 *  .=floor, #=wall
	 */
    if (*cave[(y - 1 as libc::c_int) as usize].offset(x as isize)).feat as
           libc::c_int == feat_wall_outer as libc::c_int &&
           (*cave[(y + 1 as libc::c_int) as usize].offset(x as isize)).feat as
               libc::c_int == feat_wall_outer as libc::c_int &&
           get_is_floor(x - 1 as libc::c_int, y) as libc::c_int != 0 &&
           get_is_floor(x + 1 as libc::c_int, y) as libc::c_int != 0 {
        /* secret door */
        place_secret_door(y, x);
        /* set boundarys so don't get wide doors */
        place_filler(y - 1 as libc::c_int, x);
        place_filler(y + 1 as libc::c_int, x);
    };
}
/*
 * Fill the empty areas of a room with treasure and monsters.
 */
unsafe extern "C" fn fill_treasure(mut x1: libc::c_int, mut x2: libc::c_int,
                                   mut y1: libc::c_int, mut y2: libc::c_int,
                                   mut difficulty: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut value: s32b = 0;
    /* center of room:*/
    cx = (x1 + x2) / 2 as libc::c_int;
    cy = (y1 + y2) / 2 as libc::c_int;
    /* Rough measure of size of vault= sum of lengths of sides */
    size = abs(x2 - x1) + abs(y2 - y1);
    x = x1;
    while x <= x2 {
        y = y1;
        while y <= y2 {
            /*
			 * Thing added based on distance to center of vault
			 * Difficulty is 1-easy to 10-hard
			 */
            value =
                distance(cx, cy, x, y) * 100 as libc::c_int / size +
                    (Rand_div(10 as libc::c_int) + 1 as libc::c_int) -
                    difficulty;
            /* Hack -- Empty square part of the time */
            if Rand_div(100 as libc::c_int) + 1 as libc::c_int -
                   difficulty * 3 as libc::c_int > 50 as libc::c_int {
                value = 20 as libc::c_int
            }
            /* If floor, shallow water or lava */
            if get_is_floor(x, y) as libc::c_int != 0 ||
                   (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                       == 0x54 as libc::c_int ||
                   (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                       == 0x56 as libc::c_int {
                /* The smaller 'value' is, the better the stuff */
                if value < 0 as libc::c_int {
                    /* Meanest monster + treasure */
                    monster_level =
                        (dun_level as libc::c_int + 40 as libc::c_int) as
                            s16b;
                    place_monster(y, x, 1 as libc::c_int as bool_,
                                  1 as libc::c_int as bool_);
                    monster_level = dun_level;
                    object_level =
                        (dun_level as libc::c_int + 20 as libc::c_int) as
                            s16b;
                    place_object(y, x, 1 as libc::c_int as bool_,
                                 0 as libc::c_int as bool_, 2 as libc::c_int);
                    object_level = dun_level
                } else if value < 5 as libc::c_int {
                    /* Mean monster +treasure */
                    monster_level =
                        (dun_level as libc::c_int + 20 as libc::c_int) as
                            s16b;
                    place_monster(y, x, 1 as libc::c_int as bool_,
                                  1 as libc::c_int as bool_);
                    monster_level = dun_level;
                    object_level =
                        (dun_level as libc::c_int + 10 as libc::c_int) as
                            s16b;
                    place_object(y, x, 1 as libc::c_int as bool_,
                                 0 as libc::c_int as bool_, 2 as libc::c_int);
                    object_level = dun_level
                } else if value < 10 as libc::c_int {
                    /* Monster */
                    monster_level =
                        (dun_level as libc::c_int + 9 as libc::c_int) as s16b;
                    place_monster(y, x, 1 as libc::c_int as bool_,
                                  1 as libc::c_int as bool_);
                    monster_level = dun_level
                } else if !(value < 17 as libc::c_int) {
                    if value < 23 as libc::c_int {
                        /* Object or trap */
                        if Rand_div(100 as libc::c_int) < 25 as libc::c_int {
                            place_object(y, x, 0 as libc::c_int as bool_,
                                         0 as libc::c_int as bool_,
                                         2 as libc::c_int);
                        } else { place_trap(y, x); }
                    } else if value < 30 as libc::c_int {
                        /* Monster and trap */
                        monster_level =
                            (dun_level as libc::c_int + 5 as libc::c_int) as
                                s16b;
                        place_monster(y, x, 1 as libc::c_int as bool_,
                                      1 as libc::c_int as bool_);
                        monster_level = dun_level;
                        place_trap(y, x);
                    } else if value < 40 as libc::c_int {
                        /* Monster or object */
                        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                            monster_level =
                                (dun_level as libc::c_int + 3 as libc::c_int)
                                    as s16b;
                            place_monster(y, x, 1 as libc::c_int as bool_,
                                          1 as libc::c_int as bool_);
                            monster_level = dun_level
                        }
                        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                            object_level =
                                (dun_level as libc::c_int + 7 as libc::c_int)
                                    as s16b;
                            place_object(y, x, 0 as libc::c_int as bool_,
                                         0 as libc::c_int as bool_,
                                         2 as libc::c_int);
                            object_level = dun_level
                        }
                    } else if value < 50 as libc::c_int {
                        /* Trap */
                        place_trap(y, x);
                    } else if Rand_div(100 as libc::c_int) < 20 as libc::c_int
                     {
                        place_monster(y, x, 1 as libc::c_int as bool_,
                                      1 as libc::c_int as bool_);
                    } else if Rand_div(100 as libc::c_int) < 50 as libc::c_int
                     {
                        place_trap(y, x);
                    } else if Rand_div(100 as libc::c_int) < 50 as libc::c_int
                     {
                        place_object(y, x, 0 as libc::c_int as bool_,
                                     0 as libc::c_int as bool_,
                                     2 as libc::c_int);
                    }
                }
            }
            y += 1
        }
        x += 1
    };
}
/* Various Stuff */
/* 20% monster, 40% trap, 20% object, 20% blank space */
/* number of bubbles */
unsafe extern "C" fn build_bubble_vault(mut x0: libc::c_int,
                                        mut y0: libc::c_int,
                                        mut xsize: libc::c_int,
                                        mut ysize: libc::c_int) {
    /* array of center points of bubbles */
    let mut center: [coord; 10] = [coord{y: 0, x: 0,}; 10];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut min1: u16b = 0;
    let mut min2: u16b = 0;
    let mut temp: u16b = 0;
    let mut done: bool_ = 0;
    /* Offset from center to top left hand corner */
    let mut xhsize: libc::c_int = xsize / 2 as libc::c_int;
    let mut yhsize: libc::c_int = ysize / 2 as libc::c_int;
    if cheat_room != 0 {
        msg_print(b"Bubble Vault\x00" as *const u8 as *const libc::c_char);
    }
    /* Allocate center of bubbles */
    center[0 as libc::c_int as usize].x =
        (Rand_div(xsize - 3 as libc::c_int) + 1 as libc::c_int +
             1 as libc::c_int) as byte_hack;
    center[0 as libc::c_int as usize].y =
        (Rand_div(ysize - 3 as libc::c_int) + 1 as libc::c_int +
             1 as libc::c_int) as byte_hack;
    i = 1 as libc::c_int;
    while i < 10 as libc::c_int {
        done = 0 as libc::c_int as bool_;
        /* Get center and check to see if it is unique */
        k = 0 as libc::c_int;
        while done == 0 && k < 2000 as libc::c_int {
            done = 1 as libc::c_int as bool_;
            x =
                Rand_div(xsize - 3 as libc::c_int) + 1 as libc::c_int +
                    1 as libc::c_int;
            y =
                Rand_div(ysize - 3 as libc::c_int) + 1 as libc::c_int +
                    1 as libc::c_int;
            j = 0 as libc::c_int;
            while j < i {
                /* Rough test to see if there is an overlap */
                if x == center[j as usize].x as libc::c_int ||
                       y == center[j as usize].y as libc::c_int {
                    done = 0 as libc::c_int as bool_
                }
                j += 1
            }
            k += 1
        }
        /* Too many failures */
        if k >= 2000 as libc::c_int { return }
        center[i as usize].x = x as byte_hack;
        center[i as usize].y = y as byte_hack;
        i += 1
    }
    build_rectangle(y0 - yhsize, x0 - xhsize,
                    y0 - yhsize + ysize - 1 as libc::c_int,
                    x0 - xhsize + xsize - 1 as libc::c_int,
                    feat_wall_outer as libc::c_int,
                    0x8 as libc::c_int | 0x4 as libc::c_int);
    /* Fill in middle with bubbles */
    x = 1 as libc::c_int;
    while x < xsize - 1 as libc::c_int {
        y = 1 as libc::c_int;
        while y < ysize - 1 as libc::c_int {
            let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
            /* Get distances to two closest centers */
            /* Initialise */
            min1 =
                distance(x, y,
                         center[0 as libc::c_int as usize].x as libc::c_int,
                         center[0 as libc::c_int as usize].y as libc::c_int)
                    as u16b;
            min2 =
                distance(x, y,
                         center[1 as libc::c_int as usize].x as libc::c_int,
                         center[1 as libc::c_int as usize].y as libc::c_int)
                    as u16b;
            if min1 as libc::c_int > min2 as libc::c_int {
                /* Swap if in wrong order */
                temp = min1;
                min1 = min2;
                min2 = temp
            }
            /* Scan the rest */
            i = 2 as libc::c_int;
            while i < 10 as libc::c_int {
                temp =
                    distance(x, y, center[i as usize].x as libc::c_int,
                             center[i as usize].y as libc::c_int) as u16b;
                if (temp as libc::c_int) < min1 as libc::c_int {
                    /* Smallest */
                    min2 = min1;
                    min1 = temp
                } else if (temp as libc::c_int) < min2 as libc::c_int {
                    /* Second smallest */
                    min2 = temp
                }
                i += 1
            }
            /* Access the grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset((y + y0 - yhsize) as
                                                     isize)).offset((x + x0 -
                                                                         xhsize)
                                                                        as
                                                                        isize)
                    as *mut cave_type;
            /*
			 * Boundary at midpoint+ not at inner region of bubble
			 *
			 * SCSCSC: was feat_wall_outer
			 */
            if min2 as libc::c_int - min1 as libc::c_int <= 2 as libc::c_int
                   && !((min1 as libc::c_int) < 3 as libc::c_int) {
                place_filler(y + y0 - yhsize, x + x0 - xhsize);
            } else {
                /* Middle of a bubble */
                place_floor(y + y0 - yhsize, x + x0 - xhsize);
            }
            /* Clean up rest of flags */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int |
                     (0x8 as libc::c_int | 0x4 as libc::c_int)) as u16b;
            y += 1
        }
        x += 1
    }
    /* Try to add some random doors */
    i = 0 as libc::c_int;
    while i < 500 as libc::c_int {
        x =
            Rand_div(xsize - 3 as libc::c_int) + 1 as libc::c_int - xhsize +
                x0 + 1 as libc::c_int;
        y =
            Rand_div(ysize - 3 as libc::c_int) + 1 as libc::c_int - yhsize +
                y0 + 1 as libc::c_int;
        add_door(x, y);
        i += 1
    }
    /* Fill with monsters and treasure, low difficulty */
    fill_treasure(x0 - xhsize + 1 as libc::c_int,
                  x0 - xhsize + xsize - 2 as libc::c_int,
                  y0 - yhsize + 1 as libc::c_int,
                  y0 - yhsize + ysize - 2 as libc::c_int,
                  Rand_div(5 as libc::c_int) + 1 as libc::c_int);
}
/*
 * Convert FEAT_WALL_EXTRA (used by random vaults) to normal dungeon wall
 */
unsafe extern "C" fn convert_extra(mut y1: libc::c_int, mut x1: libc::c_int,
                                   mut y2: libc::c_int, mut x2: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    x = x1;
    while x <= x2 {
        y = y1;
        while y <= y2 {
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   0x3a as libc::c_int {
                place_filler(y, x);
            }
            y += 1
        }
        x += 1
    };
}
/*
 * Overlay a rectangular room given its bounds
 *
 * This routine is used by build_room_vault (hence FEAT_WALL_OUTER)
 * The area inside the walls is not touched: only granite is removed
 * and normal walls stay
 */
unsafe extern "C" fn build_room(mut x1: libc::c_int, mut x2: libc::c_int,
                                mut y1: libc::c_int, mut y2: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xsize: libc::c_int = 0;
    let mut ysize: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    /* Check if rectangle has no width */
    if x1 == x2 || y1 == y2 { return }
    /* initialize */
    if x1 > x2 {
        /* Swap boundaries if in wrong order */
        temp = x1;
        x1 = x2;
        x2 = temp
    }
    if y1 > y2 {
        /* Swap boundaries if in wrong order */
        temp = y1;
        y1 = y2;
        y2 = temp
    }
    /* Get total widths */
    xsize = x2 - x1;
    ysize = y2 - y1;
    build_rectangle(y1, x1, y2, x2, feat_wall_outer as libc::c_int,
                    0x8 as libc::c_int | 0x4 as libc::c_int);
    /* Middle */
    x = 1 as libc::c_int;
    while x < xsize {
        y = 1 as libc::c_int;
        while y < ysize {
            if (*cave[(y1 + y) as usize].offset((x1 + x) as isize)).feat as
                   libc::c_int == 0x3a as libc::c_int {
                /* Clear the untouched region */
                place_floor(y1 + y, x1 + x);
                let ref mut fresh20 =
                    (*cave[(y1 + y) as usize].offset((x1 + x) as isize)).info;
                *fresh20 =
                    (*fresh20 as libc::c_int |
                         (0x8 as libc::c_int | 0x4 as libc::c_int)) as u16b
            } else {
                /* Make it a room -- but don't touch */
                let ref mut fresh21 =
                    (*cave[(y1 + y) as usize].offset((x1 + x) as isize)).info;
                *fresh21 =
                    (*fresh21 as libc::c_int |
                         (0x8 as libc::c_int | 0x4 as libc::c_int)) as u16b
            }
            y += 1
        }
        x += 1
    };
}
/*
 * Create a random vault that looks like a collection of overlapping rooms
 */
unsafe extern "C" fn build_room_vault(mut x0: libc::c_int,
                                      mut y0: libc::c_int,
                                      mut xsize: libc::c_int,
                                      mut ysize: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut xhsize: libc::c_int = 0;
    let mut yhsize: libc::c_int = 0;
    /* Get offset from center */
    xhsize = xsize / 2 as libc::c_int;
    yhsize = ysize / 2 as libc::c_int;
    if cheat_room != 0 {
        msg_print(b"Room Vault\x00" as *const u8 as *const libc::c_char);
    }
    /* Fill area so don't get problems with arena levels */
    x1 = 0 as libc::c_int;
    while x1 <= xsize {
        let mut x: libc::c_int = x0 - xhsize + x1;
        y1 = 0 as libc::c_int;
        while y1 <= ysize {
            let mut y: libc::c_int = y0 - yhsize + y1;
            cave_set_feat(y, x, 0x3a as libc::c_int);
            let ref mut fresh22 = (*cave[y as usize].offset(x as isize)).info;
            *fresh22 =
                (*fresh22 as libc::c_int & !(0x4 as libc::c_int)) as u16b;
            y1 += 1
        }
        x1 += 1
    }
    /* Add ten random rooms */
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        x1 =
            (Rand_div(xhsize) + 1 as libc::c_int) * 2 as libc::c_int + x0 -
                xhsize;
        x2 =
            (Rand_div(xhsize) + 1 as libc::c_int) * 2 as libc::c_int + x0 -
                xhsize;
        y1 =
            (Rand_div(yhsize) + 1 as libc::c_int) * 2 as libc::c_int + y0 -
                yhsize;
        y2 =
            (Rand_div(yhsize) + 1 as libc::c_int) * 2 as libc::c_int + y0 -
                yhsize;
        build_room(x1, x2, y1, y2);
        i += 1
    }
    convert_extra(y0 - yhsize, x0 - xhsize, y0 - yhsize + ysize,
                  x0 - xhsize + xsize);
    /* Add some random doors */
    i = 0 as libc::c_int;
    while i < 500 as libc::c_int {
        x1 =
            Rand_div(xsize - 2 as libc::c_int) + 1 as libc::c_int - xhsize +
                x0 + 1 as libc::c_int;
        y1 =
            Rand_div(ysize - 2 as libc::c_int) + 1 as libc::c_int - yhsize +
                y0 + 1 as libc::c_int;
        add_door(x1, y1);
        i += 1
    }
    /* Fill with monsters and treasure, high difficulty */
    fill_treasure(x0 - xhsize + 1 as libc::c_int,
                  x0 - xhsize + xsize - 1 as libc::c_int,
                  y0 - yhsize + 1 as libc::c_int,
                  y0 - yhsize + ysize - 1 as libc::c_int,
                  Rand_div(5 as libc::c_int) + 1 as libc::c_int +
                      5 as libc::c_int);
}
/*
 * Create a random vault out of a fractal cave
 */
unsafe extern "C" fn build_cave_vault(mut x0: libc::c_int,
                                      mut y0: libc::c_int,
                                      mut xsiz: libc::c_int,
                                      mut ysiz: libc::c_int) {
    let mut grd: libc::c_int = 0;
    let mut roug: libc::c_int = 0;
    let mut cutoff: libc::c_int = 0;
    let mut xhsize: libc::c_int = 0;
    let mut yhsize: libc::c_int = 0;
    let mut xsize: libc::c_int = 0;
    let mut ysize: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut done: bool_ = 0;
    let mut light: bool_ = 0;
    let mut room: bool_ = 0;
    /* Round to make sizes even */
    xhsize = xsiz / 2 as libc::c_int;
    yhsize = ysiz / 2 as libc::c_int;
    xsize = xhsize * 2 as libc::c_int;
    ysize = yhsize * 2 as libc::c_int;
    if cheat_room != 0 {
        msg_print(b"Cave Vault\x00" as *const u8 as *const libc::c_char);
    }
    done = 0 as libc::c_int as bool_;
    light = done;
    room = 1 as libc::c_int as bool_;
    while done == 0 {
        /* Testing values for these parameters feel free to adjust */
        grd = (1 as libc::c_int) << Rand_div(4 as libc::c_int);
        /* Want average of about 16 */
        roug =
            (Rand_div(8 as libc::c_int) + 1 as libc::c_int) *
                (Rand_div(4 as libc::c_int) + 1 as libc::c_int);
        /* About size/2 */
        cutoff =
            Rand_div(xsize / 4 as libc::c_int) + 1 as libc::c_int +
                (Rand_div(ysize / 4 as libc::c_int) + 1 as libc::c_int) +
                (Rand_div(xsize / 4 as libc::c_int) + 1 as libc::c_int) +
                (Rand_div(ysize / 4 as libc::c_int) + 1 as libc::c_int);
        /* Make it */
        generate_hmap(y0, x0, xsize, ysize, grd, roug, cutoff);
        /* Convert to normal format + clean up */
        done = generate_fracave(y0, x0, xsize, ysize, cutoff, light, room)
    }
    /* Set icky flag because is a vault */
    x = 0 as libc::c_int;
    while x <= xsize {
        y = 0 as libc::c_int;
        while y <= ysize {
            let ref mut fresh23 =
                (*cave[(y0 - yhsize + y) as
                           usize].offset((x0 - xhsize + x) as isize)).info;
            *fresh23 = (*fresh23 as libc::c_int | 0x4 as libc::c_int) as u16b;
            y += 1
        }
        x += 1
    }
    /* Fill with monsters and treasure, low difficulty */
    fill_treasure(x0 - xhsize + 1 as libc::c_int,
                  x0 - xhsize + xsize - 1 as libc::c_int,
                  y0 - yhsize + 1 as libc::c_int,
                  y0 - yhsize + ysize - 1 as libc::c_int,
                  Rand_div(5 as libc::c_int) + 1 as libc::c_int);
}
/*
 * Maze vault -- rectangular labyrinthine rooms
 *
 * maze vault uses two routines:
 *    r_visit - a recursive routine that builds the labyrinth
 *    build_maze_vault - a driver routine that calls r_visit and adds
 *                   monsters, traps and treasure
 *
 * The labyrinth is built by creating a spanning tree of a graph.
 * The graph vertices are at
 *    (x, y) = (2j + x1, 2k + y1)   j = 0,...,m-1    k = 0,...,n-1
 * and the edges are the vertical and horizontal nearest neighbors.
 *
 * The spanning tree is created by performing a suitably randomized
 * depth-first traversal of the graph. The only adjustable parameter
 * is the rand_int(3) below; it governs the relative density of
 * twists and turns in the labyrinth: smaller number, more twists.
 */
unsafe extern "C" fn r_visit(mut y1: libc::c_int, mut x1: libc::c_int,
                             mut y2: libc::c_int, mut x2: libc::c_int,
                             mut node: libc::c_int, mut dir: libc::c_int,
                             mut visited: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut adj: [libc::c_int; 4] = [0; 4];
    /* Dimensions of vertex array */
    m = (x2 - x1) / 2 as libc::c_int + 1 as libc::c_int;
    n = (y2 - y1) / 2 as libc::c_int + 1 as libc::c_int;
    /* Mark node visited and set it to a floor */
    *visited.offset(node as isize) = 1 as libc::c_int;
    x = 2 as libc::c_int * (node % m) + x1;
    y = 2 as libc::c_int * (node / m) + y1;
    place_floor(y, x);
    /* Setup order of adjacent node visits */
    if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
        /* Pick a random ordering */
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int { adj[i as usize] = i; i += 1 }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            j = Rand_div(4 as libc::c_int);
            temp = adj[i as usize];
            adj[i as usize] = adj[j as usize];
            adj[j as usize] = temp;
            i += 1
        }
        dir = adj[0 as libc::c_int as usize]
    } else {
        /* Pick a random ordering with dir first */
        adj[0 as libc::c_int as usize] = dir;
        i = 1 as libc::c_int;
        while i < 4 as libc::c_int { adj[i as usize] = i; i += 1 }
        i = 1 as libc::c_int;
        while i < 4 as libc::c_int {
            j = 1 as libc::c_int + Rand_div(3 as libc::c_int);
            temp = adj[i as usize];
            adj[i as usize] = adj[j as usize];
            adj[j as usize] = temp;
            i += 1
        }
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        match adj[i as usize] {
            0 => {
                /* (0,+) - check for bottom boundary */
                if node / m < n - 1 as libc::c_int &&
                       *visited.offset((node + m) as isize) ==
                           0 as libc::c_int {
                    place_floor(y + 1 as libc::c_int, x);
                    r_visit(y1, x1, y2, x2, node + m, dir, visited);
                }
            }
            1 => {
                /* (0,-) - check for top boundary */
                if node / m > 0 as libc::c_int &&
                       *visited.offset((node - m) as isize) ==
                           0 as libc::c_int {
                    place_floor(y - 1 as libc::c_int, x);
                    r_visit(y1, x1, y2, x2, node - m, dir, visited);
                }
            }
            2 => {
                /* (+,0) - check for right boundary */
                if node % m < m - 1 as libc::c_int &&
                       *visited.offset((node + 1 as libc::c_int) as isize) ==
                           0 as libc::c_int {
                    place_floor(y, x + 1 as libc::c_int);
                    r_visit(y1, x1, y2, x2, node + 1 as libc::c_int, dir,
                            visited);
                }
            }
            3 => {
                /* (-,0) - check for left boundary */
                if node % m > 0 as libc::c_int &&
                       *visited.offset((node - 1 as libc::c_int) as isize) ==
                           0 as libc::c_int {
                    place_floor(y, x - 1 as libc::c_int);
                    r_visit(y1, x1, y2, x2, node - 1 as libc::c_int, dir,
                            visited);
                }
            }
            _ => { }
        }
        i += 1
    };
}
unsafe extern "C" fn build_maze_vault(mut x0: libc::c_int,
                                      mut y0: libc::c_int,
                                      mut xsize: libc::c_int,
                                      mut ysize: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut num_vertices: libc::c_int = 0;
    let mut visited: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut light: bool_ = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    if cheat_room != 0 {
        msg_print(b"Maze Vault\x00" as *const u8 as *const libc::c_char);
    }
    /* Choose lite or dark */
    light =
        (dun_level as libc::c_int <=
             Rand_div(25 as libc::c_int) + 1 as libc::c_int) as libc::c_int as
            bool_;
    /* Pick a random room size - randomized by calling routine */
    dy = ysize / 2 as libc::c_int - 1 as libc::c_int;
    dx = xsize / 2 as libc::c_int - 1 as libc::c_int;
    y1 = y0 - dy;
    x1 = x0 - dx;
    y2 = y0 + dy;
    x2 = x0 + dx;
    /* Generate the room */
    y = y1 - 1 as libc::c_int;
    while y <= y2 + 1 as libc::c_int {
        x = x1 - 1 as libc::c_int;
        while x <= x2 + 1 as libc::c_int {
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int |
                     (0x8 as libc::c_int | 0x4 as libc::c_int)) as u16b;
            if x == x1 - 1 as libc::c_int || x == x2 + 1 as libc::c_int ||
                   y == y1 - 1 as libc::c_int || y == y2 + 1 as libc::c_int {
                cave_set_feat(y, x, feat_wall_outer as libc::c_int);
            } else { cave_set_feat(y, x, feat_wall_inner as libc::c_int); }
            if light != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as
                        u16b
            }
            x += 1
        }
        y += 1
    }
    /* Dimensions of vertex array */
    m = dx + 1 as libc::c_int;
    n = dy + 1 as libc::c_int;
    num_vertices = m * n;
    /* Allocate an array for visited vertices */
    visited =
        memset(ralloc((num_vertices as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (num_vertices as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    /* Initialise array of visited vertices */
    i = 0 as libc::c_int;
    while i < num_vertices {
        *visited.offset(i as isize) = 0 as libc::c_int;
        i += 1
    }
    /* Traverse the graph to create a spaning tree, pick a random root */
    r_visit(y1, x1, y2, x2, Rand_div(num_vertices), 0 as libc::c_int,
            visited);
    /* Fill with monsters and treasure, low difficulty */
    fill_treasure(x1, x2, y1, y2,
                  Rand_div(5 as libc::c_int) + 1 as libc::c_int);
    /* Free the array for visited vertices */
    rnfree(visited as vptr,
           (num_vertices as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
}
/*
 * Build a "mini" checkerboard vault
 *
 * This is done by making a permanent wall maze and setting
 * the diagonal sqaures of the checker board to be granite.
 * The vault has two entrances on opposite sides to guarantee
 * a way to get in even if the vault abuts a side of the dungeon.
 */
unsafe extern "C" fn build_mini_c_vault(mut x0: libc::c_int,
                                        mut y0: libc::c_int,
                                        mut xsize: libc::c_int,
                                        mut ysize: libc::c_int) {
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut num_vertices: libc::c_int = 0;
    let mut visited: *mut libc::c_int = 0 as *mut libc::c_int;
    if cheat_room != 0 {
        msg_print(b"Mini Checker Board Vault\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Pick a random room size */
    dy = ysize / 2 as libc::c_int - 1 as libc::c_int;
    dx = xsize / 2 as libc::c_int - 1 as libc::c_int;
    y1 = y0 - dy;
    x1 = x0 - dx;
    y2 = y0 + dy;
    x2 = x0 + dx;
    /* Generate the room */
    y = y1 - 1 as libc::c_int;
    while y <= y2 + 1 as libc::c_int {
        x = x1 - 1 as libc::c_int;
        while x <= x2 + 1 as libc::c_int {
            let ref mut fresh24 = (*cave[y as usize].offset(x as isize)).info;
            *fresh24 =
                (*fresh24 as libc::c_int |
                     (0x8 as libc::c_int | 0x4 as libc::c_int)) as u16b;
            /* Permanent walls */
            cave_set_feat(y, x, 0x3d as libc::c_int);
            x += 1
        }
        y += 1
    }
    /* Dimensions of vertex array */
    m = dx + 1 as libc::c_int;
    n = dy + 1 as libc::c_int;
    num_vertices = m * n;
    /* Allocate an array for visited vertices */
    visited =
        memset(ralloc((num_vertices as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (num_vertices as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    /* Initialise array of visited vertices */
    i = 0 as libc::c_int;
    while i < num_vertices {
        *visited.offset(i as isize) = 0 as libc::c_int;
        i += 1
    }
    /* Traverse the graph to create a spannng tree, pick a random root */
    r_visit(y1, x1, y2, x2, Rand_div(num_vertices), 0 as libc::c_int,
            visited);
    /* Make it look like a checker board vault */
    x = x1;
    while x <= x2 {
        y = y1;
        while y <= y2 {
            total = x - x1 + y - y1;
            /* If total is odd and is a floor, then make a wall */
            if total % 2 as libc::c_int == 1 as libc::c_int &&
                   get_is_floor(x, y) as libc::c_int != 0 {
                cave_set_feat(y, x, feat_wall_inner as libc::c_int);
            }
            y += 1
        }
        x += 1
    }
    /* Make a couple of entrances */
    if Rand_div(2 as libc::c_int) == 0 as libc::c_int {
        /* Left and right */
        y = Rand_div(dy) + 1 as libc::c_int + dy / 2 as libc::c_int;
        cave_set_feat(y1 + y, x1 - 1 as libc::c_int,
                      feat_wall_outer as libc::c_int);
        cave_set_feat(y1 + y, x2 + 1 as libc::c_int,
                      feat_wall_outer as libc::c_int);
    } else {
        /* Top and bottom */
        x = Rand_div(dx) + 1 as libc::c_int + dx / 2 as libc::c_int;
        cave_set_feat(y1 - 1 as libc::c_int, x1 + x,
                      feat_wall_outer as libc::c_int);
        cave_set_feat(y2 + 1 as libc::c_int, x1 + x,
                      feat_wall_outer as libc::c_int);
    }
    /* Fill with monsters and treasure, highest difficulty */
    fill_treasure(x1, x2, y1, y2, 10 as libc::c_int);
    /* Free the array for visited vertices */
    rnfree(visited as vptr,
           (num_vertices as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
}
/*
 * Build a town/ castle by using a recursive algorithm.
 * Basically divide each region in a probalistic way to create
 * smaller regions.  When the regions get too small stop.
 *
 * The power variable is a measure of how well defended a region is.
 * This alters the possible choices.
 */
unsafe extern "C" fn build_recursive_room(mut x1: libc::c_int,
                                          mut y1: libc::c_int,
                                          mut x2: libc::c_int,
                                          mut y2: libc::c_int,
                                          mut power: libc::c_int) {
    let mut xsize: libc::c_int = 0;
    let mut ysize: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut choice: libc::c_int = 0;
    /* Temp variables */
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t3: libc::c_int = 0;
    let mut t4: libc::c_int = 0;
    xsize = x2 - x1;
    ysize = y2 - y1;
    if power < 3 as libc::c_int && xsize > 12 as libc::c_int &&
           ysize > 12 as libc::c_int {
        /* Need outside wall +keep */
        choice = 1 as libc::c_int
    } else if power < 10 as libc::c_int {
        /* Make rooms + subdivide */
        if Rand_div(10 as libc::c_int) + 1 as libc::c_int > 2 as libc::c_int
               && xsize < 8 as libc::c_int && ysize < 8 as libc::c_int {
            choice = 4 as libc::c_int
        } else {
            choice =
                Rand_div(2 as libc::c_int) + 1 as libc::c_int +
                    1 as libc::c_int
        }
    } else {
        /* Mostly subdivide */
        choice =
            Rand_div(3 as libc::c_int) + 1 as libc::c_int + 1 as libc::c_int
    }
    let mut current_block_102: u64;
    /* Based on the choice made above, do something */
    match choice {
        1 => {
            /* Outer walls */
            /* Top and bottom */
            x = x1;
            while x <= x2 {
                cave_set_feat(y1, x, feat_wall_outer as libc::c_int);
                cave_set_feat(y2, x, feat_wall_outer as libc::c_int);
                x += 1
            }
            /* Fall through */
            y = y1 + 1 as libc::c_int;
            while y < y2 {
                cave_set_feat(y, x1, feat_wall_outer as libc::c_int);
                cave_set_feat(y, x2, feat_wall_outer as libc::c_int);
                y += 1
            }
            if Rand_div(2 as libc::c_int) == 0 as libc::c_int {
                /* Left and right */
                /* Make a couple of entrances */
                /* Left and right */
                y = Rand_div(ysize) + 1 as libc::c_int + y1;
                place_floor(y, x1);
                place_floor(y, x2);
            } else {
                /* Top and bottom */
                x = Rand_div(xsize) + 1 as libc::c_int + x1;
                place_floor(y1, x);
                place_floor(y2, x);
            }
            t1 = Rand_div(ysize / 3 as libc::c_int) + 1 as libc::c_int + y1;
            t2 = y2 - (Rand_div(ysize / 3 as libc::c_int) + 1 as libc::c_int);
            t3 = Rand_div(xsize / 3 as libc::c_int) + 1 as libc::c_int + x1;
            t4 = x2 - (Rand_div(xsize / 3 as libc::c_int) + 1 as libc::c_int);
            build_recursive_room(x1 + 1 as libc::c_int, y1 + 1 as libc::c_int,
                                 x2 - 1 as libc::c_int, t1,
                                 power + 1 as libc::c_int);
            build_recursive_room(x1 + 1 as libc::c_int, t2,
                                 x2 - 1 as libc::c_int, y2,
                                 power + 1 as libc::c_int);
            build_recursive_room(x1 + 1 as libc::c_int, t1 + 1 as libc::c_int,
                                 t3, t2 - 1 as libc::c_int,
                                 power + 3 as libc::c_int);
            build_recursive_room(t4, t1 + 1 as libc::c_int,
                                 x2 - 1 as libc::c_int, t2 - 1 as libc::c_int,
                                 power + 3 as libc::c_int);
            x1 = t3;
            x2 = t4;
            y1 = t1;
            y2 = t2;
            xsize = x2 - x1;
            ysize = y2 - y1;
            power += 2 as libc::c_int;
            current_block_102 = 12497913735442871383;
        }
        4 => { current_block_102 = 12497913735442871383; }
        2 => {
            /* Select size of keep */
            /* Do outside areas */
            /* Above and below keep */
            /* Left and right of keep */
            /* Make the keep itself: */
            /* Try and divide vertically */
            if xsize < 3 as libc::c_int {
                /* Too small */
                y = y1;
                while y < y2 {
                    x = x1;
                    while x < x2 {
                        cave_set_feat(y, x, feat_wall_inner as libc::c_int);
                        x += 1
                    }
                    y += 1
                }
                return
            }
            t1 =
                Rand_div(xsize - 2 as libc::c_int) + 1 as libc::c_int + x1 +
                    1 as libc::c_int;
            build_recursive_room(x1, y1, t1, y2, power - 2 as libc::c_int);
            build_recursive_room(t1 + 1 as libc::c_int, y1, x2, y2,
                                 power - 2 as libc::c_int);
            current_block_102 = 10301740260014665685;
        }
        3 => {
            /* Try and divide horizontally */
            if ysize < 3 as libc::c_int {
                /* Too small */
                y = y1;
                while y < y2 {
                    x = x1;
                    while x < x2 {
                        cave_set_feat(y, x, feat_wall_inner as libc::c_int);
                        x += 1
                    }
                    y += 1
                }
                return
            }
            t1 =
                Rand_div(ysize - 2 as libc::c_int) + 1 as libc::c_int + y1 +
                    1 as libc::c_int;
            build_recursive_room(x1, y1, x2, t1, power - 2 as libc::c_int);
            build_recursive_room(x1, t1 + 1 as libc::c_int, x2, y2,
                                 power - 2 as libc::c_int);
            current_block_102 = 10301740260014665685;
        }
        _ => { current_block_102 = 10301740260014665685; }
    }
    match current_block_102 {
        12497913735442871383 =>
        /* Try to build a room */
        {
            if xsize < 3 as libc::c_int || ysize < 3 as libc::c_int {
                y = y1;
                while y < y2 {
                    x = x1;
                    while x < x2 {
                        cave_set_feat(y, x, feat_wall_inner as libc::c_int);
                        x += 1
                    }
                    y += 1
                }
                /* Too small */
                return
            }
            /* Make outside walls */
            /* Top and bottom */
            x = x1 + 1 as libc::c_int;
            while x <= x2 - 1 as libc::c_int {
                cave_set_feat(y1 + 1 as libc::c_int, x,
                              feat_wall_inner as libc::c_int);
                cave_set_feat(y2 - 1 as libc::c_int, x,
                              feat_wall_inner as libc::c_int);
                x += 1
            }
            /* Left and right */
            y = y1 + 1 as libc::c_int;
            while y <= y2 - 1 as libc::c_int {
                cave_set_feat(y, x1 + 1 as libc::c_int,
                              feat_wall_inner as libc::c_int);
                cave_set_feat(y, x2 - 1 as libc::c_int,
                              feat_wall_inner as libc::c_int);
                y += 1
            }
            /* Make a door */
            y =
                Rand_div(ysize - 3 as libc::c_int) + 1 as libc::c_int + y1 +
                    1 as libc::c_int;
            if Rand_div(2 as libc::c_int) == 0 as libc::c_int {
                /* Left */
                place_floor(y, x1 + 1 as libc::c_int);
            } else {
                /* Right */
                place_floor(y, x2 - 1 as libc::c_int);
            }
            /* Build the room */
            build_recursive_room(x1 + 2 as libc::c_int, y1 + 2 as libc::c_int,
                                 x2 - 2 as libc::c_int, y2 - 2 as libc::c_int,
                                 power + 3 as libc::c_int);
        }
        _ => { }
    };
}
/*
 * Build a castle
 *
 * Clear the region and call the recursive room routine.
 *
 * This makes a vault that looks like a castle or city in the dungeon.
 */
unsafe extern "C" fn build_castle_vault(mut x0: libc::c_int,
                                        mut y0: libc::c_int,
                                        mut xsize: libc::c_int,
                                        mut ysize: libc::c_int) {
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Pick a random room size */
    dy = ysize / 2 as libc::c_int - 1 as libc::c_int;
    dx = xsize / 2 as libc::c_int - 1 as libc::c_int;
    y1 = y0 - dy;
    x1 = x0 - dx;
    y2 = y0 + dy;
    x2 = x0 + dx;
    if cheat_room != 0 {
        msg_print(b"Castle Vault\x00" as *const u8 as *const libc::c_char);
    }
    /* Generate the room */
    y = y1 - 1 as libc::c_int;
    while y <= y2 + 1 as libc::c_int {
        x = x1 - 1 as libc::c_int;
        while x <= x2 + 1 as libc::c_int {
            let ref mut fresh25 = (*cave[y as usize].offset(x as isize)).info;
            *fresh25 =
                (*fresh25 as libc::c_int |
                     (0x8 as libc::c_int | 0x4 as libc::c_int)) as u16b;
            /* Make everything a floor */
            place_floor(y, x);
            x += 1
        }
        y += 1
    }
    /* Make the castle */
    build_recursive_room(x1, y1, x2, y2,
                         Rand_div(5 as libc::c_int) + 1 as libc::c_int);
    /* Fill with monsters and treasure, low difficulty */
    fill_treasure(x1, x2, y1, y2,
                  Rand_div(3 as libc::c_int) + 1 as libc::c_int);
}
/*
 * Add outer wall to a floored region
 *
 * Note: no range checking is done so must be inside dungeon
 * This routine also stomps on doors
 */
unsafe extern "C" fn add_outer_wall(mut x: libc::c_int, mut y: libc::c_int,
                                    mut light: libc::c_int,
                                    mut x1: libc::c_int, mut y1: libc::c_int,
                                    mut x2: libc::c_int,
                                    mut y2: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
             y < cur_hgt as libc::c_int - 1 as libc::c_int &&
             x < cur_wid as libc::c_int - 1 as libc::c_int) {
        return
    }
    /*
	 * Hack -- Check to see if square has been visited before
	 * if so, then exit (use room flag to do this)
	 */
    if (*cave[y as usize].offset(x as isize)).info as libc::c_int &
           0x8 as libc::c_int != 0 {
        return
    }
    /* Set room flag */
    let ref mut fresh26 = (*cave[y as usize].offset(x as isize)).info;
    *fresh26 = (*fresh26 as libc::c_int | 0x8 as libc::c_int) as u16b;
    if get_is_floor(x, y) != 0 {
        i = -(1 as libc::c_int);
        while i <= 1 as libc::c_int {
            j = -(1 as libc::c_int);
            while j <= 1 as libc::c_int {
                if x + i >= x1 && x + i <= x2 && y + j >= y1 && y + j <= y2 {
                    add_outer_wall(x + i, y + j, light, x1, y1, x2, y2);
                    if light != 0 {
                        let ref mut fresh27 =
                            (*cave[y as usize].offset(x as isize)).info;
                        *fresh27 =
                            (*fresh27 as libc::c_int | 0x2 as libc::c_int) as
                                u16b
                    }
                }
                j += 1
            }
            i += 1
        }
    } else if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                  0x38 as libc::c_int {
        (*cave[y as usize].offset(x as isize)).feat = feat_wall_outer;
        if light == 1 as libc::c_int {
            let ref mut fresh28 = (*cave[y as usize].offset(x as isize)).info;
            *fresh28 = (*fresh28 as libc::c_int | 0x2 as libc::c_int) as u16b
        }
    } else if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                  0x3e as libc::c_int {
        if light == 1 as libc::c_int {
            let ref mut fresh29 = (*cave[y as usize].offset(x as isize)).info;
            *fresh29 = (*fresh29 as libc::c_int | 0x2 as libc::c_int) as u16b
        }
    };
}
/* Set bounding walls */
/* Set bounding walls */
/*
 * Hacked distance formula - gives the 'wrong' answer
 *
 * Used to build crypts
 */
unsafe extern "C" fn dist2(mut x1: libc::c_int, mut y1: libc::c_int,
                           mut x2: libc::c_int, mut y2: libc::c_int,
                           mut h1: libc::c_int, mut h2: libc::c_int,
                           mut h3: libc::c_int, mut h4: libc::c_int)
 -> libc::c_int {
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    dx = abs(x2 - x1);
    dy = abs(y2 - y1);
    /*
	 * Basically this works by taking the normal pythagorean formula
	 * and using an expansion to express this in a way without the
	 * square root.  This approximate formula is then perturbed to give
	 * the distorted results.  (I found this by making a mistake when I was
	 * trying to fix the circular rooms.)
	 */
    /* h1-h4 are constants that describe the metric */
    if dx >= 2 as libc::c_int * dy { return dx + dy * h1 / h2 }
    if dy >= 2 as libc::c_int * dx { return dy + dx * h1 / h2 }
    /* 128/181 is approx. 1/sqrt(2) */
    return (dx + dy) * 128 as libc::c_int / 181 as libc::c_int +
               (dx * dx / (dy * h3) + dy * dy / (dx * h3)) * h4;
}
/*
 * Build target vault
 *
 * This is made by two concentric "crypts" with perpendicular
 * walls creating the cross-hairs.
 */
unsafe extern "C" fn build_target_vault(mut x0: libc::c_int,
                                        mut y0: libc::c_int,
                                        mut xsize: libc::c_int,
                                        mut ysize: libc::c_int) {
    let mut rad: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut h1: libc::c_int = 0;
    let mut h2: libc::c_int = 0;
    let mut h3: libc::c_int = 0;
    let mut h4: libc::c_int = 0;
    /* Make a random metric */
    h1 = Rand_div(32 as libc::c_int) + 1 as libc::c_int - 16 as libc::c_int;
    h2 = Rand_div(16 as libc::c_int) + 1 as libc::c_int;
    h3 = Rand_div(32 as libc::c_int) + 1 as libc::c_int;
    h4 = Rand_div(32 as libc::c_int) + 1 as libc::c_int - 16 as libc::c_int;
    if cheat_room != 0 {
        msg_print(b"Target Vault\x00" as *const u8 as *const libc::c_char);
    }
    /* Work out outer radius */
    if xsize > ysize {
        rad = ysize / 2 as libc::c_int
    } else { rad = xsize / 2 as libc::c_int }
    /* Make floor */
    x = x0 - rad;
    while x <= x0 + rad {
        y = y0 - rad;
        while y <= y0 + rad {
            let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
            /* Access the grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            /* Clear room flag */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int & !(0x8 as libc::c_int)) as
                    u16b;
            /* Grids in vaults are required to be "icky" */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int | 0x4 as libc::c_int) as u16b;
            /* Inside -- floor */
            if dist2(y0, x0, y, x, h1, h2, h3, h4) <= rad - 1 as libc::c_int {
                place_floor(y, x);
            } else {
                /* Outside -- make it granite so that arena works */
                (*c_ptr).feat = 0x38 as libc::c_int as byte_hack
            }
            /* Proper boundary for arena */
            if y + rad == y0 || y - rad == y0 || x + rad == x0 ||
                   x - rad == x0 {
                cave_set_feat(y, x, feat_wall_outer as libc::c_int);
            }
            y += 1
        }
        x += 1
    }
    /* Find visible outer walls and set to be FEAT_OUTER */
    add_outer_wall(x0, y0, 0 as libc::c_int, x0 - rad - 1 as libc::c_int,
                   y0 - rad - 1 as libc::c_int, x0 + rad + 1 as libc::c_int,
                   y0 + rad + 1 as libc::c_int);
    /* Add inner wall */
    x = x0 - rad / 2 as libc::c_int;
    while x <= x0 + rad / 2 as libc::c_int {
        y = y0 - rad / 2 as libc::c_int;
        while y <= y0 + rad / 2 as libc::c_int {
            if dist2(y0, x0, y, x, h1, h2, h3, h4) == rad / 2 as libc::c_int {
                /* Make an internal wall */
                cave_set_feat(y, x, feat_wall_inner as libc::c_int);
            }
            y += 1
        }
        x += 1
    }
    /* Add perpendicular walls */
    x = x0 - rad;
    while x <= x0 + rad {
        cave_set_feat(y0, x, feat_wall_inner as libc::c_int);
        x += 1
    }
    y = y0 - rad;
    while y <= y0 + rad {
        cave_set_feat(y, x0, feat_wall_inner as libc::c_int);
        y += 1
    }
    /* Make inner vault */
    y = y0 - 1 as libc::c_int;
    while y <= y0 + 1 as libc::c_int {
        cave_set_feat(y, x0 - 1 as libc::c_int,
                      feat_wall_inner as libc::c_int);
        cave_set_feat(y, x0 + 1 as libc::c_int,
                      feat_wall_inner as libc::c_int);
        y += 1
    }
    x = x0 - 1 as libc::c_int;
    while x <= x0 + 1 as libc::c_int {
        cave_set_feat(y0 - 1 as libc::c_int, x,
                      feat_wall_inner as libc::c_int);
        cave_set_feat(y0 + 1 as libc::c_int, x,
                      feat_wall_inner as libc::c_int);
        x += 1
    }
    place_floor(y0, x0);
    /*
	 * Add doors to vault
	 *
	 * Get two distances so can place doors relative to centre
	 */
    x = (rad - 2 as libc::c_int) / 4 as libc::c_int + 1 as libc::c_int;
    y = rad / 2 as libc::c_int + x;
    add_door(x0 + x, y0);
    add_door(x0 + y, y0);
    add_door(x0 - x, y0);
    add_door(x0 - y, y0);
    add_door(x0, y0 + x);
    add_door(x0, y0 + y);
    add_door(x0, y0 - x);
    add_door(x0, y0 - y);
    /* Fill with stuff - medium difficulty */
    fill_treasure(x0 - rad, x0 + rad, y0 - rad, y0 + rad,
                  Rand_div(3 as libc::c_int) + 1 as libc::c_int +
                      3 as libc::c_int);
}
/*
 * Random vaults
 */
unsafe extern "C" fn build_type11(mut by0: libc::c_int,
                                  mut bx0: libc::c_int) {
    let mut y0: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut xsize: libc::c_int = 0;
    let mut ysize: libc::c_int = 0;
    let mut vtype: libc::c_int = 0;
    /* Get size -- gig enough to look good, small enough to be fairly common */
    xsize =
        Rand_div(22 as libc::c_int) + 1 as libc::c_int + 22 as libc::c_int;
    ysize =
        Rand_div(11 as libc::c_int) + 1 as libc::c_int + 11 as libc::c_int;
    /* Allocate in room_map.  If will not fit, exit */
    if room_alloc(xsize + 2 as libc::c_int, ysize + 2 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut x0, &mut y0) == 0
       {
        return
    }
    /*
	 * Boost the rating -- Higher than lesser vaults and lower than
	 * greater vaults
	 */
    rating = (rating as libc::c_int + 10 as libc::c_int) as s16b;
    /* (Sometimes) Cause a special feeling */
    if dun_level as libc::c_int <= 50 as libc::c_int ||
           (Rand_div((dun_level as libc::c_int - 40 as libc::c_int) *
                         (dun_level as libc::c_int - 40 as libc::c_int) +
                         1 as libc::c_int) + 1 as libc::c_int) <
               400 as libc::c_int {
        good_item_flag = 1 as libc::c_int as bool_
    }
    /* Select type of vault */
    vtype = Rand_div(8 as libc::c_int) + 1 as libc::c_int;
    match vtype {
        1 => {
            /* Build an appropriate room */
            build_bubble_vault(x0, y0, xsize, ysize);
        }
        2 => { build_room_vault(x0, y0, xsize, ysize); }
        3 => { build_cave_vault(x0, y0, xsize, ysize); }
        4 => { build_maze_vault(x0, y0, xsize, ysize); }
        5 => { build_mini_c_vault(x0, y0, xsize, ysize); }
        6 => { build_castle_vault(x0, y0, xsize, ysize); }
        7 => { build_target_vault(x0, y0, xsize, ysize); }
        _ => {
            /* I know how to add a few more... give me some time. */
            /* Paranoia */
            return
        }
    };
}
/*
 * Crypt room generation from Z 2.5.1
 */
/*
 * Build crypt room.
 * For every grid in the possible square, check the (fake) distance.
 * If it's less than the radius, make it a room square.
 *
 * When done fill from the inside to find the walls,
 */
unsafe extern "C" fn build_type12(mut by0: libc::c_int,
                                  mut bx0: libc::c_int) {
    let mut light: libc::c_int = 0;
    let mut rad: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    let mut emptyflag: bool_ = 1 as libc::c_int as bool_;
    let mut h1: libc::c_int = 0;
    let mut h2: libc::c_int = 0;
    let mut h3: libc::c_int = 0;
    let mut h4: libc::c_int = 0;
    /* Make a random metric */
    h1 = Rand_div(32 as libc::c_int) + 1 as libc::c_int - 16 as libc::c_int;
    h2 = Rand_div(16 as libc::c_int) + 1 as libc::c_int;
    h3 = Rand_div(32 as libc::c_int) + 1 as libc::c_int;
    h4 = Rand_div(32 as libc::c_int) + 1 as libc::c_int - 16 as libc::c_int;
    /* Occasional light */
    light =
        if Rand_div(dun_level as s32b) + 1 as libc::c_int <= 5 as libc::c_int
           {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    rad = Rand_div(9 as libc::c_int) + 1 as libc::c_int;
    /* Allocate in room_map.  If will not fit, exit */
    if room_alloc(rad * 2 as libc::c_int + 3 as libc::c_int,
                  rad * 2 as libc::c_int + 3 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut x0, &mut y0) == 0
       {
        return
    }
    /* Make floor */
    x = x0 - rad;
    while x <= x0 + rad {
        y = y0 - rad;
        while y <= y0 + rad {
            /* Clear room flag */
            let ref mut fresh30 = (*cave[y as usize].offset(x as isize)).info;
            *fresh30 =
                (*fresh30 as libc::c_int & !(0x8 as libc::c_int)) as u16b;
            /* Inside -- floor */
            if dist2(y0, x0, y, x, h1, h2, h3, h4) <= rad - 1 as libc::c_int {
                place_floor(y, x);
            } else if distance(y0, x0, y, x) < 3 as libc::c_int {
                place_floor(y, x);
            } else {
                /* Outside -- make it granite so that arena works */
                cave_set_feat(y, x, feat_wall_outer as libc::c_int);
            }
            /* Proper boundary for arena */
            if y + rad == y0 || y - rad == y0 || x + rad == x0 ||
                   x - rad == x0 {
                cave_set_feat(y, x, feat_wall_outer as libc::c_int);
            }
            y += 1
        }
        x += 1
    }
    /* Find visible outer walls and set to be FEAT_OUTER */
    add_outer_wall(x0, y0, light, x0 - rad - 1 as libc::c_int,
                   y0 - rad - 1 as libc::c_int, x0 + rad + 1 as libc::c_int,
                   y0 + rad + 1 as libc::c_int);
    /* Check to see if there is room for an inner vault */
    x = x0 - 2 as libc::c_int;
    while x <= x0 + 2 as libc::c_int {
        y = y0 - 2 as libc::c_int;
        while y <= y0 + 2 as libc::c_int {
            if get_is_floor(x, y) == 0 {
                /* Wall in the way */
                emptyflag = 0 as libc::c_int as bool_
            }
            y += 1
        }
        x += 1
    }
    if emptyflag as libc::c_int != 0 &&
           Rand_div(2 as libc::c_int) == 0 as libc::c_int {
        /* Build the vault */
        build_small_room(x0, y0);
        /* Place a treasure in the vault */
        place_object(y0, x0, 0 as libc::c_int as bool_,
                     0 as libc::c_int as bool_, 2 as libc::c_int);
        /* Let's guard the treasure well */
        vault_monsters(y0, x0, Rand_div(2 as libc::c_int) + 3 as libc::c_int);
        /* Traps naturally */
        vault_traps(y0, x0, 4 as libc::c_int, 4 as libc::c_int,
                    Rand_div(3 as libc::c_int) + 2 as libc::c_int);
    };
}
/*
 * Constructs a tunnel between two points
 *
 * This function must be called BEFORE any streamers are created,
 * since we use the special "granite wall" sub-types to keep track
 * of legal places for corridors to pierce rooms.
 *
 * We use "door_flag" to prevent excessive construction of doors
 * along overlapping corridors.
 *
 * We queue the tunnel grids to prevent door creation along a corridor
 * which intersects itself.
 *
 * We queue the wall piercing grids to prevent a corridor from leaving
 * a room and then coming back in through the same entrance.
 *
 * We "pierce" grids which are "outer" walls of rooms, and when we
 * do so, we change all adjacent "outer" walls of rooms into "solid"
 * walls so that no two corridors may use adjacent grids for exits.
 *
 * The "solid" wall check prevents corridors from "chopping" the
 * corners of rooms off, as well as "silly" door placement, and
 * "excessively wide" room entrances.
 *
 * Useful "feat" values:
 *   FEAT_WALL_EXTRA -- granite walls
 *   FEAT_WALL_INNER -- inner room walls
 *   FEAT_WALL_OUTER -- outer room walls
 *   FEAT_WALL_SOLID -- solid room walls
 *   FEAT_PERM_EXTRA -- shop walls (perma)
 *   FEAT_PERM_INNER -- inner room walls (perma)
 *   FEAT_PERM_OUTER -- outer room walls (perma)
 *   FEAT_PERM_SOLID -- dungeon border (perma)
 */
unsafe extern "C" fn build_tunnel(mut row1: libc::c_int,
                                  mut col1: libc::c_int,
                                  mut row2: libc::c_int,
                                  mut col2: libc::c_int, mut water: bool_) {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut tmp_row: libc::c_int = 0;
    let mut tmp_col: libc::c_int = 0;
    let mut row_dir: libc::c_int = 0;
    let mut col_dir: libc::c_int = 0;
    let mut start_row: libc::c_int = 0;
    let mut start_col: libc::c_int = 0;
    let mut main_loop_count: libc::c_int = 0 as libc::c_int;
    let mut door_flag: bool_ = 0 as libc::c_int as bool_;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Reset the arrays */
    (*dun).tunn_n = 0 as libc::c_int;
    (*dun).wall_n = 0 as libc::c_int;
    /* Save the starting location */
    start_row = row1;
    start_col = col1;
    /* Start out in the correct direction */
    correct_dir(&mut row_dir, &mut col_dir, row1, col1, row2, col2);
    /* Keep going until done (or bored) */
    while row1 != row2 || col1 != col2 {
        /* Mega-Hack -- Paranoia -- prevent infinite loops */
        let fresh31 = main_loop_count;
        main_loop_count = main_loop_count + 1;
        if fresh31 > 2000 as libc::c_int { break ; }
        /* Allow bends in the tunnel */
        if Rand_div(100 as libc::c_int) < 30 as libc::c_int {
            /* Acquire the correct direction */
            correct_dir(&mut row_dir, &mut col_dir, row1, col1, row2, col2);
            /* Random direction */
            if Rand_div(100 as libc::c_int) < 10 as libc::c_int {
                rand_dir(&mut row_dir, &mut col_dir);
            }
        }
        /* Get the next location */
        tmp_row = row1 + row_dir;
        tmp_col = col1 + col_dir;
        /* Extremely Important -- do not leave the dungeon */
        while !(tmp_row > 0 as libc::c_int && tmp_col > 0 as libc::c_int &&
                    tmp_row < cur_hgt as libc::c_int - 1 as libc::c_int &&
                    tmp_col < cur_wid as libc::c_int - 1 as libc::c_int) {
            /* Acquire the correct direction */
            correct_dir(&mut row_dir, &mut col_dir, row1, col1, row2, col2);
            /* Random direction */
            if Rand_div(100 as libc::c_int) < 10 as libc::c_int {
                rand_dir(&mut row_dir, &mut col_dir);
            }
            /* Get the next location */
            tmp_row = row1 + row_dir;
            tmp_col = col1 + col_dir
        }
        /* Access the location */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(tmp_row as
                                                 isize)).offset(tmp_col as
                                                                    isize) as
                *mut cave_type;
        /* Avoid the edge of the dungeon */
        if (*c_ptr).feat as libc::c_int == 0x3f as libc::c_int { continue ; }
        /* Avoid the edge of vaults */
        if (*c_ptr).feat as libc::c_int == 0x3e as libc::c_int { continue ; }
        /* Avoid "solid" granite walls */
        if (*c_ptr).feat as libc::c_int == 0x3b as libc::c_int { continue ; }
        /*
		 * Pierce "outer" walls of rooms
		 * Cannot trust feat code any longer...
		 */
        if (*c_ptr).feat as libc::c_int == feat_wall_outer as libc::c_int &&
               (*c_ptr).info as libc::c_int & 0x8 as libc::c_int != 0 {
            /* Acquire the "next" location */
            y = tmp_row + row_dir;
            x = tmp_col + col_dir;
            /* Hack -- Avoid outer/solid permanent walls */
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   0x3f as libc::c_int {
                continue ;
            }
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   0x3e as libc::c_int {
                continue ;
            }
            /* Hack -- Avoid outer/solid granite walls */
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   feat_wall_outer as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x8 as libc::c_int != 0 {
                continue ;
            }
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   0x3b as libc::c_int {
                continue ;
            }
            /* Accept this location */
            row1 = tmp_row;
            col1 = tmp_col;
            /* Save the wall location */
            if (*dun).wall_n < 500 as libc::c_int {
                (*dun).wall[(*dun).wall_n as usize].y = row1 as byte_hack;
                (*dun).wall[(*dun).wall_n as usize].x = col1 as byte_hack;
                (*dun).wall_n += 1
            }
            /* Forbid re-entry near this piercing */
            y = row1 - 1 as libc::c_int;
            while y <= row1 + 1 as libc::c_int {
                x = col1 - 1 as libc::c_int;
                while x <= col1 + 1 as libc::c_int {
                    /* Convert adjacent "outer" walls as "solid" walls */
                    if (*cave[y as usize].offset(x as isize)).feat as
                           libc::c_int == feat_wall_outer as libc::c_int &&
                           (*cave[y as usize].offset(x as isize)).info as
                               libc::c_int & 0x8 as libc::c_int != 0 {
                        /* Change the wall to a "solid" wall */
						/* Mega-Hack -- to be brought back later... */
                        cave_set_feat(y, x, 0x3b as libc::c_int);
                    }
                    x += 1
                }
                y += 1
            }
        } else if (*c_ptr).info as libc::c_int & 0x8 as libc::c_int != 0 {
            /* Travel quickly through rooms */
            /* Accept the location */
            row1 = tmp_row;
            col1 = tmp_col
        } else if (*c_ptr).feat as libc::c_int ==
                      (*d_info.offset(dungeon_type as isize)).fill_type1 as
                          libc::c_int ||
                      (*c_ptr).feat as libc::c_int ==
                          (*d_info.offset(dungeon_type as isize)).fill_type2
                              as libc::c_int ||
                      (*c_ptr).feat as libc::c_int ==
                          (*d_info.offset(dungeon_type as isize)).fill_type3
                              as libc::c_int {
            /* Tunnel through all other walls */
            /* Accept this location */
            row1 = tmp_row;
            col1 = tmp_col;
            /* Save the tunnel location */
            if (*dun).tunn_n < 900 as libc::c_int {
                (*dun).tunn[(*dun).tunn_n as usize].y = row1 as byte_hack;
                (*dun).tunn[(*dun).tunn_n as usize].x = col1 as byte_hack;
                (*dun).tunn_n += 1
            }
            /* Allow door in next grid */
            door_flag = 0 as libc::c_int as bool_
        } else {
            /* Handle corridor intersections or overlaps */
            /* Accept the location */
            row1 = tmp_row;
            col1 = tmp_col;
            /* Collect legal door locations */
            if door_flag == 0 {
                /* Save the door location */
                if (*dun).door_n < 200 as libc::c_int {
                    (*dun).door[(*dun).door_n as usize].y = row1 as byte_hack;
                    (*dun).door[(*dun).door_n as usize].x = col1 as byte_hack;
                    (*dun).door_n += 1
                }
                /* No door in next grid */
                door_flag = 1 as libc::c_int as bool_
            }
            /* Hack -- allow pre-emptive tunnel termination */
            if !(Rand_div(100 as libc::c_int) >= 15 as libc::c_int) {
                continue ;
            }
            /* Distance between row1 and start_row */
            tmp_row = row1 - start_row;
            if tmp_row < 0 as libc::c_int { tmp_row = -tmp_row }
            /* Distance between col1 and start_col */
            tmp_col = col1 - start_col;
            if tmp_col < 0 as libc::c_int { tmp_col = -tmp_col }
            /* Terminate the tunnel */
            if tmp_row > 10 as libc::c_int || tmp_col > 10 as libc::c_int {
                break ;
            }
        }
    }
    /* Turn the tunnel into corridor */
    i = 0 as libc::c_int;
    while i < (*dun).tunn_n {
        /* Access the grid */
        y = (*dun).tunn[i as usize].y as libc::c_int;
        x = (*dun).tunn[i as usize].x as libc::c_int;
        /* Access the grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Clear previous contents, add a floor */
        if water == 0 {
            place_floor(y, x);
        } else { cave_set_feat(y, x, 0x54 as libc::c_int); }
        i += 1
    }
    /* Apply the piercings that we found */
    i = 0 as libc::c_int;
    while i < (*dun).wall_n {
        /* Access the grid */
        y = (*dun).wall[i as usize].y as libc::c_int;
        x = (*dun).wall[i as usize].x as libc::c_int;
        /* Access the grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Clear previous contents, add up floor */
        place_floor(y, x);
        /* Occasional doorway */
        if dungeon_flags1 as libc::c_long & 0x20 as libc::c_long == 0 &&
               Rand_div(100 as libc::c_int) < 25 as libc::c_int {
            /* Place a random door */
            place_random_door(y, x);
        }
        i += 1
    };
}
/*
 * Count the number of "corridor" grids adjacent to the given grid.
 *
 * Note -- Assumes "in_bounds(y1, x1)"
 *
 * XXX XXX This routine currently only counts actual "empty floor"
 * grids which are not in rooms.  We might want to also count stairs,
 * open doors, closed doors, etc.
 */
unsafe extern "C" fn next_to_corr(mut y1: libc::c_int, mut x1: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Scan adjacent grids */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        /* Extract the location */
        y = y1 + ddy_ddd[i as usize] as libc::c_int;
        x = x1 + ddx_ddd[i as usize] as libc::c_int;
        /* Skip non floors */
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int {
            /* Access the grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            /* Skip non "empty floor" grids */
            if !((*c_ptr).feat as libc::c_int !=
                     (*d_info.offset(dungeon_type as isize)).floor1 as
                         libc::c_int &&
                     (*c_ptr).feat as libc::c_int !=
                         (*d_info.offset(dungeon_type as isize)).floor2 as
                             libc::c_int &&
                     (*c_ptr).feat as libc::c_int !=
                         (*d_info.offset(dungeon_type as isize)).floor3 as
                             libc::c_int) {
                /* Skip grids inside rooms */
                if !((*c_ptr).info as libc::c_int & 0x8 as libc::c_int != 0) {
                    /* Count these grids */
                    k += 1
                }
            }
        }
        i += 1
    }
    /* Return the number of corridors */
    return k;
}
/*
 * Determine if the given location is "between" two walls,
 * and "next to" two corridor spaces.  XXX XXX XXX
 *
 * Assumes "in_bounds(y,x)"
 */
unsafe extern "C" fn possible_doorway(mut y: libc::c_int, mut x: libc::c_int)
 -> bool_ {
    /* Count the adjacent corridors */
    if next_to_corr(y, x) >= 2 as libc::c_int {
        /* Check Vertical */
        if (*f_info.offset((*cave[(y - 1 as libc::c_int) as
                                      usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x20 as libc::c_long != 0 &&
               (*f_info.offset((*cave[(y + 1 as libc::c_int) as
                                          usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x20 as libc::c_long != 0 {
            return 1 as libc::c_int as bool_
        }
        /* Check Horizontal */
        if (*f_info.offset((*cave[y as
                                      usize].offset((x - 1 as libc::c_int) as
                                                        isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x20 as libc::c_long != 0 &&
               (*f_info.offset((*cave[y as
                                          usize].offset((x + 1 as libc::c_int)
                                                            as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x20 as libc::c_long != 0 {
            return 1 as libc::c_int as bool_
        }
    }
    /* No doorway */
    return 0 as libc::c_int as bool_;
}
/*
 * Places doors around y, x position
 */
unsafe extern "C" fn try_doors(mut y: libc::c_int, mut x: libc::c_int) {
    let mut dir_ok: [bool_; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut xx: libc::c_int = 0;
    /* Paranoia */
	/* if (!in_bounds(y, x)) return; */
    /* Some dungeons don't have doors at all */
    if dungeon_flags1 as libc::c_long & 0x20 as libc::c_long != 0 { return }
    /* Reset tally */
    n = 0 as libc::c_int;
    /* Look four cardinal directions */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        /* Assume NG */
        dir_ok[i as usize] = 0 as libc::c_int as bool_;
        /* Access location */
        yy = y + ddy_ddd[i as usize] as libc::c_int;
        xx = x + ddx_ddd[i as usize] as libc::c_int;
        /* Out of level boundary */
        if yy > 0 as libc::c_int && xx > 0 as libc::c_int &&
               yy < cur_hgt as libc::c_int - 1 as libc::c_int &&
               xx < cur_wid as libc::c_int - 1 as libc::c_int {
            /* Ignore walls */
            if !((*f_info.offset((*cave[yy as usize].offset(xx as isize)).feat
                                     as isize)).flags1 as libc::c_long &
                     0x20 as libc::c_long != 0) {
                /* Ignore room grids */
                if !((*cave[yy as usize].offset(xx as isize)).info as
                         libc::c_int & 0x8 as libc::c_int != 0) {
                    /* Not a doorway */
                    if !(possible_doorway(yy, xx) == 0) {
                        /* Accept the direction */
                        dir_ok[i as usize] = 1 as libc::c_int as bool_;
                        /* Count good spots */
                        n += 1
                    }
                }
            }
        }
        i += 1
    }
    /* Use the traditional method 75% of time */
    if Rand_div(100 as libc::c_int) < 75 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            /* Bad locations */
            if !(dir_ok[i as usize] == 0) {
                /* Place one of various kinds of doors */
                if Rand_div(100 as libc::c_int) < 90 as libc::c_int {
                    /* Access location */
                    yy = y + ddy_ddd[i as usize] as libc::c_int;
                    xx = x + ddx_ddd[i as usize] as libc::c_int;
                    /* Place a door */
                    place_random_door(yy, xx);
                }
            }
            i += 1
        }
    } else {
        /* Use alternative method */
        /* A crossroad */
        if n == 4 as libc::c_int {
            /* Clear OK flags XXX */
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                dir_ok[i as usize] = 0 as libc::c_int as bool_;
                i += 1
            }
            /* Put one or two secret doors */
            dir_ok[Rand_div(4 as libc::c_int) as usize] =
                1 as libc::c_int as bool_;
            dir_ok[Rand_div(4 as libc::c_int) as usize] =
                1 as libc::c_int as bool_
        } else if n == 3 as libc::c_int || n == 2 as libc::c_int {
            /* A T-shaped intersection or two possible doorways */
            /* Pick one random location from the list */
            k = Rand_div(n);
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                /* Reject all but k'th OK direction */
                if dir_ok[i as usize] as libc::c_int != 0 &&
                       {
                           let fresh32 = k;
                           k = k - 1;
                           (fresh32) != 0 as libc::c_int
                       } {
                    dir_ok[i as usize] = 0 as libc::c_int as bool_
                }
                i += 1
            }
        }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            /* Place secret door(s) */
            /* Bad location */
            if !(dir_ok[i as usize] == 0) {
                /* Access location */
                yy = y + ddy_ddd[i as usize] as libc::c_int;
                xx = x + ddx_ddd[i as usize] as libc::c_int;
                /* Place a secret door */
                place_secret_door(yy, xx);
            }
            i += 1
        }
    };
}
/*
 * Attempt to build a room of the given type at the given block
 *
 * Note that we restrict the number of "crowded" rooms to reduce
 * the chance of overflowing the monster list during level creation.
 */
unsafe extern "C" fn room_build(mut y: libc::c_int, mut x: libc::c_int,
                                mut typ: libc::c_int) -> bool_ {
    /* Restrict level */
    if (dun_level as libc::c_int) < roomdep[typ as usize] as libc::c_int &&
           ironman_rooms == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Restrict "crowded" rooms */
    if (*dun).crowded as libc::c_int != 0 &&
           (typ == 5 as libc::c_int || typ == 6 as libc::c_int) {
        return 0 as libc::c_int as bool_
    }
    /* Build a room */
    match typ {
        12 => {
            /* Build an appropriate room */
            build_type12(y, x);
        }
        11 => { build_type11(y, x); }
        10 => { build_type10(y, x); }
        9 => { build_type9(y, x); }
        8 => { build_type8(y, x); }
        7 => { build_type7(y, x); }
        6 => { build_type6(y, x); }
        5 => { build_type5(y, x); }
        4 => { build_type4(y, x); }
        3 => { build_type3(y, x); }
        2 => { build_type2(y, x); }
        1 => { build_type1(y, x); }
        _ => {
            /* Paranoia */
            return 0 as libc::c_int as bool_
        }
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Set level boundaries
 */
#[no_mangle]
pub unsafe extern "C" fn set_bounders(mut empty_level: bool_) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Special boundary walls -- Top */
    x = 0 as libc::c_int;
    while x < cur_wid as libc::c_int {
        /* XXX XXX */
        if empty_level != 0 {
            (*cave[0 as libc::c_int as usize].offset(x as isize)).mimic =
                fill_type[Rand_div(100 as libc::c_int) as usize] as byte_hack
        } else {
            (*cave[0 as libc::c_int as usize].offset(x as isize)).mimic =
                (*cave[0 as libc::c_int as usize].offset(x as isize)).feat
        }
        /* Clear previous contents, add "solid" perma-wall */
        cave_set_feat(0 as libc::c_int, x, 0x3f as libc::c_int);
        x += 1
    }
    /* Special boundary walls -- Bottom */
    x = 0 as libc::c_int;
    while x < cur_wid as libc::c_int {
        /* XXX XXX */
        if empty_level != 0 {
            (*cave[(cur_hgt as libc::c_int - 1 as libc::c_int) as
                       usize].offset(x as isize)).mimic =
                fill_type[Rand_div(100 as libc::c_int) as usize] as byte_hack
        } else {
            (*cave[(cur_hgt as libc::c_int - 1 as libc::c_int) as
                       usize].offset(x as isize)).mimic =
                (*cave[(cur_hgt as libc::c_int - 1 as libc::c_int) as
                           usize].offset(x as isize)).feat
        }
        /* Clear previous contents, add "solid" perma-wall */
        cave_set_feat(cur_hgt as libc::c_int - 1 as libc::c_int, x,
                      0x3f as libc::c_int);
        x += 1
    }
    /* Special boundary walls --  Left */
    y = 1 as libc::c_int;
    while y < cur_hgt as libc::c_int - 1 as libc::c_int {
        /* XXX XXX */
        if empty_level != 0 {
            (*cave[y as usize].offset(0 as libc::c_int as isize)).mimic =
                fill_type[Rand_div(100 as libc::c_int) as usize] as byte_hack
        } else {
            (*cave[y as usize].offset(0 as libc::c_int as isize)).mimic =
                (*cave[y as usize].offset(0 as libc::c_int as isize)).feat
        }
        /* Clear previous contents, add "solid" perma-wall */
        cave_set_feat(y, 0 as libc::c_int, 0x3f as libc::c_int);
        y += 1
    }
    /* Special boundary walls --  Right */
    y = 1 as libc::c_int;
    while y < cur_hgt as libc::c_int - 1 as libc::c_int {
        /* XXX XXX */
        if empty_level != 0 {
            (*cave[y as
                       usize].offset((cur_wid as libc::c_int -
                                          1 as libc::c_int) as isize)).mimic =
                fill_type[Rand_div(100 as libc::c_int) as usize] as byte_hack
        } else {
            (*cave[y as
                       usize].offset((cur_wid as libc::c_int -
                                          1 as libc::c_int) as isize)).mimic =
                (*cave[y as
                           usize].offset((cur_wid as libc::c_int -
                                              1 as libc::c_int) as
                                             isize)).feat
        }
        /* Clear previous contents, add "solid" perma-wall */
        cave_set_feat(y, cur_wid as libc::c_int - 1 as libc::c_int,
                      0x3f as libc::c_int);
        y += 1
    };
}
/*
 * Generate a normal dungeon level
 */
#[no_mangle]
pub unsafe extern "C" fn level_generate_dungeon() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut branch: libc::c_int = get_branch();
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    let mut max_vault_ok: libc::c_int = 2 as libc::c_int;
    let mut destroyed: bool_ = 0 as libc::c_int as bool_;
    let mut empty_level: bool_ = 0 as libc::c_int as bool_;
    let mut cavern: bool_ = 0 as libc::c_int as bool_;
    let mut town_level: s16b = 0 as libc::c_int as s16b;
    /* Is it a town level ? */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*d_ptr).t_level[i as usize] as libc::c_int ==
               dun_level as libc::c_int {
            town_level = (*d_ptr).t_idx[i as usize]
        }
        i += 1
    }
    /* Check for arena level */
    if dungeon_flags1 as libc::c_long & 0x100000 as libc::c_long != 0 ||
           empty_levels as libc::c_int != 0 &&
               Rand_div(15 as libc::c_int) == 0 as libc::c_int {
        empty_level = 1 as libc::c_int as bool_;
        if cheat_room as libc::c_int != 0 ||
               (*p_ptr).precognition as libc::c_int != 0 {
            msg_print(b"Arena level.\x00" as *const u8 as
                          *const libc::c_char);
        }
        /* Refill the level with floor tiles */
        fill_level(empty_level, (*d_ptr).fill_method);
    }
    /* Possible cavern */
    if dungeon_flags1 as libc::c_long & 0x800 as libc::c_long != 0 &&
           Rand_div(dun_level as libc::c_int / 2 as libc::c_int) >
               30 as libc::c_int {
        cavern = 1 as libc::c_int as bool_;
        /* Make a large fractal cave in the middle of the dungeon */
        if cheat_room != 0 {
            msg_print(b"Cavern on level.\x00" as *const u8 as
                          *const libc::c_char);
        }
        build_cavern();
    }
    /* Possible "destroyed" level */
    if dun_level as libc::c_int > 10 as libc::c_int &&
           Rand_div(18 as libc::c_int) == 0 as libc::c_int {
        destroyed = 1 as libc::c_int as bool_
    }
    /* Hack -- No destroyed "quest" levels */
    if is_quest(dun_level as libc::c_int) != 0 {
        destroyed = 0 as libc::c_int as bool_
    }
    /* Hack -- No destroyed "small" levels */
    if cur_wid as libc::c_int != 198 as libc::c_int ||
           cur_hgt as libc::c_int != 66 as libc::c_int {
        destroyed = 0 as libc::c_int as bool_
    }
    /* Hack -- No destroyed levels */
    if dungeon_flags1 as libc::c_long & 0x20000 as libc::c_long != 0 {
        destroyed = 0 as libc::c_int as bool_
    }
    /* Actual maximum number of rooms on this level */
    (*dun).row_rooms = cur_hgt as libc::c_int / 11 as libc::c_int;
    (*dun).col_rooms = cur_wid as libc::c_int / 11 as libc::c_int;
    /* Initialize the room table */
    y = 0 as libc::c_int;
    while y < (*dun).row_rooms {
        x = 0 as libc::c_int;
        while x < (*dun).col_rooms {
            (*dun).room_map[y as usize][x as usize] =
                0 as libc::c_int as bool_;
            x += 1
        }
        y += 1
    }
    /* No "crowded" rooms yet */
    (*dun).crowded = 0 as libc::c_int as bool_;
    /* No rooms yet */
    (*dun).cent_n = 0 as libc::c_int;
    /* Pick a block for the room */
    y = Rand_div((*dun).row_rooms);
    x = Rand_div((*dun).col_rooms);
    /* Align dungeon rooms */
    if dungeon_align != 0 {
        /* Slide some rooms right */
        if x % 3 as libc::c_int == 0 as libc::c_int { x += 1 }
        /* Slide some rooms left */
        if x % 3 as libc::c_int == 2 as libc::c_int { x -= 1 }
    }
    /* Ugly */
    process_hooks(7 as libc::c_int,
                  b"(d,d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, y, x);
    let mut current_block_64: u64;
    /* Build some rooms */
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        /* Pick a block for the room */
        y = Rand_div((*dun).row_rooms);
        x = Rand_div((*dun).col_rooms);
        /* Align dungeon rooms */
        if dungeon_align != 0 {
            /* Slide some rooms right */
            if x % 3 as libc::c_int == 0 as libc::c_int { x += 1 }
            /* Slide some rooms left */
            if x % 3 as libc::c_int == 2 as libc::c_int { x -= 1 }
        }
        /* Destroyed levels are boring */
        if destroyed != 0 {
            /* The deeper you are, the more cavelike the rooms are */
            /* no caves when cavern exists: they look bad */
            k = Rand_div(100 as libc::c_int) + 1 as libc::c_int;
            if cavern == 0 && k < dun_level as libc::c_int {
                /* Type 10 -- Fractal cave */
                (room_build(y, x, 10 as libc::c_int)) != 0;
            } else if !(dungeon_flags1 as libc::c_long &
                            0x80000 as libc::c_long != 0 &&
                            room_build(y, x, 9 as libc::c_int) as libc::c_int
                                != 0) {
                (room_build(y, x, 1 as libc::c_int)) != 0;
            }
        } else {
            /* Attempt a "trivial" room */
            /* Attempt an "unusual" room -- no vaults on town levels */
            if town_level == 0 &&
                   (ironman_rooms as libc::c_int != 0 ||
                        Rand_div(194 as libc::c_int) <
                            dun_level as libc::c_int) {
                /* Roll for room type */
                k =
                    if ironman_rooms as libc::c_int != 0 {
                        0 as libc::c_int
                    } else { Rand_div(100 as libc::c_int) };
                /* Attempt a very unusual room */
                /* test hack */
                if ironman_rooms as libc::c_int != 0 ||
                       Rand_div(194 as libc::c_int) < dun_level as libc::c_int
                   {
                    /* Type 8 -- Greater vault (10%) */
                    if k < 10 as libc::c_int {
                        if max_vault_ok > 1 as libc::c_int {
                            if room_build(y, x, 8 as libc::c_int) != 0 {
                                current_block_64 = 14220266465818359136;
                            } else {
                                current_block_64 = 17441561948628420366;
                            }
                        } else {
                            if cheat_room != 0 {
                                msg_print(b"Refusing a greater vault.\x00" as
                                              *const u8 as
                                              *const libc::c_char);
                            }
                            current_block_64 = 17441561948628420366;
                        }
                    } else { current_block_64 = 17441561948628420366; }
                    match current_block_64 {
                        14220266465818359136 => { }
                        _ =>
                        /* Type 7 -- Lesser vault (15%) */
                        {
                            if k < 25 as libc::c_int {
                                if max_vault_ok > 0 as libc::c_int {
                                    if room_build(y, x, 7 as libc::c_int) != 0
                                       {
                                        current_block_64 =
                                            14220266465818359136;
                                    } else {
                                        current_block_64 =
                                            18325745679564279244;
                                    }
                                } else {
                                    if cheat_room != 0 {
                                        msg_print(b"Refusing a lesser vault.\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                    }
                                    current_block_64 = 18325745679564279244;
                                }
                            } else {
                                current_block_64 = 18325745679564279244;
                            }
                            match current_block_64 {
                                14220266465818359136 => { }
                                _ =>
                                /* Type 5 -- Monster nest (15%) */
                                {
                                    if k < 40 as libc::c_int &&
                                           room_build(y, x, 5 as libc::c_int)
                                               as libc::c_int != 0 {
                                        current_block_64 =
                                            14220266465818359136;
                                    } else if k < 55 as libc::c_int &&
                                                  room_build(y, x,
                                                             6 as libc::c_int)
                                                      as libc::c_int != 0 {
                                        current_block_64 =
                                            14220266465818359136;
                                    } else if k < 60 as libc::c_int &&
                                                  room_build(y, x,
                                                             11 as
                                                                 libc::c_int)
                                                      as libc::c_int != 0 {
                                        current_block_64 =
                                            14220266465818359136;
                                    } else {
                                        current_block_64 =
                                            10859911333150192671;
                                    }
                                }
                            }
                        }
                    }
                } else { current_block_64 = 10859911333150192671; }
                match current_block_64 {
                    14220266465818359136 => { }
                    _ =>
                    /* Type 6 -- Monster pit (15%) */
                    /* Type 11 -- Random vault (5%) */
                    /* Type 4 -- Large room (25%) */
                    {
                        if k < 25 as libc::c_int &&
                               room_build(y, x, 4 as libc::c_int) as
                                   libc::c_int != 0 {
                            current_block_64 = 14220266465818359136;
                        } else if k < 45 as libc::c_int &&
                                      room_build(y, x, 3 as libc::c_int) as
                                          libc::c_int != 0 {
                            current_block_64 = 14220266465818359136;
                        } else if k < 65 as libc::c_int &&
                                      room_build(y, x, 2 as libc::c_int) as
                                          libc::c_int != 0 {
                            current_block_64 = 14220266465818359136;
                        } else if k < 80 as libc::c_int &&
                                      room_build(y, x, 10 as libc::c_int) as
                                          libc::c_int != 0 {
                            current_block_64 = 14220266465818359136;
                        } else {
                            /* Type 3 -- Cross room (20%) */
                            /* Type 2 -- Overlapping (20%) */
                            /* Type 10 -- Fractal cave (15%) */
                            /* Type 9 -- Circular (10%) */
			/* Hack - build standard rectangular rooms if needed */
                            if k < 90 as libc::c_int {
                                if dungeon_flags1 as libc::c_long &
                                       0x80000 as libc::c_long != 0 &&
                                       room_build(y, x, 1 as libc::c_int) as
                                           libc::c_int != 0 {
                                    current_block_64 = 14220266465818359136;
                                } else if room_build(y, x, 9 as libc::c_int)
                                              != 0 {
                                    current_block_64 = 14220266465818359136;
                                } else {
                                    current_block_64 = 16463303006880176998;
                                }
                            } else {
                                current_block_64 = 16463303006880176998;
                            }
                            match current_block_64 {
                                14220266465818359136 => { }
                                _ =>
                                /* Type 12 -- Crypt (10%) */
                                {
                                    if k < 100 as libc::c_int &&
                                           room_build(y, x, 12 as libc::c_int)
                                               as libc::c_int != 0 {
                                        current_block_64 =
                                            14220266465818359136;
                                    } else {
                                        current_block_64 =
                                            4488496028633655612;
                                    }
                                }
                            }
                        }
                    }
                }
            } else { current_block_64 = 4488496028633655612; }
            match current_block_64 {
                14220266465818359136 => { }
                _ =>
                /* Attempt a trivial room */
                {
                    if dungeon_flags1 as libc::c_long & 0x400 as libc::c_long
                           != 0 {
                        (room_build(y, x, 10 as libc::c_int)) != 0;
                    } else if !(dungeon_flags1 as libc::c_long &
                                    0x80000 as libc::c_long != 0 &&
                                    room_build(y, x, 9 as libc::c_int) as
                                        libc::c_int != 0) {
                        (room_build(y, x, 1 as libc::c_int)) != 0;
                    }
                }
            }
        }
        /* Never mind */
        i += 1
    }
    /* If no rooms are allocated... */
    while (*dun).cent_n == 0 as libc::c_int {
        /* ...force the creation of a small rectangular room */
        room_build(0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
    }
    /* Hack -- Scramble the room order */
    i = 0 as libc::c_int;
    while i < (*dun).cent_n {
        let mut pick1: libc::c_int = Rand_div((*dun).cent_n);
        let mut pick2: libc::c_int = Rand_div((*dun).cent_n);
        y1 = (*dun).cent[pick1 as usize].y as libc::c_int;
        x1 = (*dun).cent[pick1 as usize].x as libc::c_int;
        (*dun).cent[pick1 as usize].y = (*dun).cent[pick2 as usize].y;
        (*dun).cent[pick1 as usize].x = (*dun).cent[pick2 as usize].x;
        (*dun).cent[pick2 as usize].y = y1 as byte_hack;
        (*dun).cent[pick2 as usize].x = x1 as byte_hack;
        i += 1
    }
    /* Start with no tunnel doors */
    (*dun).door_n = 0 as libc::c_int;
    /* Hack -- connect the first room to the last room */
    y =
        (*dun).cent[((*dun).cent_n - 1 as libc::c_int) as usize].y as
            libc::c_int;
    x =
        (*dun).cent[((*dun).cent_n - 1 as libc::c_int) as usize].x as
            libc::c_int;
    /* Connect all the rooms together */
    i = 0 as libc::c_int;
    while i < (*dun).cent_n {
        /* Connect the room to the previous room */
        build_tunnel((*dun).cent[i as usize].y as libc::c_int,
                     (*dun).cent[i as usize].x as libc::c_int, y, x,
                     0 as libc::c_int as bool_);
        /* Remember the "previous" room */
        y = (*dun).cent[i as usize].y as libc::c_int;
        x = (*dun).cent[i as usize].x as libc::c_int;
        i += 1
    }
    /* Mega-Hack -- Convert FEAT_WALL_SOLID back into outer walls */
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   0x3b as libc::c_int {
                cave_set_feat(y, x, feat_wall_outer as libc::c_int);
            }
            x += 1
        }
        y += 1
    }
    /* Place intersection doors	 */
    i = 0 as libc::c_int;
    while i < (*dun).door_n {
        /* Extract junction location */
        y = (*dun).door[i as usize].y as libc::c_int;
        x = (*dun).door[i as usize].x as libc::c_int;
        /* Try placing doors */
        try_doors(y, x);
        i += 1
    }
    if strcmp(game_module, b"ToME\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        /* Hack -- Add some magma streamers */
        if dungeon_type as libc::c_int == 2 as libc::c_int ||
               dungeon_type as libc::c_int == 3 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                build_streamer(0x32 as libc::c_int, 90 as libc::c_int);
                i += 1
            }
        }
        /* Hack -- Add some quartz streamers */
        if dungeon_type as libc::c_int == 2 as libc::c_int ||
               dungeon_type as libc::c_int == 3 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 2 as libc::c_int {
                build_streamer(0x33 as libc::c_int, 40 as libc::c_int);
                i += 1
            }
        }
    }
    /* Add some sand streamers */
    if dungeon_flags1 as libc::c_long & 0x40000 as libc::c_long != 0 &&
           Rand_div(4 as libc::c_int) == 0 {
        if cheat_room as libc::c_int != 0 ||
               (*p_ptr).precognition as libc::c_int != 0 {
            msg_print(b"Sand vein.\x00" as *const u8 as *const libc::c_char);
        }
        build_streamer(0x62 as libc::c_int, 10 as libc::c_int);
    }
    /* Destroy the level if necessary */
    if destroyed != 0 { destroy_level(); }
    /* Create the town if needed */
    if town_level != 0 { town_gen(town_level as libc::c_int); }
    /* Hack -- Add some rivers if requested */
    if dungeon_flags1 as libc::c_long & 0x40 as libc::c_long != 0 &&
           Rand_div(4 as libc::c_int) == 0 {
        if cheat_room as libc::c_int != 0 ||
               (*p_ptr).precognition as libc::c_int != 0 {
            msg_print(b"River of water.\x00" as *const u8 as
                          *const libc::c_char);
        }
        add_river(0xbb as libc::c_int, 0x54 as libc::c_int);
    }
    if dungeon_flags1 as libc::c_long & 0x80 as libc::c_long != 0 &&
           Rand_div(4 as libc::c_int) == 0 {
        if cheat_room as libc::c_int != 0 ||
               (*p_ptr).precognition as libc::c_int != 0 {
            msg_print(b"River of lava.\x00" as *const u8 as
                          *const libc::c_char);
        }
        add_river(0x55 as libc::c_int, 0x56 as libc::c_int);
    }
    if dungeon_flags1 as libc::c_long & 0x100 as libc::c_long != 0 {
        let mut max: libc::c_int =
            3 as libc::c_int + Rand_div(2 as libc::c_int);
        let mut said: bool_ = 0 as libc::c_int as bool_;
        i = 0 as libc::c_int;
        while i < max {
            if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
                add_river(0xbb as libc::c_int, 0x54 as libc::c_int);
                if said == 0 &&
                       (cheat_room as libc::c_int != 0 ||
                            (*p_ptr).precognition as libc::c_int != 0) {
                    msg_print(b"Rivers of water.\x00" as *const u8 as
                                  *const libc::c_char);
                }
                said = 1 as libc::c_int as bool_
            }
            i += 1
        }
    }
    if dungeon_flags1 as libc::c_long & 0x200 as libc::c_long != 0 {
        let mut max_0: libc::c_int =
            2 as libc::c_int + Rand_div(2 as libc::c_int);
        let mut said_0: bool_ = 0 as libc::c_int as bool_;
        i = 0 as libc::c_int;
        while i < max_0 {
            if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
                add_river(0x55 as libc::c_int, 0x56 as libc::c_int);
                if said_0 == 0 &&
                       (cheat_room as libc::c_int != 0 ||
                            (*p_ptr).precognition as libc::c_int != 0) {
                    msg_print(b"Rivers of lava.\x00" as *const u8 as
                                  *const libc::c_char);
                }
                said_0 = 1 as libc::c_int as bool_
            }
            i += 1
        }
    }
    /* Add streamers of trees, water, or lava -KMW- */
    if dungeon_flags1 as libc::c_long & 0x80000000 as libc::c_long == 0 {
        let mut num: libc::c_int = 0;
        /*
		 * Flat levels (was: levels 1--2)
		 *
		 * Small trees (penetrate walls)
		 */
        if dungeon_flags1 as libc::c_long & 0x400000 as libc::c_long != 0 &&
               Rand_div(20 as libc::c_int) + 1 as libc::c_int >
                   15 as libc::c_int {
            num = Rand_div(2 as libc::c_int) + 1 as libc::c_int;
            i = 0 as libc::c_int;
            while i < num {
                build_streamer2(0xca as libc::c_int, 1 as libc::c_int);
                i += 1
            }
        }
        /*
		 * Levels 1 -- 33 (was: 1 -- 19)
		 *
		 * Shallow water (preserve walls)
		 * Deep water (penetrate walls)
		 */
        if !(dun_level as libc::c_int <= 33 as libc::c_int) &&
               Rand_div(20 as libc::c_int) + 1 as libc::c_int >
                   15 as libc::c_int {
            num =
                Rand_div(2 as libc::c_int - 1 as libc::c_int) +
                    1 as libc::c_int;
            i = 0 as libc::c_int;
            while i < num {
                build_streamer2(0x54 as libc::c_int, 0 as libc::c_int);
                i += 1
            }
            if Rand_div(20 as libc::c_int) + 1 as libc::c_int >
                   15 as libc::c_int {
                num = Rand_div(2 as libc::c_int) + 1 as libc::c_int;
                i = 0 as libc::c_int;
                while i < num {
                    build_streamer2(0xbb as libc::c_int, 1 as libc::c_int);
                    i += 1
                }
            }
        } else if dun_level as libc::c_int > 33 as libc::c_int {
            /*
		 * Levels 34 -- (was: 20 --)
		 */
            /*
			 * Shallow lava (preserve walls)
			 * Deep lava (penetrate walls)
			 */
            if Rand_div(20 as libc::c_int) + 1 as libc::c_int >
                   15 as libc::c_int {
                num = Rand_div(2 as libc::c_int) + 1 as libc::c_int;
                i = 0 as libc::c_int;
                while i < num {
                    build_streamer2(0x56 as libc::c_int, 0 as libc::c_int);
                    i += 1
                }
                if Rand_div(20 as libc::c_int) + 1 as libc::c_int >
                       15 as libc::c_int {
                    num =
                        Rand_div(2 as libc::c_int - 1 as libc::c_int) +
                            1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < num {
                        build_streamer2(0x55 as libc::c_int,
                                        1 as libc::c_int);
                        i += 1
                    }
                }
            } else if Rand_div(20 as libc::c_int) + 1 as libc::c_int >
                          15 as libc::c_int {
                num =
                    Rand_div(2 as libc::c_int - 1 as libc::c_int) +
                        1 as libc::c_int;
                i = 0 as libc::c_int;
                while i < num {
                    build_streamer2(0x54 as libc::c_int, 0 as libc::c_int);
                    i += 1
                }
                if Rand_div(20 as libc::c_int) + 1 as libc::c_int >
                       15 as libc::c_int {
                    num = Rand_div(2 as libc::c_int) + 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < num {
                        build_streamer2(0xbb as libc::c_int,
                                        1 as libc::c_int);
                        i += 1
                    }
                }
            }
        }
    }
    /*
			 * Shallow water (preserve walls)
			 * Deep water (penetrate walls)
			 */
    /* Hack, seems like once a room overrode the level boundaries, this is BAD */
    set_bounders(empty_level);
    /* Determine the character location */
    if new_player_spot(branch) == 0 { return 0 as libc::c_int as bool_ }
    return 1 as libc::c_int as bool_;
}
/*
 * Bring the imprinted pets from the old level
 */
#[no_mangle]
pub unsafe extern "C" fn replace_all_friends() {
    let mut i: libc::c_int = 0;
    if (*p_ptr).wild_mode != 0 { return }
    /* Scan every saved pet */
    i = 0 as libc::c_int;
    while i < max_m_idx as libc::c_int {
        if (*km_list.offset(i as isize)).r_idx as libc::c_int != 0 &&
               (*km_list.offset(i as isize)).status as libc::c_int ==
                   4 as libc::c_int {
            let mut y: libc::c_int = (*p_ptr).py as libc::c_int;
            let mut x: libc::c_int = (*p_ptr).px as libc::c_int;
            let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
            let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
            /* Find a suitable location */
            get_pos_player(5 as libc::c_int, &mut y, &mut x);
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            /* Get a m_idx to use */
            (*c_ptr).m_idx = m_pop();
            m_ptr =
                &mut *m_list.offset((*c_ptr).m_idx as isize) as
                    *mut monster_type;
            /* Actualy place the monster */
            *m_list.offset((*c_ptr).m_idx as isize) =
                *km_list.offset(i as isize);
            (*m_ptr).fy = y as byte_hack;
            (*m_ptr).fx = x as byte_hack;
            (*m_ptr).hold_o_idx = 0 as libc::c_int as s16b
        }
        i += 1
    };
}
/*
 * Save the imprinted pets from the old level
 */
#[no_mangle]
pub unsafe extern "C" fn save_all_friends() {
    if (*p_ptr).old_wild_mode != 0 { return }
    memcpy(km_list as *mut libc::c_char as *mut libc::c_void,
           m_list as *mut libc::c_char as *const libc::c_void,
           (max_m_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<monster_type>()
                                                as libc::c_ulong));
}
/*
 * Return the dungeon type of the current level(it can only return the
 * principal dungeons)
 */
#[no_mangle]
pub unsafe extern "C" fn calc_dungeon_type() -> byte_hack {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < max_d_idx as libc::c_int {
        if dun_level as libc::c_int >=
               (*d_info.offset(i as isize)).mindepth as libc::c_int &&
               dun_level as libc::c_int <=
                   (*d_info.offset(i as isize)).maxdepth as libc::c_int &&
               (*d_info.offset(i as isize)).flags1 as libc::c_long &
                   0x1 as libc::c_long != 0 {
            return i as byte_hack
        }
        i += 1
    }
    return 0 as libc::c_int as byte_hack;
}
/*
 * Build probability tables for walls and floors and set feat_wall_outer
 * and feat_wall_inner according to the current information in d_info.txt
 *
 * *hint* *hint* with this made extern, and we no longer have to
 * store fill_type and floor_type in the savefile...
 */
unsafe extern "C" fn init_feat_info() {
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    let mut i: libc::c_int = 0;
    let mut cur_depth: libc::c_int = 0;
    let mut max_depth: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut floor_lim1: libc::c_int = 0;
    let mut floor_lim2: libc::c_int = 0;
    let mut fill_lim1: libc::c_int = 0;
    let mut fill_lim2: libc::c_int = 0;
    /* Retrieve dungeon depth info (base 1, to avoid zero divide errors) */
    cur_depth =
        dun_level as libc::c_int - (*d_ptr).mindepth as libc::c_int +
            1 as libc::c_int;
    max_depth =
        (*d_ptr).maxdepth as libc::c_int - (*d_ptr).mindepth as libc::c_int +
            1 as libc::c_int;
    /* Set room wall types */
    feat_wall_outer = (*d_ptr).outer_wall as byte_hack;
    feat_wall_inner = (*d_ptr).inner_wall as byte_hack;
    /* Setup probability info -- Floors */
    p1 = (*d_ptr).floor_percent1[0 as libc::c_int as usize] as libc::c_int;
    p2 = (*d_ptr).floor_percent1[1 as libc::c_int as usize] as libc::c_int;
    floor_lim1 = p1 + (p2 - p1) * cur_depth / max_depth;
    p1 = (*d_ptr).floor_percent2[0 as libc::c_int as usize] as libc::c_int;
    p2 = (*d_ptr).floor_percent2[1 as libc::c_int as usize] as libc::c_int;
    floor_lim2 = floor_lim1 + p1 + (p2 - p1) * cur_depth / max_depth;
    /* Setup probability info -- Fillers */
    p1 = (*d_ptr).fill_percent1[0 as libc::c_int as usize] as libc::c_int;
    p2 = (*d_ptr).fill_percent1[1 as libc::c_int as usize] as libc::c_int;
    fill_lim1 = p1 + (p2 - p1) * cur_depth / max_depth;
    p1 = (*d_ptr).fill_percent2[0 as libc::c_int as usize] as libc::c_int;
    p2 = (*d_ptr).fill_percent2[1 as libc::c_int as usize] as libc::c_int;
    fill_lim2 = fill_lim1 + p1 + (p2 - p1) * cur_depth / max_depth;
    /* Fill the arrays of floors and walls in the good proportions */
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        if i < floor_lim1 {
            floor_type[i as usize] = (*d_ptr).floor1
        } else if i < floor_lim2 {
            floor_type[i as usize] = (*d_ptr).floor2
        } else { floor_type[i as usize] = (*d_ptr).floor3 }
        if i < fill_lim1 {
            fill_type[i as usize] = (*d_ptr).fill_type1
        } else if i < fill_lim2 {
            fill_type[i as usize] = (*d_ptr).fill_type2
        } else { fill_type[i as usize] = (*d_ptr).fill_type3 }
        i += 1
    };
}
/* Needed to refill empty levels */
unsafe extern "C" fn fill_level(mut use_floor: bool_, mut smooth: byte_hack) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    /* Convert smoothness to initial step */
    if smooth as libc::c_int == 0 as libc::c_int {
        step = 0 as libc::c_int
    } else if smooth as libc::c_int == 1 as libc::c_int {
        step = 1 as libc::c_int
    } else if smooth as libc::c_int == 2 as libc::c_int {
        step = 2 as libc::c_int
    } else if smooth as libc::c_int == 3 as libc::c_int {
        step = 4 as libc::c_int
    } else { step = 8 as libc::c_int }
    /*
	 * Paranoia -- step must be less than or equal to a half of
	 * width or height, whichever shorter
	 */
    if (cur_hgt as libc::c_int) < 16 as libc::c_int && step > 4 as libc::c_int
       {
        step = 4 as libc::c_int
    }
    if (cur_wid as libc::c_int) < 16 as libc::c_int && step > 4 as libc::c_int
       {
        step = 4 as libc::c_int
    }
    /* Special case -- simple fill */
    if step == 0 as libc::c_int {
        let mut filler: byte_hack = 0;
        /* Pick a filler XXX XXX XXX */
        if use_floor != 0 {
            filler =
                (*d_info.offset(dungeon_type as isize)).floor1 as byte_hack
        } else {
            filler =
                (*d_info.offset(dungeon_type as isize)).fill_type1 as
                    byte_hack
        }
        /* Fill the level with the filler without calling RNG */
        y = 0 as libc::c_int;
        while y < cur_hgt as libc::c_int {
            x = 0 as libc::c_int;
            while x < cur_wid as libc::c_int {
                cave_set_feat(y, x, filler as libc::c_int);
                x += 1
            }
            y += 1
        }
        /* Done */
        return
    }
    /*
	 * Fill starting positions -- every 'step' grids horizontally and
	 * vertically
	 */
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            /*
			 * Place randomly selected terrain feature using the prebuilt
			 * probability table
			 *
			 * By slightly modifying this, you can build streamers as
			 * well as normal fillers all at once, but this calls for
			 * modifications to the other part of the dungeon generator.
			 */
            if use_floor != 0 {
                place_floor(y, x);
            } else { place_filler(y, x); }
            x += step
        }
        y += step
    }
    /*
	 * Fill spaces between, randomly picking one of their neighbours
	 *
	 * This simple yet powerful algorithm was described by Mike Anderson:
	 *
	 * A   B      A | B      A a B
	 *        ->  --+--  ->  d e b
	 * D   C      D | C      D c C
	 *
	 * a can be either A or B, b B or C, c C or D and d D or A.
	 * e is chosen from A, B, C and D.
	 * Subdivide and repeat the process as many times as you like.
	 *
	 * All the nasty tricks that obscure this simplicity are mine (^ ^;)
	 */
    /* Initialise bit shift counter */
    shift = 14 as libc::c_int;
    loop 
         /* Repeat subdivision until all the grids are filled in */
         {
        step = step >> 1 as libc::c_int;
        if !(step > 0 as libc::c_int) { break ; }
        let mut y_even: bool_ = 0;
        let mut x_even: bool_ = 0;
        let mut y_wrap: s16b = 0;
        let mut x_wrap: s16b = 0;
        let mut y_sel: s16b = 0;
        let mut x_sel: s16b = 0;
        let mut selector: u32b = 0 as libc::c_int as u32b;
        /* Hacklette -- Calculate wrap-around locations */
        y_wrap =
            ((cur_hgt as libc::c_int - 1 as libc::c_int) /
                 (step * 2 as libc::c_int) * (step * 2 as libc::c_int)) as
                s16b;
        x_wrap =
            ((cur_wid as libc::c_int - 1 as libc::c_int) /
                 (step * 2 as libc::c_int) * (step * 2 as libc::c_int)) as
                s16b;
        /* Initialise vertical phase */
        y_even = 0 as libc::c_int as bool_;
        y = 0 as libc::c_int;
        while y < cur_hgt as libc::c_int {
            /* Flip vertical phase */
            y_even = (y_even == 0) as libc::c_int as bool_;
            /* Initialise horizontal phase */
            x_even = 0 as libc::c_int as bool_;
            x = 0 as libc::c_int;
            while x < cur_wid as libc::c_int {
                /* Flip horizontal phase */
                x_even = (x_even == 0) as libc::c_int as bool_;
                /* Already filled in by previous iterations */
                if !(y_even as libc::c_int != 0 && x_even as libc::c_int != 0)
                   {
                    /*
				 * Retrieve next two bits from pseudo-random bit sequence
				 *
				 * You can do well not caring so much about their randomness.
				 *
				 * This is not really necessary, but I don't like to invoke
				 * relatively expensive RNG when we can do with much smaller
				 * number of calls.
				 */
                    if shift >= 14 as libc::c_int {
                        selector =
                            Rand_div(0x10000000 as libc::c_long as s32b) as
                                u32b;
                        shift = 0 as libc::c_int
                    } else { selector >>= 2 as libc::c_int; shift += 1 }
                    /* Vertically in sync */
                    if y_even != 0 {
                        y_sel = y as s16b
                    } else {
                        /* Bit 1 selects neighbouring y */
                        y_sel =
                            if selector & 2 as libc::c_int as libc::c_uint !=
                                   0 {
                                (y) + step
                            } else { (y) - step } as s16b
                    }
                    /* Horizontally in sync */
                    if x_even != 0 {
                        x_sel = x as s16b
                    } else {
                        /* Bit 0 selects neighbouring x */
                        x_sel =
                            if selector & 1 as libc::c_int as libc::c_uint !=
                                   0 {
                                (x) + step
                            } else { (x) - step } as s16b
                    }
                    /* Hacklette -- Fix out of range indices by wrapping around */
                    if y_sel as libc::c_int >= cur_hgt as libc::c_int {
                        y_sel = 0 as libc::c_int as s16b
                    } else if (y_sel as libc::c_int) < 0 as libc::c_int {
                        y_sel = y_wrap
                    }
                    if x_sel as libc::c_int >= cur_wid as libc::c_int {
                        x_sel = 0 as libc::c_int as s16b
                    } else if (x_sel as libc::c_int) < 0 as libc::c_int {
                        x_sel = x_wrap
                    }
                    /*
				 * Fill the grid with terrain feature of the randomly
				 * picked up neighbour
				 */
                    cave_set_feat(y, x,
                                  (*cave[y_sel as
                                             usize].offset(x_sel as
                                                               isize)).feat as
                                      libc::c_int);
                }
                x += step
            }
            y += step
        }
    };
}
/*
 * Generate a new dungeon level
 *
 * Note that "dun_body" adds about 4000 bytes of memory to the stack.
 */
unsafe extern "C" fn cave_gen() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut branch: libc::c_int = 0;
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    let mut max_vault_ok: libc::c_int = 2 as libc::c_int;
    let mut empty_level: bool_ = 0 as libc::c_int as bool_;
    let mut generator: *mut level_generator_type =
        0 as *mut level_generator_type;
    let mut dun_body: dun_data =
        dun_data{cent_n: 0,
                 cent: [coord{y: 0, x: 0,}; 100],
                 door_n: 0,
                 door: [coord{y: 0, x: 0,}; 200],
                 wall_n: 0,
                 wall: [coord{y: 0, x: 0,}; 500],
                 tunn_n: 0,
                 tunn: [coord{y: 0, x: 0,}; 900],
                 row_rooms: 0,
                 col_rooms: 0,
                 room_map: [[0; 18]; 6],
                 crowded: 0,};
    let mut generator_name: [libc::c_char; 100] = [0; 100];
    if get_dungeon_generator(generator_name.as_mut_ptr()) == 0 {
        strnfmt(generator_name.as_mut_ptr(), 99 as libc::c_int as uint_hack,
                b"%s\x00" as *const u8 as *const libc::c_char,
                (*d_ptr).generator.as_mut_ptr());
    }
    /*
	 * We generate a double dungeon. First we should halve the desired
	 * width/height, generate the dungeon normally, then double it
	 * in both directions
	 */
    if dungeon_flags1 as libc::c_long & 0x2000000 as libc::c_long != 0 {
        cur_wid = (cur_wid as libc::c_int / 2 as libc::c_int) as s16b;
        cur_hgt = (cur_hgt as libc::c_int / 2 as libc::c_int) as s16b
    }
    /* Fill the arrays of floors and walls in the good proportions */
    init_feat_info();
    /* Set the correct monster hook */
    set_mon_num_hook();
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Global data */
    dun = &mut dun_body;
    if max_panel_rows == 0 { max_vault_ok -= 1 }
    if max_panel_cols == 0 { max_vault_ok -= 1 }
    /*
	 * Hack -- Start with fill_type's
	 *
	 * Need a way to know appropriate smoothing factor for the current
	 * dungeon. Maybe we need another d_info flag/value.
	 */
    fill_level(empty_level, (*d_ptr).fill_method);
    set_bounders(empty_level);
    /*
	 * Call the good level generator
	 */
    generator = level_generators;
    while !generator.is_null() {
        if strcmp((*generator).name, generator_name.as_mut_ptr()) == 0 {
            if (*generator).generator.expect("non-null function pointer")((*generator).name)
                   == 0 {
                return 0 as libc::c_int as bool_
            }
            break ;
        } else { generator = (*generator).next }
    }
    /* Only if requested */
    if (*generator).default_stairs != 0 {
        /* Is there a dungeon branch ? */
        branch = get_branch();
        if branch != 0 {
            /* Place 5 down stair some walls */
            alloc_stairs(0x7 as libc::c_int, 5 as libc::c_int,
                         3 as libc::c_int, branch);
        }
        /* Is there a father dungeon branch ? */
        branch = get_fbranch();
        if branch != 0 {
            /* Place 1 down stair some walls */
            alloc_stairs(0x6 as libc::c_int, 5 as libc::c_int,
                         3 as libc::c_int, branch);
        }
        if (dun_level as libc::c_int) < (*d_ptr).maxdepth as libc::c_int ||
               dun_level as libc::c_int == (*d_ptr).maxdepth as libc::c_int &&
                   dungeon_flags1 as libc::c_long & 0x8000 as libc::c_long !=
                       0 {
            /* Place 3 or 4 down stairs near some walls */
            alloc_stairs(if dungeon_flags1 as libc::c_long &
                                0x400000 as libc::c_long != 0 {
                             0xb3 as libc::c_int
                         } else { 0x7 as libc::c_int },
                         3 as libc::c_int +
                             Rand_div(1 as libc::c_int + 4 as libc::c_int -
                                          3 as libc::c_int), 3 as libc::c_int,
                         0 as libc::c_int);
            /* Place 0 or 1 down shafts near some walls */
            if dungeon_flags2 as libc::c_long & 0x2 as libc::c_long == 0 {
                alloc_stairs(if dungeon_flags1 as libc::c_long &
                                    0x400000 as libc::c_long != 0 {
                                 0xb3 as libc::c_int
                             } else { 0xd as libc::c_int },
                             0 as libc::c_int +
                                 Rand_div(1 as libc::c_int + 1 as libc::c_int
                                              - 0 as libc::c_int),
                             3 as libc::c_int, 0 as libc::c_int);
            }
        }
        if dun_level as libc::c_int > (*d_ptr).mindepth as libc::c_int ||
               dun_level as libc::c_int == (*d_ptr).mindepth as libc::c_int &&
                   dungeon_flags1 as libc::c_long & 0x1000 as libc::c_long ==
                       0 {
            /* Place 1 or 2 up stairs near some walls */
            alloc_stairs(if dungeon_flags1 as libc::c_long &
                                0x400000 as libc::c_long != 0 {
                             0xb4 as libc::c_int
                         } else { 0x6 as libc::c_int },
                         1 as libc::c_int +
                             Rand_div(1 as libc::c_int + 2 as libc::c_int -
                                          1 as libc::c_int), 3 as libc::c_int,
                         0 as libc::c_int);
            /* Place 0 or 1 up shafts near some walls */
            if dungeon_flags2 as libc::c_long & 0x2 as libc::c_long == 0 {
                alloc_stairs(if dungeon_flags1 as libc::c_long &
                                    0x400000 as libc::c_long != 0 {
                                 0xb4 as libc::c_int
                             } else { 0xe as libc::c_int },
                             0 as libc::c_int +
                                 Rand_div(1 as libc::c_int + 1 as libc::c_int
                                              - 0 as libc::c_int),
                             3 as libc::c_int, 0 as libc::c_int);
            }
        }
    }
    process_hooks(6 as libc::c_int,
                  b"(d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, is_quest(dun_level as libc::c_int));
    /* Basic "amount" */
    k = dun_level as libc::c_int / 3 as libc::c_int;
    if k > 10 as libc::c_int { k = 10 as libc::c_int }
    if k < 2 as libc::c_int { k = 2 as libc::c_int }
    /* Only if requested */
    if (*generator).default_monsters != 0 {
        /*
		 * Pick a base number of monsters
		 */
        i = (*d_ptr).min_m_alloc_level;
        /* To make small levels a bit more playable */
        if (cur_hgt as libc::c_int) < 66 as libc::c_int ||
               (cur_wid as libc::c_int) < 198 as libc::c_int {
            let mut small_tester: libc::c_int = i;
            i = i * cur_hgt as libc::c_int / 66 as libc::c_int;
            i = i * cur_wid as libc::c_int / 198 as libc::c_int;
            i += 1 as libc::c_int;
            if i > small_tester {
                i = small_tester
            } else if cheat_hear != 0 {
                msg_format(b"Reduced monsters base from %d to %d\x00" as
                               *const u8 as *const libc::c_char, small_tester,
                           i);
            }
        }
        i += Rand_div(8 as libc::c_int) + 1 as libc::c_int;
        /* Put some monsters in the dungeon */
        i = i + k;
        while i > 0 as libc::c_int {
            alloc_monster(0 as libc::c_int, 1 as libc::c_int as bool_);
            i -= 1
        }
    }
    /* Check fates */
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        /* Ignore empty slots */
        if !(fates[i as usize].fate as libc::c_int == 0 as libc::c_int) {
            /* Check dungeon depth */
            if !(fates[i as usize].level as libc::c_int !=
                     dun_level as libc::c_int) {
                /* Non-serious fates don't always fire */
                if !(fates[i as usize].serious == 0 &&
                         Rand_div(2 as libc::c_int) + 1 as libc::c_int !=
                             1 as libc::c_int) {
                    /* Player meets his/her fate now... */
                    fate_flag = 1 as libc::c_int as bool_;
                    match fates[i as usize].fate as libc::c_int {
                        1 => {
                            let mut oy: libc::c_int =
                                (*p_ptr).py as libc::c_int + 1 as libc::c_int;
                            let mut ox: libc::c_int =
                                (*p_ptr).px as libc::c_int;
                            let mut q_ptr: *mut object_type =
                                0 as *mut object_type;
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
                            /* Mega-Hack */
                            object_prep(q_ptr,
                                        fates[i as usize].o_idx as
                                            libc::c_int);
                            /* Mega-Hack */
                            apply_magic(q_ptr, dun_level as libc::c_int,
                                        1 as libc::c_int as bool_,
                                        1 as libc::c_int as bool_,
                                        fates[i as usize].serious as bool_);
                            get_pos_player(10 as libc::c_int, &mut oy,
                                           &mut ox);
                            /* Drop it from the heaven */
                            drop_near(q_ptr, -(1 as libc::c_int), oy, ox);
                            /* Make it icky */
                            fates[i as usize].icky = 1 as libc::c_int as bool_
                        }
                        4 => {
                            let mut oy_0: libc::c_int =
                                (*p_ptr).py as libc::c_int + 1 as libc::c_int;
                            let mut ox_0: libc::c_int =
                                (*p_ptr).px as libc::c_int;
                            get_pos_player(10 as libc::c_int, &mut oy_0,
                                           &mut ox_0);
                            place_monster_one(oy_0, ox_0,
                                              fates[i as usize].r_idx as
                                                  libc::c_int,
                                              0 as libc::c_int,
                                              fates[i as usize].serious as
                                                  bool_, -(2 as libc::c_int));
                            fates[i as usize].icky = 1 as libc::c_int as bool_
                        }
                        3 => {
                            let mut oy_1: libc::c_int =
                                (*p_ptr).py as libc::c_int + 1 as libc::c_int;
                            let mut ox_1: libc::c_int =
                                (*p_ptr).px as libc::c_int;
                            let mut q_ptr_0: *mut object_type =
                                0 as *mut object_type;
                            let mut forge_0: object_type =
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
                            get_pos_player(10 as libc::c_int, &mut oy_1,
                                           &mut ox_1);
                            /* XXX XXX XXX Grant a randart */
                            if fates[i as usize].a_idx as libc::c_int ==
                                   0 as libc::c_int {
                                let mut obj_lev: libc::c_int = 0;
                                let mut k_idx: s16b = 0;
                                /* Apply restriction */
                                get_obj_num_hook =
                                    Some(kind_is_artifactable as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> bool_);
                                /* Object level a la find object fates */
                                obj_lev =
                                    *max_dlv.offset(dungeon_type as isize) as
                                        libc::c_int +
                                        (Rand_div(10 as libc::c_int) +
                                             1 as libc::c_int);
                                /* Rebuild allocation table */
                                get_obj_num_prep();
                                /* Roll for an object */
                                k_idx = get_obj_num(obj_lev);
                                /* Reset restriction */
                                get_obj_num_hook =
                                    Some(kind_is_legal as
                                             unsafe extern "C" fn(_:
                                                                      libc::c_int)
                                                 -> bool_);
                                /* Invalidate the allocation table */
                                alloc_kind_table_valid =
                                    0 as libc::c_int as bool_;
                                /* Get a local object */
                                q_ptr_0 = &mut forge_0;
                                /* Wipe it */
                                object_wipe(q_ptr_0);
                                /* Create the object */
                                object_prep(q_ptr_0, k_idx as libc::c_int);
                                /* SoAC it */
                                create_artifact(q_ptr_0,
                                                0 as libc::c_int as bool_,
                                                1 as libc::c_int as bool_);
                                /* Drop the artifact from heaven */
                                drop_near(q_ptr_0, -(1 as libc::c_int), oy_1,
                                          ox_1);
                            } else if (*a_info.offset(fates[i as usize].a_idx
                                                          as isize)).cur_num
                                          as libc::c_int == 0 as libc::c_int {
                                let mut a_ptr: *mut artifact_type =
                                    &mut *a_info.offset((*fates.as_mut_ptr().offset(i
                                                                                        as
                                                                                        isize)).a_idx
                                                            as isize) as
                                        *mut artifact_type;
                                let mut I_kind: s16b = 0;
                                /* Grant a normal artefact */
                                /* Get local object */
                                q_ptr_0 = &mut forge_0;
                                /* Wipe the object */
                                object_wipe(q_ptr_0);
                                /* Acquire the "kind" index */
                                I_kind =
                                    lookup_kind((*a_ptr).tval as libc::c_int,
                                                (*a_ptr).sval as libc::c_int);
                                /* Create the artifact */
                                object_prep(q_ptr_0, I_kind as libc::c_int);
                                /* Save the name */
                                (*q_ptr_0).name1 =
                                    fates[i as usize].a_idx as byte_hack;
                                apply_magic(q_ptr_0, -(1 as libc::c_int),
                                            1 as libc::c_int as bool_,
                                            1 as libc::c_int as bool_,
                                            1 as libc::c_int as bool_);
                                /* Drop the artifact from heaven */
                                drop_near(q_ptr_0, -(1 as libc::c_int), oy_1,
                                          ox_1);
                            }
                            fates[i as usize].icky = 1 as libc::c_int as bool_
                        }
                        _ => { }
                    }
                }
            }
        }
        i += 1
    }
    /* Re scan the list to eliminate the inutile fate */
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        match fates[i as usize].fate as libc::c_int {
            3 => {
                if (*a_info.offset(fates[i as usize].a_idx as isize)).cur_num
                       as libc::c_int == 1 as libc::c_int {
                    fates[i as usize].icky = 1 as libc::c_int as bool_
                }
            }
            4 => {
                if (*r_info.offset(fates[i as usize].r_idx as isize)).cur_num
                       as libc::c_int == 1 as libc::c_int &&
                       (*r_info.offset(fates[i as usize].r_idx as
                                           isize)).flags1 &
                           0x1 as libc::c_int as libc::c_uint != 0 {
                    fates[i as usize].icky = 1 as libc::c_int as bool_
                }
            }
            _ => { }
        }
        i += 1
    }
    /* Only if requested */
    if (*generator).default_miscs != 0 {
        /* Place some traps in the dungeon */
        alloc_object(3 as libc::c_int, 3 as libc::c_int,
                     Rand_div(k * 2 as libc::c_int) + 1 as libc::c_int);
        /* Put some rubble in corridors */
        alloc_object(1 as libc::c_int, 1 as libc::c_int,
                     Rand_div(k) + 1 as libc::c_int);
    }
    /* Only if requested */
    if (*generator).default_objects != 0 {
        /* Put some objects in rooms */
        if dungeon_type as libc::c_int != DUNGEON_DEATH {
            alloc_object(2 as libc::c_int, 5 as libc::c_int,
                         randnor(9 as libc::c_int, 3 as libc::c_int) as
                             libc::c_int);
        }
        /* Put some objects/gold in the dungeon */
        if dungeon_type as libc::c_int != DUNGEON_DEATH {
            alloc_object(3 as libc::c_int, 5 as libc::c_int,
                         randnor(3 as libc::c_int, 3 as libc::c_int) as
                             libc::c_int);
        }
        if dungeon_type as libc::c_int != DUNGEON_DEATH {
            alloc_object(3 as libc::c_int, 4 as libc::c_int,
                         randnor(3 as libc::c_int, 3 as libc::c_int) as
                             libc::c_int);
        }
    }
    /* Only if requested */
    if (*generator).default_miscs != 0 {
        /* Put some altars */
        alloc_object(2 as libc::c_int, 6 as libc::c_int,
                     randnor(1 as libc::c_int, 3 as libc::c_int) as
                         libc::c_int);
        /* Put some between gates */
        alloc_object(2 as libc::c_int, 7 as libc::c_int,
                     randnor(2 as libc::c_int, 3 as libc::c_int) as
                         libc::c_int);
        /* Put some fountains */
        alloc_object(2 as libc::c_int, 8 as libc::c_int,
                     randnor(1 as libc::c_int, 3 as libc::c_int) as
                         libc::c_int);
    }
    /* Put an Artifact and Artifact Guardian is requested */
    if (*d_ptr).final_guardian != 0 &&
           (*d_ptr).maxdepth as libc::c_int == dun_level as libc::c_int {
        let mut oy_2: libc::c_int = 0;
        let mut ox_2: libc::c_int = 0;
        let mut m_idx: libc::c_int = 0;
        let mut tries: libc::c_int = 10000 as libc::c_int;
        /* Find a good position */
        while tries != 0 {
            /* Get a random spot */
            oy_2 =
                Rand_div(cur_hgt as libc::c_int - 4 as libc::c_int) +
                    1 as libc::c_int + 2 as libc::c_int;
            ox_2 =
                Rand_div(cur_wid as libc::c_int - 4 as libc::c_int) +
                    1 as libc::c_int + 2 as libc::c_int;
            /* Is it a good spot ? */
            if (*f_info.offset((*cave[oy_2 as
                                          usize].offset(ox_2 as isize)).feat
                                   as isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[oy_2 as usize].offset(ox_2 as isize)).feat as
                       libc::c_int != 0xaf as libc::c_int &&
                   (*cave[oy_2 as usize].offset(ox_2 as isize)).m_idx == 0 &&
                   !(oy_2 == (*p_ptr).py as libc::c_int &&
                         ox_2 == (*p_ptr).px as libc::c_int) {
                break ;
            }
            /* One less try */
            tries -= 1
        }
        /* Place the guardian */
        *m_allow_special.offset((*d_ptr).final_guardian as isize) =
            1 as libc::c_int as bool_;
        place_monster_one(oy_2, ox_2, (*d_ptr).final_guardian,
                          0 as libc::c_int, 0 as libc::c_int as bool_,
                          -(2 as libc::c_int));
        *m_allow_special.offset((*d_ptr).final_guardian as isize) =
            0 as libc::c_int as bool_;
        m_idx =
            (*cave[oy_2 as usize].offset(ox_2 as isize)).m_idx as libc::c_int;
        if m_idx == 0 && wizard as libc::c_int != 0 {
            cmsg_print(12 as libc::c_int as byte_hack,
                       b"WARNING: Could not place guardian.\x00" as *const u8
                           as *const libc::c_char);
        }
        /*
		 * If guardian is successfully created and his/her/its
		 * treasure hasn't been found, let him/her/it own that
		 */
        if m_idx != 0 && (*d_ptr).final_artifact != 0 &&
               (*a_info.offset((*d_ptr).final_artifact as isize)).cur_num as
                   libc::c_int == 0 as libc::c_int {
            let mut a_ptr_0: *mut artifact_type =
                &mut *a_info.offset((*d_ptr).final_artifact as isize) as
                    *mut artifact_type;
            let mut q_ptr_1: *mut object_type = 0 as *mut object_type;
            let mut forge_1: object_type =
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
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
            let mut I_kind_0: libc::c_int = 0;
            let mut o_idx: libc::c_int = 0;
            /* Get new object */
            o_idx = o_pop() as libc::c_int;
            /* Proceed only if there's an object slot available */
            if o_idx != 0 {
                *a_allow_special.offset((*d_ptr).final_artifact as isize) =
                    1 as libc::c_int as bool_;
                /* Get local object */
                q_ptr_1 = &mut forge_1;
                /* Wipe the object */
                object_wipe(q_ptr_1);
                /* Acquire the "kind" index */
                I_kind_0 =
                    lookup_kind((*a_ptr_0).tval as libc::c_int,
                                (*a_ptr_0).sval as libc::c_int) as
                        libc::c_int;
                /* Create the artifact */
                object_prep(q_ptr_1, I_kind_0);
                /* Save the name */
                (*q_ptr_1).name1 = (*d_ptr).final_artifact as byte_hack;
                /* Actually create it */
                apply_magic(q_ptr_1, -(1 as libc::c_int),
                            1 as libc::c_int as bool_,
                            1 as libc::c_int as bool_,
                            1 as libc::c_int as bool_);
                /* Where it is found ? */
                (*q_ptr_1).found = 1 as libc::c_int as byte_hack;
                (*q_ptr_1).found_aux1 = (*d_ptr).final_guardian as s16b;
                (*q_ptr_1).found_aux2 = 0 as libc::c_int as s16b;
                (*q_ptr_1).found_aux3 = dungeon_type as s16b;
                (*q_ptr_1).found_aux4 =
                    if dungeon_type as libc::c_int == 0 as libc::c_int {
                        (*(*wild_map.offset((*p_ptr).wilderness_y as
                                                isize)).offset((*p_ptr).wilderness_x
                                                                   as
                                                                   isize)).feat
                    } else { dun_level as libc::c_int } as s16b;
                *a_allow_special.offset((*d_ptr).final_artifact as isize) =
                    0 as libc::c_int as bool_;
                /* Get the item */
                o_ptr =
                    &mut *o_list.offset(o_idx as isize) as *mut object_type;
                /* Structure copy */
                object_copy(o_ptr, q_ptr_1);
                /* Build a stack */
                (*o_ptr).next_o_idx =
                    (*m_list.offset(m_idx as isize)).hold_o_idx;
                (*o_ptr).held_m_idx = m_idx as s16b;
                (*o_ptr).ix = 0 as libc::c_int as byte_hack;
                (*o_ptr).iy = 0 as libc::c_int as byte_hack;
                (*m_list.offset(m_idx as isize)).hold_o_idx = o_idx as s16b
            }
        }
        if m_idx != 0 && (*d_ptr).final_object != 0 &&
               (*k_info.offset((*d_ptr).final_object as isize)).artifact as
                   libc::c_int == 0 as libc::c_int {
            let mut q_ptr_2: *mut object_type = 0 as *mut object_type;
            let mut forge_2: object_type =
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
            let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
            let mut o_idx_0: libc::c_int = 0;
            /* Get new object */
            o_idx_0 = o_pop() as libc::c_int;
            /* Proceed only if there's an object slot available */
            if o_idx_0 != 0 {
                /* Get local object */
                q_ptr_2 = &mut forge_2;
                *k_allow_special.offset((*d_ptr).final_object as isize) =
                    1 as libc::c_int as bool_;
                /* Wipe the object */
                object_wipe(q_ptr_2);
                /* Create the final object */
                object_prep(q_ptr_2, (*d_ptr).final_object);
                apply_magic(q_ptr_2, 1 as libc::c_int,
                            0 as libc::c_int as bool_,
                            0 as libc::c_int as bool_,
                            0 as libc::c_int as bool_);
                /* Where it is found ? */
                (*q_ptr_2).found = 1 as libc::c_int as byte_hack;
                (*q_ptr_2).found_aux1 = (*d_ptr).final_guardian as s16b;
                (*q_ptr_2).found_aux2 = 0 as libc::c_int as s16b;
                (*q_ptr_2).found_aux3 = dungeon_type as s16b;
                (*q_ptr_2).found_aux4 =
                    if dungeon_type as libc::c_int == 0 as libc::c_int {
                        (*(*wild_map.offset((*p_ptr).wilderness_y as
                                                isize)).offset((*p_ptr).wilderness_x
                                                                   as
                                                                   isize)).feat
                    } else { dun_level as libc::c_int } as s16b;
                *k_allow_special.offset((*d_ptr).final_object as isize) =
                    0 as libc::c_int as bool_;
                (*k_info.offset((*d_ptr).final_object as isize)).artifact =
                    1 as libc::c_int as bool_;
                /* Get the item */
                o_ptr_0 =
                    &mut *o_list.offset(o_idx_0 as isize) as *mut object_type;
                /* Structure copy */
                object_copy(o_ptr_0, q_ptr_2);
                /* Build a stack */
                (*o_ptr_0).next_o_idx =
                    (*m_list.offset(m_idx as isize)).hold_o_idx;
                (*o_ptr_0).held_m_idx = m_idx as s16b;
                (*o_ptr_0).ix = 0 as libc::c_int as byte_hack;
                (*o_ptr_0).iy = 0 as libc::c_int as byte_hack;
                (*m_list.offset(m_idx as isize)).hold_o_idx = o_idx_0 as s16b
            }
        }
    }
    if empty_level as libc::c_int != 0 &&
           (Rand_div(5 as libc::c_int) + 1 as libc::c_int != 1 as libc::c_int
                ||
                Rand_div(100 as libc::c_int) + 1 as libc::c_int >
                    dun_level as libc::c_int) {
        wiz_lite();
    }
    /* Now double the generated dungeon */
    if dungeon_flags1 as libc::c_long & 0x2000000 as libc::c_long != 0 {
        /* We begin at the bottom-right corner and from there move
		 * up/left (this way we don't need another array for the
		 * dungeon data) */
		/* Note: we double the border permanent walls, too. It is
		 * easier this way and I think it isn't too ugly */
        y = cur_hgt as libc::c_int - 1 as libc::c_int;
        y1 = y * 2 as libc::c_int;
        while y >= 0 as libc::c_int {
            x = cur_wid as libc::c_int - 1 as libc::c_int;
            x1 = x * 2 as libc::c_int;
            while x >= 0 as libc::c_int {
                let mut disp: [[libc::c_int; 2]; 4] =
                    [[0 as libc::c_int, 0 as libc::c_int],
                     [0 as libc::c_int, 1 as libc::c_int],
                     [1 as libc::c_int, 0 as libc::c_int],
                     [1 as libc::c_int, 1 as libc::c_int]];
                let mut c_ptr: [*mut cave_type; 4] = [0 as *mut cave_type; 4];
                let mut cc_ptr: *mut cave_type =
                    &mut *(*cave.as_mut_ptr().offset(y as
                                                         isize)).offset(x as
                                                                            isize)
                        as *mut cave_type;
                let mut o_ptr_1: *mut object_type =
                    &mut *o_list.offset((*cc_ptr).o_idx as isize) as
                        *mut object_type;
                let mut m_ptr: *mut monster_type =
                    &mut *m_list.offset((*cc_ptr).m_idx as isize) as
                        *mut monster_type;
                /*
				 * Now we copy the generated data to the
				 * appropriate grids
				 */
                i = 0 as libc::c_int;
                while i < 4 as libc::c_int {
                    c_ptr[i as usize] =
                        &mut *(*cave.as_mut_ptr().offset((y1 +
                                                              *(*disp.as_mut_ptr().offset(i
                                                                                              as
                                                                                              isize)).as_mut_ptr().offset(0
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize))
                                                             as
                                                             isize)).offset((x1
                                                                                 +
                                                                                 *(*disp.as_mut_ptr().offset(i
                                                                                                                 as
                                                                                                                 isize)).as_mut_ptr().offset(1
                                                                                                                                                 as
                                                                                                                                                 libc::c_int
                                                                                                                                                 as
                                                                                                                                                 isize))
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    *c_ptr[i as usize] = *cc_ptr;
                    (*c_ptr[i as usize]).o_idx = 0 as libc::c_int as s16b;
                    (*c_ptr[i as usize]).m_idx = 0 as libc::c_int as s16b;
                    if (*cc_ptr).feat as libc::c_int == 0xa0 as libc::c_int {
                        let mut xxx: libc::c_int =
                            (*cc_ptr).special as libc::c_int &
                                0xff as libc::c_int;
                        let mut yyy: libc::c_int =
                            (*cc_ptr).special as libc::c_int >>
                                8 as libc::c_int;
                        xxx *= 2 as libc::c_int;
                        yyy *= 2 as libc::c_int;
                        xxx += disp[i as usize][1 as libc::c_int as usize];
                        yyy += disp[i as usize][0 as libc::c_int as usize];
                        (*c_ptr[i as usize]).special =
                            (xxx + (yyy << 8 as libc::c_int)) as s16b
                    }
                    i += 1
                }
                /* Objects should be put only in 1 of the
				 * new grids (otherwise we would segfault
				 * a lot) ... */
                if (*cc_ptr).o_idx as libc::c_int != 0 as libc::c_int {
                    i = Rand_div(4 as libc::c_int);
                    (*c_ptr[i as usize]).o_idx = (*cc_ptr).o_idx;
                    (*o_ptr_1).iy =
                        (y1 + disp[i as usize][0 as libc::c_int as usize]) as
                            byte_hack;
                    (*o_ptr_1).ix =
                        (x1 + disp[i as usize][1 as libc::c_int as usize]) as
                            byte_hack
                }
                /* ..just like monsters */
                if (*cc_ptr).m_idx as libc::c_int != 0 as libc::c_int {
                    i = Rand_div(4 as libc::c_int);
                    (*c_ptr[i as usize]).m_idx = (*cc_ptr).m_idx;
                    (*m_ptr).fy =
                        (y1 + disp[i as usize][0 as libc::c_int as usize]) as
                            byte_hack;
                    (*m_ptr).fx =
                        (x1 + disp[i as usize][1 as libc::c_int as usize]) as
                            byte_hack
                }
                x -= 1;
                x1 -= 2 as libc::c_int
            }
            y -= 1;
            y1 -= 2 as libc::c_int
        }
        /* Set the width/height ... */
        cur_wid = (cur_wid as libc::c_int * 2 as libc::c_int) as s16b;
        cur_hgt = (cur_hgt as libc::c_int * 2 as libc::c_int) as s16b;
        /* ... and player position to the right place */
        (*p_ptr).py = ((*p_ptr).py as libc::c_int * 2 as libc::c_int) as s16b;
        (*p_ptr).px = ((*p_ptr).px as libc::c_int * 2 as libc::c_int) as s16b
    }
    return 1 as libc::c_int as bool_;
}
/*
 * Builds the arena after it is entered -KMW-
 */
unsafe extern "C" fn build_arena() {
    let mut yval: libc::c_int = 0;
    let mut y_height: libc::c_int = 0;
    let mut y_depth: libc::c_int = 0;
    let mut xval: libc::c_int = 0;
    let mut x_left: libc::c_int = 0;
    let mut x_right: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    yval = 22 as libc::c_int / 2 as libc::c_int;
    xval = 66 as libc::c_int / 2 as libc::c_int;
    y_height = yval - 10 as libc::c_int + 22 as libc::c_int;
    y_depth = yval + 10 as libc::c_int + 22 as libc::c_int;
    x_left = xval - 32 as libc::c_int + 66 as libc::c_int;
    x_right = xval + 32 as libc::c_int + 66 as libc::c_int;
    i = y_height;
    while i <= y_height + 5 as libc::c_int {
        j = x_left;
        while j <= x_right {
            cave_set_feat(i, j, 0x3c as libc::c_int);
            let ref mut fresh33 = (*cave[i as usize].offset(j as isize)).info;
            *fresh33 =
                (*fresh33 as libc::c_int |
                     (0x2 as libc::c_int | 0x1 as libc::c_int)) as u16b;
            j += 1
        }
        i += 1
    }
    i = y_depth;
    while i >= y_depth - 5 as libc::c_int {
        j = x_left;
        while j <= x_right {
            cave_set_feat(i, j, 0x3c as libc::c_int);
            let ref mut fresh34 = (*cave[i as usize].offset(j as isize)).info;
            *fresh34 =
                (*fresh34 as libc::c_int |
                     (0x2 as libc::c_int | 0x1 as libc::c_int)) as u16b;
            j += 1
        }
        i -= 1
    }
    j = x_left;
    while j <= x_left + 17 as libc::c_int {
        i = y_height;
        while i <= y_depth {
            cave_set_feat(i, j, 0x3c as libc::c_int);
            let ref mut fresh35 = (*cave[i as usize].offset(j as isize)).info;
            *fresh35 =
                (*fresh35 as libc::c_int |
                     (0x2 as libc::c_int | 0x1 as libc::c_int)) as u16b;
            i += 1
        }
        j += 1
    }
    j = x_right;
    while j >= x_right - 17 as libc::c_int {
        i = y_height;
        while i <= y_depth {
            cave_set_feat(i, j, 0x3c as libc::c_int);
            let ref mut fresh36 = (*cave[i as usize].offset(j as isize)).info;
            *fresh36 =
                (*fresh36 as libc::c_int |
                     (0x2 as libc::c_int | 0x1 as libc::c_int)) as u16b;
            i += 1
        }
        j -= 1
    }
    cave_set_feat(y_height + 6 as libc::c_int, x_left + 18 as libc::c_int,
                  0x3c as libc::c_int);
    let ref mut fresh37 =
        (*cave[(y_height + 6 as libc::c_int) as
                   usize].offset((x_left + 18 as libc::c_int) as isize)).info;
    *fresh37 =
        (*fresh37 as libc::c_int | (0x2 as libc::c_int | 0x1 as libc::c_int))
            as u16b;
    cave_set_feat(y_depth - 6 as libc::c_int, x_left + 18 as libc::c_int,
                  0x3c as libc::c_int);
    let ref mut fresh38 =
        (*cave[(y_depth - 6 as libc::c_int) as
                   usize].offset((x_left + 18 as libc::c_int) as isize)).info;
    *fresh38 =
        (*fresh38 as libc::c_int | (0x2 as libc::c_int | 0x1 as libc::c_int))
            as u16b;
    cave_set_feat(y_height + 6 as libc::c_int, x_right - 18 as libc::c_int,
                  0x3c as libc::c_int);
    let ref mut fresh39 =
        (*cave[(y_height + 6 as libc::c_int) as
                   usize].offset((x_right - 18 as libc::c_int) as
                                     isize)).info;
    *fresh39 =
        (*fresh39 as libc::c_int | (0x2 as libc::c_int | 0x1 as libc::c_int))
            as u16b;
    cave_set_feat(y_depth - 6 as libc::c_int, x_right - 18 as libc::c_int,
                  0x3c as libc::c_int);
    let ref mut fresh40 =
        (*cave[(y_depth - 6 as libc::c_int) as
                   usize].offset((x_right - 18 as libc::c_int) as
                                     isize)).info;
    *fresh40 =
        (*fresh40 as libc::c_int | (0x2 as libc::c_int | 0x1 as libc::c_int))
            as u16b;
    i = y_height + 5 as libc::c_int;
    j = xval + 66 as libc::c_int;
    cave_set_feat(i, j, 0x4a as libc::c_int);
    let ref mut fresh41 = (*cave[i as usize].offset(j as isize)).info;
    *fresh41 =
        (*fresh41 as libc::c_int | (0x2 as libc::c_int | 0x1 as libc::c_int))
            as u16b;
    player_place(i + 1 as libc::c_int, j);
}
/*
 * Town logic flow for generation of arena -KMW-
 */
unsafe extern "C" fn arena_gen() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut qy: libc::c_int = 22 as libc::c_int;
    let mut qx: libc::c_int = 66 as libc::c_int;
    let mut daytime: bool_ = 0;
    /* Day time */
    if (turn as libc::c_long %
            (10 as libc::c_long * 11520 as libc::c_int as libc::c_long)) <
           10 as libc::c_long * 11520 as libc::c_int as libc::c_long /
               2 as libc::c_int as libc::c_long {
        daytime = 1 as libc::c_int as bool_
    } else {
        /* Night time */
        daytime = 0 as libc::c_int as bool_
    }
    /* Start with solid walls */
    y = 0 as libc::c_int;
    while y < 66 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 198 as libc::c_int {
            /* Create "solid" perma-wall */
            cave_set_feat(y, x, 0x3f as libc::c_int);
            /* Illuminate and memorize the walls */
            let ref mut fresh42 = (*cave[y as usize].offset(x as isize)).info;
            *fresh42 =
                (*fresh42 as libc::c_int |
                     (0x2 as libc::c_int | 0x1 as libc::c_int)) as u16b;
            x += 1
        }
        y += 1
    }
    /* Then place some floors */
    y = qy + 1 as libc::c_int;
    while y < qy + 22 as libc::c_int - 1 as libc::c_int {
        x = qx + 1 as libc::c_int;
        while x < qx + 66 as libc::c_int - 1 as libc::c_int {
            /* Create empty floor */
            cave_set_feat(y, x, 0x1 as libc::c_int);
            /* Darken and forget the floors */
            let ref mut fresh43 = (*cave[y as usize].offset(x as isize)).info;
            *fresh43 =
                (*fresh43 as libc::c_int &
                     !(0x2 as libc::c_int | 0x1 as libc::c_int)) as u16b;
            /* Day time */
            if daytime != 0 {
                /* Perma-Lite */
                let ref mut fresh44 =
                    (*cave[y as usize].offset(x as isize)).info;
                *fresh44 =
                    (*fresh44 as libc::c_int | 0x2 as libc::c_int) as u16b;
                /* Memorize */
                if view_perma_grids != 0 {
                    let ref mut fresh45 =
                        (*cave[y as usize].offset(x as isize)).info;
                    *fresh45 =
                        (*fresh45 as libc::c_int | 0x1 as libc::c_int) as u16b
                }
            }
            x += 1
        }
        y += 1
    }
    build_arena();
    place_monster_aux((*p_ptr).py as libc::c_int + 5 as libc::c_int,
                      (*p_ptr).px as libc::c_int,
                      arena_monsters[(*p_ptr).arena_number as usize] as
                          libc::c_int, 0 as libc::c_int as bool_,
                      0 as libc::c_int as bool_, -(2 as libc::c_int));
}
/*
 * Generate a quest level
 */
unsafe extern "C" fn quest_gen() {
    process_hooks(2 as libc::c_int,
                  b"(d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, is_quest(dun_level as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn build_special_level() -> bool_ {
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut level: s16b = 0;
    /* No special levels on the surface */
    if dun_level == 0 { return 0 as libc::c_int as bool_ }
    level =
        (dun_level as libc::c_int -
             (*d_info.offset(dungeon_type as isize)).mindepth as libc::c_int)
            as s16b;
    if get_dungeon_save(buf.as_mut_ptr()) == 0 &&
           *special_lvl[level as usize].offset(dungeon_type as isize) as
               libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    if get_dungeon_special(buf.as_mut_ptr()) == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Big town */
    cur_hgt = 66 as libc::c_int as s16b;
    cur_wid = 198 as libc::c_int as s16b;
    /* Determine number of panels */
    max_panel_rows =
        (cur_hgt as libc::c_int / 22 as libc::c_int * 2 as libc::c_int -
             2 as libc::c_int) as s16b;
    max_panel_cols =
        (cur_wid as libc::c_int / 66 as libc::c_int * 2 as libc::c_int -
             2 as libc::c_int) as s16b;
    /* Assume illegal panel */
    panel_row_min =
        (max_panel_rows as libc::c_int *
             (22 as libc::c_int / 2 as libc::c_int)) as s16b;
    panel_col_min =
        (max_panel_cols as libc::c_int *
             (66 as libc::c_int / 2 as libc::c_int)) as s16b;
    /* Start with perm walls */
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    /* Set the correct monster hook */
    set_mon_num_hook();
    /* Prepare allocation table */
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int | 0x10 as libc::c_int;
    process_dungeon_file(buf.as_mut_ptr() as cptr, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
    *special_lvl[level as usize].offset(dungeon_type as isize) =
        0x2 as libc::c_int as bool_;
    generate_special_feeling = 1 as libc::c_int as bool_;
    /* Special feeling because it's special */
    good_item_flag = 1 as libc::c_int as bool_;
    /*
	 * Hack -- It's better/more dangerous than a greater vault.
	 * Better to have a rating field in special level description.
	 */
    rating = (rating as libc::c_int + 40 as libc::c_int) as s16b;
    return 1 as libc::c_int as bool_;
}
/*
 * Prepare regeneration of a special level, which should not happen,
 * but just in case...
 */
unsafe extern "C" fn wipe_special_level() {
    let mut level: s16b = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* No special levels on the surface */
    if dun_level == 0 { return }
    process_hooks(42 as libc::c_int,
                  b"()\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    /* Calculate relative depth */
    level =
        (dun_level as libc::c_int -
             (*d_info.offset(dungeon_type as isize)).mindepth as libc::c_int)
            as s16b;
    /* No special level at this depth */
    if get_dungeon_save(buf.as_mut_ptr()) == 0 &&
           *special_lvl[level as usize].offset(dungeon_type as isize) as
               libc::c_int != 0 {
        return
    }
    if get_dungeon_special(buf.as_mut_ptr()) == 0 { return }
    /* Clear the Mega-Hack flag */
    if *special_lvl[level as usize].offset(dungeon_type as isize) as
           libc::c_int == 0x2 as libc::c_int {
        *special_lvl[level as usize].offset(dungeon_type as isize) =
            0 as libc::c_int as bool_
    };
}
/*
 * Finalise generation of a special level
 */
unsafe extern "C" fn finalise_special_level() {
    let mut level: s16b = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* No special levels on the surface */
    if dun_level == 0 { return }
    process_hooks(43 as libc::c_int,
                  b"()\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    /* Calculate relative depth */
    level =
        (dun_level as libc::c_int -
             (*d_info.offset(dungeon_type as isize)).mindepth as libc::c_int)
            as s16b;
    /* No special level at this depth */
    if get_dungeon_save(buf.as_mut_ptr()) == 0 &&
           *special_lvl[level as usize].offset(dungeon_type as isize) as
               libc::c_int != 0 {
        return
    }
    if get_dungeon_special(buf.as_mut_ptr()) == 0 { return }
    /* Set the "generated" flag */
    if *special_lvl[level as usize].offset(dungeon_type as isize) as
           libc::c_int == 0x2 as libc::c_int {
        *special_lvl[level as usize].offset(dungeon_type as isize) =
            1 as libc::c_int as bool_
    };
}
/*
 * Give some magical energy to the each grid of the level
 */
#[no_mangle]
pub unsafe extern "C" fn generate_grid_mana() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut mana: libc::c_int = 0;
    let mut mult: libc::c_int = 0;
    let mut xtra_magic: bool_ = 0 as libc::c_int as bool_;
    if Rand_div(10 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int {
        xtra_magic = 1 as libc::c_int as bool_;
        if cheat_room as libc::c_int != 0 ||
               (*p_ptr).precognition as libc::c_int != 0 {
            msg_print(b"Magical level\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    mult =
        if xtra_magic as libc::c_int != 0 {
            3 as libc::c_int
        } else { 2 as libc::c_int };
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            /* Calculate the amount of mana in each grid */
            mana =
                mult *
                    m_bonus(255 as libc::c_int, dun_level as libc::c_int) as
                        libc::c_int / 2 as libc::c_int;
            if xtra_magic != 0 {
                mana += 10 as libc::c_int + Rand_div(10 as libc::c_int)
            }
            /* Never more than 255 or less than 0(paranoia) */
            if mana < 0 as libc::c_int { mana = 0 as libc::c_int }
            if mana > 255 as libc::c_int { mana = 255 as libc::c_int }
            (*c_ptr).mana = mana as byte_hack;
            x += 1
        }
        y += 1
    };
}
/*
 * Generates a random dungeon level			-RAK-
 *
 * Hack -- regenerate any "overflow" levels
 *
 * Hack -- allow auto-scumming via a gameplay option.
 */
#[no_mangle]
pub unsafe extern "C" fn generate_cave() {
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    let mut tester_1: libc::c_int = 0;
    let mut tester_2: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut loaded: bool_ = 0 as libc::c_int as bool_;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut town_level: s16b = 0 as libc::c_int as s16b;
    /* The dungeon is not ready */
    character_dungeon = 0 as libc::c_int as bool_;
    generate_special_feeling = 0 as libc::c_int as bool_;
    /* Initialize the flags with the basic dungeon flags */
    if dun_level == 0 {
        dungeon_flags1 = (*d_info.offset(0 as libc::c_int as isize)).flags1;
        dungeon_flags2 = (*d_info.offset(0 as libc::c_int as isize)).flags2
    } else {
        dungeon_flags1 = (*d_ptr).flags1;
        dungeon_flags2 = (*d_ptr).flags2
    }
    /* Is it a town level ? */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*d_ptr).t_level[i as usize] as libc::c_int ==
               dun_level as libc::c_int {
            town_level = (*d_ptr).t_idx[i as usize]
        }
        i += 1
    }
    /* Save the imprinted monsters */
    save_all_friends();
    wipe_m_list();
    /* Seed the RNG if appropriate */
    if town_level != 0 {
        Rand_quick = 1 as libc::c_int as bool_;
        Rand_value = (*town_info.offset(town_level as isize)).seed
    }
    process_hooks(49 as libc::c_int,
                  b"\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    /* Try to load a saved level */
    if get_dungeon_save(buf.as_mut_ptr()) != 0 {
        /* No effects */
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            effects[i as usize].time = 0 as libc::c_int as s16b;
            i += 1
        }
        /* Start with a blank cave */
        y = 0 as libc::c_int;
        while y < 66 as libc::c_int {
            x = 0 as libc::c_int;
            while x < 198 as libc::c_int {
                /* No flags */
                (*cave[y as usize].offset(x as isize)).info =
                    0 as libc::c_int as u16b;
                /* No features */
                cave_set_feat(y, x, 0x3d as libc::c_int);
                /* No objects */
                (*cave[y as usize].offset(x as isize)).o_idx =
                    0 as libc::c_int as s16b;
                /* No monsters */
                (*cave[y as usize].offset(x as isize)).m_idx =
                    0 as libc::c_int as s16b;
                /* No traps */
                (*cave[y as usize].offset(x as isize)).t_idx =
                    0 as libc::c_int as s16b;
                /* No mimic */
                (*cave[y as usize].offset(x as isize)).mimic =
                    0 as libc::c_int as byte_hack;
                /* No effects */
                (*cave[y as usize].offset(x as isize)).effect =
                    0 as libc::c_int as s16b;
                /* No inscription */
                (*cave[y as usize].offset(x as isize)).inscription =
                    0 as libc::c_int as s16b;
                /* No flow */
                (*cave[y as usize].offset(x as isize)).cost =
                    0 as libc::c_int as byte_hack;
                (*cave[y as usize].offset(x as isize)).when =
                    0 as libc::c_int as byte_hack;
                x += 1
            }
            y += 1
        }
        loaded = load_dungeon(buf.as_mut_ptr())
    }
    /* No saved level -- generate new one */
    if loaded == 0 {
        if get_dungeon_special(buf.as_mut_ptr()) == 0 ||
               *special_lvl[(dun_level as libc::c_int -
                                 (*d_info.offset(dungeon_type as
                                                     isize)).mindepth as
                                     libc::c_int) as
                                usize].offset(dungeon_type as isize) == 0 {
            get_level_flags();
        }
        /* Generate */
        num = 0 as libc::c_int;
        loop  {
            let mut okay: bool_ = 1 as libc::c_int as bool_;
            let mut why: cptr = 0 as cptr;
            /* No effects */
            i = 0 as libc::c_int;
            while i < 128 as libc::c_int {
                effects[i as usize].time = 0 as libc::c_int as s16b;
                i += 1
            }
            /* Start with a blank cave */
            y = 0 as libc::c_int;
            while y < 66 as libc::c_int {
                x = 0 as libc::c_int;
                while x < 198 as libc::c_int {
                    /* No flags */
                    (*cave[y as usize].offset(x as isize)).info =
                        0 as libc::c_int as u16b;
                    /* No features */
                    cave_set_feat(y, x, 0x3d as libc::c_int);
                    /* No objects */
                    (*cave[y as usize].offset(x as isize)).o_idx =
                        0 as libc::c_int as s16b;
                    /* No monsters */
                    (*cave[y as usize].offset(x as isize)).m_idx =
                        0 as libc::c_int as s16b;
                    /* No traps */
                    (*cave[y as usize].offset(x as isize)).t_idx =
                        0 as libc::c_int as s16b;
                    /* No mimic */
                    (*cave[y as usize].offset(x as isize)).mimic =
                        0 as libc::c_int as byte_hack;
                    /* No effect */
                    (*cave[y as usize].offset(x as isize)).effect =
                        0 as libc::c_int as s16b;
                    /* No inscription */
                    (*cave[y as usize].offset(x as isize)).inscription =
                        0 as libc::c_int as s16b;
                    /* No flow */
                    (*cave[y as usize].offset(x as isize)).cost =
                        0 as libc::c_int as byte_hack;
                    (*cave[y as usize].offset(x as isize)).when =
                        0 as libc::c_int as byte_hack;
                    x += 1
                }
                y += 1
            }
            /* XXX XXX XXX XXX */
            o_max = 1 as libc::c_int as s16b;
            /* Mega-Hack -- no player yet */
            (*p_ptr).py = 0 as libc::c_int as s16b;
            (*p_ptr).px = (*p_ptr).py;
            /* Mega-Hack -- no panel yet */
            panel_row_min = 0 as libc::c_int as s16b;
            panel_row_max = 0 as libc::c_int as s16b;
            panel_col_min = 0 as libc::c_int as s16b;
            panel_col_max = 0 as libc::c_int as s16b;
            /* Reset the monster generation level */
            if dungeon_type as libc::c_int != DUNGEON_DEATH {
                monster_level = dun_level
            } else {
                monster_level =
                    ((*p_ptr).lev as libc::c_int * 2 as libc::c_int +
                         10 as libc::c_int + Rand_div(40 as libc::c_int)) as
                        s16b
            }
            /* Reset the object generation level */
            object_level = dun_level;
            /* Nothing special here yet */
            good_item_flag = 0 as libc::c_int as bool_;
            /* Nothing good here yet */
            rating = 0 as libc::c_int as s16b;
            /* No ambush here yet */
            ambush_flag = 0 as libc::c_int as bool_;
            /* No fated level here yet */
            fate_flag = 0 as libc::c_int as bool_;
            /* Build the arena -KMW- */
            if (*p_ptr).inside_arena != 0 {
                /* Small arena */
                arena_gen();
            } else if (*p_ptr).inside_quest != 0 {
                quest_gen();
            } else if !(build_special_level() != 0) {
                /* Quest levels -KMW- */
                /* Special levels */
                /* Build the town */
                if dun_level == 0 {
                    /* Big town */
                    cur_hgt = 66 as libc::c_int as s16b;
                    cur_wid = 198 as libc::c_int as s16b;
                    /* Determine number of panels */
                    max_panel_rows =
                        (cur_hgt as libc::c_int / 22 as libc::c_int *
                             2 as libc::c_int - 2 as libc::c_int) as s16b;
                    max_panel_cols =
                        (cur_wid as libc::c_int / 66 as libc::c_int *
                             2 as libc::c_int - 2 as libc::c_int) as s16b;
                    /* Assume illegal panel */
                    panel_row_min =
                        (max_panel_rows as libc::c_int *
                             (22 as libc::c_int / 2 as libc::c_int)) as s16b;
                    panel_col_min =
                        (max_panel_cols as libc::c_int *
                             (66 as libc::c_int / 2 as libc::c_int)) as s16b;
                    /* Big wilderness mode */
                    if (*p_ptr).wild_mode == 0 {
                        /* Make the wilderness */
                        wilderness_gen(0 as libc::c_int);
                    } else {
                        /* Small wilderness mode */
                        /* Make the wilderness */
                        wilderness_gen_small();
                    }
                    okay = 1 as libc::c_int as bool_
                } else {
                    /* Build a dungeon level */
                    /* Requested size level */
                    if (*d_ptr).size_x != -(1 as libc::c_int) {
                        if cheat_room as libc::c_int != 0 ||
                               (*p_ptr).precognition as libc::c_int != 0 {
                            msg_print(b"A \'size\' dungeon level.\x00" as
                                          *const u8 as *const libc::c_char);
                        }
                        cur_hgt =
                            ((*d_ptr).size_y * 22 as libc::c_int) as s16b;
                        cur_wid =
                            ((*d_ptr).size_x * 66 as libc::c_int) as s16b;
                        /* Determine number of panels */
                        max_panel_rows =
                            (cur_hgt as libc::c_int / 22 as libc::c_int *
                                 2 as libc::c_int - 2 as libc::c_int) as s16b;
                        max_panel_cols =
                            (cur_wid as libc::c_int / 66 as libc::c_int *
                                 2 as libc::c_int - 2 as libc::c_int) as s16b;
                        /* Assume illegal panel */
                        panel_row_min =
                            (max_panel_rows as libc::c_int *
                                 (22 as libc::c_int / 2 as libc::c_int)) as
                                s16b;
                        panel_col_min =
                            (max_panel_cols as libc::c_int *
                                 (66 as libc::c_int / 2 as libc::c_int)) as
                                s16b;
                        if cheat_room != 0 {
                            msg_format(b"X:%d, Y:%d.\x00" as *const u8 as
                                           *const libc::c_char,
                                       max_panel_cols as libc::c_int,
                                       max_panel_rows as libc::c_int);
                        }
                    } else if dungeon_flags1 as libc::c_long &
                                  0x10 as libc::c_long == 0 &&
                                  dungeon_flags1 as libc::c_long &
                                      0x4 as libc::c_long != 0 {
                        if cheat_room as libc::c_int != 0 ||
                               (*p_ptr).precognition as libc::c_int != 0 {
                            msg_print(b"A \'small\' dungeon level.\x00" as
                                          *const u8 as *const libc::c_char);
                        }
                        cur_hgt = 22 as libc::c_int as s16b;
                        cur_wid = 66 as libc::c_int as s16b;
                        /* Very small (1 x 1 panel) level */
                        /* Determine number of panels */
                        max_panel_rows = 1 as libc::c_int as s16b;
                        max_panel_cols = 1 as libc::c_int as s16b;
                        /* Assume illegal panel */
                        panel_row_min =
                            (max_panel_rows as libc::c_int *
                                 (22 as libc::c_int / 2 as libc::c_int)) as
                                s16b;
                        panel_col_min =
                            (max_panel_cols as libc::c_int *
                                 (66 as libc::c_int / 2 as libc::c_int)) as
                                s16b;
                        if cheat_room != 0 {
                            msg_format(b"X:1, Y:1.\x00" as *const u8 as
                                           *const libc::c_char);
                        }
                    } else if dungeon_flags1 as libc::c_long &
                                  0x10 as libc::c_long == 0 &&
                                  (always_small_level as libc::c_int != 0 ||
                                       dungeon_flags1 as libc::c_long &
                                           0x8 as libc::c_long != 0 ||
                                       small_levels as libc::c_int != 0 &&
                                           Rand_div(6 as libc::c_int) ==
                                               0 as libc::c_int) {
                        if cheat_room as libc::c_int != 0 ||
                               (*p_ptr).precognition as libc::c_int != 0 {
                            msg_print(b"A \'small\' dungeon level.\x00" as
                                          *const u8 as *const libc::c_char);
                        }
                        tester_1 =
                            1 as libc::c_int +
                                Rand_div(1 as libc::c_int +
                                             66 as libc::c_int /
                                                 22 as libc::c_int -
                                             1 as libc::c_int);
                        tester_2 =
                            1 as libc::c_int +
                                Rand_div(1 as libc::c_int +
                                             (198 as libc::c_int /
                                                  66 as libc::c_int -
                                                  1 as libc::c_int) -
                                             1 as libc::c_int);
                        cur_hgt = (tester_1 * 22 as libc::c_int) as s16b;
                        cur_wid = (tester_2 * 66 as libc::c_int) as s16b;
                        /* Small level */
                        /* Determine number of panels */
                        max_panel_rows =
                            (cur_hgt as libc::c_int / 22 as libc::c_int *
                                 2 as libc::c_int - 2 as libc::c_int) as s16b;
                        max_panel_cols =
                            (cur_wid as libc::c_int / 66 as libc::c_int *
                                 2 as libc::c_int - 2 as libc::c_int) as s16b;
                        /* Assume illegal panel */
                        panel_row_min =
                            (max_panel_rows as libc::c_int *
                                 (22 as libc::c_int / 2 as libc::c_int)) as
                                s16b;
                        panel_col_min =
                            (max_panel_cols as libc::c_int *
                                 (66 as libc::c_int / 2 as libc::c_int)) as
                                s16b;
                        if cheat_room != 0 {
                            msg_format(b"X:%d, Y:%d.\x00" as *const u8 as
                                           *const libc::c_char,
                                       max_panel_cols as libc::c_int,
                                       max_panel_rows as libc::c_int);
                        }
                    } else {
                        /* Normal level */
                        /* Use full panels */
                        cur_hgt = 66 as libc::c_int as s16b;
                        cur_wid = 198 as libc::c_int as s16b;
                        max_panel_rows =
                            (cur_hgt as libc::c_int / 22 as libc::c_int *
                                 2 as libc::c_int - 2 as libc::c_int) as s16b;
                        max_panel_cols =
                            (cur_wid as libc::c_int / 66 as libc::c_int *
                                 2 as libc::c_int - 2 as libc::c_int) as s16b;
                        panel_row_min =
                            (max_panel_rows as libc::c_int *
                                 (22 as libc::c_int / 2 as libc::c_int)) as
                                s16b;
                        panel_col_min =
                            (max_panel_cols as libc::c_int *
                                 (66 as libc::c_int / 2 as libc::c_int)) as
                                s16b
                    }
                    if cave_gen() == 0 {
                        why =
                            b"could not place player\x00" as *const u8 as
                                *const libc::c_char;
                        okay = 0 as libc::c_int as bool_
                    }
                }
            }
            /* Determine number of panels */
            /* Assume illegal panel */
            /* Generate a level */
            /* Extract the feeling */
            if rating as libc::c_int > 100 as libc::c_int {
                feeling = 2 as libc::c_int as s16b
            } else if rating as libc::c_int > 80 as libc::c_int {
                feeling = 3 as libc::c_int as s16b
            } else if rating as libc::c_int > 60 as libc::c_int {
                feeling = 4 as libc::c_int as s16b
            } else if rating as libc::c_int > 40 as libc::c_int {
                feeling = 5 as libc::c_int as s16b
            } else if rating as libc::c_int > 30 as libc::c_int {
                feeling = 6 as libc::c_int as s16b
            } else if rating as libc::c_int > 20 as libc::c_int {
                feeling = 7 as libc::c_int as s16b
            } else if rating as libc::c_int > 10 as libc::c_int {
                feeling = 8 as libc::c_int as s16b
            } else if rating as libc::c_int > 0 as libc::c_int {
                feeling = 9 as libc::c_int as s16b
            } else { feeling = 10 as libc::c_int as s16b }
            /* Hack -- Have a special feeling sometimes */
            if good_item_flag as libc::c_int != 0 && (*p_ptr).preserve == 0 {
                feeling = 1 as libc::c_int as s16b
            }
            /* It takes 1000 game turns for "feelings" to recharge */
            if turn - old_turn < 1000 as libc::c_int {
                feeling = 0 as libc::c_int as s16b
            }
            /* Hack -- no feeling in the town */
            if dun_level == 0 { feeling = 0 as libc::c_int as s16b }
            /* Prevent object over-flow */
            if o_max as libc::c_int >= max_o_idx as libc::c_int {
                /* Message */
                why =
                    b"too many objects\x00" as *const u8 as
                        *const libc::c_char;
                /* Message */
                okay = 0 as libc::c_int as bool_
            }
            /* Prevent monster over-flow */
            if m_max as libc::c_int >= max_m_idx as libc::c_int {
                /* Message */
                why =
                    b"too many monsters\x00" as *const u8 as
                        *const libc::c_char;
                /* Message */
                okay = 0 as libc::c_int as bool_
            }
            /* Mega-Hack -- "auto-scum" */
            if auto_scum as libc::c_int != 0 && num < 100 as libc::c_int &&
                   (*p_ptr).inside_quest == 0 && dun_level as libc::c_int != 0
               {
                /* Require "goodness" */
                if feeling as libc::c_int > 9 as libc::c_int ||
                       dun_level as libc::c_int >= 5 as libc::c_int &&
                           feeling as libc::c_int > 8 as libc::c_int ||
                       dun_level as libc::c_int >= 10 as libc::c_int &&
                           feeling as libc::c_int > 7 as libc::c_int ||
                       dun_level as libc::c_int >= 20 as libc::c_int &&
                           feeling as libc::c_int > 6 as libc::c_int ||
                       dun_level as libc::c_int >= 40 as libc::c_int &&
                           feeling as libc::c_int > 5 as libc::c_int {
                    /* Give message to cheaters */
                    if cheat_room as libc::c_int != 0 ||
                           cheat_hear as libc::c_int != 0 ||
                           cheat_peek as libc::c_int != 0 ||
                           cheat_xtra as libc::c_int != 0 ||
                           (*p_ptr).precognition as libc::c_int != 0 {
                        /* Message */
                        why =
                            b"boring level\x00" as *const u8 as
                                *const libc::c_char
                    }
                    /* Try again */
                    okay = 0 as libc::c_int as bool_
                }
            }
            /* Accept */
            if okay as libc::c_int != 0 || town_level as libc::c_int != 0 {
                break ;
            }
            /* Message */
            if !why.is_null() {
                msg_format(b"Generation restarted (%s)\x00" as *const u8 as
                               *const libc::c_char, why);
            }
            /* Wipe the objects */
            wipe_o_list();
            /* Wipe the monsters */
            wipe_m_list();
            /* Clear the fate icky flags */
            i = 0 as libc::c_int;
            while i < 200 as libc::c_int {
                fates[i as usize].icky = 0 as libc::c_int as bool_;
                i += 1
            }
            /*
			 * Mega-Hack -- Reset special level flag if necessary
			 * XXX XXX XXX
			 */
            wipe_special_level();
            num += 1
        }
        /* Give some mana to each grid -- DG */
        generate_grid_mana();
    }
    /* Put the kept monsters -- DG */
    if (*p_ptr).wild_mode == 0 { replace_all_friends(); }
    /* Hack -- Clear used up fates */
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        if fates[i as usize].icky != 0 {
            /* Mark the artefact as generated */
            if fates[i as usize].fate as libc::c_int == 3 as libc::c_int &&
                   fates[i as usize].a_idx as libc::c_int != 0 {
                (*a_info.offset(fates[i as usize].a_idx as isize)).cur_num =
                    1 as libc::c_int as byte_hack
            }
            fates[i as usize].fate = 0 as libc::c_int as byte_hack;
            fates[i as usize].icky = 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Set special level generated flag if applicable */
    finalise_special_level();
    /* No teleporatations yet */
    last_teleportation_y = -(1 as libc::c_int) as s16b;
    last_teleportation_x = -(1 as libc::c_int) as s16b;
    /* Mark the dungeon town as found */
    if town_level != 0 {
        /* Set the known flag */
        let ref mut fresh46 = (*town_info.offset(town_level as isize)).flags;
        *fresh46 = (*fresh46 as libc::c_int | 0x2 as libc::c_int) as byte_hack
    }
    /* The dungeon is ready */
    character_dungeon = 1 as libc::c_int as bool_;
    /* Remember when this level was "created" */
    old_turn = turn;
    /* Provide astral chars with the full map */
    if (*p_ptr).astral as libc::c_int != 0 && dun_level as libc::c_int != 0 {
        wiz_lite_extra();
    }
    /* Player should get the first move upon entering the dungeon */
    (*p_ptr).energy = 100 as libc::c_int;
}
