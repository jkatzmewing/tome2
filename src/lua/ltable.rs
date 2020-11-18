use ::libc;
extern "C" {
    pub type TM;
    pub type lua_longjmp;
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn luaM_realloc(L: *mut lua_State, oldblock: *mut libc::c_void,
                    size: lint32) -> *mut libc::c_void;
    #[no_mangle]
    static luaO_nilobject: TObject;
    #[no_mangle]
    fn luaO_power2(n: lint32) -> lint32;
    #[no_mangle]
    fn luaO_equalObj(t1: *const TObject, t2: *const TObject) -> libc::c_int;
    #[no_mangle]
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
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
/*
** returns the `main' position of an element in a table (that is, the index
** of its hash value)
*/
#[no_mangle]
pub unsafe extern "C" fn luaH_mainposition(mut t: *const Hash,
                                           mut key: *const TObject)
 -> *mut Node {
    let mut h: libc::c_ulong = 0;
    match (*key).ttype {
        2 => {
            h = (*key).value.n as libc::c_ulong
            /* invalid key */
        }
        3 => { h = (*(*key).value.ts).u.s.hash }
        0 => { h = (*key).value.ts as libc::c_ulong >> 3 as libc::c_int }
        4 => { h = (*key).value.a as libc::c_ulong >> 3 as libc::c_int }
        5 => { h = (*key).value.cl as libc::c_ulong >> 3 as libc::c_int }
        _ => { return 0 as *mut Node }
    }
    return &mut *(*t).node.offset((h &
                                       ((*t).size - 1 as libc::c_int) as
                                           libc::c_ulong) as isize) as
               *mut Node;
}
unsafe extern "C" fn luaH_getany(mut L: *mut lua_State, mut t: *const Hash,
                                 mut key: *const TObject) -> *const TObject {
    let mut n: *mut Node = luaH_mainposition(t, key);
    if n.is_null() {
        lua_error(L,
                  b"table index is nil\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        loop  {
            if luaO_equalObj(key, &mut (*n).key) != 0 { return &mut (*n).val }
            n = (*n).next;
            if n.is_null() { break ; }
        }
    }
    return &luaO_nilobject;
    /* key not found */
}
/* specialized version for numbers */
#[no_mangle]
pub unsafe extern "C" fn luaH_getnum(mut t: *const Hash, mut key: Number)
 -> *const TObject {
    let mut n: *mut Node =
        &mut *(*t).node.offset((key as libc::c_ulong &
                                    ((*t).size - 1 as libc::c_int) as
                                        libc::c_ulong) as isize) as *mut Node;
    loop  {
        if (*n).key.ttype == 2 as libc::c_int && (*n).key.value.n == key {
            return &mut (*n).val
        }
        n = (*n).next;
        if n.is_null() { break ; }
    }
    return &luaO_nilobject;
    /* key not found */
}
/* specialized version for strings */
#[no_mangle]
pub unsafe extern "C" fn luaH_getstr(mut t: *const Hash,
                                     mut key: *mut TString)
 -> *const TObject {
    let mut n: *mut Node =
        &mut *(*t).node.offset(((*key).u.s.hash &
                                    ((*t).size - 1 as libc::c_int) as
                                        libc::c_ulong) as isize) as *mut Node;
    loop  {
        if (*n).key.ttype == 3 as libc::c_int && (*n).key.value.ts == key {
            return &mut (*n).val
        }
        n = (*n).next;
        if n.is_null() { break ; }
    }
    return &luaO_nilobject;
    /* key not found */
}
#[no_mangle]
pub unsafe extern "C" fn luaH_get(mut L: *mut lua_State, mut t: *const Hash,
                                  mut key: *const TObject) -> *const TObject {
    match (*key).ttype {
        2 => { return luaH_getnum(t, (*key).value.n) }
        3 => { return luaH_getstr(t, (*key).value.ts) }
        _ => { return luaH_getany(L, t, key) }
    }; /* first iteration */
}
#[no_mangle]
pub unsafe extern "C" fn luaH_next(mut L: *mut lua_State, mut t: *const Hash,
                                   mut key: *const TObject) -> *mut Node {
    let mut i: libc::c_int = 0;
    if (*key).ttype == 1 as libc::c_int {
        i = 0 as libc::c_int
    } else {
        let mut v: *const TObject = luaH_get(L, t, key);
        if v == &luaO_nilobject as *const TObject {
            lua_error(L,
                      b"invalid key for `next\'\x00" as *const u8 as
                          *const libc::c_char);
        }
        i =
            ((v as
                  *const libc::c_char).wrapping_offset_from(&mut (*(*t).node.offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).val
                                                                as
                                                                *mut TObject
                                                                as
                                                                *const libc::c_char)
                 as libc::c_long as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<Node>() as
                                                 libc::c_ulong) as libc::c_int
                + 1 as libc::c_int
    }
    while i < (*t).size {
        let mut n: *mut Node =
            &mut *(*t).node.offset(i as isize) as *mut Node;
        if (*n).val.ttype != 1 as libc::c_int { return n }
        i += 1
    }
    return 0 as *mut Node;
    /* no more elements */
}
/*
** try to remove a key without value from a table. To avoid problems with
** hash, change `key' for a number with the same hash.
*/
#[no_mangle]
pub unsafe extern "C" fn luaH_remove(mut t: *mut Hash,
                                     mut key: *mut TObject) {
    if (*key).ttype == 2 as libc::c_int ||
           (*key).ttype == 3 as libc::c_int &&
               (*(*key).value.ts).len <= 30 as libc::c_int as libc::c_ulong {
        return
    } else { /* do not remove numbers nor small strings */
        /* try to find a number `n' with the same hash as `key' */
        let mut mp: *mut Node = luaH_mainposition(t, key);
        let mut n: libc::c_int =
            mp.wrapping_offset_from(&mut *(*t).node.offset(0 as libc::c_int as
                                                               isize) as
                                        *mut Node) as libc::c_long as
                libc::c_int;
        /* make sure `n' is not in `t' */
        while luaH_getnum(t, n as Number) != &luaO_nilobject as *const TObject
              {
            if n >= 2147483647 as libc::c_int - 2 as libc::c_int - (*t).size {
                return
            } /* give up; (to avoid overflow) */
            n += (*t).size
        }
        (*key).ttype = 2 as libc::c_int;
        (*key).value.n = n as Number
    };
}
unsafe extern "C" fn setnodevector(mut L: *mut lua_State, mut t: *mut Hash,
                                   mut size: lint32) {
    let mut i: libc::c_int = 0;
    if size > (2147483647 as libc::c_int - 2 as libc::c_int) as libc::c_ulong
       {
        lua_error(L,
                  b"table overflow\x00" as *const u8 as *const libc::c_char);
    }
    (*t).node =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     size.wrapping_mul(::std::mem::size_of::<Node>() as
                                           libc::c_ulong)) as *mut Node;
    i = 0 as libc::c_int;
    while i < size as libc::c_int {
        let ref mut fresh0 = (*(*t).node.offset(i as isize)).val.ttype;
        *fresh0 = 1 as libc::c_int;
        (*(*t).node.offset(i as isize)).key.ttype = *fresh0;
        let ref mut fresh1 = (*(*t).node.offset(i as isize)).next;
        *fresh1 = 0 as *mut Node;
        i += 1
    }
    (*L).nblocks =
        (*L).nblocks.wrapping_add((::std::mem::size_of::<Hash>() as
                                       libc::c_ulong).wrapping_add(size.wrapping_mul(::std::mem::size_of::<Node>()
                                                                                         as
                                                                                         libc::c_ulong)).wrapping_sub((::std::mem::size_of::<Hash>()
                                                                                                                           as
                                                                                                                           libc::c_ulong).wrapping_add(((*t).size
                                                                                                                                                            as
                                                                                                                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<Node>()
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_ulong))));
    (*t).size = size as libc::c_int;
    (*t).firstfree =
        &mut *(*t).node.offset(size.wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong) as isize)
            as *mut Node;
    /* first free position to be used */
}
#[no_mangle]
pub unsafe extern "C" fn luaH_new(mut L: *mut lua_State,
                                  mut size: libc::c_int) -> *mut Hash {
    let mut t: *mut Hash =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     ::std::mem::size_of::<Hash>() as libc::c_ulong) as
            *mut Hash;
    (*t).htag = 4 as libc::c_int;
    (*t).next = (*L).roottable;
    (*L).roottable = t;
    (*t).mark = t;
    (*t).size = 0 as libc::c_int;
    (*L).nblocks =
        (*L).nblocks.wrapping_add((::std::mem::size_of::<Hash>() as
                                       libc::c_ulong).wrapping_add((0 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<Node>()
                                                                                                        as
                                                                                                        libc::c_ulong)));
    (*t).node = 0 as *mut Node;
    setnodevector(L, t, luaO_power2(size as lint32));
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_free(mut L: *mut lua_State, mut t: *mut Hash) {
    (*L).nblocks =
        (*L).nblocks.wrapping_sub((::std::mem::size_of::<Hash>() as
                                       libc::c_ulong).wrapping_add(((*t).size
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<Node>()
                                                                                                        as
                                                                                                        libc::c_ulong)));
    luaM_realloc(L, (*t).node as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    luaM_realloc(L, t as *mut libc::c_void, 0 as libc::c_int as lint32);
}
unsafe extern "C" fn numuse(mut t: *const Hash) -> libc::c_int {
    let mut v: *mut Node = (*t).node;
    let mut size: libc::c_int = (*t).size;
    let mut realuse: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        if (*v.offset(i as isize)).val.ttype != 1 as libc::c_int {
            realuse += 1
        }
        i += 1
    }
    return realuse;
}
unsafe extern "C" fn rehash(mut L: *mut lua_State, mut t: *mut Hash) {
    let mut oldsize: libc::c_int = (*t).size;
    let mut nold: *mut Node = (*t).node;
    let mut nelems: libc::c_int = numuse(t);
    let mut i: libc::c_int = 0;
    if nelems >= oldsize - oldsize / 4 as libc::c_int {
        /* using more than 3/4? */
        setnodevector(L, t,
                      (oldsize as
                           lint32).wrapping_mul(2 as libc::c_int as
                                                    libc::c_ulong));
    } else if nelems <= oldsize / 4 as libc::c_int &&
                  oldsize > 4 as libc::c_int {
        setnodevector(L, t, (oldsize / 2 as libc::c_int) as lint32);
    } else { setnodevector(L, t, oldsize as lint32); }
    i = 0 as libc::c_int;
    while i < oldsize {
        let mut old: *mut Node = nold.offset(i as isize);
        if (*old).val.ttype != 1 as libc::c_int {
            *luaH_set(L, t, &mut (*old).key) = (*old).val
        }
        i += 1
    }
    luaM_realloc(L, nold as *mut libc::c_void, 0 as libc::c_int as lint32);
    /* free old array */
}
/*
** inserts a key into a hash table; first, check whether key is
** already present; if not, check whether key's main position is free;
** if not, check whether colliding node is in its main position or not;
** if it is not, move colliding node to an empty place and put new key
** in its main position; otherwise (colliding node is in its main position),
** new key goes to an empty position.
*/
#[no_mangle]
pub unsafe extern "C" fn luaH_set(mut L: *mut lua_State, mut t: *mut Hash,
                                  mut key: *const TObject) -> *mut TObject {
    let mut mp: *mut Node = luaH_mainposition(t, key);
    let mut n: *mut Node = mp;
    if mp.is_null() {
        lua_error(L,
                  b"table index is nil\x00" as *const u8 as
                      *const libc::c_char);
    }
    loop  {
        /* check whether `key' is somewhere in the chain */
        if luaO_equalObj(key, &mut (*n).key) != 0 {
            return &mut (*n).val
        } else { n = (*n).next } /* that's all */
        if n.is_null() { break ; }
    }
    /* `key' not found; must insert it */
    if (*mp).key.ttype != 1 as libc::c_int {
        /* main position is not free? */
        let mut othern: *mut Node =
            0 as *mut Node; /* main position of colliding node */
        n = (*t).firstfree; /* get a free place */
        /* is colliding node out of its main position? (can only happens if
       its position is after "firstfree") */
        if mp > n &&
               {
                   othern = luaH_mainposition(t, &mut (*mp).key);
                   (othern) != mp
               } {
            /* yes; move colliding node into free position */
            while (*othern).next != mp {
                othern = (*othern).next
            } /* find previous */
            /* now `mp' is free */
            (*othern).next = n; /* redo the chain with `n' in place of `mp' */
            *n =
                *mp; /* copy colliding node into free pos. (mp->next also goes) */
            (*mp).next = 0 as *mut Node
        } else {
            /* colliding node is in its own main position */
            /* new node will go into free position */
            (*n).next = (*mp).next; /* chain new position */
            (*mp).next = n;
            mp = n
        }
    }
    (*mp).key = *key;
    loop 
         /* correct `firstfree' */
         {
        if (*(*t).firstfree).key.ttype == 1 as libc::c_int {
            return &mut (*mp).val
        } else {
            if (*t).firstfree == (*t).node {
                break ; /* OK; table still has a free place */
            } /* no more free places */
            (*t).firstfree = (*t).firstfree.offset(-1)
        }
    }
    rehash(L, t);
    return luaH_set(L, t, key);
    /* `rehash' invalidates this insertion */
}
#[no_mangle]
pub unsafe extern "C" fn luaH_setint(mut L: *mut lua_State, mut t: *mut Hash,
                                     mut key: libc::c_int) -> *mut TObject {
    let mut index: TObject =
        TObject{ttype: 0,
                value: Value{ts: 0 as *const TString as *mut TString,},};
    index.ttype = 2 as libc::c_int;
    index.value.n = key as Number;
    return luaH_set(L, t, &mut index);
}
#[no_mangle]
pub unsafe extern "C" fn luaH_setstrnum(mut L: *mut lua_State,
                                        mut t: *mut Hash,
                                        mut key: *mut TString,
                                        mut val: Number) {
    let mut value: *mut TObject = 0 as *mut TObject;
    let mut index: TObject =
        TObject{ttype: 0,
                value: Value{ts: 0 as *const TString as *mut TString,},};
    index.ttype = 3 as libc::c_int;
    index.value.ts = key;
    value = luaH_set(L, t, &mut index);
    (*value).ttype = 2 as libc::c_int;
    (*value).value.n = val;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getglobal(mut L: *mut lua_State,
                                        mut name: *const libc::c_char)
 -> *const TObject {
    return luaH_getstr((*L).gt, luaS_new(L, name));
}
