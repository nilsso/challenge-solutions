def palindrome_products(max_factor, min_factor):
    for i in range(min_factor, max_factor+1):
        for j in range(i, max_factor+1):
            p = i*j
            s = str(p)
            if s == s[::-1]:
                yield p, (i, j)


def smallest_palindrome(max_factor, min_factor):
    return min(palindrome_products(max_factor, min_factor))


def largest_palindrome(max_factor, min_factor=1):
    return max(palindrome_products(max_factor, min_factor))
