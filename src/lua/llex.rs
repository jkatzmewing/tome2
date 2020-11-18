use ::libc;
extern "C" {
    pub type TM;
    pub type lua_longjmp;
    pub type Breaklabel;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn luaO_openspace(L: *mut lua_State, n: size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn luaO_str2d(s: *const libc::c_char, result: *mut Number) -> libc::c_int;
    #[no_mangle]
    fn luaO_verror(L: *mut lua_State, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn luaO_chunkid(out: *mut libc::c_char, source: *const libc::c_char,
                    len: libc::c_int);
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: size_t)
     -> *mut TString;
    #[no_mangle]
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
    pub f: C2RustUnnamed_0,
    pub next: *mut Closure,
    pub mark: *mut Closure,
    pub isC: libc::c_short,
    pub nupvalues: libc::c_short,
    pub upvalue: [TObject; 1],
}
pub type TObject = lua_TObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
    pub u: C2RustUnnamed_1,
    pub len: size_t,
    pub nexthash: *mut TString,
    pub marked: libc::c_int,
    pub str_0: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: C2RustUnnamed_3,
    pub d: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tag: libc::c_int,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub struct zio {
    pub n: size_t,
    pub p: *const libc::c_uchar,
    pub filbuf: Option<unsafe extern "C" fn(_: *mut ZIO) -> libc::c_int>,
    pub u: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub buffer: [libc::c_uchar; 256],
}
pub type ZIO = zio;
pub type RESERVED = libc::c_uint;
pub const TK_EOS: RESERVED = 284;
pub const TK_STRING: RESERVED = 283;
pub const TK_NUMBER: RESERVED = 282;
pub const TK_NE: RESERVED = 281;
pub const TK_LE: RESERVED = 280;
pub const TK_GE: RESERVED = 279;
pub const TK_EQ: RESERVED = 278;
pub const TK_DOTS: RESERVED = 277;
pub const TK_CONCAT: RESERVED = 276;
pub const TK_NAME: RESERVED = 275;
pub const TK_WHILE: RESERVED = 274;
pub const TK_UNTIL: RESERVED = 273;
pub const TK_THEN: RESERVED = 272;
pub const TK_RETURN: RESERVED = 271;
pub const TK_REPEAT: RESERVED = 270;
pub const TK_OR: RESERVED = 269;
pub const TK_NOT: RESERVED = 268;
pub const TK_NIL: RESERVED = 267;
pub const TK_LOCAL: RESERVED = 266;
pub const TK_IF: RESERVED = 265;
pub const TK_FUNCTION: RESERVED = 264;
pub const TK_FOR: RESERVED = 263;
pub const TK_END: RESERVED = 262;
pub const TK_ELSEIF: RESERVED = 261;
pub const TK_ELSE: RESERVED = 260;
pub const TK_DO: RESERVED = 259;
pub const TK_BREAK: RESERVED = 258;
pub const TK_AND: RESERVED = 257;
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    pub r: Number,
    pub ts: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: libc::c_int,
    pub seminfo: SemInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: libc::c_int,
    pub t: Token,
    pub lookahead: Token,
    pub fs: *mut FuncState,
    pub L: *mut lua_State,
    pub z: *mut zio,
    pub linenumber: libc::c_int,
    pub lastline: libc::c_int,
    pub source: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncState {
    pub f: *mut Proto,
    pub prev: *mut FuncState,
    pub ls: *mut LexState,
    pub L: *mut lua_State,
    pub pc: libc::c_int,
    pub lasttarget: libc::c_int,
    pub jlt: libc::c_int,
    pub stacklevel: libc::c_short,
    pub nactloc: libc::c_short,
    pub nupvalues: libc::c_short,
    pub lastline: libc::c_int,
    pub bl: *mut Breaklabel,
    pub upvalues: [expdesc; 32],
    pub actloc: [libc::c_int; 200],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expdesc {
    pub k: expkind,
    pub u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub index: libc::c_int,
    pub l: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub t: libc::c_int,
    pub f: libc::c_int,
}
pub type expkind = libc::c_uint;
pub const VEXP: expkind = 3;
pub const VINDEXED: expkind = 2;
pub const VLOCAL: expkind = 1;
pub const VGLOBAL: expkind = 0;
/* ORDER RESERVED */
static mut token2string: [*const libc::c_char; 28] =
    [b"and\x00" as *const u8 as *const libc::c_char,
     b"break\x00" as *const u8 as *const libc::c_char,
     b"do\x00" as *const u8 as *const libc::c_char,
     b"else\x00" as *const u8 as *const libc::c_char,
     b"elseif\x00" as *const u8 as *const libc::c_char,
     b"end\x00" as *const u8 as *const libc::c_char,
     b"for\x00" as *const u8 as *const libc::c_char,
     b"function\x00" as *const u8 as *const libc::c_char,
     b"if\x00" as *const u8 as *const libc::c_char,
     b"local\x00" as *const u8 as *const libc::c_char,
     b"nil\x00" as *const u8 as *const libc::c_char,
     b"not\x00" as *const u8 as *const libc::c_char,
     b"or\x00" as *const u8 as *const libc::c_char,
     b"repeat\x00" as *const u8 as *const libc::c_char,
     b"return\x00" as *const u8 as *const libc::c_char,
     b"then\x00" as *const u8 as *const libc::c_char,
     b"until\x00" as *const u8 as *const libc::c_char,
     b"while\x00" as *const u8 as *const libc::c_char,
     b"\x00" as *const u8 as *const libc::c_char,
     b"..\x00" as *const u8 as *const libc::c_char,
     b"...\x00" as *const u8 as *const libc::c_char,
     b"==\x00" as *const u8 as *const libc::c_char,
     b">=\x00" as *const u8 as *const libc::c_char,
     b"<=\x00" as *const u8 as *const libc::c_char,
     b"~=\x00" as *const u8 as *const libc::c_char,
     b"\x00" as *const u8 as *const libc::c_char,
     b"\x00" as *const u8 as *const libc::c_char,
     b"<eof>\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn luaX_init(mut L: *mut lua_State) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < TK_WHILE as libc::c_int - 257 as libc::c_int + 1 as libc::c_int
          {
        let mut ts: *mut TString = luaS_new(L, token2string[i as usize]);
        (*ts).marked = (3 as libc::c_int + i) as libc::c_uchar as libc::c_int;
        i += 1
        /* reserved word */
    }; /* skip '\n' */
}
#[no_mangle]
pub unsafe extern "C" fn luaX_checklimit(mut ls: *mut LexState,
                                         mut val: libc::c_int,
                                         mut limit: libc::c_int,
                                         mut msg: *const libc::c_char) {
    if val > limit {
        let mut buff: [libc::c_char; 100] =
            [0; 100]; /* no look-ahead token */
        sprintf(buff.as_mut_ptr(),
                b"too many %.50s (limit=%d)\x00" as *const u8 as
                    *const libc::c_char, msg, limit); /* read first char */
        luaX_error(ls, buff.as_mut_ptr(), (*ls).t.token);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaX_syntaxerror(mut ls: *mut LexState,
                                          mut s: *const libc::c_char,
                                          mut token: *const libc::c_char) {
    let mut buff: [libc::c_char; 80] = [0; 80];
    luaO_chunkid(buff.as_mut_ptr(), (*(*ls).source).str_0.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong
                     as libc::c_int);
    luaO_verror((*ls).L,
                b"%.99s;\n  last token read: `%.30s\' at line %d in %.80s\x00"
                    as *const u8 as *const libc::c_char, s, token,
                (*ls).linenumber, buff.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn luaX_error(mut ls: *mut LexState,
                                    mut s: *const libc::c_char,
                                    mut token: libc::c_int) {
    let mut buff: [libc::c_char; 15] = [0; 15];
    luaX_token2str(token, buff.as_mut_ptr());
    if buff[0 as libc::c_int as usize] as libc::c_int == '\u{0}' as i32 {
        luaX_syntaxerror(ls, s, (*(*ls).L).Mbuffer);
    } else { luaX_syntaxerror(ls, s, buff.as_mut_ptr()); };
}
#[no_mangle]
pub unsafe extern "C" fn luaX_token2str(mut token: libc::c_int,
                                        mut s: *mut libc::c_char) {
    if token < 256 as libc::c_int {
        *s.offset(0 as libc::c_int as isize) = token as libc::c_char;
        *s.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char
    } else {
        strcpy(s, token2string[(token - 257 as libc::c_int) as usize]);
    };
}
unsafe extern "C" fn luaX_invalidchar(mut ls: *mut LexState,
                                      mut c: libc::c_int) {
    let mut buff: [libc::c_char; 8] = [0; 8];
    sprintf(buff.as_mut_ptr(),
            b"0x%02X\x00" as *const u8 as *const libc::c_char, c);
    luaX_syntaxerror(ls,
                     b"invalid control char\x00" as *const u8 as
                         *const libc::c_char, buff.as_mut_ptr());
}
unsafe extern "C" fn inclinenumber(mut LS: *mut LexState) {
    let fresh0 = (*(*LS).z).n;
    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
    (*LS).current =
        if fresh0 > 0 as libc::c_int as libc::c_ulong {
            let fresh1 = (*(*LS).z).p;
            (*(*LS).z).p = (*(*LS).z).p.offset(1);
            *fresh1 as libc::c_int
        } else {
            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
        };
    (*LS).linenumber += 1;
    luaX_checklimit(LS, (*LS).linenumber,
                    2147483647 as libc::c_int - 2 as libc::c_int,
                    b"lines in a chunk\x00" as *const u8 as
                        *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn luaX_setinput(mut L: *mut lua_State,
                                       mut LS: *mut LexState, mut z: *mut ZIO,
                                       mut source: *mut TString) {
    (*LS).L = L;
    (*LS).lookahead.token = TK_EOS as libc::c_int;
    (*LS).z = z;
    (*LS).fs = 0 as *mut FuncState;
    (*LS).linenumber = 1 as libc::c_int;
    (*LS).lastline = 1 as libc::c_int;
    (*LS).source = source;
    let fresh2 = (*(*LS).z).n;
    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
    (*LS).current =
        if fresh2 > 0 as libc::c_int as libc::c_ulong {
            let fresh3 = (*(*LS).z).p;
            (*(*LS).z).p = (*(*LS).z).p.offset(1);
            *fresh3 as libc::c_int
        } else {
            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
        };
    if (*LS).current == '#' as i32 {
        loop  {
            /* skip first line */
            let fresh4 = (*(*LS).z).n;
            (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
            (*LS).current =
                if fresh4 > 0 as libc::c_int as libc::c_ulong {
                    let fresh5 = (*(*LS).z).p;
                    (*(*LS).z).p = (*(*LS).z).p.offset(1);
                    *fresh5 as libc::c_int
                } else {
                    (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                };
            if !((*LS).current != '\n' as i32 && (*LS).current != '\r' as i32
                     && (*LS).current != -(1 as libc::c_int)) {
                break ;
            }
        }
    };
}
unsafe extern "C" fn readname(mut LS: *mut LexState) -> *const libc::c_char {
    let mut L: *mut lua_State = (*LS).L;
    let mut l: size_t = 0 as libc::c_int as size_t;
    if l.wrapping_add(10 as libc::c_int as libc::c_ulong) > (*L).Mbuffsize {
        luaO_openspace(L,
                       l.wrapping_add(10 as libc::c_int as
                                          libc::c_ulong).wrapping_add(128 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong));
    }
    loop  {
        if l.wrapping_add(10 as libc::c_int as libc::c_ulong) > (*L).Mbuffsize
           {
            luaO_openspace(L,
                           l.wrapping_add(10 as libc::c_int as
                                              libc::c_ulong).wrapping_add(128
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong));
        }
        let fresh6 = l;
        l = l.wrapping_add(1);
        *(*L).Mbuffer.offset(fresh6 as isize) = (*LS).current as libc::c_char;
        let fresh7 = (*(*LS).z).n;
        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
        (*LS).current =
            (if fresh7 > 0 as libc::c_int as libc::c_ulong {
                 let fresh8 = (*(*LS).z).p;
                 (*(*LS).z).p = (*(*LS).z).p.offset(1);
                 *fresh8 as libc::c_int
             } else {
                 (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
             });
        if !(*(*__ctype_b_loc()).offset((*LS).current as isize) as libc::c_int
                 & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int !=
                 0 || (*LS).current == '_' as i32) {
            break ;
        }
    }
    let fresh9 = l;
    l = l.wrapping_add(1);
    *(*L).Mbuffer.offset(fresh9 as isize) = '\u{0}' as i32 as libc::c_char;
    return (*L).Mbuffer;
}
/* LUA_NUMBER */
unsafe extern "C" fn read_number(mut LS: *mut LexState,
                                 mut comma: libc::c_int,
                                 mut seminfo: *mut SemInfo) {
    let mut L: *mut lua_State = (*LS).L; /* read 'E' */
    let mut l: size_t =
        0 as libc::c_int as size_t; /* optional exponent sign */
    if l.wrapping_add(10 as libc::c_int as libc::c_ulong) > (*L).Mbuffsize {
        luaO_openspace(L,
                       l.wrapping_add(10 as libc::c_int as
                                          libc::c_ulong).wrapping_add(128 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)); /* save first '[' */
    } /* pass the second '[' */
    if comma != 0 {
        let fresh10 = l; /* to avoid warnings */
        l = l.wrapping_add(1);
        *(*L).Mbuffer.offset(fresh10 as isize) = '.' as i32 as libc::c_char
    }
    while *(*__ctype_b_loc()).offset((*LS).current as isize) as libc::c_int &
              _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        if l.wrapping_add(10 as libc::c_int as libc::c_ulong) > (*L).Mbuffsize
           {
            luaO_openspace(L,
                           l.wrapping_add(10 as libc::c_int as
                                              libc::c_ulong).wrapping_add(128
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong));
        }
        let fresh11 = l;
        l = l.wrapping_add(1);
        *(*L).Mbuffer.offset(fresh11 as isize) =
            (*LS).current as libc::c_char;
        let fresh12 = (*(*LS).z).n;
        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
        (*LS).current =
            (if fresh12 > 0 as libc::c_int as libc::c_ulong {
                 let fresh13 = (*(*LS).z).p;
                 (*(*LS).z).p = (*(*LS).z).p.offset(1);
                 *fresh13 as libc::c_int
             } else {
                 (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
             })
    }
    if (*LS).current == '.' as i32 {
        let fresh14 = l;
        l = l.wrapping_add(1);
        *(*L).Mbuffer.offset(fresh14 as isize) =
            (*LS).current as libc::c_char;
        let fresh15 = (*(*LS).z).n;
        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
        (*LS).current =
            (if fresh15 > 0 as libc::c_int as libc::c_ulong {
                 let fresh16 = (*(*LS).z).p;
                 (*(*LS).z).p = (*(*LS).z).p.offset(1);
                 *fresh16 as libc::c_int
             } else {
                 (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
             });
        if (*LS).current == '.' as i32 {
            let fresh17 = l;
            l = l.wrapping_add(1);
            *(*L).Mbuffer.offset(fresh17 as isize) =
                (*LS).current as libc::c_char;
            let fresh18 = (*(*LS).z).n;
            (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
            (*LS).current =
                (if fresh18 > 0 as libc::c_int as libc::c_ulong {
                     let fresh19 = (*(*LS).z).p;
                     (*(*LS).z).p = (*(*LS).z).p.offset(1);
                     *fresh19 as libc::c_int
                 } else {
                     (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                 });
            let fresh20 = l;
            l = l.wrapping_add(1);
            *(*L).Mbuffer.offset(fresh20 as isize) =
                '\u{0}' as i32 as libc::c_char;
            luaX_error(LS,
                       b"ambiguous syntax (decimal point x string concatenation)\x00"
                           as *const u8 as *const libc::c_char,
                       TK_NUMBER as libc::c_int);
        }
    }
    while *(*__ctype_b_loc()).offset((*LS).current as isize) as libc::c_int &
              _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        if l.wrapping_add(10 as libc::c_int as libc::c_ulong) > (*L).Mbuffsize
           {
            luaO_openspace(L,
                           l.wrapping_add(10 as libc::c_int as
                                              libc::c_ulong).wrapping_add(128
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong));
        }
        let fresh21 = l;
        l = l.wrapping_add(1);
        *(*L).Mbuffer.offset(fresh21 as isize) =
            (*LS).current as libc::c_char;
        let fresh22 = (*(*LS).z).n;
        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
        (*LS).current =
            (if fresh22 > 0 as libc::c_int as libc::c_ulong {
                 let fresh23 = (*(*LS).z).p;
                 (*(*LS).z).p = (*(*LS).z).p.offset(1);
                 *fresh23 as libc::c_int
             } else {
                 (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
             })
    }
    if (*LS).current == 'e' as i32 || (*LS).current == 'E' as i32 {
        let fresh24 = l;
        l = l.wrapping_add(1);
        *(*L).Mbuffer.offset(fresh24 as isize) =
            (*LS).current as libc::c_char;
        let fresh25 = (*(*LS).z).n;
        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
        (*LS).current =
            (if fresh25 > 0 as libc::c_int as libc::c_ulong {
                 let fresh26 = (*(*LS).z).p;
                 (*(*LS).z).p = (*(*LS).z).p.offset(1);
                 *fresh26 as libc::c_int
             } else {
                 (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
             });
        if (*LS).current == '+' as i32 || (*LS).current == '-' as i32 {
            let fresh27 = l;
            l = l.wrapping_add(1);
            *(*L).Mbuffer.offset(fresh27 as isize) =
                (*LS).current as libc::c_char;
            let fresh28 = (*(*LS).z).n;
            (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
            (*LS).current =
                (if fresh28 > 0 as libc::c_int as libc::c_ulong {
                     let fresh29 = (*(*LS).z).p;
                     (*(*LS).z).p = (*(*LS).z).p.offset(1);
                     *fresh29 as libc::c_int
                 } else {
                     (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                 })
        }
        while *(*__ctype_b_loc()).offset((*LS).current as isize) as
                  libc::c_int &
                  _ISdigit as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            if l.wrapping_add(10 as libc::c_int as libc::c_ulong) >
                   (*L).Mbuffsize {
                luaO_openspace(L,
                               l.wrapping_add(10 as libc::c_int as
                                                  libc::c_ulong).wrapping_add(128
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong));
            }
            let fresh30 = l;
            l = l.wrapping_add(1);
            *(*L).Mbuffer.offset(fresh30 as isize) =
                (*LS).current as libc::c_char;
            let fresh31 = (*(*LS).z).n;
            (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
            (*LS).current =
                (if fresh31 > 0 as libc::c_int as libc::c_ulong {
                     let fresh32 = (*(*LS).z).p;
                     (*(*LS).z).p = (*(*LS).z).p.offset(1);
                     *fresh32 as libc::c_int
                 } else {
                     (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                 })
        }
    }
    let fresh33 = l;
    l = l.wrapping_add(1);
    *(*L).Mbuffer.offset(fresh33 as isize) = '\u{0}' as i32 as libc::c_char;
    if luaO_str2d((*L).Mbuffer, &mut (*seminfo).r) == 0 {
        luaX_error(LS,
                   b"malformed number\x00" as *const u8 as
                       *const libc::c_char, TK_NUMBER as libc::c_int);
    };
}
unsafe extern "C" fn read_long_string(mut LS: *mut LexState,
                                      mut seminfo: *mut SemInfo) {
    let mut L: *mut lua_State = (*LS).L;
    let mut cont: libc::c_int = 0 as libc::c_int;
    let mut l: size_t = 0 as libc::c_int as size_t;
    if l.wrapping_add(10 as libc::c_int as libc::c_ulong) > (*L).Mbuffsize {
        luaO_openspace(L,
                       l.wrapping_add(10 as libc::c_int as
                                          libc::c_ulong).wrapping_add(128 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong));
    }
    let fresh34 = l;
    l = l.wrapping_add(1);
    *(*L).Mbuffer.offset(fresh34 as isize) = '[' as i32 as libc::c_char;
    let fresh35 = l;
    l = l.wrapping_add(1);
    *(*L).Mbuffer.offset(fresh35 as isize) = (*LS).current as libc::c_char;
    let fresh36 = (*(*LS).z).n;
    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
    (*LS).current =
        (if fresh36 > 0 as libc::c_int as libc::c_ulong {
             let fresh37 = (*(*LS).z).p;
             (*(*LS).z).p = (*(*LS).z).p.offset(1);
             *fresh37 as libc::c_int
         } else {
             (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
         });
    loop  {
        if l.wrapping_add(10 as libc::c_int as libc::c_ulong) > (*L).Mbuffsize
           {
            luaO_openspace(L,
                           l.wrapping_add(10 as libc::c_int as
                                              libc::c_ulong).wrapping_add(128
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong));
        }
        match (*LS).current {
            -1 => {
                let fresh38 = l;
                l = l.wrapping_add(1);
                *(*L).Mbuffer.offset(fresh38 as isize) =
                    '\u{0}' as i32 as libc::c_char;
                if !seminfo.is_null() {
                    luaX_error(LS,
                               b"unfinished long string\x00" as *const u8 as
                                   *const libc::c_char,
                               TK_STRING as libc::c_int);
                } else {
                    luaX_error(LS,
                               b"unfinished comment\x00" as *const u8 as
                                   *const libc::c_char,
                               TK_EOS as libc::c_int);
                }
            }
            91 => {
                let fresh39 = l;
                l = l.wrapping_add(1);
                *(*L).Mbuffer.offset(fresh39 as isize) =
                    (*LS).current as libc::c_char;
                let fresh40 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    (if fresh40 > 0 as libc::c_int as libc::c_ulong {
                         let fresh41 = (*(*LS).z).p;
                         (*(*LS).z).p = (*(*LS).z).p.offset(1);
                         *fresh41 as libc::c_int
                     } else {
                         (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                     });
                if (*LS).current == '[' as i32 {
                    cont += 1;
                    let fresh42 = l;
                    l = l.wrapping_add(1);
                    *(*L).Mbuffer.offset(fresh42 as isize) =
                        (*LS).current as libc::c_char;
                    let fresh43 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        (if fresh43 > 0 as libc::c_int as libc::c_ulong {
                             let fresh44 = (*(*LS).z).p;
                             (*(*LS).z).p = (*(*LS).z).p.offset(1);
                             *fresh44 as libc::c_int
                         } else {
                             (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                         })
                }
            }
            93 => {
                let fresh45 = l;
                l = l.wrapping_add(1);
                *(*L).Mbuffer.offset(fresh45 as isize) =
                    (*LS).current as libc::c_char;
                let fresh46 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    (if fresh46 > 0 as libc::c_int as libc::c_ulong {
                         let fresh47 = (*(*LS).z).p;
                         (*(*LS).z).p = (*(*LS).z).p.offset(1);
                         *fresh47 as libc::c_int
                     } else {
                         (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                     });
                if !((*LS).current == ']' as i32) { continue ; }
                if cont == 0 as libc::c_int { break ; }
                cont -= 1;
                let fresh48 = l;
                l = l.wrapping_add(1);
                *(*L).Mbuffer.offset(fresh48 as isize) =
                    (*LS).current as libc::c_char;
                let fresh49 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    (if fresh49 > 0 as libc::c_int as libc::c_ulong {
                         let fresh50 = (*(*LS).z).p;
                         (*(*LS).z).p = (*(*LS).z).p.offset(1);
                         *fresh50 as libc::c_int
                     } else {
                         (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                     })
            }
            10 => {
                let fresh51 = l;
                l = l.wrapping_add(1);
                *(*L).Mbuffer.offset(fresh51 as isize) =
                    '\n' as i32 as libc::c_char;
                inclinenumber(LS);
                if (*LS).current == '\r' as i32 {
                    let fresh52 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        if fresh52 > 0 as libc::c_int as libc::c_ulong {
                            let fresh53 = (*(*LS).z).p;
                            (*(*LS).z).p = (*(*LS).z).p.offset(1);
                            *fresh53 as libc::c_int
                        } else {
                            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                        }
                }
            }
            13 => {
                let fresh54 = l;
                l = l.wrapping_add(1);
                *(*L).Mbuffer.offset(fresh54 as isize) =
                    '\n' as i32 as libc::c_char;
                inclinenumber(LS);
                if (*LS).current == '\n' as i32 {
                    let fresh55 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        if fresh55 > 0 as libc::c_int as libc::c_ulong {
                            let fresh56 = (*(*LS).z).p;
                            (*(*LS).z).p = (*(*LS).z).p.offset(1);
                            *fresh56 as libc::c_int
                        } else {
                            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                        }
                }
            }
            _ => {
                if !seminfo.is_null() {
                    /* no need to save complete comment */
                    let fresh57 = l; /* skip the second ']' */
                    l = l.wrapping_add(1); /* do not save the '\' */
                    *(*L).Mbuffer.offset(fresh57 as isize) =
                        (*LS).current as libc::c_char
                }
                let fresh58 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    if fresh58 > 0 as libc::c_int as libc::c_ulong {
                        let fresh59 = (*(*LS).z).p;
                        (*(*LS).z).p = (*(*LS).z).p.offset(1);
                        *fresh59 as libc::c_int
                    } else {
                        (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                    }
            }
        }
    }
    let fresh60 = l;
    l = l.wrapping_add(1);
    *(*L).Mbuffer.offset(fresh60 as isize) = (*LS).current as libc::c_char;
    let fresh61 = (*(*LS).z).n;
    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
    (*LS).current =
        (if fresh61 > 0 as libc::c_int as libc::c_ulong {
             let fresh62 = (*(*LS).z).p;
             (*(*LS).z).p = (*(*LS).z).p.offset(1);
             *fresh62 as libc::c_int
         } else {
             (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
         });
    let fresh63 = l;
    l = l.wrapping_add(1);
    *(*L).Mbuffer.offset(fresh63 as isize) = '\u{0}' as i32 as libc::c_char;
    if !seminfo.is_null() {
        (*seminfo).ts =
            luaS_newlstr(L, (*L).Mbuffer.offset(2 as libc::c_int as isize),
                         l.wrapping_sub(5 as libc::c_int as libc::c_ulong))
    };
}
unsafe extern "C" fn read_string(mut LS: *mut LexState, mut del: libc::c_int,
                                 mut seminfo: *mut SemInfo) {
    let mut L: *mut lua_State = (*LS).L;
    let mut l: size_t = 0 as libc::c_int as size_t;
    if l.wrapping_add(10 as libc::c_int as libc::c_ulong) > (*L).Mbuffsize {
        luaO_openspace(L,
                       l.wrapping_add(10 as libc::c_int as
                                          libc::c_ulong).wrapping_add(128 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong));
    }
    let fresh64 = l;
    l = l.wrapping_add(1);
    *(*L).Mbuffer.offset(fresh64 as isize) = (*LS).current as libc::c_char;
    let fresh65 = (*(*LS).z).n;
    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
    (*LS).current =
        (if fresh65 > 0 as libc::c_int as libc::c_ulong {
             let fresh66 = (*(*LS).z).p;
             (*(*LS).z).p = (*(*LS).z).p.offset(1);
             *fresh66 as libc::c_int
         } else {
             (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
         });
    while (*LS).current != del {
        if l.wrapping_add(10 as libc::c_int as libc::c_ulong) > (*L).Mbuffsize
           {
            luaO_openspace(L,
                           l.wrapping_add(10 as libc::c_int as
                                              libc::c_ulong).wrapping_add(128
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong));
        }
        match (*LS).current {
            -1 | 10 | 13 => {
                let fresh67 = l;
                l = l.wrapping_add(1);
                *(*L).Mbuffer.offset(fresh67 as isize) =
                    '\u{0}' as i32 as libc::c_char;
                luaX_error(LS,
                           b"unfinished string\x00" as *const u8 as
                               *const libc::c_char, TK_STRING as libc::c_int);
            }
            92 => {
                let fresh68 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    if fresh68 > 0 as libc::c_int as libc::c_ulong {
                        let fresh69 = (*(*LS).z).p;
                        (*(*LS).z).p = (*(*LS).z).p.offset(1);
                        *fresh69 as libc::c_int
                    } else {
                        (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                    };
                match (*LS).current {
                    97 => {
                        let fresh70 = l;
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh70 as isize) =
                            '\u{7}' as i32 as libc::c_char;
                        let fresh71 = (*(*LS).z).n;
                        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                        (*LS).current =
                            if fresh71 > 0 as libc::c_int as libc::c_ulong {
                                let fresh72 = (*(*LS).z).p;
                                (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                *fresh72 as libc::c_int
                            } else {
                                (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                            }
                    }
                    98 => {
                        let fresh73 = l;
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh73 as isize) =
                            '\u{8}' as i32 as libc::c_char;
                        let fresh74 = (*(*LS).z).n;
                        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                        (*LS).current =
                            if fresh74 > 0 as libc::c_int as libc::c_ulong {
                                let fresh75 = (*(*LS).z).p;
                                (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                *fresh75 as libc::c_int
                            } else {
                                (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                            }
                    }
                    102 => {
                        let fresh76 = l;
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh76 as isize) =
                            '\u{c}' as i32 as libc::c_char;
                        let fresh77 = (*(*LS).z).n;
                        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                        (*LS).current =
                            if fresh77 > 0 as libc::c_int as libc::c_ulong {
                                let fresh78 = (*(*LS).z).p;
                                (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                *fresh78 as libc::c_int
                            } else {
                                (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                            }
                    }
                    110 => {
                        let fresh79 = l;
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh79 as isize) =
                            '\n' as i32 as libc::c_char;
                        let fresh80 = (*(*LS).z).n;
                        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                        (*LS).current =
                            if fresh80 > 0 as libc::c_int as libc::c_ulong {
                                let fresh81 = (*(*LS).z).p;
                                (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                *fresh81 as libc::c_int
                            } else {
                                (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                            }
                    }
                    114 => {
                        let fresh82 = l;
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh82 as isize) =
                            '\r' as i32 as libc::c_char;
                        let fresh83 = (*(*LS).z).n;
                        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                        (*LS).current =
                            if fresh83 > 0 as libc::c_int as libc::c_ulong {
                                let fresh84 = (*(*LS).z).p;
                                (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                *fresh84 as libc::c_int
                            } else {
                                (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                            }
                    }
                    116 => {
                        let fresh85 = l;
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh85 as isize) =
                            '\t' as i32 as libc::c_char;
                        let fresh86 = (*(*LS).z).n;
                        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                        (*LS).current =
                            if fresh86 > 0 as libc::c_int as libc::c_ulong {
                                let fresh87 = (*(*LS).z).p;
                                (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                *fresh87 as libc::c_int
                            } else {
                                (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                            }
                    }
                    118 => {
                        let fresh88 = l;
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh88 as isize) =
                            '\u{b}' as i32 as libc::c_char;
                        let fresh89 = (*(*LS).z).n;
                        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                        (*LS).current =
                            if fresh89 > 0 as libc::c_int as libc::c_ulong {
                                let fresh90 = (*(*LS).z).p;
                                (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                *fresh90 as libc::c_int
                            } else {
                                (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                            }
                    }
                    10 => {
                        let fresh91 = l;
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh91 as isize) =
                            '\n' as i32 as libc::c_char;
                        inclinenumber(LS);
                        if (*LS).current == '\r' as i32 {
                            let fresh92 = (*(*LS).z).n;
                            (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                            (*LS).current =
                                if fresh92 > 0 as libc::c_int as libc::c_ulong
                                   {
                                    let fresh93 = (*(*LS).z).p;
                                    (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                    *fresh93 as libc::c_int
                                } else {
                                    (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                                }
                        }
                    }
                    13 => {
                        let fresh94 = l;
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh94 as isize) =
                            '\n' as i32 as libc::c_char;
                        inclinenumber(LS);
                        if (*LS).current == '\n' as i32 {
                            let fresh95 = (*(*LS).z).n;
                            (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                            (*LS).current =
                                if fresh95 > 0 as libc::c_int as libc::c_ulong
                                   {
                                    let fresh96 = (*(*LS).z).p;
                                    (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                    *fresh96 as libc::c_int
                                } else {
                                    (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                                }
                        }
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        let mut c: libc::c_int = 0 as libc::c_int;
                        let mut i: libc::c_int = 0 as libc::c_int;
                        loop  {
                            c =
                                10 as libc::c_int * c +
                                    ((*LS).current - '0' as i32);
                            let fresh97 = (*(*LS).z).n;
                            (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                            (*LS).current =
                                if fresh97 > 0 as libc::c_int as libc::c_ulong
                                   {
                                    let fresh98 = (*(*LS).z).p;
                                    (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                    *fresh98 as libc::c_int
                                } else {
                                    (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                                };
                            i += 1;
                            if !(i < 3 as libc::c_int &&
                                     *(*__ctype_b_loc()).offset((*LS).current
                                                                    as isize)
                                         as libc::c_int &
                                         _ISdigit as libc::c_int as
                                             libc::c_ushort as libc::c_int !=
                                         0) {
                                break ;
                            }
                        }
                        if c != c as libc::c_uchar as libc::c_int {
                            let fresh99 = l;
                            l = l.wrapping_add(1);
                            *(*L).Mbuffer.offset(fresh99 as isize) =
                                '\u{0}' as i32 as libc::c_char;
                            luaX_error(LS,
                                       b"escape sequence too large\x00" as
                                           *const u8 as *const libc::c_char,
                                       TK_STRING as libc::c_int);
                        }
                        let fresh100 = l;
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh100 as isize) =
                            c as libc::c_char
                    }
                    _ => {
                        /* handles \\, \", \', and \? */
                        let fresh101 = l; /* skip delimiter */
                        l = l.wrapping_add(1);
                        *(*L).Mbuffer.offset(fresh101 as isize) =
                            (*LS).current as libc::c_char;
                        let fresh102 = (*(*LS).z).n;
                        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                        (*LS).current =
                            (if fresh102 > 0 as libc::c_int as libc::c_ulong {
                                 let fresh103 = (*(*LS).z).p;
                                 (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                 *fresh103 as libc::c_int
                             } else {
                                 (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                             })
                    }
                }
            }
            _ => {
                let fresh104 = l;
                l = l.wrapping_add(1);
                *(*L).Mbuffer.offset(fresh104 as isize) =
                    (*LS).current as libc::c_char;
                let fresh105 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    (if fresh105 > 0 as libc::c_int as libc::c_ulong {
                         let fresh106 = (*(*LS).z).p;
                         (*(*LS).z).p = (*(*LS).z).p.offset(1);
                         *fresh106 as libc::c_int
                     } else {
                         (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                     })
            }
        }
    }
    let fresh107 = l;
    l = l.wrapping_add(1);
    *(*L).Mbuffer.offset(fresh107 as isize) = (*LS).current as libc::c_char;
    let fresh108 = (*(*LS).z).n;
    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
    (*LS).current =
        (if fresh108 > 0 as libc::c_int as libc::c_ulong {
             let fresh109 = (*(*LS).z).p;
             (*(*LS).z).p = (*(*LS).z).p.offset(1);
             *fresh109 as libc::c_int
         } else {
             (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
         });
    let fresh110 = l;
    l = l.wrapping_add(1);
    *(*L).Mbuffer.offset(fresh110 as isize) = '\u{0}' as i32 as libc::c_char;
    (*seminfo).ts =
        luaS_newlstr(L, (*L).Mbuffer.offset(1 as libc::c_int as isize),
                     l.wrapping_sub(3 as libc::c_int as libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn luaX_lex(mut LS: *mut LexState,
                                  mut seminfo: *mut SemInfo) -> libc::c_int {
    loop  {
        match (*LS).current {
            32 | 9 => {
                let fresh111 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    if fresh111 > 0 as libc::c_int as libc::c_ulong {
                        let fresh112 = (*(*LS).z).p;
                        (*(*LS).z).p = (*(*LS).z).p.offset(1);
                        *fresh112 as libc::c_int
                    } else {
                        (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                    };
                continue ;
            }
            10 => {
                inclinenumber(LS);
                if (*LS).current == '\r' as i32 {
                    let fresh113 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        if fresh113 > 0 as libc::c_int as libc::c_ulong {
                            let fresh114 = (*(*LS).z).p;
                            (*(*LS).z).p = (*(*LS).z).p.offset(1);
                            *fresh114 as libc::c_int
                        } else {
                            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                        }
                }
                continue ;
            }
            13 => {
                inclinenumber(LS);
                if (*LS).current == '\n' as i32 {
                    let fresh115 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        if fresh115 > 0 as libc::c_int as libc::c_ulong {
                            let fresh116 = (*(*LS).z).p;
                            (*(*LS).z).p = (*(*LS).z).p.offset(1);
                            *fresh116 as libc::c_int
                        } else {
                            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                        }
                }
                continue ;
            }
            36 => {
                luaX_error(LS,
                           b"unexpected `$\' (pragmas are no longer supported)\x00"
                               as *const u8 as *const libc::c_char,
                           '$' as i32);
                continue ;
            }
            45 => {
                let fresh117 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    if fresh117 > 0 as libc::c_int as libc::c_ulong {
                        let fresh118 = (*(*LS).z).p;
                        (*(*LS).z).p = (*(*LS).z).p.offset(1);
                        *fresh118 as libc::c_int
                    } else {
                        (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                    };
                if (*LS).current != '-' as i32 { return '-' as i32 }
                let fresh119 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    (if fresh119 > 0 as libc::c_int as libc::c_ulong {
                         let fresh120 = (*(*LS).z).p;
                         (*(*LS).z).p = (*(*LS).z).p.offset(1);
                         *fresh120 as libc::c_int
                     } else {
                         (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                     });
                if (*LS).current == '[' as i32 &&
                       {
                           let fresh121 = (*(*LS).z).n;
                           (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                           (*LS).current =
                               (if fresh121 >
                                       0 as libc::c_int as libc::c_ulong {
                                    let fresh122 = (*(*LS).z).p;
                                    (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                    *fresh122 as libc::c_int
                                } else {
                                    (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                                });
                           ((*LS).current) == '[' as i32
                       } {
                    read_long_string(LS, 0 as *mut SemInfo);
                } else {
                    while (*LS).current != '\n' as i32 &&
                              (*LS).current != '\r' as i32 &&
                              (*LS).current != -(1 as libc::c_int) {
                        let fresh123 = (*(*LS).z).n;
                        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                        (*LS).current =
                            if fresh123 > 0 as libc::c_int as libc::c_ulong {
                                let fresh124 = (*(*LS).z).p;
                                (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                *fresh124 as libc::c_int
                            } else {
                                (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                            }
                    }
                }
                continue ;
            }
            91 => {
                let fresh125 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    if fresh125 > 0 as libc::c_int as libc::c_ulong {
                        let fresh126 = (*(*LS).z).p;
                        (*(*LS).z).p = (*(*LS).z).p.offset(1);
                        *fresh126 as libc::c_int
                    } else {
                        (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                    };
                if (*LS).current != '[' as i32 {
                    return '[' as i32
                } else {
                    read_long_string(LS, seminfo);
                    return TK_STRING as libc::c_int
                }
            }
            61 => {
                let fresh127 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    if fresh127 > 0 as libc::c_int as libc::c_ulong {
                        let fresh128 = (*(*LS).z).p;
                        (*(*LS).z).p = (*(*LS).z).p.offset(1);
                        *fresh128 as libc::c_int
                    } else {
                        (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                    };
                if (*LS).current != '=' as i32 {
                    return '=' as i32
                } else {
                    let fresh129 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        if fresh129 > 0 as libc::c_int as libc::c_ulong {
                            let fresh130 = (*(*LS).z).p;
                            (*(*LS).z).p = (*(*LS).z).p.offset(1);
                            *fresh130 as libc::c_int
                        } else {
                            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                        };
                    return TK_EQ as libc::c_int
                }
            }
            60 => {
                let fresh131 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    if fresh131 > 0 as libc::c_int as libc::c_ulong {
                        let fresh132 = (*(*LS).z).p;
                        (*(*LS).z).p = (*(*LS).z).p.offset(1);
                        *fresh132 as libc::c_int
                    } else {
                        (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                    };
                if (*LS).current != '=' as i32 {
                    return '<' as i32
                } else {
                    let fresh133 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        if fresh133 > 0 as libc::c_int as libc::c_ulong {
                            let fresh134 = (*(*LS).z).p;
                            (*(*LS).z).p = (*(*LS).z).p.offset(1);
                            *fresh134 as libc::c_int
                        } else {
                            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                        };
                    return TK_LE as libc::c_int
                }
            }
            62 => {
                let fresh135 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    if fresh135 > 0 as libc::c_int as libc::c_ulong {
                        let fresh136 = (*(*LS).z).p;
                        (*(*LS).z).p = (*(*LS).z).p.offset(1);
                        *fresh136 as libc::c_int
                    } else {
                        (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                    };
                if (*LS).current != '=' as i32 {
                    return '>' as i32
                } else {
                    let fresh137 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        if fresh137 > 0 as libc::c_int as libc::c_ulong {
                            let fresh138 = (*(*LS).z).p;
                            (*(*LS).z).p = (*(*LS).z).p.offset(1);
                            *fresh138 as libc::c_int
                        } else {
                            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                        };
                    return TK_GE as libc::c_int
                }
            }
            126 => {
                let fresh139 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    if fresh139 > 0 as libc::c_int as libc::c_ulong {
                        let fresh140 = (*(*LS).z).p;
                        (*(*LS).z).p = (*(*LS).z).p.offset(1);
                        *fresh140 as libc::c_int
                    } else {
                        (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                    };
                if (*LS).current != '=' as i32 {
                    return '~' as i32
                } else {
                    let fresh141 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        if fresh141 > 0 as libc::c_int as libc::c_ulong {
                            let fresh142 = (*(*LS).z).p;
                            (*(*LS).z).p = (*(*LS).z).p.offset(1);
                            *fresh142 as libc::c_int
                        } else {
                            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                        };
                    return TK_NE as libc::c_int
                }
            }
            34 | 39 => {
                read_string(LS, (*LS).current, seminfo);
                return TK_STRING as libc::c_int
            }
            46 => {
                let fresh143 = (*(*LS).z).n;
                (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                (*LS).current =
                    if fresh143 > 0 as libc::c_int as libc::c_ulong {
                        let fresh144 = (*(*LS).z).p;
                        (*(*LS).z).p = (*(*LS).z).p.offset(1);
                        *fresh144 as libc::c_int
                    } else {
                        (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                    };
                if (*LS).current == '.' as i32 {
                    let fresh145 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        if fresh145 > 0 as libc::c_int as libc::c_ulong {
                            let fresh146 = (*(*LS).z).p;
                            (*(*LS).z).p = (*(*LS).z).p.offset(1);
                            *fresh146 as libc::c_int
                        } else {
                            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                        };
                    if (*LS).current == '.' as i32 {
                        let fresh147 = (*(*LS).z).n;
                        (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                        (*LS).current =
                            if fresh147 > 0 as libc::c_int as libc::c_ulong {
                                let fresh148 = (*(*LS).z).p;
                                (*(*LS).z).p = (*(*LS).z).p.offset(1);
                                *fresh148 as libc::c_int
                            } else {
                                (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                            };
                        return TK_DOTS as libc::c_int
                        /* .. */
                        /* ... */
                    } else { return TK_CONCAT as libc::c_int }
                } else if *(*__ctype_b_loc()).offset((*LS).current as isize)
                              as libc::c_int &
                              _ISdigit as libc::c_int as libc::c_ushort as
                                  libc::c_int == 0 {
                    return '.' as i32
                } else {
                    read_number(LS, 1 as libc::c_int, seminfo);
                    return TK_NUMBER as libc::c_int
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                read_number(LS, 0 as libc::c_int, seminfo);
                return TK_NUMBER as libc::c_int
            }
            -1 => { return TK_EOS as libc::c_int }
            95 => { }
            _ => {
                if *(*__ctype_b_loc()).offset((*LS).current as isize) as
                       libc::c_int &
                       _ISalpha as libc::c_int as libc::c_ushort as
                           libc::c_int == 0 {
                    let mut c: libc::c_int = (*LS).current;
                    if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                           _IScntrl as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        luaX_invalidchar(LS, c);
                    }
                    let fresh149 = (*(*LS).z).n;
                    (*(*LS).z).n = (*(*LS).z).n.wrapping_sub(1);
                    (*LS).current =
                        if fresh149 > 0 as libc::c_int as libc::c_ulong {
                            let fresh150 = (*(*LS).z).p;
                            (*(*LS).z).p = (*(*LS).z).p.offset(1);
                            *fresh150 as libc::c_int
                        } else {
                            (*(*LS).z).filbuf.expect("non-null function pointer")((*LS).z)
                        };
                    return c
                }
            }
        }
        /* identifier or reserved word */
        let mut ts: *mut TString = luaS_new((*LS).L, readname(LS));
        if (*ts).marked >= 3 as libc::c_int {
            /* reserved word? */
            return (*ts).marked - 3 as libc::c_int + 257 as libc::c_int
        }
        (*seminfo).ts = ts;
        return TK_NAME as libc::c_int
    };
}
