from codewars.decode_morse_adv import decode_bits, decode_morse


def test_simple_messages() -> None:
    assert decode_bits('111') == '.'
    assert decode_bits('111000111') == '..'
    assert decode_bits('111000111000111') == '...'
    assert decode_bits('10001') == '. .'
    assert decode_bits('111000000000111') == '. .'
    assert decode_bits('111110000011111') == '..'


def test_leading_and_trailing_zeros() -> None:
    assert decode_bits('01110') == '.'
    assert decode_bits('000000011100000') == '.'


def test_example_input() -> None:
    bits = (
        '110011001100110000001100000011111100110011111100111111'
        '000000000000001100111111001111110011111100000011001100'
        '1111110000001111110011001100000011'
    )
    assert decode_bits(bits) == '.... . -.--   .--- ..- -.. .'
    assert decode_morse('.... . -.--   .--- ..- -.. .') == 'HEY JUDE'
