
// llvm_sys is a non-official library that brings the C++ API of LLVM to Rust.
extern crate llvm_sys as llvm;


/**
 * To use C libraries from Rust we need to transform Rust strings into a
 * a pointer to an array that contains a null-terminated sequence of characters
 * (i.e., a C-string) representing the current value of the string object.
 *
 * This macro performs the same operation as the C++ `std::string::c_str`
 * method.
 *
 * The operation is safe, but derreferencieting the pointer is unsafe.
 */
macro_rules! c_str {
    ($s:expr) => (
        // Static strings (type str in Rust) are a collection of bytes
        // included in the binary file (the compiled file), they have a pointer
        // to those bytes and a length.
        // The output of the concat! macro is also an str.
        // This concatenation is indeed computed at compile time.
        // as_ptr() returns a raw pointer of type `*const u8` that points to
        // first position of the bytes array that contains the data.
        // We need to cast it to `*const libc::c_char`, use `*const i8` that
        // is how libc::c_char is defined or use `*const _` so the compiler can
        // infer the type.
        // If you use the c_char definition of libc, you need to import the
        // library using `extern crate libc;`
        concat!($s, "\0").as_ptr() as *const _
    );
}


fn main() {
    println!("Hello, LLVM! I'm going to generate some code... please, wait.");

    // Usafe block because we are calling external functions.
    // Rust's memory management at compile time is not available in external
    // libraries, so Rust treats it as unsafe.
    unsafe {
        // Create the minimum objects needed to generate some working code.
        // An LLVM Context is very similar to programming scopes. It gives you
        // isolation. Modules, functions of types defined in one context
        // cannot be used in other contexts.
        // http://llvm.org/docs/ProgrammersManual.html#achieving-isolation-with-llvmcontext
        let context = llvm::core::LLVMContextCreate();
        // Top level structure present in LLVM programs.
        // http://llvm.org/docs/ProgrammersManual.html#the-module-class
        let module = llvm::core::LLVMModuleCreateWithName(c_str!("main"));
        // Create an intermediate code builder.
        // We can see this object as a cursor. We can create functions and
        // modules without this object, but when we want to write the body
        // of those functions we need this object.
        // Useful for creating strings, constants, returns statements...
        let builder = llvm::core::LLVMCreateBuilderInContext(context);

        // Create u8 type and "function that returns u8" type.
        // All this types need to be defined on the created context.
        // Indeed, LLVM does not make distinction between unsigned integer and
        // signed.
        // Some notes from Chris Lattner writing about the reasons:
        // http://nondot.org/~sabre/LLVMNotes/TypeSystemChanges.txt
        // Basically it says that the code get more complex, the IR to high
        // level and having that distinction doesn't provide really good
        // things.
		let u8_type = llvm::core::LLVMIntTypeInContext(context, 8);
        // This constructor does not ask for a context because it asks for
        // an already defined type that already has a context.
        // The first parameter is the return type of the function.
        // The second one are the list of arguments.
        // In this case, we do not want arguments, so we provide a pointer
        // pointing to null. The pointer must be mutable.
        // The number of parameters is indicated in the third parameter.
        // The last parameter is a boolean that indicates if the number of
        // arguments is variable. We use 0 as false.
        let fun_void_type = llvm::core::LLVMFunctionType(
            u8_type, std::ptr::null_mut(), 0, 0);

        // Create a main function that does nothing.
        #[warn(unused_variables)]
        let main_fun = llvm::core::LLVMAddFunction(
            module, c_str!("main"), fun_void_type);

        // Create a body in the main function. We need to return void.
        let bb = llvm::core::LLVMAppendBasicBlockInContext(
            context, main_fun, c_str!("main_body"));
        // Moves the builder (moves the cursor) at the end of that new block
        // so we can start writing statements there.
        llvm::core::LLVMPositionBuilderAtEnd(builder, bb);
		// Create a constant value that we will use in the return.
		let u8_val = llvm::core::LLVMConstInt(u8_type, 9, 0);
        // Add a "return u8_val;" statement in the main body.
        llvm::core::LLVMBuildRet(builder, u8_val);

        // Write our empty module to a bitcode file.
        llvm::bit_writer::LLVMWriteBitcodeToFile(module, c_str!("main.bc"));

        // Dispose objects (calling destructor method).
        llvm::core::LLVMDisposeModule(module);
        llvm::core::LLVMContextDispose(context);
    }

    println!("Done!");
}

