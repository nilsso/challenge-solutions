def detect_anagrams(word, candidates):
    L = word.lower()
    S = sorted(L)
    return [ w for w in candidates if
            w.lower() != L and sorted(w.lower()) == S ]
