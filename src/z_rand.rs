use ::libc;
pub type bool_ = libc::c_char;
pub type s16b = libc::c_short;
pub type u16b = libc::c_ushort;
pub type s32b = libc::c_int;
pub type u32b = libc::c_uint;
/*
 * Use the "simple" LCRNG
 */
#[no_mangle]
pub static mut Rand_quick: bool_ = 1 as libc::c_int as bool_;
/*
 * Current "value" of the "simple" RNG
 */
#[no_mangle]
pub static mut Rand_value: u32b = 0;
/*
 * Current "index" for the "complex" RNG
 */
#[no_mangle]
pub static mut Rand_place: u16b = 0;
/*
 * Current "state" table for the "complex" RNG
 */
#[no_mangle]
pub static mut Rand_state: [u32b; 63] = [0; 63];
/*
 * Initialize the "complex" RNG using a new seed
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_state_init(mut seed: u32b) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* Seed the table */
    Rand_state[0 as libc::c_int as usize] = seed;
    /* Propagate the seed */
    i = 1 as libc::c_int;
    while i < 63 as libc::c_int {
        Rand_state[i as usize] =
            Rand_state[(i - 1 as libc::c_int) as
                           usize].wrapping_mul(1103515245 as libc::c_int as
                                                   libc::c_uint).wrapping_add(12345
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint);
        i += 1
    }
    /* Cycle the table ten times per degree */
    i = 0 as libc::c_int;
    while i < 63 as libc::c_int * 10 as libc::c_int {
        /* Acquire the next index */
        j = Rand_place as libc::c_int + 1 as libc::c_int;
        if j == 63 as libc::c_int { j = 0 as libc::c_int }
        /* Update the table, extract an entry */
        Rand_state[j as usize] =
            (Rand_state[j as usize] as
                 libc::c_uint).wrapping_add(Rand_state[Rand_place as usize])
                as u32b as u32b;
        /* Advance the index */
        Rand_place = j as u16b;
        i += 1
    };
}
/*
 * Extract a "random" number from 0 to m-1, via "modulus"
 *
 * Note that "m" should probably be less than 500000, or the
 * results may be rather biased towards low values.
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_mod(mut m: s32b) -> s32b {
    let mut j: libc::c_int = 0;
    let mut r: u32b = 0;
    /* Hack -- simple case */
    if m <= 1 as libc::c_int { return 0 as libc::c_int }
    /* Use the "simple" RNG */
    if Rand_quick != 0 {
        /* Cycle the generator */
        Rand_value =
            Rand_value.wrapping_mul(1103515245 as libc::c_int as
                                        libc::c_uint).wrapping_add(12345 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint);
        r = Rand_value;
        /* Mutate a 28-bit "random" number */
        r = (r >> 4 as libc::c_int).wrapping_rem(m as libc::c_uint)
    } else {
        /* Use the "complex" RNG */
        /* Acquire the next index */
        j = Rand_place as libc::c_int + 1 as libc::c_int;
        if j == 63 as libc::c_int { j = 0 as libc::c_int }
        Rand_state[j as usize] =
            (Rand_state[j as usize] as
                 libc::c_uint).wrapping_add(Rand_state[Rand_place as usize])
                as u32b as u32b;
        r = Rand_state[j as usize];
        Rand_place = j as u16b;
        r = (r >> 4 as libc::c_int).wrapping_rem(m as libc::c_uint)
    }
    /* Update the table, extract an entry */
    /* Advance the index */
    /* Extract a "random" number */
    /* Use the value */
    return r as s32b;
}
/*
 * Extract a "random" number from 0 to m-1, via "division"
 *
 * This method selects "random" 28-bit numbers, and then uses
 * division to drop those numbers into "m" different partitions,
 * plus a small non-partition to reduce bias, taking as the final
 * value the first "good" partition that a number falls into.
 *
 * This method has no bias, and is much less affected by patterns
 * in the "low" bits of the underlying RNG's.
 */
#[no_mangle]
pub unsafe extern "C" fn Rand_div(mut m: s32b) -> s32b {
    let mut r: u32b = 0;
    let mut n: u32b = 0;
    /* Hack -- simple case */
    if m <= 1 as libc::c_int { return 0 as libc::c_int }
    /* Partition size */
    n = (0x10000000 as libc::c_int / m) as u32b;
    /* Use a simple RNG */
    if Rand_quick != 0 {
        loop 
             /* Wait for it */
             /* Cycle the generator */
             {
            Rand_value =
                Rand_value.wrapping_mul(1103515245 as libc::c_int as
                                            libc::c_uint).wrapping_add(12345
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint);
            r = Rand_value;
            /* Mutate a 28-bit "random" number */
            r =
                (r >> 4 as libc::c_int &
                     0xfffffff as libc::c_int as
                         libc::c_uint).wrapping_div(n);
            /* Done */
            if r < m as u32b { break ; }
        }
    } else {
        /* Use a complex RNG */
        loop 
             /* Wait for it */
             {
            let mut j: libc::c_int = 0;
            /* Acquire the next index */
            j = Rand_place as libc::c_int + 1 as libc::c_int;
            if j == 63 as libc::c_int { j = 0 as libc::c_int }
            /* Update the table, extract an entry */
            Rand_state[j as usize] =
                (Rand_state[j as usize] as
                     libc::c_uint).wrapping_add(Rand_state[Rand_place as
                                                               usize]) as u32b
                    as u32b;
            r = Rand_state[j as usize];
            /* Hack -- extract a 28-bit "random" number */
            r =
                (r >> 4 as libc::c_int &
                     0xfffffff as libc::c_int as
                         libc::c_uint).wrapping_div(n);
            /* Advance the index */
            Rand_place = j as u16b;
            /* Done */
            if r < m as u32b { break ; }
        }
    }
    /* Use the value */
    return r as s32b;
}
/*
 * The normal distribution table for the "randnor()" function (below)
 */
static mut randnor_table: [s16b; 256] =
    [206 as libc::c_int as s16b, 613 as libc::c_int as s16b,
     1022 as libc::c_int as s16b, 1430 as libc::c_int as s16b,
     1838 as libc::c_int as s16b, 2245 as libc::c_int as s16b,
     2652 as libc::c_int as s16b, 3058 as libc::c_int as s16b,
     3463 as libc::c_int as s16b, 3867 as libc::c_int as s16b,
     4271 as libc::c_int as s16b, 4673 as libc::c_int as s16b,
     5075 as libc::c_int as s16b, 5475 as libc::c_int as s16b,
     5874 as libc::c_int as s16b, 6271 as libc::c_int as s16b,
     6667 as libc::c_int as s16b, 7061 as libc::c_int as s16b,
     7454 as libc::c_int as s16b, 7845 as libc::c_int as s16b,
     8234 as libc::c_int as s16b, 8621 as libc::c_int as s16b,
     9006 as libc::c_int as s16b, 9389 as libc::c_int as s16b,
     9770 as libc::c_int as s16b, 10148 as libc::c_int as s16b,
     10524 as libc::c_int as s16b, 10898 as libc::c_int as s16b,
     11269 as libc::c_int as s16b, 11638 as libc::c_int as s16b,
     12004 as libc::c_int as s16b, 12367 as libc::c_int as s16b,
     12727 as libc::c_int as s16b, 13085 as libc::c_int as s16b,
     13440 as libc::c_int as s16b, 13792 as libc::c_int as s16b,
     14140 as libc::c_int as s16b, 14486 as libc::c_int as s16b,
     14828 as libc::c_int as s16b, 15168 as libc::c_int as s16b,
     15504 as libc::c_int as s16b, 15836 as libc::c_int as s16b,
     16166 as libc::c_int as s16b, 16492 as libc::c_int as s16b,
     16814 as libc::c_int as s16b, 17133 as libc::c_int as s16b,
     17449 as libc::c_int as s16b, 17761 as libc::c_int as s16b,
     18069 as libc::c_int as s16b, 18374 as libc::c_int as s16b,
     18675 as libc::c_int as s16b, 18972 as libc::c_int as s16b,
     19266 as libc::c_int as s16b, 19556 as libc::c_int as s16b,
     19842 as libc::c_int as s16b, 20124 as libc::c_int as s16b,
     20403 as libc::c_int as s16b, 20678 as libc::c_int as s16b,
     20949 as libc::c_int as s16b, 21216 as libc::c_int as s16b,
     21479 as libc::c_int as s16b, 21738 as libc::c_int as s16b,
     21994 as libc::c_int as s16b, 22245 as libc::c_int as s16b,
     22493 as libc::c_int as s16b, 22737 as libc::c_int as s16b,
     22977 as libc::c_int as s16b, 23213 as libc::c_int as s16b,
     23446 as libc::c_int as s16b, 23674 as libc::c_int as s16b,
     23899 as libc::c_int as s16b, 24120 as libc::c_int as s16b,
     24336 as libc::c_int as s16b, 24550 as libc::c_int as s16b,
     24759 as libc::c_int as s16b, 24965 as libc::c_int as s16b,
     25166 as libc::c_int as s16b, 25365 as libc::c_int as s16b,
     25559 as libc::c_int as s16b, 25750 as libc::c_int as s16b,
     25937 as libc::c_int as s16b, 26120 as libc::c_int as s16b,
     26300 as libc::c_int as s16b, 26476 as libc::c_int as s16b,
     26649 as libc::c_int as s16b, 26818 as libc::c_int as s16b,
     26983 as libc::c_int as s16b, 27146 as libc::c_int as s16b,
     27304 as libc::c_int as s16b, 27460 as libc::c_int as s16b,
     27612 as libc::c_int as s16b, 27760 as libc::c_int as s16b,
     27906 as libc::c_int as s16b, 28048 as libc::c_int as s16b,
     28187 as libc::c_int as s16b, 28323 as libc::c_int as s16b,
     28455 as libc::c_int as s16b, 28585 as libc::c_int as s16b,
     28711 as libc::c_int as s16b, 28835 as libc::c_int as s16b,
     28955 as libc::c_int as s16b, 29073 as libc::c_int as s16b,
     29188 as libc::c_int as s16b, 29299 as libc::c_int as s16b,
     29409 as libc::c_int as s16b, 29515 as libc::c_int as s16b,
     29619 as libc::c_int as s16b, 29720 as libc::c_int as s16b,
     29818 as libc::c_int as s16b, 29914 as libc::c_int as s16b,
     30007 as libc::c_int as s16b, 30098 as libc::c_int as s16b,
     30186 as libc::c_int as s16b, 30272 as libc::c_int as s16b,
     30356 as libc::c_int as s16b, 30437 as libc::c_int as s16b,
     30516 as libc::c_int as s16b, 30593 as libc::c_int as s16b,
     30668 as libc::c_int as s16b, 30740 as libc::c_int as s16b,
     30810 as libc::c_int as s16b, 30879 as libc::c_int as s16b,
     30945 as libc::c_int as s16b, 31010 as libc::c_int as s16b,
     31072 as libc::c_int as s16b, 31133 as libc::c_int as s16b,
     31192 as libc::c_int as s16b, 31249 as libc::c_int as s16b,
     31304 as libc::c_int as s16b, 31358 as libc::c_int as s16b,
     31410 as libc::c_int as s16b, 31460 as libc::c_int as s16b,
     31509 as libc::c_int as s16b, 31556 as libc::c_int as s16b,
     31601 as libc::c_int as s16b, 31646 as libc::c_int as s16b,
     31688 as libc::c_int as s16b, 31730 as libc::c_int as s16b,
     31770 as libc::c_int as s16b, 31808 as libc::c_int as s16b,
     31846 as libc::c_int as s16b, 31882 as libc::c_int as s16b,
     31917 as libc::c_int as s16b, 31950 as libc::c_int as s16b,
     31983 as libc::c_int as s16b, 32014 as libc::c_int as s16b,
     32044 as libc::c_int as s16b, 32074 as libc::c_int as s16b,
     32102 as libc::c_int as s16b, 32129 as libc::c_int as s16b,
     32155 as libc::c_int as s16b, 32180 as libc::c_int as s16b,
     32205 as libc::c_int as s16b, 32228 as libc::c_int as s16b,
     32251 as libc::c_int as s16b, 32273 as libc::c_int as s16b,
     32294 as libc::c_int as s16b, 32314 as libc::c_int as s16b,
     32333 as libc::c_int as s16b, 32352 as libc::c_int as s16b,
     32370 as libc::c_int as s16b, 32387 as libc::c_int as s16b,
     32404 as libc::c_int as s16b, 32420 as libc::c_int as s16b,
     32435 as libc::c_int as s16b, 32450 as libc::c_int as s16b,
     32464 as libc::c_int as s16b, 32477 as libc::c_int as s16b,
     32490 as libc::c_int as s16b, 32503 as libc::c_int as s16b,
     32515 as libc::c_int as s16b, 32526 as libc::c_int as s16b,
     32537 as libc::c_int as s16b, 32548 as libc::c_int as s16b,
     32558 as libc::c_int as s16b, 32568 as libc::c_int as s16b,
     32577 as libc::c_int as s16b, 32586 as libc::c_int as s16b,
     32595 as libc::c_int as s16b, 32603 as libc::c_int as s16b,
     32611 as libc::c_int as s16b, 32618 as libc::c_int as s16b,
     32625 as libc::c_int as s16b, 32632 as libc::c_int as s16b,
     32639 as libc::c_int as s16b, 32645 as libc::c_int as s16b,
     32651 as libc::c_int as s16b, 32657 as libc::c_int as s16b,
     32662 as libc::c_int as s16b, 32667 as libc::c_int as s16b,
     32672 as libc::c_int as s16b, 32677 as libc::c_int as s16b,
     32682 as libc::c_int as s16b, 32686 as libc::c_int as s16b,
     32690 as libc::c_int as s16b, 32694 as libc::c_int as s16b,
     32698 as libc::c_int as s16b, 32702 as libc::c_int as s16b,
     32705 as libc::c_int as s16b, 32708 as libc::c_int as s16b,
     32711 as libc::c_int as s16b, 32714 as libc::c_int as s16b,
     32717 as libc::c_int as s16b, 32720 as libc::c_int as s16b,
     32722 as libc::c_int as s16b, 32725 as libc::c_int as s16b,
     32727 as libc::c_int as s16b, 32729 as libc::c_int as s16b,
     32731 as libc::c_int as s16b, 32733 as libc::c_int as s16b,
     32735 as libc::c_int as s16b, 32737 as libc::c_int as s16b,
     32739 as libc::c_int as s16b, 32740 as libc::c_int as s16b,
     32742 as libc::c_int as s16b, 32743 as libc::c_int as s16b,
     32745 as libc::c_int as s16b, 32746 as libc::c_int as s16b,
     32747 as libc::c_int as s16b, 32748 as libc::c_int as s16b,
     32749 as libc::c_int as s16b, 32750 as libc::c_int as s16b,
     32751 as libc::c_int as s16b, 32752 as libc::c_int as s16b,
     32753 as libc::c_int as s16b, 32754 as libc::c_int as s16b,
     32755 as libc::c_int as s16b, 32756 as libc::c_int as s16b,
     32757 as libc::c_int as s16b, 32757 as libc::c_int as s16b,
     32758 as libc::c_int as s16b, 32758 as libc::c_int as s16b,
     32759 as libc::c_int as s16b, 32760 as libc::c_int as s16b,
     32760 as libc::c_int as s16b, 32761 as libc::c_int as s16b,
     32761 as libc::c_int as s16b, 32761 as libc::c_int as s16b,
     32762 as libc::c_int as s16b, 32762 as libc::c_int as s16b,
     32763 as libc::c_int as s16b, 32763 as libc::c_int as s16b,
     32763 as libc::c_int as s16b, 32764 as libc::c_int as s16b,
     32764 as libc::c_int as s16b, 32764 as libc::c_int as s16b,
     32764 as libc::c_int as s16b, 32765 as libc::c_int as s16b,
     32765 as libc::c_int as s16b, 32765 as libc::c_int as s16b,
     32765 as libc::c_int as s16b, 32766 as libc::c_int as s16b,
     32766 as libc::c_int as s16b, 32766 as libc::c_int as s16b,
     32766 as libc::c_int as s16b, 32767 as libc::c_int as s16b];
/*
 * Generate a random integer number of NORMAL distribution
 *
 * The table above is used to generate a psuedo-normal distribution,
 * in a manner which is much faster than calling a transcendental
 * function to calculate a true normal distribution.
 *
 * Basically, entry 64*N in the table above represents the number of
 * times out of 32767 that a random variable with normal distribution
 * will fall within N standard deviations of the mean.  That is, about
 * 68 percent of the time for N=1 and 95 percent of the time for N=2.
 *
 * The table above contains a "faked" final entry which allows us to
 * pretend that all values in a normal distribution are strictly less
 * than four standard deviations away from the mean.  This results in
 * "conservative" distribution of approximately 1/32768 values.
 *
 * Note that the binary search takes up to 16 quick iterations.
 */
#[no_mangle]
pub unsafe extern "C" fn randnor(mut mean: libc::c_int,
                                 mut stand: libc::c_int) -> s16b {
    let mut tmp: s16b = 0;
    let mut offset: s16b = 0;
    let mut low: s16b = 0 as libc::c_int as s16b;
    let mut high: s16b = 256 as libc::c_int as s16b;
    /* Paranoia */
    if stand < 1 as libc::c_int { return mean as s16b }
    /* Roll for probability */
    tmp = Rand_div(32768 as libc::c_int) as s16b;
    /* Binary Search */
    while (low as libc::c_int) < high as libc::c_int {
        let mut mid: libc::c_int =
            low as libc::c_int + high as libc::c_int >> 1 as libc::c_int;
        /* Move right if forced */
        if (randnor_table[mid as usize] as libc::c_int) < tmp as libc::c_int {
            low = (mid + 1 as libc::c_int) as s16b
        } else {
            /* Move left otherwise */
            high = mid as s16b
        }
    }
    /* Convert the index into an offset */
    offset =
        (stand as libc::c_long * low as libc::c_long /
             64 as libc::c_int as libc::c_long) as s16b;
    /* One half should be negative */
    if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        return (mean - offset as libc::c_int) as s16b
    }
    /* One half should be positive */
    return (mean + offset as libc::c_int) as s16b;
}
/*
 * Generates damage for "2d6" style dice rolls
 */
#[no_mangle]
pub unsafe extern "C" fn damroll(mut num: s16b, mut sides: s16b) -> s32b {
    let mut i: libc::c_int = 0;
    let mut sum: s32b = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num as libc::c_int {
        sum += Rand_div(sides as s32b) + 1 as libc::c_int;
        i += 1
    }
    return sum;
}
/*
 * Same as above, but always maximal
 */
#[no_mangle]
pub unsafe extern "C" fn maxroll(mut num: s16b, mut sides: s16b) -> s32b {
    return num as libc::c_int * sides as libc::c_int;
}
