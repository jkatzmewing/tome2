use ::libc;
/* USE_XAW */
/*
 * This file helps Angband work with UNIX/X11 computers.
 *
 * To use this file, compile with "USE_XAW" defined, and link against all
 * the various "X11" libraries which may be needed.
 *
 * See also "main-x11.c".
 *
 * The Angband widget is not as self-contained as it really should be.
 * Originally everything was output to a Pixmap which was later copied
 * to the screen when necessary.  The idea was abandoned since Pixmaps
 * create big performance problems for some really old X terminals (such
 * as 3/50's running Xkernel).
 *
 * Initial framework (and some code) by Ben Harrison (benh@phial.com).
 *
 * Most of this file is by Torbjorn Lindgren (tl@cd.chalmers.se).
 *
 * Major modifications by Ben Harrison (benh@phial.com).
 */
/*
 * Copyright (c) 1997 Ben Harrison, Torbjorn Lindgren, and others
 *
 * This software may be copied and distributed for educational, research,
 * and not for profit purposes provided that this copyright and statement
 * are included in all such copies.
 */
/* File: main-xaw.c */
