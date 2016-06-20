#pragma once

#include "h-basic.h"

/**
 * Size of allocation table for objects
 */
constexpr int ALLOCATION_MAX = 8;

/**
 * Object "kind" descriptor. Includes player knowledge.
 *
 * Only "aware" and "tried" are saved in the savefile
 */
struct object_kind
{
	const char *name = nullptr;              /* Name */
	char *text = nullptr;                    /* Text */

	byte tval = 0;                           /* Object type */
	byte sval = 0;                           /* Object sub type */

	s32b pval = 0;                           /* Object extra info */
	s32b pval2 = 0;                          /* Object extra info */

	s16b to_h = 0;                           /* Bonus to hit */
	s16b to_d = 0;                           /* Bonus to damage */
	s16b to_a = 0;                           /* Bonus to armor */

	s16b activate = 0;                       /* Activation number */

	s16b ac = 0;                             /* Base armor */

	byte dd = 0;                             /* Damage dice */
	byte ds = 0;                             /* Damage sides */

	s32b weight = 0;                         /* Weight */

	s32b cost = 0;                           /* Object "base cost" */

	u32b flags1 = 0;                         /* Flags, set 1 */
	u32b flags2 = 0;                         /* Flags, set 2 */
	u32b flags3 = 0;                         /* Flags, set 3 */
	u32b flags4 = 0;                         /* Flags, set 4 */
	u32b flags5 = 0;                         /* Flags, set 5 */

	u32b oflags1 = 0;                        /* Obvious Flags, set 1 */
	u32b oflags2 = 0;                        /* Obvious Flags, set 2 */
	u32b oflags3 = 0;                        /* Obvious Flags, set 3 */
	u32b oflags4 = 0;                        /* Obvious Flags, set 4 */
	u32b oflags5 = 0;                        /* Obvious Flags, set 5 */

	byte locale[ALLOCATION_MAX] = { 0 };     /* Allocation level(s) */
	byte chance[ALLOCATION_MAX] = { 0 };     /* Allocation chance(s) */

	byte level = 0;                          /* Level */


	byte d_attr = 0;                         /* Default object attribute */
	char d_char = 0;                         /* Default object character */


	byte x_attr = 0;                         /* Desired object attribute */
	char x_char = 0;                         /* Desired object character */


	byte flavor = 0;                         /* Special object flavor (or zero) */

	bool_ easy_know = 0;                     /* This object is always known (if aware) */

	bool_ aware = 0;                         /* The player is "aware" of the item's effects */

	bool_ tried = 0;                         /* The player has "tried" one of the items */

	u32b esp = 0;                            /* ESP flags */
	u32b oesp = 0;                           /* Obvious ESP flags */

	byte btval = 0;                          /* Become Object type */
	byte bsval = 0;                          /* Become Object sub type */
	bool_ artifact = 0;                      /* Is it a normal artifact(already generated) */

	s16b power = 0;                          /* Power granted(if any) */
};
