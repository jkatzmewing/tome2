#pragma once

#include "h-basic.h"
#include "player_race_ability_type.hpp"
#include "player_race_flag_set.hpp"
#include "skill_modifiers.hpp"
#include "skills_defs.hpp"

#include <array>

/**
 * Player class descriptor.
 */
struct player_spec
{
	const char *title = nullptr;                            /* Type of class spec */
	char *desc = nullptr;                                   /* Small desc of the class spec */

	struct skill_modifiers skill_modifiers;

	u32b skill_ideal[MAX_SKILLS] { };                       /* Ideal skill levels at level 50 */

	s16b obj_tval[5] { };
	s16b obj_sval[5] { };
	s16b obj_pval[5] { };
	s16b obj_dd[5] { };
	s16b obj_ds[5] { };
	s16b obj_num = 0;

	u32b gods = 0;

	player_race_flag_set flags;

	std::array<player_race_ability_type, 10> abilities;     /* Abilities to be gained by level(doesnt take prereqs in account) */
};
