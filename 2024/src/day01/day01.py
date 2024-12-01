def first():
    with open('input.txt', 'r') as file:
        l1 = []
        l2 = []
        for line in file:
            # Split the line into parts (assuming numbers are space-separated)
            num1, num2 = map(int, line.split())
            l1.append(num1)
            l2.append(num2)
        l1.sort()
        l2.sort()
        diff = 0
        for index,value in enumerate(l1):
            diff += abs(l2[index]- l1[index])
        print(diff)

def second():
    with open('input.txt', 'r') as file:
        l1 = []
        l2 = []
        frequency_map = {}
        for line in file:
            # Split the line into parts (assuming numbers are space-separated)
            num1, num2 = map(int, line.split())
            l1.append(num1)
            l2.append(num2)
        for item in l2:
            if item in frequency_map:
                frequency_map[item] += 1
            else:
                frequency_map[item] = 1
        l1.sort()
        result = 0
        for index, value in enumerate(l1):
            if (l1[index] in frequency_map):
                result += l1[index]*frequency_map[l1[index]]
        print(result)

first()
second()