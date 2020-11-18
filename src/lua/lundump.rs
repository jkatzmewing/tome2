use ::libc;
extern "C" {
    pub type TM;
    pub type lua_longjmp;
    #[no_mangle]
    fn luaF_protook(L: *mut lua_State, f: *mut Proto, pc: libc::c_int);
    #[no_mangle]
    fn luaF_newproto(L: *mut lua_State) -> *mut Proto;
    #[no_mangle]
    fn luaO_openspace(L: *mut lua_State, n: size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn luaO_verror(L: *mut lua_State, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn luaM_realloc(L: *mut lua_State, oldblock: *mut libc::c_void,
                    size: lint32) -> *mut libc::c_void;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: size_t)
     -> *mut TString;
    #[no_mangle]
    fn luaZ_read(z: *mut ZIO, b: *mut libc::c_void, n: size_t) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type Number = libc::c_long;
pub type lint32 = libc::c_ulong;
pub type Instruction = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: *mut Closure,
    pub pc: *mut *const Instruction,
    pub lastpc: libc::c_int,
    pub line: libc::c_int,
    pub refi: libc::c_int,
}
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
pub type StkId = *mut TObject;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const OP_CLOSURE: C2RustUnnamed_3 = 48;
pub const OP_LFORLOOP: C2RustUnnamed_3 = 47;
pub const OP_LFORPREP: C2RustUnnamed_3 = 46;
pub const OP_FORLOOP: C2RustUnnamed_3 = 45;
pub const OP_FORPREP: C2RustUnnamed_3 = 44;
pub const OP_PUSHNILJMP: C2RustUnnamed_3 = 43;
pub const OP_JMP: C2RustUnnamed_3 = 42;
pub const OP_JMPONF: C2RustUnnamed_3 = 41;
pub const OP_JMPONT: C2RustUnnamed_3 = 40;
pub const OP_JMPF: C2RustUnnamed_3 = 39;
pub const OP_JMPT: C2RustUnnamed_3 = 38;
pub const OP_JMPGE: C2RustUnnamed_3 = 37;
pub const OP_JMPGT: C2RustUnnamed_3 = 36;
pub const OP_JMPLE: C2RustUnnamed_3 = 35;
pub const OP_JMPLT: C2RustUnnamed_3 = 34;
pub const OP_JMPEQ: C2RustUnnamed_3 = 33;
pub const OP_JMPNE: C2RustUnnamed_3 = 32;
pub const OP_NOT: C2RustUnnamed_3 = 31;
pub const OP_MINUS: C2RustUnnamed_3 = 30;
pub const OP_CONCAT: C2RustUnnamed_3 = 29;
pub const OP_POW: C2RustUnnamed_3 = 28;
pub const OP_DIV: C2RustUnnamed_3 = 27;
pub const OP_MULT: C2RustUnnamed_3 = 26;
pub const OP_SUB: C2RustUnnamed_3 = 25;
pub const OP_ADDI: C2RustUnnamed_3 = 24;
pub const OP_ADD: C2RustUnnamed_3 = 23;
pub const OP_SETMAP: C2RustUnnamed_3 = 22;
pub const OP_SETLIST: C2RustUnnamed_3 = 21;
pub const OP_SETTABLE: C2RustUnnamed_3 = 20;
pub const OP_SETGLOBAL: C2RustUnnamed_3 = 19;
pub const OP_SETLOCAL: C2RustUnnamed_3 = 18;
pub const OP_CREATETABLE: C2RustUnnamed_3 = 17;
pub const OP_PUSHSELF: C2RustUnnamed_3 = 16;
pub const OP_GETINDEXED: C2RustUnnamed_3 = 15;
pub const OP_GETDOTTED: C2RustUnnamed_3 = 14;
pub const OP_GETTABLE: C2RustUnnamed_3 = 13;
pub const OP_GETGLOBAL: C2RustUnnamed_3 = 12;
pub const OP_GETLOCAL: C2RustUnnamed_3 = 11;
pub const OP_PUSHUPVALUE: C2RustUnnamed_3 = 10;
pub const OP_PUSHNEGNUM: C2RustUnnamed_3 = 9;
pub const OP_PUSHNUM: C2RustUnnamed_3 = 8;
pub const OP_PUSHSTRING: C2RustUnnamed_3 = 7;
pub const OP_PUSHINT: C2RustUnnamed_3 = 6;
pub const OP_POP: C2RustUnnamed_3 = 5;
pub const OP_PUSHNIL: C2RustUnnamed_3 = 4;
pub const OP_TAILCALL: C2RustUnnamed_3 = 3;
pub const OP_CALL: C2RustUnnamed_3 = 2;
pub const OP_RETURN: C2RustUnnamed_3 = 1;
pub const OP_END: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zio {
    pub n: size_t,
    pub p: *const libc::c_uchar,
    pub filbuf: Option<unsafe extern "C" fn(_: *mut ZIO) -> libc::c_int>,
    pub u: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub buffer: [libc::c_uchar; 256],
}
pub type ZIO = zio;
unsafe extern "C" fn ZNAME(mut Z: *mut ZIO) -> *const libc::c_char {
    let mut s: *const libc::c_char = (*Z).name;
    return if *s as libc::c_int == '@' as i32 {
               s.offset(1 as libc::c_int as isize)
           } else { s };
}
unsafe extern "C" fn unexpectedEOZ(mut L: *mut lua_State, mut Z: *mut ZIO) {
    luaO_verror(L,
                b"unexpected end of file in `%.99s\'\x00" as *const u8 as
                    *const libc::c_char, ZNAME(Z));
}
unsafe extern "C" fn ezgetc(mut L: *mut lua_State, mut Z: *mut ZIO)
 -> libc::c_int {
    let fresh0 = (*Z).n;
    (*Z).n = (*Z).n.wrapping_sub(1);
    let mut c: libc::c_int =
        if fresh0 > 0 as libc::c_int as libc::c_ulong {
            let fresh1 = (*Z).p;
            (*Z).p = (*Z).p.offset(1);
            *fresh1 as libc::c_int
        } else { (*Z).filbuf.expect("non-null function pointer")(Z) };
    if c == -(1 as libc::c_int) { unexpectedEOZ(L, Z); }
    return c;
}
unsafe extern "C" fn ezread(mut L: *mut lua_State, mut Z: *mut ZIO,
                            mut b: *mut libc::c_void, mut n: libc::c_int) {
    let mut r: libc::c_int = luaZ_read(Z, b, n as size_t) as libc::c_int;
    if r != 0 as libc::c_int { unexpectedEOZ(L, Z); };
}
unsafe extern "C" fn LoadBlock(mut L: *mut lua_State,
                               mut b: *mut libc::c_void, mut size: size_t,
                               mut Z: *mut ZIO, mut swap: libc::c_int) {
    if swap != 0 {
        let mut p: *mut libc::c_char =
            (b as
                 *mut libc::c_char).offset(size as
                                               isize).offset(-(1 as
                                                                   libc::c_int
                                                                   as isize));
        let mut n: libc::c_int = size as libc::c_int;
        loop  {
            let fresh2 = n;
            n = n - 1;
            if !(fresh2 != 0) { break ; }
            let fresh3 = p;
            p = p.offset(-1);
            *fresh3 = ezgetc(L, Z) as libc::c_char
        }
    } else { ezread(L, Z, b, size as libc::c_int); };
}
unsafe extern "C" fn LoadVector(mut L: *mut lua_State,
                                mut b: *mut libc::c_void, mut m: libc::c_int,
                                mut size: size_t, mut Z: *mut ZIO,
                                mut swap: libc::c_int) {
    if swap != 0 {
        let mut q: *mut libc::c_char = b as *mut libc::c_char;
        loop  {
            let fresh4 = m;
            m = m - 1;
            if !(fresh4 != 0) { break ; }
            let mut p: *mut libc::c_char =
                q.offset(size as isize).offset(-(1 as libc::c_int as isize));
            let mut n: libc::c_int = size as libc::c_int;
            loop  {
                let fresh5 = n;
                n = n - 1;
                if !(fresh5 != 0) { break ; }
                let fresh6 = p;
                p = p.offset(-1);
                *fresh6 = ezgetc(L, Z) as libc::c_char
            }
            q = q.offset(size as isize)
        }
    } else {
        ezread(L, Z, b,
               (m as libc::c_ulong).wrapping_mul(size) as libc::c_int);
    };
}
unsafe extern "C" fn LoadInt(mut L: *mut lua_State, mut Z: *mut ZIO,
                             mut swap: libc::c_int) -> libc::c_int {
    let mut x: libc::c_int = 0;
    LoadBlock(L, &mut x as *mut libc::c_int as *mut libc::c_void,
              ::std::mem::size_of::<libc::c_int>() as libc::c_ulong, Z, swap);
    return x;
}
unsafe extern "C" fn LoadSize(mut L: *mut lua_State, mut Z: *mut ZIO,
                              mut swap: libc::c_int) -> size_t {
    let mut x: size_t = 0;
    LoadBlock(L, &mut x as *mut size_t as *mut libc::c_void,
              ::std::mem::size_of::<size_t>() as libc::c_ulong, Z, swap);
    return x;
}
unsafe extern "C" fn LoadNumber(mut L: *mut lua_State, mut Z: *mut ZIO,
                                mut swap: libc::c_int) -> Number {
    let mut x: Number = 0;
    LoadBlock(L, &mut x as *mut Number as *mut libc::c_void,
              ::std::mem::size_of::<Number>() as libc::c_ulong, Z, swap);
    return x;
}
unsafe extern "C" fn LoadString(mut L: *mut lua_State, mut Z: *mut ZIO,
                                mut swap: libc::c_int) -> *mut TString {
    let mut size: size_t = LoadSize(L, Z, swap);
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut TString
    } else {
        let mut s: *mut libc::c_char = luaO_openspace(L, size);
        LoadBlock(L, s as *mut libc::c_void, size, Z, 0 as libc::c_int);
        return luaS_newlstr(L, s,
                            size.wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong))
        /* remove trailing '\0' */
    };
}
unsafe extern "C" fn LoadCode(mut L: *mut lua_State, mut tf: *mut Proto,
                              mut Z: *mut ZIO, mut swap: libc::c_int) {
    let mut size: libc::c_int = LoadInt(L, Z, swap);
    (*tf).code =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (size as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>()
                                                          as libc::c_ulong))
            as *mut Instruction;
    LoadVector(L, (*tf).code as *mut libc::c_void, size,
               ::std::mem::size_of::<Instruction>() as libc::c_ulong, Z,
               swap);
    if *(*tf).code.offset((size - 1 as libc::c_int) as isize) !=
           OP_END as libc::c_int as libc::c_ulong {
        luaO_verror(L,
                    b"bad code in `%.99s\'\x00" as *const u8 as
                        *const libc::c_char, ZNAME(Z));
    }
    luaF_protook(L, tf, size);
}
unsafe extern "C" fn LoadLocals(mut L: *mut lua_State, mut tf: *mut Proto,
                                mut Z: *mut ZIO, mut swap: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = LoadInt(L, Z, swap);
    (*tf).nlocvars = n;
    (*tf).locvars =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (n as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<LocVar>()
                                                          as libc::c_ulong))
            as *mut LocVar;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh7 = (*(*tf).locvars.offset(i as isize)).varname;
        *fresh7 = LoadString(L, Z, swap);
        (*(*tf).locvars.offset(i as isize)).startpc = LoadInt(L, Z, swap);
        (*(*tf).locvars.offset(i as isize)).endpc = LoadInt(L, Z, swap);
        i += 1
    };
}
unsafe extern "C" fn LoadLines(mut L: *mut lua_State, mut tf: *mut Proto,
                               mut Z: *mut ZIO, mut swap: libc::c_int) {
    let mut n: libc::c_int = 0;
    n = LoadInt(L, Z, swap);
    (*tf).nlineinfo = n;
    (*tf).lineinfo =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (n as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                          as libc::c_ulong))
            as *mut libc::c_int;
    LoadVector(L, (*tf).lineinfo as *mut libc::c_void, n,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong, Z,
               swap);
}
unsafe extern "C" fn LoadConstants(mut L: *mut lua_State, mut tf: *mut Proto,
                                   mut Z: *mut ZIO, mut swap: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = LoadInt(L, Z, swap);
    (*tf).nkstr = n;
    (*tf).kstr =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (n as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                          as libc::c_ulong))
            as *mut *mut TString;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh8 = *(*tf).kstr.offset(i as isize);
        *fresh8 = LoadString(L, Z, swap);
        i += 1
    }
    n = LoadInt(L, Z, swap);
    (*tf).nknum = n;
    (*tf).knum =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (n as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<Number>()
                                                          as libc::c_ulong))
            as *mut Number;
    LoadVector(L, (*tf).knum as *mut libc::c_void, n,
               ::std::mem::size_of::<Number>() as libc::c_ulong, Z, swap);
    n = LoadInt(L, Z, swap);
    (*tf).nkproto = n;
    (*tf).kproto =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     (n as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>()
                                                          as libc::c_ulong))
            as *mut *mut Proto;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh9 = *(*tf).kproto.offset(i as isize);
        *fresh9 = LoadFunction(L, Z, swap);
        i += 1
    };
}
unsafe extern "C" fn LoadFunction(mut L: *mut lua_State, mut Z: *mut ZIO,
                                  mut swap: libc::c_int) -> *mut Proto {
    let mut tf: *mut Proto = luaF_newproto(L);
    (*tf).source = LoadString(L, Z, swap);
    (*tf).lineDefined = LoadInt(L, Z, swap);
    (*tf).numparams = LoadInt(L, Z, swap) as libc::c_short;
    (*tf).is_vararg = ezgetc(L, Z) as libc::c_short;
    (*tf).maxstacksize = LoadInt(L, Z, swap) as libc::c_short;
    LoadLocals(L, tf, Z, swap);
    LoadLines(L, tf, Z, swap);
    LoadConstants(L, tf, Z, swap);
    LoadCode(L, tf, Z, swap);
    return tf;
}
unsafe extern "C" fn LoadSignature(mut L: *mut lua_State, mut Z: *mut ZIO) {
    let mut s: *const libc::c_char =
        b"Lua\x00" as *const u8 as *const libc::c_char;
    while *s as libc::c_int != 0 as libc::c_int &&
              ezgetc(L, Z) == *s as libc::c_int {
        s = s.offset(1)
    }
    if *s as libc::c_int != 0 as libc::c_int {
        luaO_verror(L,
                    b"bad signature in `%.99s\'\x00" as *const u8 as
                        *const libc::c_char, ZNAME(Z));
    };
}
unsafe extern "C" fn TestSize(mut L: *mut lua_State, mut s: libc::c_int,
                              mut what: *const libc::c_char,
                              mut Z: *mut ZIO) {
    let mut r: libc::c_int = ezgetc(L, Z);
    if r != s {
        luaO_verror(L,
                    b"virtual machine mismatch in `%.99s\':\n  %.20s is %d but read %d\x00"
                        as *const u8 as *const libc::c_char, ZNAME(Z), what,
                    s, r);
    };
}
unsafe extern "C" fn LoadHeader(mut L: *mut lua_State, mut Z: *mut ZIO)
 -> libc::c_int {
    let mut version: libc::c_int = 0;
    let mut swap: libc::c_int = 0;
    let mut f: Number = 0 as libc::c_int as Number;
    let mut tf: Number = 3 as libc::c_int as Number;
    LoadSignature(L, Z);
    version = ezgetc(L, Z);
    if version > 0x40 as libc::c_int {
        luaO_verror(L,
                    b"`%.99s\' too new:\n  read version %d.%d; expected at most %d.%d\x00"
                        as *const u8 as *const libc::c_char, ZNAME(Z),
                    version / 16 as libc::c_int, version % 16 as libc::c_int,
                    0x40 as libc::c_int / 16 as libc::c_int,
                    0x40 as libc::c_int % 16 as libc::c_int);
    }
    if version < 0x40 as libc::c_int {
        /* check last major change */
        luaO_verror(L,
                    b"`%.99s\' too old:\n  read version %d.%d; expected at least %d.%d\x00"
                        as *const u8 as *const libc::c_char, ZNAME(Z),
                    version / 16 as libc::c_int, version % 16 as libc::c_int,
                    0x40 as libc::c_int / 16 as libc::c_int,
                    0x40 as libc::c_int %
                        16 as libc::c_int); /* need to swap bytes? */
    }
    swap = (luaU_endianess() != ezgetc(L, Z)) as libc::c_int;
    TestSize(L,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                 libc::c_int,
             b"sizeof(int)\x00" as *const u8 as *const libc::c_char, Z);
    TestSize(L,
             ::std::mem::size_of::<size_t>() as libc::c_ulong as libc::c_int,
             b"sizeof(size_t)\x00" as *const u8 as *const libc::c_char, Z);
    TestSize(L,
             ::std::mem::size_of::<Instruction>() as libc::c_ulong as
                 libc::c_int,
             b"sizeof(Instruction)\x00" as *const u8 as *const libc::c_char,
             Z);
    TestSize(L, 32 as libc::c_int,
             b"SIZE_INSTRUCTION\x00" as *const u8 as *const libc::c_char, Z);
    TestSize(L, 6 as libc::c_int,
             b"SIZE_OP\x00" as *const u8 as *const libc::c_char, Z);
    TestSize(L, 9 as libc::c_int,
             b"SIZE_B\x00" as *const u8 as *const libc::c_char, Z);
    TestSize(L,
             ::std::mem::size_of::<Number>() as libc::c_ulong as libc::c_int,
             b"sizeof(Number)\x00" as *const u8 as *const libc::c_char, Z);
    f = LoadNumber(L, Z, swap);
    if f != tf {
        /* disregard errors in last bit of fraction */
        luaO_verror(L,
                    b"unknown number format in `%.99s\':\n  read %ld; expected %ld\x00"
                        as *const u8 as *const libc::c_char, ZNAME(Z), f, tf);
    }
    return swap;
}
unsafe extern "C" fn LoadChunk(mut L: *mut lua_State, mut Z: *mut ZIO)
 -> *mut Proto {
    return LoadFunction(L, Z, LoadHeader(L, Z));
}
/*
** load one chunk from a file or buffer
** return main if ok and NULL at EOF
*/
#[no_mangle]
pub unsafe extern "C" fn luaU_undump(mut L: *mut lua_State, mut Z: *mut ZIO)
 -> *mut Proto {
    let mut tf: *mut Proto = 0 as *mut Proto;
    let fresh10 = (*Z).n;
    (*Z).n = (*Z).n.wrapping_sub(1);
    let mut c: libc::c_int =
        if fresh10 > 0 as libc::c_int as libc::c_ulong {
            let fresh11 = (*Z).p;
            (*Z).p = (*Z).p.offset(1);
            *fresh11 as libc::c_int
        } else { (*Z).filbuf.expect("non-null function pointer")(Z) };
    if c == 27 as libc::c_int { tf = LoadChunk(L, Z) }
    let fresh12 = (*Z).n;
    (*Z).n = (*Z).n.wrapping_sub(1);
    c =
        if fresh12 > 0 as libc::c_int as libc::c_ulong {
            let fresh13 = (*Z).p;
            (*Z).p = (*Z).p.offset(1);
            *fresh13 as libc::c_int
        } else { (*Z).filbuf.expect("non-null function pointer")(Z) };
    if c != -(1 as libc::c_int) {
        luaO_verror(L,
                    b"`%.99s\' apparently contains more than one chunk\x00" as
                        *const u8 as *const libc::c_char, ZNAME(Z));
    }
    return tf;
}
/*
** find byte order
*/
#[no_mangle]
pub unsafe extern "C" fn luaU_endianess() -> libc::c_int {
    let mut x: libc::c_int = 1 as libc::c_int;
    return *(&mut x as *mut libc::c_int as *mut libc::c_char) as libc::c_int;
}
