
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"


int main(int argc, char *argv[]) {
    // Create the minimum objects needed to generate some working code.
    // An LLVM Context is very similar to programming scopes. It gives you
    // isolation. Modules, functions of types defined in one context
    // cannot be used in other contexts.
    // http://llvm.org/docs/ProgrammersManual.html#achieving-isolation-with-llvmcontext
    llvm::LLVMContext ctx;
    // Top level structure present in LLVM programs.
    // https://releases.llvm.org/10.0.0/docs/LangRef.html#module-structure
    llvm::Module mainModule("uranium_module", ctx);

    // Write the IR code of our empty module to stdout.
    mainModule.dump();

    return 0;
}

