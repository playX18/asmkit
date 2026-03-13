# A simple parser for the AsmJIT declarations of arm64 instructions. It converts
# them into a more structured format that can be used to generate Rust code.

import os
import re

from docenizer_arm64 import collect_instruction_docs


CC_VARIANTS = ["eq", "ne", "cs", "hs", "cc", "lo", "mi", "pl", "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le", "al"]
DOCS_INPUT = os.environ.get("ASMKIT_ARM64_DOCS", "asm-docs-arm")
RUST_KEYWORDS = {
    "yield",
}


class Opcode:
    def __init__(self, name, inst_type):
        self.name = name
        self.variants = []
        self.inst_type = inst_type


def load_opcode_docs(inputfolder):
    try:
        return collect_instruction_docs(inputfolder)
    except Exception as exc:
        print(f"Warning: failed to load ARM64 docs from {inputfolder}: {exc}")
        return {}


def parse_opcodes(path):
    opcodes = {}
    inst_type = ""

    with open(path, 'r') as f:
        for raw_line in f:
            line = raw_line.strip()
            if not line:
                continue

            if line.startswith('ASMJIT_INST_'):
                inst_type = line[len('ASMJIT_INST_'):]

            if '(' not in line:
                continue

            elems = line.split('(', 1)[1].split(')', 1)[0].split(',')
            if len(elems) < 2:
                continue

            name = elems[0]
            inst_id = elems[1]
            operands = elems[2:]

            if name not in opcodes:
                opcodes[name] = Opcode(name, inst_type)
                opcodes[name].variants.append((inst_id, operands))
                continue

            if len(operands) != len(opcodes[name].variants[0][1]):
                new_name = f"{name}_{len(operands)}"
                if new_name not in opcodes:
                    opcodes[new_name] = Opcode(new_name, inst_type)
                opcodes[new_name].variants.append((inst_id, operands))
            else:
                opcodes[name].variants.append((inst_id, operands))

    return opcodes


def trait_name(name):
    camel_case_name = ''.join(word.capitalize() for word in name.split('_'))
    if name == 'mvn_':
        return 'Mvn_'
    return camel_case_name


def rust_method_name(name):
    if name in RUST_KEYWORDS:
        return f"r#{name}"
    return name


def doc_lookup_name(name):
    return re.sub(r'(_\d+|_)+$', '', name).upper()


def write_doc_block(out, name, opcode_docs, indent=''):
    canonical_name = doc_lookup_name(name)
    doc = opcode_docs.get(canonical_name)
    if doc:
        out.write(f"{indent}/// Emits `{name.upper()}` (`{canonical_name}`). {doc['tooltip']}\n")
        if doc.get('url'):
            out.write(f"{indent}/// Reference: [Arm docs for {canonical_name}]({doc['url']})\n")
    else:
        out.write(f"{indent}/// Emits `{name.upper()}`.\n")


def main():
    opcodes = parse_opcodes('meta/arm64.txt')
    opcode_docs = load_opcode_docs(DOCS_INPUT)

    with open('emitter.rs', 'w') as out:
        for name, opcode in opcodes.items():
            generics = [f"T{i}" for i in range(len(opcode.variants[0][1]))]
            generics_str = f"<{', '.join(generics)}>" if generics else ""
            operands_str = [f"op{i}: T{i}" for i in range(len(opcode.variants[0][1]))]
            method_name = rust_method_name(name)

            write_doc_block(out, name, opcode_docs)
            out.write(f"pub trait {trait_name(name)}Emitter{generics_str} {{\n")
            if opcode.inst_type.startswith("1cc("):
                out.write(f"\tfn {method_name}(&mut self, {','.join(operands_str)});\n")
                for cc in CC_VARIANTS:
                    out.write(f"\tfn {method_name}_{cc}(&mut self, {','.join(operands_str)});\n")
            else:
                out.write(f"\tfn {method_name}(&mut self, {','.join(operands_str)});\n")
            out.write("}\n\n")

        for name, opcode in opcodes.items():
            method_name = rust_method_name(name)
            for inst_id, operands in opcode.variants:
                generics_str = f"<{', '.join(operands)}>" if operands else ""
                operands_str = [f"op{i}: {operands[i]}" for i in range(len(operands))]
                as_operands = [f"op{i}.as_operand()" for i in range(len(operands))]
                out.write(f"impl {trait_name(name)}Emitter{generics_str} for Assembler<'_> {{\n")
                if opcode.inst_type.startswith("1cc("):
                    out.write(f"\tfn {method_name}(&mut self, {','.join(operands_str)}) {{\n")
                    out.write(f"\t\tself.emit_n(InstId::{inst_id}, &[{', '.join(as_operands)}]);\n")
                    out.write("\t}\n")
                    for cc in CC_VARIANTS:
                        out.write(f"\tfn {method_name}_{cc}(&mut self, {','.join(operands_str)}) {{\n")
                        out.write(f"\t\tself.emit_n(InstId::{inst_id}.with_cc(CondCode::{cc.upper()}), &[{', '.join(as_operands)}]);\n")
                        out.write("\t}\n")
                else:
                    out.write(f"\tfn {method_name}(&mut self, {','.join(operands_str)}) {{\n")
                    out.write(f"\t\tself.emit_n(InstId::{inst_id}, &[{', '.join(as_operands)}]);\n")
                    out.write("\t}\n")
                out.write("}\n\n")

        out.write("impl Assembler<'_> {\n")
        for name, opcode in opcodes.items():
            generics = [f"T{i}" for i in range(len(opcode.variants[0][1]))]
            generics_str = f"<{', '.join(generics)}>" if generics else ""
            operands_str = [f"op{i}: T{i}" for i in range(len(opcode.variants[0][1]))]
            call_args = ','.join(f'op{i}' for i in range(len(opcode.variants[0][1])))
            method_name = rust_method_name(name)
            trait = f"{trait_name(name)}Emitter{generics_str}"

            if opcode.inst_type.startswith("1cc("):
                for cc in CC_VARIANTS:
                    variant_name = f"{method_name}_{cc}"
                    write_doc_block(out, f"{name}_{cc}", opcode_docs, indent='\t')
                    out.write(f"\tpub fn {variant_name}{generics_str}(&mut self, {','.join(operands_str)}) where Self: {trait} {{\n")
                    out.write(f"\t\t<Self as {trait}>::{variant_name}(self, {call_args});\n")
                    out.write("\t}\n")
            else:
                write_doc_block(out, name, opcode_docs, indent='\t')
                out.write(f"\tpub fn {method_name}{generics_str}(&mut self, {','.join(operands_str)}) where Self: {trait} {{\n")
                out.write(f"\t\t<Self as {trait}>::{method_name}(self, {call_args});\n")
                out.write("\t}\n")
        out.write("}\n")


if __name__ == '__main__':
    main()