use ::libc;
pub type bool_ = libc::c_char;
/* File: help.c */
/* Purpose: ingame help */
/*
 * Actually this is now handled by lua,
 * I'll remove this file when I feel un-lazy
 */
/*
 * Copyright (c) 2001 DarkGod
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Driver for the context-sensitive help system
 */
#[no_mangle]
pub unsafe extern "C" fn ingame_help(mut enable: bool_) { }
