use ::libc;
extern "C" {
    pub type TM;
    pub type lua_longjmp;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn luaM_realloc(L: *mut lua_State, oldblock: *mut libc::c_void,
                    size: lint32) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_State {
    pub top: StkId,
    pub stack: StkId,
    pub stack_last: StkId,
    pub stacksize: libc::c_int,
    pub Cbase: StkId,
    pub errorJmp: *mut lua_longjmp,
    pub Mbuffer: *mut libc::c_char,
    pub Mbuffsize: size_t,
    pub rootproto: *mut Proto,
    pub rootcl: *mut Closure,
    pub roottable: *mut Hash,
    pub strt: stringtable,
    pub udt: stringtable,
    pub gt: *mut Hash,
    pub TMtable: *mut TM,
    pub last_tag: libc::c_int,
    pub refArray: *mut Ref,
    pub refSize: libc::c_int,
    pub refFree: libc::c_int,
    pub GCthreshold: libc::c_ulong,
    pub nblocks: libc::c_ulong,
    pub callhook: lua_Hook,
    pub linehook: lua_Hook,
    pub allowhooks: libc::c_int,
}
pub type lua_Hook
    =
    Option<unsafe extern "C" fn(_: *mut lua_State, _: *mut lua_Debug) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: *const libc::c_char,
    pub currentline: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub nups: libc::c_int,
    pub linedefined: libc::c_int,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub short_src: [libc::c_char; 60],
    pub _func: *mut lua_TObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_TObject {
    pub ttype: libc::c_int,
    pub value: Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub ts: *mut TString,
    pub cl: *mut Closure,
    pub a: *mut Hash,
    pub i: *mut CallInfo,
    pub n: Number,
}
pub type Number = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: *mut Closure,
    pub pc: *mut *const Instruction,
    pub lastpc: libc::c_int,
    pub line: libc::c_int,
    pub refi: libc::c_int,
}
pub type Instruction = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Closure {
    pub f: C2RustUnnamed,
    pub next: *mut Closure,
    pub mark: *mut Closure,
    pub isC: libc::c_short,
    pub nupvalues: libc::c_short,
    pub upvalue: [TObject; 1],
}
pub type TObject = lua_TObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c: lua_CFunction,
    pub l: *mut Proto,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Proto {
    pub knum: *mut Number,
    pub nknum: libc::c_int,
    pub kstr: *mut *mut TString,
    pub nkstr: libc::c_int,
    pub kproto: *mut *mut Proto,
    pub nkproto: libc::c_int,
    pub code: *mut Instruction,
    pub ncode: libc::c_int,
    pub numparams: libc::c_short,
    pub is_vararg: libc::c_short,
    pub maxstacksize: libc::c_short,
    pub marked: libc::c_short,
    pub next: *mut Proto,
    pub lineinfo: *mut libc::c_int,
    pub nlineinfo: libc::c_int,
    pub nlocvars: libc::c_int,
    pub locvars: *mut LocVar,
    pub lineDefined: libc::c_int,
    pub source: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub u: C2RustUnnamed_0,
    pub len: size_t,
    pub nexthash: *mut TString,
    pub marked: libc::c_int,
    pub str_0: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: C2RustUnnamed_2,
    pub d: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tag: libc::c_int,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub hash: libc::c_ulong,
    pub constindex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
pub type lua_CFunction
    =
    Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub node: *mut Node,
    pub htag: libc::c_int,
    pub size: libc::c_int,
    pub firstfree: *mut Node,
    pub next: *mut Hash,
    pub mark: *mut Hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub key: TObject,
    pub val: TObject,
    pub next: *mut Node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ref {
    pub o: TObject,
    pub st: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable {
    pub size: libc::c_int,
    pub nuse: lint32,
    pub hash: *mut *mut TString,
}
pub type lint32 = libc::c_ulong;
pub type StkId = *mut TObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    pub d: libc::c_long,
    pub s: *mut libc::c_char,
    pub l: libc::c_long,
}
/*
** $Id: lstring.c,v 1.3 2001/11/26 23:00:26 darkgod Exp $
** String table (keeps all strings handled by Lua)
** See Copyright Notice in lua.h
*/
/*
** type equivalent to TString, but with maximum alignment requirements
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_UTString {
    pub ts: TString,
    pub dummy: L_Umaxalign,
}
/* ensures maximum alignment for `local' udata */
#[no_mangle]
pub unsafe extern "C" fn luaS_init(mut L: *mut lua_State) {
    (*L).strt.hash =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (1 as libc::c_int as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                          as libc::c_ulong))
            as *mut *mut TString; /* seed */
    (*L).udt.hash =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (1 as libc::c_int as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                          as libc::c_ulong))
            as
            *mut *mut TString; /* if string is too long, don't hash all its chars */
    (*L).nblocks =
        (*L).nblocks.wrapping_add((2 as libc::c_int as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                                       as
                                                                       libc::c_ulong));
    (*L).udt.size = 1 as libc::c_int;
    (*L).strt.size = (*L).udt.size;
    (*L).udt.nuse = 0 as libc::c_int as lint32;
    (*L).strt.nuse = (*L).udt.nuse;
    let ref mut fresh0 = *(*L).udt.hash.offset(0 as libc::c_int as isize);
    *fresh0 = 0 as *mut TString;
    let ref mut fresh1 = *(*L).strt.hash.offset(0 as libc::c_int as isize);
    *fresh1 = *fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_freeall(mut L: *mut lua_State) {
    (*L).nblocks =
        (*L).nblocks.wrapping_sub((((*L).strt.size + (*L).udt.size) as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                                       as
                                                                       libc::c_ulong));
    luaM_realloc(L, (*L).strt.hash as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    luaM_realloc(L, (*L).udt.hash as *mut libc::c_void,
                 0 as libc::c_int as lint32);
}
unsafe extern "C" fn hash_s(mut s: *const libc::c_char, mut l: size_t)
 -> libc::c_ulong {
    let mut h: libc::c_ulong = l;
    let mut step: size_t =
        l >> 5 as libc::c_int | 1 as libc::c_int as libc::c_ulong;
    while l >= step {
        let fresh2 = s;
        s = s.offset(1);
        h =
            h ^
                (h <<
                     5 as
                         libc::c_int).wrapping_add(h >>
                                                       2 as
                                                           libc::c_int).wrapping_add(*fresh2
                                                                                         as
                                                                                         libc::c_uchar
                                                                                         as
                                                                                         libc::c_ulong);
        l = (l as libc::c_ulong).wrapping_sub(step) as size_t as size_t
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_resize(mut L: *mut lua_State,
                                     mut tb: *mut stringtable,
                                     mut newsize: libc::c_int) {
    let mut newhash: *mut *mut TString =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (newsize as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                          as libc::c_ulong))
            as *mut *mut TString;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < newsize {
        let ref mut fresh3 = *newhash.offset(i as isize);
        *fresh3 = 0 as *mut TString;
        i += 1
    }
    /* rehash */
    i = 0 as libc::c_int;
    while i < (*tb).size {
        let mut p: *mut TString = *(*tb).hash.offset(i as isize);
        while !p.is_null() {
            /* for each node in the list */
            let mut next: *mut TString = (*p).nexthash; /* save next */
            let mut h: libc::c_ulong =
                if tb == &mut (*L).strt as *mut stringtable {
                    (*p).u.s.hash
                } else {
                    ((*p).u.d.value as libc::c_ulong) >> 3 as libc::c_int
                }; /* new position */
            let mut h1: libc::c_int =
                (h & (newsize - 1 as libc::c_int) as libc::c_ulong) as
                    libc::c_int; /* chain it in new position */
            (*p).nexthash =
                *newhash.offset(h1 as isize); /* chain new entry */
            let ref mut fresh4 = *newhash.offset(h1 as isize);
            *fresh4 = p;
            p = next
        }
        i += 1
    }
    luaM_realloc(L, (*tb).hash as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    (*L).nblocks =
        (*L).nblocks.wrapping_add(((newsize - (*tb).size) as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                                       as
                                                                       libc::c_ulong));
    (*tb).size = newsize;
    (*tb).hash = newhash;
}
unsafe extern "C" fn newentry(mut L: *mut lua_State, mut tb: *mut stringtable,
                              mut ts: *mut TString, mut h: libc::c_int) {
    (*ts).nexthash = *(*tb).hash.offset(h as isize);
    let ref mut fresh5 = *(*tb).hash.offset(h as isize);
    *fresh5 = ts;
    (*tb).nuse = (*tb).nuse.wrapping_add(1);
    if (*tb).nuse > (*tb).size as lint32 &&
           (*tb).size <
               (2147483647 as libc::c_int - 2 as libc::c_int) /
                   2 as libc::c_int {
        /* too crowded? */
        luaS_resize(L, tb, (*tb).size * 2 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newlstr(mut L: *mut lua_State,
                                      mut str: *const libc::c_char,
                                      mut l: size_t) -> *mut TString {
    let mut h: libc::c_ulong = hash_s(str, l);
    let mut h1: libc::c_int =
        (h & ((*L).strt.size - 1 as libc::c_int) as libc::c_ulong) as
            libc::c_int;
    let mut ts: *mut TString = 0 as *mut TString;
    ts = *(*L).strt.hash.offset(h1 as isize);
    while !ts.is_null() {
        if (*ts).len == l &&
               memcmp(str as *const libc::c_void,
                      (*ts).str_0.as_mut_ptr() as *const libc::c_void, l) ==
                   0 as libc::c_int {
            return ts
        }
        ts = (*ts).nexthash
    }
    /* not found */
    ts =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (::std::mem::size_of::<TString>() as libc::c_ulong as
                          libc::c_long +
                          (l.wrapping_add(1 as libc::c_int as libc::c_ulong)
                               as libc::c_long -
                               ::std::mem::size_of::<libc::c_int>() as
                                   libc::c_ulong as libc::c_int as
                                   libc::c_long) *
                              ::std::mem::size_of::<libc::c_char>() as
                                  libc::c_ulong as libc::c_long) as lint32) as
            *mut TString; /* ending 0 */
    (*ts).marked = 0 as libc::c_int; /* insert it on table */
    (*ts).nexthash = 0 as *mut TString;
    (*ts).len = l;
    (*ts).u.s.hash = h;
    (*ts).u.s.constindex = 0 as libc::c_int;
    memcpy((*ts).str_0.as_mut_ptr() as *mut libc::c_void,
           str as *const libc::c_void, l);
    (*ts).str_0[l as usize] = 0 as libc::c_int as libc::c_char;
    (*L).nblocks =
        (*L).nblocks.wrapping_add((::std::mem::size_of::<TString>() as
                                       libc::c_ulong as libc::c_long +
                                       (l.wrapping_add(1 as libc::c_int as
                                                           libc::c_ulong) as
                                            libc::c_long -
                                            ::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong as
                                                libc::c_int as libc::c_long) *
                                           ::std::mem::size_of::<libc::c_char>()
                                               as libc::c_ulong as
                                               libc::c_long) as
                                      libc::c_ulong);
    newentry(L, &mut (*L).strt, ts, h1);
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newudata(mut L: *mut lua_State, mut s: size_t,
                                       mut udata: *mut libc::c_void)
 -> *mut TString {
    let mut uts: *mut L_UTString =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (::std::mem::size_of::<L_UTString>() as
                          libc::c_ulong).wrapping_add(s)) as *mut L_UTString;
    let mut ts: *mut TString = &mut (*uts).ts;
    (*ts).marked = 0 as libc::c_int;
    (*ts).nexthash = 0 as *mut TString;
    (*ts).len = s;
    (*ts).u.d.tag = 0 as libc::c_int;
    (*ts).u.d.value =
        if udata.is_null() {
            uts.offset(1 as libc::c_int as isize) as *mut libc::c_void
        } else { udata };
    (*L).nblocks =
        (*L).nblocks.wrapping_add((::std::mem::size_of::<TString>() as
                                       libc::c_ulong as libc::c_long +
                                       (s.wrapping_add(1 as libc::c_int as
                                                           libc::c_ulong) as
                                            libc::c_long -
                                            ::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong as
                                                libc::c_int as libc::c_long) *
                                           ::std::mem::size_of::<libc::c_char>()
                                               as libc::c_ulong as
                                               libc::c_long) as
                                      libc::c_ulong);
    /* insert it on table */
    newentry(L, &mut (*L).udt, ts,
             ((*ts).u.d.value as libc::c_ulong >> 3 as libc::c_int &
                  ((*L).udt.size - 1 as libc::c_int) as libc::c_ulong) as
                 libc::c_int);
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_createudata(mut L: *mut lua_State,
                                          mut udata: *mut libc::c_void,
                                          mut tag: libc::c_int)
 -> *mut TString {
    let mut h1: libc::c_int =
        (udata as libc::c_ulong >> 3 as libc::c_int &
             ((*L).udt.size - 1 as libc::c_int) as libc::c_ulong) as
            libc::c_int;
    let mut ts: *mut TString = 0 as *mut TString;
    ts = *(*L).udt.hash.offset(h1 as isize);
    while !ts.is_null() {
        if udata == (*ts).u.d.value &&
               (tag == (*ts).u.d.tag || tag == -(1 as libc::c_int)) {
            return ts
        }
        ts = (*ts).nexthash
    }
    /* not found */
    ts = luaS_newudata(L, 0 as libc::c_int as size_t, udata); /* avoid GC */
    if tag != -(1 as libc::c_int) { (*ts).u.d.tag = tag }
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_new(mut L: *mut lua_State,
                                  mut str: *const libc::c_char)
 -> *mut TString {
    return luaS_newlstr(L, str, strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newfixed(mut L: *mut lua_State,
                                       mut str: *const libc::c_char)
 -> *mut TString {
    let mut ts: *mut TString = luaS_new(L, str);
    if (*ts).marked == 0 as libc::c_int { (*ts).marked = 2 as libc::c_int }
    return ts;
}
