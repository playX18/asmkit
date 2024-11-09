import argparse
import re 

remspaces = re.compile(r"[\n\t\s]*")
# fetch opcode name from the line, return name + line after the opcode
# format of the line: `<opname>, rest...`
# format of the output: `rest...`
def opname(line):
    # Remove leading and trailing spaces, tabs, and newlines
    line = remspaces.sub("", line)
    
    # Split the line by the first comma to get the opcode name and the rest
    parts = line.split(",", 1)
    
    # Ensure the line has an opcode name and a rest part
    if len(parts) != 2:
        raise ValueError("Invalid line format: expected '<opname>, rest...'")

    # Return the opcode name and the rest of the line
    opcode_name, rest = parts[0], parts[1]
    return opcode_name, rest

# Matches a closing bracket in string `s` starting `from` the given index.
# It behaves like `s.indexOf()`, but uses a counter and skips all nested
# matches.
def match_closing_char(s, from_index):
    len_s = len(s)
    opening = ord(s[from_index])
    closing = 41 if opening == 40 else 62 if opening == 60 else 93 if opening == 91 else 125 if opening == 123 else 0

    i = from_index
    pending = 1
    while pending:
        i += 1
        if i >= len_s:
            break
        c = ord(s[i])
        pending += 1 if c == opening else 0
        pending -= 1 if c == closing else 0

    return i


def opcode(opc):
    if opc.startswith('OP'):
        clos = match_closing_char(opc, 2)
        opn = int(opc[2:clos])
        return (opn & 0x3f) << 26
    elif opc.startswith('X'):
        # XOP(op, xop)
        clos = match_closing_char(opc, 1)
        s = opc[2:clos].split(',', maxsplit=1) 
        op = int(s[0])
        xop = int(s[1])

        return (op & 0x3f) << 26 | ((xop & 0x3ff) << 1)
    else:
        return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--opcodes", type=argparse.FileType('r'))
    parser.add_argument('out_opcodes', type=argparse.FileType('w'))
    args =parser.parse_args()
    for line in args.opcodes.read().splitlines():

        pattern = re.match(r'(\w+),\s*(\S+)\s*\((\d+),\s*(\d+)\),\s*(\w+),\s*(\w+),\s*(\d+),\s*\{([^\}]+)\}', line)
        if pattern: 
            for m in pattern.groups():
                print(m)
