use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn lua_rawset(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_insert(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isnumber(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_iscfunction(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tag(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tonumber(L: *mut lua_State, index: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn lua_tostring(L: *mut lua_State, index: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, index: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State);
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State, n: libc::c_long);
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction,
                        n: libc::c_int);
    #[no_mangle]
    fn lua_pushusertag(L: *mut lua_State, u: *mut libc::c_void,
                       tag: libc::c_int);
    #[no_mangle]
    fn lua_getglobal(L: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_gettable(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_getglobals(L: *mut lua_State);
    #[no_mangle]
    fn lua_newtable(L: *mut lua_State);
    #[no_mangle]
    fn lua_settable(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    static mut tolua_tag_table: libc::c_int;
    #[no_mangle]
    fn tolua_error(L: *mut lua_State, msg: *mut libc::c_char);
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_call(L: *mut lua_State, nargs: libc::c_int, nresults: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn lua_settag(L: *mut lua_State, tag: libc::c_int);
    #[no_mangle]
    fn lua_copytagmethods(L: *mut lua_State, tagto: libc::c_int,
                          tagfrom: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_newtag(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settagmethod(L: *mut lua_State, tag: libc::c_int,
                        event: *const libc::c_char);
    #[no_mangle]
    fn toluaI_tt_register(L: *mut lua_State, tag: libc::c_int,
                          type_0: *mut libc::c_char);
    #[no_mangle]
    fn toluaI_tt_sethierarchy(L: *mut lua_State, tag: libc::c_int,
                              btag: libc::c_int);
    #[no_mangle]
    fn toluaI_tt_concat(s1: *const libc::c_char, s2: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn toluaI_setregistry(L: *mut lua_State, field: *mut libc::c_char);
    #[no_mangle]
    fn toluaI_getregistry(L: *mut lua_State, field: *mut libc::c_char);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
}
pub type lua_CFunction
    =
    Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
unsafe extern "C" fn settag(mut L: *mut lua_State, mut lo: libc::c_int,
                            mut tag_registry_field: *mut libc::c_char) {
    toluaI_getregistry(L, tag_registry_field);
    lua_pushvalue(L, lo);
    lua_settag(L, lua_tonumber(L, -(2 as libc::c_int)) as libc::c_int);
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tm_global(mut L: *mut lua_State,
                                          mut lo: libc::c_int) {
    settag(L, lo,
           b"tolua_tag_global\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tm_module(mut L: *mut lua_State,
                                          mut lo: libc::c_int) {
    settag(L, lo,
           b"tolua_tag_module\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tm_setclass(mut L: *mut lua_State,
                                            mut lo: libc::c_int) {
    settag(L, lo,
           b"tolua_tag_class\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tm_class(mut L: *mut lua_State,
                                         mut lo: libc::c_int,
                                         mut name: *mut libc::c_char) {
    let mut tag_class: libc::c_int = 0;
    let mut tag: libc::c_int = lua_newtag(L);
    let mut type_0: *mut libc::c_char =
        toluaI_tt_concat(b"class \x00" as *const u8 as *const libc::c_char,
                         name);
    toluaI_getregistry(L,
                       b"tolua_tag_class\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    tag_class = lua_tonumber(L, -(1 as libc::c_int)) as libc::c_int;
    lua_copytagmethods(L, tag, tag_class);
    toluaI_tt_register(L, tag, type_0);
    toluaI_tt_sethierarchy(L, tag, tag_class);
    lua_pushvalue(L, lo);
    lua_settag(L, tag);
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    /* tag_class and lo */
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tm_instance(mut L: *mut lua_State,
                                            mut tag: libc::c_int,
                                            mut lo: libc::c_int) {
    toluaI_getregistry(L,
                       b"tolua_tbl_class\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, tag as libc::c_long);
    lua_pushvalue(L, lo);
    lua_settable(L, -(3 as libc::c_int));
    toluaI_getregistry(L,
                       b"tolua_tag_instance\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_copytagmethods(L, tag,
                       lua_tonumber(L, -(1 as libc::c_int)) as libc::c_int);
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    /* tbl_class and tag_instance */
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tm_linstance(mut L: *mut lua_State,
                                             mut tag: libc::c_int,
                                             mut lo: libc::c_int) {
    toluaI_getregistry(L,
                       b"tolua_tbl_class\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, tag as libc::c_long);
    lua_pushvalue(L, lo);
    lua_settable(L, -(3 as libc::c_int));
    toluaI_getregistry(L,
                       b"tolua_tag_linstance\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_copytagmethods(L, tag,
                       lua_tonumber(L, -(1 as libc::c_int)) as libc::c_int);
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    /* tbl_class and tag_linstance */
}
#[no_mangle]
pub unsafe extern "C" fn tolua_doclone(mut L: *mut lua_State,
                                       mut value: *mut libc::c_void,
                                       mut tag: libc::c_int)
 -> *mut libc::c_void {
    toluaI_getregistry(L,
                       b"tolua_tbl_clone\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushusertag(L, value, 0 as libc::c_int);
    lua_pushnumber(L, tag as libc::c_long);
    lua_settable(L, -(3 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_copy(mut L: *mut lua_State,
                                    mut value: *mut libc::c_void,
                                    mut size: libc::c_uint)
 -> *mut libc::c_void {
    let mut clone: *mut libc::c_void = malloc(size as libc::c_ulong);
    if !clone.is_null() {
        memcpy(clone, value, size as libc::c_ulong);
    } else {
        tolua_error(L,
                    b"insuficient memory\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
    }
    return clone;
}
unsafe extern "C" fn toluaI_tm_undoclone(mut L: *mut lua_State,
                                         mut tag: libc::c_int,
                                         mut clone: *mut libc::c_void) {
    toluaI_getregistry(L,
                       b"tolua_tbl_clone\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushusertag(L, clone, 0 as libc::c_int);
    lua_gettable(L, -(2 as libc::c_int));
    if lua_isnumber(L, -(1 as libc::c_int)) != 0 &&
           lua_tonumber(L, -(1 as libc::c_int)) == tag as libc::c_long {
        lua_pushusertag(L, clone, 0 as libc::c_int);
        lua_pushnil(L);
        lua_settable(L, -(4 as libc::c_int));
        /* tbl_class and class method table */
        toluaI_getregistry(L,
                           b"tolua_tbl_class\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char);
        lua_pushnumber(L, tag as libc::c_long);
        lua_rawget(L, -(2 as libc::c_int));
        lua_pushstring(L, b"delete\x00" as *const u8 as *const libc::c_char);
        lua_gettable(L, -(2 as libc::c_int));
        if lua_iscfunction(L, -(1 as libc::c_int)) != 0 {
            lua_pushusertag(L, clone, tag);
            lua_call(L, 1 as libc::c_int, 0 as libc::c_int);
        } else {
            /* get base class */
            /* look for destructor */
            free(clone);
            lua_settop(L,
                       -(1 as libc::c_int) -
                           1 as
                               libc::c_int); /* no destructor: use raw free */
            /* the nil function value */
        }
        lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    }
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    /* table and value */
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tm_pushmate(mut L: *mut lua_State,
                                            mut lo: libc::c_int) {
    toluaI_getregistry(L,
                       b"tolua_tbl_mate\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushvalue(L, lo);
    lua_rawget(L, -(2 as libc::c_int));
    lua_insert(L, -(2 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tm_pushclass(mut L: *mut lua_State,
                                             mut lo: libc::c_int) {
    toluaI_getregistry(L,
                       b"tolua_tbl_class\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, lua_tag(L, lo) as libc::c_long);
    lua_rawget(L, -(2 as libc::c_int));
    lua_insert(L, -(2 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_gettag(mut L: *mut lua_State,
                                       mut tagname: *mut libc::c_char)
 -> libc::c_int {
    let mut tag: libc::c_int = 0;
    toluaI_getregistry(L, tagname);
    tag = lua_tonumber(L, -(1 as libc::c_int)) as libc::c_int;
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return tag;
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tm_init(mut L: *mut lua_State) {
    lua_newtable(L);
    toluaI_setregistry(L,
                       b"tolua_tbl_class\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_newtable(L);
    toluaI_setregistry(L,
                       b"tolua_tbl_clone\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_newtable(L);
    toluaI_setregistry(L,
                       b"tolua_tbl_mate\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, lua_newtag(L) as libc::c_long);
    toluaI_setregistry(L,
                       b"tolua_tag_global\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, lua_newtag(L) as libc::c_long);
    toluaI_setregistry(L,
                       b"tolua_tag_module\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, lua_newtag(L) as libc::c_long);
    toluaI_setregistry(L,
                       b"tolua_tag_class\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, lua_newtag(L) as libc::c_long);
    toluaI_setregistry(L,
                       b"tolua_tag_instance\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, lua_newtag(L) as libc::c_long);
    toluaI_setregistry(L,
                       b"tolua_tag_linstance\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, lua_newtag(L) as libc::c_long);
    toluaI_setregistry(L,
                       b"tolua_tag_indirect\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    toluaI_tt_register(L,
                       toluaI_gettag(L,
                                     b"tolua_tag_global\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char),
                       b"generic variable\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    toluaI_tt_register(L,
                       toluaI_gettag(L,
                                     b"tolua_tag_module\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char),
                       b"generic module\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    toluaI_tt_register(L,
                       toluaI_gettag(L,
                                     b"tolua_tag_class\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char),
                       b"generic class\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    toluaI_tt_register(L,
                       toluaI_gettag(L,
                                     b"tolua_tag_indirect\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char),
                       b"generic indirect\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    toluaI_tt_register(L,
                       toluaI_gettag(L,
                                     b"tolua_tag_instance\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char),
                       b"generic instance\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    toluaI_tt_register(L,
                       toluaI_gettag(L,
                                     b"tolua_tag_linstance\x00" as *const u8
                                         as *const libc::c_char as
                                         *mut libc::c_char),
                       b"generic lua instance\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    /* allows modules and classes to be used as ordinary tables */
    toluaI_tt_sethierarchy(L,
                           toluaI_gettag(L,
                                         b"tolua_tag_module\x00" as *const u8
                                             as *const libc::c_char as
                                             *mut libc::c_char),
                           tolua_tag_table);
    toluaI_tt_sethierarchy(L,
                           toluaI_gettag(L,
                                         b"tolua_tag_class\x00" as *const u8
                                             as *const libc::c_char as
                                             *mut libc::c_char),
                           tolua_tag_table);
    setmethods(L);
}
unsafe extern "C" fn map(mut L: *mut lua_State) -> libc::c_int {
    let mut m: libc::c_int = lua_gettop(L);
    /* do not pass string fields starting with a dot */
    if lua_isstring(L, 1 as libc::c_int) == 0 ||
           *lua_tostring(L, 1 as libc::c_int) as libc::c_int != '.' as i32 {
        lua_getglobals(L); /* module table */
        lua_pushvalue(L, 1 as libc::c_int);
        lua_pushvalue(L, m);
        lua_rawset(L, -(3 as libc::c_int));
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tm_using(mut L: *mut lua_State,
                                         mut module: libc::c_int) {
    lua_newtable(L);
    lua_settag(L,
               toluaI_gettag(L,
                             b"tolua_tag_indirect\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char));
    lua_pushstring(L, b".module\x00" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, module);
    lua_settable(L, -(3 as libc::c_int));
    lua_getglobal(L, b"foreach\x00" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, module);
    lua_pushvalue(L, -(3 as libc::c_int));
    lua_pushcclosure(L,
                     Some(map as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 1 as libc::c_int);
    lua_call(L, 2 as libc::c_int, 0 as libc::c_int);
    lua_getglobal(L, b"foreach\x00" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, module);
    lua_pushstring(L, b".get\x00" as *const u8 as *const libc::c_char);
    lua_gettable(L, -(2 as libc::c_int));
    lua_insert(L, -(2 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    lua_pushvalue(L, -(3 as libc::c_int));
    lua_pushcclosure(L,
                     Some(map as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 1 as libc::c_int);
    lua_call(L, 2 as libc::c_int, 0 as libc::c_int);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    /* indirect table */
}
/* ********************************************************* tag methods */
/* tag methods coded in C */
/* generic gettable */
unsafe extern "C" fn oo_gettable(mut L: *mut lua_State,
                                 mut table: libc::c_int,
                                 mut base: libc::c_int,
                                 mut index: libc::c_int) {
    while lua_type(L, base) == 4 as libc::c_int {
        lua_pushvalue(L, index); /* returned value already on the top */
        lua_rawget(L, base); /* need to access array field (?) */
        if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
            return
        } else {
            if lua_isnumber(L, index) != 0 {
                lua_pushstring(L,
                               b"operator_get\x00" as *const u8 as
                                   *const libc::c_char);
                lua_rawget(L, base);
                if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
                    lua_pushvalue(L, table);
                    lua_pushvalue(L, index);
                    lua_call(L, 2 as libc::c_int, 1 as libc::c_int);
                    return
                }
            } else {
                lua_pushstring(L,
                               b".get\x00" as *const u8 as
                                   *const libc::c_char);
                lua_rawget(L, base);
                if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
                    lua_pushvalue(L, index);
                    lua_rawget(L, -(2 as libc::c_int));
                    if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int)
                       {
                        lua_pushvalue(L, table);
                        lua_pushvalue(L, index);
                        lua_call(L, 2 as libc::c_int, 1 as libc::c_int);
                        return
                    }
                }
            }
        }
        lua_pushstring(L, b".base\x00" as *const u8 as *const libc::c_char);
        lua_rawget(L, base);
        base = lua_gettop(L)
    }
    lua_pushnil(L);
}
/* generic settable */
unsafe extern "C" fn oo_settable(mut L: *mut lua_State,
                                 mut table: libc::c_int,
                                 mut base: libc::c_int,
                                 mut index: libc::c_int,
                                 mut value: libc::c_int) -> libc::c_int {
    while lua_type(L, base) == 4 as libc::c_int {
        lua_pushstring(L, b".set\x00" as *const u8 as *const libc::c_char);
        lua_rawget(L, base);
        if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
            lua_pushvalue(L, index);
            lua_rawget(L, -(2 as libc::c_int));
            if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
                lua_pushvalue(L, table);
                lua_pushvalue(L, value);
                lua_call(L, 2 as libc::c_int, 0 as libc::c_int);
                return 1 as libc::c_int
            }
        }
        lua_pushstring(L, b".base\x00" as *const u8 as *const libc::c_char);
        lua_rawget(L, base);
        base = lua_gettop(L)
    }
    return 0 as libc::c_int;
}
/* class tag methods */
unsafe extern "C" fn class_index(mut L: *mut lua_State) -> libc::c_int {
    let mut table: libc::c_int = 1 as libc::c_int;
    let mut index: libc::c_int = 2 as libc::c_int;
    oo_gettable(L, table, table, index);
    return 1 as libc::c_int;
}
unsafe extern "C" fn class_settable(mut L: *mut lua_State) -> libc::c_int {
    let mut table: libc::c_int = 1 as libc::c_int;
    let mut index: libc::c_int = 2 as libc::c_int;
    let mut value: libc::c_int = 3 as libc::c_int;
    if oo_settable(L, table, table, index, value) == 0 as libc::c_int {
        lua_pushvalue(L, table);
        lua_pushvalue(L, index);
        lua_pushvalue(L, value);
        lua_rawset(L, -(3 as libc::c_int));
    }
    return 0 as libc::c_int;
}
/* instance tags */
unsafe extern "C" fn instance_gettable(mut L: *mut lua_State) -> libc::c_int {
    let mut table: libc::c_int = 1 as libc::c_int; /* pushes mate */
    let mut index: libc::c_int = 2 as libc::c_int;
    toluaI_tm_pushmate(L, table);
    if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
        /* if there's a mate table */
        lua_pushvalue(L, index);
        lua_rawget(L, -(2 as libc::c_int));
        if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
            /* if field in mate table exists */
            return 1 as libc::c_int
        }
    } /* pushes base */
    toluaI_tm_pushclass(L, table); /* pushes base */
    oo_gettable(L, table, lua_gettop(L), index);
    return 1 as libc::c_int;
}
unsafe extern "C" fn instance_settable(mut L: *mut lua_State) -> libc::c_int {
    let mut table: libc::c_int = 1 as libc::c_int;
    let mut index: libc::c_int = 2 as libc::c_int;
    let mut value: libc::c_int = 3 as libc::c_int;
    toluaI_tm_pushclass(L, table);
    if lua_isnumber(L, index) != 0 {
        lua_pushstring(L,
                       b"operator_set\x00" as *const u8 as
                           *const libc::c_char);
        lua_rawget(L, -(2 as libc::c_int));
        if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
            /* the stack here is: table,index,value,base,operator */
   /* call operator passing table, index, and value */
            lua_insert(L, 1 as libc::c_int); /* base */
            lua_settop(L,
                       -(1 as libc::c_int) -
                           1 as libc::c_int); /* pushes mate */
            lua_call(L, 3 as libc::c_int, 0 as libc::c_int);
            return 0 as libc::c_int
        }
    }
    if oo_settable(L, table, 4 as libc::c_int, index, value) ==
           0 as libc::c_int {
        toluaI_tm_pushmate(L, table);
        if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
            /* creates mate table */
            lua_newtable(L);
            toluaI_getregistry(L,
                               b"tolua_tbl_mate\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char);
            /* tbl_mate */
            lua_pushvalue(L, table); /* that is the userdata */
            lua_pushvalue(L, -(3 as libc::c_int));
            lua_rawset(L, -(3 as libc::c_int));
            lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        }
        /* the mate table is on the top */
        lua_pushvalue(L, index);
        lua_pushvalue(L, value);
        lua_rawset(L, -(3 as libc::c_int));
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn instance_gc(mut L: *mut lua_State) -> libc::c_int {
    toluaI_tm_undoclone(L, lua_tag(L, 1 as libc::c_int),
                        lua_touserdata(L, 1 as libc::c_int));
    return 0 as libc::c_int;
}
unsafe extern "C" fn gen_operator(mut L: *mut lua_State) -> libc::c_int {
    let mut op1: libc::c_int = 1 as libc::c_int;
    let mut op2: libc::c_int = 2 as libc::c_int;
    let mut event: libc::c_int = 3 as libc::c_int;
    let mut name: *mut libc::c_char =
        toluaI_tt_concat(b"operator_\x00" as *const u8 as *const libc::c_char,
                         lua_tostring(L, event));
    lua_pushstring(L, name);
    lua_gettable(L, op1);
    lua_pushvalue(L, op1);
    lua_pushvalue(L, op2);
    lua_call(L, 2 as libc::c_int, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn instance_operator(mut L: *mut lua_State) -> libc::c_int {
    return gen_operator(L);
}
unsafe extern "C" fn instance_relational(mut L: *mut lua_State)
 -> libc::c_int {
    gen_operator(L);
    if lua_tonumber(L, -(1 as libc::c_int)) as libc::c_int == 0 as libc::c_int
       {
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
/* lua instance tags */
unsafe extern "C" fn linstance_index(mut L: *mut lua_State) -> libc::c_int {
    toluaI_tm_pushclass(L, 1 as libc::c_int); /* table,base,index */
    oo_gettable(L, 1 as libc::c_int, 3 as libc::c_int, 2 as libc::c_int);
    return 1 as libc::c_int;
}
/* module tag methods */
unsafe extern "C" fn module_index(mut L: *mut lua_State) -> libc::c_int {
    let mut table: libc::c_int = 1 as libc::c_int;
    let mut index: libc::c_int = 2 as libc::c_int;
    lua_pushstring(L, b".get\x00" as *const u8 as *const libc::c_char);
    lua_rawget(L, table);
    if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
        lua_pushvalue(L, index);
        lua_rawget(L, -(2 as libc::c_int));
        if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
            lua_call(L, 0 as libc::c_int, 1 as libc::c_int);
            return 1 as libc::c_int
        }
    }
    lua_pushnil(L);
    return 1 as libc::c_int;
}
unsafe extern "C" fn module_settable(mut L: *mut lua_State) -> libc::c_int {
    let mut table: libc::c_int = 1 as libc::c_int;
    let mut index: libc::c_int = 2 as libc::c_int;
    let mut value: libc::c_int = 3 as libc::c_int;
    lua_pushstring(L, b".set\x00" as *const u8 as *const libc::c_char);
    lua_rawget(L, table);
    if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
        lua_pushvalue(L, index);
        lua_rawget(L, -(2 as libc::c_int));
        if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
            lua_pushvalue(L, value);
            lua_call(L, 1 as libc::c_int, 0 as libc::c_int);
            return 0 as libc::c_int
        }
    }
    lua_pushvalue(L, index);
    lua_pushvalue(L, value);
    lua_rawset(L, table);
    return 0 as libc::c_int;
}
/* global variable tag methods */
unsafe extern "C" fn global_getglobal(mut L: *mut lua_State) -> libc::c_int {
    let mut value: libc::c_int = 2 as libc::c_int;
    lua_pushstring(L, b".get\x00" as *const u8 as *const libc::c_char);
    lua_rawget(L, value);
    lua_call(L, 0 as libc::c_int, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn global_setglobal(mut L: *mut lua_State) -> libc::c_int {
    let mut value: libc::c_int = 2 as libc::c_int;
    let mut newvalue: libc::c_int = 3 as libc::c_int;
    lua_pushstring(L, b".set\x00" as *const u8 as *const libc::c_char);
    lua_rawget(L, value);
    if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
        lua_error(L,
                  b"value of const variable cannot be changed\x00" as
                      *const u8 as *const libc::c_char);
    } else {
        lua_pushvalue(L, newvalue);
        lua_call(L, 1 as libc::c_int, 0 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/* indirect tag methods */
unsafe extern "C" fn indirect_getglobal(mut L: *mut lua_State)
 -> libc::c_int {
    let mut varname: libc::c_int = 1 as libc::c_int;
    let mut value: libc::c_int = 2 as libc::c_int;
    lua_pushstring(L, b".module\x00" as *const u8 as *const libc::c_char);
    lua_gettable(L, value);
    lua_pushvalue(L, varname);
    lua_gettable(L, -(2 as libc::c_int));
    return 1 as libc::c_int;
}
unsafe extern "C" fn indirect_setglobal(mut L: *mut lua_State)
 -> libc::c_int {
    let mut varname: libc::c_int = 1 as libc::c_int;
    let mut value: libc::c_int = 2 as libc::c_int;
    let mut newvalue: libc::c_int = 3 as libc::c_int;
    lua_pushstring(L, b".module\x00" as *const u8 as *const libc::c_char);
    lua_gettable(L, value);
    lua_pushvalue(L, varname);
    lua_pushvalue(L, newvalue);
    lua_settable(L, -(3 as libc::c_int));
    return 0 as libc::c_int;
}
/* tolua: tag methods
** Support code for Lua bindings.
** Written by Waldemar Celes
** TeCGraf/PUC-Rio
** Jul 1998
** $Id: tolua_tm.c,v 1.2 2001/11/26 23:00:27 darkgod Exp $
*/
/* This code is free software; you can redistribute it and/or modify it. 
** The software provided hereunder is on an "as is" basis, and 
** the author has no obligation to provide maintenance, support, updates,
** enhancements, or modifications. 
*/
/* Global tables created in Lua registry:
    tolua_tbl_class: indexed by instance tags, stores the class tables.
    tolua_tbl_clone: indexed by memory address, stores the tag indicanting 
                     it is a clone.
    tolua_tbl_mate:  indexed by memory address, stores the associate instance
                     table.

   General tags stored in Lua registry:
    tolua_tag_global;
    tolua_tag_module;
    tolua_tag_class;
    tolua_tag_instance;
    tolua_tag_linstance;
    tolua_tag_indirect;
*/
/* internal function prototype */
unsafe extern "C" fn setmethods(mut L: *mut lua_State) {
    /* global variable */
    lua_pushcclosure(L,
                     Some(global_getglobal as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_global\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"getglobal\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(global_setglobal as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_global\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"setglobal\x00" as *const u8 as *const libc::c_char);
    /* module */
    lua_pushcclosure(L,
                     Some(module_index as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_module\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"index\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(module_settable as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_module\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"settable\x00" as *const u8 as *const libc::c_char);
    /* class */
    lua_pushcclosure(L,
                     Some(class_index as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_class\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"index\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(class_settable as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_class\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"settable\x00" as *const u8 as *const libc::c_char);
    /* instance */
    lua_pushcclosure(L,
                     Some(instance_gettable as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_instance\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"gettable\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(instance_settable as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_instance\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"settable\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(instance_operator as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_instance\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"add\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(instance_operator as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_instance\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"sub\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(instance_operator as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_instance\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"mul\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(instance_operator as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_instance\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"div\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(instance_relational as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_instance\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"lt\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(instance_gc as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_instance\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"gc\x00" as *const u8 as *const libc::c_char);
    /* lua instance */
    lua_pushcclosure(L,
                     Some(linstance_index as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_linstance\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"index\x00" as *const u8 as *const libc::c_char);
    /* indirect */
    lua_pushcclosure(L,
                     Some(indirect_getglobal as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_indirect\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"getglobal\x00" as *const u8 as *const libc::c_char);
    lua_pushcclosure(L,
                     Some(indirect_setglobal as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_settagmethod(L,
                     toluaI_gettag(L,
                                   b"tolua_tag_indirect\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char),
                     b"setglobal\x00" as *const u8 as *const libc::c_char);
}
