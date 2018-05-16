NODE, EDGE, ATTR = range(3)


class Node(object):
    @staticmethod
    def validateNode(obj):
        if not (isinstance(obj[1], str) and
                isinstance(obj[2], (dict, set))):
            raise ValueError('Malformed node item')

    def __init__(self, name, attrs={}):
        self.name = name
        self.attrs = attrs

    def __eq__(self, other):
        return self.name == other.name and self.attrs == other.attrs


class Edge(object):
    @staticmethod
    def validateEdge(obj):
        if not (isinstance(obj[1], str) and
                isinstance(obj[2], str) and
                isinstance(obj[3], (dict, set))):
            raise ValueError('Malformed edge item')

    def __init__(self, src, dst, attrs={}):
        self.src = src
        self.dst = dst
        self.attrs = attrs

    def __eq__(self, other):
        return (self.src == other.src and
                self.dst == other.dst and
                self.attrs == other.attrs)


class Graph(object):
    @staticmethod
    def validateAttr(obj):
        if not all(isinstance(o, str) for o in obj[1:]):
            raise ValueError('Malformed attribute item')

    @staticmethod
    def validateItem(obj):
        if obj[0] == NODE:
            Node.validateNode(obj)
        elif obj[0] == EDGE:
            Edge.validateEdge(obj)
        elif obj[0] == ATTR:
            Graph.validateAttr(obj)
        else:
            raise ValueError('Malformed graph item')

    def __init__(self, data=[]):
        if not isinstance(data, list):
            raise TypeError('Malformed data list')

        self.nodes = []
        self.edges = []
        self.attrs = {}

        try:
            for obj in data:
                Graph.validateItem(obj)
                if obj[0] == NODE:
                    self.nodes.append(Node(obj[1], obj[2]))
                elif obj[0] == EDGE:
                    self.edges.append(Edge(obj[1], obj[2], obj[3]))
                elif obj[0] == ATTR:
                    self.attrs[obj[1]] = obj[2]
        except IndexError:
            raise TypeError('Malformed item')
