#!/usr/bin/env python
# -*- coding: utf-8 -*-

# https://projecteuler.net/problem=701

from random import randint
from typing import List
import networkx as nx


# ############## Algo de calcule ###########################
def createGraph(rectangle: List[List], number_of_row: int, number_of_column: int):
    number_of_vertex: int = number_of_row * number_of_column
    G = nx.Graph()

    G.add_nodes_from(range(0, number_of_vertex))

    # Create link
    k: int = 0
    for i in range(0, number_of_row):  # Hauteur
        for j in range(0, number_of_column):  # Largeur
            if j != 0:
                if rectangle[k - 1] == 1 and rectangle[k] == 1:
                    G.add_edge(k - 1, k)
            if i != 0:
                if rectangle[k - number_of_column] == 1 and rectangle[k] == 1:
                    G.add_edge(k - number_of_column, k)
            k += 1
    return G


def find_area(G, bit_string):
    def f(G, node: int, visited: set):
        neighbors = G.neighbors(node)
        for i in neighbors:
            if i not in visited:
                visited.add(i)
                f(G, i, visited)

    areas: List = []

    nodes_to_explore: set = set(range(0, len(G)))
    singleton: bool = False
    while len(nodes_to_explore) != 0:
        node = nodes_to_explore.pop()
        singleton = singleton or bit_string[node]
        visited: set = set()
        f(G, node, visited)
        areas += [len(visited)]
        nodes_to_explore = nodes_to_explore.difference(visited)
    return max(singleton, max(areas))


# ############## Algo de calcule ###########################


def test_2_2_deterministe():

    number_of_row = 2
    number_of_column = 2
    combis = [
        [0, 0, 0, 0],
        [0, 0, 0, 1],
        [0, 0, 1, 0],
        [0, 0, 1, 1],
        [0, 1, 0, 0],
        [0, 1, 0, 1],
        [0, 1, 1, 0],
        [0, 1, 1, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 1],
        [1, 0, 1, 0],
        [1, 0, 1, 1],
        [1, 1, 0, 0],
        [1, 1, 0, 1],
        [1, 1, 1, 0],
        [1, 1, 1, 1],
    ]
    somme = 0
    for combi in combis:
        G = createGraph(combi, number_of_row, number_of_column)
        somme += find_area(G, combi)
    print(somme / (len(combis)))  # -> 1.875


def test_2_2_stochastic():
    max_area_sum = 0
    num_possibilities = 16 * 10000
    number_of_row = 2
    number_of_column = 2
    size = 4

    for i in range(num_possibilities):
        bit_string = [randint(0, 1) for _ in range(size)]
        if all(value == 0 for value in bit_string):
            continue
        G = createGraph(bit_string, number_of_row, number_of_column)
        max_area_sum += find_area(G, bit_string)

    print(max_area_sum / num_possibilities)


def test_5_5_deterministe():
    # number_of_row = 5
    # number_of_column = 5
    # bit_string_exemple_1 = [0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1] # -> 15
    # bit_string_exemple_1[12] = 0
    # G = createGraph(bit_string_exemple_1, number_of_row, number_of_column)
    pass


if __name__ == "__main__":
    print("Version deterministe : ")
    test_2_2_deterministe()
    print("Version stochastic")
    test_2_2_stochastic()
