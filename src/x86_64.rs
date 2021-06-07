/// **Inline assembly demo for x86_64**. Note that x86 has the two main syntax flavors:
/// AT&T syntax and Intel syntax, see https://en.wikipedia.org/wiki/X86_assembly_language#Syntax
/// for more details.
///
/// For `llvm_asm` AT&T-syntax is used by default. https://doc.rust-lang.org/nightly/unstable-book/library-features/llvm-asm.html
/// For `asm!` GAS-Syntax (AT&T-adoption by GNU Assembler) is used by default with the
/// `.intel_syntax noprefix mode` of GAS.
/// https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#asm
///
/// To get an overview over the registers (eax, rax, ax, ...) see
/// https://en.wikibooks.org/wiki/X86_Assembly/X86_Architecture
pub fn run_demos() {
    // we use this vars as destination variables for reading data/registers via llvm_asm.
    let mut result_8: u8;
    let mut result_16: u16;
    let mut result_32: u32;
    let mut result_64: u64;

    // Modern `asm!`-examples.
    unsafe {
        // #####################################################################
        // ### Store immediate "2" in eax (32 bit) read the value.
        {
            asm!("mov eax, 17", out("eax") result_32);
            // works also with "$" prefix (AT&T syntax)
            // asm!("mov eax, $17", out("eax") result_32);
            assert_eq!(result_32, 17);
            // println!("eax = {}", result_32);
        }
        // #####################################################################
        // ### Store variable value in rax (fetch from memory) & read the value.
        {
            let input = 0xff00000000000000_u64;
            asm!("mov rax, {0}", in(reg) input, out("rax") result_64);

            // ALTERNATIVELY: this fetches the value from memory during runtime
            // asm!("mov rax, [{0}]", in(reg) &input, out("rax") result_64);
            // whereas the above will put the value into a register first

            assert_eq!(result_64, input);
            // println!("rax = {}", result_64);
        }
        // #####################################################################
        // ### Store 64 bit value and read "part registers" one by one (rax => eax, ax, ah, al).
        // https://en.wikibooks.org/wiki/X86_Assembly/X86_Architecture
        {
           asm!(
                "mov rax, {input_number}",
                input_number = const 0x12_34_56_78_90_ab_cd_ef_u64);
            asm!("", out("rax") result_64);
            asm!("", out("eax") result_32);
            asm!("", out("ax") result_16);
            // not possible on x86(_64): https://stackoverflow.com/questions/45500399/
            // to use higher address registers as operand
            // asm!("", out("ah") result_8_1);
            asm!("", out("al") result_8);
            assert_eq!(result_64, 0x12_34_56_78_90_ab_cd_ef);
            assert_eq!(result_32, 0x90_ab_cd_ef);
            assert_eq!(result_16, 0xcd_ef);
            assert_eq!(result_8, 0xef);
        }
    }

    // Legacy `llvm_asm!`-examples.
    unsafe {
        // #####################################################################
        // ### store immediate "2" in eax (32 bit) in Intel syntax and read the value
        {
            llvm_asm!("mov eax, 2" : "={eax}"(result_32) : : : "intel");
            assert_eq!(result_32, 2);
            // println!("eax = {}", result_32);
        }

        // #####################################################################
        // ### store immediate "3" in eax (32 bit) in AT&T syntax and read the value
        {
            // note: registers need "%" prefix
            // and immediate "$$" prefix (TODO I don't know why)
            llvm_asm!("movl $$3, %eax" : "={eax}"(result_32) : :);
            assert_eq!(result_32, 3);
            // println!("eax = {}", result_32);
        }

        // #####################################################################
        // ### store immediate "4" in eax (32 bit) in AT&T syntax; read in separate step
        {
            // store
            llvm_asm!("movl $$4, %eax" : : :);
            // read
            llvm_asm!("" : "={eax}"(result_32) : :);

            assert_eq!(result_32, 4);
            // println!("eax = {}", result_32);
        }

        // #####################################################################
        // ### store a (runtime) value in rax (64 bit) in AT&T syntax; read in separate step
        {
            let input = 0xfe_00_fe_00_fe_00_fe_00;

            // store
            // $0 is a placeholder
            // the first input operand (started with prefix "r) will be the value for "$0".
            // for more details, see:
            // https://stackoverflow.com/questions/3589157/what-is-r-and-double-percent-in-gcc-inline-assembly-language
            llvm_asm!("movq $0, %rax" : : "r"(input) :);

            // read
            llvm_asm!("" : "={rax}"(result_64) : :);

            assert_eq!(result_64, input);
            // println!("rax = 0x{:x}", result_64);
        }
    }
}
