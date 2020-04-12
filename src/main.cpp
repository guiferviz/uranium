
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"


int main(int argc, char *argv[]) {
    // Create the minimum objects needed to generate some working code.
    // An LLVM Context is very similar to programming scopes. It gives you
    // isolation. Modules, functions and types defined in one context
    // cannot be used in other contexts.
    llvm::LLVMContext ctx;
    // Top level structure present in LLVM programs.
    llvm::Module mainModule("uranium_module", ctx);
    // This is only for fun. Let's simulate that our code comes from a file
    // called 'main.ura'.
    // Yes, '*.ura' is the extension for Uranium source code files :)
    // If we do not set the source filename the name of the module is going
    // to be used.
    mainModule.setSourceFileName("main.ura");

    // Write the IR code of our empty module to stderr.
    // Why stderr? I don't know indeed...
    mainModule.dump();

    return 0;
}

