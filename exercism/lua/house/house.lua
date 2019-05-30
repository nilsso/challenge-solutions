local house = {}

local things = {
  {"malt", "lay in"},
  {"rat", "ate"},
  {"cat", "killed"},
  {"dog", "worried"},
  {"cow with the crumpled horn", "tossed"},
  {"maiden all forlorn", "milked"},
  {"man all tattered and torn", "kissed"},
  {"priest all shaven and shorn", "married"},
  {"rooster that crowed in the morn", "woke"},
  {"farmer sowing his corn", "kept"},
  {"horse and the hound and the horn", "belonged to"}
}

function house.verse(i)
  v = "This is the "
  for j=i-1, 1, -1 do
    thing = things[j]
    v = v .. thing[1] .. "\nthat " .. thing[2] .. " the "
  end
  return v .. "house that Jack built."
end

function house.recite()
  s = house.verse(1)
  for i=2, #things+1 do
    s = s .. "\n" .. house.verse(i)
  end
  return s
end

return house
