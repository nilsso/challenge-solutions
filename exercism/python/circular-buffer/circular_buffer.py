class BufferFullException(Exception):
    def __init__(self, msg=None):
        super().__init__(msg or "Attempted to write with full buffer")


class BufferEmptyException(Exception):
    def __init__(self, msg=None):
        super().__init__(msg or "Attempted to read with empty buffer")


class CircularBuffer(object):
    def __init__(self, capacity):
        self.capacity = capacity
        self.data = []

    def write(self, n):
        if len(self.data) == self.capacity:
            raise BufferFullException
        self.data.append(n)

    def read(self):
        if len(self.data) == 0:
            raise BufferEmptyException
        return self.data.pop(0)

    def overwrite(self, n):
        if len(self.data) == self.capacity:
            self.read()
        self.write(n)

    def clear(self):
        self.data = []
