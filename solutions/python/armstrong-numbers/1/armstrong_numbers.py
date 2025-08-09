def is_armstrong_number(number):
    digits = list()
    og_number = number

    while number > 0:
        # Get right-most digit from the number
        digit = number % 10
        digits.append(digit)

        # Get rid of last digit from the number
        number //= 10

    num_digits = len(digits)

    sum_of_powers = 0
    for digit in digits:
        sum_of_powers += digit**num_digits

    if sum_of_powers == og_number:
        return True
    else:
        return False
