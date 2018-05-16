def binary_search(list_of_numbers, number):
    left, right = 0, len(list_of_numbers)-1
    while left <= right:
        i = (left + right) // 2
        n = list_of_numbers[i]
        if number < n:
            right = i - 1
        elif number > n:
            left = i + 1
        else:
            return i
    raise ValueError(f'{number} not found in list')
