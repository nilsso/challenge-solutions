things = [
    ('malt', 'lay in the '),
    ('rat', 'ate the '),
    ('cat', 'killed the '),
    ('dog', 'worried the '),
    ('cow with the crumpled horn', 'tossed the '),
    ('maiden all forlorn', 'milked the '),
    ('man all tattered and torn', 'kissed the '),
    ('priest all shaven and shorn', 'married the '),
    ('rooster that crowed in the morn', 'woke the '),
    ('farmer sowing his corn', 'kept the '),
    ('horse and the hound and the horn', 'belonged to the ')]


def verse(n):
    return 'This is the {}house that Jack built.'.format(
            ''.join(
                '{}\nthat {}'.format(who_it_be, what_it_do)
                for who_it_be, what_it_do in reversed(things[:n])))


def rhyme():
    return '\n\n'.join(verse(n) for n in range(len(things)+1))
