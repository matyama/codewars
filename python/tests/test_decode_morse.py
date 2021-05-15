from codewars.decode_morse import decodeMorse


def test_decode_morse() -> None:
    assert decodeMorse('.... . -.--   .--- ..- -.. .') == 'HEY JUDE'
