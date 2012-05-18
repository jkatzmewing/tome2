-- handle the udun school

DRAIN = add_spell
{
	["name"] = 	"Drain",
	["school"] = 	{SCHOOL_UDUN, SCHOOL_MANA},
	["level"] = 	1,
	["mana"] = 	0,
	["mana_max"] = 	0,
	["fail"] = 	20,
	["spell"] = 	function() return udun_drain() end,
	["info"] = 	function() return udun_drain_info() end,
	["desc"] =	{
			"Drains the mana contained in wands, staves and rods to increase yours",
	}
}

GENOCIDE = add_spell
{
	["name"] = 	"Genocide",
	["school"] = 	{SCHOOL_UDUN, SCHOOL_NATURE},
	["level"] = 	25,
	["mana"] = 	50,
	["mana_max"] = 	50,
	["fail"] = 	90,
	["stick"] =
	{
			["charge"] =    { 2, 2 },
			[TV_STAFF] =
			{
				["rarity"] = 		85,
				["base_level"] =	{ 1, 1 },
				["max_level"] =		{ 5, 15 },
			},
	},
	["spell"] = 	function() return udun_genocide() end,
	["info"] = 	function() return udun_genocide_info() end,
	["desc"] =	{
			"Genocides all monsters of a race on the level",
			"At level 10 it can genocide all monsters near you"
	}
}

WRAITHFORM = add_spell
{
	["name"] = 	"Wraithform",
	["school"] = 	{SCHOOL_UDUN, SCHOOL_CONVEYANCE},
	["level"] = 	30,
	["mana"] = 	20,
	["mana_max"] = 	40,
	["fail"] = 	95,
	["inertia"] = 	{ 4, 30 },
	["spell"] = 	function() return udun_wraithform() end,
	["info"] = 	function() return udun_wraithform_info() end,
	["desc"] =	{
			"Turns you into an immaterial being",
	}
}

FLAMEOFUDUN = add_spell
{
	["name"] = 	"Flame of Udun",
	["school"] = 	{SCHOOL_UDUN, SCHOOL_FIRE},
	["level"] = 	35,
	["mana"] = 	70,
	["mana_max"] = 	100,
	["fail"] = 	95,
	["inertia"] = 	{ 7, 15 },
	["spell"] = 	function() return udun_flame_of_udun() end,
	["info"] = 	function() return udun_flame_of_udun_info() end,
	["desc"] =	{
			"Turns you into a powerful Balrog",
	}
}
