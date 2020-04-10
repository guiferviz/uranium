
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"


using namespace llvm;


static LLVMContext ctx;
static Module mainModule("uranium_module", ctx);


int main(int argc, char *argv[]) {
    mainModule.dump();
    return 0;
}

