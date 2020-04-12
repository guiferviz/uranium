
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"


using namespace llvm;


int main(int argc, char *argv[]) {
    // Create the minimum objects needed to generate some working code.
    // An LLVM Context is very similar to programming scopes. It gives you
    // isolation. Modules, functions and types defined in one context
    // cannot be used in other contexts.
    LLVMContext ctx;
    // Top level structure present in LLVM programs.
    // Now using a pointer.
    Module *mainModule = new Module("uranium_module", ctx);
    // This is only for fun. Let's simulate that our code comes from a file
    // called 'main.ura'.
    // Yes, '*.ura' is the extension for Uranium source code files :)
    // If we do not set the source filename the name of the module is going
    // to be used.
    mainModule->setSourceFileName("main.ura");

    // Create void type and "function that returns void" type.
    // All this types need to be defined on the created context.
    Type *void_type = Type::getVoidTy(ctx);
    // This constructor does not ask for a context because it asks for
    // a type that already has a context associated.
    // The first parameter is the return type of the function.
    // The second parameter is a boolean that indicates if the number of
    // arguments is variable. Why a function that does not take parameters
    // needs this? No idea yet.
    FunctionType *fun_void_type = FunctionType::get(void_type, false);

    // Write the IR code of our empty module to stderr.
    // Why stderr? I don't know indeed...
    mainModule->dump();

    return 0;
}

