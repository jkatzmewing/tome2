use ::libc;
/* USE_GTK2 */
/* OANGBAND */
/* GUMBAND */
/* ANGBAND300 */
/*
 * Some examples
 */
/* TOME */
/* Mogami's bigtile patch */
/* New/Open integrated into the game */
/* The old style reset_visuals() */
/* Requires V2.9.1 compatibility code */
/* Requires V2.9.3 compatibility code */
/*
 * Activate variant-specific features
 *
 * Angband 2.9.3 and close variants don't require any.
 *
 * Angband 2.9.4 alpha and later removed the short-lived
 * can_save flag, so please #define can_save TRUE, or remove
 * all the references to it. They also changed long-lived
 * z-virt macro names. Find C_FREE/C_KILL and replace them
 * with FREE/KILL, which takes one pointer parameter.
 *
 * [Z]-based variants (Gum and Cth, for example) usually need
 * ANG293_COMPAT, ANG291_COMPAT and ANG281_RESET_VISUALS.
 *
 * [O] needs ANG293_COMPAT and ZANG_SAVE_GAME.
 *
 * ZAngband has its own enhanced main-gtk.c as mentioned above, and
 * you *should* use it :-)
 *
 */
/*
 * Robert Ruehlmann wrote the original Gtk port. Since an initial work is
 * much harder than enhancements, his effort worth more credits than
 * others.
 *
 * Steven Fuerst implemented colour-depth independent X server support,
 * graphics, resizing and big screen support for ZAngband as well as
 * fast image rescaling that is included here.
 *
 * Uwe Siems wrote smooth tiles rescaling code (on by default).
 * Try this with 8x8 tiles. They *will* look different.
 *
 * "pelpel" wrote another colour-depth independent X support
 * using GdkRGB, added several hooks and callbacks for various
 * reasons, wrote no-backing store mode (off by default),
 * added GtkItemFactory based menu system, introduced
 * USE_GRAPHICS code bloat (^ ^;), added comments (I have
 * a strange habit of writing comments while I code...)
 * and reorganised the file a bit.
 */
/*
 * Copyright (c) 2000-2001 Robert Ruehlmann,
 * Steven Fuerst, Uwe Siems, "pelpel", et al.
 *
 * This software may be copied and distributed for educational, research,
 * and not for profit purposes provided that this copyright and statement
 * are included in all such copies.
 */
/* File: main-gtk.c */
