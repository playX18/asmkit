#!/usr/bin/env python3

import collections
import copy
import glob
import logging
import os
import pprint
import re
import sys

import yaml

from constants import *

pp = pprint.PrettyPrinter(indent=2)
logging.basicConfig(level=logging.INFO, format="%(levelname)s:: %(message)s")


def process_enc_line(line, ext):
    """
    This function processes each line of the encoding files (rv*). As part of
    the processing, the function ensures that the encoding is legal through the
    following checks::

        - there is no over specification (same bits assigned different values)
        - there is no under specification (some bits not assigned values)
        - bit ranges are in the format hi..lo=val where hi > lo
        - value assigned is representable in the bit range
        - also checks that the mapping of arguments of an instruction exists in
          arg_lut.

    If the above checks pass, then the function returns a tuple of the name and
    a dictionary containing basic information of the instruction which includes:
        - variables: list of arguments used by the instruction whose mapping
          exists in the arg_lut dictionary
        - encoding: this contains the 32-bit encoding of the instruction where
          '-' is used to represent position of arguments and 1/0 is used to
          reprsent the static encoding of the bits
        - extension: this field contains the rv* filename from which this
          instruction was included
        - match: hex value representing the bits that need to match to detect
          this instruction
        - mask: hex value representin the bits that need to be masked to extract
          the value required for matching.
    """
    single_dict = {}

    # fill all bits with don't care. we use '-' to represent don't care
    # TODO: hardcoded for 32-bits.
    encoding = ["-"] * 32

    # get the name of instruction by splitting based on the first space
    [name, remaining] = line.split(" ", 1)

    # replace dots with underscores as dot doesn't work with C/Sverilog, etc
    name = name.replace(".", "_")

    # remove leading whitespaces
    remaining = remaining.lstrip()

    # check each field for it's length and overlapping bits
    # ex: 1..0=5 will result in an error --> x<y
    # ex: 5..0=0 2..1=2 --> overlapping bits
    for s2, s1, entry in fixed_ranges.findall(remaining):
        msb = int(s2)
        lsb = int(s1)

        # check msb < lsb
        if msb < lsb:
            logging.error(
                f'{line.split(" ")[0]:<10} has position {msb} less than position {lsb} in it\'s encoding'
            )
            raise SystemExit(1)

        # illegal value assigned as per bit width
        entry_value = int(entry, 0)
        if entry_value >= (1 << (msb - lsb + 1)):
            logging.error(
                f'{line.split(" ")[0]:<10} has an illegal value {entry_value} assigned as per the bit width {msb - lsb}'
            )
            raise SystemExit(1)

        for ind in range(lsb, msb + 1):
            # overlapping bits
            if encoding[31 - ind] != "-":
                logging.error(
                    f'{line.split(" ")[0]:<10} has {ind} bit overlapping in it\'s opcodes'
                )
                raise SystemExit(1)
            bit = str((entry_value >> (ind - lsb)) & 1)
            encoding[31 - ind] = bit

    # extract bit pattern assignments of the form hi..lo=val
    remaining = fixed_ranges.sub(" ", remaining)

    # do the same as above but for <lsb>=<val> pattern. single_fixed is a regex
    # expression present in constants.py
    for lsb, value, drop in single_fixed.findall(remaining):
        lsb = int(lsb, 0)
        value = int(value, 0)
        if encoding[31 - lsb] != "-":
            logging.error(
                f'{line.split(" ")[0]:<10} has {lsb} bit overlapping in it\'s opcodes'
            )
            raise SystemExit(1)
        encoding[31 - lsb] = str(value)

    # convert the list of encodings into a single string for match and mask
    match = "".join(encoding).replace("-", "0")
    mask = "".join(encoding).replace("0", "1").replace("-", "0")

    # check if all args of the instruction are present in arg_lut present in
    # constants.py
    args = single_fixed.sub(" ", remaining).split()
    encoding_args = encoding.copy()
    for a in args:
        if a not in arg_lut:
            parts = a.split("=")
            if len(parts) == 2:
                existing_arg, new_arg = parts
                if existing_arg in arg_lut:
                    arg_lut[a] = arg_lut[existing_arg]

                else:
                    logging.error(
                        f" Found field {existing_arg} in variable {a} in instruction {name} whose mapping in arg_lut does not exist"
                    )
                    raise SystemExit(1)
            else:
                logging.error(
                    f" Found variable {a} in instruction {name} whose mapping in arg_lut does not exist"
                )
                raise SystemExit(1)
        (msb, lsb) = arg_lut[a]
        for ind in range(lsb, msb + 1):
            # overlapping bits
            if encoding_args[31 - ind] != "-":
                logging.error(
                    f" Found variable {a} in instruction {name} overlapping {encoding_args[31 - ind]} variable in bit {ind}"
                )
                raise SystemExit(1)
            encoding_args[31 - ind] = a

    # update the fields of the instruction as a dict and return back along with
    # the name of the instruction
    single_dict["encoding"] = "".join(encoding)
    single_dict["variable_fields"] = args
    single_dict["extension"] = [os.path.basename(ext)]
    single_dict["match"] = hex(int(match, 2))
    single_dict["mask"] = hex(int(mask, 2))

    return (name, single_dict)


def same_base_isa(ext_name, ext_name_list):
    type1 = ext_name.split("_")[0]
    for ext_name1 in ext_name_list:
        type2 = ext_name1.split("_")[0]
        # "rv" mean insn for rv32 and rv64
        if (
            type1 == type2
            or (type2 == "rv" and (type1 == "rv32" or type1 == "rv64"))
            or (type1 == "rv" and (type2 == "rv32" or type2 == "rv64"))
        ):
            return True
    return False


def overlaps(x, y):
    x = x.rjust(len(y), "-")
    y = y.rjust(len(x), "-")

    for i in range(0, len(x)):
        if not (x[i] == "-" or y[i] == "-" or x[i] == y[i]):
            return False

    return True


def overlap_allowed(a, x, y):
    return x in a and y in a[x] or y in a and x in a[y]


def extension_overlap_allowed(x, y):
    return overlap_allowed(overlapping_extensions, x, y)


def instruction_overlap_allowed(x, y):
    return overlap_allowed(overlapping_instructions, x, y)


def add_segmented_vls_insn(instr_dict):
    updated_dict = {}
    for k, v in instr_dict.items():
        if "nf" in v["variable_fields"]:
            for new_key, new_value in expand_nf_field(k, v):
                updated_dict[new_key] = new_value
        else:
            updated_dict[k] = v
    return updated_dict


def expand_nf_field(name, single_dict):
    if "nf" not in single_dict["variable_fields"]:
        logging.error(f"Cannot expand nf field for instruction {name}")
        raise SystemExit(1)

    # nf no longer a variable field
    single_dict["variable_fields"].remove("nf")
    # include nf in mask
    single_dict["mask"] = hex(int(single_dict["mask"], 16) | 0b111 << 29)

    name_expand_index = name.find("e")
    expanded_instructions = []
    for nf in range(0, 8):
        new_single_dict = copy.deepcopy(single_dict)
        new_single_dict["match"] = hex(int(single_dict["match"], 16) | nf << 29)
        new_single_dict["encoding"] = format(nf, "03b") + single_dict["encoding"][3:]
        new_name = (
            name
            if nf == 0
            else name[:name_expand_index]
            + "seg"
            + str(nf + 1)
            + name[name_expand_index:]
        )
        expanded_instructions.append((new_name, new_single_dict))
    return expanded_instructions


def create_inst_dict(file_filter, include_pseudo=False, include_pseudo_ops=[]):
    """
    This function return a dictionary containing all instructions associated
    with an extension defined by the file_filter input. The file_filter input
    needs to be rv* file name with out the 'rv' prefix i.e. '_i', '32_i', etc.

    Each node of the dictionary will correspond to an instruction which again is
    a dictionary. The dictionary contents of each instruction includes:
        - variables: list of arguments used by the instruction whose mapping
          exists in the arg_lut dictionary
        - encoding: this contains the 32-bit encoding of the instruction where
          '-' is used to represent position of arguments and 1/0 is used to
          reprsent the static encoding of the bits
        - extension: this field contains the rv* filename from which this
          instruction was included
        - match: hex value representing the bits that need to match to detect
          this instruction
        - mask: hex value representin the bits that need to be masked to extract
          the value required for matching.

    In order to build this dictionary, the function does 2 passes over the same
    rv<file_filter> file. The first pass is to extract all standard
    instructions. In this pass, all pseudo ops and imported instructions are
    skipped. For each selected line of the file, we call process_enc_line
    function to create the above mentioned dictionary contents of the
    instruction. Checks are performed in this function to ensure that the same
    instruction is not added twice to the overall dictionary.

    In the second pass, this function parses only pseudo_ops. For each pseudo_op
    this function checks if the dependent extension and instruction, both, exist
    before parsing it. The pseudo op is only added to the overall dictionary if
    the dependent instruction is not present in the dictionary, else it is
    skipped.


    """
    opcodes_dir = RISCV_OPCODES
    print(opcodes_dir)
    instr_dict = {}

    # file_names contains all files to be parsed in the riscv-opcodes directory
    file_names = []
    for fil in file_filter:
        file_names += glob.glob(f"{opcodes_dir}/{fil}")
    file_names.sort(reverse=True)
    print(file_names)
    # first pass if for standard/regular instructions
    logging.debug("Collecting standard instructions first")
    for f in file_names:
        logging.debug(f"Parsing File: {f} for standard instructions")
        with open(f) as fp:
            lines = (line.rstrip() for line in fp)  # All lines including the blank ones
            lines = list(line for line in lines if line)  # Non-blank lines
            lines = list(
                line for line in lines if not line.startswith("#")
            )  # remove comment lines

        # go through each line of the file
        for line in lines:
            # if the an instruction needs to be imported then go to the
            # respective file and pick the line that has the instruction.
            # The variable 'line' will now point to the new line from the
            # imported file

            # ignore all lines starting with $import and $pseudo
            if "$import" in line or "$pseudo" in line:
                continue
            logging.debug(f"     Processing line: {line}")

            # call process_enc_line to get the data about the current
            # instruction
            (name, single_dict) = process_enc_line(line, f)
            ext_name = os.path.basename(f)

            # if an instruction has already been added to the filtered
            # instruction dictionary throw an error saying the given
            # instruction is already imported and raise SystemExit
            if name in instr_dict:
                var = instr_dict[name]["extension"]
                if same_base_isa(ext_name, var):
                    # disable same names on the same base ISA
                    err_msg = f"instruction : {name} from "
                    err_msg += f"{ext_name} is already "
                    err_msg += f"added from {var} in same base ISA"
                    logging.error(err_msg)
                    raise SystemExit(1)
                elif instr_dict[name]["encoding"] != single_dict["encoding"]:
                    # disable same names with different encodings on different base ISAs
                    err_msg = f"instruction : {name} from "
                    err_msg += f"{ext_name} is already "
                    err_msg += f"added from {var} but each have different encodings in different base ISAs"
                    logging.error(err_msg)
                    raise SystemExit(1)
                instr_dict[name]["extension"].extend(single_dict["extension"])
            else:
                for key in instr_dict:
                    item = instr_dict[key]
                    if (
                        overlaps(item["encoding"], single_dict["encoding"])
                        and not extension_overlap_allowed(
                            ext_name, item["extension"][0]
                        )
                        and not instruction_overlap_allowed(name, key)
                        and same_base_isa(ext_name, item["extension"])
                    ):
                        # disable different names with overlapping encodings on the same base ISA
                        err_msg = f"instruction : {name} in extension "
                        err_msg += f"{ext_name} overlaps instruction {key} "
                        err_msg += f'in extension {item["extension"]}'
                        logging.error(err_msg)
                        raise SystemExit(1)

            if name not in instr_dict:
                # update the final dict with the instruction
                instr_dict[name] = single_dict

    # second pass if for pseudo instructions
    logging.debug("Collecting pseudo instructions now")
    for f in file_names:
        logging.debug(f"Parsing File: {f} for pseudo_ops")
        with open(f) as fp:
            lines = (line.rstrip() for line in fp)  # All lines including the blank ones
            lines = list(line for line in lines if line)  # Non-blank lines
            lines = list(
                line for line in lines if not line.startswith("#")
            )  # remove comment lines

        # go through each line of the file
        for line in lines:

            # ignore all lines not starting with $pseudo
            if "$pseudo" not in line:
                continue
            logging.debug(f"     Processing line: {line}")

            # use the regex pseudo_regex from constants.py to find the dependent
            # extension, dependent instruction, the pseudo_op in question and
            # its encoding
            (ext, orig_inst, pseudo_inst, line) = pseudo_regex.findall(line)[0]
            ext_file = f"{opcodes_dir}/{ext}"

            # check if the file of the dependent extension exist. Throw error if
            # it doesn't
            if not os.path.exists(ext_file):
                ext1_file = f"{opcodes_dir}/unratified/{ext}"
                if not os.path.exists(ext1_file):
                    logging.error(
                        f"Pseudo op {pseudo_inst} in {f} depends on {ext} which is not available"
                    )
                    raise SystemExit(1)
                else:
                    ext_file = ext1_file

            # check if the dependent instruction exist in the dependent
            # extension. Else throw error.
            found = False
            for oline in open(ext_file):
                if not re.findall(f"^\\s*{orig_inst}\\s+", oline):
                    continue
                else:
                    found = True
                    break
            if not found:
                logging.error(
                    f"Orig instruction {orig_inst} not found in {ext}. Required by pseudo_op {pseudo_inst} present in {f}"
                )
                raise SystemExit(1)

            (name, single_dict) = process_enc_line(pseudo_inst + " " + line, f)
            # add the pseudo_op to the dictionary only if the original
            # instruction is not already in the dictionary.
            if (
                orig_inst.replace(".", "_") not in instr_dict
                or include_pseudo
                or name in include_pseudo_ops
            ):

                # update the final dict with the instruction
                if name not in instr_dict:
                    instr_dict[name] = single_dict
                    logging.debug(f"        including pseudo_ops:{name}")
                else:
                    if single_dict["match"] != instr_dict[name]["match"]:
                        instr_dict[name + "_pseudo"] = single_dict

                    # if a pseudo instruction has already been added to the filtered
                    # instruction dictionary but the extension is not in the current
                    # list, add it
                    else:
                        ext_name = single_dict["extension"]

                    if (ext_name not in instr_dict[name]["extension"]) & (
                        name + "_pseudo" not in instr_dict
                    ):
                        instr_dict[name]["extension"].extend(ext_name)
            else:
                logging.debug(
                    f"        Skipping pseudo_op {pseudo_inst} since original instruction {orig_inst} already selected in list"
                )

    # third pass if for imported instructions
    logging.debug("Collecting imported instructions")
    for f in file_names:
        logging.debug(f"Parsing File: {f} for imported ops")
        with open(f) as fp:
            lines = (line.rstrip() for line in fp)  # All lines including the blank ones
            lines = list(line for line in lines if line)  # Non-blank lines
            lines = list(
                line for line in lines if not line.startswith("#")
            )  # remove comment lines

        # go through each line of the file
        for line in lines:
            # if the an instruction needs to be imported then go to the
            # respective file and pick the line that has the instruction.
            # The variable 'line' will now point to the new line from the
            # imported file

            # ignore all lines starting with $import and $pseudo
            if "$import" not in line:
                continue
            logging.debug(f"     Processing line: {line}")

            (import_ext, reg_instr) = imported_regex.findall(line)[0]
            import_ext_file = f"{opcodes_dir}/{import_ext}"

            # check if the file of the dependent extension exist. Throw error if
            # it doesn't
            if not os.path.exists(import_ext_file):
                ext1_file = f"{opcodes_dir}/unratified/{import_ext}"
                if not os.path.exists(ext1_file):
                    logging.error(
                        f"Instruction {reg_instr} in {f} cannot be imported from {import_ext}"
                    )
                    raise SystemExit(1)
                else:
                    ext_file = ext1_file
            else:
                ext_file = import_ext_file

            # check if the dependent instruction exist in the dependent
            # extension. Else throw error.
            found = False
            for oline in open(ext_file):
                if not re.findall(f"^\\s*{reg_instr}\\s+", oline):
                    continue
                else:
                    found = True
                    break
            if not found:
                logging.error(
                    f"imported instruction {reg_instr} not found in {ext_file}. Required by {line} present in {f}"
                )
                logging.error(f"Note: you cannot import pseudo/imported ops.")
                raise SystemExit(1)

            # call process_enc_line to get the data about the current
            # instruction
            (name, single_dict) = process_enc_line(oline, f)

            # if an instruction has already been added to the filtered
            # instruction dictionary throw an error saying the given
            # instruction is already imported and raise SystemExit
            if name in instr_dict:
                var = instr_dict[name]["extension"]
                if instr_dict[name]["encoding"] != single_dict["encoding"]:
                    err_msg = f"imported instruction : {name} in "
                    err_msg += f"{os.path.basename(f)} is already "
                    err_msg += f"added from {var} but each have different encodings for the same instruction"
                    logging.error(err_msg)
                    raise SystemExit(1)
                instr_dict[name]["extension"].extend(single_dict["extension"])
            else:
                # update the final dict with the instruction
                instr_dict[name] = single_dict
    return instr_dict

def to_camel_case(text):
    s = text.replace("-", " ").replace("_", " ")
    s = s.split()
    if len(text) == 0:
        return text
    return s[0] + ''.join(i.capitalize() for i in s[1:])

def immediates():
    immediate_map = dict()
    for name, _ in arg_lut.items():
        if "imm" in name: 
            has_lo_or_hi = False 
            if "lo" in name or "hi" in name:
                name = name.replace("lo", "").replace("hi", "")
                has_lo_or_hi = True
                name += "lohi"
            
            encoder = f"encode_immediate(&{name.upper()}, {name} as _)"
            typ = f"{"u32" if 'u' in name else "i32"}"
            immediate_map[name] = (encoder, typ)
    return immediate_map
            

def make_encoders(instr_dict):
    """
    Make encoder functions which produce desired opcode. Generates code like 
    ```
    fn addi(rd: u32, rs1: u32, rs2: u32) -> u32 { ... }
    ```

    If immediates are used we still pass `u32` as argument. If immediates have `hi` and `lo`
    values then we move them to the end of arguments e.g `beq bimm12hi rs1 rs2 bimm12lo` becomes
    ```
    fn beq(rs1: u32, rs2: u32, bimm12hi: u32, bimm12lo: u32) -> u32 { ... }
    ```
    """
    code = ""
    asm_code = "pub trait EmitterExplicit: Emitter {\n"
    imms = immediates()
    for i in instr_dict:
        args: list[str] = instr_dict[i]["variable_fields"]

        actual_args: list[str] = []
        imm_args = []
        for arg in args:
            if "hi" in arg or "lo" in arg: 
                imm_args.append(arg)
                continue
            actual_args.append(arg)
        for imm in imm_args:
            actual_args.append(imm)
        short = i.lower().startswith('c')
        
        # array of <argname>, <typname>, <encode>
        args_types: list[(str, str, str)] = []
        imm_args: set[str] = set()
        for arg in args: 
            arg = arg.replace('=', '_eq_').replace(' ', '_')
            if "imm" in arg: 
                has_lo_hi = False
                if "lo" in arg or "hi" in arg: 
                    has_lo_hi = True
                    
                    arg = arg.replace("hi", "").replace("lo", "")
                    arg += "lohi"
                if arg in imm_args:
                    continue 
                imm_args.add(arg)
                encoder, arg_ty = imms[arg]
                args_types.append((arg, arg_ty, encoder))
            elif arg.startswith('R') | arg.startswith('r'):
                args_types.append((arg, 'Reg', f'inst.set_{arg}({arg}.0).value'))
            elif arg.startswith('V') | arg.startswith('v'):
                args_types.append((arg.lower(), 'VReg', f'inst.set_{arg.lower()}({arg.lower()}.0).value'))
            elif "aq" == arg or "rl" == arg: 
                args_types.append((arg.lower(), "bool", f"inst.set_{arg.lower()}({arg.lower()} as _).value")) 
            else: 
                args_types.append((arg, "u32", f"inst.set_{arg}({arg} as _).value"))
        typ = False 
        if short:
            typ = 'u16'
        else: 
            typ = 'u32'
        nargs = len(args_types)
        rem = 4 - nargs 
        code += f"""

        """

        asm_code += f"""
    pub fn {i.lower()}(&mut self, {", ".join(f"op{i}: impl OperandCast" for i in range(nargs))}) -> Result<(), AsmError> {{
        self.emit_n(Opcode::{i.upper().replace("_", "")} as i64, &[{','.join(f"op{i}.as_operand()" for i in range(nargs))}])
    }}
        """

    asm_code += "}"

    return code, asm_code
def make_immediates(instr_dict):
    """
    Emit `Immediate` type helpers 
    """




def make_rust(instr_dict):

    encoders,asm = make_encoders(instr_dict)
    mask_match_str = ""
    for i in instr_dict:
        mask_match_str += f'pub const MATCH_{i.upper().replace(".","_")}: u32 = {(instr_dict[i]["match"])};\n'
        mask_match_str += f'pub const MASK_{i.upper().replace(".","_")}: u32 = {(instr_dict[i]["mask"])};\n'
    for num, name in csrs + csrs32:
        mask_match_str += f"pub const CSR_{name.upper()}: u16 = {hex(num)};\n"
    for num, name in causes:
        mask_match_str += (
            f'pub const CAUSE_{name.upper().replace(" ","_")}: u8 = {hex(num)};\n'
        )
    short_count = 0

    for i in instr_dict:
        if i.lower().startswith('c'):
            short_count += 1

    rv32ext = set()
    rv64ext = set()

    for i in instr_dict:
        exts = instr_dict[i]["extension"]

        for ext in exts: 
            if ext.startswith('rv32'):
                rv32ext.add(i)
            elif ext.startswith('rv64'):
                rv64ext.add(i)
            else:
                # append to both lists, we can match these opcodes on both 32-bit and 64-bit ISA
                rv32ext.add(i)
                rv64ext.add(i)

    mask_match_str += f"pub static OPCODE32_MATCH: [u32; {len(instr_dict)}] = [\n"
    for i in instr_dict: 
        if i in rv32ext:
            mask_match_str += f"{instr_dict[i]["match"]}, /* {i} */\n"
        else: 
            mask_match_str += f"0xffff_ffff,/* {i} */\n"
    mask_match_str += "];\n"

    mask_match_str += f"pub static OPCODE32_MASK: [u32; {len(instr_dict)}] = [\n"
    for i in instr_dict: 
        if i in rv32ext:
            mask_match_str += f"{instr_dict[i]["mask"]}, /* {i} */\n"
        else: 
            mask_match_str += f"0xffff_ffff, /* {i} */\n"
    mask_match_str += "];\n"

    mask_match_str += f"pub static OPCODE64_MATCH: [u32; {len(instr_dict)}] = [\n"
    for i in instr_dict: 
        if i in rv64ext:
            mask_match_str += f"{instr_dict[i]["match"]}, /* {i} */\n"
        else: 
            mask_match_str += f"0xffff_ffff, /* {i} */\n"
    mask_match_str += "];\n"

    mask_match_str += f"pub static OPCODE64_MASK: [u32; {len(instr_dict)}] = [\n"
    for i in instr_dict: 
        if i in rv64ext:
            mask_match_str += f"{instr_dict[i]["mask"]}, /* {i} */\n"
        else: 
            mask_match_str += f"0xffff_ffff, /* {i} */\n"
    mask_match_str += "];\n"


    mask_match_str += f"pub static OPCODE_MATCH: [u32; {len(instr_dict)}] = [\n"
    for i in instr_dict:
        mask_match_str += f"{instr_dict[i]["match"]},\n"
    mask_match_str += "];\n"

    mask_match_str += f"pub static OPCODE_MASK: [u32; {len(instr_dict)}] = [\n"
    for i in instr_dict:
        mask_match_str += f"{instr_dict[i]["mask"]},\n"
    mask_match_str += "];"

    mask_match_str += f"\npub static OPCODE_MASK_COMPRESSED: [u16; {len(instr_dict)}] = [\n"
    for i in instr_dict:
        if i.lower().startswith('c'):
            mask_match_str += f"{int(instr_dict[i]['mask'], 16) & 0xffff},\n"
        else: 
            mask_match_str += f"0,\n"
    mask_match_str += "];\n"

    mask_match_str += f"\npub static OPCODE_MATCH_COMPRESSED: [u16; {len(instr_dict)}] = [\n"
    for i in instr_dict:
        if i.lower().startswith('c'):
            mask_match_str += f"{int(instr_dict[i]['match'], 16) & 0xffff},\n"
        else: 
            mask_match_str += f"0,\n"
    mask_match_str += "];\n"


    mask_match_str += f"\npub static ALL_OPCODES: [Opcode; {len(instr_dict)}] = [\n"
    for i in instr_dict: 
        mask_match_str += f"Opcode::{i.upper().replace("_", "")},\n"
    mask_match_str += "];"

    mask_match_str += f"\npub static SHORT_OPCODE: [bool; {len(instr_dict)}] = [\n"
    for i in instr_dict:
        mask_match_str += f"{str(i.lower().startswith('c')).lower()},\n"
    mask_match_str += "];\n"

    mask_match_str += f"pub const SHORT_OPCODES: [Opcode; {short_count}] = [\n"
    for i in instr_dict:
        if i.lower().startswith('c'):
            mask_match_str += f"Opcode::{i.upper().replace("_", "")},\n"
    mask_match_str += "];\n"

    enum_opcode = ""
    enum_opcode += "#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]\n#[repr(u32)]\n"
    enum_opcode += "pub enum Opcode {\n"
    for i in instr_dict:
        enum_opcode += "    "
        enum_opcode += i.upper().replace("_", "")
        enum_opcode += ",\n"
    enum_opcode += "    Invalid\n"
    enum_opcode += "}\n\n"

    enum_opcode += "pub const OPCODE_STR: &[&str] = &[\n"
    for i in instr_dict:
        enum_opcode += f"    \"{i.lower().replace("c_", "c.").replace("cm_", "cm.").replace("_", ".")}\",\n"
    enum_opcode += "    \"<invalid>\"\n"
    enum_opcode += "];\n"
    inst = """
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Inst {
    pub opcode: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub csr: i64,
    pub funct7: u32,
}

impl Inst {
    pub const fn encode(&self) -> InstructionValue {
        InstructionValue::new(
            0
            | (self.funct7 << 25)
            | (self.rs2 << 20)
            | (self.rs1 << 15)
            | (self.funct3 << 12)
            | self.opcode)
    }

    pub const fn new(op: Opcode) -> Self {
        match op {
            Opcode::Invalid => unreachable!(),
    """
    for i in instr_dict:
        enc_match = int(instr_dict[i]["match"], 0)
        opcode = (enc_match >> 0) & ((1 << 7) - 1)
        funct3 = (enc_match >> 12) & ((1 << 3) - 1)
        rs1 = (enc_match >> 15) & ((1 << 5) - 1)
        rs2 = (enc_match >> 20) & ((1 << 5) - 1)
        csr = (enc_match >> 20) & ((1 << 12) - 1)
        funct7 = (enc_match >> 25) & ((1 << 7) - 1)
        inst += f""" Opcode::{i.upper().replace("_", "")} => Inst {{
            opcode: {hex(opcode)},
            funct3: {hex(funct3)},
            rs1: {hex(rs1)},
            rs2: {hex(rs2)},
            csr: {hex(csr)},
            funct7: {hex(funct7)}
        }},\n"""
    inst += "}}\n}\n"

    
    arg_str = ""

    insn_value = """
    /// InstructionValue contains the 32-bit instruction value and also provides access into the desired field.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct InstructionValue {
        pub value: u32,
    }

    impl InstructionValue {
        pub const fn new(value: u32) -> Self {
            Self { value }
        }

        pub const fn field<const FIELD_START: usize, const FIELD_SIZE: usize>(self) -> u32 {
            (self.value >> FIELD_START) & ((1 << FIELD_SIZE) - 1)
        }
        

    """

    # a dict of encodings. e.g (rs1, rd, imm12hi)
    encodings = dict()

    for i in instr_dict:
        args: list[str] = instr_dict[i]["variable_fields"]

        encoding = to_camel_case("_".join(i.replace('=', '_eq_').title() for i in args))
        ops = False
        if encoding in encodings:
            ops = encodings[encoding]
        else:
            ops = []
            encodings[encoding] = ops

        ops.append(i.upper().replace("_", ""))
    if '' in encodings:
        ops = encodings['']
        del encodings['']
        encodings['Empty'] = ops

    enc = """
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Encoding {
    """

    for e in sorted(encodings):
        enc += e
        enc += ",\n"
    enc += "}\n"

    enc += """
impl Opcode {
    pub fn encoding(self) -> Encoding {
        use Opcode::*;
        match self {
            Opcode::Invalid => unreachable!(),
"""
    for e in sorted(encodings):
        ops = encodings[e]
        first = ops[0]
        enc += f"  {first}\n"
        for op in ops[1:]:
            enc += f"  | {op}\n"
        enc += f"  => Encoding::{e},\n"
    enc += "}}}"
    
           
    for name, rng in arg_lut.items():
        sanitized_name = name.replace(" ", "_").replace("=", "_eq_")

        begin = rng[1]
        end = rng[0]
        mask = ((1 << (end - begin + 1)) - 1) << begin
        field = f"INSN_FIELD_{sanitized_name.upper()}"
        arg_str += f"pub const {field}: u32 = {hex(mask)};\n"
        arg_str += f"pub const {field}_START: u32 = {begin};\n"
        arg_str += f"pub const {field}_SIZE: u32 = {(end - begin) + 1};\n"

        if "imm" in sanitized_name:
            sanitized_name += "_raw"

        insn_value += f"pub const fn {sanitized_name}(self) -> u32 {{ (self.value >> {field}_START) & ((1 << {field}_SIZE) - 1) }}\n"
        insn_value += f"""
    pub const fn set_{sanitized_name}(mut self, value: u32) -> Self {{
        let mask = {field};
        

        self.value &= !mask;
        self.value |= (value & ((1 << {field}_SIZE) - 1)) << {field}_START;
        self
    }}
        """

    imms = immediates()

    for name in imms.keys():
        encoder,ty = imms[name]
        sanitized_name = name.replace(" ", "_").replace("=", "_eq_")


        insn_value += f"""
    /// {name}
    pub const fn {sanitized_name}(self) -> {ty} {{
        decode_immediate(&{sanitized_name.upper()}, self.value as _) as _
    }}

    pub const fn set_{sanitized_name}(mut self, {name}: {ty}) -> Self {{
        self.value |= encode_immediate(&{sanitized_name.upper()}, {name} as _);
        self
    }}
        """

    insn_value += "}\n"


    asm_file = open("inst_impl.rs", "w")
    asm_file.write(
        f"""
        /* Automatically generated by parse_opcodes */
{asm}
        """
    )
    rust_file = open("inst.rs", "w")
    rust_file.write(
        f"""
/* Automatically generated by parse_opcodes */

{mask_match_str}
{enum_opcode}
{inst}
{enc}
{arg_str}
{insn_value}
{encoders}

"""
    )
    rust_file.close()



def signed(value, width):
    if 0 <= value < (1 << (width - 1)):
        return value
    else:
        return value - (1 << width)


if __name__ == "__main__":
    print(f"Running with args : {sys.argv}")

    extensions = sys.argv[1:]
    for i in ["-c", "-latex", "-chisel", "-sverilog", "-rust", "-go", "-spinalhdl"]:
        if i in extensions:
            extensions.remove(i)
    print(f"Extensions selected : {extensions}")

    include_pseudo = True
    instr_dict = create_inst_dict(extensions, include_pseudo)

    instr_dict = collections.OrderedDict(sorted(instr_dict.items()))
    if '-decode' in sys.argv[1:]:
        print('decoder')
    if "-rust" in sys.argv[1:]:
        make_rust(instr_dict)
        logging.info("inst.rs generated successfully")

