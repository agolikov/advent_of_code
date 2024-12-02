def first():
    with open('input.txt', 'r') as file:
        count = 0
        for line in file:
            nums = list(map(int, line.split()))
            if is_safe(nums):
                count+=1
        print(count)

def is_safe(nums):
    df = []
    for index in range(1, len(nums)):
        df.append(nums[index - 1] - nums[index])
    if any(x > 3 or x < -3 or x == 0 for x in df):
        return False
    negatives = {num for num in df if num < 0}
    positives = {num for num in df if num > 0}
    if len(negatives) == 0 or len(positives) == 0:
        return True
    return False

def second():
    with open('input.txt', 'r') as file:
        count = 0
        for line in file:
            nums = list(map(int, line.split()))
            if is_safe(nums):
                count += 1
            else:
                for x in range(0,len(nums)):
                    cp = nums.copy()
                    del cp[x]
                    if is_safe(cp):
                        count+=1
                        break
        print(count)
first()
second()
