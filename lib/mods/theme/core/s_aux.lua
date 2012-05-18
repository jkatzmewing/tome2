-- Functions to help with spells, do not touch

__schools = {}
__schools_num = 0

__tmp_spells = {}
__tmp_spells_num = 0

function add_school(s)
	__schools[__schools_num] = s
	
	__schools_num = __schools_num + 1
	return (__schools_num - 1)
end

function finish_school(i)
	local s
	
	s = __schools[i]
	assert(s.name, "No school name!")
	assert(s.skill, "No school skill!")

	-- Need hooks?
	if s.hooks then
		add_hooks(s.hooks)
	end

	new_school(i, s.name, s.skill)
end

function add_spell(s)
	__tmp_spells[__tmp_spells_num] = s
	
	__tmp_spells_num = __tmp_spells_num + 1
	return (__tmp_spells_num - 1)
end

function finish_spell(must_i)
	local i, s
	
	s = __tmp_spells[must_i]
	assert(s.name, "No spell name!")
	assert(s.school, "No spell school!")
	assert(s.level, "No spell level!")
	assert(s.mana, "No spell mana!")
	if not s.mana_max then s.mana_max = s.mana end
	assert(s.fail, "No spell failure rate!")
	assert(s.spell, "No spell function!")
	if not s.info then s.info = function() return "" end end
	assert(s.desc, "No spell desc!")
	if not s.random then s.random = SKILL_MAGIC end
	if s.lasting then
		assert(type(s.lasting) == "function", "Spell lasting is not function")
	end
	if s.stick then
		local k, e
		for k, e in s.stick do
			if type(k) == "table" then
				assert(e.base_level, "Arg no stick base level")
				assert(e.max_level, "Arg no stick max level")
			end
		end
	end

	i = new_spell(must_i, s.name)
	assert(i == must_i, "ACK ! i != must_i ! please contact the maintainer")
	if type(s.school) == "number" then __spell_school[i] = {s.school}
	else __spell_school[i] = s.school end
	spell(i).mana = s.mana
	spell(i).mana_max = s.mana_max
	spell(i).fail = s.fail
	spell(i).skill_level = s.level
	__spell_spell[i] = s.spell
	__spell_info[i] = s.info
	__spell_desc[i] = s.desc
	return i
end

-- Creates the school books array
__spell_spell = {}
__spell_info = {}
__spell_desc = {}
__spell_school = {}

-- Find a spell by name
function find_spell(name)
	local i

	i = 0
	while (i < __tmp_spells_num) do
		if __tmp_spells[i].name == name then return i end
		i = i + 1
	end
	return -1
end

-- Find if the school is under the influence of a god, returns nil or the level
function get_god_level(sch)
	if __schools[sch].gods[player.pgod] then
		return (s_info[__schools[sch].gods[player.pgod].skill + 1].value * __schools[sch].gods[player.pgod].mul) / __schools[sch].gods[player.pgod].div
	else
		return nil
	end
end

-- Change this fct if I want to switch to learnable spells
function get_level_school(s, max, min)
	local lvl, sch, index, num, bonus
	local allow_spell_power = TRUE

	lvl = 0
	num = 0
	bonus = 0

	-- No max specified ? assume 50
	if not max then
		max = 50
	end
	if not min then
		min = 1
	end

	-- Do we pass tests?
	if __tmp_spells[s].depend then
		if __tmp_spells[s].depend() ~= TRUE then
			return min, "n/a"
		end
	end

	for index, sch in __spell_school[s] do
		local r, s, p, ok = 0, 0, 0, 0

		-- Does it require we worship a specific god?
		if __schools[sch].god then
			if __schools[sch].god ~= player.pgod then
				if min then return min, "n/a"
				else return 1, "n/a" end
			end
		end

		-- Take the basic skill value
		r = s_info[(school(sch).skill) + 1].value

		-- Do we pass tests?
		if __schools[sch].depend then
			if __schools[sch].depend() ~= TRUE then
				return min, "n/a"
			end
		end

		-- Are we under sorcery effect ?
		if __schools[sch].sorcery then
			s = s_info[SKILL_SORCERY + 1].value
		end

		-- Are we affected by spell power ?
		-- All teh schools must allow it for it to work
		if not __schools[sch].spell_power then
			allow_spell_power = nil
		end

		-- Are we under a god effect ?
		if __schools[sch].gods then
			p = get_god_level(sch)
			if not p then p = 0 end
		end
		
		-- Find the higher
		ok = r
		if ok < s then ok = s end
		if ok < p then ok = p end

		-- Do we need to add a special bonus ?
		if __schools[sch].bonus_level then
			bonus = bonus + (__schools[sch].bonus_level() * (SKILL_STEP / 10))
		end

		-- All schools must be non zero to be able to use it
		if ok == 0 then return min, "n/a" end

		-- Apply it
		lvl = lvl + ok
		num = num + 1
	end

	-- Add the Spellpower skill as a bonus
	if allow_spell_power then
		bonus = bonus + (get_skill_scale(SKILL_SPELL, 20) * (SKILL_STEP / 10))
	end

	-- Add bonus from objects
	bonus = bonus + (player.to_s * (SKILL_STEP / 10))

	-- / 10 because otherwise we can overflow a s32b and we can use a u32b because the value can be negative
	-- The loss of information should be negligible since 1 skill = 1000 internally
	lvl = (lvl / num) / 10
	lvl = lua_get_level(s, lvl, max, min, bonus)

	return lvl, nil
end

-- The real get_level, works for schooled magic and for innate powers
function get_level(s, max, min)
   if not max then max = 50 end
   if not min then min = 1  end
   return %get_level(s, max, min)
end

-- Can we cast the spell ?
function is_ok_spell(s, obj)
	if get_level(s, 50, 0) == 0 then return nil end
	if __tmp_spells[s].pval and obj.pval < __tmp_spells[s].pval then return nil end
	return 1
end

-- Get the amount of mana(or power) needed
function get_mana(s)
	return spell(s).mana + get_level(s, spell(s).mana_max - spell(s).mana, 0)
end

-- Return the amount of power(mana, piety, whatever) for the spell
function get_power(s)
	if check_affect(s, "piety", FALSE) then
		return player.grace
	else
		return player.csp
	end
end

-- Return the amount of power(mana, piety, whatever) for the spell
function get_power_name(s)
	if check_affect(s, "piety", FALSE) then
		return "piety"
	else
		return "mana"
	end
end

-- Changes the amount of power(mana, piety, whatever) for the spell
function adjust_power(s, x)
	if check_affect(s, "piety", FALSE) then
		inc_piety(GOD_ALL, x)
	else
		increase_mana(x)
	end
end

-- Get spell school name(s) as a /-separated string.
function spell_school_name(s)
	local xx, sch_str
	xx = nil
	sch_str = ""
	for index, sch in __spell_school[s] do
		if xx then
			sch_str = sch_str.."/"..school(sch).name
		else
			xx = 1
			sch_str = sch_str..school(sch).name
		end
	end
	return sch_str
end

-- Output the desc when sued as a device
function print_device_desc(s)
	local index, desc

	for index, desc in __spell_desc[s] do
		text_out("\n" .. desc)
	end
end

function check_affect(s, name, default)
	local s_ptr = __tmp_spells[s]
	local a

	if type(s_ptr[name]) == "number" then
		a = s_ptr[name]
	else
		a = default
	end
	if a == FALSE then
		return nil
	else
		return TRUE
	end
end

-- Returns the stat to use for the spell, INT by default
function get_spell_stat(s)
	if not __tmp_spells[s].stat then return A_INT
	else return __tmp_spells[s].stat end
end

function cast_school_spell(s, s_ptr, no_cost)
	local use = FALSE

	-- No magic
	if (player.antimagic > 0) then
		msg_print("Your anti-magic field disrupts any magic attempts.")
		return
	end

	-- No magic
	if (player.anti_magic == TRUE) then
		msg_print("Your anti-magic shell disrupts any magic attempts.")
		return
	end

	-- if it costs something then some condition must be met
	if not no_cost then
	 	-- Require lite
		if (check_affect(s, "blind")) and ((player.blind > 0) or (no_lite() == TRUE)) then
			msg_print("You cannot see!")
			return
		end

		-- Not when confused
		if (check_affect(s, "confusion")) and (player.confused > 0) then
			msg_print("You are too confused!")
			return
		end

		-- Enough mana
		if (get_mana(s) > get_power(s)) then
			if (get_check("You do not have enough "..get_power_name(s)..", do you want to try anyway?") == FALSE) then return end
		end
	
		-- Invoke the spell effect
		if (magik(spell_chance(s)) == FALSE) then
			if (__spell_spell[s]() ~= nil) then
				use  = TRUE
			end
		else
			local index, sch

			-- added because this is *extremely* important --pelpel
			if (flush_failure) then flush() end

			msg_print("You failed to get the spell off!")
			for index, sch in __spell_school[s] do
				if __schools[sch].fail then
					__schools[sch].fail(spell_chance(s))
				end
			end
			use  = TRUE
		end
	else
		__spell_spell[s]()
	end

	if use == TRUE then
		-- Reduce mana
		adjust_power(s, -get_mana(s))

		-- Take a turn
		if is_magestaff() == TRUE then energy_use = 80
		else energy_use = 100 end
	end

	player.redraw = bor(player.redraw, PR_MANA)
	player.window = bor(player.window, PW_PLAYER)
end


-- Can the spell be randomly found(in random books)
function can_spell_random(i)
	return __tmp_spells[i].random
end

-- Find a random spell
function get_random_spell(typ, level)
	local spl, tries

	tries = 1000
	while tries > 0 do
		tries = tries - 1
		spl = rand_int(__tmp_spells_num)
		if (can_spell_random(spl) == typ) and (rand_int(spell(spl).skill_level * 3) < level) then
			break
		end
	end
	if tries > 0 then
		return spl
	else
		return -1
	end
end

-- Execute a lasting spell
function exec_lasting_spell(spl)
	assert(__tmp_spells[spl].lasting, "No lasting effect for spell "..__tmp_spells[spl].name.." but called as such")
	return __tmp_spells[spl].lasting()
end

-- Helper function for spell effect to know if they are or not obvious
function is_obvious(effect, old)
	if old then
		if old == TRUE or effect == TRUE then
			return TRUE
		else
			return FALSE
		end
	else
		return effect
	end
end

-------------------------Sticks-------------------------

-- Fire off the spell
function activate_stick(spl)
	local ret = __spell_spell[spl]()
	local charge, obvious
	if not ret then
		charge = FALSE
		obvious = FALSE
	else
		charge = TRUE
		obvious = ret
	end
	return obvious, charge
end

----------------------------------- Wand, Staves, Rods specific functions ----------------------------

-- Get a spell for a given stick(wand, staff, rod)
function get_random_stick(stick, level)
	local spl, tries

	tries = 1000
	while tries > 0 do
		tries = tries - 1
		spl = rand_int(__tmp_spells_num)
		if __tmp_spells[spl].stick and (type(__tmp_spells[spl].stick[stick]) == "table") then
			if (rand_int(spell(spl).skill_level * 3) < level) and (magik(100 - __tmp_spells[spl].stick[stick].rarity) == TRUE) then
				break
			end
		end
	end
	if tries > 0 then
		return spl
	else
		return -1
	end
end

-- Get a random base level
function get_stick_base_level(stick, level, spl)
	-- Paranoia
	if spl < 0 or spl >= __tmp_spells_num or not __tmp_spells[spl].stick[stick] then return 0 end

	local min, max = __tmp_spells[spl].stick[stick].base_level[1], __tmp_spells[spl].stick[stick].base_level[2]
	local range = max - min;

	-- Ok the basic idea is to have a max possible level of half the dungeon level
	if range * 2 > level then range = level / 2 end

	-- Randomize a bit
	range = m_bonus(range, dun_level)

	-- And get the result
	return min + range
end

-- Get a random max level
function get_stick_max_level(stick, level, spl)
	-- Paranoia
	if spl < 0 or spl >= __tmp_spells_num or not __tmp_spells[spl].stick[stick] then return 0 end

	local min, max = __tmp_spells[spl].stick[stick].max_level[1], __tmp_spells[spl].stick[stick].max_level[2]
	local range = max - min;

	-- Ok the basic idea is to have a max possible level of half the dungeon level
	if range * 2 > level then range = level / 2 end

	-- Randomize a bit
	range = m_bonus(range, dun_level)

	-- And get the result
	return min + range
end

-- Get the number of desired charges
function get_stick_charges(spl)
	return __tmp_spells[spl].stick.charge[1] + randint(__tmp_spells[spl].stick.charge[2]);
end

-- Get activation desc
function get_activation_desc(spl)
	local turns
	if type(__tmp_spells[spl].activate) == 'number' then
		turns = __tmp_spells[spl].activate
	else
		turns = __tmp_spells[spl].activate[1] .. '+d' .. __tmp_spells[spl].activate[2]
	end
	return __tmp_spells[spl].desc[1] .. ' every ' .. turns .. ' turns'
end

-- Compute the timeout of an activation
function get_activation_timeout(spl)
	if type(__tmp_spells[spl].activate) == 'number' then
		return __tmp_spells[spl].activate
	else
		return __tmp_spells[spl].activate[1] + randint(__tmp_spells[spl].activate[2])
	end
end

-- Fire off the spell
function activate_activation(spl, item)
	__spell_spell[spl](item)
end
