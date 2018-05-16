import re

students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry" ]

plant_list = [
        "Grass", "Clover", "Radishes", "Violets" ]

def diagram_to_list(d, s):
    d = re.sub("[^GCRV]", "", d)
    n = len(d) // 2
    return [ d[i:i+2]+d[i+n:i+2+n] for i in range(0, n, 2) ]

class Garden(object):

    def __init__(self, diagram, students=students, plant_list=plant_list):
        self.students = sorted(students)
        self.plant_list = plant_list
        self.plant_dict = { p[0]:p for p in plant_list }
        self.assignments = diagram_to_list(diagram, students)

    def plants(self, name):
        return [ self.plant_dict[c] for c in
                self.assignments[self.students.index(name)] ]

