# Set 1, Challenge 1
# Convert the string to base64
from codecs import decode, encode

s = '''\
49276d206b696c6c696e6720796f757220627261696e206c\
696b65206120706f69736f6e6f7573206d757368726f6f6d'''

print(encode(decode(s, 'hex'), 'base64'))

