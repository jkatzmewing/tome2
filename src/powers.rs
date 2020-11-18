use ::libc;
extern "C" {
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut stat_names: [cptr; 6];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut num_repro: s16b;
    #[no_mangle]
    static mut flush_failure: bool_;
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
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut item_tester_tval: byte_hack;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut powers_type: *mut power_type;
    #[no_mangle]
    static mut power_max: s16b;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn map_area();
    #[no_mangle]
    fn scatter(yp: *mut libc::c_int, xp: *mut libc::c_int, y: libc::c_int,
               x: libc::c_int, d: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn py_attack(y: libc::c_int, x: libc::c_int, max_blow: libc::c_int);
    #[no_mangle]
    fn do_cmd_integrate_body() -> bool_;
    #[no_mangle]
    fn do_cmd_throw();
    #[no_mangle]
    fn do_cmd_immovable_special();
    #[no_mangle]
    fn fetch(dir: libc::c_int, wgt: libc::c_int, require_los: bool_);
    #[no_mangle]
    fn do_poly_self();
    #[no_mangle]
    fn use_ability_blade();
    #[no_mangle]
    fn do_cmd_beastmaster();
    #[no_mangle]
    fn do_cmd_necromancer();
    #[no_mangle]
    fn autosave_checkpoint();
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn delete_monster_idx(i: libc::c_int);
    #[no_mangle]
    fn delete_monster(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn summon_specific_friendly(y1: libc::c_int, x1: libc::c_int,
                                lev: libc::c_int, type_0: libc::c_int,
                                Group_ok: bool_) -> bool_;
    #[no_mangle]
    fn place_monster_one(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         ego: libc::c_int, slp: bool_, status: libc::c_int)
     -> s16b;
    #[no_mangle]
    fn can_create_companion() -> bool_;
    #[no_mangle]
    fn inc_stack_size(item: libc::c_int, delta: libc::c_int);
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn floor_item_describe(item: libc::c_int);
    #[no_mangle]
    fn floor_item_increase(item: libc::c_int, num: libc::c_int);
    #[no_mangle]
    fn floor_item_optimize(item: libc::c_int);
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_value(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn fire_bolt(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_rep_dir(dp: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn recall_player(d: libc::c_int, f: libc::c_int);
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn fire_beam(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_aim_dir(dp: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn turn_monsters(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn confuse_monsters(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn stun_monsters(dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn teleport_player(dis: libc::c_int);
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn report_magics();
    #[no_mangle]
    fn item_tester_hook_recharge(o_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn earthquake(cy: libc::c_int, cx: libc::c_int, r: libc::c_int);
    #[no_mangle]
    fn set_oppose_pois(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_cold(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_fire(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_elec(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_acid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn alchemy() -> bool_;
    #[no_mangle]
    fn lite_area(dam: libc::c_int, rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn aggravate_monsters(who: libc::c_int);
    #[no_mangle]
    fn fire_ball(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int,
                 rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn teleport_swap(dir: libc::c_int);
    #[no_mangle]
    fn verify_panel();
    #[no_mangle]
    fn wall_to_mud(dir: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_food(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_monsters_normal(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_treasure(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn charm_monster(dir: libc::c_int, plev: libc::c_int) -> bool_;
    #[no_mangle]
    fn restore_level() -> bool_;
    #[no_mangle]
    fn fear_monster(dir: libc::c_int, plev: libc::c_int) -> bool_;
    #[no_mangle]
    fn hp_player(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn drain_life(dir: libc::c_int, dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn grow_trees(rad: libc::c_int);
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn teleport_player_to(ny: libc::c_int, nx: libc::c_int);
    #[no_mangle]
    fn tgt_pt(x: *mut libc::c_int, y: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn fire_bolt_or_beam(prob: libc::c_int, typ: libc::c_int,
                         dir: libc::c_int, dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_stairs(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_doors(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn detect_traps(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_light_speed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn explosive_rune();
    #[no_mangle]
    fn set_shero(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_afraid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn passwall(dir: libc::c_int, safe: bool_) -> bool_;
    #[no_mangle]
    fn do_cmd_set_trap();
    #[no_mangle]
    fn ident_spell() -> bool_;
    #[no_mangle]
    fn quark_add(str: cptr) -> s16b;
    #[no_mangle]
    fn quark_str(num: s16b) -> cptr;
    #[no_mangle]
    fn resolve_mimic_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn set_mimic(v: libc::c_int, p: libc::c_int, level: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_disrupt_shield(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_invuln(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn repeat_push(what: libc::c_int);
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn repeat_pull(what: *mut libc::c_int) -> bool_;
}
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
/* File: powers.c */
/* Purpose: Powers */
/*
 * Copyright (c) 2001 James E. Wilson, Robert A. Koeneke, DarkGod
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Note: return value indicates the amount of mana to use
 */
#[no_mangle]
pub unsafe extern "C" fn power_chance(mut x_ptr: *mut power_type) -> bool_ {
    let mut use_hp: bool_ = 0 as libc::c_int as bool_;
    let mut diff: libc::c_int = (*x_ptr).diff as libc::c_int;
    /* Always true ? */
    if (*x_ptr).cost == 0 { return 1 as libc::c_int as bool_ }
    /* Not enough mana - use hp */
    if ((*p_ptr).csp as libc::c_int) < (*x_ptr).cost as libc::c_int {
        use_hp = 1 as libc::c_int as bool_
    }
    /* Power is not available yet */
    if ((*p_ptr).lev as libc::c_int) < (*x_ptr).level as libc::c_int {
        msg_format(b"You need to attain level %d to use this power.\x00" as
                       *const u8 as *const libc::c_char,
                   (*x_ptr).level as libc::c_int);
        energy_use = 0 as libc::c_int;
        return 0 as libc::c_int as bool_
    } else {
        /* Too confused */
        if (*p_ptr).confused != 0 {
            msg_print(b"You are too confused to use this power.\x00" as
                          *const u8 as *const libc::c_char);
            energy_use = 0 as libc::c_int;
            return 0 as libc::c_int as bool_
        } else {
            /* Risk death? */
            if use_hp as libc::c_int != 0 &&
                   ((*p_ptr).chp as libc::c_int) <
                       (*x_ptr).cost as libc::c_int {
                if get_check(b"Really use the power in your weakened state? \x00"
                                 as *const u8 as *const libc::c_char) == 0 {
                    energy_use = 0 as libc::c_int;
                    return 0 as libc::c_int as bool_
                }
            }
        }
    }
    /* Else attempt to do it! */
    if (*p_ptr).stun != 0 {
        diff += (*p_ptr).stun as libc::c_int
    } else if (*p_ptr).lev as libc::c_int > (*x_ptr).level as libc::c_int {
        let mut lev_adj: libc::c_int =
            ((*p_ptr).lev as libc::c_int - (*x_ptr).level as libc::c_int) /
                3 as libc::c_int;
        if lev_adj > 10 as libc::c_int { lev_adj = 10 as libc::c_int }
        diff -= lev_adj
    }
    if diff < 5 as libc::c_int { diff = 5 as libc::c_int }
    /* take time and pay the price */
    if use_hp != 0 {
        take_hit((*x_ptr).cost as libc::c_int / 2 as libc::c_int +
                     (Rand_div((*x_ptr).cost as libc::c_int /
                                   2 as libc::c_int) + 1 as libc::c_int),
                 b"concentrating too hard\x00" as *const u8 as
                     *const libc::c_char);
    } else {
        (*p_ptr).csp =
            ((*p_ptr).csp as libc::c_int -
                 ((*x_ptr).cost as libc::c_int / 2 as libc::c_int +
                      (Rand_div((*x_ptr).cost as libc::c_int /
                                    2 as libc::c_int) + 1 as libc::c_int))) as
                s16b
    }
    energy_use = 100 as libc::c_int;
    /* Redraw mana and hp */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x40 as libc::c_long | 0x80 as libc::c_long)) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    /* Success? */
    if Rand_div((*p_ptr).stat_cur[(*x_ptr).stat as usize] as s32b) +
           1 as libc::c_int >=
           diff / 2 as libc::c_int +
               (Rand_div(diff / 2 as libc::c_int) + 1 as libc::c_int) {
        return 1 as libc::c_int as bool_
    }
    if flush_failure != 0 { flush(); }
    msg_print(b"You\'ve failed to concentrate hard enough.\x00" as *const u8
                  as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
unsafe extern "C" fn power_activate(mut power: libc::c_int) {
    let mut plev: s16b = (*p_ptr).lev;
    let mut ch: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut amber_power: libc::c_int = 0 as libc::c_int;
    let mut dir: libc::c_int = 0 as libc::c_int;
    let mut dummy: libc::c_int = 0;
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
    let mut ii: libc::c_int = 0 as libc::c_int;
    let mut ij: libc::c_int = 0 as libc::c_int;
    /* char out_val[80]; */
	/* cptr p = "Power of the flame: "; */
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut x_ptr: *mut power_type =
        &mut *powers_type.offset(power as isize) as *mut power_type;
    let mut x_ptr_foo: power_type =
        power_type{name: 0 as *mut libc::c_char,
                   desc_text: 0 as *mut libc::c_char,
                   gain_text: 0 as *mut libc::c_char,
                   lose_text: 0 as *mut libc::c_char,
                   level: 0,
                   cost: 0,
                   stat: 0,
                   diff: 0,};
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Break goi/manashield */
    if (*p_ptr).invuln != 0 { set_invuln(0 as libc::c_int); }
    if (*p_ptr).disrupt_shield != 0 { set_disrupt_shield(0 as libc::c_int); }
    if power_chance(x_ptr) == 0 { return }
    let mut current_block_502: u64;
    match power {
        61 => {
            set_mimic((*p_ptr).lev as libc::c_int / 2 as libc::c_int,
                      resolve_mimic_name(b"Balrog\x00" as *const u8 as
                                             *const libc::c_char),
                      (*p_ptr).lev as libc::c_int);
        }
        59 => {
            set_mimic(150 as libc::c_int +
                          (*p_ptr).lev as libc::c_int * 10 as libc::c_int,
                      resolve_mimic_name(b"Bear\x00" as *const u8 as
                                             *const libc::c_char),
                      (*p_ptr).lev as libc::c_int);
        }
        58 => {
            if can_create_companion() == 0 {
                msg_print(b"You cannot have more companions.\x00" as *const u8
                              as *const libc::c_char);
            } else {
                let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
                let mut ii_0: libc::c_int = 0;
                let mut jj: libc::c_int = 0;
                msg_print(b"Select the friendly monster:\x00" as *const u8 as
                              *const libc::c_char);
                if tgt_pt(&mut ii_0, &mut jj) == 0 { return }
                if (*cave[jj as usize].offset(ii_0 as isize)).m_idx != 0 {
                    m_ptr =
                        &mut *m_list.offset((*(*cave.as_mut_ptr().offset(jj as
                                                                             isize)).offset(ii_0
                                                                                                as
                                                                                                isize)).m_idx
                                                as isize) as
                            *mut monster_type;
                    if (*m_ptr).status as libc::c_int != 3 as libc::c_int {
                        msg_print(b"You cannot convert this monster.\x00" as
                                      *const u8 as *const libc::c_char);
                        return
                    }
                    (*m_ptr).status = 4 as libc::c_int as s16b
                }
            }
        }
        57 => {
            loop 
                 /* Select power to use */
                 {
                if get_com(b"[A]ppraise item, [W]arp item or [I]dentify item? \x00"
                               as *const u8 as *const libc::c_char, &mut ch)
                       == 0 {
                    amber_power = 0 as libc::c_int;
                    break ;
                } else if ch as libc::c_int == 'A' as i32 ||
                              ch as libc::c_int == 'a' as i32 {
                    amber_power = 1 as libc::c_int;
                    break ;
                } else if ch as libc::c_int == 'W' as i32 ||
                              ch as libc::c_int == 'w' as i32 {
                    amber_power = 2 as libc::c_int;
                    break ;
                } else {
                    if !(ch as libc::c_int == 'I' as i32 ||
                             ch as libc::c_int == 'i' as i32) {
                        continue ;
                    }
                    amber_power = 3 as libc::c_int;
                    break ;
                }
            }
            if amber_power == 1 as libc::c_int {
                x_ptr_foo.level = 5 as libc::c_int as byte_hack;
                x_ptr_foo.cost = 5 as libc::c_int as byte_hack;
                x_ptr_foo.stat = 1 as libc::c_int as byte_hack;
                x_ptr_foo.diff = 5 as libc::c_int as byte_hack;
                if power_chance(&mut x_ptr_foo) != 0 {
                    /* Appraise an object */
                    let mut idx: libc::c_int = 0;
                    let mut q_0: cptr = 0 as *const libc::c_char;
                    let mut s_0: cptr = 0 as *const libc::c_char;
                    /* Get the item */
                    q_0 =
                        b"Appraise which item? \x00" as *const u8 as
                            *const libc::c_char;
                    s_0 =
                        b"You have nothing to appraise.\x00" as *const u8 as
                            *const libc::c_char;
                    if get_item(&mut idx, q_0, s_0,
                                0x1 as libc::c_int | 0x2 as libc::c_int |
                                    0x4 as libc::c_int) != 0 {
                        let mut o_ptr: *mut object_type =
                            0 as *mut object_type;
                        let mut out_val: [libc::c_char; 80] = [0; 80];
                        let mut value: [libc::c_char; 16] = [0; 16];
                        /* The item is in the pack */
                        if idx >= 0 as libc::c_int {
                            o_ptr =
                                &mut *(*p_ptr).inventory.as_mut_ptr().offset(idx
                                                                                 as
                                                                                 isize)
                                    as *mut object_type
                        } else {
                            /* The item is on the floor */
                            o_ptr =
                                &mut *o_list.offset((0 as libc::c_int - idx)
                                                        as isize) as
                                    *mut object_type
                        }
                        /* Appraise it */
                        sprintf(value.as_mut_ptr(),
                                b"%i au\x00" as *const u8 as
                                    *const libc::c_char, object_value(o_ptr));
                        /* Inscribe the value */
					/* Get the original inscription */
                        if (*o_ptr).note != 0 {
                            strcpy(out_val.as_mut_ptr(),
                                   quark_str((*o_ptr).note as s16b));
                            strcat(out_val.as_mut_ptr(),
                                   b" \x00" as *const u8 as
                                       *const libc::c_char);
                        } else {
                            out_val[0 as libc::c_int as usize] =
                                '\u{0}' as i32 as libc::c_char
                        }
                        strcat(out_val.as_mut_ptr(), value.as_mut_ptr());
                        /* Save the new inscription */
                        (*o_ptr).note =
                            quark_add(out_val.as_mut_ptr() as cptr) as u16b;
                        /* Combine the pack */
                        (*p_ptr).notice =
                            ((*p_ptr).notice as libc::c_long |
                                 0x1 as libc::c_long) as u32b;
                        /* Window stuff */
                        (*p_ptr).window =
                            ((*p_ptr).window as libc::c_long |
                                 (0x1 as libc::c_long | 0x2 as libc::c_long))
                                as u32b
                    }
                }
            }
            if amber_power == 2 as libc::c_int {
                x_ptr_foo.level = 15 as libc::c_int as byte_hack;
                x_ptr_foo.cost = 10 as libc::c_int as byte_hack;
                x_ptr_foo.stat = 1 as libc::c_int as byte_hack;
                x_ptr_foo.diff = 7 as libc::c_int as byte_hack;
                if power_chance(&mut x_ptr_foo) != 0 {
                    let mut chest: libc::c_int = 0;
                    let mut item: libc::c_int = 0;
                    let mut q1: cptr = 0 as *const libc::c_char;
                    let mut s1: cptr = 0 as *const libc::c_char;
                    let mut q2: cptr = 0 as *const libc::c_char;
                    let mut s2: cptr = 0 as *const libc::c_char;
                    let mut flag: u32b =
                        (0x1 as libc::c_int | 0x2 as libc::c_int |
                             0x4 as libc::c_int) as u32b;
                    let mut o1_ptr: *mut object_type =
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                            as *mut object_type;
                    let mut o2_ptr: *mut object_type =
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset(1 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                            as *mut object_type;
                    let mut ok: libc::c_int = 0 as libc::c_int;
                    q1 =
                        b"Select a chest! \x00" as *const u8 as
                            *const libc::c_char;
                    s1 =
                        b"You need a chest to warp items.\x00" as *const u8 as
                            *const libc::c_char;
                    q2 =
                        b"Warp which item? \x00" as *const u8 as
                            *const libc::c_char;
                    s2 =
                        b"You have nothing to warp.\x00" as *const u8 as
                            *const libc::c_char;
                    item_tester_tval = 7 as libc::c_int as byte_hack;
                    /* Get the chest */
                    if get_item(&mut chest, q1, s1, flag as libc::c_int) != 0
                       {
                        if chest >= 0 as libc::c_int {
                            o1_ptr =
                                &mut *(*p_ptr).inventory.as_mut_ptr().offset(chest
                                                                                 as
                                                                                 isize)
                                    as *mut object_type
                        } else {
                            o1_ptr =
                                &mut *o_list.offset((0 as libc::c_int - chest)
                                                        as isize) as
                                    *mut object_type
                        }
                        /* Is the chest disarmed? */
                        if (*o1_ptr).pval > 0 as libc::c_int {
                            msg_print(b"This chest may be trapped.\x00" as
                                          *const u8 as *const libc::c_char);
                        } else if (*k_info.offset((*o1_ptr).k_idx as
                                                      isize)).level as
                                      libc::c_int <= 0 as libc::c_int {
                            msg_print(b"This chest is broken.\x00" as
                                          *const u8 as *const libc::c_char);
                        } else if (*o1_ptr).pval2 as libc::c_int >=
                                      (*o1_ptr).sval as libc::c_int %
                                          4 as libc::c_int * 2 as libc::c_int
                         {
                            msg_print(b"This chest is full.\x00" as *const u8
                                          as *const libc::c_char);
                        } else { ok = 1 as libc::c_int }
                    }
                    /* Is it ruined? */
                    /* Is it empty? */
                    /* Get the item */
                    if ok != 0 &&
                           get_item(&mut item, q2, s2, flag as libc::c_int) as
                               libc::c_int != 0 {
                        ok = 0 as libc::c_int;
                        o2_ptr = get_object(item);
                        /* Is the item cursed? */
                        if item >= 24 as libc::c_int &&
                               (*o2_ptr).ident as libc::c_int &
                                   0x40 as libc::c_int != 0 {
                            msg_print(b"Hmmm, it seems to be cursed.\x00" as
                                          *const u8 as *const libc::c_char);
                        }
                        /* Is it the same chest? */
                        if item == chest {
                            msg_print(b"You can\'t put a chest into itself.\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                        }
                        /* Is it another chest? */
                        if (*o2_ptr).tval as libc::c_int == 7 as libc::c_int {
                            msg_print(b"You can\'t put a chest into another one.\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                        } else {
                            /* Try to use the power */
                            ok = 1 as libc::c_int
                        }
                    }
                    if ok != 0 {
                        let mut tmp: libc::c_int = 0;
                        let mut level: libc::c_int = 0;
                        /* Calculate the level of objects */
                        tmp = (*o1_ptr).pval;
                        /* Get the level of the current object */
					/* Cursed items/cheap items always break */
                        if (*k_info.offset((*o2_ptr).k_idx as isize)).cost <
                               20 as libc::c_int {
                            level = 0 as libc::c_int
                        } else if (*k_info.offset((*o2_ptr).k_idx as
                                                      isize)).cost <
                                      100 as libc::c_int {
                            level = 1 as libc::c_int
                        } else {
                            level =
                                (*k_info.offset((*o2_ptr).k_idx as
                                                    isize)).level as
                                    libc::c_int
                        }
                        /* Not-so-cheap items break 90% of the time */
                        /* Break some items */
                        if Rand_div(10 as libc::c_int) + 1 as libc::c_int >
                               level {
                            msg_print(b"The item disappeared!\x00" as
                                          *const u8 as *const libc::c_char);
                        } else {
                            level /=
                                (*o1_ptr).sval as libc::c_int %
                                    4 as libc::c_int * 2 as libc::c_int;
                            /* Increase the number of objects in
						 * the chest */
                            (*o1_ptr).pval2 += 1;
                            /* Set the level of chest */
                            tmp = tmp - level;
                            (*o1_ptr).pval = tmp
                        }
                        /* Destroy item */
                        inc_stack_size(item, -(1 as libc::c_int));
                    }
                }
            }
            if amber_power == 3 as libc::c_int {
                x_ptr_foo.level = 30 as libc::c_int as byte_hack;
                x_ptr_foo.cost = 20 as libc::c_int as byte_hack;
                x_ptr_foo.stat = 1 as libc::c_int as byte_hack;
                x_ptr_foo.diff = 7 as libc::c_int as byte_hack;
                if power_chance(&mut x_ptr_foo) != 0 { ident_spell(); }
            }
        }
        56 => { do_cmd_set_trap(); }
        55 => {
            msg_print(b"You sense the world around you.\x00" as *const u8 as
                          *const libc::c_char);
            map_area();
        }
        32 => {
            if !(get_aim_dir(&mut dir) == 0) {
                if passwall(dir, 1 as libc::c_int as bool_) != 0 {
                    msg_print(b"A passage opens, and you step through.\x00" as
                                  *const u8 as *const libc::c_char);
                } else {
                    msg_print(b"There is no wall there!\x00" as *const u8 as
                                  *const libc::c_char);
                }
            }
        }
        34 => {
            /* Get local object */
            q_ptr = &mut forge;
            /* Create the item */
            object_prep(q_ptr, 21 as libc::c_int);
            /* Drop the object from heaven */
            drop_near(q_ptr, -(1 as libc::c_int), (*p_ptr).py as libc::c_int,
                      (*p_ptr).px as libc::c_int);
            msg_print(b"You cook some food.\x00" as *const u8 as
                          *const libc::c_char);
        }
        35 => {
            msg_print(b"You play tough.\x00" as *const u8 as
                          *const libc::c_char);
            set_afraid(0 as libc::c_int);
        }
        16 => {
            msg_print(b"RAAAGH!\x00" as *const u8 as *const libc::c_char);
            set_afraid(0 as libc::c_int);
            set_shero((*p_ptr).shero as libc::c_int + 10 as libc::c_int +
                          (Rand_div(plev as s32b) + 1 as libc::c_int));
            hp_player(30 as libc::c_int);
        }
        36 => {
            msg_print(b"You carefully set an explosive rune...\x00" as
                          *const u8 as *const libc::c_char);
            explosive_rune();
        }
        37 => {
            if !(get_aim_dir(&mut dir) == 0) {
                msg_print(b"You bash at a stone wall.\x00" as *const u8 as
                              *const libc::c_char);
                wall_to_mud(dir);
            }
        }
        49 => {
            loop 
                 /* Select power to use */
                 {
                if get_com(b"Use [F]lash aura or [L]ight speed jump? \x00" as
                               *const u8 as *const libc::c_char, &mut ch) == 0
                   {
                    amber_power = 0 as libc::c_int;
                    break ;
                } else if ch as libc::c_int == 'F' as i32 ||
                              ch as libc::c_int == 'f' as i32 {
                    amber_power = 1 as libc::c_int;
                    break ;
                } else {
                    if !(ch as libc::c_int == 'L' as i32 ||
                             ch as libc::c_int == 'l' as i32) {
                        continue ;
                    }
                    amber_power = 2 as libc::c_int;
                    break ;
                }
            }
            if amber_power == 1 as libc::c_int {
                x_ptr_foo.level = 1 as libc::c_int as byte_hack;
                x_ptr_foo.cost = 9 as libc::c_int as byte_hack;
                x_ptr_foo.stat = 5 as libc::c_int as byte_hack;
                x_ptr_foo.diff = 7 as libc::c_int as byte_hack;
                if power_chance(&mut x_ptr_foo) != 0 {
                    if get_aim_dir(&mut dir) == 0 {
                        current_block_502 = 4919164432616328491;
                    } else {
                        msg_print(b"You flash a bright aura.\x00" as *const u8
                                      as *const libc::c_char);
                        if ((*p_ptr).lev as libc::c_int) < 10 as libc::c_int {
                            fire_bolt(22 as libc::c_int, dir,
                                      plev as libc::c_int * 2 as libc::c_int);
                        } else {
                            fire_ball(22 as libc::c_int, dir,
                                      plev as libc::c_int * 2 as libc::c_int,
                                      2 as libc::c_int);
                        }
                        current_block_502 = 9952640327414195044;
                    }
                } else { current_block_502 = 9952640327414195044; }
            } else { current_block_502 = 9952640327414195044; }
            match current_block_502 {
                4919164432616328491 => { }
                _ => {
                    if amber_power == 2 as libc::c_int {
                        x_ptr_foo.level = 30 as libc::c_int as byte_hack;
                        x_ptr_foo.cost = 30 as libc::c_int as byte_hack;
                        x_ptr_foo.stat = 2 as libc::c_int as byte_hack;
                        x_ptr_foo.diff = 7 as libc::c_int as byte_hack;
                        if power_chance(&mut x_ptr_foo) != 0 {
                            set_light_speed((*p_ptr).lightspeed as libc::c_int
                                                + 3 as libc::c_int);
                        }
                    }
                }
            }
        }
        38 => {
            if !(get_aim_dir(&mut dir) == 0) {
                msg_print(b"You throw a dart of poison.\x00" as *const u8 as
                              *const libc::c_char);
                fire_bolt(2 as libc::c_int, dir, plev as libc::c_int);
            }
        }
        33 => {
            msg_print(b"You examine your surroundings.\x00" as *const u8 as
                          *const libc::c_char);
            detect_traps(25 as libc::c_int);
            detect_doors(25 as libc::c_int);
            detect_stairs(25 as libc::c_int);
        }
        39 => {
            if !(get_aim_dir(&mut dir) == 0) {
                msg_print(b"You cast a magic missile.\x00" as *const u8 as
                              *const libc::c_char);
                fire_bolt_or_beam(10 as libc::c_int, 10 as libc::c_int, dir,
                                  damroll((3 as libc::c_int +
                                               (plev as libc::c_int -
                                                    1 as libc::c_int) /
                                                   5 as libc::c_int) as s16b,
                                          4 as libc::c_int as s16b));
            }
        }
        50 => {
            loop 
                 /* Select power to use */
                 {
                if get_com(b"Use [T]hunder strike, [R]ide the straight road, go [B]ack in town? \x00"
                               as *const u8 as *const libc::c_char, &mut ch)
                       == 0 {
                    amber_power = 0 as libc::c_int;
                    break ;
                } else if ch as libc::c_int == 'T' as i32 ||
                              ch as libc::c_int == 't' as i32 {
                    amber_power = 1 as libc::c_int;
                    break ;
                } else if ch as libc::c_int == 'R' as i32 ||
                              ch as libc::c_int == 'r' as i32 {
                    amber_power = 2 as libc::c_int;
                    break ;
                } else {
                    if !(ch as libc::c_int == 'B' as i32 ||
                             ch as libc::c_int == 'b' as i32) {
                        continue ;
                    }
                    amber_power = 3 as libc::c_int;
                    break ;
                }
            }
            if amber_power == 1 as libc::c_int {
                x_ptr_foo.level = 1 as libc::c_int as byte_hack;
                x_ptr_foo.cost = (*p_ptr).lev as byte_hack;
                x_ptr_foo.stat = 4 as libc::c_int as byte_hack;
                x_ptr_foo.diff = 6 as libc::c_int as byte_hack;
                if power_chance(&mut x_ptr_foo) != 0 {
                    if get_aim_dir(&mut dir) == 0 {
                        current_block_502 = 4919164432616328491;
                    } else {
                        msg_format(b"You conjure up thunder!\x00" as *const u8
                                       as *const libc::c_char);
                        fire_beam(1 as libc::c_int, dir,
                                  (*p_ptr).lev as libc::c_int *
                                      2 as libc::c_int);
                        fire_beam(21 as libc::c_int, dir,
                                  (*p_ptr).lev as libc::c_int *
                                      2 as libc::c_int);
                        (*p_ptr).energy -= 100 as libc::c_int;
                        current_block_502 = 16304063406973696512;
                    }
                } else { current_block_502 = 16304063406973696512; }
            } else { current_block_502 = 16304063406973696512; }
            match current_block_502 {
                4919164432616328491 => { }
                _ => {
                    if amber_power == 2 as libc::c_int {
                        if dungeon_flags2 as libc::c_long &
                               0x8 as libc::c_long != 0 {
                            msg_print(b"No teleport on special levels ...\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            current_block_502 = 4919164432616328491;
                        } else {
                            x_ptr_foo.level = 3 as libc::c_int as byte_hack;
                            x_ptr_foo.cost = 15 as libc::c_int as byte_hack;
                            x_ptr_foo.stat = 4 as libc::c_int as byte_hack;
                            x_ptr_foo.diff = 6 as libc::c_int as byte_hack;
                            if power_chance(&mut x_ptr_foo) != 0 {
                                msg_print(b"You enter the straight road and fly beside the world. Where to exit?\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                if tgt_pt(&mut ii, &mut ij) == 0 { return }
                                (*p_ptr).energy -=
                                    60 as libc::c_int - plev as libc::c_int;
                                if !((*f_info.offset((*cave[ij as
                                                                usize].offset(ii
                                                                                  as
                                                                                  isize)).feat
                                                         as isize)).flags1 as
                                         libc::c_long & 0x10 as libc::c_long
                                         != 0 &&
                                         (*cave[ij as
                                                    usize].offset(ii as
                                                                      isize)).feat
                                             as libc::c_int !=
                                             0xaf as libc::c_int &&
                                         (*cave[ij as
                                                    usize].offset(ii as
                                                                      isize)).m_idx
                                             == 0 &&
                                         !(ij == (*p_ptr).py as libc::c_int &&
                                               ii ==
                                                   (*p_ptr).px as
                                                       libc::c_int)) ||
                                       (*cave[ij as
                                                  usize].offset(ii as
                                                                    isize)).info
                                           as libc::c_int & 0x4 as libc::c_int
                                           != 0 ||
                                       distance(ij, ii,
                                                (*p_ptr).py as libc::c_int,
                                                (*p_ptr).px as libc::c_int) >
                                           plev as libc::c_int *
                                               20 as libc::c_int +
                                               2 as libc::c_int ||
                                       (*cave[ij as
                                                  usize].offset(ii as
                                                                    isize)).info
                                           as libc::c_int & 0x1 as libc::c_int
                                           == 0 {
                                    msg_print(b"You fail to exit correctly!\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                    (*p_ptr).energy -= 100 as libc::c_int;
                                    teleport_player(10 as libc::c_int);
                                } else { teleport_player_to(ij, ii); }
                            }
                            current_block_502 = 10029375464402185584;
                        }
                    } else { current_block_502 = 10029375464402185584; }
                    match current_block_502 {
                        4919164432616328491 => { }
                        _ => {
                            if amber_power == 3 as libc::c_int {
                                if dungeon_flags2 as libc::c_long &
                                       0x8 as libc::c_long != 0 {
                                    msg_print(b"No recall on special levels..\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                } else {
                                    x_ptr_foo.level =
                                        7 as libc::c_int as byte_hack;
                                    x_ptr_foo.cost =
                                        30 as libc::c_int as byte_hack;
                                    x_ptr_foo.stat =
                                        4 as libc::c_int as byte_hack;
                                    x_ptr_foo.diff =
                                        6 as libc::c_int as byte_hack;
                                    if power_chance(&mut x_ptr_foo) != 0 {
                                        if dun_level as libc::c_int ==
                                               0 as libc::c_int {
                                            msg_print(b"You are already in town!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                        } else {
                                            msg_print(b"You enter the straight road and fly beside the world.\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            (*p_ptr).energy -=
                                                100 as libc::c_int;
                                            (*p_ptr).word_recall =
                                                1 as libc::c_int as s16b
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        40 => {
            msg_print(b"You make the trees grow!\x00" as *const u8 as
                          *const libc::c_char);
            grow_trees(if (plev as libc::c_int / 8 as libc::c_int) <
                              1 as libc::c_int {
                           1 as libc::c_int
                       } else { (plev as libc::c_int) / 8 as libc::c_int });
        }
        51 => { do_cmd_immovable_special(); }
        41 => {
            msg_print(b"You breathe cold...\x00" as *const u8 as
                          *const libc::c_char);
            if get_aim_dir(&mut dir) != 0 {
                fire_ball(4 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 2 as libc::c_int,
                          1 as libc::c_int +
                              (*p_ptr).lev as libc::c_int /
                                  20 as libc::c_int);
            }
        }
        42 => {
            msg_print(b"You breathe chaos...\x00" as *const u8 as
                          *const libc::c_char);
            if get_aim_dir(&mut dir) != 0 {
                fire_ball(30 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 2 as libc::c_int,
                          1 as libc::c_int +
                              (*p_ptr).lev as libc::c_int /
                                  20 as libc::c_int);
            }
        }
        43 => {
            if !(get_aim_dir(&mut dir) == 0) {
                msg_format(b"You breathe the elements.\x00" as *const u8 as
                               *const libc::c_char);
                fire_ball(10 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 2 as libc::c_int,
                          (*p_ptr).lev as libc::c_int / 15 as libc::c_int +
                              1 as libc::c_int);
            }
        }
        47 => { do_cmd_beastmaster(); }
        44 => {
            msg_print(b"The power of Eru Iluvatar flows through you!\x00" as
                          *const u8 as *const libc::c_char);
            msg_print(b"The world changes!\x00" as *const u8 as
                          *const libc::c_char);
            autosave_checkpoint();
            /* Leaving */
            (*p_ptr).leaving = 1 as libc::c_int as bool_
        }
        7 => {
            /* Only works on adjacent monsters */
            if !(get_rep_dir(&mut dir) == 0) {
                y =
                    (*p_ptr).py as libc::c_int +
                        ddy[dir as usize] as
                            libc::c_int; /* was get_aim_dir */
                x =
                    (*p_ptr).px as libc::c_int +
                        ddx[dir as usize] as libc::c_int; /* Dmg */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(y as
                                                         isize)).offset(x as
                                                                            isize)
                        as *mut cave_type;
                if (*c_ptr).m_idx == 0 {
                    msg_print(b"You bite into thin air!\x00" as *const u8 as
                                  *const libc::c_char);
                } else {
                    msg_print(b"You grin and bare your fangs...\x00" as
                                  *const u8 as *const libc::c_char);
                    dummy =
                        plev as libc::c_int +
                            (Rand_div(plev as s32b) + 1 as libc::c_int) *
                                (if (1 as libc::c_int) <
                                        plev as libc::c_int /
                                            10 as libc::c_int {
                                     (plev as libc::c_int) / 10 as libc::c_int
                                 } else { 1 as libc::c_int });
                    if drain_life(dir, dummy) != 0 {
                        if ((*p_ptr).food as libc::c_int) <
                               10000 as libc::c_int {
                            /* No heal if we are "full" */
                            hp_player(dummy);
                        } else {
                            msg_print(b"You were not hungry.\x00" as *const u8
                                          as *const libc::c_char);
                        }
                        /* Gain nutritional sustenance: 150/hp drained */
				/* A Food ration gives 5000 food points (by contrast) */
				/* Don't ever get more than "Full" this way */
				/* But if we ARE Gorged,  it won't cure us */
                        dummy =
                            (*p_ptr).food as libc::c_int +
                                (if 5000 as libc::c_int >
                                        100 as libc::c_int * dummy {
                                     (100 as libc::c_int) * dummy
                                 } else { 5000 as libc::c_int });
                        if ((*p_ptr).food as libc::c_int) <
                               15000 as libc::c_int {
                            /* Not gorged already */
                            set_food(if dummy >= 15000 as libc::c_int {
                                         (15000 as libc::c_int) -
                                             1 as libc::c_int
                                     } else { dummy });
                        }
                    } else {
                        msg_print(b"Yechh. That tastes foul.\x00" as *const u8
                                      as *const libc::c_char);
                    }
                }
            }
        }
        45 => {
            msg_print(b"You emit an eldritch howl!\x00" as *const u8 as
                          *const libc::c_char);
            if !(get_aim_dir(&mut dir) == 0) {
                fear_monster(dir, plev as libc::c_int);
            }
        }
        46 => {
            msg_print(b"You attempt to restore your lost energies.\x00" as
                          *const u8 as *const libc::c_char);
            restore_level();
        }
        52 => {
            let mut dir_0: libc::c_int = 0;
            let mut x_0: libc::c_int = 0;
            let mut y_0: libc::c_int = 0;
            let mut c_ptr_0: *mut cave_type = 0 as *mut cave_type;
            let mut m_ptr_0: *mut monster_type = 0 as *mut monster_type;
            let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
            let mut q_ptr_0: *mut object_type = 0 as *mut object_type;
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
            msg_print(b"Hypnotize which pet?\x00" as *const u8 as
                          *const libc::c_char);
            if get_rep_dir(&mut dir_0) == 0 { return }
            y_0 =
                (*p_ptr).py as libc::c_int +
                    ddy[dir_0 as usize] as libc::c_int;
            x_0 =
                (*p_ptr).px as libc::c_int +
                    ddx[dir_0 as usize] as libc::c_int;
            c_ptr_0 =
                &mut *(*cave.as_mut_ptr().offset(y_0 as
                                                     isize)).offset(x_0 as
                                                                        isize)
                    as *mut cave_type;
            if (*c_ptr_0).m_idx != 0 {
                m_ptr_0 =
                    &mut *m_list.offset((*c_ptr_0).m_idx as isize) as
                        *mut monster_type;
                r_ptr =
                    if !(*m_ptr_0).sr_ptr.is_null() {
                        (*m_ptr_0).sr_ptr
                    } else {
                        race_info_idx((*m_ptr_0).r_idx as libc::c_int,
                                      (*m_ptr_0).ego as libc::c_int)
                    };
                if (*r_ptr).flags1 & 0x20000 as libc::c_int as libc::c_uint !=
                       0 &&
                       (*m_ptr_0).status as libc::c_int == 3 as libc::c_int &&
                       (*r_ptr).flags9 & 0x2000 as libc::c_int as libc::c_uint
                           == 0 {
                    q_ptr_0 = &mut forge_0;
                    object_prep(q_ptr_0,
                                lookup_kind(99 as libc::c_int,
                                            1 as libc::c_int) as libc::c_int);
                    (*q_ptr_0).number = 1 as libc::c_int as byte_hack;
                    (*q_ptr_0).pval = (*m_ptr_0).r_idx as s32b;
                    (*q_ptr_0).pval2 = (*m_ptr_0).hp as s16b;
                    object_aware(q_ptr_0);
                    object_known(q_ptr_0);
                    (*q_ptr_0).ident =
                        ((*q_ptr_0).ident as libc::c_int |
                             0x10 as libc::c_int) as byte_hack;
                    drop_near(q_ptr_0, 0 as libc::c_int, y_0, x_0);
                    delete_monster(y_0, x_0);
                    health_who = 0 as libc::c_int as s16b
                } else {
                    msg_print(b"You can only hypnotize monsters that can\'t move.\x00"
                                  as *const u8 as *const libc::c_char);
                }
            } else {
                msg_print(b"There is no pet here!\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        53 => {
            let mut m_ptr_1: *mut monster_type = 0 as *mut monster_type;
            let mut m_idx: libc::c_int = 0;
            let mut item_0: libc::c_int = 0;
            let mut x_1: libc::c_int = 0;
            let mut y_1: libc::c_int = 0;
            let mut d: libc::c_int = 0;
            let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
            let mut q_1: cptr = 0 as *const libc::c_char;
            let mut s_1: cptr = 0 as *const libc::c_char;
            /* Restrict choices to monsters */
            item_tester_tval = 99 as libc::c_int as byte_hack;
            /* Get an item */
            q_1 =
                b"Awaken which monster? \x00" as *const u8 as
                    *const libc::c_char;
            s_1 =
                b"You have no monster to awaken.\x00" as *const u8 as
                    *const libc::c_char;
            if get_item(&mut item_0, q_1, s_1, 0x4 as libc::c_int) == 0 {
                return
            }
            o_ptr_0 =
                &mut *o_list.offset((0 as libc::c_int - item_0) as isize) as
                    *mut object_type;
            d = 2 as libc::c_int;
            while d < 100 as libc::c_int {
                scatter(&mut y_1, &mut x_1, (*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, d);
                if (*f_info.offset((*cave[y_1 as
                                              usize].offset(x_1 as
                                                                isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[y_1 as usize].offset(x_1 as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int &&
                       (*cave[y_1 as usize].offset(x_1 as isize)).m_idx == 0 {
                    break ;
                }
                d += 1
            }
            if d >= 100 as libc::c_int { return }
            m_idx =
                place_monster_one(y_1, x_1, (*o_ptr_0).pval, 0 as libc::c_int,
                                  0 as libc::c_int as bool_, 3 as libc::c_int)
                    as libc::c_int;
            if m_idx == 0 as libc::c_int { return }
            m_ptr_1 =
                &mut *m_list.offset(m_idx as isize) as *mut monster_type;
            (*m_ptr_1).hp = (*o_ptr_0).pval2 as s32b;
            floor_item_increase(0 as libc::c_int - item_0,
                                -(1 as libc::c_int));
            floor_item_describe(0 as libc::c_int - item_0);
            floor_item_optimize(0 as libc::c_int - item_0);
        }
        48 => { do_cmd_necromancer(); }
        54 => { do_cmd_integrate_body(); }
        0 => {
            msg_print(b"You spit acid...\x00" as *const u8 as
                          *const libc::c_char);
            if get_aim_dir(&mut dir) != 0 {
                fire_ball(3 as libc::c_int, dir, (*p_ptr).lev as libc::c_int,
                          1 as libc::c_int +
                              (*p_ptr).lev as libc::c_int /
                                  30 as libc::c_int);
            }
        }
        1 => {
            msg_print(b"You breathe fire...\x00" as *const u8 as
                          *const libc::c_char);
            if get_aim_dir(&mut dir) != 0 {
                fire_ball(5 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 2 as libc::c_int,
                          1 as libc::c_int +
                              (*p_ptr).lev as libc::c_int /
                                  20 as libc::c_int);
            }
        }
        2 => {
            msg_print(b"Your eyes look mesmerising...\x00" as *const u8 as
                          *const libc::c_char);
            if get_aim_dir(&mut dir) != 0 {
                charm_monster(dir, (*p_ptr).lev as libc::c_int);
            }
        }
        3 => {
            msg_print(b"You concentrate...\x00" as *const u8 as
                          *const libc::c_char);
            if get_aim_dir(&mut dir) != 0 {
                fetch(dir, (*p_ptr).lev as libc::c_int * 10 as libc::c_int,
                      1 as libc::c_int as bool_);
            }
        }
        4 => {
            msg_print(b"You concentrate...\x00" as *const u8 as
                          *const libc::c_char);
            teleport_player(10 as libc::c_int +
                                4 as libc::c_int *
                                    (*p_ptr).lev as libc::c_int);
        }
        5 => {
            msg_print(b"You concentrate...\x00" as *const u8 as
                          *const libc::c_char);
            if get_aim_dir(&mut dir) == 0 { return }
            fire_bolt(85 as libc::c_int, dir,
                      damroll((3 as libc::c_int +
                                   ((*p_ptr).lev as libc::c_int -
                                        1 as libc::c_int) / 5 as libc::c_int)
                                  as s16b, 3 as libc::c_int as s16b));
        }
        6 => {
            msg_print(b"Radiation flows from your body!\x00" as *const u8 as
                          *const libc::c_char);
            fire_ball(73 as libc::c_int, 0 as libc::c_int,
                      (*p_ptr).lev as libc::c_int * 2 as libc::c_int,
                      3 as libc::c_int +
                          (*p_ptr).lev as libc::c_int / 20 as libc::c_int);
        }
        8 => { detect_treasure(25 as libc::c_int); }
        9 => { detect_monsters_normal(25 as libc::c_int); }
        10 => { teleport_player(10 as libc::c_int); }
        11 => {
            let mut x_2: libc::c_int = 0;
            let mut y_2: libc::c_int = 0;
            let mut ox: libc::c_int = 0;
            let mut oy: libc::c_int = 0;
            let mut c_ptr_1: *mut cave_type = 0 as *mut cave_type;
            if !(get_rep_dir(&mut dir) == 0) {
                y_2 =
                    (*p_ptr).py as libc::c_int +
                        ddy[dir as usize] as libc::c_int;
                x_2 =
                    (*p_ptr).px as libc::c_int +
                        ddx[dir as usize] as libc::c_int;
                c_ptr_1 =
                    &mut *(*cave.as_mut_ptr().offset(y_2 as
                                                         isize)).offset(x_2 as
                                                                            isize)
                        as *mut cave_type;
                if (*f_info.offset((*cave[y_2 as
                                              usize].offset(x_2 as
                                                                isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[y_2 as usize].offset(x_2 as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int {
                    msg_print(b"You bite into thin air!\x00" as *const u8 as
                                  *const libc::c_char);
                } else if (*f_info.offset((*c_ptr_1).feat as isize)).flags1 as
                              libc::c_long & 0x40 as libc::c_long != 0 ||
                              (*c_ptr_1).feat as libc::c_int ==
                                  0x61 as libc::c_int {
                    msg_print(b"Ouch!  This wall is harder than your teeth!\x00"
                                  as *const u8 as *const libc::c_char);
                } else if (*c_ptr_1).m_idx != 0 {
                    msg_print(b"There\'s something in the way!\x00" as
                                  *const u8 as *const libc::c_char);
                } else if (*c_ptr_1).feat as libc::c_int ==
                              0x60 as libc::c_int {
                    msg_print(b"You don\'t like the woody taste!\x00" as
                                  *const u8 as *const libc::c_char);
                } else {
                    if (*c_ptr_1).feat as libc::c_int >= 0x20 as libc::c_int
                           &&
                           (*c_ptr_1).feat as libc::c_int <=
                               0x31 as libc::c_int {
                        set_food((*p_ptr).food as libc::c_int +
                                     3000 as libc::c_int);
                    } else if (*c_ptr_1).feat as libc::c_int >=
                                  0x32 as libc::c_int &&
                                  (*c_ptr_1).feat as libc::c_int <=
                                      0x37 as libc::c_int {
                        set_food((*p_ptr).food as libc::c_int +
                                     5000 as libc::c_int);
                    } else if (*c_ptr_1).feat as libc::c_int >=
                                  0x62 as libc::c_int &&
                                  (*c_ptr_1).feat as libc::c_int <=
                                      0x64 as libc::c_int {
                        set_food((*p_ptr).food as libc::c_int +
                                     500 as libc::c_int);
                    } else {
                        msg_print(b"This granite is very filling!\x00" as
                                      *const u8 as *const libc::c_char);
                        set_food((*p_ptr).food as libc::c_int +
                                     10000 as libc::c_int);
                    }
                    wall_to_mud(dir);
                    oy = (*p_ptr).py as libc::c_int;
                    ox = (*p_ptr).px as libc::c_int;
                    (*p_ptr).py = y_2 as s16b;
                    (*p_ptr).px = x_2 as s16b;
                    lite_spot((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int);
                    lite_spot(oy, ox);
                    verify_panel();
                    (*p_ptr).update =
                        ((*p_ptr).update as libc::c_long |
                             (0x100000 as libc::c_long |
                                  0x10000000 as libc::c_long |
                                  0x200000 as libc::c_long)) as u32b;
                    (*p_ptr).update =
                        ((*p_ptr).update as libc::c_long |
                             0x2000000 as libc::c_long) as u32b;
                    (*p_ptr).window =
                        ((*p_ptr).window as libc::c_long |
                             0x80 as libc::c_long) as u32b
                }
            }
        }
        12 => { if get_aim_dir(&mut dir) == 0 { return } teleport_swap(dir); }
        13 => {
            fire_ball(21 as libc::c_int, 0 as libc::c_int,
                      4 as libc::c_int * (*p_ptr).lev as libc::c_int,
                      8 as libc::c_int);
            aggravate_monsters(0 as libc::c_int);
        }
        14 => {
            lite_area(damroll(2 as libc::c_int as s16b,
                              ((*p_ptr).lev as libc::c_int / 2 as libc::c_int)
                                  as s16b),
                      (*p_ptr).lev as libc::c_int / 10 as libc::c_int +
                          1 as libc::c_int);
        }
        15 => {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < 52 as libc::c_int {
                let mut o_ptr_1: *mut object_type =
                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize)
                        as *mut object_type;
                if !((*o_ptr_1).k_idx == 0) {
                    if !((*o_ptr_1).ident as libc::c_int & 0x40 as libc::c_int
                             == 0) {
                        if (*o_ptr_1).sense == 0 {
                            (*o_ptr_1).sense = 1 as libc::c_int as byte_hack
                        }
                    }
                }
                i += 1
            }
        }
        17 => { do_poly_self(); }
        18 => { alchemy(); }
        19 => {
            let mut i_0: libc::c_int = 0;
            i_0 = 0 as libc::c_int;
            while i_0 < 8 as libc::c_int {
                summon_specific_friendly((*p_ptr).py as libc::c_int,
                                         (*p_ptr).px as libc::c_int,
                                         (*p_ptr).lev as libc::c_int,
                                         33 as libc::c_int,
                                         0 as libc::c_int as bool_);
                i_0 += 1
            }
        }
        20 => {
            let mut num: libc::c_int =
                (*p_ptr).lev as libc::c_int / 10 as libc::c_int;
            let mut dur: libc::c_int =
                Rand_div(20 as libc::c_int) + 1 as libc::c_int +
                    20 as libc::c_int;
            if Rand_div(5 as libc::c_int) < num {
                set_oppose_acid((*p_ptr).oppose_acid as libc::c_int + dur);
                num -= 1
            }
            if Rand_div(4 as libc::c_int) < num {
                set_oppose_elec((*p_ptr).oppose_elec as libc::c_int + dur);
                num -= 1
            }
            if Rand_div(3 as libc::c_int) < num {
                set_oppose_fire((*p_ptr).oppose_fire as libc::c_int + dur);
                num -= 1
            }
            if Rand_div(2 as libc::c_int) < num {
                set_oppose_cold((*p_ptr).oppose_cold as libc::c_int + dur);
                num -= 1
            }
            if num != 0 {
                set_oppose_pois((*p_ptr).oppose_pois as libc::c_int + dur);
                num -= 1
            }
        }
        21 => {
            /* Prevent destruction of quest levels and town */
            if is_quest(dun_level as libc::c_int) == 0 &&
                   dun_level as libc::c_int != 0 {
                earthquake((*p_ptr).py as libc::c_int,
                           (*p_ptr).px as libc::c_int, 10 as libc::c_int);
            }
        }
        22 => {
            let mut o_ptr_2: *mut object_type = 0 as *mut object_type;
            let mut lev: libc::c_int = 0;
            let mut item_1: libc::c_int = 0;
            item_tester_hook =
                Some(item_tester_hook_recharge as
                         unsafe extern "C" fn(_: *mut object_type) -> bool_);
            /* Get an item */
            q = b"Drain which item? \x00" as *const u8 as *const libc::c_char;
            s =
                b"You have nothing to drain.\x00" as *const u8 as
                    *const libc::c_char;
            if !(get_item(&mut item_1, q, s,
                          0x2 as libc::c_int | 0x4 as libc::c_int) == 0) {
                o_ptr_2 = get_object(item_1);
                lev =
                    (*k_info.offset((*o_ptr_2).k_idx as isize)).level as
                        libc::c_int;
                if (*o_ptr_2).tval as libc::c_int == 67 as libc::c_int {
                    if (*o_ptr_2).timeout == 0 {
                        msg_print(b"You can\'t absorb energy from a discharged rod.\x00"
                                      as *const u8 as *const libc::c_char);
                    } else {
                        (*p_ptr).csp =
                            ((*p_ptr).csp as libc::c_int +
                                 (*o_ptr_2).timeout as libc::c_int) as s16b;
                        (*o_ptr_2).timeout = 0 as libc::c_int as s16b
                    }
                } else {
                    if (*o_ptr_2).pval > 0 as libc::c_int {
                        (*p_ptr).csp =
                            ((*p_ptr).csp as libc::c_int +
                                 (*o_ptr_2).pval * lev) as s16b;
                        (*o_ptr_2).pval = 0 as libc::c_int
                    } else {
                        msg_print(b"There\'s no energy there to absorb!\x00"
                                      as *const u8 as *const libc::c_char);
                    }
                    (*o_ptr_2).ident =
                        ((*o_ptr_2).ident as libc::c_int | 0x4 as libc::c_int)
                            as byte_hack
                }
                if (*p_ptr).csp as libc::c_int > (*p_ptr).msp as libc::c_int {
                    (*p_ptr).csp = (*p_ptr).msp
                }
                (*p_ptr).notice =
                    ((*p_ptr).notice as libc::c_long |
                         (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long | 0x1 as libc::c_long) as
                        u32b
            }
        }
        23 => { report_magics(); }
        24 => {
            /* Fake a population explosion. */
            msg_print(b"You suddenly have a headache!\x00" as *const u8 as
                          *const libc::c_char);
            take_hit(Rand_div(30 as libc::c_int) + 1 as libc::c_int +
                         30 as libc::c_int,
                     b"the strain of forcing abstinence\x00" as *const u8 as
                         *const libc::c_char);
            num_repro =
                (num_repro as libc::c_int + 100 as libc::c_int) as s16b
        }
        25 => {
            let mut x_3: libc::c_int = 0;
            let mut y_3: libc::c_int = 0;
            if get_rep_dir(&mut dir) == 0 { return }
            y_3 =
                (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
            x_3 =
                (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
            if (*cave[y_3 as usize].offset(x_3 as isize)).m_idx != 0 {
                py_attack(y_3, x_3, -(1 as libc::c_int));
                teleport_player(30 as libc::c_int);
            } else {
                msg_print(b"You don\'t see any monster in this direction.\x00"
                              as *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
            }
        }
        26 => {
            stun_monsters((*p_ptr).lev as libc::c_int * 4 as libc::c_int);
            confuse_monsters((*p_ptr).lev as libc::c_int * 4 as libc::c_int);
            turn_monsters((*p_ptr).lev as libc::c_int * 4 as libc::c_int);
        }
        27 => {
            if get_aim_dir(&mut dir) == 0 { return }
            fire_beam(15 as libc::c_int, dir,
                      2 as libc::c_int * (*p_ptr).lev as libc::c_int);
        }
        28 => {
            if dungeon_flags2 as libc::c_long & 0x10 as libc::c_long == 0 ||
                   dungeon_flags2 as libc::c_long & 0x10 as libc::c_long != 0
                       &&
                       get_check(b"Leave this unique level forever? \x00" as
                                     *const u8 as *const libc::c_char) == 0 {
                recall_player(21 as libc::c_int, 15 as libc::c_int);
            }
        }
        29 => {
            let mut x_4: libc::c_int = 0;
            let mut y_4: libc::c_int = 0;
            let mut c_ptr_2: *mut cave_type = 0 as *mut cave_type;
            let mut m_ptr_2: *mut monster_type = 0 as *mut monster_type;
            let mut r_ptr_0: *mut monster_race = 0 as *mut monster_race;
            if get_rep_dir(&mut dir) == 0 { return }
            y_4 =
                (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
            x_4 =
                (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
            c_ptr_2 =
                &mut *(*cave.as_mut_ptr().offset(y_4 as
                                                     isize)).offset(x_4 as
                                                                        isize)
                    as *mut cave_type;
            if (*c_ptr_2).m_idx == 0 {
                msg_print(b"You sense no evil there!\x00" as *const u8 as
                              *const libc::c_char);
            } else {
                m_ptr_2 =
                    &mut *m_list.offset((*c_ptr_2).m_idx as isize) as
                        *mut monster_type;
                r_ptr_0 =
                    if !(*m_ptr_2).sr_ptr.is_null() {
                        (*m_ptr_2).sr_ptr
                    } else {
                        race_info_idx((*m_ptr_2).r_idx as libc::c_int,
                                      (*m_ptr_2).ego as libc::c_int)
                    };
                if (*r_ptr_0).flags3 & 0x40 as libc::c_int as libc::c_uint !=
                       0 {
                    /* Delete the monster, rather than killing it. */
                    delete_monster_idx((*c_ptr_2).m_idx as libc::c_int);
                    msg_print(b"The evil creature vanishes in a puff of sulphurous smoke!\x00"
                                  as *const u8 as *const libc::c_char);
                } else {
                    msg_print(b"Your invocation is ineffectual!\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
        }
        30 => {
            let mut x_5: libc::c_int = 0;
            let mut y_5: libc::c_int = 0;
            let mut c_ptr_3: *mut cave_type = 0 as *mut cave_type;
            if get_rep_dir(&mut dir) == 0 { return }
            y_5 =
                (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
            x_5 =
                (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
            c_ptr_3 =
                &mut *(*cave.as_mut_ptr().offset(y_5 as
                                                     isize)).offset(x_5 as
                                                                        isize)
                    as *mut cave_type;
            if (*c_ptr_3).m_idx == 0 {
                msg_print(b"You wave your hands in the air.\x00" as *const u8
                              as *const libc::c_char);
            } else {
                fire_bolt(4 as libc::c_int, dir,
                          2 as libc::c_int * (*p_ptr).lev as libc::c_int);
            }
        }
        31 => {
            /* Gives a multiplier of 2 at first, up to 5 at 48th */
            (*p_ptr).throw_mult =
                (2 as libc::c_int +
                     (*p_ptr).lev as libc::c_int / 16 as libc::c_int) as
                    byte_hack;
            do_cmd_throw();
            (*p_ptr).throw_mult = 1 as libc::c_int as byte_hack
        }
        60 => { use_ability_blade(); }
        _ => {
            if process_hooks(38 as libc::c_int,
                             b"(d)\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, power) == 0 {
                msg_format(b"Warning power_activate() called with invalid power(%d).\x00"
                               as *const u8 as *const libc::c_char, power);
                energy_use = 0 as libc::c_int
            }
        }
    }
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x40 as libc::c_long | 0x80 as libc::c_long)) as u32b;
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
}
/*
 * Print a batch of power.
 */
unsafe extern "C" fn print_power_batch(mut p: *mut libc::c_int,
                                       mut start: libc::c_int,
                                       mut max: libc::c_int,
                                       mut mode: bool_) {
    let mut buff: [libc::c_char; 80] = [0; 80];
    let mut spell: *mut power_type = 0 as *mut power_type;
    let mut i: libc::c_int = start;
    let mut j: libc::c_int = 0 as libc::c_int;
    if mode != 0 {
        prt(format(b"         %-31s Level Mana Fail\x00" as *const u8 as
                       *const libc::c_char,
                   b"Name\x00" as *const u8 as *const libc::c_char) as cptr,
            1 as libc::c_int, 20 as libc::c_int);
    }
    i = start;
    while i < start + 20 as libc::c_int {
        if i >= max { break ; }
        spell =
            &mut *powers_type.offset(*p.offset(i as isize) as isize) as
                *mut power_type;
        sprintf(buff.as_mut_ptr(),
                b"  %c-%3d) %-30s  %5d %4d %s@%d\x00" as *const u8 as
                    *const libc::c_char, j + 'a' as i32,
                *p.offset(i as isize) + 1 as libc::c_int, (*spell).name,
                (*spell).level as libc::c_int, (*spell).cost as libc::c_int,
                stat_names[(*spell).stat as usize],
                (*spell).diff as libc::c_int);
        if mode != 0 {
            prt(buff.as_mut_ptr() as cptr, 2 as libc::c_int + j,
                20 as libc::c_int);
        }
        j += 1;
        i += 1
    }
    if mode != 0 {
        prt(b"\x00" as *const u8 as *const libc::c_char, 2 as libc::c_int + j,
            20 as libc::c_int);
    }
    prt(format(b"Select a power (a-%c), +/- to scroll:\x00" as *const u8 as
                   *const libc::c_char, j - 1 as libc::c_int + 'a' as i32) as
            cptr, 0 as libc::c_int, 0 as libc::c_int);
}
/*
 * List powers and ask to pick one. 
 */
unsafe extern "C" fn select_power(mut x_idx: *mut libc::c_int)
 -> *mut power_type {
    let mut which: libc::c_char = 0;
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut ret: *mut power_type = 0 as *mut power_type;
    let mut mode: bool_ = 0 as libc::c_int as bool_;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    p =
        memset(ralloc((power_max as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (power_max as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    /* Count the max */
    i = 0 as libc::c_int;
    while i < power_max as libc::c_int {
        if *(*p_ptr).powers.offset(i as isize) != 0 {
            let fresh0 = max;
            max = max + 1;
            *p.offset(fresh0 as isize) = i
        }
        i += 1
    }
    /* Exit if there aren't powers */
    if max == 0 as libc::c_int {
        *x_idx = -(1 as libc::c_int);
        ret = 0 as *mut power_type;
        msg_print(b"You don\'t have any special powers.\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        character_icky = 1 as libc::c_int as bool_;
        Term_save();
        loop  {
            print_power_batch(p, start, max, mode);
            which = inkey();
            if which as libc::c_int == '\u{1b}' as i32 {
                *x_idx = -(1 as libc::c_int);
                ret = 0 as *mut power_type;
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
            } else {
                which = tolower(which as libc::c_int) as libc::c_char;
                if start + (which as libc::c_int - 'a' as i32) >= max {
                    bell();
                } else if (start + (which as libc::c_int - 'a' as i32)) <
                              0 as libc::c_int {
                    bell();
                } else {
                    *x_idx =
                        *p.offset((start +
                                       (which as libc::c_int - 'a' as i32)) as
                                      isize);
                    ret =
                        &mut *powers_type.offset(*p.offset((start +
                                                                (which as
                                                                     libc::c_int
                                                                     -
                                                                     'a' as
                                                                         i32))
                                                               as isize) as
                                                     isize) as
                            *mut power_type;
                    break ;
                }
            }
        }
        Term_load();
        character_icky = 0 as libc::c_int as bool_
    }
    rnfree(p as vptr,
           (power_max as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    return ret;
}
/* Ask & execute a power */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_power() {
    let mut x_idx: libc::c_int = 0;
    let mut x_ptr: *mut power_type = 0 as *mut power_type;
    let mut push: bool_ = 1 as libc::c_int as bool_;
    /* Get the skill, if available */
    if repeat_pull(&mut x_idx) != 0 {
        if x_idx < 0 as libc::c_int || x_idx >= power_max as libc::c_int {
            return
        }
        x_ptr = &mut *powers_type.offset(x_idx as isize) as *mut power_type;
        push = 0 as libc::c_int as bool_
    } else if command_arg == 0 {
        x_ptr = select_power(&mut x_idx)
    } else {
        x_idx = command_arg as libc::c_int - 1 as libc::c_int;
        if x_idx < 0 as libc::c_int || x_idx >= power_max as libc::c_int {
            return
        }
        x_ptr = &mut *powers_type.offset(x_idx as isize) as *mut power_type
    }
    if x_ptr.is_null() { return }
    if push != 0 { repeat_push(x_idx); }
    if *(*p_ptr).powers.offset(x_idx as isize) != 0 {
        power_activate(x_idx);
    } else {
        msg_print(b"You do not have access to this power.\x00" as *const u8 as
                      *const libc::c_char);
    };
}
