import random

def bubbleSort(array):
    length = len(array)
    for i in range(length):
        for j in range(length - i - 1):
            if array[j] > array[j + 1]:
                array[j], array[j + 1] = array[j + 1], array[j]

tab = []

for i in range(10):
    tab.append(random.randint(1, 100))
print(*tab, sep=", ")

bubbleSort(tab)

print(*tab, sep=", ")