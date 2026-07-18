// Minimal asmjit build/emit smoke test for asmkit's differential validation.
//
// Proves the pinned asmjit clone (meta/asmjit) compiles standalone and emits
// expected bytes for a few x86-64 instructions. The full corpus generator
// (P2 differential validation) builds on this.
//
// Build (from repo root):
//   g++ -std=c++17 -O1 -DASMJIT_STATIC -I meta/asmjit \
//     meta/golden/smoke.cpp \
//     meta/asmjit/asmjit/core/*.cpp meta/asmjit/asmjit/support/*.cpp \
//     meta/asmjit/asmjit/x86/*.cpp meta/asmjit/asmjit/arm/*.cpp \
//     -o target/x86_golden_smoke
#include <asmjit/core.h>
#include <asmjit/x86.h>
#include <cstdio>

using namespace asmjit;

static void dump(const char* name, CodeHolder& code) {
  const CodeBuffer& buf = code.text_section()->buffer();
  printf("%-12s", name);
  for (size_t i = 0; i < buf.size(); i++)
    printf("%02x", buf.data()[i]);
  printf("\n");
}

int main() {
  Environment env(Arch::kX64);

  {
    CodeHolder code;
    code.init(env);
    x86::Assembler a(&code);
    a.add(x86::rax, x86::rcx);       // 48 01 c8
    a.mov(x86::eax, 42);             // b8 2a 00 00 00
    a.vaddps(x86::xmm0, x86::xmm1, x86::xmm2); // c5 e8 58 c2
    a.ret();
    dump("base", code);
  }

  {
    CodeHolder code;
    code.init(env);
    x86::Assembler a(&code);
    a.vaddps(x86::zmm0, x86::zmm1, x86::zmm2); // 62 f1 6c 48 58 c2
    dump("evex", code);
  }

  return 0;
}
