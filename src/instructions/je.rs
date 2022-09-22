use iced_x86::Code::*;
use iced_x86::Instruction;
use iced_x86::Mnemonic::Je;
use iced_x86::OpKind;
use iced_x86::Register::*;

use super::axecutor::Axecutor;
use super::errors::AxError;
use crate::instructions::flags::*;

impl Axecutor {
    pub fn mnemonic_je(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.mnemonic(), Je);

        match i.code() {
            Je_rel8_16 => self.instr_je_rel8_16(i),
            Je_rel8_32 => self.instr_je_rel8_32(i),
            Je_rel8_64 => self.instr_je_rel8_64(i),
            Jecxz_rel8_16 => self.instr_jecxz_rel8_16(i),
            Jecxz_rel8_32 => self.instr_jecxz_rel8_32(i),
            Jecxz_rel8_64 => self.instr_jecxz_rel8_64(i),
            Je_rel16 => self.instr_je_rel16(i),
            Je_rel32_32 => self.instr_je_rel32_32(i),
            Je_rel32_64 => self.instr_je_rel32_64(i),
            _ => panic!("Invalid instruction code {:?} for mnemonic Je", i.code()),
        }
    }

    /// JE rel8
    ///
    /// o16 74 cb
    fn instr_je_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Je_rel8_16);

        todo!("instr_je_rel8_16 for Je")
    }

    /// JE rel8
    ///
    /// o32 74 cb
    fn instr_je_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Je_rel8_32);

        todo!("instr_je_rel8_32 for Je")
    }

    /// JE rel8
    ///
    /// o64 74 cb
    fn instr_je_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Je_rel8_64);

        if self.state.rflags & FLAG_ZF != 0 {
            match i.op0_kind() {
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP.into(), offset);
                }
                _ => panic!("Invalid op0_kind {:?} for JE rel8_64", i.op0_kind()),
            }
        }

        Ok(())
    }

    /// JECXZ rel8
    ///
    /// a32 o16 E3 cb
    fn instr_jecxz_rel8_16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jecxz_rel8_16);

        todo!("instr_jecxz_rel8_16 for Je")
    }

    /// JECXZ rel8
    ///
    /// a32 o32 E3 cb
    fn instr_jecxz_rel8_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jecxz_rel8_32);

        todo!("instr_jecxz_rel8_32 for Je")
    }

    /// JECXZ rel8
    ///
    /// a32 o64 E3 cb
    fn instr_jecxz_rel8_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Jecxz_rel8_64);

        todo!("instr_jecxz_rel8_64 for Je")
    }

    /// JE rel16
    ///
    /// o16 0F 84 cw
    fn instr_je_rel16(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Je_rel16);

        todo!("instr_je_rel16 for Je")
    }

    /// JE rel32
    ///
    /// o32 0F 84 cd
    fn instr_je_rel32_32(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Je_rel32_32);

        todo!("instr_je_rel32_32 for Je")
    }

    /// JE rel32
    ///
    /// o64 0F 84 cd
    fn instr_je_rel32_64(&mut self, i: Instruction) -> Result<(), AxError> {
        debug_assert_eq!(i.code(), Je_rel32_64);

        if self.state.rflags & FLAG_ZF != 0 {
            match i.op0_kind() {
                OpKind::NearBranch32 => {
                    let offset = i.near_branch32() as i32 as u64;
                    self.reg_write_64(RIP.into(), offset);
                }
                OpKind::NearBranch64 => {
                    let offset = i.near_branch64() as i64 as u64;
                    self.reg_write_64(RIP.into(), offset);
                }
                _ => panic!("Invalid op0_kind {:?} for JE rel32_64", i.op0_kind()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::axecutor::Axecutor;
    use crate::{assert_reg_value, instructions::registers::SupportedRegister, jmp_test};
    use iced_x86::Register::*;

    jmp_test![mov_rax_3_cmp_rax_3_je_end_mov_rax_42_end_nop_pf_zf;
        start: 0x401010; end: 0x4046af;
        0x48, 0xc7, 0xc0, 0x3, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x3, 0xf, 0x84, 0x8d, 0x36, 0x0, 0x0; // mov rax, 3; cmp rax, 3; je .end
        13958; // 13958 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 3);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];

    jmp_test![mov_rax_4_cmp_rax_3_je_end_mov_rax_42_end_nop;
        start: 0x401010; end: 0x4046af;
        0x48, 0xc7, 0xc0, 0x4, 0x0, 0x0, 0x0, 0x48, 0x83, 0xf8, 0x3, 0xf, 0x84, 0x8d, 0x36, 0x0, 0x0; // mov rax, 4; cmp rax, 3; je .end
        13958; // 13958 bytes of 0x90 (nop) as padding
        0x48, 0xc7, 0xc0, 0x2a, 0x0, 0x0, 0x0, 0x90; // mov rax, 42; .end: nop
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 42);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF | FLAG_AF)
    ];
}
