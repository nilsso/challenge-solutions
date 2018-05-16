class Record():
    def __init__(self, record_id, parent_id):
        self.record_id = record_id
        self.parent_id = parent_id

    def __lt__(self, other):
        if isinstance(other, Record):
            return self.record_id < other.record_id
        return NotImplemented

    def __iter__(self):
        yield self.record_id
        yield self.parent_id


class Node():
    def __init__(self, node_id):
        self.node_id = node_id
        self.children = []


def BuildTree(records):
    if not records:
        return
    records.sort()
    if [r.record_id for r in records] != list(range(len(records))):
        raise ValueError('Non-sequential record IDs')
    if tuple(records[0]) != (0, 0):
        raise ValueError('Root record ID and parent ID not both zero')
    tree = [Node(0)]
    for i, r in list(enumerate(records))[1:]:
        if r.record_id <= r.parent_id:
            raise ValueError('Same record ID and parent ID')
        n = Node(i)
        tree.append(n)
        tree[r.parent_id].children.append(n)
    return tree[0]
