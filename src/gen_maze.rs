use ::libc;
extern "C" {
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn place_floor(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn get_branch() -> libc::c_int;
    #[no_mangle]
    fn new_player_spot(branch: libc::c_int) -> bool_;
}
pub type vptr = *mut libc::c_void;
pub type bool_ = libc::c_char;
pub type huge_hack = libc::c_ulong;
pub type s16b = libc::c_short;
pub type s32b = libc::c_int;
/*
 * Maze dungeon generator
 */
/*
 * Copyright (c) 2003 DarkGod. And somebody who posted the algorith on
 * rec.games.roguelike.development. I can't remember teh name :( please mail me
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * If we wasted static memory for this, it would look like:
 *
 * static char maze[(MAX_HGT / 2) + 2][(MAX_WID / 2) + 2];
 */
pub type maze_row = [libc::c_schar; 101];
#[no_mangle]
pub unsafe extern "C" fn dig(mut maze: *mut maze_row, mut y: libc::c_int,
                             mut x: libc::c_int, mut d: libc::c_int) {
    let mut k: libc::c_int = 0;
    let mut dy: libc::c_int = 0 as libc::c_int;
    let mut dx: libc::c_int = 0 as libc::c_int;
    /*
	 * first, open the wall of the new cell
	 * in the direction we come from.
	 */
    match d {
        0 => {
            let ref mut fresh0 = (*maze.offset(y as isize))[x as usize];
            *fresh0 =
                (*fresh0 as libc::c_int | 4 as libc::c_int) as libc::c_schar
        }
        1 => {
            let ref mut fresh1 = (*maze.offset(y as isize))[x as usize];
            *fresh1 =
                (*fresh1 as libc::c_int | 8 as libc::c_int) as libc::c_schar
        }
        2 => {
            let ref mut fresh2 = (*maze.offset(y as isize))[x as usize];
            *fresh2 =
                (*fresh2 as libc::c_int | 1 as libc::c_int) as libc::c_schar
        }
        3 => {
            let ref mut fresh3 = (*maze.offset(y as isize))[x as usize];
            *fresh3 =
                (*fresh3 as libc::c_int | 2 as libc::c_int) as libc::c_schar
        }
        _ => { }
    }
    /*
	 * try to chage direction, here 50% times.
	 * with smaller values (say 25%) the maze
	 * is made of long straight corridors. with
	 * greaters values (say 75%) the maze is
	 * very "turny".
	 */
    if 1 as libc::c_int +
           Rand_div(1 as libc::c_int + 100 as libc::c_int - 1 as libc::c_int)
           < 50 as libc::c_int {
        d =
            0 as libc::c_int +
                Rand_div(1 as libc::c_int + 3 as libc::c_int -
                             0 as libc::c_int)
    }
    k = 1 as libc::c_int;
    while k <= 4 as libc::c_int {
        match d {
            0 => { dy = 0 as libc::c_int; dx = 1 as libc::c_int }
            1 => { dy = -(1 as libc::c_int); dx = 0 as libc::c_int }
            2 => { dy = 0 as libc::c_int; dx = -(1 as libc::c_int) }
            3 => { dy = 1 as libc::c_int; dx = 0 as libc::c_int }
            _ => { }
        }
        if (*maze.offset((y + dy) as isize))[(x + dx) as usize] as libc::c_int
               == 0 as libc::c_int {
            /*
			 * now, open the wall of the new cell
			 * in the direction we go to.
			 */
            match d {
                0 => {
                    let ref mut fresh4 =
                        (*maze.offset(y as isize))[x as usize];
                    *fresh4 =
                        (*fresh4 as libc::c_int | 1 as libc::c_int) as
                            libc::c_schar
                }
                1 => {
                    let ref mut fresh5 =
                        (*maze.offset(y as isize))[x as usize];
                    *fresh5 =
                        (*fresh5 as libc::c_int | 2 as libc::c_int) as
                            libc::c_schar
                }
                2 => {
                    let ref mut fresh6 =
                        (*maze.offset(y as isize))[x as usize];
                    *fresh6 =
                        (*fresh6 as libc::c_int | 4 as libc::c_int) as
                            libc::c_schar
                }
                3 => {
                    let ref mut fresh7 =
                        (*maze.offset(y as isize))[x as usize];
                    *fresh7 =
                        (*fresh7 as libc::c_int | 8 as libc::c_int) as
                            libc::c_schar
                }
                _ => { }
            }
            dig(maze, y + dy, x + dx, d);
        }
        d = (d + 1 as libc::c_int) % 4 as libc::c_int;
        k += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn level_generate_maze() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut dy: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut dx: libc::c_int = 0 as libc::c_int;
    let mut m_1: libc::c_int = 0 as libc::c_int;
    let mut m_2: libc::c_int = 0 as libc::c_int;
    let mut maze: *mut maze_row = 0 as *mut maze_row;
    /* Allocate temporary memory */
    maze =
        memset(ralloc(((66 as libc::c_int / 2 as libc::c_int +
                            2 as libc::c_int) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<maze_row>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((66 as libc::c_int / 2 as libc::c_int + 2 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<maze_row>()
                                                    as libc::c_ulong)) as
            *mut maze_row;
    /*
	 * the empty maze is:
	 *
	 * -1 -1 ... -1 -1
	 * -1  0      0 -1
	 *  .            .
	 *  .            .
	 * -1  0      0 -1
	 * -1 -1 ... -1 -1
	 *
	 *  -1 are so-called "sentinel value".
	 *   0 are empty cells.
	 *
	 *  walls are not represented, only cells.
	 *  at the end of the algorithm each cell
	 *  contains a value that is bit mask
	 *  representing surrounding walls:
	 *
	 *         bit #1
	 *
	 *        +------+
	 *        |      |
	 * bit #2 |      | bit #0
	 *        |      |
	 *        +------+
	 *
	 *         bit #3
	 *
	 * d is the direction you are digging
	 * to. d value is the bit number:
	 * d=0 --> go east
	 * d=1 --> go north
	 * etc
	 *
	 * you need only 4 bits per cell.
	 * this gives you a very compact
	 * maze representation.
	 *
	 */
    j = 0 as libc::c_int;
    while j <= cur_hgt as libc::c_int / 2 as libc::c_int + 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i <=
                  cur_wid as libc::c_int / 2 as libc::c_int + 1 as libc::c_int
              {
            (*maze.offset(j as isize))[i as usize] =
                -(1 as libc::c_int) as libc::c_schar;
            i += 1
        }
        j += 1
    }
    j = 1 as libc::c_int;
    while j <= cur_hgt as libc::c_int / 2 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= cur_wid as libc::c_int / 2 as libc::c_int {
            (*maze.offset(j as isize))[i as usize] =
                0 as libc::c_int as libc::c_schar;
            i += 1
        }
        j += 1
    }
    y =
        1 as libc::c_int +
            Rand_div(1 as libc::c_int +
                         cur_hgt as libc::c_int / 2 as libc::c_int -
                         1 as libc::c_int);
    x =
        1 as libc::c_int +
            Rand_div(1 as libc::c_int +
                         cur_wid as libc::c_int / 2 as libc::c_int -
                         1 as libc::c_int);
    d =
        0 as libc::c_int +
            Rand_div(1 as libc::c_int + 3 as libc::c_int - 0 as libc::c_int);
    dig(maze, y, x, d);
    (*maze.offset(y as isize))[x as usize] =
        0 as libc::c_int as libc::c_schar;
    d = 0 as libc::c_int;
    while d <= 3 as libc::c_int {
        match d {
            0 => {
                dy = 0 as libc::c_int;
                dx = 1 as libc::c_int;
                m_1 = 1 as libc::c_int;
                m_2 = 4 as libc::c_int
            }
            1 => {
                dy = -(1 as libc::c_int);
                dx = 0 as libc::c_int;
                m_1 = 2 as libc::c_int;
                m_2 = 8 as libc::c_int
            }
            2 => {
                dy = 0 as libc::c_int;
                dx = -(1 as libc::c_int);
                m_1 = 4 as libc::c_int;
                m_2 = 1 as libc::c_int
            }
            3 => {
                dy = 1 as libc::c_int;
                dx = 0 as libc::c_int;
                m_1 = 8 as libc::c_int;
                m_2 = 2 as libc::c_int
            }
            _ => { }
        }
        if (*maze.offset((y + dy) as isize))[(x + dx) as usize] as libc::c_int
               != -(1 as libc::c_int) &&
               (*maze.offset((y + dy) as isize))[(x + dx) as usize] as
                   libc::c_int & m_2 != 0 as libc::c_int {
            let ref mut fresh8 = (*maze.offset(y as isize))[x as usize];
            *fresh8 = (*fresh8 as libc::c_int | m_1) as libc::c_schar
        }
        d += 1
    }
    /* Translate the maze bit array into a real dungeon map -- DG */
    j = 1 as libc::c_int;
    while j <= cur_hgt as libc::c_int / 2 as libc::c_int - 2 as libc::c_int {
        i = 1 as libc::c_int;
        while i <=
                  cur_wid as libc::c_int / 2 as libc::c_int - 2 as libc::c_int
              {
            if (*maze.offset(j as isize))[i as usize] != 0 {
                place_floor(j * 2 as libc::c_int, i * 2 as libc::c_int);
            }
            if (*maze.offset(j as isize))[i as usize] as libc::c_int &
                   1 as libc::c_int != 0 {
                place_floor(j * 2 as libc::c_int,
                            i * 2 as libc::c_int + 1 as libc::c_int);
            }
            if (*maze.offset(j as isize))[i as usize] as libc::c_int &
                   8 as libc::c_int != 0 {
                place_floor(j * 2 as libc::c_int + 1 as libc::c_int,
                            i * 2 as libc::c_int);
            }
            i += 1
        }
        j += 1
    }
    /* Free temporary memory */
    rnfree(maze as vptr,
           ((66 as libc::c_int / 2 as libc::c_int + 2 as libc::c_int) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<maze_row>()
                                                as libc::c_ulong));
    /* Determine the character location */
    if new_player_spot(get_branch()) == 0 { return 0 as libc::c_int as bool_ }
    return 1 as libc::c_int as bool_;
}
