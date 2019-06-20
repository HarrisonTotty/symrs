# Miscellaneous Jinja Functions
# -----------------------------


def format_list(format_string, input_list):
    '''
    Formats each item in the given list according to the specified format string.
    '''
    return [format_string.format(i) for i in input_list]
