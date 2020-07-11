#
# Box-ball system, v1
# https://projecteuler.net/problem=426
#
# Example of sequences:
#     [46, 24, 9, 1, 21, 51, 54, 6, 3, 7, 31]
#     [24, 9, 1, 21, 51, 54, 6, 3, 7, 31, 75]
#     [9, 1, 16, 56, 51, 9, 3, 7, 10, 96, 75]
#     [1, 16, 24, 83, 9, 3, 7, 10, 48, 123, 75]
#     [1, 39, 24, 68, 3, 7, 10, 48, 51, 147, 75]
#     [1, 62, 24, 47, 3, 14, 10, 89, 51, 171, 75]
#     [1, 85, 24, 26, 3, 21, 10, 130, 51, 195, 75]
#     [1, 108, 24, 5, 3, 28, 10, 171, 51, 219, 75]
#     [1, 131, 5, 3, 22, 16, 10, 212, 51, 243, 75]
#     [1, 135, 3, 22, 16, 10, 18, 245, 51, 267, 75]
#     [1, 137, 3, 35, 10, 18, 24, 272, 51, 291, 75]
#     => [1, 3, 10, 24, 51, 75]
#


def next_turn(row):
    moved_cell_indexes = set()
    while True:
        cell_index = next(
            (i for i, row in enumerate(row) if row and i not in moved_cell_indexes), None
        )
        if cell_index is None:
            break
        next_index = next((i for i in range(cell_index + 1, len(row)) if not row[i]), None)
        if next_index is None:
            row.append(False)
            next_index = len(row) - 1
        row[cell_index], row[next_index] = row[next_index], row[cell_index]
        moved_cell_indexes.add(next_index)

    # reducing memomry consuption
    row = row[next(i for i, row in enumerate(row) if row) :]


def is_final_turn(row):
    previous_num_occupied_boxes = 0
    num_occupied_boxes = 0
    num_empty_boxes = 0
    start = next(i for i, row in enumerate(row) if row)
    for i in range(start, len(row)):
        if row[i]:
            if num_empty_boxes > 0:
                if num_empty_boxes > num_occupied_boxes:
                    if previous_num_occupied_boxes > num_occupied_boxes:
                        return False
                    num_empty_boxes = 0
                    previous_num_occupied_boxes = num_occupied_boxes
                    num_occupied_boxes = 0
                else:
                    return False
            num_occupied_boxes += 1
        else:
            num_empty_boxes += 1
    return True


def run():
    limit = 10

    sequence = []
    s = 290797
    for _ in range(limit + 1):
        sequence.append(s % 64 + 1)
        s = s ** 2 % 50515093

    row = []
    for i, num_boxes in enumerate(sequence):
        row.extend([i % 2 == 0 for _ in range(num_boxes)])

    while not is_final_turn(row):
        next_turn(row)

    start = next(i for i, row in enumerate(row) if row)
    resulting_sequence = []
    num_occupied_boxes = 0
    for i in range(start, len(row)):
        if row[i]:
            num_occupied_boxes += 1
        else:
            if num_occupied_boxes > 0:
                resulting_sequence.append(num_occupied_boxes)
                num_occupied_boxes = 0
    if num_occupied_boxes > 0:
        resulting_sequence.append(num_occupied_boxes)

    print('Resulting sequence:', resulting_sequence)
