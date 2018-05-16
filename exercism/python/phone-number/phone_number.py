import re


pattern = re.compile(r'''
\D*(\d)?    # Country code
\D*(\d{3})  # Area code
\D*(\d{3})  # Exchange code
\D*(\d{4})  # Subscriber number
\D*''', re.VERBOSE)


class Phone(object):
    def __init__(self, token):
        self.area_code,\
        self.exhange_code,\
        self.subscriber_number = Phone.parse(token)

    @property
    def number(self):
        return ''.join([
            self.area_code, self.exhange_code, self.subscriber_number])

    def pretty(self):
        return '({}) {}-{}'.format(
            self.area_code, self.exhange_code, self.subscriber_number)

    @staticmethod
    def parse(token):
        m = pattern.match(token)
        if not m:
            raise ValueError('Malformed phone number')
        if m[1] and m[1] != '1':
            raise ValueError('Unsuported country code "{}"'.format(m[1]))
        if m[2][0] not in "23456789":
            raise ValueError('Invalid area code "{}"'.format(m[2]))
        if m[3][0] not in "23456789":
            raise ValueError('Invalid exchange code "{}"'.format(m[3]))
        return m.groups()[1:]
