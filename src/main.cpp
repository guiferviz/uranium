
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/IRBuilder.h"


using namespace llvm;


int main(int argc, char *argv[]) {
    // Create the minimum objects needed to generate some working code.
    // An LLVM Context is very similar to programming scopes. It gives you
    // isolation. Modules, functions and types defined in one context
    // cannot be used in other contexts.
    LLVMContext ctx;
    // Top level structure present in LLVM programs.
    Module mainModule("uranium_module", ctx);
    // This is only for fun. Let's simulate that our code comes from a file
    // called 'main.ura'.
    // Yes, '*.ura' is the extension for Uranium source code files :)
    // If we do not set the source filename the name of the module is going
    // to be used.
    mainModule.setSourceFileName("main.ura");

    // Create void type and "function that returns void" type.
    // All this types need to be defined on the created context.
    Type *voidType = Type::getVoidTy(ctx);
    // This constructor does not ask for a context because it asks for
    // a type that already has a context associated.
    // The first parameter is the return type of the function.
    // The second parameter is a boolean that indicates if the number of
    // arguments is variable.
    FunctionType *funVoidType = FunctionType::get(voidType, false);

    // Create main function associated to the mainModule.
    // The linkage defines the visibility of that function during the linkage
    // phase, that is, the phase in which the objects files are merged into
    // one executable.
    // We indicate that we want that function to be present in that linkage.
    Function *mainFun = Function::Create(
            funVoidType, Function::ExternalLinkage, "main", mainModule);

    // Creating a block to write the body of the function.
    // While creating functions the order does not matter, a builder is like a
    // cursor in our text editor, allow us to write a sequence of instructions.
    IRBuilder<> builder(ctx);
    // A block is where the builder will write the code.
    BasicBlock *mainBlock = BasicBlock::Create(ctx, "main_block", mainFun);
    // Place cursor at the beggining of the main body.
    builder.SetInsertPoint(mainBlock);
    // Add "return void;" statement.
    builder.CreateRetVoid();

    // Write the IR code of our empty module to stderr.
    // Why stderr? I don't know indeed...
    mainModule.dump();

    return 0;
}

