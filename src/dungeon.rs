use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    static mut adj_str_blow: [byte_hack; 0];
    #[no_mangle]
    static mut adj_con_fix: [byte_hack; 0];
    #[no_mangle]
    static mut extract_energy: [byte_hack; 300];
    #[no_mangle]
    static mut option_info: [option_type; 0];
    #[no_mangle]
    static mut sense_desc: [cptr; 0];
    #[no_mangle]
    static mut arg_wizard: bool_;
    #[no_mangle]
    static mut arg_force_original: bool_;
    #[no_mangle]
    static mut arg_force_roguelike: bool_;
    #[no_mangle]
    static mut character_generated: bool_;
    #[no_mangle]
    static mut character_dungeon: bool_;
    #[no_mangle]
    static mut character_loaded: bool_;
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut character_xtra: bool_;
    #[no_mangle]
    static mut seed_flavor: u32b;
    #[no_mangle]
    static mut command_cmd: s16b;
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut command_rep: s16b;
    #[no_mangle]
    static mut command_dir: s16b;
    #[no_mangle]
    static mut command_new: s16b;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut create_up_stair: bool_;
    #[no_mangle]
    static mut create_down_stair: bool_;
    #[no_mangle]
    static mut create_up_shaft: bool_;
    #[no_mangle]
    static mut create_down_shaft: bool_;
    #[no_mangle]
    static mut msg_flag: bool_;
    #[no_mangle]
    static mut alive: bool_;
    #[no_mangle]
    static mut death: bool_;
    #[no_mangle]
    static mut running: s16b;
    #[no_mangle]
    static mut resting: s16b;
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
    static mut wizard: bool_;
    #[no_mangle]
    static mut use_graphics: bool_;
    #[no_mangle]
    static mut total_winner: u16b;
    #[no_mangle]
    static mut has_won: u16b;
    #[no_mangle]
    static mut noscore: u16b;
    #[no_mangle]
    static mut inkey_scan: bool_;
    #[no_mangle]
    static mut shimmer_monsters: bool_;
    #[no_mangle]
    static mut shimmer_objects: bool_;
    #[no_mangle]
    static mut repair_monsters: bool_;
    #[no_mangle]
    static mut repair_objects: bool_;
    #[no_mangle]
    static mut o_max: s16b;
    #[no_mangle]
    static mut o_cnt: s16b;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut m_cnt: s16b;
    #[no_mangle]
    static mut total_friends: libc::c_int;
    #[no_mangle]
    static mut total_friend_levels: s32b;
    #[no_mangle]
    static mut leaving_quest: libc::c_int;
    #[no_mangle]
    static mut multi_rew: bool_;
    #[no_mangle]
    static mut hack_mind: bool_;
    #[no_mangle]
    static mut hack_corruption: bool_;
    #[no_mangle]
    static mut is_autosave: bool_;
    #[no_mangle]
    static mut rogue_like_commands: bool_;
    #[no_mangle]
    static mut quick_messages: bool_;
    #[no_mangle]
    static mut always_pickup: bool_;
    #[no_mangle]
    static mut disturb_state: bool_;
    #[no_mangle]
    static mut disturb_minor: bool_;
    #[no_mangle]
    static mut avoid_abort: bool_;
    #[no_mangle]
    static mut avoid_shimmer: bool_;
    #[no_mangle]
    static mut avoid_other: bool_;
    #[no_mangle]
    static mut fresh_before: bool_;
    #[no_mangle]
    static mut fresh_after: bool_;
    #[no_mangle]
    static mut alert_hitpoint: bool_;
    #[no_mangle]
    static mut view_perma_grids: bool_;
    #[no_mangle]
    static mut dungeon_stair: bool_;
    #[no_mangle]
    static mut cheat_live: bool_;
    #[no_mangle]
    static mut last_words: bool_;
    #[no_mangle]
    static mut hitpoint_warn: byte_hack;
    #[no_mangle]
    static mut autosave_freq: s16b;
    #[no_mangle]
    static mut autosave_t: bool_;
    #[no_mangle]
    static mut panel_row_min: s16b;
    #[no_mangle]
    static mut panel_row_max: s16b;
    #[no_mangle]
    static mut panel_col_min: s16b;
    #[no_mangle]
    static mut panel_col_max: s16b;
    #[no_mangle]
    static mut target_who: s16b;
    #[no_mangle]
    static mut health_who: s16b;
    #[no_mangle]
    static mut player_name: [libc::c_char; 32];
    #[no_mangle]
    static mut died_from: [libc::c_char; 80];
    #[no_mangle]
    static mut option_flag: [u32b; 8];
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
    static mut c_name: *mut libc::c_char;
    #[no_mangle]
    static mut rp_name: *mut libc::c_char;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut ANGBAND_DIR_USER: cptr;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut old_max_s_idx: u16b;
    #[no_mangle]
    static mut max_s_idx: u16b;
    #[no_mangle]
    static mut max_o_idx: u16b;
    #[no_mangle]
    static mut max_m_idx: u16b;
    #[no_mangle]
    static mut ambush_flag: bool_;
    #[no_mangle]
    static mut no_breeds: s16b;
    #[no_mangle]
    static mut fates: [fate; 200];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut take_notes: bool_;
    #[no_mangle]
    static mut autoload_old_colors: bool_;
    #[no_mangle]
    static mut fate_option: bool_;
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut effects: [effect_type; 128];
    #[no_mangle]
    static mut DUNGEON_DEATH: s32b;
    #[no_mangle]
    static mut deity_info: *mut deity_type;
    #[no_mangle]
    static mut gl_timers: *mut timer_type;
    #[no_mangle]
    fn init_hooks();
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn ingame_help(enable: bool_);
    #[no_mangle]
    static mut no_begin_screen: bool_;
    #[no_mangle]
    fn begin_screen() -> bool_;
    #[no_mangle]
    fn player_birth();
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn los(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int, x2: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn cave_valid_bold(y: libc::c_int, x: libc::c_int) -> bool_;
    #[no_mangle]
    fn move_cursor_relative(row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn note_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn do_cmd_view_map();
    #[no_mangle]
    fn forget_view();
    #[no_mangle]
    fn forget_mon_lite();
    #[no_mangle]
    fn wiz_dark();
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn scatter(yp: *mut libc::c_int, xp: *mut libc::c_int, y: libc::c_int,
               x: libc::c_int, d: libc::c_int);
    #[no_mangle]
    fn health_track(m_idx: libc::c_int);
    #[no_mangle]
    fn disturb(stop_search: libc::c_int, flush_output: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn do_stop_cmovie();
    #[no_mangle]
    fn do_start_cmovie();
    #[no_mangle]
    fn run_step(dir: libc::c_int);
    #[no_mangle]
    fn do_cmd_pet();
    #[no_mangle]
    fn do_cmd_engrave();
    #[no_mangle]
    fn do_cmd_go_up();
    #[no_mangle]
    fn do_cmd_go_down();
    #[no_mangle]
    fn do_cmd_search();
    #[no_mangle]
    fn do_cmd_toggle_search();
    #[no_mangle]
    fn do_cmd_open();
    #[no_mangle]
    fn do_cmd_close();
    #[no_mangle]
    fn do_cmd_chat();
    #[no_mangle]
    fn do_cmd_give();
    #[no_mangle]
    fn do_cmd_tunnel();
    #[no_mangle]
    fn do_cmd_disarm();
    #[no_mangle]
    fn do_cmd_bash();
    #[no_mangle]
    fn do_cmd_alter();
    #[no_mangle]
    fn do_cmd_spike();
    #[no_mangle]
    fn do_cmd_walk(pickup: libc::c_int, disarm: bool_);
    #[no_mangle]
    fn do_cmd_stay(pickup: libc::c_int);
    #[no_mangle]
    fn do_cmd_run();
    #[no_mangle]
    fn do_cmd_rest();
    #[no_mangle]
    fn do_cmd_fire();
    #[no_mangle]
    fn do_cmd_throw();
    #[no_mangle]
    fn do_cmd_boomerang();
    #[no_mangle]
    fn do_cmd_sacrifice();
    #[no_mangle]
    fn do_cmd_steal();
    #[no_mangle]
    fn do_cmd_html_dump();
    #[no_mangle]
    fn do_cmd_cli();
    #[no_mangle]
    fn do_cmd_cli_help();
    #[no_mangle]
    fn do_cmd_inven();
    #[no_mangle]
    fn do_cmd_equip();
    #[no_mangle]
    fn do_cmd_wield();
    #[no_mangle]
    fn do_cmd_takeoff();
    #[no_mangle]
    fn do_cmd_drop();
    #[no_mangle]
    fn do_cmd_destroy();
    #[no_mangle]
    fn do_cmd_observe();
    #[no_mangle]
    fn do_cmd_uninscribe();
    #[no_mangle]
    fn do_cmd_inscribe();
    #[no_mangle]
    fn do_cmd_refill();
    #[no_mangle]
    fn do_cmd_target();
    #[no_mangle]
    fn do_cmd_look();
    #[no_mangle]
    fn do_cmd_locate();
    #[no_mangle]
    fn do_cmd_query_symbol();
    #[no_mangle]
    fn do_cmd_sense_grid_mana() -> bool_;
    #[no_mangle]
    fn do_cmd_macro_recorder();
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
    fn do_cmd_knowledge();
    #[no_mangle]
    fn do_cmd_checkquest();
    #[no_mangle]
    fn do_cmd_time();
    #[no_mangle]
    fn do_cmd_browse();
    #[no_mangle]
    fn do_cmd_pray();
    #[no_mangle]
    fn symbiote_name(capitalize: bool_) -> cptr;
    #[no_mangle]
    fn do_cmd_eat_food();
    #[no_mangle]
    fn do_cmd_quaff_potion();
    #[no_mangle]
    fn do_cmd_read_scroll();
    #[no_mangle]
    fn do_cmd_aim_wand();
    #[no_mangle]
    fn do_cmd_use_staff();
    #[no_mangle]
    fn do_cmd_zap_rod();
    #[no_mangle]
    fn do_cmd_activate();
    #[no_mangle]
    fn do_cmd_cut_corpse();
    #[no_mangle]
    fn do_cmd_cure_meat();
    #[no_mangle]
    fn do_cmd_drink_fountain();
    #[no_mangle]
    fn process_pref_file(name: cptr) -> errr;
    #[no_mangle]
    fn do_cmd_help();
    #[no_mangle]
    fn process_player_name(sf: bool_);
    #[no_mangle]
    fn do_cmd_suicide();
    #[no_mangle]
    fn do_cmd_save_game();
    #[no_mangle]
    fn autosave_checkpoint();
    #[no_mangle]
    fn close_game();
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    fn evolve_level(noise: bool_);
    #[no_mangle]
    fn generate_cave();
    #[no_mangle]
    fn reveal_wilderness_around_player(y: libc::c_int, x: libc::c_int,
                                       h: libc::c_int, w: libc::c_int);
    #[no_mangle]
    fn init_v_info() -> errr;
    #[no_mangle]
    fn save_dungeon();
    #[no_mangle]
    fn load_player() -> bool_;
    #[no_mangle]
    fn carried_make_attack_normal(r_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn process_monsters();
    #[no_mangle]
    fn monster_check_experience(m_idx: libc::c_int, silent: bool_);
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn compact_monsters(size: libc::c_int);
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn update_mon(m_idx: libc::c_int, full: bool_);
    #[no_mangle]
    fn place_monster_aux(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         slp: bool_, grp: bool_, status: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn alloc_monster(dis: libc::c_int, slp: bool_) -> bool_;
    #[no_mangle]
    fn place_monster_one(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         ego: libc::c_int, slp: bool_, status: libc::c_int)
     -> s16b;
    #[no_mangle]
    fn do_control_drop() -> bool_;
    #[no_mangle]
    fn do_control_magic() -> bool_;
    #[no_mangle]
    fn do_control_pickup() -> bool_;
    #[no_mangle]
    fn do_control_inven() -> bool_;
    #[no_mangle]
    fn do_control_walk() -> bool_;
    #[no_mangle]
    fn can_create_companion() -> bool_;
    #[no_mangle]
    fn is_friend(m_ptr: *mut monster_type) -> libc::c_int;
    #[no_mangle]
    fn inc_stack_size(item: libc::c_int, delta: libc::c_int);
    #[no_mangle]
    fn calc_total_weight() -> s32b;
    #[no_mangle]
    fn flavor_init();
    #[no_mangle]
    fn reset_visuals();
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn index_to_label(i: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn describe_use(i: libc::c_int) -> cptr;
    #[no_mangle]
    fn inven_item_describe(item: libc::c_int);
    #[no_mangle]
    fn inven_item_optimize(item: libc::c_int) -> bool_;
    #[no_mangle]
    fn floor_item_describe(item: libc::c_int);
    #[no_mangle]
    fn floor_item_increase(item: libc::c_int, num: libc::c_int);
    #[no_mangle]
    fn floor_item_optimize(item: libc::c_int);
    #[no_mangle]
    fn toggle_inven_equip();
    #[no_mangle]
    fn delete_object(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn compact_objects(size: libc::c_int);
    #[no_mangle]
    fn wipe_o_list();
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn set_blessed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_shield(v: libc::c_int, p: libc::c_int, o: s16b, d1: s16b, d2: s16b)
     -> bool_;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn Term_activate(t: *mut term) -> errr;
    #[no_mangle]
    fn Term_set_cursor(v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_user(n: libc::c_int) -> errr;
    #[no_mangle]
    static mut do_movies: libc::c_int;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn Rand_state_init(seed: u32b);
    #[no_mangle]
    static mut Rand_quick: bool_;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn set_food(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_cut(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_stun(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_image(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_afraid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_poisoned(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_blind(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn window_stuff();
    #[no_mangle]
    fn redraw_stuff();
    #[no_mangle]
    fn update_stuff();
    #[no_mangle]
    fn notice_stuff();
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn get_pos_player(dis: libc::c_int, ny: *mut libc::c_int,
                      nx: *mut libc::c_int);
    #[no_mangle]
    fn floor_decay(item: libc::c_int);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn pack_decay(item: libc::c_int);
    #[no_mangle]
    fn quark_str(num: s16b) -> cptr;
    #[no_mangle]
    fn teleport_player(dis: libc::c_int);
    #[no_mangle]
    fn activate_dg_curse();
    #[no_mangle]
    fn activate_ty_curse();
    #[no_mangle]
    fn check_experience();
    #[no_mangle]
    fn do_dec_stat(stat: libc::c_int, mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn project(who: libc::c_int, rad: libc::c_int, y: libc::c_int,
               x: libc::c_int, dam: libc::c_int, typ: libc::c_int,
               flg: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn set_rush(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_mental_barrier(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_nex(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_ss(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_cc(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_oppose_ld(v: libc::c_int) -> bool_;
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
    fn set_hero(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_shadow(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_invuln(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_protundead(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_protgood(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_protevil(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_slow(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_light_speed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_fast(v: libc::c_int, p: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_infra(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_esp(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_invis(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_invis(v: libc::c_int, p: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_mimic(v: libc::c_int, p: libc::c_int, level: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_no_breeders(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_lite(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_fire_aura(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_poison(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_thunder(v: libc::c_int, p1: libc::c_int, p2: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn set_tim_fly(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_ffall(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_res_time(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_prob_travel(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_reflect(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_parasite(v: libc::c_int, r: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_disrupt_shield(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_regen(v: libc::c_int, p: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_breath(v: libc::c_int, magical: bool_) -> bool_;
    #[no_mangle]
    fn set_roots(v: libc::c_int, ac: s16b, dam: s16b) -> bool_;
    #[no_mangle]
    fn set_project(v: libc::c_int, gf: s16b, dam: s16b, rad: s16b, flag: s16b)
     -> bool_;
    #[no_mangle]
    fn set_meditation(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_strike(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_walk_water(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn set_absorb_soul(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_holy(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_res_stat(stat: libc::c_int, full: bool_) -> bool_;
    #[no_mangle]
    fn wisdom_scale(max: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inc_piety(god: libc::c_int, amt: s32b);
    #[no_mangle]
    fn has_ability(ab: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn hp_player(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn restore_level() -> bool_;
    #[no_mangle]
    fn weight_limit() -> libc::c_int;
    #[no_mangle]
    fn drop_from_wild();
    #[no_mangle]
    fn test_monster_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn select_bounties();
    #[no_mangle]
    fn bst(what: s32b, t: s32b) -> s32b;
    #[no_mangle]
    fn get_month_name(month: libc::c_int, full: bool_, compact: bool_)
     -> cptr;
    #[no_mangle]
    fn get_day(day: libc::c_int) -> cptr;
    #[no_mangle]
    fn gain_fate(fate: byte_hack);
    #[no_mangle]
    fn fire_explosion(y: libc::c_int, x: libc::c_int, typ: libc::c_int,
                      rad: libc::c_int, dam: libc::c_int) -> bool_;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn do_cmd_ability();
    #[no_mangle]
    fn do_cmd_skill();
    #[no_mangle]
    fn squeltch_grid();
    #[no_mangle]
    fn squeltch_inventory();
    #[no_mangle]
    fn do_cmd_power();
    #[no_mangle]
    fn do_cmd_activate_skill();
    #[no_mangle]
    fn change_wild_mode();
    #[no_mangle]
    fn do_cmd_store();
    #[no_mangle]
    fn repeat_check();
    #[no_mangle]
    fn request_command(shopping: libc::c_int);
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn gain_random_corruption(choose_mut: libc::c_int) -> bool_;
    #[no_mangle]
    fn verify_panel();
    #[no_mangle]
    fn get_fbranch() -> libc::c_int;
    #[no_mangle]
    fn add_note_type(note_number: libc::c_int);
    #[no_mangle]
    fn tome_dofile_anywhere(dir: cptr, file: *mut libc::c_char,
                            test_exist: bool_) -> bool_;
    #[no_mangle]
    static mut calc_powers_silent: bool_;
    #[no_mangle]
    fn msg_box(text: cptr, y: libc::c_int, x: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn init_skill(value: s32b, mod_0: s32b, i: libc::c_int);
    #[no_mangle]
    fn compute_skills(v: *mut s32b, m: *mut s32b, i: libc::c_int);
    #[no_mangle]
    fn resize_window();
    #[no_mangle]
    fn resize_map();
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn set_shero(v: libc::c_int) -> bool_;
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
    fn tolua_getnumber(L_0: *mut lua_State, narg: libc::c_int,
                       def: libc::c_long) -> libc::c_long;
    #[no_mangle]
    fn tolua_pushnumber(L_0: *mut lua_State, value: libc::c_long);
    /* File: dungeon.c */
    /* Purpose: Angband game engine */
    /*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
    #[no_mangle]
    static mut L: *mut lua_State;
    /*
 * Hack -- Declare the Debug Routines
 */
    #[no_mangle]
    fn do_cmd_debug();
}
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
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
pub struct deity_type {
    pub name: cptr,
    pub desc: [[libc::c_char; 80]; 10],
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
pub struct timer_type {
    pub next: *mut timer_type,
    pub enabled: bool_,
    pub delay: s32b,
    pub countdown: s32b,
    pub callback: cptr,
}
/*
 * Return a "feeling" (or NULL) about an item.  Method 1 (Heavy).
 */
#[no_mangle]
pub unsafe extern "C" fn value_check_aux1(mut o_ptr: *mut object_type)
 -> byte_hack {
    /* Artifacts */
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
        /* Cursed/Broken */
        if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
            return 7 as libc::c_int as byte_hack
        }
        /* Normal */
        return 8 as libc::c_int as byte_hack
    }
    /* Ego-Items */
    if if (*o_ptr).name2 as libc::c_int != 0 ||
              (*o_ptr).name2b as libc::c_int != 0 {
           1 as libc::c_int
       } else { 0 as libc::c_int } != 0 {
        /* Cursed/Broken */
        if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
            return 6 as libc::c_int as byte_hack
        }
        /* Normal */
        return 5 as libc::c_int as byte_hack
    }
    /* Cursed items */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        return 1 as libc::c_int as byte_hack
    }
    /* Good "armor" bonus */
    if (*o_ptr).to_a as libc::c_int > 0 as libc::c_int {
        return 4 as libc::c_int as byte_hack
    }
    /* Good "weapon" bonus */
    if (*o_ptr).to_h as libc::c_int + (*o_ptr).to_d as libc::c_int >
           0 as libc::c_int {
        return 4 as libc::c_int as byte_hack
    }
    /* Default to "average" */
    return 2 as libc::c_int as byte_hack;
}
#[no_mangle]
pub unsafe extern "C" fn value_check_aux1_magic(mut o_ptr: *mut object_type)
 -> byte_hack {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    match (*o_ptr).tval as libc::c_int {
        70 | 71 | 72 | 65 | 55 | 66 | 67 => {
            /* Scrolls, Potions, Wands, Staves and Rods */
            /* "Cursed" scrolls/potions have a cost of 0 */
            if (*k_ptr).cost == 0 as libc::c_int {
                return 7 as libc::c_int as byte_hack
            }
            /* Artifacts */
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
                return 8 as libc::c_int as byte_hack
            }
            /* Scroll of Nothing, Apple Juice, etc. */
            if (*k_ptr).cost < 3 as libc::c_int {
                return 6 as libc::c_int as byte_hack
            }
            /*
			 * Identify, Phase Door, Cure Light Wounds, etc. are
			 * just average
			 */
            if (*k_ptr).cost < 100 as libc::c_int {
                return 2 as libc::c_int as byte_hack
            }
            /* Enchant Armor, *Identify*, Restore Stat, etc. */
            if (*k_ptr).cost < 10000 as libc::c_int {
                return 4 as libc::c_int as byte_hack
            }
            /* Acquirement, Deincarnation, Strength, Blood of Life, ... */
            if (*k_ptr).cost >= 10000 as libc::c_int {
                return 5 as libc::c_int as byte_hack
            }
        }
        80 => {
            /* Food */
            /* "Cursed" food */
            if (*k_ptr).cost == 0 as libc::c_int {
                return 7 as libc::c_int as byte_hack
            }
            /* Artifacts */
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
                return 8 as libc::c_int as byte_hack
            }
            /* Normal food (no magical properties) */
            if (*k_ptr).cost <= 10 as libc::c_int {
                return 2 as libc::c_int as byte_hack
            }
            /* Everything else is good */
            if (*k_ptr).cost > 10 as libc::c_int {
                return 4 as libc::c_int as byte_hack
            }
        }
        _ => { }
    }
    /* No feeling */
    return 0 as libc::c_int as byte_hack;
}
/*
 * Return a "feeling" (or NULL) about an item.  Method 2 (Light).
 */
#[no_mangle]
pub unsafe extern "C" fn value_check_aux2(mut o_ptr: *mut object_type)
 -> byte_hack {
    /* Cursed items (all of them) */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        return 1 as libc::c_int as byte_hack
    }
    /* Artifacts -- except cursed/broken ones */
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
        return 3 as libc::c_int as byte_hack
    }
    /* Ego-Items -- except cursed/broken ones */
    if if (*o_ptr).name2 as libc::c_int != 0 ||
              (*o_ptr).name2b as libc::c_int != 0 {
           1 as libc::c_int
       } else { 0 as libc::c_int } != 0 {
        return 3 as libc::c_int as byte_hack
    }
    /* Good armor bonus */
    if (*o_ptr).to_a as libc::c_int > 0 as libc::c_int {
        return 3 as libc::c_int as byte_hack
    }
    /* Good weapon bonuses */
    if (*o_ptr).to_h as libc::c_int + (*o_ptr).to_d as libc::c_int >
           0 as libc::c_int {
        return 3 as libc::c_int as byte_hack
    }
    /* Default to "average" */
    return 2 as libc::c_int as byte_hack;
}
#[no_mangle]
pub unsafe extern "C" fn value_check_aux2_magic(mut o_ptr: *mut object_type)
 -> byte_hack {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    match (*o_ptr).tval as libc::c_int {
        70 | 71 | 72 | 65 | 55 | 66 | 67 => {
            /* Scrolls, Potions, Wands, Staves and Rods */
            /* "Cursed" scrolls/potions have a cost of 0 */
            if (*k_ptr).cost == 0 as libc::c_int {
                return 1 as libc::c_int as byte_hack
            }
            /* Artifacts */
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
                return 3 as libc::c_int as byte_hack
            }
            /* Scroll of Nothing, Apple Juice, etc. */
            if (*k_ptr).cost < 3 as libc::c_int {
                return 2 as libc::c_int as byte_hack
            }
            /*
			 * Identify, Phase Door, Cure Light Wounds, etc. are
			 * just average
			 */
            if (*k_ptr).cost < 100 as libc::c_int {
                return 2 as libc::c_int as byte_hack
            }
            /* Enchant Armor, *Identify*, Restore Stat, etc. */
            if (*k_ptr).cost < 10000 as libc::c_int {
                return 3 as libc::c_int as byte_hack
            }
            /* Acquirement, Deincarnation, Strength, Blood of Life, ... */
            if (*k_ptr).cost >= 10000 as libc::c_int {
                return 3 as libc::c_int as byte_hack
            }
        }
        80 => {
            /* Food */
            /* "Cursed" food */
            if (*k_ptr).cost == 0 as libc::c_int {
                return 1 as libc::c_int as byte_hack
            }
            /* Artifacts */
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
                return 3 as libc::c_int as byte_hack
            }
            /* Normal food (no magical properties) */
            if (*k_ptr).cost <= 10 as libc::c_int {
                return 2 as libc::c_int as byte_hack
            }
            /* Everything else is good */
            if (*k_ptr).cost > 10 as libc::c_int {
                return 3 as libc::c_int as byte_hack
            }
        }
        _ => { }
    }
    /* No feeling */
    return 0 as libc::c_int as byte_hack;
}
/*
 * Can a player be resurrected?
 */
unsafe extern "C" fn granted_resurrection() -> bool_ {
    if (*p_ptr).pgod as libc::c_int == 1 as libc::c_int &&
           (*p_ptr).praying as libc::c_int != 0 {
        if (*p_ptr).grace > 100000 as libc::c_int {
            if Rand_div(100 as libc::c_int) < 70 as libc::c_int {
                return 1 as libc::c_int as bool_
            } else { return 0 as libc::c_int as bool_ }
        }
    }
    return 0 as libc::c_int as bool_;
}
unsafe extern "C" fn select_sense(mut o_ptr: *mut object_type) -> byte_hack {
    /* Valid "tval" codes */
    match (*o_ptr).tval as libc::c_int {
        16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 6 | 24 | 30 | 31 | 32 | 33 |
        34 | 35 | 36 | 37 | 38 | 15 | 46 => {
            return 1 as libc::c_int as byte_hack
        }
        71 | 72 | 70 | 65 | 55 | 66 | 67 => {
            return 2 as libc::c_int as byte_hack
        }
        115 => {
            /* Dual use? */
            return 1 as libc::c_int as byte_hack
        }
        _ => { }
    }
    return 0 as libc::c_int as byte_hack;
}
/*
 * Sense the inventory
 *
 * Combat items (weapons and armour) - Fast, weak if combat skill < 10, strong
 * otherwise.
 *
 * Magic items (scrolls, staffs, wands, potions etc) - Slow, weak if
 * magic skill < 10, strong otherwise.
 *
 * It shouldn't matter a lot to discriminate against magic users, because
 * they learn one form of ID or another, and because most magic items are
 * easy_know.
 */
#[no_mangle]
pub unsafe extern "C" fn sense_inventory() {
    let mut i: libc::c_int = 0;
    let mut combat_lev: libc::c_int = 0;
    let mut magic_lev: libc::c_int = 0;
    let mut heavy_combat: bool_ = 0;
    let mut heavy_magic: bool_ = 0;
    let mut feel: byte_hack = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* ** Check for "sensing" ***/
    /* No sensing when confused */
    if (*p_ptr).confused != 0 { return }
    /*
	 * In Angband, the chance of pseudo-id uses two different formulae:
	 *
	 * (1) Fast. 0 == rand_int(BASE / (plev * plev + 40)
	 * (2) Slow. 0 == rand_int(BASE / (plev + 5)
	 *
	 * Warriors: Fase with BASE == 9000
	 * Magi: Slow with BASE == 240000
	 * Priests: Fast with BASE == 10000
	 * Rogues: Fase with BASE == 20000
	 * Rangers: Slow with BASE == 120000
	 * Paladins: Fast with BASE == 80000
	 *
	 * In other words, those who have identify spells are penalised.
	 * The problems with Pern/Tome since it externalised player classes
	 * is that it uses the same and slow formula for spellcasters and
	 * fighters.
	 *
	 * In the following code, combat item pseudo-ID improves exponentially,
	 * (fast with BASE 9000) and magic one linear (slow with base 60000 --
	 * twice faster than V rangers).
	 *
	 * I hope this makes it closer to the original model -- pelpel
	 */
    /* The combat skill affects weapon/armour pseudo-ID */
    combat_lev = get_skill(16 as libc::c_int) as libc::c_int;
    /* The magic skill affects magic item pseudo-ID */
    magic_lev = get_skill(15 as libc::c_int) as libc::c_int;
    /* Higher skill levels give the player better sense of items */
    heavy_combat =
        if combat_lev > 10 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    heavy_magic =
        if magic_lev > 10 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    /* ** Sense everything ***/
    /* Check everything */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut okay: byte_hack = 0 as libc::c_int as byte_hack;
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip empty slots */
        if !((*o_ptr).k_idx == 0) {
            /* We know about it already, do not tell us again */
            if !((*o_ptr).ident as libc::c_int & 0x1 as libc::c_int != 0) {
                /* It is fully known, no information needed */
                if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0
                         ||
                         (*k_info.offset((*o_ptr).k_idx as isize)).easy_know
                             as libc::c_int != 0 &&
                             (*k_info.offset((*o_ptr).k_idx as isize)).aware
                                 as libc::c_int != 0) {
                    /* Valid "tval" codes */
                    okay = select_sense(o_ptr);
                    /* Skip non-sense machines */
                    if !(okay == 0) {
                        /* Check for a feeling */
                        if okay as libc::c_int == 1 as libc::c_int {
                            feel =
                                if heavy_combat as libc::c_int != 0 {
                                    value_check_aux1(o_ptr) as libc::c_int
                                } else {
                                    value_check_aux2(o_ptr) as libc::c_int
                                } as byte_hack
                        } else {
                            feel =
                                if heavy_magic as libc::c_int != 0 {
                                    value_check_aux1_magic(o_ptr) as
                                        libc::c_int
                                } else {
                                    value_check_aux2_magic(o_ptr) as
                                        libc::c_int
                                } as byte_hack
                        }
                        /* Skip non-feelings */
                        if !(feel as libc::c_int == 0 as libc::c_int) {
                            /* Get an object description */
                            object_desc(o_name.as_mut_ptr(), o_ptr,
                                        0 as libc::c_int, 0 as libc::c_int);
                            /* Message (equipment) */
                            if i >= 24 as libc::c_int {
                                msg_format(b"You feel the %s (%c) you are %s %s %s...\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           o_name.as_mut_ptr(),
                                           index_to_label(i) as libc::c_int,
                                           describe_use(i),
                                           if (*o_ptr).number as libc::c_int
                                                  == 1 as libc::c_int {
                                               b"is\x00" as *const u8 as
                                                   *const libc::c_char
                                           } else {
                                               b"are\x00" as *const u8 as
                                                   *const libc::c_char
                                           },
                                           *sense_desc.as_mut_ptr().offset(feel
                                                                               as
                                                                               isize));
                            } else {
                                /* Message (inventory) */
                                msg_format(b"You feel the %s (%c) in your pack %s %s...\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           o_name.as_mut_ptr(),
                                           index_to_label(i) as libc::c_int,
                                           if (*o_ptr).number as libc::c_int
                                                  == 1 as libc::c_int {
                                               b"is\x00" as *const u8 as
                                                   *const libc::c_char
                                           } else {
                                               b"are\x00" as *const u8 as
                                                   *const libc::c_char
                                           },
                                           *sense_desc.as_mut_ptr().offset(feel
                                                                               as
                                                                               isize));
                            }
                            /* We have "felt" it */
                            (*o_ptr).ident =
                                ((*o_ptr).ident as libc::c_int |
                                     0x1 as libc::c_int) as byte_hack;
                            /* Set sense property */
                            (*o_ptr).sense = feel;
                            /* Combine / Reorder the pack (later) */
                            (*p_ptr).notice =
                                ((*p_ptr).notice as libc::c_long |
                                     (0x1 as libc::c_long |
                                          0x2 as libc::c_long)) as u32b;
                            /* Window stuff */
                            (*p_ptr).window =
                                ((*p_ptr).window as libc::c_long |
                                     (0x1 as libc::c_long |
                                          0x2 as libc::c_long)) as u32b
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Squelch ! */
    squeltch_inventory();
}
/*
 * Go to any level (ripped off from wiz_jump)
 */
unsafe extern "C" fn pattern_teleport() {
    /* Ask for level */
    if get_check(b"Teleport level? \x00" as *const u8 as *const libc::c_char)
           != 0 {
        let mut ppp: [libc::c_char; 80] = [0; 80];
        let mut tmp_val: [libc::c_char; 160] = [0; 160];
        /* Prompt */
        sprintf(ppp.as_mut_ptr(),
                b"Teleport to level (0-%d): \x00" as *const u8 as
                    *const libc::c_char, 99 as libc::c_int);
        /* Default */
        sprintf(tmp_val.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                dun_level as libc::c_int);
        /* Ask for a level */
        if get_string(ppp.as_mut_ptr() as cptr, tmp_val.as_mut_ptr(),
                      10 as libc::c_int) == 0 {
            return
        }
        /* Extract request */
        command_arg = atoi(tmp_val.as_mut_ptr()) as s16b
    } else if get_check(b"Normal teleport? \x00" as *const u8 as
                            *const libc::c_char) != 0 {
        teleport_player(200 as libc::c_int);
        return
    } else { return }
    /* Paranoia */
    if (command_arg as libc::c_int) < 0 as libc::c_int {
        command_arg = 0 as libc::c_int as s16b
    }
    /* Paranoia */
    if command_arg as libc::c_int > 99 as libc::c_int {
        command_arg = 99 as libc::c_int as s16b
    }
    /* Accept request */
    msg_format(b"You teleport to dungeon level %d.\x00" as *const u8 as
                   *const libc::c_char, command_arg as libc::c_int);
    autosave_checkpoint();
    /* Change level */
    dun_level = command_arg;
    /* Leaving */
    (*p_ptr).leaving = 1 as libc::c_int as bool_;
}
/*
 * Returns TRUE if we are on the Straight Road...
 */
unsafe extern "C" fn pattern_effect() -> bool_ {
    if ((*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
            libc::c_int) < 0x41 as libc::c_int ||
           (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
               libc::c_int > 0x49 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
           libc::c_int == 0x46 as libc::c_int {
        set_poisoned(0 as libc::c_int);
        set_image(0 as libc::c_int);
        set_stun(0 as libc::c_int);
        set_cut(0 as libc::c_int);
        set_blind(0 as libc::c_int);
        set_afraid(0 as libc::c_int);
        do_res_stat(0 as libc::c_int, 1 as libc::c_int as bool_);
        do_res_stat(1 as libc::c_int, 1 as libc::c_int as bool_);
        do_res_stat(2 as libc::c_int, 1 as libc::c_int as bool_);
        do_res_stat(3 as libc::c_int, 1 as libc::c_int as bool_);
        do_res_stat(4 as libc::c_int, 1 as libc::c_int as bool_);
        do_res_stat(5 as libc::c_int, 1 as libc::c_int as bool_);
        restore_level();
        hp_player(1000 as libc::c_int);
        cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                      0x47 as libc::c_int);
        msg_print(b"This section of the Straight Road looks less powerful.\x00"
                      as *const u8 as *const libc::c_char);
    } else if !((*cave[(*p_ptr).py as
                           usize].offset((*p_ptr).px as isize)).feat as
                    libc::c_int == 0x47 as libc::c_int) {
        if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
               libc::c_int == 0x48 as libc::c_int {
            pattern_teleport();
        } else if (*cave[(*p_ptr).py as
                             usize].offset((*p_ptr).px as isize)).feat as
                      libc::c_int == 0x49 as libc::c_int {
            if (*p_ptr).invuln == 0 {
                take_hit(200 as libc::c_int,
                         b"walking the corrupted Straight Road\x00" as
                             *const u8 as *const libc::c_char);
            }
        } else if (*p_ptr).invuln == 0 {
            take_hit(damroll(1 as libc::c_int as s16b,
                             3 as libc::c_int as s16b),
                     b"walking the Straight Road\x00" as *const u8 as
                         *const libc::c_char);
        }
    }
    return 1 as libc::c_int as bool_;
}
/*
	 * We could make the healing effect of the
	 * Pattern center one-time only to avoid various kinds
	 * of abuse, like luring the win monster into fighting you
	 * in the middle of the pattern...
	 */
/*
 * If player has inscribed the object with "!!", let him know when it's
 * recharged. -LM-
 */
unsafe extern "C" fn recharged_notice(mut o_ptr: *mut object_type) {
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut s: cptr = 0 as *const libc::c_char;
    /* No inscription */
    if (*o_ptr).note == 0 { return }
    /* Find a '!' */
    s = strchr(quark_str((*o_ptr).note as s16b), '!' as i32) as cptr;
    /* Process notification request. */
    while !s.is_null() {
        /* Find another '!' */
        if *s.offset(1 as libc::c_int as isize) as libc::c_int == '!' as i32 {
            /* Describe (briefly) */
            object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                        0 as libc::c_int);
            /* Notify the player */
            if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                msg_format(b"Your %s are recharged.\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr());
            } else {
                msg_format(b"Your %s is recharged.\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr());
            }
            /* Done. */
            return
        }
        /* Keep looking for '!'s */
        s = strchr(s.offset(1 as libc::c_int as isize), '!' as i32) as cptr
    };
}
/*
 * Regenerate hit points				-RAK-
 */
unsafe extern "C" fn regenhp(mut percent: libc::c_int) {
    let mut new_chp: s32b = 0;
    let mut new_chp_frac: s32b = 0;
    let mut old_chp: libc::c_int = 0;
    /* Only if alive */
    if (*p_ptr).necro_extra & 0x8 as libc::c_int as libc::c_uint == 0 {
        /* Save the old hitpoints */
        old_chp = (*p_ptr).chp as libc::c_int;
        /* Extract the new hitpoints */
        new_chp =
            ((*p_ptr).mhp as libc::c_long * percent as libc::c_long +
                 1442 as libc::c_int as libc::c_long) as s32b;
        /* div 65536 */
        (*p_ptr).chp =
            ((*p_ptr).chp as libc::c_int + (new_chp >> 16 as libc::c_int)) as
                s16b;
        /* check for overflow */
        if ((*p_ptr).chp as libc::c_int) < 0 as libc::c_int &&
               old_chp > 0 as libc::c_int {
            (*p_ptr).chp = 32767 as libc::c_int as s16b
        }
        /* mod 65536 */
        new_chp_frac =
            (new_chp & 0xffff as libc::c_int) +
                (*p_ptr).chp_frac as libc::c_int;
        if new_chp_frac as libc::c_long >= 0x10000 as libc::c_long {
            (*p_ptr).chp_frac =
                (new_chp_frac as libc::c_long - 0x10000 as libc::c_long) as
                    u16b;
            (*p_ptr).chp += 1
        } else { (*p_ptr).chp_frac = new_chp_frac as u16b }
        /* Fully healed */
        if (*p_ptr).chp as libc::c_int >= (*p_ptr).mhp as libc::c_int {
            (*p_ptr).chp = (*p_ptr).mhp;
            (*p_ptr).chp_frac = 0 as libc::c_int as u16b
        }
        /* Notice changes */
        if old_chp != (*p_ptr).chp as libc::c_int {
            /* Redraw */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as
                    u32b;
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as
                    u32b
        }
    };
}
/*
 * Regenerate mana points				-RAK-
 */
unsafe extern "C" fn regenmana(mut percent: libc::c_int) {
    let mut new_mana: s32b = 0;
    let mut new_mana_frac: s32b = 0;
    let mut old_csp: libc::c_int = 0;
    /* Incraese regen with int */
    percent +=
        *adj_str_blow.as_mut_ptr().offset((*p_ptr).stat_ind[1 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int *
            3 as libc::c_int;
    old_csp = (*p_ptr).csp as libc::c_int;
    new_mana =
        ((*p_ptr).msp as libc::c_long * percent as libc::c_long +
             524 as libc::c_int as libc::c_long) as s32b;
    /* div 65536 */
    (*p_ptr).csp =
        ((*p_ptr).csp as libc::c_int + (new_mana >> 16 as libc::c_int)) as
            s16b;
    /* check for overflow */
    if ((*p_ptr).csp as libc::c_int) < 0 as libc::c_int &&
           old_csp > 0 as libc::c_int {
        (*p_ptr).csp = 32767 as libc::c_int as s16b
    }
    /* mod 65536 */
    new_mana_frac =
        (new_mana & 0xffff as libc::c_int) + (*p_ptr).csp_frac as libc::c_int;
    if new_mana_frac as libc::c_long >= 0x10000 as libc::c_long {
        (*p_ptr).csp_frac =
            (new_mana_frac as libc::c_long - 0x10000 as libc::c_long) as u16b;
        (*p_ptr).csp += 1
    } else { (*p_ptr).csp_frac = new_mana_frac as u16b }
    /* Must set frac to zero even if equal */
    if (*p_ptr).csp as libc::c_int >= (*p_ptr).msp as libc::c_int {
        (*p_ptr).csp = (*p_ptr).msp;
        (*p_ptr).csp_frac = 0 as libc::c_int as u16b
    }
    /* Redraw mana */
    if old_csp != (*p_ptr).csp as libc::c_int {
        /* Redraw */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    };
}
/*
 * Regenerate the monsters (once per 100 game turns)
 *
 * XXX XXX XXX Should probably be done during monster turns.
 */
unsafe extern "C" fn regen_monsters() {
    let mut i: libc::c_int = 0;
    let mut frac: libc::c_int = 0;
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int as
                                                         isize) as
            *mut object_type;
    if (*o_ptr).k_idx != 0 {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset((*o_ptr).pval as isize) as *mut monster_race;
        /* Allow regeneration (if needed) */
        if ((*o_ptr).pval2 as libc::c_int) < (*o_ptr).pval3 {
            /* Hack -- Base regeneration */
            frac = (*o_ptr).pval3 / 100 as libc::c_int;
            /* Hack -- Minimal regeneration rate */
            if frac == 0 { frac = 1 as libc::c_int }
            /* Hack -- Some monsters regenerate quickly */
            if (*r_ptr).flags2 & 0x200 as libc::c_int as libc::c_uint != 0 {
                frac *= 2 as libc::c_int
            }
            /* Hack -- Regenerate */
            (*o_ptr).pval2 = ((*o_ptr).pval2 as libc::c_int + frac) as s16b;
            /* Do not over-regenerate */
            if (*o_ptr).pval2 as libc::c_int > (*o_ptr).pval3 {
                (*o_ptr).pval2 = (*o_ptr).pval3 as s16b
            }
            /* Redraw (later) */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x10000000 as libc::c_long)
                    as u32b
        }
    }
    /* Regenerate everyone */
    i = 1 as libc::c_int;
    while i < m_max as libc::c_int {
        /* Check the i'th monster */
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        let mut r_ptr_0: *mut monster_race =
            if !(*m_ptr).sr_ptr.is_null() {
                (*m_ptr).sr_ptr
            } else {
                race_info_idx((*m_ptr).r_idx as libc::c_int,
                              (*m_ptr).ego as libc::c_int)
            };
        /* Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Dont regen bleeding/poisonned monsters */
            if !((*m_ptr).bleeding as libc::c_int != 0 ||
                     (*m_ptr).poisoned as libc::c_int != 0) {
                /* Allow regeneration (if needed) */
                if (*m_ptr).hp < (*m_ptr).maxhp {
                    /* Hack -- Base regeneration */
                    frac = (*m_ptr).maxhp / 100 as libc::c_int;
                    /* Hack -- Minimal regeneration rate */
                    if frac == 0 { frac = 1 as libc::c_int }
                    /* Hack -- Some monsters regenerate quickly */
                    if (*r_ptr_0).flags2 &
                           0x200 as libc::c_int as libc::c_uint != 0 {
                        frac *= 2 as libc::c_int
                    }
                    /* Hack -- Regenerate */
                    (*m_ptr).hp += frac;
                    /* Do not over-regenerate */
                    if (*m_ptr).hp > (*m_ptr).maxhp {
                        (*m_ptr).hp = (*m_ptr).maxhp
                    }
                    /* Redraw (later) if needed */
                    if health_who as libc::c_int == i {
                        (*p_ptr).redraw =
                            ((*p_ptr).redraw as libc::c_long |
                                 0x800 as libc::c_long) as u32b
                    }
                }
            }
        }
        i += 1
    };
}
/*
 * Does an object decay?
 *
 * Should belong to object1.c, renamed to object_decays() -- pelpel
 */
#[no_mangle]
pub unsafe extern "C" fn decays(mut o_ptr: *mut object_type) -> bool_ {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Extract some flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f3 as libc::c_long & 0x8 as libc::c_long != 0 {
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
unsafe extern "C" fn process_lasting_spell(mut music: s16b) -> libc::c_int {
    let mut oldtop: libc::c_int = 0;
    let mut use_mana: libc::c_int = 0;
    if music as libc::c_int > 0 as libc::c_int { return 0 as libc::c_int }
    oldtop = lua_gettop(L);
    music = -(music as libc::c_int) as s16b;
    /* Push the function */
    lua_getglobal(L,
                  b"exec_lasting_spell\x00" as *const u8 as
                      *const libc::c_char);
    /* Push the spell */
    tolua_pushnumber(L, music as libc::c_long);
    /* Call the function */
    if lua_call(L, 1 as libc::c_int, 1 as libc::c_int) != 0 {
        cmsg_format(10 as libc::c_int as byte_hack,
                    b"ERROR in lua_call while calling lasting spell\x00" as
                        *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    use_mana =
        tolua_getnumber(L, -(lua_gettop(L) - oldtop),
                        0 as libc::c_int as libc::c_long) as libc::c_int;
    lua_settop(L, oldtop);
    return use_mana;
}
unsafe extern "C" fn gere_class_special() {
    match (*p_ptr).druid_extra2 {
        1 => {
            /* Lay a path of mana on the floor */
            /* Does the player have enought mana ? */
            if ((*p_ptr).csp as libc::c_int) <
                   ((*p_ptr).druid_extra & 255 as libc::c_int as libc::c_uint)
                       as s32b {
                (*p_ptr).druid_extra = 0 as libc::c_int as u32b;
                (*p_ptr).druid_extra2 = 0 as libc::c_int as u32b;
                msg_print(b"You stop laying a mana path.\x00" as *const u8 as
                              *const libc::c_char);
            } else {
                /* Use some mana */
                (*p_ptr).csp =
                    ((*p_ptr).csp as
                         libc::c_uint).wrapping_sub((*p_ptr).druid_extra &
                                                        255 as libc::c_int as
                                                            libc::c_uint) as
                        s16b as s16b;
                if (*p_ptr).druid_extra >> 8 as libc::c_int &
                       0x1 as libc::c_int as libc::c_uint != 0 {
                    /* Absorb some of the mana of the grid */
                    (*p_ptr).csp =
                        ((*p_ptr).csp as libc::c_int +
                             (*cave[(*p_ptr).py as
                                        usize].offset((*p_ptr).px as
                                                          isize)).mana as
                                 libc::c_int / 50 as libc::c_int) as s16b;
                    if (*p_ptr).csp as libc::c_int >
                           (*p_ptr).msp as libc::c_int {
                        (*p_ptr).csp = (*p_ptr).msp
                    }
                    /* Set the new grid mana */
                    (*cave[(*p_ptr).py as
                               usize].offset((*p_ptr).px as isize)).mana =
                        ((*p_ptr).druid_extra &
                             255 as libc::c_int as libc::c_uint) as byte_hack
                } else {
                    let mut m: libc::c_int =
                        (*cave[(*p_ptr).py as
                                   usize].offset((*p_ptr).px as isize)).mana
                            as libc::c_int;
                    if (m as
                            libc::c_uint).wrapping_add((*p_ptr).druid_extra &
                                                           255 as libc::c_int
                                                               as
                                                               libc::c_uint) >
                           255 as libc::c_int as libc::c_uint {
                        (*cave[(*p_ptr).py as
                                   usize].offset((*p_ptr).px as isize)).mana =
                            255 as libc::c_int as byte_hack
                    } else {
                        let ref mut fresh0 =
                            (*cave[(*p_ptr).py as
                                       usize].offset((*p_ptr).px as
                                                         isize)).mana;
                        *fresh0 =
                            (*fresh0 as
                                 libc::c_uint).wrapping_add((*p_ptr).druid_extra
                                                                &
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                as byte_hack as byte_hack
                    }
                }
            }
        }
        3 => {
            /* Lay a path of mana on the floor */
            /* Does the player have enought mana ? */
            if ((*p_ptr).csp as libc::c_int) <
                   ((*p_ptr).druid_extra & 255 as libc::c_int as libc::c_uint)
                       as s32b {
                (*p_ptr).druid_extra = 0 as libc::c_int as u32b;
                msg_print(b"You stop expulsing mana winds.\x00" as *const u8
                              as *const libc::c_char);
            } else {
                let mut dam: libc::c_int = 0 as libc::c_int;
                /* Use some mana */
                (*p_ptr).csp =
                    ((*p_ptr).csp as
                         libc::c_uint).wrapping_sub((*p_ptr).druid_extra &
                                                        255 as libc::c_int as
                                                            libc::c_uint) as
                        s16b as s16b;
                if (*p_ptr).druid_extra >> 8 as libc::c_int &
                       0x1 as libc::c_int as libc::c_uint != 0 {
                    dam =
                        ((*p_ptr).druid_extra &
                             255 as libc::c_int as
                                 libc::c_uint).wrapping_add(256 as libc::c_int
                                                                as
                                                                libc::c_uint)
                            as libc::c_int
                } else {
                    dam =
                        ((*p_ptr).druid_extra &
                             255 as libc::c_int as libc::c_uint) as
                            libc::c_int
                }
                fire_explosion((*p_ptr).py as libc::c_int,
                               (*p_ptr).px as libc::c_int, 104 as libc::c_int,
                               2 as libc::c_int, dam);
            }
        }
        2 => {
            if (*p_ptr).druid_extra & 0x4 as libc::c_int as libc::c_uint != 0
               {
                (*p_ptr).csp =
                    ((*p_ptr).csp as libc::c_int +
                         (*cave[(*p_ptr).py as
                                    usize].offset((*p_ptr).px as isize)).mana
                             as libc::c_int / 10 as libc::c_int) as s16b
            } else {
                (*p_ptr).csp =
                    ((*p_ptr).csp as libc::c_int +
                         (*cave[(*p_ptr).py as
                                    usize].offset((*p_ptr).px as isize)).mana
                             as libc::c_int / 20 as libc::c_int) as s16b
            }
            if (*p_ptr).csp as libc::c_int > (*p_ptr).msp as libc::c_int {
                (*p_ptr).csp = (*p_ptr).msp
            }
            (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).mana =
                0 as libc::c_int as byte_hack
        }
        _ => {
            /* CLASS_NONE, possibly others? */
            /* No mana update */
            return
        }
    }
    /* Redraw mana */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
}
unsafe extern "C" fn check_music() {
    let mut use_mana: libc::c_int = 0;
    /* Music sung by player */
    if (*p_ptr).music_extra == 0 { return }
    use_mana = process_lasting_spell((*p_ptr).music_extra as s16b);
    if ((*p_ptr).csp as libc::c_int) < use_mana {
        msg_print(b"You stop your spell.\x00" as *const u8 as
                      *const libc::c_char);
        (*p_ptr).music_extra = 0 as libc::c_int as u32b;
        (*p_ptr).music_extra2 = 0 as libc::c_int as u32b
    } else {
        (*p_ptr).csp = ((*p_ptr).csp as libc::c_int - use_mana) as s16b;
        /* Redraw mana */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    };
}
/*
 * Generate the feature effect
 */
#[no_mangle]
pub unsafe extern "C" fn apply_effect(mut y: libc::c_int,
                                      mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut f_ptr: *mut feature_type =
        &mut *f_info.offset((*c_ptr).feat as isize) as *mut feature_type;
    if (*f_ptr).d_frequency[0 as libc::c_int as usize] != 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            /* Check the frequency */
            if !((*f_ptr).d_frequency[i as usize] == 0 as libc::c_int) {
                if turn % (*f_ptr).d_frequency[i as usize] == 0 as libc::c_int
                       &&
                       ((*f_ptr).d_side[i as usize] != 0 as libc::c_int ||
                            (*f_ptr).d_dice[i as usize] != 0 as libc::c_int) {
                    let mut l: libc::c_int = 0;
                    let mut dam: libc::c_int = 0 as libc::c_int;
                    let mut d: libc::c_int = (*f_ptr).d_dice[i as usize];
                    let mut s: libc::c_int = (*f_ptr).d_side[i as usize];
                    if d == -(1 as libc::c_int) {
                        d = (*p_ptr).lev as libc::c_int
                    }
                    if s == -(1 as libc::c_int) {
                        s = (*p_ptr).lev as libc::c_int
                    }
                    /* Roll damage */
                    l = 0 as libc::c_int;
                    while l < d {
                        dam += Rand_div(s) + 1 as libc::c_int;
                        l += 1
                    }
                    /* Apply damage */
                    project(-(100 as libc::c_int), 0 as libc::c_int, y, x,
                            dam, (*f_ptr).d_type[i as usize],
                            0x40 as libc::c_int | 0x80 as libc::c_int);
                    /* Hack -- notice death */
                    if alive == 0 || death as libc::c_int != 0 { return }
                }
            }
            i += 1
        }
    };
}
/* XXX XXX XXX */
#[no_mangle]
pub static mut is_recall: bool_ = 0 as libc::c_int as bool_;
/*
 * Handle certain things once every 10 game turns
 *
 * Note that a single movement in the overhead wilderness mode
 * consumes 132 times as much energy as a normal one...
 */
unsafe extern "C" fn process_world() {
    let mut t_ptr: *mut timer_type = 0 as *mut timer_type;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut regen_amount: libc::c_int = 0;
    let mut cave_no_regen: bool_ = 0 as libc::c_int as bool_;
    let mut upkeep_factor: libc::c_int = 0 as libc::c_int;
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut f1: u32b = 0 as libc::c_int as u32b;
    let mut f2: u32b = 0 as libc::c_int as u32b;
    let mut f3: u32b = 0 as libc::c_int as u32b;
    let mut f4: u32b = 0 as libc::c_int as u32b;
    let mut f5: u32b = 0 as libc::c_int as u32b;
    let mut esp: u32b = 0 as libc::c_int as u32b;
    /*
	 * Every 10 game turns -- which means this section is invoked once
	 * in a player turn under the normal speed, and 132 times in a move
	 * in the reduced map mode.
	 */
    if turn % 10 as libc::c_int != 0 { return }
    /*
	 * I don't know if this is the right thing to do because I'm totally
	 * ignorant (yes, I must admit that...) about the scripting part of
	 * the game, but since there have been complaints telling us it
	 * runs terribly slow in the reduced map mode... -- pelpel
	 *
	 * Note to coders: if it is desirable to make this active in the
	 * reduced map mode, remove the if condition surrounding the line
	 * and move the code inside into every 1000 game turns section.
	 */
    if dun_level as libc::c_int != 0 || (*p_ptr).wild_mode == 0 {
        /* Let the script live! */
        process_hooks(45 as libc::c_int,
                      b"()\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
        /* Handle the player song */
        check_music();
    }
    /* Handle the timers */
    t_ptr = gl_timers;
    while !t_ptr.is_null() {
        if !((*t_ptr).enabled == 0) {
            (*t_ptr).countdown -= 1;
            if (*t_ptr).countdown == 0 {
                (*t_ptr).countdown = (*t_ptr).delay;
                call_lua((*t_ptr).callback,
                         b"()\x00" as *const u8 as *const libc::c_char,
                         b"\x00" as *const u8 as *const libc::c_char);
            }
        }
        t_ptr = (*t_ptr).next
    }
    /* Handle class special actions */
    gere_class_special();
    /* Check the fate */
    if fate_option as libc::c_int != 0 &&
           (*p_ptr).lev as libc::c_int > 10 as libc::c_int {
        /*
		 * WAS: == 666 against randint(50000).
		 * Since CPU's don't know Judeo-Christian / Cabalistic traditions,
		 * and since comparisons with zero is more efficient in many
		 * architectures...
		 */
        if Rand_div(50000 as libc::c_int) == 0 as libc::c_int {
            gain_fate(0 as libc::c_int as byte_hack);
        }
    }
    /* ** Is the wielded monsters still hypnotised ***/
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int as
                                                         isize) as
            *mut object_type;
    if (*o_ptr).k_idx != 0 {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset((*o_ptr).pval as isize) as *mut monster_race;
        if (Rand_div(1000 as libc::c_int) + 1 as libc::c_int) <
               (*r_ptr).level as libc::c_int -
                   ((*p_ptr).lev as libc::c_int * 2 as libc::c_int +
                        get_skill(8 as libc::c_int) as libc::c_int) {
            msg_format(b"%s breaks free from hypnosis!\x00" as *const u8 as
                           *const libc::c_char,
                       symbiote_name(1 as libc::c_int as bool_));
            carried_make_attack_normal((*o_ptr).pval);
        }
    }
    /* ** Attempt timed autosave ***/
    if autosave_t as libc::c_int != 0 && autosave_freq as libc::c_int != 0 {
        if turn % (autosave_freq as s32b * 10 as libc::c_int) ==
               0 as libc::c_int {
            is_autosave = 1 as libc::c_int as bool_;
            msg_print(b"Autosaving the game...\x00" as *const u8 as
                          *const libc::c_char);
            do_cmd_save_game();
            is_autosave = 0 as libc::c_int as bool_
        }
    }
    /* ** Handle the wilderness/town (sunshine) ***/
    /* While in town/wilderness and not in the overworld mode */
    if dun_level == 0 && (*p_ptr).wild_mode == 0 {
        /* Hack -- Daybreak/Nighfall in town */
        if turn as libc::c_long %
               (10 as libc::c_long * 11520 as libc::c_int as libc::c_long /
                    2 as libc::c_int as libc::c_long) ==
               0 as libc::c_int as libc::c_long {
            let mut dawn: bool_ = 0;
            /* Check for dawn */
            dawn =
                (turn as libc::c_long %
                     (10 as libc::c_long *
                          11520 as libc::c_int as libc::c_long) ==
                     0 as libc::c_int as libc::c_long) as libc::c_int as
                    bool_;
            /* Day breaks */
            if dawn != 0 {
                /* Message */
                msg_print(b"The sun has risen.\x00" as *const u8 as
                              *const libc::c_char);
                /* Hack -- Scan the town */
                y = 0 as libc::c_int;
                while y < cur_hgt as libc::c_int {
                    x = 0 as libc::c_int;
                    while x < cur_wid as libc::c_int {
                        /* Get the cave grid */
                        c_ptr =
                            &mut *(*cave.as_mut_ptr().offset(y as
                                                                 isize)).offset(x
                                                                                    as
                                                                                    isize)
                                as *mut cave_type;
                        /* Assume lit */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int |
                                 0x2 as libc::c_int) as u16b;
                        /* Hack -- Memorize lit grids if allowed */
                        if view_perma_grids != 0 {
                            (*c_ptr).info =
                                ((*c_ptr).info as libc::c_int |
                                     0x1 as libc::c_int) as u16b
                        }
                        /* Hack -- Notice spot */
                        note_spot(y, x);
                        x += 1
                    }
                    y += 1
                }
            } else {
                /* Night falls */
                /* Message */
                msg_print(b"The sun has set.\x00" as *const u8 as
                              *const libc::c_char);
                y = 0 as libc::c_int;
                while y < cur_hgt as libc::c_int {
                    x = 0 as libc::c_int;
                    while x < cur_wid as libc::c_int {
                        /* Hack -- Scan the town */
                        /* Get the cave grid */
                        c_ptr =
                            &mut *(*cave.as_mut_ptr().offset(y as
                                                                 isize)).offset(x
                                                                                    as
                                                                                    isize)
                                as *mut cave_type;
                        /* Darken "boring" features */
                        if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                               libc::c_long & 0x10 as libc::c_long != 0 &&
                               (*f_info.offset((*c_ptr).feat as isize)).flags1
                                   as libc::c_long & 0x100 as libc::c_long ==
                                   0 {
                            /* Forget the grid */
                            (*c_ptr).info =
                                ((*c_ptr).info as libc::c_int &
                                     !(0x2 as libc::c_int |
                                           0x1 as libc::c_int)) as u16b;
                            /* Hack -- Notice spot */
                            note_spot(y, x);
                        }
                        x += 1
                    }
                    y += 1
                }
            }
            /* Update the monsters */
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long)
                    as u32b;
            /* Redraw map */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long)
                    as u32b;
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as
                    u32b
        }
    }
    /* Tell a day passed */
    if (turn as libc::c_long +
            (11520 as libc::c_int / 24 as libc::c_int * 6 as libc::c_int) as
                libc::c_long * 10 as libc::c_long) %
           (10 as libc::c_long * 11520 as libc::c_int as libc::c_long) ==
           0 as libc::c_int as libc::c_long {
        let mut buf: [libc::c_char; 20] = [0; 20];
        sprintf(buf.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char,
                get_day(bst(11520 as libc::c_int * 365 as libc::c_int, turn) +
                            2890 as libc::c_int));
        cmsg_format(13 as libc::c_int as byte_hack,
                    b"Today it is %s of the %s year of the third age.\x00" as
                        *const u8 as *const libc::c_char,
                    get_month_name(bst(11520 as libc::c_int, turn), wizard,
                                   0 as libc::c_int as bool_),
                    buf.as_mut_ptr());
    }
    /* Set back the rewards once a day */
    if turn as libc::c_long %
           (10 as libc::c_long * 1000 as libc::c_int as libc::c_long) ==
           0 as libc::c_int as libc::c_long {
        /* Select new bounties. */
        if Rand_div(100 as libc::c_int) < 20 as libc::c_int {
            select_bounties();
        }
    }
    /* Modify loan */
    if (*p_ptr).loan != 0 {
        if (*p_ptr).loan_time != 0 { (*p_ptr).loan_time -= 1 }
        if turn % 5000 as libc::c_int == 0 as libc::c_int &&
               (*p_ptr).loan_time == 0 {
            cmsg_print(4 as libc::c_int as byte_hack,
                       b"You should pay your loan...\x00" as *const u8 as
                           *const libc::c_char);
            (*p_ptr).loan += (*p_ptr).loan / 12 as libc::c_int;
            if (*p_ptr).loan as libc::c_long > 999999999 as libc::c_long {
                (*p_ptr).loan = 999999999 as libc::c_long as s32b
            }
            /* Do a nasty stuff */
            if (*p_ptr).wild_mode as libc::c_int != 0 &&
                   Rand_div(2 as libc::c_int) != 0 {
                /* Discount player items */
                let mut z: libc::c_int = 0 as libc::c_int;
                let mut tries: libc::c_int = 200 as libc::c_int;
                let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
                loop  {
                    let fresh1 = tries;
                    tries = tries - 1;
                    if !(fresh1 != 0) { break ; }
                    z = Rand_div(52 as libc::c_int);
                    o_ptr_0 =
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset(z as
                                                                         isize)
                            as *mut object_type;
                    if (*o_ptr_0).k_idx == 0 { continue ; }
                    if !((*o_ptr_0).discount as libc::c_int >=
                             100 as libc::c_int) {
                        break ;
                    }
                }
                if tries != 0 {
                    (*o_ptr_0).discount =
                        ((*o_ptr_0).discount as libc::c_int +
                             70 as libc::c_int) as byte_hack;
                    if (*o_ptr_0).discount as libc::c_int >=
                           100 as libc::c_int {
                        (*o_ptr_0).discount = 100 as libc::c_int as byte_hack
                    }
                    inven_item_optimize(z);
                    inven_item_describe(z);
                    (*p_ptr).window =
                        ((*p_ptr).window as libc::c_long |
                             (0x1 as libc::c_long | 0x2 as libc::c_long |
                                  0x8 as libc::c_long)) as u32b
                }
            } else {
                let mut merc: libc::c_int =
                    test_monster_name(b"Mean-looking mercenary\x00" as
                                          *const u8 as *const libc::c_char);
                let mut agent: libc::c_int =
                    test_monster_name(b"Agent of the black market\x00" as
                                          *const u8 as *const libc::c_char);
                let mut num: libc::c_int =
                    5 as libc::c_int +
                        (*p_ptr).lev as libc::c_int / 3 as libc::c_int;
                let mut z_0: libc::c_int = 0;
                z_0 = 0 as libc::c_int;
                while z_0 < num {
                    let mut yy: libc::c_int = 0;
                    let mut xx: libc::c_int = 0;
                    let mut attempts: libc::c_int = 200 as libc::c_int;
                    let mut m_idx: libc::c_int = 0;
                    loop 
                         /* Summon */
                         {
                        scatter(&mut yy, &mut xx, (*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int, 6 as libc::c_int);
                        /* Accept an empty grid within the boundary */
                        if yy > 0 as libc::c_int && xx > 0 as libc::c_int &&
                               yy < cur_hgt as libc::c_int - 1 as libc::c_int
                               &&
                               xx < cur_wid as libc::c_int - 1 as libc::c_int
                               &&
                               ((*f_info.offset((*cave[yy as
                                                           usize].offset(xx as
                                                                             isize)).feat
                                                    as isize)).flags1 as
                                    libc::c_long & 0x10 as libc::c_long != 0
                                    &&
                                    (*cave[yy as
                                               usize].offset(xx as
                                                                 isize)).feat
                                        as libc::c_int != 0xaf as libc::c_int)
                           {
                            break ;
                        }
                        /* Max number of retries reached */
                        attempts -= 1;
                        if attempts == 0 as libc::c_int { break ; }
                    }
                    /* All the attempts failed */
                    if !(attempts == 0 as libc::c_int) {
                        /* Summon a monster */
                        m_idx =
                            place_monster_one(yy, xx,
                                              if Rand_div(100 as libc::c_int)
                                                     < 80 as libc::c_int {
                                                  merc
                                              } else { agent },
                                              0 as libc::c_int,
                                              0 as libc::c_int as bool_,
                                              -(2 as libc::c_int)) as
                                libc::c_int;
                        /* Level it */
                        if m_idx != 0 {
                            let mut m_ptr: *mut monster_type =
                                &mut *m_list.offset(m_idx as isize) as
                                    *mut monster_type;
                            (*m_ptr).exp =
                                ((if (*p_ptr).lev as libc::c_int *
                                         2 as libc::c_int > 150 as libc::c_int
                                     {
                                      150 as libc::c_int
                                  } else {
                                      ((*p_ptr).lev as libc::c_int) *
                                          2 as libc::c_int
                                  }) *
                                     (if (*p_ptr).lev as libc::c_int *
                                             2 as libc::c_int >
                                             150 as libc::c_int {
                                          150 as libc::c_int
                                      } else {
                                          ((*p_ptr).lev as libc::c_int) *
                                              2 as libc::c_int
                                      }) *
                                     (if (*p_ptr).lev as libc::c_int *
                                             2 as libc::c_int >
                                             150 as libc::c_int {
                                          150 as libc::c_int
                                      } else {
                                          ((*p_ptr).lev as libc::c_int) *
                                              2 as libc::c_int
                                      }) * 6 as libc::c_int) as u32b;
                            monster_check_experience(m_idx,
                                                     1 as libc::c_int as
                                                         bool_);
                        }
                    }
                    z_0 += 1
                }
            }
        }
    }
    /* ** Process the monsters ***/
    /* Check for creature generation. */
    if (*p_ptr).wild_mode == 0 && (*p_ptr).inside_arena == 0 &&
           (*p_ptr).inside_quest == 0 &&
           Rand_div((*d_info.offset((if dun_level as libc::c_int != 0 {
                                         dungeon_type as libc::c_int
                                     } else { 0 as libc::c_int }) as
                                        isize)).max_m_alloc_chance) ==
               0 as libc::c_int {
        /* Make a new monster */
        if dungeon_flags2 as libc::c_long & 0x80 as libc::c_long == 0 {
            alloc_monster(20 as libc::c_int + 5 as libc::c_int,
                          0 as libc::c_int as bool_);
        }
    }
    /* Hack -- Check for creature regeneration */
    if (*p_ptr).wild_mode == 0 &&
           turn % 100 as libc::c_int == 0 as libc::c_int {
        regen_monsters();
    }
    /* ** Damage over Time ***/
    /* Take damage from poison */
    if (*p_ptr).poisoned as libc::c_int != 0 && (*p_ptr).invuln == 0 {
        /* Take damage */
        take_hit(1 as libc::c_int,
                 b"poison\x00" as *const u8 as *const libc::c_char);
    }
    /* Vampires take damage from sunlight */
    if (*p_ptr).sensible_lite != 0 {
        if dun_level == 0 &&
               turn as libc::c_long /
                   (10 as libc::c_long * 11520 as libc::c_int as libc::c_long
                        / 2 as libc::c_int as libc::c_long) %
                   2 as libc::c_int as libc::c_long ==
                   0 as libc::c_int as libc::c_long {
            if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).info
                   as libc::c_int & 0x2 as libc::c_int != 0 {
                /* Take damage */
                msg_print(b"The sun\'s rays scorch your undead flesh!\x00" as
                              *const u8 as *const libc::c_char);
                take_hit(1 as libc::c_int,
                         b"sunlight\x00" as *const u8 as *const libc::c_char);
                cave_no_regen = 1 as libc::c_int as bool_;
                drop_from_wild();
            }
        }
        if (*p_ptr).inventory[36 as libc::c_int as usize].tval as libc::c_int
               != 0 as libc::c_int &&
               (*p_ptr).inventory[36 as libc::c_int as usize].sval as
                   libc::c_int >= 100 as libc::c_int &&
               (*p_ptr).inventory[36 as libc::c_int as usize].sval as
                   libc::c_int <= 106 as libc::c_int &&
               (*p_ptr).inventory[36 as libc::c_int as usize].sval as
                   libc::c_int != 103 as libc::c_int {
            let mut o_ptr_1: *mut object_type =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(36 as libc::c_int
                                                                 as isize) as
                    *mut object_type;
            let mut o_name: [libc::c_char; 80] = [0; 80];
            let mut ouch: [libc::c_char; 80] = [0; 80];
            /* Get an object description */
            object_desc(o_name.as_mut_ptr(), o_ptr_1, 0 as libc::c_int,
                        0 as libc::c_int);
            msg_format(b"The %s scorches your undead flesh!\x00" as *const u8
                           as *const libc::c_char, o_name.as_mut_ptr());
            cave_no_regen = 1 as libc::c_int as bool_;
            /* Get an object description */
            object_desc(o_name.as_mut_ptr(), o_ptr_1, 1 as libc::c_int,
                        0 as libc::c_int);
            sprintf(ouch.as_mut_ptr(),
                    b"wielding %s\x00" as *const u8 as *const libc::c_char,
                    o_name.as_mut_ptr());
            take_hit(1 as libc::c_int, ouch.as_mut_ptr() as cptr);
        }
    }
    /* Drown in deep water unless the player have levitation, water walking
	   water breathing, or magic breathing.*/
    if (*p_ptr).ffall == 0 && (*p_ptr).walk_water == 0 &&
           (*p_ptr).magical_breath == 0 && (*p_ptr).water_breath == 0 &&
           (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
               libc::c_int == 0xbb as libc::c_int {
        if calc_total_weight() > weight_limit() / 2 as libc::c_int {
            /* Take damage */
            msg_print(b"You are drowning!\x00" as *const u8 as
                          *const libc::c_char);
            take_hit(Rand_div((*p_ptr).lev as s32b) + 1 as libc::c_int,
                     b"drowning\x00" as *const u8 as *const libc::c_char);
            cave_no_regen = 1 as libc::c_int as bool_
        }
    }
    /* Spectres -- take damage when moving through walls */
    /*
	 * Added: ANYBODY takes damage if inside through walls
	 * without wraith form -- NOTE: Spectres will never be
	 * reduced below 0 hp by being inside a stone wall; others
	 * WILL BE!
	 */
    if !((*f_info.offset((*cave[(*p_ptr).py as
                                    usize].offset((*p_ptr).px as isize)).feat
                             as isize)).flags1 as libc::c_long &
             0x10 as libc::c_long != 0 &&
             (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat
                 as libc::c_int != 0xaf as libc::c_int) {
        let mut feature: libc::c_int =
            (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
                libc::c_int;
        /* Player can walk through or fly over trees */
        if !((has_ability(1 as libc::c_int) as libc::c_int != 0 ||
                  (*p_ptr).fly as libc::c_int != 0) &&
                 feature == 0x60 as libc::c_int) {
            /* Player can climb over mountains */
            if !((*p_ptr).climb as libc::c_int != 0 &&
                     (*f_info.offset(feature as isize)).flags1 as libc::c_long
                         & 0x4000 as libc::c_long != 0) {
                if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                        (*spp_ptr).flags1) as libc::c_long &
                       0x20000 as libc::c_long != 0 &&
                       (*p_ptr).wraith_form == 0 &&
                       (*f_info.offset((*cave[(*p_ptr).py as
                                                  usize].offset((*p_ptr).px as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x8 as libc::c_long != 0 {
                    let mut amt: libc::c_int =
                        1 as libc::c_int +
                            (*p_ptr).lev as libc::c_int / 5 as libc::c_int;
                    cave_no_regen = 1 as libc::c_int as bool_;
                    if amt > (*p_ptr).chp as libc::c_int - 1 as libc::c_int {
                        amt = (*p_ptr).chp as libc::c_int - 1 as libc::c_int
                    }
                    take_hit(amt,
                             b" walls ...\x00" as *const u8 as
                                 *const libc::c_char);
                }
            }
        }
    }
    /* Take damage from cuts */
    if (*p_ptr).cut as libc::c_int != 0 && (*p_ptr).invuln == 0 {
        /* Mortal wound or Deep Gash */
        if (*p_ptr).cut as libc::c_int > 200 as libc::c_int {
            i = 3 as libc::c_int
        } else if (*p_ptr).cut as libc::c_int > 100 as libc::c_int {
            i = 2 as libc::c_int
        } else {
            /* Severe cut */
            /* Other cuts */
            i = 1 as libc::c_int
        }
        /* Take damage */
        take_hit(i, b"a fatal wound\x00" as *const u8 as *const libc::c_char);
    }
    /* ** Check the Food, and Regenerate ***/
    /* Digest normally */
    if ((*p_ptr).food as libc::c_int) < 15000 as libc::c_int {
        /* Every 100 game turns */
        if turn % 100 as libc::c_int == 0 as libc::c_int {
            let mut speed_use: libc::c_int = (*p_ptr).pspeed as libc::c_int;
            /* Maximum */
            if speed_use > 199 as libc::c_int {
                speed_use = 199 as libc::c_int
            } else if speed_use < 0 as libc::c_int {
                speed_use = 0 as libc::c_int
            }
            /* Minimum */
            /* Basic digestion rate based on speed */
            i =
                extract_energy[speed_use as usize] as libc::c_int *
                    2 as libc::c_int;
            /* Regeneration takes more food */
            if (*p_ptr).regenerate != 0 { i += 30 as libc::c_int }
            /* Regeneration takes more food */
            if (*p_ptr).tim_regen != 0 {
                i += (*p_ptr).tim_regen_pow as libc::c_int / 10 as libc::c_int
            }
            /* Invisibility consume a lot of food */
            i += (*p_ptr).invis as libc::c_int / 2 as libc::c_int;
            /* Invulnerability consume a lot of food */
            if (*p_ptr).invuln != 0 { i += 40 as libc::c_int }
            /* Wraith Form consume a lot of food */
            if (*p_ptr).wraith_form != 0 { i += 30 as libc::c_int }
            /* Get the weapon */
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int
                                                                 as isize) as
                    *mut object_type;
            /* Examine the sword */
            object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                         &mut esp);
            /* Hitpoints multiplier consume a lot of food */
            if (*o_ptr).k_idx as libc::c_int != 0 &&
                   f2 as libc::c_long & 0x80 as libc::c_long != 0 {
                i += (*o_ptr).pval * 5 as libc::c_int
            }
            /* Slow digestion takes less food */
            if (*p_ptr).slow_digest != 0 { i -= 10 as libc::c_int }
            /* Minimal digestion */
            if i < 1 as libc::c_int { i = 1 as libc::c_int }
            /* Digest some food */
            set_food((*p_ptr).food as libc::c_int - i);
        }
    } else {
        /* Digest quickly when gorged */
        /* Digest a lot of food */
        set_food((*p_ptr).food as libc::c_int - 100 as libc::c_int);
    }
    /* Starve to death (slowly) */
    if ((*p_ptr).food as libc::c_int) < 100 as libc::c_int {
        /* Calculate damage */
        i =
            (100 as libc::c_int - (*p_ptr).food as libc::c_int) /
                10 as libc::c_int;
        /* Take damage */
        if (*p_ptr).invuln == 0 {
            take_hit(i,
                     b"starvation\x00" as *const u8 as *const libc::c_char);
        }
    }
    /* Default regeneration */
    regen_amount = 197 as libc::c_int;
    /* Getting Weak */
    if ((*p_ptr).food as libc::c_int) < 1000 as libc::c_int {
        /* Lower regeneration */
        if ((*p_ptr).food as libc::c_int) < 100 as libc::c_int {
            regen_amount = 0 as libc::c_int
        } else if ((*p_ptr).food as libc::c_int) < 500 as libc::c_int {
            regen_amount = 33 as libc::c_int
        } else { regen_amount = 98 as libc::c_int }
        /* Getting Faint */
        if ((*p_ptr).food as libc::c_int) < 500 as libc::c_int {
            /* Faint occasionally */
            if (*p_ptr).paralyzed == 0 &&
                   Rand_div(100 as libc::c_int) < 10 as libc::c_int {
                /* Message */
                msg_print(b"You faint from the lack of food.\x00" as *const u8
                              as *const libc::c_char);
                disturb(1 as libc::c_int, 0 as libc::c_int);
                /* Hack -- faint (bypass free action) */
                set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                                  1 as libc::c_int +
                                  Rand_div(5 as libc::c_int));
            }
        }
    }
    /* Are we walking the pattern? */
    if (*p_ptr).wild_mode == 0 && pattern_effect() as libc::c_int != 0 {
        cave_no_regen = 1 as libc::c_int as bool_
    } else if (*p_ptr).regenerate != 0 {
        regen_amount = regen_amount * 2 as libc::c_int
    }
    /* Regeneration ability */
    /* Searching or Resting */
    if (*p_ptr).searching as libc::c_int != 0 || resting as libc::c_int != 0 {
        regen_amount = regen_amount * 2 as libc::c_int
    }
    if total_friends != 0 {
        let mut upkeep_divider: libc::c_int = 20 as libc::c_int;
        if has_ability(2 as libc::c_int) != 0 {
            upkeep_divider = 15 as libc::c_int
        }
        /* TRACK_FRIENDS */
        if total_friends >
               1 as libc::c_int + (*p_ptr).lev as libc::c_int / upkeep_divider
           {
            upkeep_factor = total_friend_levels;
            if upkeep_factor > 100 as libc::c_int {
                upkeep_factor = 100 as libc::c_int
            } else if upkeep_factor < 10 as libc::c_int {
                upkeep_factor = 10 as libc::c_int
            }
            /* TRACK_FRIENDS */
        }
    }
    /* Regenerate the mana */
    if ((*p_ptr).csp as libc::c_int) < (*p_ptr).msp as libc::c_int {
        if upkeep_factor != 0 {
            let mut upkeep_regen: s16b =
                ((100 as libc::c_int - upkeep_factor) * regen_amount /
                     100 as libc::c_int) as s16b;
            regenmana(upkeep_regen as libc::c_int);
            /* TRACK_FRIENDS */
        } else { regenmana(regen_amount); }
    }
    /* Eru piety incraese with time */
    if turn % 100 as libc::c_int == 0 as libc::c_int &&
           (*p_ptr).did_nothing == 0 && (*p_ptr).wild_mode == 0 {
        if (*p_ptr).pgod as libc::c_int == 1 as libc::c_int &&
               (*p_ptr).praying == 0 {
            let mut inc: libc::c_int = wisdom_scale(10 as libc::c_int);
            /* Increase by wisdom/4 */
            if inc == 0 { inc = 1 as libc::c_int }
            inc_piety(1 as libc::c_int, inc);
        }
    }
    /* Most gods piety decrease with time */
    if turn % 300 as libc::c_int == 0 as libc::c_int &&
           (*p_ptr).did_nothing == 0 && (*p_ptr).wild_mode == 0 &&
           dun_level as libc::c_int != 0 {
        if (*p_ptr).pgod as libc::c_int == 2 as libc::c_int {
            let mut dec: libc::c_int =
                4 as libc::c_int - wisdom_scale(3 as libc::c_int);
            if (*p_ptr).pgod as libc::c_int == 2 as libc::c_int &&
                   (*p_ptr).praying as libc::c_int != 0 {
                dec += 1
            }
            if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                    (*spp_ptr).flags1) as libc::c_long &
                   0x10000 as libc::c_long != 0 {
                dec -= wisdom_scale(2 as libc::c_int)
            }
            if dec < 1 as libc::c_int { dec = 1 as libc::c_int }
            inc_piety(2 as libc::c_int, -dec);
        }
        if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int {
            let mut dec_0: libc::c_int =
                8 as libc::c_int - wisdom_scale(6 as libc::c_int);
            if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int &&
                   (*p_ptr).praying as libc::c_int != 0 {
                dec_0 += 1
            }
            if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                    (*spp_ptr).flags1) as libc::c_long &
                   0x10000 as libc::c_long != 0 {
                dec_0 += 5 as libc::c_int - wisdom_scale(4 as libc::c_int)
            }
            if dec_0 < 1 as libc::c_int { dec_0 = 1 as libc::c_int }
            inc_piety(4 as libc::c_int, -dec_0);
        }
        if (*p_ptr).pgod as libc::c_int == 3 as libc::c_int &&
               (*p_ptr).praying as libc::c_int != 0 {
            let mut dec_1: libc::c_int =
                4 as libc::c_int - wisdom_scale(3 as libc::c_int);
            if dec_1 < 1 as libc::c_int { dec_1 = 1 as libc::c_int }
            inc_piety(3 as libc::c_int, -dec_1);
        }
    }
    /* Yavanna piety decrease with time */
    if turn % 400 as libc::c_int == 0 as libc::c_int &&
           (*p_ptr).did_nothing == 0 && (*p_ptr).wild_mode == 0 &&
           dun_level as libc::c_int != 0 {
        if (*p_ptr).pgod as libc::c_int == 5 as libc::c_int {
            let mut dec_2: libc::c_int =
                5 as libc::c_int - wisdom_scale(3 as libc::c_int);
            /* Blech what an hideous hack */
            if strcmp(rp_name.offset((*rp_ptr).title as isize),
                      b"Ent\x00" as *const u8 as *const libc::c_char) == 0 {
                dec_2 -= wisdom_scale(2 as libc::c_int)
            }
            if dec_2 < 1 as libc::c_int { dec_2 = 1 as libc::c_int }
            inc_piety(5 as libc::c_int, -dec_2);
        }
    }
    (*p_ptr).did_nothing = 0 as libc::c_int as bool_;
    /* Increase regen by tim regen */
    if (*p_ptr).tim_regen != 0 {
        regen_amount += (*p_ptr).tim_regen_pow as libc::c_int
    }
    /* Poisoned or cut yields no healing */
    if (*p_ptr).poisoned != 0 { regen_amount = 0 as libc::c_int }
    if (*p_ptr).cut != 0 { regen_amount = 0 as libc::c_int }
    /* Special floor -- Pattern, in a wall -- yields no healing */
    if cave_no_regen != 0 { regen_amount = 0 as libc::c_int }
    /* Being over grass allows Yavanna to regen you */
    if (*p_ptr).pgod as libc::c_int == 5 as libc::c_int &&
           (*p_ptr).praying as libc::c_int != 0 {
        if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
               libc::c_int == 0x59 as libc::c_int {
            regen_amount +=
                200 as libc::c_int + wisdom_scale(800 as libc::c_int)
        }
    }
    /* Regenerate Hit Points if needed */
    if ((*p_ptr).chp as libc::c_int) < (*p_ptr).mhp as libc::c_int &&
           cave_no_regen == 0 {
        if ((*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
                libc::c_int) < 0x46 as libc::c_int &&
               (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat
                   as libc::c_int >= 0x41 as libc::c_int {
            /* Hmmm. this should never happen? */
            regenhp(regen_amount / 5 as libc::c_int);
        } else { regenhp(regen_amount); }
    }
    /* ** Timeout Various Things ***/
    /* Handle temporary stat drains */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if (*p_ptr).stat_cnt[i as usize] as libc::c_int > 0 as libc::c_int {
            (*p_ptr).stat_cnt[i as usize] -= 1;
            if (*p_ptr).stat_cnt[i as usize] as libc::c_int ==
                   0 as libc::c_int {
                do_res_stat(i, 0 as libc::c_int as bool_);
            }
        }
        i += 1
    }
    /* Hack -- Hallucinating */
    if (*p_ptr).image != 0 {
        set_image((*p_ptr).image as libc::c_int - 1 as libc::c_int);
    }
    /* Holy Aura */
    if (*p_ptr).holy != 0 {
        set_holy((*p_ptr).holy as libc::c_int - 1 as libc::c_int);
    }
    /* Soul absorbtion */
    if (*p_ptr).absorb_soul != 0 {
        set_absorb_soul((*p_ptr).absorb_soul as libc::c_int -
                            1 as libc::c_int);
    }
    /* Undead loose Death Points */
    if (*p_ptr).necro_extra & 0x8 as libc::c_int as libc::c_uint != 0 {
        let mut old_chp: libc::c_int = (*p_ptr).chp as libc::c_int;
        let mut warning: libc::c_int =
            (*p_ptr).mhp as libc::c_int * hitpoint_warn as libc::c_int /
                10 as libc::c_int;
        /* Bypass invulnerability and wraithform */
        (*p_ptr).chp -= 1;
        /* Display the hitpoints */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
        /* Dead player */
        if ((*p_ptr).chp as libc::c_int) < 0 as libc::c_int {
            let mut old_quick: bool_ = quick_messages;
            /* Sound */
            sound(7 as libc::c_int);
            /* Hack -- Note death */
            if last_words == 0 {
                msg_print(b"You die.\x00" as *const u8 as
                              *const libc::c_char);
                msg_print(0 as cptr);
            } else {
                let mut death_message: [libc::c_char; 80] = [0; 80];
                get_rnd_line(b"death.txt\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             death_message.as_mut_ptr());
                msg_print(death_message.as_mut_ptr() as cptr);
            }
            /* Note cause of death */
            strcpy(died_from.as_mut_ptr(),
                   b"being undead too long\x00" as *const u8 as
                       *const libc::c_char);
            if (*p_ptr).image != 0 {
                strcat(died_from.as_mut_ptr(),
                       b"(?)\x00" as *const u8 as *const libc::c_char);
            }
            /* No longer a winner */
            total_winner = 0 as libc::c_int as u16b;
            /* Leaving */
            (*p_ptr).leaving = 1 as libc::c_int as bool_;
            /* Note death */
            death = 1 as libc::c_int as bool_;
            quick_messages = 0 as libc::c_int as bool_;
            if get_check(b"Make a last screenshot? \x00" as *const u8 as
                             *const libc::c_char) != 0 {
                do_cmd_html_dump();
            }
            quick_messages = old_quick;
            /* Dead */
            return
        }
        /* Hitpoint warning */
        if ((*p_ptr).chp as libc::c_int) < warning {
            /* Hack -- bell on first notice */
            if alert_hitpoint as libc::c_int != 0 && old_chp > warning {
                bell();
            }
            sound(28 as libc::c_int);
            /* Message */
            msg_print(b"*** LOW DEATHPOINT WARNING! ***\x00" as *const u8 as
                          *const libc::c_char);
            msg_print(0 as cptr);
        }
    }
    /* Walk water */
    if (*p_ptr).walk_water != 0 {
        set_walk_water((*p_ptr).walk_water as libc::c_int - 1 as libc::c_int);
    }
    /* True Strike */
    if (*p_ptr).strike != 0 {
        set_strike((*p_ptr).strike as libc::c_int - 1 as libc::c_int);
    }
    /* Meditation */
    if (*p_ptr).meditation != 0 {
        set_meditation((*p_ptr).meditation as libc::c_int - 1 as libc::c_int);
    }
    /* Timed project */
    if (*p_ptr).tim_project != 0 {
        set_project((*p_ptr).tim_project as libc::c_int - 1 as libc::c_int,
                    (*p_ptr).tim_project_gf, (*p_ptr).tim_project_dam,
                    (*p_ptr).tim_project_rad, (*p_ptr).tim_project_flag);
    }
    /* Timed roots */
    if (*p_ptr).tim_roots != 0 {
        set_roots((*p_ptr).tim_roots as libc::c_int - 1 as libc::c_int,
                  (*p_ptr).tim_roots_ac, (*p_ptr).tim_roots_dam);
    }
    /* Timed breath */
    if (*p_ptr).tim_water_breath != 0 {
        set_tim_breath((*p_ptr).tim_water_breath as libc::c_int -
                           1 as libc::c_int, 0 as libc::c_int as bool_);
    }
    if (*p_ptr).tim_magic_breath != 0 {
        set_tim_breath((*p_ptr).tim_magic_breath as libc::c_int -
                           1 as libc::c_int, 1 as libc::c_int as bool_);
    }
    /* Timed regen */
    if (*p_ptr).tim_regen != 0 {
        set_tim_regen((*p_ptr).tim_regen as libc::c_int - 1 as libc::c_int,
                      (*p_ptr).tim_regen_pow as libc::c_int);
    }
    /* Timed Disrupt shield */
    if (*p_ptr).disrupt_shield != 0 {
        set_disrupt_shield((*p_ptr).disrupt_shield as libc::c_int -
                               1 as libc::c_int);
    }
    /* Timed Parasite */
    if (*p_ptr).parasite != 0 {
        set_parasite((*p_ptr).parasite as libc::c_int - 1 as libc::c_int,
                     (*p_ptr).parasite_r_idx as libc::c_int);
    }
    /* Timed Reflection */
    if (*p_ptr).tim_reflect != 0 {
        set_tim_reflect((*p_ptr).tim_reflect as libc::c_int -
                            1 as libc::c_int);
    }
    /* Timed Prob Travel */
    if (*p_ptr).prob_travel != 0 {
        set_prob_travel((*p_ptr).prob_travel as libc::c_int -
                            1 as libc::c_int);
    }
    /* Timed Time Resistance */
    if (*p_ptr).tim_res_time != 0 {
        set_tim_res_time((*p_ptr).tim_res_time as libc::c_int -
                             1 as libc::c_int);
    }
    /* Timed Levitation */
    if (*p_ptr).tim_ffall != 0 {
        set_tim_ffall((*p_ptr).tim_ffall as libc::c_int - 1 as libc::c_int);
    }
    if (*p_ptr).tim_fly != 0 {
        set_tim_fly((*p_ptr).tim_fly as libc::c_int - 1 as libc::c_int);
    }
    /* Thunderstorm */
    if (*p_ptr).tim_thunder != 0 {
        let mut dam: libc::c_int =
            damroll((*p_ptr).tim_thunder_p1, (*p_ptr).tim_thunder_p2);
        let mut i_0: libc::c_int = 0;
        let mut tries_0: libc::c_int = 600 as libc::c_int;
        let mut m_ptr_0: *mut monster_type = 0 as *mut monster_type;
        while tries_0 != 0 {
            /* Access the monster */
            i_0 =
                1 as libc::c_int +
                    (Rand_div as
                         unsafe extern "C" fn(_: s32b)
                             ->
                                 s32b)(1 as libc::c_int +
                                           (m_max as libc::c_int -
                                                1 as libc::c_int) -
                                           1 as libc::c_int);
            m_ptr_0 = &mut *m_list.offset(i_0 as isize) as *mut monster_type;
            tries_0 -= 1;
            /* Ignore "dead" monsters */
            if (*m_ptr_0).r_idx == 0 { continue ; }
            /* Cant see ? cant hit */
            if los((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                   (*m_ptr_0).fy as libc::c_int, (*m_ptr_0).fx as libc::c_int)
                   == 0 {
                continue ;
            }
            /* Do not hurt friends! */
            if !(is_friend(m_ptr_0) >= 0 as libc::c_int) { break ; }
        }
        if tries_0 != 0 {
            let mut m_name: [libc::c_char; 80] = [0; 80];
            monster_desc(m_name.as_mut_ptr(), m_ptr_0, 0 as libc::c_int);
            msg_format(b"Lightning strikes %s.\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr());
            project(0 as libc::c_int, 0 as libc::c_int,
                    (*m_ptr_0).fy as libc::c_int,
                    (*m_ptr_0).fx as libc::c_int, dam / 3 as libc::c_int,
                    1 as libc::c_int,
                    0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x80 as libc::c_int);
            project(0 as libc::c_int, 0 as libc::c_int,
                    (*m_ptr_0).fy as libc::c_int,
                    (*m_ptr_0).fx as libc::c_int, dam / 3 as libc::c_int,
                    15 as libc::c_int,
                    0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x80 as libc::c_int);
            project(0 as libc::c_int, 0 as libc::c_int,
                    (*m_ptr_0).fy as libc::c_int,
                    (*m_ptr_0).fx as libc::c_int, dam / 3 as libc::c_int,
                    21 as libc::c_int,
                    0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x80 as libc::c_int);
        }
        set_tim_thunder((*p_ptr).tim_thunder as libc::c_int -
                            1 as libc::c_int,
                        (*p_ptr).tim_thunder_p1 as libc::c_int,
                        (*p_ptr).tim_thunder_p2 as libc::c_int);
    }
    /* Poisonned hands */
    if (*p_ptr).tim_poison != 0 {
        set_poison((*p_ptr).tim_poison as libc::c_int - 1 as libc::c_int);
    }
    /* Timed Fire Aura */
    if (*p_ptr).tim_fire_aura != 0 {
        set_tim_fire_aura((*p_ptr).tim_fire_aura as libc::c_int -
                              1 as libc::c_int);
    }
    /* Brightness */
    if (*p_ptr).tim_lite != 0 {
        set_lite((*p_ptr).tim_lite as libc::c_int - 1 as libc::c_int);
    }
    /* Blindness */
    if (*p_ptr).blind != 0 {
        set_blind((*p_ptr).blind as libc::c_int - 1 as libc::c_int);
    }
    /* Timed no_breeds */
    if no_breeds != 0 {
        set_no_breeders(no_breeds as libc::c_int - 1 as libc::c_int);
    }
    /* Timed mimic */
    if (*p_ptr).tim_mimic != 0 {
        set_mimic((*p_ptr).tim_mimic as libc::c_int - 1 as libc::c_int,
                  (*p_ptr).mimic_form as libc::c_int,
                  (*p_ptr).mimic_level as libc::c_int);
    }
    /* Timed special move commands */
    if (*p_ptr).immov_cntr != 0 { (*p_ptr).immov_cntr -= 1 }
    /* Timed invisibility */
    if (*p_ptr).tim_invisible != 0 {
        set_invis((*p_ptr).tim_invisible as libc::c_int - 1 as libc::c_int,
                  (*p_ptr).tim_inv_pow as libc::c_int);
    }
    /* Times see-invisible */
    if (*p_ptr).tim_invis != 0 {
        set_tim_invis((*p_ptr).tim_invis as libc::c_int - 1 as libc::c_int);
    }
    if multi_rew != 0 { multi_rew = 0 as libc::c_int as bool_ }
    /* Timed esp */
    if (*p_ptr).tim_esp != 0 {
        set_tim_esp((*p_ptr).tim_esp as libc::c_int - 1 as libc::c_int);
    }
    /* Timed infra-vision */
    if (*p_ptr).tim_infra != 0 {
        set_tim_infra((*p_ptr).tim_infra as libc::c_int - 1 as libc::c_int);
    }
    /* Paralysis */
    if (*p_ptr).paralyzed != 0 {
        set_paralyzed((*p_ptr).paralyzed as libc::c_int - 1 as libc::c_int);
    }
    /* Confusion */
    if (*p_ptr).confused != 0 {
        set_confused((*p_ptr).confused as libc::c_int - 1 as libc::c_int);
    }
    /* Afraid */
    if (*p_ptr).afraid != 0 {
        set_afraid((*p_ptr).afraid as libc::c_int - 1 as libc::c_int);
    }
    /* Fast */
    if (*p_ptr).fast != 0 {
        set_fast((*p_ptr).fast as libc::c_int - 1 as libc::c_int,
                 (*p_ptr).speed_factor as libc::c_int);
    }
    /* Light speed */
    if (*p_ptr).lightspeed != 0 {
        set_light_speed((*p_ptr).lightspeed as libc::c_int -
                            1 as libc::c_int);
    }
    /* Slow */
    if (*p_ptr).slow != 0 {
        set_slow((*p_ptr).slow as libc::c_int - 1 as libc::c_int);
    }
    /* Protection from evil */
    if (*p_ptr).protevil != 0 {
        set_protevil((*p_ptr).protevil as libc::c_int - 1 as libc::c_int);
    }
    /* Protection from good */
    if (*p_ptr).protgood != 0 {
        set_protgood((*p_ptr).protgood as libc::c_int - 1 as libc::c_int);
    }
    /* Protection from undead */
    if (*p_ptr).protundead != 0 {
        set_protundead((*p_ptr).protundead as libc::c_int - 1 as libc::c_int);
    }
    /* Invulnerability */
    if (*p_ptr).invuln != 0 {
        set_invuln((*p_ptr).invuln as libc::c_int - 1 as libc::c_int);
    }
    /* Wraith form */
    if (*p_ptr).tim_wraith != 0 {
        set_shadow((*p_ptr).tim_wraith as libc::c_int - 1 as libc::c_int);
    }
    /* Heroism */
    if (*p_ptr).hero != 0 {
        set_hero((*p_ptr).hero as libc::c_int - 1 as libc::c_int);
    }
    /* Super Heroism */
    if (*p_ptr).shero != 0 {
        set_shero((*p_ptr).shero as libc::c_int - 1 as libc::c_int);
    }
    /* Blessed */
    if (*p_ptr).blessed != 0 {
        set_blessed((*p_ptr).blessed as libc::c_int - 1 as libc::c_int);
    }
    /* Shield */
    if (*p_ptr).shield != 0 {
        set_shield((*p_ptr).shield as libc::c_int - 1 as libc::c_int,
                   (*p_ptr).shield_power as libc::c_int, (*p_ptr).shield_opt,
                   (*p_ptr).shield_power_opt, (*p_ptr).shield_power_opt2);
    }
    /* Oppose Acid */
    if (*p_ptr).oppose_acid != 0 {
        set_oppose_acid((*p_ptr).oppose_acid as libc::c_int -
                            1 as libc::c_int);
    }
    /* Oppose Lightning */
    if (*p_ptr).oppose_elec != 0 {
        set_oppose_elec((*p_ptr).oppose_elec as libc::c_int -
                            1 as libc::c_int);
    }
    /* Oppose Fire */
    if (*p_ptr).oppose_fire != 0 {
        set_oppose_fire((*p_ptr).oppose_fire as libc::c_int -
                            1 as libc::c_int);
    }
    /* Oppose Cold */
    if (*p_ptr).oppose_cold != 0 {
        set_oppose_cold((*p_ptr).oppose_cold as libc::c_int -
                            1 as libc::c_int);
    }
    /* Oppose Poison */
    if (*p_ptr).oppose_pois != 0 {
        set_oppose_pois((*p_ptr).oppose_pois as libc::c_int -
                            1 as libc::c_int);
    }
    /* Oppose Light & Dark */
    if (*p_ptr).oppose_ld != 0 {
        set_oppose_ld((*p_ptr).oppose_ld as libc::c_int - 1 as libc::c_int);
    }
    /* Oppose Chaos & Confusion */
    if (*p_ptr).oppose_cc != 0 {
        set_oppose_cc((*p_ptr).oppose_cc as libc::c_int - 1 as libc::c_int);
    }
    /* Oppose Sound & Shards */
    if (*p_ptr).oppose_ss != 0 {
        set_oppose_ss((*p_ptr).oppose_ss as libc::c_int - 1 as libc::c_int);
    }
    /* Oppose Nexus */
    if (*p_ptr).oppose_nex != 0 {
        set_oppose_nex((*p_ptr).oppose_nex as libc::c_int - 1 as libc::c_int);
    }
    /* Mental Barrier */
    if (*p_ptr).tim_mental_barrier != 0 {
        set_mental_barrier((*p_ptr).tim_mental_barrier as libc::c_int -
                               1 as libc::c_int);
    }
    /* The rush */
    if (*p_ptr).rush != 0 {
        set_rush((*p_ptr).rush as libc::c_int - 1 as libc::c_int);
    }
    /* Timed mimicry */
    if get_skill(32 as libc::c_int) != 0 {
        /* Extract the value and the flags */
        let mut value: u32b = (*p_ptr).mimic_extra >> 16 as libc::c_int;
        let mut att: u32b =
            (*p_ptr).mimic_extra & 0xffff as libc::c_int as libc::c_uint;
        if att & 0x20 as libc::c_int as libc::c_uint != 0 ||
               att & 0x80 as libc::c_int as libc::c_uint != 0 ||
               att & 0x40 as libc::c_int as libc::c_uint != 0 {
            value = value.wrapping_sub(1);
            if value == 0 {
                if att & 0x20 as libc::c_int as libc::c_uint != 0 {
                    msg_print(b"You lose your extra pair of legs.\x00" as
                                  *const u8 as *const libc::c_char);
                }
                if att & 0x40 as libc::c_int as libc::c_uint != 0 {
                    msg_print(b"You lose your extra pair of arms.\x00" as
                                  *const u8 as *const libc::c_char);
                }
                if att & 0x80 as libc::c_int as libc::c_uint != 0 {
                    msg_print(b"You lose your affinity for walls.\x00" as
                                  *const u8 as *const libc::c_char);
                }
                att &= !(0x40 as libc::c_int) as libc::c_uint;
                att &= !(0x20 as libc::c_int) as libc::c_uint;
                att &= !(0x80 as libc::c_int) as libc::c_uint;
                if disturb_state != 0 {
                    disturb(0 as libc::c_int, 0 as libc::c_int);
                }
            }
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long | 0x4 as libc::c_long) as
                    u32b;
            (*p_ptr).mimic_extra =
                att.wrapping_add(value << 16 as libc::c_int)
        }
    }
    /* ** Poison and Stun and Cut ***/
    /* Poison */
    if (*p_ptr).poisoned != 0 {
        let mut adjust: libc::c_int =
            *adj_con_fix.as_mut_ptr().offset((*p_ptr).stat_ind[4 as
                                                                   libc::c_int
                                                                   as usize]
                                                 as isize) as libc::c_int +
                1 as libc::c_int;
        /* Apply some healing */
        set_poisoned((*p_ptr).poisoned as libc::c_int - adjust);
    }
    /* Stun */
    if (*p_ptr).stun != 0 {
        let mut adjust_0: libc::c_int =
            *adj_con_fix.as_mut_ptr().offset((*p_ptr).stat_ind[4 as
                                                                   libc::c_int
                                                                   as usize]
                                                 as isize) as libc::c_int +
                1 as libc::c_int;
        /* Apply some healing */
        set_stun((*p_ptr).stun as libc::c_int - adjust_0);
    }
    /* Cut */
    if (*p_ptr).cut != 0 {
        let mut adjust_1: libc::c_int =
            *adj_con_fix.as_mut_ptr().offset((*p_ptr).stat_ind[4 as
                                                                   libc::c_int
                                                                   as usize]
                                                 as isize) as libc::c_int +
                1 as libc::c_int;
        /* Hack -- Truly "mortal" wound */
        if (*p_ptr).cut as libc::c_int > 1000 as libc::c_int {
            adjust_1 = 0 as libc::c_int
        }
        /* Apply some healing */
        set_cut((*p_ptr).cut as libc::c_int - adjust_1);
    }
    /* Hack - damage done by the dungeon -SC- */
    if dun_level as libc::c_int != 0 as libc::c_int &&
           (*d_ptr).d_frequency[0 as libc::c_int as usize] != 0 as libc::c_int
       {
        let mut i_1: libc::c_int = 0;
        let mut j_0: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        /* Apply damage to every grid in the dungeon */
        i_1 = 0 as libc::c_int;
        while i_1 < 4 as libc::c_int {
            /* Check the frequency */
            if !((*d_ptr).d_frequency[i_1 as usize] == 0 as libc::c_int) {
                if turn % (*d_ptr).d_frequency[i_1 as usize] ==
                       0 as libc::c_int &&
                       ((*d_ptr).d_side[i_1 as usize] != 0 as libc::c_int ||
                            (*d_ptr).d_dice[i_1 as usize] != 0 as libc::c_int)
                   {
                    j_0 = 0 as libc::c_int;
                    while j_0 < cur_hgt as libc::c_int - 1 as libc::c_int {
                        let mut current_block_573: u64;
                        k = 0 as libc::c_int;
                        while k < cur_wid as libc::c_int - 1 as libc::c_int {
                            let mut l: libc::c_int = 0;
                            let mut dam_0: libc::c_int = 0 as libc::c_int;
                            if dungeon_flags1 as libc::c_long &
                                   0x200000 as libc::c_long == 0 {
                                /* If the grid is empty, skip it */
                                if (*cave[j_0 as
                                              usize].offset(k as isize)).o_idx
                                       as libc::c_int == 0 as libc::c_int &&
                                       (j_0 != (*p_ptr).py as libc::c_int &&
                                            i_1 != (*p_ptr).px as libc::c_int)
                                   {
                                    current_block_573 = 6585354869280258233;
                                } else {
                                    current_block_573 = 4403808156861475175;
                                }
                            } else {
                                current_block_573 = 4403808156861475175;
                            }
                            match current_block_573 {
                                4403808156861475175 =>
                                /* Let's not hurt poor monsters */
                                {
                                    if !((*cave[j_0 as
                                                    usize].offset(k as
                                                                      isize)).m_idx
                                             != 0) {
                                        /* Roll damage */
                                        l = 0 as libc::c_int;
                                        while l <
                                                  (*d_ptr).d_dice[i_1 as
                                                                      usize] {
                                            dam_0 +=
                                                Rand_div((*d_ptr).d_side[i_1
                                                                             as
                                                                             usize])
                                                    + 1 as libc::c_int;
                                            l += 1
                                        }
                                        /* Apply damage */
                                        project(-(100 as libc::c_int),
                                                0 as libc::c_int, j_0, k,
                                                dam_0,
                                                (*d_ptr).d_type[i_1 as usize],
                                                0x40 as libc::c_int |
                                                    0x20 as libc::c_int |
                                                    0x80 as libc::c_int);
                                    }
                                }
                                _ => { }
                            }
                            k += 1
                        }
                        j_0 += 1
                    }
                }
            }
            i_1 += 1
        }
    }
    /* handle spell effects */
    if (*p_ptr).wild_mode == 0 {
        /*
		 * I noticed significant performance degrade after the introduction
		 * of staying spell effects. I believe serious optimisation effort
		 * is required before another release.
		 *
		 * More important is to fix that display weirdness...
		 *
		 * It seems that the game never expects that monster deaths and
		 * terrain feature changes should happen here... Moving these
		 * to process_player() [before resting code, with "every 10 game turn"
		 * 'if'] may or may not fix the problem... -- pelpel to DG
		 */
        j = 0 as libc::c_int;
        while j < cur_hgt as libc::c_int - 1 as libc::c_int {
            i = 0 as libc::c_int;
            while i < cur_wid as libc::c_int - 1 as libc::c_int {
                let mut e: libc::c_int =
                    (*cave[j as usize].offset(i as isize)).effect as
                        libc::c_int;
                if e != 0 {
                    let mut e_ptr: *mut effect_type =
                        &mut *effects.as_mut_ptr().offset(e as isize) as
                            *mut effect_type;
                    if (*e_ptr).time != 0 {
                        /* Apply damage */
                        project(0 as libc::c_int, 0 as libc::c_int, j, i,
                                (*e_ptr).dam as libc::c_int,
                                (*e_ptr).type_0 as libc::c_int,
                                0x40 as libc::c_int | 0x20 as libc::c_int |
                                    0x80 as libc::c_int);
                    } else {
                        (*cave[j as usize].offset(i as isize)).effect =
                            0 as libc::c_int as s16b
                    }
                    if (*e_ptr).flags & 0x1 as libc::c_int as libc::c_uint !=
                           0 &&
                           (*e_ptr).flags & 0x2 as libc::c_int as libc::c_uint
                               == 0 {
                        if distance((*e_ptr).cy as libc::c_int,
                                    (*e_ptr).cx as libc::c_int, j, i) <
                               (*e_ptr).rad as libc::c_int - 1 as libc::c_int
                           {
                            (*cave[j as usize].offset(i as isize)).effect =
                                0 as libc::c_int as s16b
                        }
                    } else if (*e_ptr).flags &
                                  0x4 as libc::c_int as libc::c_uint != 0 &&
                                  (*e_ptr).flags &
                                      0x2 as libc::c_int as libc::c_uint == 0
                     {
                        (*cave[j as usize].offset(i as isize)).effect =
                            0 as libc::c_int as s16b
                    }
                    lite_spot(j, i);
                }
                i += 1
            }
            j += 1
        }
        /* Reduce & handle effects */
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            /* Skip empty slots */
            if !(effects[i as usize].time as libc::c_int == 0 as libc::c_int)
               {
                /* Reduce duration */
                effects[i as usize].time -= 1;
                /* Creates a "wave" effect*/
                if effects[i as usize].flags &
                       0x1 as libc::c_int as libc::c_uint != 0 {
                    let mut e_ptr_0: *mut effect_type =
                        &mut *effects.as_mut_ptr().offset(i as isize) as
                            *mut effect_type;
                    let mut x_0: libc::c_int = 0;
                    let mut y_0: libc::c_int = 0;
                    let mut z_1: libc::c_int = 0;
                    (*e_ptr_0).rad += 1;
                    /* What a frelling ugly line of ifs ... */
                    if effects[i as usize].flags &
                           0x200 as libc::c_int as libc::c_uint != 0 {
                        y_0 =
                            (*e_ptr_0).cy as libc::c_int -
                                (*e_ptr_0).rad as libc::c_int;
                        z_1 = 0 as libc::c_int;
                        while y_0 <= (*e_ptr_0).cy as libc::c_int {
                            x_0 =
                                (*e_ptr_0).cx as libc::c_int -
                                    ((*e_ptr_0).rad as libc::c_int - z_1);
                            while x_0 <=
                                      (*e_ptr_0).cx as libc::c_int +
                                          ((*e_ptr_0).rad as libc::c_int -
                                               z_1) {
                                if y_0 > 0 as libc::c_int &&
                                       x_0 > 0 as libc::c_int &&
                                       y_0 <
                                           cur_hgt as libc::c_int -
                                               1 as libc::c_int &&
                                       x_0 <
                                           cur_wid as libc::c_int -
                                               1 as libc::c_int {
                                    if los((*e_ptr_0).cy as libc::c_int,
                                           (*e_ptr_0).cx as libc::c_int, y_0,
                                           x_0) as libc::c_int != 0 &&
                                           distance((*e_ptr_0).cy as
                                                        libc::c_int,
                                                    (*e_ptr_0).cx as
                                                        libc::c_int, y_0, x_0)
                                               ==
                                               (*e_ptr_0).rad as libc::c_int {
                                        (*cave[y_0 as
                                                   usize].offset(x_0 as
                                                                     isize)).effect
                                            = i as s16b
                                    }
                                }
                                x_0 += 1
                            }
                            y_0 += 1;
                            z_1 += 1
                        }
                    } else if effects[i as usize].flags &
                                  0x10 as libc::c_int as libc::c_uint != 0 {
                        y_0 = (*e_ptr_0).cy as libc::c_int;
                        z_1 = (*e_ptr_0).rad as libc::c_int;
                        while y_0 <=
                                  (*e_ptr_0).cy as libc::c_int +
                                      (*e_ptr_0).rad as libc::c_int {
                            x_0 =
                                (*e_ptr_0).cx as libc::c_int -
                                    ((*e_ptr_0).rad as libc::c_int - z_1);
                            while x_0 <=
                                      (*e_ptr_0).cx as libc::c_int +
                                          ((*e_ptr_0).rad as libc::c_int -
                                               z_1) {
                                if y_0 > 0 as libc::c_int &&
                                       x_0 > 0 as libc::c_int &&
                                       y_0 <
                                           cur_hgt as libc::c_int -
                                               1 as libc::c_int &&
                                       x_0 <
                                           cur_wid as libc::c_int -
                                               1 as libc::c_int {
                                    if los((*e_ptr_0).cy as libc::c_int,
                                           (*e_ptr_0).cx as libc::c_int, y_0,
                                           x_0) as libc::c_int != 0 &&
                                           distance((*e_ptr_0).cy as
                                                        libc::c_int,
                                                    (*e_ptr_0).cx as
                                                        libc::c_int, y_0, x_0)
                                               ==
                                               (*e_ptr_0).rad as libc::c_int {
                                        (*cave[y_0 as
                                                   usize].offset(x_0 as
                                                                     isize)).effect
                                            = i as s16b
                                    }
                                }
                                x_0 += 1
                            }
                            y_0 += 1;
                            z_1 -= 1
                        }
                    } else if effects[i as usize].flags &
                                  0x80 as libc::c_int as libc::c_uint != 0 {
                        x_0 = (*e_ptr_0).cx as libc::c_int;
                        z_1 = (*e_ptr_0).rad as libc::c_int;
                        while x_0 <=
                                  (*e_ptr_0).cx as libc::c_int +
                                      (*e_ptr_0).rad as libc::c_int {
                            y_0 =
                                (*e_ptr_0).cy as libc::c_int -
                                    ((*e_ptr_0).rad as libc::c_int - z_1);
                            while y_0 <=
                                      (*e_ptr_0).cy as libc::c_int +
                                          ((*e_ptr_0).rad as libc::c_int -
                                               z_1) {
                                if y_0 > 0 as libc::c_int &&
                                       x_0 > 0 as libc::c_int &&
                                       y_0 <
                                           cur_hgt as libc::c_int -
                                               1 as libc::c_int &&
                                       x_0 <
                                           cur_wid as libc::c_int -
                                               1 as libc::c_int {
                                    if los((*e_ptr_0).cy as libc::c_int,
                                           (*e_ptr_0).cx as libc::c_int, y_0,
                                           x_0) as libc::c_int != 0 &&
                                           distance((*e_ptr_0).cy as
                                                        libc::c_int,
                                                    (*e_ptr_0).cx as
                                                        libc::c_int, y_0, x_0)
                                               ==
                                               (*e_ptr_0).rad as libc::c_int {
                                        (*cave[y_0 as
                                                   usize].offset(x_0 as
                                                                     isize)).effect
                                            = i as s16b
                                    }
                                }
                                y_0 += 1
                            }
                            x_0 += 1;
                            z_1 -= 1
                        }
                    } else if effects[i as usize].flags &
                                  0x40 as libc::c_int as libc::c_uint != 0 {
                        x_0 =
                            (*e_ptr_0).cx as libc::c_int -
                                (*e_ptr_0).rad as libc::c_int;
                        z_1 = 0 as libc::c_int;
                        while x_0 <= (*e_ptr_0).cx as libc::c_int {
                            y_0 =
                                (*e_ptr_0).cy as libc::c_int -
                                    ((*e_ptr_0).rad as libc::c_int - z_1);
                            while y_0 <=
                                      (*e_ptr_0).cy as libc::c_int +
                                          ((*e_ptr_0).rad as libc::c_int -
                                               z_1) {
                                if y_0 > 0 as libc::c_int &&
                                       x_0 > 0 as libc::c_int &&
                                       y_0 <
                                           cur_hgt as libc::c_int -
                                               1 as libc::c_int &&
                                       x_0 <
                                           cur_wid as libc::c_int -
                                               1 as libc::c_int {
                                    if los((*e_ptr_0).cy as libc::c_int,
                                           (*e_ptr_0).cx as libc::c_int, y_0,
                                           x_0) as libc::c_int != 0 &&
                                           distance((*e_ptr_0).cy as
                                                        libc::c_int,
                                                    (*e_ptr_0).cx as
                                                        libc::c_int, y_0, x_0)
                                               ==
                                               (*e_ptr_0).rad as libc::c_int {
                                        (*cave[y_0 as
                                                   usize].offset(x_0 as
                                                                     isize)).effect
                                            = i as s16b
                                    }
                                }
                                y_0 += 1
                            }
                            x_0 += 1;
                            z_1 += 1
                        }
                    } else if effects[i as usize].flags &
                                  0x400 as libc::c_int as libc::c_uint != 0 {
                        y_0 =
                            (*e_ptr_0).cy as libc::c_int -
                                (*e_ptr_0).rad as libc::c_int;
                        while y_0 <= (*e_ptr_0).cy as libc::c_int {
                            x_0 = (*e_ptr_0).cx as libc::c_int;
                            while x_0 <=
                                      (*e_ptr_0).cx as libc::c_int +
                                          (*e_ptr_0).rad as libc::c_int {
                                if y_0 > 0 as libc::c_int &&
                                       x_0 > 0 as libc::c_int &&
                                       y_0 <
                                           cur_hgt as libc::c_int -
                                               1 as libc::c_int &&
                                       x_0 <
                                           cur_wid as libc::c_int -
                                               1 as libc::c_int {
                                    if los((*e_ptr_0).cy as libc::c_int,
                                           (*e_ptr_0).cx as libc::c_int, y_0,
                                           x_0) as libc::c_int != 0 &&
                                           distance((*e_ptr_0).cy as
                                                        libc::c_int,
                                                    (*e_ptr_0).cx as
                                                        libc::c_int, y_0, x_0)
                                               ==
                                               (*e_ptr_0).rad as libc::c_int {
                                        (*cave[y_0 as
                                                   usize].offset(x_0 as
                                                                     isize)).effect
                                            = i as s16b
                                    }
                                }
                                x_0 += 1
                            }
                            y_0 += 1
                        }
                    } else if effects[i as usize].flags &
                                  0x8 as libc::c_int as libc::c_uint != 0 {
                        y_0 = (*e_ptr_0).cy as libc::c_int;
                        while y_0 <=
                                  (*e_ptr_0).cy as libc::c_int +
                                      (*e_ptr_0).rad as libc::c_int {
                            x_0 =
                                (*e_ptr_0).cx as libc::c_int -
                                    (*e_ptr_0).rad as libc::c_int;
                            while x_0 <= (*e_ptr_0).cx as libc::c_int {
                                if y_0 > 0 as libc::c_int &&
                                       x_0 > 0 as libc::c_int &&
                                       y_0 <
                                           cur_hgt as libc::c_int -
                                               1 as libc::c_int &&
                                       x_0 <
                                           cur_wid as libc::c_int -
                                               1 as libc::c_int {
                                    if los((*e_ptr_0).cy as libc::c_int,
                                           (*e_ptr_0).cx as libc::c_int, y_0,
                                           x_0) as libc::c_int != 0 &&
                                           distance((*e_ptr_0).cy as
                                                        libc::c_int,
                                                    (*e_ptr_0).cx as
                                                        libc::c_int, y_0, x_0)
                                               ==
                                               (*e_ptr_0).rad as libc::c_int {
                                        (*cave[y_0 as
                                                   usize].offset(x_0 as
                                                                     isize)).effect
                                            = i as s16b
                                    }
                                }
                                x_0 += 1
                            }
                            y_0 += 1
                        }
                    } else if effects[i as usize].flags &
                                  0x100 as libc::c_int as libc::c_uint != 0 {
                        y_0 =
                            (*e_ptr_0).cy as libc::c_int -
                                (*e_ptr_0).rad as libc::c_int;
                        while y_0 <= (*e_ptr_0).cy as libc::c_int {
                            x_0 =
                                (*e_ptr_0).cx as libc::c_int -
                                    (*e_ptr_0).rad as libc::c_int;
                            while x_0 <= (*e_ptr_0).cx as libc::c_int {
                                if y_0 > 0 as libc::c_int &&
                                       x_0 > 0 as libc::c_int &&
                                       y_0 <
                                           cur_hgt as libc::c_int -
                                               1 as libc::c_int &&
                                       x_0 <
                                           cur_wid as libc::c_int -
                                               1 as libc::c_int {
                                    if los((*e_ptr_0).cy as libc::c_int,
                                           (*e_ptr_0).cx as libc::c_int, y_0,
                                           x_0) as libc::c_int != 0 &&
                                           distance((*e_ptr_0).cy as
                                                        libc::c_int,
                                                    (*e_ptr_0).cx as
                                                        libc::c_int, y_0, x_0)
                                               ==
                                               (*e_ptr_0).rad as libc::c_int {
                                        (*cave[y_0 as
                                                   usize].offset(x_0 as
                                                                     isize)).effect
                                            = i as s16b
                                    }
                                }
                                x_0 += 1
                            }
                            y_0 += 1
                        }
                    } else if effects[i as usize].flags &
                                  0x20 as libc::c_int as libc::c_uint != 0 {
                        y_0 = (*e_ptr_0).cy as libc::c_int;
                        while y_0 <=
                                  (*e_ptr_0).cy as libc::c_int +
                                      (*e_ptr_0).rad as libc::c_int {
                            x_0 = (*e_ptr_0).cx as libc::c_int;
                            while x_0 <=
                                      (*e_ptr_0).cx as libc::c_int +
                                          (*e_ptr_0).rad as libc::c_int {
                                if y_0 > 0 as libc::c_int &&
                                       x_0 > 0 as libc::c_int &&
                                       y_0 <
                                           cur_hgt as libc::c_int -
                                               1 as libc::c_int &&
                                       x_0 <
                                           cur_wid as libc::c_int -
                                               1 as libc::c_int {
                                    if los((*e_ptr_0).cy as libc::c_int,
                                           (*e_ptr_0).cx as libc::c_int, y_0,
                                           x_0) as libc::c_int != 0 &&
                                           distance((*e_ptr_0).cy as
                                                        libc::c_int,
                                                    (*e_ptr_0).cx as
                                                        libc::c_int, y_0, x_0)
                                               ==
                                               (*e_ptr_0).rad as libc::c_int {
                                        (*cave[y_0 as
                                                   usize].offset(x_0 as
                                                                     isize)).effect
                                            = i as s16b
                                    }
                                }
                                x_0 += 1
                            }
                            y_0 += 1
                        }
                    } else {
                        y_0 =
                            (*e_ptr_0).cy as libc::c_int -
                                (*e_ptr_0).rad as libc::c_int;
                        while y_0 <=
                                  (*e_ptr_0).cy as libc::c_int +
                                      (*e_ptr_0).rad as libc::c_int {
                            x_0 =
                                (*e_ptr_0).cx as libc::c_int -
                                    (*e_ptr_0).rad as libc::c_int;
                            while x_0 <=
                                      (*e_ptr_0).cx as libc::c_int +
                                          (*e_ptr_0).rad as libc::c_int {
                                if y_0 > 0 as libc::c_int &&
                                       x_0 > 0 as libc::c_int &&
                                       y_0 <
                                           cur_hgt as libc::c_int -
                                               1 as libc::c_int &&
                                       x_0 <
                                           cur_wid as libc::c_int -
                                               1 as libc::c_int {
                                    /* This is *slow* -- pelpel */
                                    if los((*e_ptr_0).cy as libc::c_int,
                                           (*e_ptr_0).cx as libc::c_int, y_0,
                                           x_0) as libc::c_int != 0 &&
                                           distance((*e_ptr_0).cy as
                                                        libc::c_int,
                                                    (*e_ptr_0).cx as
                                                        libc::c_int, y_0, x_0)
                                               ==
                                               (*e_ptr_0).rad as libc::c_int {
                                        (*cave[y_0 as
                                                   usize].offset(x_0 as
                                                                     isize)).effect
                                            = i as s16b
                                    }
                                }
                                x_0 += 1
                            }
                            y_0 += 1
                        }
                    }
                } else if effects[i as usize].flags &
                              0x4 as libc::c_int as libc::c_uint != 0 {
                    let mut e_ptr_1: *mut effect_type =
                        &mut *effects.as_mut_ptr().offset(i as isize) as
                            *mut effect_type;
                    let mut x_1: libc::c_int = 0;
                    let mut y_1: libc::c_int = 0;
                    (*e_ptr_1).cy = (*p_ptr).py;
                    (*e_ptr_1).cx = (*p_ptr).px;
                    y_1 =
                        (*e_ptr_1).cy as libc::c_int -
                            (*e_ptr_1).rad as libc::c_int;
                    while y_1 <=
                              (*e_ptr_1).cy as libc::c_int +
                                  (*e_ptr_1).rad as libc::c_int {
                        x_1 =
                            (*e_ptr_1).cx as libc::c_int -
                                (*e_ptr_1).rad as libc::c_int;
                        while x_1 <=
                                  (*e_ptr_1).cx as libc::c_int +
                                      (*e_ptr_1).rad as libc::c_int {
                            if y_1 > 0 as libc::c_int &&
                                   x_1 > 0 as libc::c_int &&
                                   y_1 <
                                       cur_hgt as libc::c_int -
                                           1 as libc::c_int &&
                                   x_1 <
                                       cur_wid as libc::c_int -
                                           1 as libc::c_int {
                                if los((*e_ptr_1).cy as libc::c_int,
                                       (*e_ptr_1).cx as libc::c_int, y_1, x_1)
                                       as libc::c_int != 0 &&
                                       distance((*e_ptr_1).cy as libc::c_int,
                                                (*e_ptr_1).cx as libc::c_int,
                                                y_1, x_1) <=
                                           (*e_ptr_1).rad as libc::c_int {
                                    (*cave[y_1 as
                                               usize].offset(x_1 as
                                                                 isize)).effect
                                        = i as s16b;
                                    lite_spot(y_1, x_1);
                                }
                            }
                            x_1 += 1
                        }
                        y_1 += 1
                    }
                }
            }
            i += 1
        }
        apply_effect((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    }
    /* Creates a "storm" effect*/
    /* Arg cannot breath? */
    if dungeon_flags2 as libc::c_long & 0x800 as libc::c_long != 0 &&
           (*p_ptr).water_breath == 0 {
        cmsg_print(12 as libc::c_int as byte_hack,
                   b"You cannot breathe water!  You suffocate!\x00" as
                       *const u8 as *const libc::c_char);
        take_hit(damroll(3 as libc::c_int as s16b, (*p_ptr).lev),
                 b"suffocating\x00" as *const u8 as *const libc::c_char);
    }
    if dungeon_flags2 as libc::c_long & 0x400 as libc::c_long != 0 &&
           (*p_ptr).magical_breath == 0 {
        cmsg_print(12 as libc::c_int as byte_hack,
                   b"There is no air there!  You suffocate!\x00" as *const u8
                       as *const libc::c_char);
        take_hit(damroll(3 as libc::c_int as s16b, (*p_ptr).lev),
                 b"suffocating\x00" as *const u8 as *const libc::c_char);
    }
    /*
	 * Every 1500 turns, warn about any Black Breath not gotten from
	 * an equipped object, and stop any resting. -LM-
	 *
	 * It's apparent that someone has halved the frequency... -- pelpel
	 */
    if turn % 3000 as libc::c_int == 0 as libc::c_int &&
           (*p_ptr).black_breath as libc::c_int != 0 {
        let mut f1_0: u32b = 0;
        let mut f2_0: u32b = 0;
        let mut f3_0: u32b = 0;
        let mut f4_0: u32b = 0;
        let mut f5_0: u32b = 0;
        let mut be_silent: bool_ = 0 as libc::c_int as bool_;
        /* check all equipment for the Black Breath flag. */
        i = 24 as libc::c_int;
        while i < 52 as libc::c_int {
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                    *mut object_type;
            /* Skip non-objects */
            if !((*o_ptr).k_idx == 0) {
                /* Extract the item flags */
                object_flags(o_ptr, &mut f1_0, &mut f2_0, &mut f3_0,
                             &mut f4_0, &mut f5_0, &mut esp);
                /* No messages if object has the flag, to avoid annoyance. */
                if f4_0 as libc::c_long & 0x4 as libc::c_long != 0 {
                    be_silent = 1 as libc::c_int as bool_
                }
            }
            i += 1
        }
        /* If we are allowed to speak, warn and disturb. */
        if be_silent == 0 {
            cmsg_print(8 as libc::c_int as byte_hack,
                       b"The Black Breath saps your soul!\x00" as *const u8 as
                           *const libc::c_char);
            disturb(0 as libc::c_int, 0 as libc::c_int);
        }
    }
    /* ** Process Light ***/
    /* Check for light being wielded */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(36 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Burn some fuel in the current lite */
    if (*o_ptr).tval as libc::c_int == 39 as libc::c_int {
        /* Extract the item flags */
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        /* Hack -- Use some fuel */
        if f4 as libc::c_long & 0x10000000 as libc::c_long != 0 &&
               (*o_ptr).timeout as libc::c_int > 0 as libc::c_int {
            /* Decrease life-span */
            (*o_ptr).timeout -= 1;
            /* Hack -- notice interesting fuel steps */
            if ((*o_ptr).timeout as libc::c_int) < 100 as libc::c_int ||
                   (*o_ptr).timeout as libc::c_int % 100 as libc::c_int ==
                       0 as libc::c_int {
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long | 0x2 as libc::c_long) as
                        u32b
            }
            /* Hack -- Special treatment when blind */
            if (*p_ptr).blind != 0 {
                /* Hack -- save some light for later */
                if (*o_ptr).timeout as libc::c_int == 0 as libc::c_int {
                    (*o_ptr).timeout += 1
                }
            } else if ((*o_ptr).timeout as libc::c_int) < 1 as libc::c_int {
                disturb(0 as libc::c_int, 0 as libc::c_int);
                cmsg_print(11 as libc::c_int as byte_hack,
                           b"Your light has gone out!\x00" as *const u8 as
                               *const libc::c_char);
            } else if ((*o_ptr).timeout as libc::c_int) < 100 as libc::c_int
                          &&
                          (*o_ptr).timeout as libc::c_int % 10 as libc::c_int
                              == 0 as libc::c_int {
                if disturb_minor != 0 {
                    disturb(0 as libc::c_int, 0 as libc::c_int);
                }
                cmsg_print(11 as libc::c_int as byte_hack,
                           b"Your light is growing faint.\x00" as *const u8 as
                               *const libc::c_char);
            }
        }
    }
    /* The light is now out */
    /* The light is getting dim */
    /* Calculate torch radius */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as u32b;
    /* ** Process Inventory ***/
    /*
	 * Handle experience draining.  In Oangband, the effect is worse,
	 * especially for high-level characters.  As per Tolkien, hobbits
	 * are resistant.
	 */
    if (*p_ptr).black_breath != 0 {
        let mut chance: byte_hack = 0 as libc::c_int as byte_hack;
        let mut plev: libc::c_int = (*p_ptr).lev as libc::c_int;
        if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                (*spp_ptr).flags1) as libc::c_long & 0x4 as libc::c_long != 0
           {
            chance = 2 as libc::c_int as byte_hack
        } else { chance = 5 as libc::c_int as byte_hack }
        if Rand_div(100 as libc::c_int) < chance as libc::c_int &&
               (*p_ptr).exp > 0 as libc::c_int {
            (*p_ptr).exp -= 1 as libc::c_int + plev / 5 as libc::c_int;
            (*p_ptr).max_exp -= 1 as libc::c_int + plev / 5 as libc::c_int;
            do_dec_stat(Rand_div(6 as libc::c_int), 2 as libc::c_int);
            check_experience();
        }
    }
    /* Drain Mana */
    if (*p_ptr).drain_mana as libc::c_int != 0 &&
           (*p_ptr).csp as libc::c_int != 0 {
        (*p_ptr).csp =
            ((*p_ptr).csp as libc::c_int - (*p_ptr).drain_mana as libc::c_int)
                as s16b;
        if Rand_div(100 as libc::c_int) < 30 as libc::c_int {
            (*p_ptr).csp =
                ((*p_ptr).csp as libc::c_int -
                     (*p_ptr).drain_mana as libc::c_int) as s16b
        }
        if ((*p_ptr).csp as libc::c_int) < 0 as libc::c_int {
            (*p_ptr).csp = 0 as libc::c_int as s16b;
            disturb(0 as libc::c_int, 0 as libc::c_int);
        }
        /* Redraw */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    }
    /* Partial summons drain mana */
    if (*p_ptr).maintain_sum != 0 {
        let mut oldcsp: u32b = (*p_ptr).csp as u32b;
        (*p_ptr).csp =
            ((*p_ptr).csp as
                 libc::c_uint).wrapping_sub((*p_ptr).maintain_sum.wrapping_div(10000
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint))
                as s16b as s16b;
        if ((*p_ptr).csp as libc::c_int) < 0 as libc::c_int {
            (*p_ptr).csp = 0 as libc::c_int as s16b;
            disturb(0 as libc::c_int, 0 as libc::c_int);
            (*p_ptr).maintain_sum = 0 as libc::c_int as u32b
        } else {
            /* Leave behind any fractional sp */
            (*p_ptr).maintain_sum =
                ((*p_ptr).maintain_sum as
                     libc::c_uint).wrapping_sub(oldcsp.wrapping_sub((*p_ptr).csp
                                                                        as
                                                                        libc::c_uint).wrapping_mul(10000
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint))
                    as u32b as u32b
        }
        /* Redraw */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    }
    /* Drain Hitpoints */
    if (*p_ptr).drain_life != 0 {
        let mut drain: libc::c_int =
            (*p_ptr).drain_life as libc::c_int +
                Rand_div((*p_ptr).mhp as libc::c_int / 100 as libc::c_int);
        (*p_ptr).chp =
            ((*p_ptr).chp as libc::c_int -
                 if drain < (*p_ptr).chp as libc::c_int {
                     drain
                 } else { (*p_ptr).chp as libc::c_int }) as s16b;
        if (*p_ptr).chp as libc::c_int == 0 as libc::c_int {
            disturb(0 as libc::c_int, 0 as libc::c_int);
        }
        /* Redraw */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    }
    /* Handle experience draining */
    if (*p_ptr).exp_drain != 0 {
        if Rand_div(100 as libc::c_int) < 10 as libc::c_int &&
               (*p_ptr).exp > 0 as libc::c_int {
            (*p_ptr).exp -= 1;
            (*p_ptr).max_exp -= 1;
            check_experience();
        }
    }
    /* Process equipment */
    j = 0 as libc::c_int;
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        /* Get the object */
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        /* TY Curse */
        if f3 as libc::c_long & 0x80 as libc::c_long != 0 &&
               Rand_div(100 as libc::c_int) == 0 as libc::c_int {
            activate_ty_curse();
        }
        /* DG Curse */
        if f4 as libc::c_long & 0x20 as libc::c_long != 0 &&
               Rand_div(50 as libc::c_int) == 0 as libc::c_int {
            activate_dg_curse();
            /* The object recurse itself ! */
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                    byte_hack
        }
        /* Auto Curse */
        if f3 as libc::c_long & 0x4 as libc::c_long != 0 &&
               Rand_div(15 as libc::c_int) == 0 as libc::c_int {
            /* The object recurse itself ! */
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                    byte_hack
        }
        /*
		 * Hack: Uncursed teleporting items (e.g. Dragon Weapons)
		 * can actually be useful!
		 */
        if f3 as libc::c_long & 0x4000000 as libc::c_long != 0 &&
               Rand_div(100 as libc::c_int) < 1 as libc::c_int {
            if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 &&
                   (*p_ptr).anti_tele == 0 {
                disturb(0 as libc::c_int, 0 as libc::c_int);
                /* Teleport player */
                teleport_player(40 as libc::c_int);
            } else if !((*p_ptr).wild_mode as libc::c_int != 0 ||
                            (*o_ptr).note as libc::c_int != 0 &&
                                !strchr(quark_str((*o_ptr).note as s16b),
                                        '.' as i32).is_null()) {
                if get_check(b"Teleport? \x00" as *const u8 as
                                 *const libc::c_char) != 0 {
                    disturb(0 as libc::c_int, 0 as libc::c_int);
                    teleport_player(50 as libc::c_int);
                }
            }
        }
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Hack: Skip wielded lights that need fuel (already handled above) */
            if !(i == 36 as libc::c_int &&
                     (*o_ptr).tval as libc::c_int == 39 as libc::c_int &&
                     f4 as libc::c_long & 0x10000000 as libc::c_long != 0) {
                /* Recharge activatable objects */
                if (*o_ptr).timeout as libc::c_int > 0 as libc::c_int {
                    /* Recharge */
                    (*o_ptr).timeout -= 1;
                    /* Notice changes */
                    if (*o_ptr).timeout as libc::c_int == 0 as libc::c_int {
                        recharged_notice(o_ptr);
                        j += 1
                    }
                }
                /* Recharge second spell in Mage Staffs of Spells */
                if ((*o_ptr).name2 as libc::c_int == 4 as libc::c_int ||
                        (*o_ptr).name2b as libc::c_int == 4 as libc::c_int) &&
                       (*o_ptr).xtra2 as libc::c_int > 0 as libc::c_int {
                    /* Recharge */
                    (*o_ptr).xtra2 -= 1;
                    /* Notice changes */
                    if (*o_ptr).xtra2 as libc::c_int == 0 as libc::c_int {
                        j += 1
                    }
                }
            }
        }
        i += 1
    }
    /* Notice changes */
    if j != 0 {
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x2 as libc::c_long) as u32b
    }
    /* Recharge rods */
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Examine the rod */
            object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                         &mut esp);
            /* Temporary items are destroyed */
            if f5 as libc::c_long & 0x1 as libc::c_long != 0 {
                (*o_ptr).timeout -= 1;
                if (*o_ptr).timeout as libc::c_int <= 0 as libc::c_int {
                    inc_stack_size(i, -(99 as libc::c_int));
                    /* Combine and Reorder pack */
                    (*p_ptr).notice =
                        ((*p_ptr).notice as libc::c_long |
                             (0x1 as libc::c_long | 0x2 as libc::c_long)) as
                            u32b
                }
            }
            /* Examine all charging rods or stacks of charging rods. */
            if (*o_ptr).tval as libc::c_int == 67 as libc::c_int &&
                   ((*o_ptr).timeout as libc::c_int) <
                       (*o_ptr).pval2 as libc::c_int {
                /* Increase the rod's mana. */
                (*o_ptr).timeout =
                    ((*o_ptr).timeout as libc::c_int +
                         if f4 as libc::c_long & 0x4000 as libc::c_long != 0 {
                             2 as libc::c_int
                         } else { 1 as libc::c_int }) as s16b;
                /* Always notice */
                j += 1;
                /* Notice changes, provide message if object is inscribed. */
                if (*o_ptr).timeout as libc::c_int >=
                       (*o_ptr).pval2 as libc::c_int {
                    (*o_ptr).timeout = (*o_ptr).pval2;
                    recharged_notice(o_ptr);
                }
            }
            /* Examine all charging random artifacts */
            if f5 as libc::c_long & 0x2000 as libc::c_long != 0 &&
                   (*o_ptr).timeout as libc::c_int > 0 as libc::c_int {
                /* Charge it */
                (*o_ptr).timeout -= 1;
                /* Notice changes */
                if (*o_ptr).timeout as libc::c_int == 0 as libc::c_int {
                    j += 1;
                    recharged_notice(o_ptr);
                }
            }
            /* Decay objects in pack */
            if decays(o_ptr) != 0 {
                /* Decay it */
                if (*o_ptr).pval != 0 as libc::c_int {
                    if (*o_ptr).timeout as libc::c_int > 0 as libc::c_int {
                        if dungeon_flags1 as libc::c_long &
                               0x2000 as libc::c_long != 0 {
                            (*o_ptr).pval -= 2 as libc::c_int
                        } else if dungeon_flags1 as libc::c_long &
                                      0x4000 as libc::c_long != 0 &&
                                      Rand_div(2 as libc::c_int) != 0 {
                            if Rand_div(100 as libc::c_int) <
                                   50 as libc::c_int {
                                (*o_ptr).pval -= 1
                            }
                        } else { (*o_ptr).pval -= 1 }
                    }
                    if (*o_ptr).timeout as libc::c_int > 0 as libc::c_int &&
                           ((*o_ptr).timeout as libc::c_int) < (*o_ptr).weight
                       {
                        (*o_ptr).timeout -= 1
                    }
                    /* Notice changes */
                    if (*o_ptr).pval <= 0 as libc::c_int {
                        pack_decay(i);
                        j += 1
                    }
                }
            }
            /* Hatch eggs */
            if (*o_ptr).tval as libc::c_int == 10 as libc::c_int {
                let mut mx: libc::c_int = 0;
                let mut my: libc::c_int = 0;
                if (*o_ptr).timeout as libc::c_int == 0 as libc::c_int {
                    (*o_ptr).pval -= 1;
                    /* Notice changes */
                    if (*o_ptr).pval <= 0 as libc::c_int {
                        let mut m_ptr_1: *mut monster_type =
                            0 as *mut monster_type;
                        let mut r_ptr_0: *mut monster_race =
                            0 as *mut monster_race;
                        mx = (*p_ptr).px as libc::c_int;
                        my = (*p_ptr).py as libc::c_int + 1 as libc::c_int;
                        get_pos_player(5 as libc::c_int, &mut my, &mut mx);
                        msg_print(b"Your egg hatches!\x00" as *const u8 as
                                      *const libc::c_char);
                        place_monster_aux(my, mx,
                                          (*o_ptr).pval2 as libc::c_int,
                                          0 as libc::c_int as bool_,
                                          0 as libc::c_int as bool_,
                                          3 as libc::c_int);
                        m_ptr_1 =
                            &mut *m_list.offset((*(*cave.as_mut_ptr().offset(my
                                                                                 as
                                                                                 isize)).offset(mx
                                                                                                    as
                                                                                                    isize)).m_idx
                                                    as isize) as
                                *mut monster_type;
                        r_ptr_0 =
                            if !(*m_ptr_1).sr_ptr.is_null() {
                                (*m_ptr_1).sr_ptr
                            } else {
                                race_info_idx((*m_ptr_1).r_idx as libc::c_int,
                                              (*m_ptr_1).ego as libc::c_int)
                            };
                        if (*r_ptr_0).flags9 &
                               0x20 as libc::c_int as libc::c_uint != 0 &&
                               can_create_companion() as libc::c_int != 0 {
                            msg_format(b"And you have given the imprint to your %s!\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       r_name.offset((*r_ptr_0).name as
                                                         isize));
                            (*m_ptr_1).status = 4 as libc::c_int as s16b
                        }
                        inc_stack_size(i, -(1 as libc::c_int));
                        j += 1
                    }
                }
            }
        }
        i += 1
    }
    /* Notice changes */
    if j != 0 {
        /* Combine pack */
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x1 as libc::c_long) as u32b
    }
    /* ** Process Objects ***/
    /* Process objects */
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        /* Access object */
        o_ptr = &mut *o_list.offset(i as isize) as *mut object_type;
        /* Skip dead objects */
        if !((*o_ptr).k_idx == 0) {
            /* Examine the rod */
            object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                         &mut esp);
            /* Temporary items are destroyed */
            if f5 as libc::c_long & 0x1 as libc::c_long != 0 {
                (*o_ptr).timeout -= 1;
                if (*o_ptr).timeout as libc::c_int <= 0 as libc::c_int {
                    floor_item_increase(i, -(99 as libc::c_int));
                    floor_item_optimize(i);
                    /* Combine and Reorder pack */
                    (*p_ptr).notice =
                        ((*p_ptr).notice as libc::c_long |
                             (0x1 as libc::c_long | 0x2 as libc::c_long)) as
                            u32b
                }
            }
            /* Recharge rods on the ground.  No messages. */
            if (*o_ptr).tval as libc::c_int == 67 as libc::c_int &&
                   ((*o_ptr).timeout as libc::c_int) <
                       (*o_ptr).pval2 as libc::c_int {
                /* Increase the rod's mana. */
                (*o_ptr).timeout =
                    ((*o_ptr).timeout as libc::c_int +
                         if f4 as libc::c_long & 0x4000 as libc::c_long != 0 {
                             2 as libc::c_int
                         } else { 1 as libc::c_int }) as s16b;
                /* Do not overflow */
                if (*o_ptr).timeout as libc::c_int >=
                       (*o_ptr).pval2 as libc::c_int {
                    (*o_ptr).timeout = (*o_ptr).pval2
                }
            }
            /* Decay objects on the ground*/
            if decays(o_ptr) != 0 {
                /* Decay it */
                if (*o_ptr).pval != 0 as libc::c_int {
                    if (*o_ptr).timeout as libc::c_int > 0 as libc::c_int {
                        if dungeon_flags1 as libc::c_long &
                               0x2000 as libc::c_long != 0 {
                            (*o_ptr).pval -= 2 as libc::c_int
                        } else if dungeon_flags1 as libc::c_long &
                                      0x4000 as libc::c_long != 0 &&
                                      Rand_div(2 as libc::c_int) != 0 {
                            if Rand_div(100 as libc::c_int) <
                                   50 as libc::c_int {
                                (*o_ptr).pval -= 1
                            }
                        } else { (*o_ptr).pval -= 1 }
                    }
                    if (*o_ptr).timeout as libc::c_int > 0 as libc::c_int &&
                           ((*o_ptr).timeout as libc::c_int) < (*o_ptr).weight
                       {
                        (*o_ptr).timeout -= 1
                    }
                    /* Turn it into a skeleton */
                    if (*o_ptr).pval <= 0 as libc::c_int { floor_decay(i); }
                }
            }
            /* Hatch eggs */
            if (*o_ptr).tval as libc::c_int == 10 as libc::c_int {
                let mut mx_0: libc::c_int = 0;
                let mut my_0: libc::c_int = 0;
                if (*o_ptr).timeout as libc::c_int > 0 as libc::c_int {
                    (*o_ptr).pval -= 1
                }
                /* Notice changes */
                if (*o_ptr).pval <= 0 as libc::c_int {
                    mx_0 = (*o_ptr).ix as libc::c_int;
                    my_0 = (*o_ptr).iy as libc::c_int;
                    get_pos_player(5 as libc::c_int, &mut my_0, &mut mx_0);
                    msg_print(b"An egg hatches!\x00" as *const u8 as
                                  *const libc::c_char);
                    place_monster_one(my_0, mx_0,
                                      (*o_ptr).pval2 as libc::c_int,
                                      0 as libc::c_int,
                                      0 as libc::c_int as bool_,
                                      -(2 as libc::c_int));
                    floor_item_increase(i, -(1 as libc::c_int));
                    floor_item_describe(i);
                    floor_item_optimize(i);
                }
            }
        }
        i += 1
    }
    /* ** Involuntary Movement ***/
    /* Delayed Word-of-Recall */
    if (*p_ptr).word_recall != 0 {
        /* Can we ? */
        if process_hooks(62 as libc::c_int,
                         b"()\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"\x00" as *const u8 as *const libc::c_char) != 0 {
            (*p_ptr).word_recall = 0 as libc::c_int as s16b
        } else if dungeon_flags2 as libc::c_long & 0x8000 as libc::c_long != 0
         {
            cmsg_print(8 as libc::c_int as byte_hack,
                       b"You cannot recall from here.\x00" as *const u8 as
                           *const libc::c_char);
            (*p_ptr).word_recall = 0 as libc::c_int as s16b
        } else if dungeon_type as libc::c_int == DUNGEON_DEATH {
            cmsg_print(8 as libc::c_int as byte_hack,
                       b"You are fated to die here.  FIGHT for your life!\x00"
                           as *const u8 as *const libc::c_char);
            (*p_ptr).word_recall = 0 as libc::c_int as s16b
        } else if (*p_ptr).astral != 0 {
            msg_print(b"As an astral being you can\'t recall.\x00" as
                          *const u8 as *const libc::c_char);
            (*p_ptr).word_recall = 0 as libc::c_int as s16b
        } else {
            /* No recall. sorry */
            /* Cannot WoR out of death fate levels */
            /* I think the 'inside_quest' code belongs here -- pelpel */
            /* They cannot use word of recall until reaching surface */
            /* Normal WoR */
            /*
			 * HACK: Autosave BEFORE resetting the recall counter (rr9)
			 * The player is yanked up/down as soon as
			 * he loads the autosaved game.
			 */
            if (*p_ptr).word_recall as libc::c_int == 1 as libc::c_int {
                autosave_checkpoint();
            }
            save_dungeon();
            (*p_ptr).word_recall -= 1;
            if (*p_ptr).word_recall as libc::c_int == 0 as libc::c_int {
                /* Make SURE that persistent levels are saved
			 * I don't know if this is needed, but I'm getting reports,
			 * so I'm adding this extra save -- Neil
			 */
                /* Count down towards recall */
                /* Disturbing! */
                disturb(0 as libc::c_int, 0 as libc::c_int);
                /* Determine the level */
                if (*p_ptr).inside_quest != 0 {
                    msg_print(b"The recall is cancelled by a powerful magic force!\x00"
                                  as *const u8 as *const libc::c_char);
                } else if dun_level != 0 {
                    msg_print(b"You feel yourself yanked upwards!\x00" as
                                  *const u8 as *const libc::c_char);
                    (*p_ptr).recall_dungeon = dungeon_type as s16b;
                    dungeon_type = 0 as libc::c_int as byte_hack;
                    dun_level = 0 as libc::c_int as s16b;
                    is_recall = 1 as libc::c_int as bool_;
                    (*p_ptr).inside_quest = 0 as libc::c_int as s16b;
                    (*p_ptr).leaving = 1 as libc::c_int as bool_
                } else {
                    msg_print(b"You feel yourself yanked downwards!\x00" as
                                  *const u8 as *const libc::c_char);
                    /* New depth */
                    dungeon_type = (*p_ptr).recall_dungeon as byte_hack;
                    dun_level = *max_dlv.offset(dungeon_type as isize);
                    if (dun_level as libc::c_int) < 1 as libc::c_int {
                        dun_level = 1 as libc::c_int as s16b
                    }
                    /* Reset player position */
                    (*p_ptr).oldpx = (*p_ptr).px;
                    (*p_ptr).oldpy = (*p_ptr).py;
                    /* Leaving */
                    is_recall = 1 as libc::c_int as bool_;
                    (*p_ptr).leaving = 1 as libc::c_int as bool_;
                    (*p_ptr).wild_mode = 0 as libc::c_int as bool_
                }
                /* Sound */
                sound(24 as libc::c_int);
            }
        }
    };
}
/* Activate the recall */
/*
 * Verify use of "wizard" mode
 */
unsafe extern "C" fn enter_wizard_mode() -> bool_ {
    /* Ask first time, but not while loading a dead char with the -w option */
    if noscore == 0 && !(((*p_ptr).chp as libc::c_int) < 0 as libc::c_int) {
        /* Mention effects */
        msg_print(b"Wizard mode is for debugging and experimenting.\x00" as
                      *const u8 as *const libc::c_char);
        msg_print(b"The game will not be scored if you enter wizard mode.\x00"
                      as *const u8 as *const libc::c_char);
        msg_print(0 as cptr);
        /* Verify request */
        if get_check(b"Are you sure you want to enter wizard mode? \x00" as
                         *const u8 as *const libc::c_char) == 0 {
            return 0 as libc::c_int as bool_
        }
        /* Mark savefile */
        noscore = (noscore as libc::c_int | 0x2 as libc::c_int) as u16b
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Verify use of "debug" commands
 */
unsafe extern "C" fn enter_debug_mode() -> bool_ {
    /* Ask first time */
    if noscore == 0 && wizard == 0 {
        /* Mention effects */
        msg_print(b"The debug commands are for debugging and experimenting.\x00"
                      as *const u8 as *const libc::c_char);
        msg_print(b"The game will not be scored if you use debug commands.\x00"
                      as *const u8 as *const libc::c_char);
        msg_print(0 as cptr);
        /* Verify request */
        if get_check(b"Are you sure you want to use debug commands? \x00" as
                         *const u8 as *const libc::c_char) == 0 {
            return 0 as libc::c_int as bool_
        }
        /* Mark savefile */
        noscore = (noscore as libc::c_int | 0x8 as libc::c_int) as u16b
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Parse and execute the current command
 * Give "Warning" on illegal commands.
 *
 * XXX XXX XXX Make some "blocks"
 */
unsafe extern "C" fn process_command() {
    let mut error_m: [libc::c_char; 80] = [0; 80];
    /* Handle repeating the last command */
    repeat_check();
    /* Process the appropriate hooks */
    if process_hooks(31 as libc::c_int,
                     b"(d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, command_cmd as libc::c_int) != 0 {
        return
    }
    /* Parse the command */
    match command_cmd as libc::c_int {
        27 | 32 | 0 | 13 => { }
        23 => {
            /* ** Wizard Commands ***/
            /* Toggle Wizard Mode */
            if wizard != 0 {
                wizard = 0 as libc::c_int as bool_;
                msg_print(b"Wizard mode off.\x00" as *const u8 as
                              *const libc::c_char);
            } else if enter_wizard_mode() != 0 {
                wizard = 1 as libc::c_int as bool_;
                msg_print(b"Wizard mode on.\x00" as *const u8 as
                              *const libc::c_char);
            }
            /* Update monsters */
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long)
                    as u32b;
            /* Redraw "title" */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x2 as libc::c_long) as
                    u32b
        }
        1 => {
            /* Special "debug" commands */
            /* Enter debug mode */
            if enter_debug_mode() != 0 { do_cmd_debug(); }
        }
        119 => {
            /* ** Inventory Commands ***/
            /* Wear/wield equipment */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_wield(); }
            }
        }
        116 => {
            /* Take off equipment */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_takeoff(); }
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long |
                         0x10000000 as libc::c_long) as u32b
            }
        }
        100 => {
            /* Drop an item */
            if !(do_control_drop() != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_drop(); }
            }
        }
        107 => {
            /* Destroy an item */
            if !((*p_ptr).control != 0) { do_cmd_destroy(); }
        }
        101 => {
            /* Equipment list */
            if !((*p_ptr).control != 0) { do_cmd_equip(); }
        }
        105 => {
            /* Inventory list */
            if !(do_control_inven() != 0) { do_cmd_inven(); }
        }
        73 => {
            /* ** Various commands ***/
            /* Identify an object */
            do_cmd_observe();
        }
        9 => {
            /* Hack -- toggle windows */
            toggle_inven_equip();
        }
        43 => {
            /* ** Standard "Movement" Commands ***/
            /* Alter a grid */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_alter(); }
            }
        }
        84 => {
            /* Dig a tunnel */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_tunnel(); }
            }
        }
        59 => {
            /* Move (usually pick up things) */
            if !(do_control_walk() != 0) {
                do_cmd_walk(always_pickup as libc::c_int,
                            1 as libc::c_int as bool_);
            }
        }
        45 => {
            /* Move (usually do not pick up) */
            if !(do_control_walk() != 0) {
                do_cmd_walk((always_pickup == 0) as libc::c_int,
                            1 as libc::c_int as bool_);
            }
        }
        46 => {
            /* ** Running, Resting, Searching, Staying */
            /* Begin Running -- Arg is Max Distance */
            if !((*p_ptr).control as libc::c_int != 0 ||
                     (*p_ptr).wild_mode as libc::c_int != 0) {
                do_cmd_run();
            }
        }
        44 => {
            /* Stay still (usually pick things up) */
            if !(do_control_pickup() != 0) {
                do_cmd_stay(always_pickup as libc::c_int);
            }
        }
        103 => {
            /* Stay still (usually do not pick up) */
            if !((*p_ptr).control != 0) {
                do_cmd_stay((always_pickup == 0) as libc::c_int);
            }
        }
        82 => {
            /* Rest -- Arg is time */
            if !((*p_ptr).control != 0) { do_cmd_rest(); }
        }
        115 => {
            /* Search for traps/doors */
            if !((*p_ptr).control != 0) { do_cmd_search(); }
        }
        83 => {
            /* Toggle search mode */
            if !((*p_ptr).control != 0) { do_cmd_toggle_search(); }
        }
        95 => {
            /* ** Stairs and Doors and Chests and Traps ***/
            /* Enter store */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_store(); }
            }
        }
        60 => {
            /* Go up staircase */
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
            let mut f1: u32b = 0 as libc::c_int as u32b;
            let mut f2: u32b = 0 as libc::c_int as u32b;
            let mut f3: u32b = 0 as libc::c_int as u32b;
            let mut f4: u32b = 0 as libc::c_int as u32b;
            let mut f5: u32b = 0 as libc::c_int as u32b;
            let mut esp: u32b = 0 as libc::c_int as u32b;
            /* Check for light being wielded */
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(36 as libc::c_int
                                                                 as isize) as
                    *mut object_type;
            /* Burn some fuel in the current lite */
            if (*o_ptr).tval as libc::c_int == 39 as libc::c_int {
                /* Extract the item flags */
                object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4,
                             &mut f5, &mut esp);
            }
            /* Cannot move if rooted in place */
            if !((*p_ptr).tim_roots != 0) {
                if !((*p_ptr).control != 0) {
                    /* Normal cases */
                    if (*p_ptr).wild_mode as libc::c_int != 0 ||
                           dun_level as libc::c_int != 0 ||
                           is_quest(dun_level as libc::c_int) != 0 {
                        do_cmd_go_up();
                    } else if ((*p_ptr).food as libc::c_int) <
                                  2000 as libc::c_int {
                        msg_print(b"You are too hungry to travel.\x00" as
                                      *const u8 as *const libc::c_char);
                    } else if (*p_ptr).sensible_lite as libc::c_int != 0 &&
                                  turn as libc::c_long /
                                      (10 as libc::c_long *
                                           11520 as libc::c_int as
                                               libc::c_long /
                                           2 as libc::c_int as libc::c_long) %
                                      2 as libc::c_int as libc::c_long ==
                                      0 as libc::c_int as libc::c_long {
                        /* Don't let the player < when he'd just drop right back down */
                        /* Burn vampires! burn! */
                        msg_print(b"You can\'t travel during the day!\x00" as
                                      *const u8 as *const libc::c_char);
                    } else if (*p_ptr).sensible_lite as libc::c_int != 0 &&
                                  (*o_ptr).tval as libc::c_int !=
                                      0 as libc::c_int &&
                                  (*o_ptr).sval as libc::c_int >=
                                      100 as libc::c_int &&
                                  (*o_ptr).sval as libc::c_int <=
                                      106 as libc::c_int &&
                                  (*o_ptr).sval as libc::c_int !=
                                      103 as libc::c_int {
                        msg_print(b"Travel with your present light would be unsafe.\x00"
                                      as *const u8 as *const libc::c_char);
                    } else if (*p_ptr).cut as libc::c_int != 0 ||
                                  (*p_ptr).poisoned as libc::c_int != 0 {
                        /* I actually died this way once -- neil */
                        msg_print(b"You are too injured to travel.\x00" as
                                      *const u8 as *const libc::c_char);
                    } else if ambush_flag != 0 {
                        msg_print(b"To flee the ambush you have to reach the edge of the map.\x00"
                                      as *const u8 as *const libc::c_char);
                    } else if process_hooks(75 as libc::c_int,
                                            b"()\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) == 0 {
                        (*p_ptr).oldpx = (*p_ptr).px;
                        (*p_ptr).oldpy = (*p_ptr).py;
                        change_wild_mode();
                        /* TODO: make the above stuff use this hook */
                        /* Update the known wilderness */
                        reveal_wilderness_around_player((*p_ptr).wilderness_y,
                                                        (*p_ptr).wilderness_x,
                                                        0 as libc::c_int,
                                                        3 as libc::c_int);
                    }
                }
            }
        }
        62 => {
            /* Cannot move if rooted in place */
            if !((*p_ptr).tim_roots != 0) {
                if !((*p_ptr).control != 0) {
                    /* Normal cases */
                    if (*p_ptr).wild_mode == 0 {
                        do_cmd_go_down();
                    } else if (*wf_info.offset((*(*wild_map.offset((*p_ptr).py
                                                                       as
                                                                       isize)).offset((*p_ptr).px
                                                                                          as
                                                                                          isize)).feat
                                                   as isize)).entrance as
                                  libc::c_int >= 1000 as libc::c_int ||
                                  (*(*wild_map.offset((*p_ptr).py as
                                                          isize)).offset((*p_ptr).px
                                                                             as
                                                                             isize)).entrance
                                      as libc::c_int > 1000 as libc::c_int {
                        (*p_ptr).wilderness_x = (*p_ptr).px as s32b;
                        (*p_ptr).wilderness_y = (*p_ptr).py as s32b;
                        (*p_ptr).wild_mode =
                            ((*p_ptr).wild_mode == 0) as libc::c_int as bool_;
                        do_cmd_go_down();
                        if dun_level as libc::c_int == 0 as libc::c_int {
                            (*p_ptr).wild_mode =
                                ((*p_ptr).wild_mode == 0) as libc::c_int as
                                    bool_
                        } else {
                            (*p_ptr).wilderness_x = (*p_ptr).px as s32b;
                            (*p_ptr).wilderness_y = (*p_ptr).py as s32b;
                            change_wild_mode();
                        }
                    } else {
                        (*p_ptr).wilderness_x = (*p_ptr).px as s32b;
                        (*p_ptr).wilderness_y = (*p_ptr).py as s32b;
                        change_wild_mode();
                    }
                }
            }
        }
        111 => {
            /* Special cases */
            /* Open a door or chest */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_open(); }
            }
        }
        99 => {
            /* Close a door */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_close(); }
            }
        }
        121 => {
            /* Give an item */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_give(); }
            }
        }
        89 => {
            /* Chat */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_chat(); }
            }
        }
        106 => {
            /* Jam a door with spikes */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_spike(); }
            }
        }
        66 => {
            /* Bash a door */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_bash(); }
            }
        }
        68 => {
            /* Disarm a trap or chest */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_disarm(); }
            }
        }
        71 => {
            /* ** Magic and Prayers ***/
            /* Interact with skills */
            if !((*p_ptr).control != 0) { do_cmd_skill(); }
        }
        78 => {
            /* Interact with abilities */
            if !((*p_ptr).control != 0) { do_cmd_ability(); }
        }
        98 => {
            /* Browse a book */
            if !((*p_ptr).control != 0) { do_cmd_browse(); }
        }
        109 => {
            /* Cast a spell */
            if !(do_control_magic() != 0) {
                /* No magic in the overworld map */
                if !((*p_ptr).wild_mode != 0) {
                    /* Neither in the Arena */
                    if (*p_ptr).inside_arena != 0 {
                        msg_print(b"The arena absorbs all attempted magic!\x00"
                                      as *const u8 as *const libc::c_char);
                    } else {
                        do_cmd_activate_skill();
                        squeltch_inventory();
                        squeltch_grid();
                    }
                }
            }
        }
        112 => {
            /* Pray a prayer */
            if !((*p_ptr).control as libc::c_int != 0 ||
                     (*p_ptr).wild_mode as libc::c_int != 0) {
                do_cmd_pray();
                squeltch_inventory();
                squeltch_grid();
            }
        }
        80 => {
            /* Issue commands to pets */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_pet(); }
            }
        }
        104 => {
            /* Cut up a corpse */
            if !((*p_ptr).control as libc::c_int != 0 ||
                     (*p_ptr).wild_mode as libc::c_int != 0) {
                do_cmd_cut_corpse();
                squeltch_inventory();
                squeltch_grid();
            }
        }
        75 => {
            /* Cure some meat */
            if !((*p_ptr).control != 0) {
                do_cmd_cure_meat();
                squeltch_inventory();
                squeltch_grid();
            }
        }
        90 => {
            /* Steal an item form a monster */
            if !((*p_ptr).control as libc::c_int != 0 ||
                     (*p_ptr).wild_mode as libc::c_int != 0) {
                do_cmd_steal();
                squeltch_inventory();
                squeltch_grid();
            }
        }
        123 => {
            /* ** Use various objects ***/
            /* Inscribe an object */
            if !((*p_ptr).control != 0) { do_cmd_inscribe(); }
        }
        125 => {
            /* Uninscribe an object */
            if !((*p_ptr).control != 0) { do_cmd_uninscribe(); }
        }
        65 => {
            /* Activate an artifact */
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    if (*p_ptr).inside_arena != 0 {
                        msg_print(b"The arena absorbs all attempted magic!\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(0 as cptr);
                    } else {
                        do_cmd_activate();
                        squeltch_inventory();
                        squeltch_grid();
                    }
                }
            }
        }
        69 => {
            /* Eat some food */
            if !((*p_ptr).control != 0) { do_cmd_eat_food(); }
        }
        70 => {
            /* Fuel your lantern/torch */
            if !((*p_ptr).control != 0) { do_cmd_refill(); }
        }
        102 => {
            /* Fire an item */
            let mut j_ptr: *mut object_type = 0 as *mut object_type;
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    if (*p_ptr).inside_arena != 0 {
                        msg_print(b"You\'re in the arena now. This is hand-to-hand!\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(0 as cptr);
                    } else {
                        j_ptr =
                            &mut *(*p_ptr).inventory.as_mut_ptr().offset(27 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                as *mut object_type;
                        if !(process_hooks(54 as libc::c_int,
                                           b"(O)\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char, j_ptr) != 0)
                           {
                            if (*j_ptr).tval as libc::c_int ==
                                   15 as libc::c_int {
                                do_cmd_boomerang();
                            } else { do_cmd_fire(); }
                        }
                    }
                }
            }
        }
        118 => {
            /* Throw an item */
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    if (*p_ptr).inside_arena != 0 {
                        msg_print(b"You\'re in the arena now. This is hand-to-hand!\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(0 as cptr);
                    } else { do_cmd_throw(); }
                }
            }
        }
        97 => {
            /* Aim a wand */
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    if (*p_ptr).inside_arena != 0 {
                        msg_print(b"The arena absorbs all attempted magic!\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(0 as cptr);
                    } else {
                        do_cmd_aim_wand();
                        squeltch_inventory();
                        squeltch_grid();
                    }
                }
            }
        }
        122 => {
            /* Zap a rod */
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    if (*p_ptr).inside_arena != 0 {
                        msg_print(b"The arena absorbs all attempted magic!\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(0 as cptr);
                    } else {
                        do_cmd_zap_rod();
                        squeltch_inventory();
                        squeltch_grid();
                    }
                }
            }
        }
        113 => {
            /* Quaff a potion */
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    if (*p_ptr).inside_arena != 0 {
                        msg_print(b"The arena absorbs all attempted magic!\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(0 as cptr);
                    } else {
                        do_cmd_quaff_potion();
                        squeltch_inventory();
                        squeltch_grid();
                    }
                }
            }
        }
        72 => {
            /* Drink from a fountain -SC- */
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                                     isize)).offset((*p_ptr).px
                                                                        as
                                                                        isize)
                    as *mut cave_type;
            if !((*p_ptr).control != 0) {
                if (*c_ptr).feat as libc::c_int == 0x2 as libc::c_int ||
                       (*c_ptr).feat as libc::c_int == 0xf as libc::c_int {
                    do_cmd_drink_fountain();
                    squeltch_inventory();
                    squeltch_grid();
                } else {
                    msg_print(b"You see no fountain here.\x00" as *const u8 as
                                  *const libc::c_char);
                }
            }
        }
        114 => {
            /* Read a scroll */
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    if (*p_ptr).inside_arena != 0 {
                        msg_print(b"The arena absorbs all attempted magic!\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(0 as cptr);
                    } else {
                        do_cmd_read_scroll();
                        squeltch_inventory();
                        squeltch_grid();
                    }
                }
            }
        }
        117 => {
            /* Use a staff */
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    if (*p_ptr).inside_arena != 0 {
                        msg_print(b"The arena absorbs all attempted magic!\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(0 as cptr);
                    } else {
                        do_cmd_use_staff();
                        squeltch_inventory();
                        squeltch_grid();
                    }
                }
            }
        }
        85 => {
            /* Use racial power */
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    if (*p_ptr).inside_arena != 0 {
                        msg_print(b"The arena absorbs all attempted magic!\x00"
                                      as *const u8 as *const libc::c_char);
                        msg_print(0 as cptr);
                    } else {
                        do_cmd_power();
                        squeltch_inventory();
                        squeltch_grid();
                    }
                }
            }
        }
        79 => {
            /* Sacrifice at an altar */
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 |
                            (*cp_ptr).flags1 | (*spp_ptr).flags1) as
                           libc::c_long & 0x4000 as libc::c_long != 0 {
                        msg_print(b"You cannot worship gods.\x00" as *const u8
                                      as *const libc::c_char);
                    } else { do_cmd_sacrifice(); }
                }
            }
        }
        77 => {
            /* ** Looking at Things (nearby or on map) ***/
            /* Full dungeon map */
            if (*p_ptr).wild_mode == 0 { do_cmd_view_map(); }
        }
        76 => {
            /* Locate player on map */
            do_cmd_locate();
        }
        108 => {
            /* Look around */
            do_cmd_look();
        }
        42 => {
            /* Target monster or location */
            if !((*p_ptr).control != 0) {
                if (*p_ptr).wild_mode == 0 { do_cmd_target(); }
            }
        }
        120 => {
            /* Engrave the floor */
            if !((*p_ptr).control != 0) {
                if !((*p_ptr).wild_mode != 0) {
                    /* No point in engraving if there isn't any mana on this grid. */
			/* DG - actualy there is, it doesnt break macros */
                    do_cmd_sense_grid_mana();
                    do_cmd_engrave();
                }
            }
        }
        63 => {
            /* ** Help and Such ***/
            /* Help */
            do_cmd_help();
        }
        47 => {
            /* Identify symbol */
            do_cmd_query_symbol();
        }
        67 => {
            /* Character description */
            do_cmd_change_name();
        }
        33 => {
            /* ** System Commands ***/
            /* Hack -- User interface */
            Term_user(0 as libc::c_int);
        }
        34 => {
            /* Single line from a pref file */
            do_cmd_pref();
        }
        64 => {
            /* Interact with macros */
            do_cmd_macros();
        }
        37 => {
            /* Interact with visuals */
            do_cmd_visuals();
        }
        38 => {
            /* Interact with colors */
            do_cmd_colors();
        }
        61 => {
            /* Interact with options */
            do_cmd_options();
        }
        58 => {
            /* ** Misc Commands ***/
            /* Take notes */
            do_cmd_note();
        }
        86 => {
            /* Version info */
            do_cmd_version();
        }
        6 => {
            /* Repeat level feeling */
            if (*p_ptr).wild_mode == 0 { do_cmd_feeling(); }
        }
        15 => {
            /* Show previous message */
            do_cmd_message_one();
        }
        16 => {
            /* Show previous messages */
            do_cmd_messages();
        }
        17 | -8184 => {
            /* Show quest status -KMW- */
            do_cmd_checkquest();
        }
        18 => {
            /* Redraw the screen */
            do_cmd_redraw();
        }
        19 => {
            /* Hack -- Save and don't quit */
            is_autosave = 0 as libc::c_int as bool_;
            do_cmd_save_game();
        }
        20 => { do_cmd_time(); }
        24 => {
            /* Save and quit */
            alive = 0 as libc::c_int as bool_;
            /* Leaving */
            (*p_ptr).leaving = 1 as libc::c_int as bool_
        }
        81 => {
            /* Quit (commit suicide) */
            do_cmd_suicide();
        }
        124 => {
            /* Activate cmovie */
            /* Stop ? */
            if do_movies == 1 as libc::c_int {
                do_stop_cmovie();
                msg_print(b"Cmovie recording stopped.\x00" as *const u8 as
                              *const libc::c_char);
            } else { do_start_cmovie(); }
        }
        35 => {
            /* Extended command */
            do_cmd_cli();
        }
        126 => {
            /* Check artifacts, uniques, objects */
            do_cmd_knowledge();
        }
        -8192 => {
            /* Commands only available as extended commands: */
            /* Extended command help. */
            do_cmd_cli_help();
        }
        -8188 => {
            /* Game time. */
            do_cmd_time();
        }
        -8187 => {
            /* Check skills. */
            do_cmd_skill();
        }
        -8182 => {
            /* Check abilities. */
            do_cmd_ability();
        }
        -8186 => {
            /* Save a html screenshot. */
            do_cmd_html_dump();
        }
        36 | -8185 => {
            /* Record a macro. */
            do_cmd_macro_recorder();
        }
        -8183 => {
            if !(do_control_walk() != 0) {
                do_cmd_walk(always_pickup as libc::c_int,
                            0 as libc::c_int as bool_);
            }
        }
        _ => {
            /* Hack -- Unknown command */
            let mut insanity: libc::c_int =
                ((*p_ptr).msane as libc::c_int -
                     (*p_ptr).csane as libc::c_int) * 100 as libc::c_int /
                    (*p_ptr).msane as libc::c_int;
            /* Would like to have an option disabling this -- pelpel */
            if Rand_div(100 as libc::c_int) < insanity {
                get_rnd_line(b"error.txt\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             error_m.as_mut_ptr());
                sound(54 as libc::c_int);
                msg_print(error_m.as_mut_ptr() as cptr);
            } else {
                prt(b"Type \'?\' for help.\x00" as *const u8 as
                        *const libc::c_char, 0 as libc::c_int,
                    0 as libc::c_int);
            }
        }
    };
}
/*
 * Process the player
 *
 * Notice the annoying code to handle "pack overflow", which
 * must come first just in case somebody manages to corrupt
 * the savefiles by clever use of menu commands or something.
 */
#[no_mangle]
pub unsafe extern "C" fn process_player() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut speed_use: libc::c_int = 0;
    /* ** Apply energy ***/
    if hack_corruption != 0 {
        msg_print(b"You feel different!\x00" as *const u8 as
                      *const libc::c_char);
        gain_random_corruption(0 as libc::c_int);
        hack_corruption = 0 as libc::c_int as bool_
    }
    /* Obtain current speed */
    speed_use = (*p_ptr).pspeed as libc::c_int;
    /* Maximum value */
    if speed_use > 199 as libc::c_int {
        speed_use = 199 as libc::c_int
    } else if speed_use < 0 as libc::c_int { speed_use = 0 as libc::c_int }
    /* Minimum value */
    /* Give the player some energy */
    (*p_ptr).energy += extract_energy[speed_use as usize] as libc::c_int;
    /* No turn yet */
    if (*p_ptr).energy < 100 as libc::c_int { return }
    /* ** Check for interupts ***/
    /* Complete resting */
    if (resting as libc::c_int) < 0 as libc::c_int {
        /* Basic resting */
        if resting as libc::c_int == -(1 as libc::c_int) {
            /* Stop resting */
            if (*p_ptr).chp as libc::c_int == (*p_ptr).mhp as libc::c_int &&
                   (*p_ptr).csp as libc::c_int >= (*p_ptr).msp as libc::c_int
               {
                disturb(0 as libc::c_int, 0 as libc::c_int);
            }
        } else if resting as libc::c_int == -(2 as libc::c_int) {
            let mut stop: bool_ = 1 as libc::c_int as bool_;
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
            /* Complete resting */
            /* Get the carried monster */
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int
                                                                 as isize) as
                    *mut object_type;
            /* Stop resting */
            if (*p_ptr).drain_life == 0 &&
                   (*p_ptr).chp as libc::c_int != (*p_ptr).mhp as libc::c_int
               {
                stop = 0 as libc::c_int as bool_
            }
            if (*p_ptr).drain_mana == 0 &&
                   (*p_ptr).csp as libc::c_int != (*p_ptr).msp as libc::c_int
               {
                stop = 0 as libc::c_int as bool_
            }
            if ((*o_ptr).pval2 as libc::c_int) < (*o_ptr).pval3 {
                stop = 0 as libc::c_int as bool_
            }
            if (*p_ptr).blind as libc::c_int != 0 ||
                   (*p_ptr).confused as libc::c_int != 0 {
                stop = 0 as libc::c_int as bool_
            }
            if (*p_ptr).poisoned as libc::c_int != 0 ||
                   (*p_ptr).afraid as libc::c_int != 0 {
                stop = 0 as libc::c_int as bool_
            }
            if (*p_ptr).stun as libc::c_int != 0 ||
                   (*p_ptr).cut as libc::c_int != 0 {
                stop = 0 as libc::c_int as bool_
            }
            if (*p_ptr).slow as libc::c_int != 0 ||
                   (*p_ptr).paralyzed as libc::c_int != 0 {
                stop = 0 as libc::c_int as bool_
            }
            if (*p_ptr).image as libc::c_int != 0 ||
                   (*p_ptr).word_recall as libc::c_int != 0 {
                stop = 0 as libc::c_int as bool_
            }
            if (*p_ptr).immov_cntr as libc::c_int != 0 as libc::c_int {
                stop = 0 as libc::c_int as bool_
            }
            i = 0 as libc::c_int;
            while i < 6 as libc::c_int {
                if (*p_ptr).stat_cnt[i as usize] as libc::c_int >
                       0 as libc::c_int {
                    stop = 0 as libc::c_int as bool_
                }
                i += 1
            }
            if stop != 0 { disturb(0 as libc::c_int, 0 as libc::c_int); }
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long)
                    as u32b
        }
    }
    /* Handle "abort" */
    if avoid_abort == 0 {
        /* Check for "player abort" (semi-efficiently for resting) */
        if running as libc::c_int != 0 || command_rep as libc::c_int != 0 ||
               resting as libc::c_int != 0 &&
                   resting as libc::c_int & 0xf as libc::c_int == 0 {
            /* Do not wait */
            inkey_scan = 1 as libc::c_int as bool_;
            /* Check for a key */
            if inkey() != 0 {
                /* Flush input */
                flush();
                /* Disturb */
                disturb(0 as libc::c_int, 0 as libc::c_int);
                /* Hack -- Show a Message */
                msg_print(b"Cancelled.\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
    }
    /* ** Handle actual user input ***/
    /* Repeat until out of energy */
    while (*p_ptr).energy >= 100 as libc::c_int {
        /* Notice stuff (if needed) */
        if (*p_ptr).notice != 0 { notice_stuff(); }
        /* Update stuff (if needed) */
        if (*p_ptr).update != 0 { update_stuff(); }
        /* Redraw stuff (if needed) */
        if (*p_ptr).redraw != 0 { redraw_stuff(); }
        /* Redraw stuff (if needed) */
        if (*p_ptr).window != 0 { window_stuff(); }
        /* Hack -- mark current wilderness location as known */
        if (*p_ptr).wild_mode == 0 &&
               dun_level as libc::c_int == 0 as libc::c_int {
            (*(*wild_map.offset((*p_ptr).wilderness_y as
                                    isize)).offset((*p_ptr).wilderness_x as
                                                       isize)).known =
                1 as libc::c_int as bool_
        }
        /* Place the cursor on the player */
        move_cursor_relative((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int);
        /* Refresh (optional) */
        if fresh_before != 0 { Term_fresh(); }
        /* Hack -- Pack Overflow */
        if (*p_ptr).inventory[23 as libc::c_int as usize].k_idx != 0 {
            let mut item: libc::c_int = 23 as libc::c_int;
            let mut o_name: [libc::c_char; 80] = [0; 80];
            let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
            /* Access the slot to be dropped */
            o_ptr_0 =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
                    *mut object_type;
            /* Disturbing */
            disturb(0 as libc::c_int, 0 as libc::c_int);
            /* Warning */
            msg_print(b"Your pack overflows!\x00" as *const u8 as
                          *const libc::c_char);
            /* Describe */
            object_desc(o_name.as_mut_ptr(), o_ptr_0, 1 as libc::c_int,
                        3 as libc::c_int);
            /* Message */
            msg_format(b"You drop %s (%c).\x00" as *const u8 as
                           *const libc::c_char, o_name.as_mut_ptr(),
                       index_to_label(item) as libc::c_int);
            /* Drop it (carefully) near the player */
            drop_near(o_ptr_0, 0 as libc::c_int, (*p_ptr).py as libc::c_int,
                      (*p_ptr).px as libc::c_int);
            /* Modify, Describe, Optimize */
            inc_stack_size(item, -(255 as libc::c_int));
            /* Notice stuff (if needed) */
            if (*p_ptr).notice != 0 { notice_stuff(); }
            /* Update stuff (if needed) */
            if (*p_ptr).update != 0 { update_stuff(); }
            /* Redraw stuff (if needed) */
            if (*p_ptr).redraw != 0 { redraw_stuff(); }
            /* Redraw stuff (if needed) */
            if (*p_ptr).window != 0 { window_stuff(); }
        }
        /* Assume free turn */
        energy_use = 0 as libc::c_int;
        /* Paralyzed or Knocked Out */
        if (*p_ptr).paralyzed as libc::c_int != 0 ||
               (*p_ptr).stun as libc::c_int >= 100 as libc::c_int {
            /* Take a turn */
            energy_use = 100 as libc::c_int
        } else if resting != 0 {
            /* Resting */
            /* Timed rest */
            if resting as libc::c_int > 0 as libc::c_int {
                /* Reduce rest count */
                resting -= 1;
                /* Redraw the state */
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long |
                         0x100000 as libc::c_long) as u32b
            }
            (*p_ptr).did_nothing = 1 as libc::c_int as bool_;
            /* Take a turn */
            energy_use = 100 as libc::c_int
        } else if running != 0 {
            /* Running */
            /* Take a step */
            run_step(0 as libc::c_int);
            /*
			 * Commented out because it doesn't make any sense
			 * to require a player holding down direction keys
			 * instead of using running commands when s/he follows
			 * Eru and do the opposite for the other deities -- pelpel
			 */
			/* p_ptr->did_nothing = TRUE; */
        } else if command_rep != 0 {
            /* Repeated command */
            /* Count this execution */
            command_rep -= 1;
            /* Redraw the state */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long)
                    as u32b;
            /* Redraw stuff */
            redraw_stuff();
            /* Hack -- Assume messages were seen */
            msg_flag = 0 as libc::c_int as bool_;
            /* Clear the top line */
            prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
                0 as libc::c_int);
            /* Process the command */
            process_command();
            (*p_ptr).did_nothing = 1 as libc::c_int as bool_
        } else {
            /* Normal command */
            /* Place the cursor on the player */
            move_cursor_relative((*p_ptr).py as libc::c_int,
                                 (*p_ptr).px as libc::c_int);
            request_command(0 as libc::c_int);
            process_command();
        }
        /* Get a command (normal) */
        /* Process the command */
        /* ** Clean up ***/
        /* Significant */
        if energy_use != 0 {
            /* Use some energy */
            (*p_ptr).energy -= energy_use;
            /* Hack -- constant hallucination */
            if (*p_ptr).image != 0 {
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long |
                         0x4000000 as libc::c_long) as u32b
            }
            /* Shimmer monsters if needed */
            if avoid_other == 0 && use_graphics == 0 &&
                   shimmer_monsters as libc::c_int != 0 {
                /* Clear the flag */
                shimmer_monsters = 0 as libc::c_int as bool_;
                /* Shimmer multi-hued monsters */
                i = 1 as libc::c_int;
                while i < m_max as libc::c_int {
                    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
                    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
                    /* Access monster */
                    m_ptr =
                        &mut *m_list.offset(i as isize) as *mut monster_type;
                    /* Skip dead monsters */
                    if !((*m_ptr).r_idx == 0) {
                        /* Access the monster race */
                        r_ptr =
                            if !(*m_ptr).sr_ptr.is_null() {
                                (*m_ptr).sr_ptr
                            } else {
                                race_info_idx((*m_ptr).r_idx as libc::c_int,
                                              (*m_ptr).ego as libc::c_int)
                            };
                        /* Skip non-multi-hued monsters */
                        if !((*r_ptr).flags1 &
                                 0x80 as libc::c_int as libc::c_uint == 0) {
                            /* Reset the flag */
                            shimmer_monsters = 1 as libc::c_int as bool_;
                            /* Redraw regardless */
                            lite_spot((*m_ptr).fy as libc::c_int,
                                      (*m_ptr).fx as libc::c_int);
                        }
                    }
                    i += 1
                }
            }
            /* Shimmer objects if needed and requested */
            if avoid_other == 0 && avoid_shimmer == 0 && use_graphics == 0 &&
                   shimmer_objects as libc::c_int != 0 {
                /* Clear the flag */
                shimmer_objects = 0 as libc::c_int as bool_;
                /* Shimmer multi-hued objects */
                i = 1 as libc::c_int;
                while i < o_max as libc::c_int {
                    /* Acquire object -- for speed only base items are allowed to shimmer */
                    let mut o_ptr_1: *mut object_type =
                        &mut *o_list.offset(i as isize) as *mut object_type;
                    let mut k_ptr: *mut object_kind =
                        &mut *k_info.offset((*o_ptr_1).k_idx as isize) as
                            *mut object_kind;
                    /* Skip dead or carried objects */
                    if !((*o_ptr_1).k_idx == 0 || (*o_ptr_1).ix == 0) {
                        /* Skip non-multi-hued monsters */
                        if !((*k_ptr).flags5 as libc::c_long &
                                 0x40 as libc::c_long == 0) {
                            /* Reset the flag */
                            shimmer_objects = 1 as libc::c_int as bool_;
                            /* Redraw regardless */
                            lite_spot((*o_ptr_1).iy as libc::c_int,
                                      (*o_ptr_1).ix as libc::c_int);
                        }
                    }
                    i += 1
                }
            }
            /*
			 * Shimmer features if needed and requested
			 *
			 * Note: this can be unbearably slow when a player chooses
			 * to use a REALLY big screen in levels filled with shallow
			 * water.  I believe this also hurts a lot on multiuser systems.
			 * However fast modern processors are, I/O cannot be made that
			 * fast, and that's why shimmering has been limited to small
			 * number of monsters -- pelpel
			 */
            if avoid_other == 0 && avoid_shimmer == 0 && use_graphics == 0 &&
                   resting == 0 && running == 0 {
                j = panel_row_min as libc::c_int;
                while j <= panel_row_max as libc::c_int {
                    i = panel_col_min as libc::c_int;
                    while i <= panel_col_max as libc::c_int {
                        let mut c_ptr: *mut cave_type =
                            &mut *(*cave.as_mut_ptr().offset(j as
                                                                 isize)).offset(i
                                                                                    as
                                                                                    isize)
                                as *mut cave_type;
                        let mut f_ptr: *mut feature_type =
                            0 as *mut feature_type;
                        /* Apply terrain feature mimics */
                        if (*c_ptr).mimic != 0 {
                            f_ptr =
                                &mut *f_info.offset((*c_ptr).mimic as isize)
                                    as *mut feature_type
                        } else {
                            f_ptr =
                                &mut *f_info.offset((*f_info.offset((*c_ptr).feat
                                                                        as
                                                                        isize)).mimic
                                                        as isize) as
                                    *mut feature_type
                        }
                        /* Skip normal features */
                        if !((*f_ptr).flags1 as libc::c_long &
                                 0x20000 as libc::c_long == 0) {
                            /* Redraw a shimmering spot */
                            lite_spot(j, i);
                        }
                        i += 1
                    }
                    j += 1
                }
            }
            /* Handle monster detection */
            if repair_monsters != 0 {
                /* Reset the flag */
                repair_monsters = 0 as libc::c_int as bool_;
                /* Rotate detection flags */
                i = 1 as libc::c_int;
                while i < m_max as libc::c_int {
                    let mut m_ptr_0: *mut monster_type =
                        0 as *mut monster_type;
                    /* Access monster */
                    m_ptr_0 =
                        &mut *m_list.offset(i as isize) as *mut monster_type;
                    /* Skip dead monsters */
                    if !((*m_ptr_0).r_idx == 0) {
                        /* Nice monsters get mean */
                        if (*m_ptr_0).mflag & 0x20 as libc::c_int != 0 {
                            /* Nice monsters get mean */
                            (*m_ptr_0).mflag &= !(0x20 as libc::c_int)
                        }
                        /* Handle memorized monsters */
                        if (*m_ptr_0).mflag & 0x80 as libc::c_int != 0 {
                            /* Maintain detection */
                            if (*m_ptr_0).mflag & 0x40 as libc::c_int != 0 {
                                /* Forget flag */
                                (*m_ptr_0).mflag &= !(0x40 as libc::c_int);
                                /* Still need repairs */
                                repair_monsters = 1 as libc::c_int as bool_
                            } else {
                                /* Remove detection */
                                /* Forget flag */
                                (*m_ptr_0).mflag &= !(0x80 as libc::c_int);
                                (*m_ptr_0).ml = 0 as libc::c_int as bool_;
                                update_mon(i, 0 as libc::c_int as bool_);
                                lite_spot((*m_ptr_0).fy as libc::c_int,
                                          (*m_ptr_0).fx as libc::c_int);
                            }
                        }
                    }
                    i += 1
                }
            }
            /* Assume invisible */
            /* Update the monster */
            /* Redraw regardless */
            /*
			 * Moved from dungeon() -- It'll get called whenever player
			 * spends energy, so that maze isn't incredibly easy for
			 * Sorcerors and alike any longer -- pelpel
			 *
			 * Forget everything when requested hehe I'm *NASTY*
			 */
            if dun_level as libc::c_int != 0 &&
                   dungeon_flags1 as libc::c_long & 0x10000 as libc::c_long !=
                       0 {
                wiz_dark();
            }
        }
        /* Hack -- notice death */
        if alive == 0 || death as libc::c_int != 0 { break ; }
        /* Handle "leaving" */
        if (*p_ptr).leaving != 0 { break ; }
    };
}
/*
 * Interact with the current dungeon level.
 *
 * This function will not exit until the level is completed,
 * the user dies, or the game is terminated.
 */
unsafe extern "C" fn dungeon() {
    /* Reset various flags */
    hack_mind = 0 as libc::c_int as bool_;
    /* Not leaving */
    (*p_ptr).leaving = 0 as libc::c_int as bool_;
    /* Reset the "command" vars */
    command_cmd = 0 as libc::c_int as s16b;
    command_new = 0 as libc::c_int as s16b;
    command_rep = 0 as libc::c_int as s16b;
    command_arg = 0 as libc::c_int as s16b;
    command_dir = 0 as libc::c_int as s16b;
    /* Make sure partial summoning counter is initialized. */
    (*p_ptr).maintain_sum = 0 as libc::c_int as u32b;
    /* Cancel the target */
    target_who = 0 as libc::c_int as s16b;
    /* Cancel the health bar */
    health_track(0 as libc::c_int);
    /* Check visual effects */
    shimmer_monsters = 1 as libc::c_int as bool_;
    shimmer_objects = 1 as libc::c_int as bool_;
    repair_monsters = 1 as libc::c_int as bool_;
    repair_objects = 1 as libc::c_int as bool_;
    /* Disturb */
    disturb(1 as libc::c_int, 0 as libc::c_int);
    /* Track maximum player level */
    if ((*p_ptr).max_plv as libc::c_int) < (*p_ptr).lev as libc::c_int {
        (*p_ptr).max_plv = (*p_ptr).lev
    }
    /* Track maximum dungeon level (if not in quest -KMW-) */
    if (*max_dlv.offset(dungeon_type as isize) as libc::c_int) <
           dun_level as libc::c_int && (*p_ptr).inside_quest == 0 {
        *max_dlv.offset(dungeon_type as isize) = dun_level
    }
    /* No stairs down from Quest */
    if is_quest(dun_level as libc::c_int) != 0 && (*p_ptr).astral == 0 {
        create_down_stair = 0 as libc::c_int as bool_;
        create_down_shaft = 0 as libc::c_int as bool_
    }
    /* Paranoia -- no stairs from town or wilderness */
    if dun_level == 0 {
        create_up_stair = 0 as libc::c_int as bool_;
        create_down_stair = create_up_stair
    }
    if dun_level == 0 {
        create_up_shaft = 0 as libc::c_int as bool_;
        create_down_shaft = create_up_shaft
    }
    /* Option -- no connected stairs */
    if dungeon_stair == 0 {
        create_up_stair = 0 as libc::c_int as bool_;
        create_down_stair = create_up_stair
    }
    if dungeon_stair == 0 {
        create_up_shaft = 0 as libc::c_int as bool_;
        create_down_shaft = create_up_shaft
    }
    /* no connecting stairs on special levels */
    if dungeon_flags2 as libc::c_long & 0x20 as libc::c_long == 0 {
        create_up_stair = 0 as libc::c_int as bool_;
        create_down_stair = create_up_stair
    }
    if dungeon_flags2 as libc::c_long & 0x20 as libc::c_long == 0 {
        create_up_shaft = 0 as libc::c_int as bool_;
        create_down_shaft = create_up_shaft
    }
    /* Make a stairway. */
    if (create_up_stair as libc::c_int != 0 ||
            create_down_stair as libc::c_int != 0 ||
            create_up_shaft as libc::c_int != 0 ||
            create_down_shaft as libc::c_int != 0) && get_fbranch() == 0 {
        /* Place a stairway */
        if cave_valid_bold((*p_ptr).py as libc::c_int,
                           (*p_ptr).px as libc::c_int) != 0 {
            /* XXX XXX XXX */
            delete_object((*p_ptr).py as libc::c_int,
                          (*p_ptr).px as libc::c_int);
            /* Make stairs */
            if create_down_stair != 0 {
                cave_set_feat((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int,
                              if dungeon_flags1 as libc::c_long &
                                     0x400000 as libc::c_long != 0 {
                                  0xb3 as libc::c_int
                              } else { 0x7 as libc::c_int });
            } else if create_down_shaft != 0 {
                cave_set_feat((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int,
                              if dungeon_flags1 as libc::c_long &
                                     0x400000 as libc::c_long != 0 {
                                  0xb3 as libc::c_int
                              } else { 0xd as libc::c_int });
            } else if create_up_shaft != 0 {
                cave_set_feat((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int,
                              if dungeon_flags1 as libc::c_long &
                                     0x400000 as libc::c_long != 0 {
                                  0xb4 as libc::c_int
                              } else { 0xe as libc::c_int });
            } else {
                cave_set_feat((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int,
                              if dungeon_flags1 as libc::c_long &
                                     0x400000 as libc::c_long != 0 {
                                  0xb4 as libc::c_int
                              } else { 0x6 as libc::c_int });
            }
        }
        /* Cancel the stair request */
        create_up_stair = 0 as libc::c_int as bool_;
        create_down_stair = create_up_stair;
        create_up_shaft = 0 as libc::c_int as bool_;
        create_down_shaft = create_up_shaft
    }
    /* Hack - Assume invalid panel */
    panel_row_min = cur_hgt;
    panel_row_max = 0 as libc::c_int as s16b;
    panel_col_min = cur_wid;
    panel_col_max = 0 as libc::c_int as s16b;
    /* Center the panel */
    verify_panel();
    /* Flush messages */
    msg_print(0 as cptr);
    /* Enter "xtra" mode */
    character_xtra = 1 as libc::c_int as bool_;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x100 as libc::c_long) as u32b;
    /* Redraw dungeon */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x8000000 as libc::c_long | 0x2000000 as libc::c_long |
                  0x1000000 as libc::c_long)) as u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x1 as libc::c_long | 0x10 as libc::c_long |
                  0x20 as libc::c_long | 0x40 as libc::c_long |
                  0x80 as libc::c_long | 0x8 as libc::c_long |
                  0x4 as libc::c_long)) as u32b;
    /* Calculate torch radius */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as u32b;
    /* Update stuff */
    update_stuff();
    /* Redraw stuff */
    redraw_stuff();
    /* Redraw stuff */
    window_stuff();
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                  0x2000000 as libc::c_long | 0x200000 as libc::c_long)) as
            u32b;
    /* Update stuff */
    update_stuff();
    /* Redraw stuff */
    redraw_stuff();
    /* Leave "xtra" mode */
    character_xtra = 0 as libc::c_int as bool_;
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x1 as libc::c_long | 0x10 as libc::c_long |
                  0x20 as libc::c_long | 0x40 as libc::c_long |
                  0x80 as libc::c_long | 0x4 as libc::c_long)) as u32b;
    /* Combine / Reorder the pack */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* Notice stuff */
    notice_stuff();
    /* Update stuff */
    update_stuff();
    /* Redraw stuff */
    redraw_stuff();
    /* Window stuff */
    window_stuff();
    /* Refresh */
    Term_fresh();
    /* Announce (or repeat) the feeling */
    if dun_level != 0 { do_cmd_feeling(); }
    /* Hack -- notice death or departure */
    if alive == 0 || death as libc::c_int != 0 { return }
    /* ** Process this dungeon level ***/
    /* Reset the monster generation level */
    monster_level = dun_level;
    /* Reset the object generation level */
    object_level = dun_level;
    hack_mind = 1 as libc::c_int as bool_;
    /* Mega Hack, if needed wipe all stairs */
    if dungeon_type as libc::c_int == DUNGEON_DEATH {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < cur_wid as libc::c_int {
            j = 0 as libc::c_int;
            while j < cur_hgt as libc::c_int {
                let mut c_ptr: *mut cave_type =
                    &mut *(*cave.as_mut_ptr().offset(j as
                                                         isize)).offset(i as
                                                                            isize)
                        as *mut cave_type;
                match (*c_ptr).feat as libc::c_int {
                    7 | 6 | 14 | 13 => {
                        cave_set_feat(j, i, 0x1 as libc::c_int);
                    }
                    _ => { }
                }
                j += 1
            }
            i += 1
        }
        /* Reset the monster generation level */
        monster_level = 127 as libc::c_int as s16b;
        /* Reset the object generation level */
        object_level = 0 as libc::c_int as s16b
    }
    loop 
         /* Main loop */
         /* Hack -- Compact the monster list occasionally */
         {
        if m_cnt as libc::c_int + 32 as libc::c_int > max_m_idx as libc::c_int
           {
            compact_monsters(64 as libc::c_int);
        }
        /* Hack -- Compress the monster list occasionally */
        if (m_cnt as libc::c_int + 32 as libc::c_int) < m_max as libc::c_int {
            compact_monsters(0 as libc::c_int);
        }
        /* Hack -- Compact the object list occasionally */
        if o_cnt as libc::c_int + 32 as libc::c_int > max_o_idx as libc::c_int
           {
            compact_objects(64 as libc::c_int);
        }
        /* Hack -- Compress the object list occasionally */
        if (o_cnt as libc::c_int + 32 as libc::c_int) < o_max as libc::c_int {
            compact_objects(0 as libc::c_int);
        }
        /* Process the player */
        process_player();
        /* Notice stuff */
        if (*p_ptr).notice != 0 { notice_stuff(); }
        /* Update stuff */
        if (*p_ptr).update != 0 { update_stuff(); }
        /* Redraw stuff */
        if (*p_ptr).redraw != 0 { redraw_stuff(); }
        /* Redraw stuff */
        if (*p_ptr).window != 0 { window_stuff(); }
        /* Hack -- Hilite the player */
        move_cursor_relative((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int);
        /* Optional fresh */
        if fresh_after != 0 { Term_fresh(); }
        /* Hack -- Notice death or departure */
        if alive == 0 || death as libc::c_int != 0 { break ; }
        total_friends = 0 as libc::c_int;
        total_friend_levels = 0 as libc::c_int;
        /* Process all of the monsters */
        process_monsters();
        /* Notice stuff */
        if (*p_ptr).notice != 0 { notice_stuff(); }
        /* Update stuff */
        if (*p_ptr).update != 0 { update_stuff(); }
        /* Redraw stuff */
        if (*p_ptr).redraw != 0 { redraw_stuff(); }
        /* Redraw stuff */
        if (*p_ptr).window != 0 { window_stuff(); }
        /* Hack -- Hilite the player */
        move_cursor_relative((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int);
        /* Optional fresh */
        if fresh_after != 0 { Term_fresh(); }
        /* Hack -- Notice death or departure */
        if alive == 0 || death as libc::c_int != 0 { break ; }
        /* Process the world */
        process_world();
        /* Process the appropriate hooks */
        process_hooks(3 as libc::c_int,
                      b"(d)\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      is_quest(dun_level as libc::c_int));
        /* Make it pulsate and live !!!! */
        if dungeon_flags1 as libc::c_long & 0x8000000 as libc::c_long != 0 &&
               dun_level as libc::c_int != 0 {
            if turn % 10 as libc::c_int == 0 {
                evolve_level(1 as libc::c_int as bool_);
            }
        }
        /* Notice stuff */
        if (*p_ptr).notice != 0 { notice_stuff(); }
        /* Update stuff */
        if (*p_ptr).update != 0 { update_stuff(); }
        /* Redraw stuff */
        if (*p_ptr).redraw != 0 { redraw_stuff(); }
        /* Window stuff */
        if (*p_ptr).window != 0 { window_stuff(); }
        /* Hack -- Hilite the player */
        move_cursor_relative((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int);
        /* Optional fresh */
        if fresh_after != 0 { Term_fresh(); }
        /* Hack -- Notice death or departure */
        if alive == 0 || death as libc::c_int != 0 { break ; }
        /* Handle "leaving" */
        if (*p_ptr).leaving != 0 { break ; }
        /* Count game turns */
        turn += 1
    }
    /* Did we leave a dungeon ? */
    if (dun_level as libc::c_int) <
           (*d_info.offset(dungeon_type as isize)).mindepth as libc::c_int &&
           is_recall == 0 {
        dun_level = 0 as libc::c_int as s16b;
        if (*d_info.offset(dungeon_type as isize)).ix > -(1 as libc::c_int) {
            (*p_ptr).wilderness_x =
                (*d_info.offset(dungeon_type as isize)).ix;
            (*p_ptr).wilderness_y = (*d_info.offset(dungeon_type as isize)).iy
        }
        dungeon_type = 0 as libc::c_int as byte_hack
    }
    if dun_level as libc::c_int >
           (*d_info.offset(dungeon_type as isize)).maxdepth as libc::c_int {
        dun_level = 0 as libc::c_int as s16b;
        if (*d_info.offset(dungeon_type as isize)).ox > -(1 as libc::c_int) {
            (*p_ptr).wilderness_x =
                (*d_info.offset(dungeon_type as isize)).ox;
            (*p_ptr).wilderness_y = (*d_info.offset(dungeon_type as isize)).oy
        }
        dungeon_type = 0 as libc::c_int as byte_hack
    }
    is_recall = 0 as libc::c_int as bool_;
}
/*
 * Load some "user pref files"
 */
unsafe extern "C" fn load_all_pref_files() {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Access the "race" pref file */
    sprintf(buf.as_mut_ptr(),
            b"%s.prf\x00" as *const u8 as *const libc::c_char,
            rp_name.offset((*rp_ptr).title as isize));
    /* Process that file */
    process_pref_file(buf.as_mut_ptr() as cptr);
    /* Access the "class" pref file */
    sprintf(buf.as_mut_ptr(),
            b"%s.prf\x00" as *const u8 as *const libc::c_char,
            c_name.offset((*spp_ptr).title as isize));
    /* Process that file */
    process_pref_file(buf.as_mut_ptr() as cptr);
    /* Access the "character" pref file */
    sprintf(buf.as_mut_ptr(),
            b"%s.prf\x00" as *const u8 as *const libc::c_char,
            player_name.as_mut_ptr());
    /* Process that file */
    process_pref_file(buf.as_mut_ptr() as cptr);
    /* Process player specific automatizer sets */
    tome_dofile_anywhere(ANGBAND_DIR_USER,
                         format(b"%s.atm\x00" as *const u8 as
                                    *const libc::c_char,
                                player_name.as_mut_ptr()),
                         0 as libc::c_int as bool_);
}
/*
 * Actually play a game
 *
 * If the "new_game" parameter is true, then, after loading the
 * savefile, we will commit suicide, if necessary, to allow the
 * player to start a new game.
 */
#[no_mangle]
pub unsafe extern "C" fn play_game(mut new_game: bool_) {
    let mut i: libc::c_int = 0;
    let mut tmp_dun: libc::c_int = 0;
    let mut cheat_death: bool_ = 0 as libc::c_int as bool_;
    hack_corruption = 0 as libc::c_int as bool_;
    /* Hack -- Character is "icky" */
    character_icky = 1 as libc::c_int as bool_;
    /* Make sure main term is active */
    Term_activate(angband_term[0 as libc::c_int as usize]);
    /* Initialise the resize hook XXX XXX XXX */
    (*angband_term[0 as libc::c_int as usize]).resize_hook =
        Some(resize_map as unsafe extern "C" fn() -> ());
    /* XXX XXX XXX hardcoded number of terms */
    i = 1 as libc::c_int;
    while i < 8 as libc::c_int {
        if !angband_term[i as usize].is_null() {
            /* Add redraw hook */
            (*angband_term[i as usize]).resize_hook =
                Some(resize_window as unsafe extern "C" fn() -> ())
        }
        i += 1
    }
    /* Hack -- turn off the cursor */
    Term_set_cursor(0 as libc::c_int);
    /* Character list */
    if new_game == 0 && no_begin_screen == 0 { new_game = begin_screen() }
    no_begin_screen = 0 as libc::c_int as bool_;
    /* Attempt to load */
    if load_player() == 0 {
        /* Oops */
        quit(b"broken savefile\x00" as *const u8 as *const libc::c_char);
    }
    /* Nothing loaded */
    if character_loaded == 0 {
        /* Make new player */
        new_game = 1 as libc::c_int as bool_;
        /* The dungeon is not ready */
        character_dungeon = 0 as libc::c_int as bool_
    } else {
        let mut i_0: libc::c_int = 0;
        /* Init new skills to their defaults */
        i_0 = old_max_s_idx as libc::c_int;
        while i_0 < max_s_idx as libc::c_int {
            let mut value: s32b = 0 as libc::c_int;
            let mut mod_0: s32b = 0 as libc::c_int;
            compute_skills(&mut value, &mut mod_0, i_0);
            init_skill(value, mod_0, i_0);
            i_0 += 1
        }
    }
    /* Process old character */
    if new_game == 0 {
        /* Process the player name */
        process_player_name(0 as libc::c_int as bool_);
    }
    /* Init the RNG */
    if Rand_quick != 0 {
        let mut seed: u32b = 0;
        /* Basic seed */
        seed = time(0 as *mut time_t) as u32b;
        /* Mutate the seed on Unix machines */
        seed =
            (seed >>
                 3 as
                     libc::c_int).wrapping_mul((getpid() << 1 as libc::c_int)
                                                   as libc::c_uint);
        /* Use the complex RNG */
        Rand_quick = 0 as libc::c_int as bool_;
        /* Seed the "complex" RNG */
        Rand_state_init(seed);
    }
    /* Extract the options */
    i = 0 as libc::c_int;
    while !(*option_info.as_mut_ptr().offset(i as isize)).o_desc.is_null() {
        let mut os: libc::c_int =
            (*option_info.as_mut_ptr().offset(i as isize)).o_page as
                libc::c_int;
        let mut ob: libc::c_int =
            (*option_info.as_mut_ptr().offset(i as isize)).o_bit as
                libc::c_int;
        /* Set the "default" options */
        if !(*option_info.as_mut_ptr().offset(i as isize)).o_var.is_null() {
            /* Set */
            if option_flag[os as usize] as libc::c_long &
                   (1 as libc::c_long) << ob != 0 {
                /* Set */
                *(*option_info.as_mut_ptr().offset(i as isize)).o_var =
                    1 as libc::c_int as bool_
            } else {
                /* Clear */
                /* Clear */
                *(*option_info.as_mut_ptr().offset(i as isize)).o_var =
                    0 as libc::c_int as bool_
            }
        }
        i += 1
    }
    /* Roll new character */
    if new_game != 0 {
        let mut ret: s32b = 0;
        /* Are we authorized to create new chars? */
        call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
                 b"(s)\x00" as *const u8 as *const libc::c_char,
                 b"d\x00" as *const u8 as *const libc::c_char,
                 b"allow_birth\x00" as *const u8 as *const libc::c_char,
                 &mut ret as *mut s32b);
        if ret == 0 {
            msg_box(b"Sorry, this module does not allow character creation.\x00"
                        as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int), -(1 as libc::c_int));
            /* Close stuff */
            close_game();
            /* Quit */
            quit(0 as cptr);
        }
        process_hooks(22 as libc::c_int,
                      b"()\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
        /* The dungeon is not ready */
        character_dungeon = 0 as libc::c_int as bool_;
        /* Hack -- seed for flavors */
        seed_flavor = Rand_div(0x10000000 as libc::c_int) as u32b;
        /* Roll up a new character */
        player_birth();
        /* Start in town, or not */
        if (*p_ptr).astral != 0 {
            dun_level = 98 as libc::c_int as s16b
        } else { dun_level = 0 as libc::c_int as s16b }
        (*p_ptr).inside_quest = 0 as libc::c_int as s16b;
        (*p_ptr).inside_arena = 0 as libc::c_int as s16b;
        /* Hack -- enter the world */
		/* Mega-hack Vampires and Spectres start in the dungeon */
        if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                (*spp_ptr).flags1) as libc::c_long & 0x400 as libc::c_long !=
               0 {
            turn =
                (10 as libc::c_long * 11520 as libc::c_int as libc::c_long /
                     2 as libc::c_int as libc::c_long +
                     (11520 as libc::c_int *
                          (42 as libc::c_int + 127 as libc::c_int) *
                          10 as libc::c_int) as libc::c_long +
                     1 as libc::c_int as libc::c_long) as s32b
        } else {
            turn =
                11520 as libc::c_int *
                    (42 as libc::c_int + 127 as libc::c_int) *
                    10 as libc::c_int + 1 as libc::c_int
        }
    }
    /* Flash a message */
    prt(b"Please wait...\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int);
    /* Flush the message */
    Term_fresh();
    /* Be sure to not bother the player */
    calc_powers_silent = 1 as libc::c_int as bool_;
    /* Hack -- Enter wizard mode */
    if arg_wizard as libc::c_int != 0 &&
           enter_wizard_mode() as libc::c_int != 0 {
        wizard = 1 as libc::c_int as bool_
    }
    /* Flavor the objects */
    flavor_init();
    /* Reset the visual mappings */
    reset_visuals();
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x100 as libc::c_long) as u32b;
    /* Window stuff */
    window_stuff();
    /* load user file */
    process_pref_file(b"user.prf\x00" as *const u8 as *const libc::c_char);
    /* Load the "pref" files */
    load_all_pref_files();
    /* Set or clear "rogue_like_commands" if requested */
    if arg_force_original != 0 {
        rogue_like_commands = 0 as libc::c_int as bool_
    }
    if arg_force_roguelike != 0 {
        rogue_like_commands = 1 as libc::c_int as bool_
    }
    /* Initialize vault info */
    if init_v_info() != 0 {
        quit(b"Cannot initialize vaults\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialize hooks */
    init_hooks();
    ingame_help((*p_ptr).help.enabled);
    /* React to changes */
    Term_xtra(10 as libc::c_int, 0 as libc::c_int);
    /* Mega hack, prevent lots of bugs */
    if (*p_ptr).px as libc::c_int == 0 as libc::c_int ||
           (*p_ptr).py as libc::c_int == 0 as libc::c_int {
        (*p_ptr).px = 1 as libc::c_int as s16b;
        (*p_ptr).py = 1 as libc::c_int as s16b
    }
    /* Hack - if note file exists, load it */
    if new_game == 0 && take_notes as libc::c_int != 0 {
        add_note_type(4 as libc::c_int);
    }
    /* Generate a dungeon level if needed */
    if character_dungeon == 0 { generate_cave(); }
    /* Ok tell the scripts that the game is about to start */
    process_hooks(72 as libc::c_int,
                  b"()\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    /* Character is now "complete" */
    character_generated = 1 as libc::c_int as bool_;
    /* Hack -- Character is no longer "icky" */
    character_icky = 0 as libc::c_int as bool_;
    /* Start game */
    alive = 1 as libc::c_int as bool_;
    /* Hack -- Enforce "delayed death" */
    if ((*p_ptr).chp as libc::c_int) < 0 as libc::c_int {
        death = 1 as libc::c_int as bool_
    }
    /* Should we use old colors */
    if autoload_old_colors != 0 {
        process_pref_file(b"422color.prf\x00" as *const u8 as
                              *const libc::c_char);
    }
    loop 
         /* Process */
         /* Save the level */
         {
        old_dun_level = dun_level;
        (*p_ptr).old_wild_mode = (*p_ptr).wild_mode;
        /* We reached surface ? good, lets go down again !! */
        if (*p_ptr).astral as libc::c_int != 0 && dun_level == 0 {
            (*p_ptr).astral = 0 as libc::c_int as bool_;
            cmsg_print(13 as libc::c_int as byte_hack,
                       b"Well done ! You reached the town ! You can now go down again.\x00"
                           as *const u8 as *const libc::c_char);
        }
        /* Update monster list window */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x10 as libc::c_long) as u32b;
        /* Process the level */
        dungeon();
        /* Save the current level if in a persistent level */
        tmp_dun = dun_level as libc::c_int;
        dun_level = old_dun_level;
        save_dungeon();
        dun_level = tmp_dun as s16b;
        /* A death fate affects level generation */
        i = 0 as libc::c_int;
        while i < 200 as libc::c_int {
            /* Ignore empty slots */
            if !(fates[i as usize].fate == 0) {
                /* Ignore non-applicable fates */
                if !(fates[i as usize].level as libc::c_int !=
                         dun_level as libc::c_int) {
                    /* Non-serious fate fails to fire 50% of time */
                    if !(fates[i as usize].serious == 0 &&
                             Rand_div(2 as libc::c_int) == 0 as libc::c_int) {
                        /* Analyse fate */
                        match fates[i as usize].fate as libc::c_int {
                            6 => {
                                /* You are doomed */
                                cmsg_print(8 as libc::c_int as byte_hack,
                                           b"You were fated to die here.  DIE!\x00"
                                               as *const u8 as
                                               *const libc::c_char);
                                /* You shall perish there */
                                dungeon_type =
                                    DUNGEON_DEATH as byte_hack; /* was 1 */
                                dun_level =
                                    (*d_info.offset(dungeon_type as
                                                        isize)).mindepth;
                                fates[i as usize].fate =
                                    0 as libc::c_int as byte_hack
                            }
                            _ => { }
                        }
                    }
                }
            }
            i += 1
        }
        /* Notice stuff */
        if (*p_ptr).notice != 0 { notice_stuff(); }
        /* Update stuff */
        if (*p_ptr).update != 0 { update_stuff(); }
        /* Redraw stuff */
        if (*p_ptr).redraw != 0 { redraw_stuff(); }
        /* Window stuff */
        if (*p_ptr).window != 0 { window_stuff(); }
        /* Cancel the target */
        target_who = 0 as libc::c_int as s16b;
        /* Cancel the health bar */
        health_track(0 as libc::c_int);
        /* Forget the lite */
        forget_mon_lite();
        /* Forget the view */
        forget_view();
        /* Handle "quit and save" */
        if alive == 0 && death == 0 { break ; }
        /* Erase the old cave */
        wipe_o_list();
        /* XXX XXX XXX */
        msg_print(0 as cptr);
        /* Accidental Death */
        if alive as libc::c_int != 0 && death as libc::c_int != 0 {
            cheat_death = 0 as libc::c_int as bool_;
            /* Can we die ? please let us die ! */
            if process_hooks(56 as libc::c_int,
                             b"()\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char) != 0 {
                cheat_death = 1 as libc::c_int as bool_
            } else if granted_resurrection() != 0 {
                cheat_death = 1 as libc::c_int as bool_;
                (*p_ptr).grace = -(200000 as libc::c_int);
                cmsg_format(13 as libc::c_int as byte_hack,
                            b"The power of %s raises you back from the grave!\x00"
                                as *const u8 as *const libc::c_char,
                            (*deity_info.offset((*p_ptr).pgod as
                                                    isize)).name);
                msg_print(0 as cptr);
            } else if (*p_ptr).allow_one_death as libc::c_int >
                          0 as libc::c_int {
                cheat_death = 1 as libc::c_int as bool_;
                /* Deus ex machina */
                /* Blood of life */
                /* Lose one extra life */
                (*p_ptr).allow_one_death =
                    (*p_ptr).allow_one_death.wrapping_sub(1);
                cmsg_print(13 as libc::c_int as byte_hack,
                           b"You have been saved by the Blood of Life!\x00" as
                               *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
            } else if (wizard as libc::c_int != 0 ||
                           cheat_live as libc::c_int != 0) &&
                          get_check(b"Die? \x00" as *const u8 as
                                        *const libc::c_char) == 0 {
                cheat_death = 1 as libc::c_int as bool_;
                /* Cheat death option */
                /* Mark social class, reset age, if needed */
                if (*p_ptr).sc != 0 {
                    (*p_ptr).age = 0 as libc::c_int as s16b;
                    (*p_ptr).sc = (*p_ptr).age
                }
                /* Increase age */
                (*p_ptr).age += 1;
                /* Mark savefile */
                noscore =
                    (noscore as libc::c_int | 0x1 as libc::c_int) as u16b;
                msg_print(b"You invoke wizard mode and cheat death.\x00" as
                              *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
            }
            if cheat_death != 0 {
                /* Restore the winner status */
                total_winner = has_won;
                /* One more life spent */
                (*p_ptr).lives += 1;
                /* Restore hit points */
                (*p_ptr).chp = (*p_ptr).mhp;
                (*p_ptr).chp_frac = 0 as libc::c_int as u16b;
                /* Heal sanity */
                (*p_ptr).csane = (*p_ptr).msane;
                (*p_ptr).csane_frac = 0 as libc::c_int as u16b;
                /* Restore spell points */
                (*p_ptr).csp = (*p_ptr).msp;
                (*p_ptr).csp_frac = 0 as libc::c_int as u16b;
                /* Hack -- Healing */
                set_blind(0 as libc::c_int);
                set_confused(0 as libc::c_int);
                set_poisoned(0 as libc::c_int);
                set_afraid(0 as libc::c_int);
                set_paralyzed(0 as libc::c_int);
                set_image(0 as libc::c_int);
                set_stun(0 as libc::c_int);
                set_cut(0 as libc::c_int);
                /* accounting for a new ailment. -LM- */
                (*p_ptr).black_breath = 0 as libc::c_int as bool_;
                /* Hack -- don't go to undead form */
                (*p_ptr).necro_extra &= !(0x8 as libc::c_int) as libc::c_uint;
                /* Hack -- Prevent starvation */
                set_food(15000 as libc::c_int - 1 as libc::c_int);
                /* Hack -- cancel recall */
                if (*p_ptr).word_recall != 0 {
                    /* Message */
                    msg_print(b"A tension leaves the air around you...\x00" as
                                  *const u8 as *const libc::c_char);
                    msg_print(0 as cptr);
                    /* Hack -- Prevent recall */
                    (*p_ptr).word_recall = 0 as libc::c_int as s16b
                }
                /* Note cause of death XXX XXX XXX */
                strcpy(died_from.as_mut_ptr(),
                       b"Cheating death\x00" as *const u8 as
                           *const libc::c_char);
                /* Do not die */
                death = 0 as libc::c_int as bool_;
                /* New depth -KMW- */
				/* dun_level = 0; */
                (*p_ptr).inside_arena = 0 as libc::c_int as s16b;
                leaving_quest = 0 as libc::c_int;
                (*p_ptr).inside_quest = 0 as libc::c_int as s16b;
                /* Leaving */
                (*p_ptr).leaving = 1 as libc::c_int as bool_
            }
        }
        /* Handle "death" */
        if death != 0 { break ; }
        /* Mega hack */
        if dun_level != 0 { (*p_ptr).wild_mode = 0 as libc::c_int as bool_ }
        /* Make a new level */
        process_hooks(8 as libc::c_int,
                      b"(d)\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      is_quest(dun_level as libc::c_int));
        generate_cave();
    }
    /* Close stuff */
    close_game();
    /* Quit */
    quit(0 as cptr);
}
