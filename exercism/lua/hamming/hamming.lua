local hamming = {}

function string.at(self, i)
  --return self:sub(i, i)
  return self:byte(i) -- something I learned
end

function string.ndiff(self, other)
  if #self ~= #other then return -1 end
  n = 0
  for i=1, #self do
    if self:at(i) ~= other:at(i) then
      n = n + 1
    end
  end
  return n
end

function hamming.compute(lhs, rhs)
  return lhs:ndiff(rhs)
end

return hamming
