
fn main()
{
    // Compile and add src/system/handler.c to the kernel
    cc::Build::new()
        .file("src/system/syscall_handler.c")
        .compile("handler");
}