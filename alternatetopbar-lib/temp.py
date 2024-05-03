"""
total_length = 11
left_length = 3
center_length = 1
right_length = 1

# Calculate total padding
total_padding = total_length - (left_length + center_length + right_length)

# Calculate left and right padding
left_padding = (total_length // 2) - left_length
right_padding = total_padding - left_padding

print("Left padding: ", left_padding)
print("Right padding: ", right_padding)
"""

def find_middle(cols):
    if cols % 2 != 0:
        return (cols//2)+1
    return cols//2

def find_padding(cols, left_size, center_size, right_size):
    total_padding = cols - (left_size + center_size + right_size)
    middle = find_middle(cols)
    if left_size >= middle:
        left_padding = 0
    elif cols % 2 == 0:
        print(f"({cols}/2)-{left_size}-({center_size}/2)")
        left_padding = (cols//2)-left_size-(center_size//2)
    else:
        left_padding = (cols//2)-left_size
    right_padding = total_padding - left_padding
    return left_padding, right_padding

# 2,2


def build(cols, left_size, center_size, right_size, correct):
    l = "L"
    r = "R"
    c = "C"

    lp, rp = find_padding(cols, left_size, center_size, right_size)

    print("Left padding:", lp)
    print("Right padding:", rp)
    print("{left}{lp}{center}{rp}{right}".format(
        left=l * left_size,
        lp="*" * lp,
        center=c * center_size,
        rp="*" * rp,
        right=r * right_size
    ))
    if correct[0] != lp or correct[1] != rp:
        print("deviation, calculation: ({}) vs expected: ({})".format((lp, rp), correct))
        exit(1)


build(11, 2, 2, 2, (3,2))
build(11, 3, 1, 1, (2,4))
build(11, 3, 1, 3, (2,2))
build(6, 3, 1, 1, (0,1))
build(6, 3, 1, 1, (0,1))
build(11, 3, 2, 2, (2,2))
build(14, 2, 2, 2, (4,4))
build(10, 2, 2, 2, (2,2))
build(14, 5, 3, 3, (1,2))
build(14, 7, 2, 3, (0,2))
