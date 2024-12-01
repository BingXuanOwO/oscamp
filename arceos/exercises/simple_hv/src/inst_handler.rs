//! I over thinked everything.
//! I may even didn't need to add this new file...
//! but I just want to... make everything more robust... and useable
//! but it's just an exercise... it won't be THAT COMPLEXED
//! and the result makes no sence.
use core::arch::asm;

// used chatGPT to pass all riscv arch theory, and GPT generated some of the code.
use crate::vcpu::VmCpuRegisters;
use riscv::register::{scause, sstatus, stval};
use bitflags::bitflags;


struct RvInst32(usize);

impl RvInst32 {
    // 获取和设置每个字段的函数
    pub fn opcode(&self) -> usize {
        self.0 & 0x7F // 取低 7 位 [6:0]
    }

    pub fn rd(&self) -> usize {
        (self.0 >> 7) & 0x1F // [11:7]
    }

    pub fn funct3(&self) -> usize {
        (self.0 >> 12) & 0x7 // [14:12]
    }

    pub fn rs1(&self) -> usize {
        (self.0 >> 15) & 0x1F // [19:15]
    }

    pub fn rs2(&self) -> usize {
        (self.0 >> 20) & 0x1F // [24:20]
    }

    pub fn funct7(&self) -> usize {
        (self.0 >> 25) & 0x7F // [31:25]
    }

    /// will use when inst acts as csr inst
    pub fn extract_csr(&self) -> usize {
        (self.0 >> 20) & 0xFFF
    }
}

bitflags! {
    #[derive(PartialEq)]
    struct Opcodes: usize {
        /// CSR Operations
        const CSR = 0b1110011;
    }
}

pub fn is_inst_32bit(inst: usize) -> bool {
    (inst & 0x3) == 0x3
}

pub fn bad_inst_exception_handler(
    inst: usize, 
    ctx: &mut VmCpuRegisters
) -> Result<(), ()> {
    if !is_inst_32bit(inst) {
        return Err(());
    }

    let parsed_inst = RvInst32(inst);
    ax_println!("{}", parsed_inst.opcode());
    
    match Opcodes::from_bits(parsed_inst.opcode()).unwrap_or(Opcodes::empty()) {
        Opcodes::CSR=>{
            csr_inst_handler(parsed_inst, ctx)
        },
        _=> Err(())
    }
}

fn csr_inst_handler(
    inst: RvInst32, 
    ctx: &mut VmCpuRegisters
) -> Result<(), ()> {
    if inst.funct3() != 0x2 {
        return Err(());
    }
    // ctx.trap_csrs.htinst
    Ok(())
}

