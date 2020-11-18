use ::libc;
extern "C" {
    pub type lua_longjmp;
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn luaA_pushobject(L: *mut lua_State, o: *const TObject);
    #[no_mangle]
    static luaO_typenames: [*const libc::c_char; 0];
    #[no_mangle]
    fn luaO_equalObj(t1: *const TObject, t2: *const TObject) -> libc::c_int;
    #[no_mangle]
    fn luaO_verror(L: *mut lua_State, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn luaO_chunkid(out: *mut libc::c_char, source: *const libc::c_char,
                    len: libc::c_int);
    #[no_mangle]
    static luaK_opproperties: [OpProperties; 49];
    #[no_mangle]
    fn luaD_checkstack(L: *mut lua_State, n: libc::c_int);
    #[no_mangle]
    fn luaF_getlocalname(func: *const Proto, local_number: libc::c_int,
                         pc: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    static luaT_eventname: [*const libc::c_char; 0];
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
pub type OpCode = libc::c_uint;
pub const OP_CLOSURE: OpCode = 48;
pub const OP_LFORLOOP: OpCode = 47;
pub const OP_LFORPREP: OpCode = 46;
pub const OP_FORLOOP: OpCode = 45;
pub const OP_FORPREP: OpCode = 44;
pub const OP_PUSHNILJMP: OpCode = 43;
pub const OP_JMP: OpCode = 42;
pub const OP_JMPONF: OpCode = 41;
pub const OP_JMPONT: OpCode = 40;
pub const OP_JMPF: OpCode = 39;
pub const OP_JMPT: OpCode = 38;
pub const OP_JMPGE: OpCode = 37;
pub const OP_JMPGT: OpCode = 36;
pub const OP_JMPLE: OpCode = 35;
pub const OP_JMPLT: OpCode = 34;
pub const OP_JMPEQ: OpCode = 33;
pub const OP_JMPNE: OpCode = 32;
pub const OP_NOT: OpCode = 31;
pub const OP_MINUS: OpCode = 30;
pub const OP_CONCAT: OpCode = 29;
pub const OP_POW: OpCode = 28;
pub const OP_DIV: OpCode = 27;
pub const OP_MULT: OpCode = 26;
pub const OP_SUB: OpCode = 25;
pub const OP_ADDI: OpCode = 24;
pub const OP_ADD: OpCode = 23;
pub const OP_SETMAP: OpCode = 22;
pub const OP_SETLIST: OpCode = 21;
pub const OP_SETTABLE: OpCode = 20;
pub const OP_SETGLOBAL: OpCode = 19;
pub const OP_SETLOCAL: OpCode = 18;
pub const OP_CREATETABLE: OpCode = 17;
pub const OP_PUSHSELF: OpCode = 16;
pub const OP_GETINDEXED: OpCode = 15;
pub const OP_GETDOTTED: OpCode = 14;
pub const OP_GETTABLE: OpCode = 13;
pub const OP_GETGLOBAL: OpCode = 12;
pub const OP_GETLOCAL: OpCode = 11;
pub const OP_PUSHUPVALUE: OpCode = 10;
pub const OP_PUSHNEGNUM: OpCode = 9;
pub const OP_PUSHNUM: OpCode = 8;
pub const OP_PUSHSTRING: OpCode = 7;
pub const OP_PUSHINT: OpCode = 6;
pub const OP_POP: OpCode = 5;
pub const OP_PUSHNIL: OpCode = 4;
pub const OP_TAILCALL: OpCode = 3;
pub const OP_CALL: OpCode = 2;
pub const OP_RETURN: OpCode = 1;
pub const OP_END: OpCode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpProperties {
    pub mode: libc::c_char,
    pub push: libc::c_uchar,
    pub pop: libc::c_uchar,
}
pub const TM_N: C2RustUnnamed_3 = 15;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const TM_FUNCTION: C2RustUnnamed_3 = 14;
pub const TM_GC: C2RustUnnamed_3 = 13;
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
unsafe extern "C" fn setnormalized(mut d: *mut TObject,
                                   mut s: *const TObject) {
    if (*s).ttype == 6 as libc::c_int {
        (*d).value.cl = (*(*s).value.i).func; /* there is no such level */
        (*d).ttype = 5 as libc::c_int
    } else { *d = *s }; /* no line info or function is not active */
}
unsafe extern "C" fn isLmark(mut o: StkId) -> libc::c_int {
    return (!o.is_null() && (*o).ttype == 6 as libc::c_int &&
                (*(*(*o).value.i).func).isC == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setcallhook(mut L: *mut lua_State,
                                         mut func: lua_Hook) -> lua_Hook {
    let mut oldhook: lua_Hook = (*L).callhook;
    (*L).callhook = func;
    return oldhook;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setlinehook(mut L: *mut lua_State,
                                         mut func: lua_Hook) -> lua_Hook {
    let mut oldhook: lua_Hook = (*L).linehook;
    (*L).linehook = func;
    return oldhook;
}
unsafe extern "C" fn aux_stackedfunction(mut L: *mut lua_State,
                                         mut level: libc::c_int,
                                         mut top: StkId) -> StkId {
    let mut i: libc::c_int = 0;
    i =
        top.offset(-(1 as libc::c_int as
                         isize)).wrapping_offset_from((*L).stack) as
            libc::c_long as libc::c_int;
    while i >= 0 as libc::c_int {
        if (*(*L).stack.offset(i as isize)).ttype == 6 as libc::c_int {
            if level == 0 as libc::c_int {
                return (*L).stack.offset(i as isize)
            }
            level -= 1
        }
        i -= 1
    }
    return 0 as StkId;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getstack(mut L: *mut lua_State,
                                      mut level: libc::c_int,
                                      mut ar: *mut lua_Debug) -> libc::c_int {
    let mut f: StkId = aux_stackedfunction(L, level, (*L).top);
    if f.is_null() {
        return 0 as libc::c_int
    } else { (*ar)._func = f; return 1 as libc::c_int };
}
unsafe extern "C" fn nups(mut f: StkId) -> libc::c_int {
    match (*f).ttype {
        5 => { return (*(*f).value.cl).nupvalues as libc::c_int }
        6 => { return (*(*(*f).value.i).func).nupvalues as libc::c_int }
        _ => { return 0 as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaG_getline(mut lineinfo: *mut libc::c_int,
                                      mut pc: libc::c_int,
                                      mut refline: libc::c_int,
                                      mut prefi: *mut libc::c_int)
 -> libc::c_int {
    let mut refi: libc::c_int = 0;
    if lineinfo.is_null() || pc == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    refi = if !prefi.is_null() { *prefi } else { 0 as libc::c_int };
    if *lineinfo.offset(refi as isize) < 0 as libc::c_int {
        let fresh0 = refi;
        refi = refi + 1;
        refline += -*lineinfo.offset(fresh0 as isize)
    }
    while *lineinfo.offset(refi as isize) > pc {
        refline -= 1;
        refi -= 1;
        if *lineinfo.offset(refi as isize) < 0 as libc::c_int {
            let fresh1 = refi;
            refi = refi - 1;
            refline -= -*lineinfo.offset(fresh1 as isize)
        }
    }
    loop  {
        let mut nextline: libc::c_int = refline + 1 as libc::c_int;
        let mut nextref: libc::c_int = refi + 1 as libc::c_int;
        if *lineinfo.offset(nextref as isize) < 0 as libc::c_int {
            let fresh2 = nextref;
            nextref = nextref + 1;
            nextline += -*lineinfo.offset(fresh2 as isize)
        }
        if *lineinfo.offset(nextref as isize) > pc { break ; }
        refline = nextline;
        refi = nextref
    }
    if !prefi.is_null() { *prefi = refi }
    return refline;
}
unsafe extern "C" fn currentpc(mut f: StkId) -> libc::c_int {
    let mut ci: *mut CallInfo = (*f).value.i;
    if !(*ci).pc.is_null() {
        return ((*(*ci).pc).wrapping_offset_from((*(*(*ci).func).f.l).code) as
                    libc::c_long - 1 as libc::c_int as libc::c_long) as
                   libc::c_int
    } else { return -(1 as libc::c_int) };
    /* function is not active */
}
unsafe extern "C" fn currentline(mut f: StkId) -> libc::c_int {
    if isLmark(f) == 0 {
        return -(1 as libc::c_int)
    } else {
        let mut ci: *mut CallInfo =
            (*f).value.i; /* only active lua functions have current-line information */
        let mut lineinfo: *mut libc::c_int =
            (*(*(*ci).func).f.l).lineinfo; /* `f' is not a Lua function? */
        return luaG_getline(lineinfo, currentpc(f), 1 as libc::c_int,
                            0 as *mut libc::c_int)
    }; /* push value */
}
unsafe extern "C" fn getluaproto(mut f: StkId) -> *mut Proto {
    return if isLmark(f) != 0 {
               (*(*(*f).value.i).func).f.l
           } else { 0 as *mut Proto }; /* pop new value */
}
#[no_mangle]
pub unsafe extern "C" fn lua_getlocal(mut L: *mut lua_State,
                                      mut ar: *const lua_Debug,
                                      mut n: libc::c_int)
 -> *const libc::c_char {
    let mut name: *const libc::c_char =
        0 as *const libc::c_char; /* `f' is not a Lua function? */
    let mut f: StkId = (*ar)._func; /* `(' starts private locals */
    let mut fp: *mut Proto = getluaproto(f);
    if fp.is_null() { return 0 as *const libc::c_char }
    name = luaF_getlocalname(fp, n, currentpc(f));
    if name.is_null() { return 0 as *const libc::c_char }
    luaA_pushobject(L,
                    f.offset(1 as libc::c_int as
                                 isize).offset((n - 1 as libc::c_int) as
                                                   isize) as *const TObject);
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setlocal(mut L: *mut lua_State,
                                      mut ar: *const lua_Debug,
                                      mut n: libc::c_int)
 -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut f: StkId = (*ar)._func;
    let mut fp: *mut Proto = getluaproto(f);
    (*L).top = (*L).top.offset(-1);
    if fp.is_null() { return 0 as *const libc::c_char }
    name = luaF_getlocalname(fp, n, currentpc(f));
    if name.is_null() ||
           *name.offset(0 as libc::c_int as isize) as libc::c_int ==
               '(' as i32 {
        return 0 as *const libc::c_char
    }
    *f.offset(1 as libc::c_int as
                  isize).offset((n - 1 as libc::c_int) as isize) = *(*L).top;
    return name;
}
unsafe extern "C" fn infoLproto(mut ar: *mut lua_Debug, mut f: *mut Proto) {
    (*ar).source = (*(*f).source).str_0.as_mut_ptr();
    (*ar).linedefined = (*f).lineDefined;
    (*ar).what = b"Lua\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn funcinfo(mut L: *mut lua_State, mut ar: *mut lua_Debug,
                              mut func: StkId) {
    let mut cl: *mut Closure = 0 as *mut Closure;
    match (*func).ttype {
        5 => { cl = (*func).value.cl }
        6 => { cl = (*(*func).value.i).func }
        _ => {
            lua_error(L,
                      b"value for `lua_getinfo\' is not a function\x00" as
                          *const u8 as *const libc::c_char);
        }
    }
    if (*cl).isC != 0 {
        (*ar).source = b"=C\x00" as *const u8 as *const libc::c_char;
        (*ar).linedefined = -(1 as libc::c_int);
        (*ar).what = b"C\x00" as *const u8 as *const libc::c_char
    } else { infoLproto(ar, (*cl).f.l); }
    luaO_chunkid((*ar).short_src.as_mut_ptr(), (*ar).source,
                 ::std::mem::size_of::<[libc::c_char; 60]>() as libc::c_ulong
                     as libc::c_int);
    if (*ar).linedefined == 0 as libc::c_int {
        (*ar).what = b"main\x00" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn travtagmethods(mut L: *mut lua_State,
                                    mut o: *const TObject)
 -> *const libc::c_char {
    if (*o).ttype == 5 as libc::c_int {
        let mut e: libc::c_int = 0;
        e = 0 as libc::c_int;
        while e < TM_N as libc::c_int {
            let mut t: libc::c_int = 0;
            t = 0 as libc::c_int;
            while t <= (*L).last_tag {
                if (*o).value.cl ==
                       (*(*L).TMtable.offset(t as isize)).method[e as usize] {
                    return *luaT_eventname.as_ptr().offset(e as isize)
                }
                t += 1
            }
            e += 1
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn travglobals(mut L: *mut lua_State, mut o: *const TObject)
 -> *const libc::c_char {
    let mut g: *mut Hash = (*L).gt;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*g).size {
        if luaO_equalObj(o, &mut (*(*g).node.offset(i as isize)).val) != 0 &&
               (*(*g).node.offset(i as isize)).key.ttype == 3 as libc::c_int {
            return (*(*(*g).node.offset(i as
                                            isize)).key.value.ts).str_0.as_mut_ptr()
        }
        i += 1
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn getname(mut L: *mut lua_State, mut f: StkId,
                             mut ar: *mut lua_Debug) {
    let mut o: TObject =
        TObject{ttype: 0, value: Value{ts: 0 as *mut TString,},};
    setnormalized(&mut o, f as *const TObject);
    /* try to find a name for given function */
    (*ar).name = travglobals(L, &mut o);
    if !(*ar).name.is_null() {
        (*ar).namewhat = b"global\x00" as *const u8 as *const libc::c_char
    } else {
        /* not found: try tag methods */
        (*ar).name = travtagmethods(L, &mut o);
        if !(*ar).name.is_null() {
            (*ar).namewhat =
                b"tag-method\x00" as *const u8 as *const libc::c_char
        } else {
            (*ar).namewhat = b"\x00" as *const u8 as *const libc::c_char
        }
    };
    /* not found at all */
}
#[no_mangle]
pub unsafe extern "C" fn lua_getinfo(mut L: *mut lua_State,
                                     mut what: *const libc::c_char,
                                     mut ar: *mut lua_Debug) -> libc::c_int {
    let mut func: StkId = 0 as *mut TObject; /* skip the '>' */
    let mut isactive: libc::c_int =
        (*what as libc::c_int != '>' as i32) as libc::c_int;
    if isactive != 0 {
        func = (*ar)._func
    } else {
        what = what.offset(1);
        func = (*L).top.offset(-(1 as libc::c_int as isize))
    }
    while *what != 0 {
        match *what as libc::c_int {
            83 => {
                funcinfo(L, ar, func);
                /* invalid option */
            }
            108 => { (*ar).currentline = currentline(func) }
            117 => { (*ar).nups = nups(func) }
            110 => {
                (*ar).namewhat =
                    if isactive != 0 {
                        getfuncname(L, func, &mut (*ar).name)
                    } else { 0 as *const libc::c_char }; /* pop function */
                if (*ar).namewhat.is_null() { getname(L, func, ar); }
            }
            102 => {
                setnormalized((*L).top, func as *const TObject);
                if (*L).top == (*L).stack_last {
                    luaD_checkstack(L, 1 as libc::c_int);
                }
                (*L).top = (*L).top.offset(1)
            }
            _ => { return 0 as libc::c_int }
        }
        what = what.offset(1)
    }
    if isactive == 0 { (*L).top = (*L).top.offset(-1) }
    return 1 as libc::c_int;
}
/*
** {======================================================
** Symbolic Execution
** =======================================================
*/
unsafe extern "C" fn pushpc(mut stack: *mut libc::c_int, mut pc: libc::c_int,
                            mut top: libc::c_int, mut n: libc::c_int)
 -> libc::c_int {
    loop  {
        let fresh3 =
            n; /* stores last instruction that changed a stack entry */
        n = n - 1; /* `arg' */
        if !(fresh3 != 0) { break ; }
        let fresh4 = top;
        top = top + 1;
        *stack.offset(fresh4 as isize) = pc - 1 as libc::c_int
    }
    return top;
}
unsafe extern "C" fn luaG_symbexec(mut pt: *const Proto,
                                   mut lastpc: libc::c_int,
                                   mut stackpos: libc::c_int) -> Instruction {
    let mut stack: [libc::c_int; 250] = [0; 250];
    let mut code: *const Instruction = (*pt).code;
    let mut top: libc::c_int = (*pt).numparams as libc::c_int;
    let mut pc: libc::c_int = 0 as libc::c_int;
    if (*pt).is_vararg != 0 {
        /* varargs? */
        top += 1
    }
    while pc < lastpc {
        let fresh5 = pc;
        pc = pc + 1;
        let i: Instruction = *code.offset(fresh5 as isize);
        match (i &
                   (!((!(0 as libc::c_int as Instruction)) <<
                          6 as libc::c_int)) << 0 as libc::c_int) as OpCode as
                  libc::c_uint {
            1 => { top = (i >> 6 as libc::c_int) as libc::c_int }
            3 => {
                top =
                    (i >> 6 as libc::c_int &
                         (!((!(0 as libc::c_int as Instruction)) <<
                                9 as libc::c_int)) << 0 as libc::c_int) as
                        libc::c_int
            }
            2 => {
                let mut nresults: libc::c_int =
                    (i >> 6 as libc::c_int &
                         (!((!(0 as libc::c_int as Instruction)) <<
                                9 as libc::c_int)) << 0 as libc::c_int) as
                        libc::c_int;
                if nresults == 255 as libc::c_int {
                    nresults = 1 as libc::c_int
                }
                top =
                    pushpc(stack.as_mut_ptr(), pc,
                           (i >> 6 as libc::c_int + 9 as libc::c_int) as
                               libc::c_int, nresults)
            }
            4 => {
                top =
                    pushpc(stack.as_mut_ptr(), pc, top,
                           (i >> 6 as libc::c_int) as libc::c_int)
            }
            5 => { top -= (i >> 6 as libc::c_int) as libc::c_int }
            20 | 21 => {
                top -=
                    (i >> 6 as libc::c_int &
                         (!((!(0 as libc::c_int as Instruction)) <<
                                9 as libc::c_int)) << 0 as libc::c_int) as
                        libc::c_int
            }
            22 => {
                top -=
                    2 as libc::c_int * (i >> 6 as libc::c_int) as libc::c_int
            }
            29 => {
                top -= (i >> 6 as libc::c_int) as libc::c_int;
                let fresh6 = top;
                top = top + 1;
                stack[fresh6 as usize] = pc - 1 as libc::c_int
            }
            48 => {
                top -=
                    (i >> 6 as libc::c_int &
                         (!((!(0 as libc::c_int as Instruction)) <<
                                9 as libc::c_int)) << 0 as libc::c_int) as
                        libc::c_int;
                let fresh7 = top;
                top = top + 1;
                stack[fresh7 as usize] = pc - 1 as libc::c_int
            }
            40 | 41 => {
                let mut newpc: libc::c_int =
                    pc +
                        ((i >> 6 as libc::c_int) as libc::c_int -
                             (((1 as libc::c_int) <<
                                   32 as libc::c_int - 6 as libc::c_int) -
                                  1 as libc::c_int >> 1 as libc::c_int));
                /* jump is forward and do not skip `lastpc'? */
                if pc < newpc && newpc <= lastpc
                   { /* do not jump; pop value */
                    stack[(top - 1 as libc::c_int) as usize] =
                        pc - 1 as libc::c_int;
                    pc = newpc /* value comes from `and'/`or' */
                    /* do the jump */
                } else { top -= 1 }
            }
            _ => {
                let mut op: OpCode =
                    (i &
                         (!((!(0 as libc::c_int as Instruction)) <<
                                6 as libc::c_int)) << 0 as libc::c_int) as
                        OpCode; /* not an active Lua function */
                top -=
                    luaK_opproperties[op as usize].pop as
                        libc::c_int; /* func+1 == function base */
                top =
                    pushpc(stack.as_mut_ptr(), pc, top,
                           luaK_opproperties[op as usize].push as libc::c_int)
            }
        }
    }
    return *code.offset(stack[stackpos as usize] as isize);
}
unsafe extern "C" fn getobjname(mut L: *mut lua_State, mut obj: StkId,
                                mut name: *mut *const libc::c_char)
 -> *const libc::c_char {
    let mut func: StkId = aux_stackedfunction(L, 0 as libc::c_int, obj);
    if isLmark(func) == 0 {
        return 0 as *const libc::c_char
    } else {
        let mut p: *mut Proto = (*(*(*func).value.i).func).f.l;
        let mut pc: libc::c_int = currentpc(func);
        let mut stackpos: libc::c_int =
            obj.wrapping_offset_from(func.offset(1 as libc::c_int as isize))
                as libc::c_long as libc::c_int;
        let mut i: Instruction = luaG_symbexec(p, pc, stackpos);
        match (i &
                   (!((!(0 as libc::c_int as Instruction)) <<
                          6 as libc::c_int)) << 0 as libc::c_int) as OpCode as
                  libc::c_uint {
            12 => {
                *name =
                    (**(*p).kstr.offset((i >> 6 as libc::c_int) as libc::c_int
                                            as isize)).str_0.as_mut_ptr();
                return b"global\x00" as *const u8 as *const libc::c_char
                /* no useful name found */
            }
            11 => {
                *name =
                    luaF_getlocalname(p,
                                      (i >> 6 as libc::c_int) as libc::c_int +
                                          1 as libc::c_int, pc);
                return b"local\x00" as *const u8 as *const libc::c_char
            }
            16 | 14 => {
                *name =
                    (**(*p).kstr.offset((i >> 6 as libc::c_int) as libc::c_int
                                            as isize)).str_0.as_mut_ptr();
                return b"field\x00" as *const u8 as *const libc::c_char
            }
            _ => { return 0 as *const libc::c_char }
        }
    };
}
/*
** $Id: ldebug.c,v 1.2 2001/11/26 23:00:23 darkgod Exp $
** Debug Interface
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn getfuncname(mut L: *mut lua_State, mut f: StkId,
                                 mut name: *mut *const libc::c_char)
 -> *const libc::c_char {
    let mut func: StkId =
        aux_stackedfunction(L, 0 as libc::c_int, f); /* calling function */
    if isLmark(func) == 0 {
        return 0 as *const libc::c_char
    } else {
        let mut p: *mut Proto =
            (*(*(*func).value.i).func).f.l; /* not an active Lua function */
        let mut pc: libc::c_int =
            currentpc(func); /* function is not activated */
        let mut i: Instruction = 0;
        if pc == -(1 as libc::c_int) { return 0 as *const libc::c_char }
        i = *(*p).code.offset(pc as isize);
        match (i &
                   (!((!(0 as libc::c_int as Instruction)) <<
                          6 as libc::c_int)) << 0 as libc::c_int) as OpCode as
                  libc::c_uint {
            2 | 3 => {
                return getobjname(L,
                                  func.offset(1 as libc::c_int as
                                                  isize).offset((i >>
                                                                     6 as
                                                                         libc::c_int
                                                                         +
                                                                         9 as
                                                                             libc::c_int)
                                                                    as
                                                                    libc::c_int
                                                                    as isize),
                                  name)
                /* no useful name found */
            }
            _ => { return 0 as *const libc::c_char }
        }
    };
}
/* }====================================================== */
#[no_mangle]
pub unsafe extern "C" fn luaG_typeerror(mut L: *mut lua_State, mut o: StkId,
                                        mut op: *const libc::c_char) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut kind: *const libc::c_char = getobjname(L, o, &mut name);
    let mut t: *const libc::c_char =
        *luaO_typenames.as_ptr().offset((*o).ttype as isize);
    if !kind.is_null() {
        luaO_verror(L,
                    b"attempt to %.30s %.20s `%.40s\' (a %.10s value)\x00" as
                        *const u8 as *const libc::c_char, op, kind, name, t);
    } else {
        luaO_verror(L,
                    b"attempt to %.30s a %.10s value\x00" as *const u8 as
                        *const libc::c_char, op, t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaG_binerror(mut L: *mut lua_State, mut p1: StkId,
                                       mut t: libc::c_int,
                                       mut op: *const libc::c_char) {
    if (*p1).ttype == t { p1 = p1.offset(1) }
    luaG_typeerror(L, p1, op);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_ordererror(mut L: *mut lua_State,
                                         mut top: StkId) {
    let mut t1: *const libc::c_char =
        *luaO_typenames.as_ptr().offset((*top.offset(-(2 as libc::c_int as
                                                           isize))).ttype as
                                            isize);
    let mut t2: *const libc::c_char =
        *luaO_typenames.as_ptr().offset((*top.offset(-(1 as libc::c_int as
                                                           isize))).ttype as
                                            isize);
    if *t1.offset(2 as libc::c_int as isize) as libc::c_int ==
           *t2.offset(2 as libc::c_int as isize) as libc::c_int {
        luaO_verror(L,
                    b"attempt to compare two %.10s values\x00" as *const u8 as
                        *const libc::c_char, t1);
    } else {
        luaO_verror(L,
                    b"attempt to compare %.10s with %.10s\x00" as *const u8 as
                        *const libc::c_char, t1, t2);
    };
}
