samples = int(input())


for _ in range(samples):
    grades_list = [int(x) for x in input().split(' ')]
    total_grades = grades_list.pop(0)
    grade_sum = 0
    for grade in grades_list:
        grade_sum += grade
    avg = grade_sum / total_grades
    students = 0
    for grade in grades_list:
        if grade > avg:
            students += 1
    total_students = students * 100 / total_grades
    print("{:.3f}%".format(total_students))