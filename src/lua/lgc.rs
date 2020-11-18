use ::libc;
extern "C" {
    pub type lua_longjmp;
    #[no_mangle]
    static luaO_nilobject: TObject;
    #[no_mangle]
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    #[no_mangle]
    fn luaD_checkstack(L: *mut lua_State, n: libc::c_int);
    #[no_mangle]
    fn luaF_freeproto(L: *mut lua_State, f: *mut Proto);
    #[no_mangle]
    fn luaF_freeclosure(L: *mut lua_State, c: *mut Closure);
    #[no_mangle]
    fn luaM_realloc(L: *mut lua_State, oldblock: *mut libc::c_void,
                    size: lint32) -> *mut libc::c_void;
    #[no_mangle]
    fn luaS_resize(L: *mut lua_State, tb: *mut stringtable,
                   newsize: libc::c_int);
    #[no_mangle]
    fn luaH_free(L: *mut lua_State, t: *mut Hash);
    #[no_mangle]
    fn luaH_remove(t: *mut Hash, key: *mut TObject);
    #[no_mangle]
    fn luaT_tag(o: *const TObject) -> libc::c_int;
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
pub struct TM {
    pub method: [*mut Closure; 15],
    pub collected: *mut TString,
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
pub const TM_GC: C2RustUnnamed_3 = 13;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GCState {
    pub tmark: *mut Hash,
    pub cmark: *mut Closure,
}
pub const TM_N: C2RustUnnamed_3 = 15;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const TM_FUNCTION: C2RustUnnamed_3 = 14;
pub const TM_CONCAT: C2RustUnnamed_3 = 12;
pub const TM_LT: C2RustUnnamed_3 = 11;
pub const TM_UNM: C2RustUnnamed_3 = 10;
pub const TM_POW: C2RustUnnamed_3 = 9;
pub const TM_DIV: C2RustUnnamed_3 = 8;
pub const TM_MUL: C2RustUnnamed_3 = 7;
pub const TM_SUB: C2RustUnnamed_3 = 6;
pub const TM_ADD: C2RustUnnamed_3 = 5;
pub const TM_SETGLOBAL: C2RustUnnamed_3 = 4;
pub const TM_GETGLOBAL: C2RustUnnamed_3 = 3;
pub const TM_INDEX: C2RustUnnamed_3 = 2;
pub const TM_SETTABLE: C2RustUnnamed_3 = 1;
pub const TM_GETTABLE: C2RustUnnamed_3 = 0;
/* mark a string; marks larger than 1 cannot be changed */
unsafe extern "C" fn protomark(mut f: *mut Proto) {
    if (*f).marked == 0 {
        let mut i: libc::c_int = 0;
        (*f).marked = 1 as libc::c_int as libc::c_short;
        if (*(*f).source).marked == 0 as libc::c_int {
            (*(*f).source).marked = 1 as libc::c_int
        }
        i = 0 as libc::c_int;
        while i < (*f).nkstr {
            if (**(*f).kstr.offset(i as isize)).marked == 0 as libc::c_int {
                (**(*f).kstr.offset(i as isize)).marked = 1 as libc::c_int
            }
            i += 1
        }
        i = 0 as libc::c_int;
        while i < (*f).nkproto {
            protomark(*(*f).kproto.offset(i as isize));
            i += 1
        }
        i = 0 as libc::c_int;
        while i < (*f).nlocvars {
            /* mark local-variable names */
            if (*(*(*f).locvars.offset(i as isize)).varname).marked ==
                   0 as libc::c_int {
                (*(*(*f).locvars.offset(i as isize)).varname).marked =
                    1 as libc::c_int
            } /* chain it for later traversal */
            i += 1
        }
    };
}
unsafe extern "C" fn markstack(mut L: *mut lua_State, mut st: *mut GCState) {
    let mut o: StkId = 0 as *mut TObject;
    o = (*L).stack;
    while o < (*L).top { markobject(st, o); o = o.offset(1) };
}
unsafe extern "C" fn marklock(mut L: *mut lua_State, mut st: *mut GCState) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*L).refSize {
        if (*(*L).refArray.offset(i as isize)).st == -(4 as libc::c_int) {
            markobject(st, &mut (*(*L).refArray.offset(i as isize)).o);
        }
        i += 1
    };
}
unsafe extern "C" fn markclosure(mut st: *mut GCState, mut cl: *mut Closure) {
    if !((*cl).mark != cl) {
        if (*cl).isC == 0 { protomark((*cl).f.l); }
        (*cl).mark = (*st).cmark;
        (*st).cmark = cl
    };
}
unsafe extern "C" fn marktagmethods(mut L: *mut lua_State,
                                    mut st: *mut GCState) {
    let mut e: libc::c_int = 0;
    e = 0 as libc::c_int;
    while e < TM_N as libc::c_int {
        let mut t: libc::c_int = 0;
        t = 0 as libc::c_int;
        while t <= (*L).last_tag {
            let mut cl: *mut Closure =
                (*(*L).TMtable.offset(t as isize)).method[e as usize];
            if !cl.is_null() { markclosure(st, cl); }
            t += 1
        }
        e += 1
    };
}
unsafe extern "C" fn markobject(mut st: *mut GCState, mut o: *mut TObject) {
    match (*o).ttype {
        0 | 3 => {
            if (*(*o).value.ts).marked == 0 as libc::c_int {
                (*(*o).value.ts).marked = 1 as libc::c_int
            }
            /* numbers, etc */
        }
        6 => {
            markclosure(st,
                        (*(*o).value.i).func); /* chain it in list of marked */
        }
        5 => {
            markclosure(st,
                        (*o).value.cl); /* put table of globals in mark list */
        }
        4 => {
            if !((*(*o).value.a).mark != (*o).value.a) {
                (*(*o).value.a).mark = (*st).tmark; /* mark tag methods */
                (*st).tmark = (*o).value.a
            }
        }
        _ => { }
    }; /* mark stack objects */
}
unsafe extern "C" fn markall(mut L: *mut lua_State) {
    let mut st: GCState =
        GCState{tmark: 0 as *mut Hash,
                cmark: 0 as *mut Closure,}; /* mark locked objects */
    st.cmark = 0 as *mut Closure;
    st.tmark = (*L).gt;
    (*(*L).gt).mark = 0 as *mut Hash;
    marktagmethods(L, &mut st);
    markstack(L, &mut st);
    marklock(L, &mut st);
    loop 
         /* mark tables and closures */
         {
        if !st.cmark.is_null() {
            let mut i: libc::c_int = 0; /* get first closure from list */
            let mut f: *mut Closure = st.cmark; /* remove it from list */
            st.cmark = (*f).mark;
            i = 0 as libc::c_int;
            while i < (*f).nupvalues as libc::c_int {
                /* mark its upvalues */
                markobject(&mut st,
                           &mut *(*f).upvalue.as_mut_ptr().offset(i as
                                                                      isize)); /* get first table from list */
                i += 1
            }
        } else {
            if st.tmark.is_null() {
                break ; /* remove it from list */
            } /* dead element; try to remove it */
            let mut i_0: libc::c_int = 0;
            let mut h: *mut Hash = st.tmark;
            st.tmark = (*h).mark;
            i_0 = 0 as libc::c_int;
            while i_0 < (*h).size {
                let mut n: *mut Node =
                    &mut *(*h).node.offset(i_0 as isize) as *mut Node;
                if (*n).key.ttype != 1 as libc::c_int {
                    if (*n).val.ttype == 1 as libc::c_int {
                        luaH_remove(h, &mut (*n).key);
                    }
                    markobject(&mut st, &mut (*n).key);
                    markobject(&mut st, &mut (*n).val);
                }
                i_0 += 1
            }
        }
    };
}
unsafe extern "C" fn hasmark(mut o: *const TObject) -> libc::c_int {
    /* valid only for locked objects */
    match (*o).ttype {
        3 | 0 => { return (*(*o).value.ts).marked }
        4 => { return ((*(*o).value.a).mark != (*o).value.a) as libc::c_int }
        5 => {
            return ((*(*o).value.cl).mark != (*o).value.cl) as libc::c_int
        }
        _ => {
            /* number */
            return 1 as libc::c_int
        }
    };
}
/* macro for internal debugging; check if a link of free refs is valid */
unsafe extern "C" fn invalidaterefs(mut L: *mut lua_State) {
    let mut n: libc::c_int = (*L).refSize; /* unmark */
    let mut i: libc::c_int = 0; /* unmark */
    i = 0 as libc::c_int;
    while i < n {
        let mut r: *mut Ref =
            &mut *(*L).refArray.offset(i as isize) as *mut Ref;
        if (*r).st == -(2 as libc::c_int) && hasmark(&mut (*r).o) == 0 {
            (*r).st = -(3 as libc::c_int)
        }
        i += 1
    };
}
unsafe extern "C" fn collectproto(mut L: *mut lua_State) {
    let mut p: *mut *mut Proto = &mut (*L).rootproto;
    let mut next: *mut Proto = 0 as *mut Proto;
    loop  {
        next = *p;
        if next.is_null() { break ; }
        if (*next).marked != 0 {
            (*next).marked = 0 as libc::c_int as libc::c_short;
            p = &mut (*next).next
        } else { *p = (*next).next; luaF_freeproto(L, next); }
    };
}
unsafe extern "C" fn collectclosure(mut L: *mut lua_State) {
    let mut p: *mut *mut Closure = &mut (*L).rootcl;
    let mut next: *mut Closure = 0 as *mut Closure;
    loop  {
        next = *p;
        if next.is_null() { break ; }
        if (*next).mark != next {
            (*next).mark = next;
            p = &mut (*next).next
        } else { *p = (*next).next; luaF_freeclosure(L, next); }
    };
}
unsafe extern "C" fn collecttable(mut L: *mut lua_State) {
    let mut p: *mut *mut Hash = &mut (*L).roottable;
    let mut next: *mut Hash = 0 as *mut Hash;
    loop  {
        next = *p;
        if next.is_null() { break ; }
        if (*next).mark != next {
            (*next).mark = next;
            p = &mut (*next).next
        } else { *p = (*next).next; luaH_free(L, next); }
    };
}
unsafe extern "C" fn checktab(mut L: *mut lua_State,
                              mut tb: *mut stringtable) {
    if (*tb).nuse < ((*tb).size / 4 as libc::c_int) as lint32 &&
           (*tb).size > 10 as libc::c_int {
        luaS_resize(L, tb, (*tb).size / 2 as libc::c_int);
    };
    /* table is too big */
}
unsafe extern "C" fn collectstrings(mut L: *mut lua_State,
                                    mut all: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*L).strt.size {
        /* for each list */
        let mut p: *mut *mut TString =
            &mut *(*L).strt.hash.offset(i as isize) as *mut *mut TString;
        let mut next: *mut TString = 0 as *mut TString;
        loop  {
            next = *p;
            if next.is_null() { break ; }
            if (*next).marked != 0 && all == 0 {
                /* preserve? */
                if (*next).marked < 2 as libc::c_int {
                    /* does not change FIXMARKs */
                    (*next).marked = 0 as libc::c_int
                }
                p = &mut (*next).nexthash
            } else {
                /* collect */
                *p = (*next).nexthash;
                (*L).strt.nuse = (*L).strt.nuse.wrapping_sub(1);
                (*L).nblocks =
                    (*L).nblocks.wrapping_sub((::std::mem::size_of::<TString>()
                                                   as libc::c_ulong as
                                                   libc::c_long +
                                                   ((*next).len.wrapping_add(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                                        as libc::c_long -
                                                        ::std::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong
                                                            as libc::c_int as
                                                            libc::c_long) *
                                                       ::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong as
                                                           libc::c_long) as
                                                  libc::c_ulong);
                luaM_realloc(L, next as *mut libc::c_void,
                             0 as libc::c_int as lint32);
            }
        }
        i += 1
    }
    checktab(L, &mut (*L).strt);
}
unsafe extern "C" fn collectudata(mut L: *mut lua_State,
                                  mut all: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*L).udt.size {
        /* for each list */
        let mut p: *mut *mut TString =
            &mut *(*L).udt.hash.offset(i as isize) as *mut *mut TString;
        let mut next: *mut TString = 0 as *mut TString;
        loop  {
            next = *p;
            if next.is_null() { break ; }
            if (*next).marked != 0 && all == 0 {
                /* preserve? */
                (*next).marked = 0 as libc::c_int;
                p = &mut (*next).nexthash
            } else {
                /* collect */
                let mut tag: libc::c_int = (*next).u.d.tag; /* chain udata */
                *p = (*next).nexthash;
                (*next).nexthash =
                    (*(*L).TMtable.offset(tag as isize)).collected;
                let ref mut fresh0 =
                    (*(*L).TMtable.offset(tag as isize)).collected;
                *fresh0 = next;
                (*L).nblocks =
                    (*L).nblocks.wrapping_sub((::std::mem::size_of::<TString>()
                                                   as libc::c_ulong as
                                                   libc::c_long +
                                                   ((*next).len.wrapping_add(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                                        as libc::c_long -
                                                        ::std::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong
                                                            as libc::c_int as
                                                            libc::c_long) *
                                                       ::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong as
                                                           libc::c_long) as
                                                  libc::c_ulong);
                (*L).udt.nuse = (*L).udt.nuse.wrapping_sub(1)
            }
        }
        i += 1
    }
    checktab(L, &mut (*L).udt);
}
unsafe extern "C" fn checkMbuffer(mut L: *mut lua_State) {
    if (*L).Mbuffsize >
           (256 as libc::c_int * 2 as libc::c_int) as libc::c_ulong {
        /* is buffer too big? */
        let mut newsize: size_t =
            (*L).Mbuffsize.wrapping_div(2 as libc::c_int as
                                            libc::c_ulong); /* still larger than MINBUFFER */
        (*L).nblocks =
            (*L).nblocks.wrapping_add(newsize.wrapping_sub((*L).Mbuffsize).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                            as
                                                                                            libc::c_ulong));
        (*L).Mbuffsize = newsize;
        (*L).Mbuffer =
            luaM_realloc(L, (*L).Mbuffer as *mut libc::c_void,
                         newsize.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                  as libc::c_ulong)) as
                *mut libc::c_char
    };
}
unsafe extern "C" fn callgcTM(mut L: *mut lua_State, mut o: *const TObject) {
    let mut tm: *mut Closure =
        (*(*L).TMtable.offset(luaT_tag(o) as
                                  isize)).method[TM_GC as libc::c_int as
                                                     usize];
    if !tm.is_null() {
        let mut oldah: libc::c_int = (*L).allowhooks;
        /* restore hooks */
        (*L).allowhooks =
            0 as libc::c_int; /* stop debug hooks during GC tag methods */
        luaD_checkstack(L,
                        2 as libc::c_int); /* avoid GC during tag methods */
        (*(*L).top).value.cl = tm;
        (*(*L).top).ttype = 5 as libc::c_int;
        *(*L).top.offset(1 as libc::c_int as isize) = *o;
        (*L).top = (*L).top.offset(2 as libc::c_int as isize);
        luaD_call(L, (*L).top.offset(-(2 as libc::c_int as isize)),
                  0 as libc::c_int);
        (*L).allowhooks = oldah
    };
}
unsafe extern "C" fn callgcTMudata(mut L: *mut lua_State) {
    let mut tag: libc::c_int = 0;
    let mut o: TObject =
        TObject{ttype: 0,
                value: Value{ts: 0 as *const TString as *mut TString,},};
    o.ttype = 0 as libc::c_int;
    (*L).GCthreshold =
        (2 as libc::c_int as libc::c_ulong).wrapping_mul((*L).nblocks);
    tag = (*L).last_tag;
    while tag >= 0 as libc::c_int {
        /* for each tag (in reverse order) */
        let mut udata: *mut TString =
            0 as *mut TString; /* remove it from list */
        loop  {
            udata =
                (*(*L).TMtable.offset(tag as
                                          isize)).collected; /* check unlocked references */
            if udata.is_null() {
                break ; /* set new threshold */
            }
            let ref mut fresh1 =
                (*(*L).TMtable.offset(tag as isize)).collected;
            *fresh1 = (*udata).nexthash;
            o.value.ts = udata;
            callgcTM(L, &mut o);
            luaM_realloc(L, udata as *mut libc::c_void,
                         0 as libc::c_int as lint32);
        }
        tag -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_collect(mut L: *mut lua_State,
                                      mut all: libc::c_int) {
    collectudata(L, all);
    callgcTMudata(L);
    collectstrings(L, all);
    collecttable(L);
    collectproto(L);
    collectclosure(L);
}
unsafe extern "C" fn luaC_collectgarbage(mut L: *mut lua_State) {
    markall(L);
    invalidaterefs(L);
    luaC_collect(L, 0 as libc::c_int);
    checkMbuffer(L);
    (*L).GCthreshold =
        (2 as libc::c_int as libc::c_ulong).wrapping_mul((*L).nblocks);
    callgcTM(L, &luaO_nilobject);
}
#[no_mangle]
pub unsafe extern "C" fn luaC_checkGC(mut L: *mut lua_State) {
    if (*L).nblocks >= (*L).GCthreshold { luaC_collectgarbage(L); };
}
