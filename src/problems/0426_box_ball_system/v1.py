#
# Box-ball system, v1
# https://projecteuler.net/problem=426
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
