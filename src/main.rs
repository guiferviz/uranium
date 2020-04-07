
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
        // Top level structure present in LLVM programs.
        // http://llvm.org/docs/ProgrammersManual.html#the-module-class
        let module = llvm::core::LLVMModuleCreateWithName(c_str!("main"));

        // Write our empty module to a bitcode file.
        llvm::bit_writer::LLVMWriteBitcodeToFile(module, c_str!("main.bc"));

        // Dispose module (calling destructor method).
        llvm::core::LLVMDisposeModule(module);
    }

    println!("Done!")
}

