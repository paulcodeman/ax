pub(crate) const NO_WRITEBACK: u64 = 0x8000_0000_0000_0000;

#[macro_export]
macro_rules! calculate_rm_r {
    [u8f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
			use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_8(src.into());


            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_8($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u8($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_8($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_8(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u8($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_8(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u16f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
			use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_16(src.into());


            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_16($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_16($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_16(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_16(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u32f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_32(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_32($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_32($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_32(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_32(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u64f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_64(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_64($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_64($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_64(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_64(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u16f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_8(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_16($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_16($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_16(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_16(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u32f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_8(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_32($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_32($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_32(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_32(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u64f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_8(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_64($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_64($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_64(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_64(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_8(src.into());


            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_8($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u8($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_8($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_8(r);
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u8($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_8(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u16; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_16(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_16($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u16($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_16($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_16(r);
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u16($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_16(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u32; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_32(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_32($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u32($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_32($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_32(r);
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u32($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_32(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u64; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_64(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_64($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u64($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_64($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_64(r);
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u64($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_64(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u8; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_r![u8; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u16; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_r![u16; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u32; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_r![u32; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u64; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_r![u64; $self; $i; $op; (set: 0; clear: 0)]
    };
}

#[macro_export]
macro_rules! calculate_r_rm {
    [u8f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
			use crate::instructions::operand::Operand;
			use crate::instructions::registers::SupportedRegister;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let dest_reg : SupportedRegister = dest.into();
            let dest_val = $self.reg_read_8(dest_reg);

            match src {
                Operand::Memory(m) => {
                    let src_val = $self.mem_read_8($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u8($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_8(dest_reg, result);
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let src_val = $self.reg_read_8(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u8($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_8(dest_reg, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", src, $i.mnemonic()),
            }
        }
    };
    [u16f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
			use crate::instructions::operand::Operand;
			use crate::instructions::registers::SupportedRegister;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let dest_reg : SupportedRegister = dest.into();
            let dest_val = $self.reg_read_16(dest_reg);

            match src {
                Operand::Memory(m) => {
                    let src_val = $self.mem_read_16($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_16(dest_reg, result);
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let src_val = $self.reg_read_16(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_16(dest_reg, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", src, $i.mnemonic()),
            }
        }
    };
    [u32f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
			use crate::instructions::operand::Operand;
			use crate::instructions::registers::SupportedRegister;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let dest_reg : SupportedRegister = dest.into();
            let dest_val = $self.reg_read_32(dest_reg);

            match src {
                Operand::Memory(m) => {
                    let src_val = $self.mem_read_32($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_32(dest_reg, result);
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let src_val = $self.reg_read_32(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_32(dest_reg, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", src, $i.mnemonic()),
            }
        }
    };
    [u64f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
			use crate::instructions::operand::Operand;
			use crate::instructions::registers::SupportedRegister;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let dest_reg : SupportedRegister = dest.into();
            let dest_val = $self.reg_read_64(dest_reg);

            match src {
                Operand::Memory(m) => {
                    let src_val = $self.mem_read_64($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_64(dest_reg, result);
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let src_val = $self.reg_read_64(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_64(dest_reg, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", src, $i.mnemonic()),
            }
        }
    };
    [u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
			use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Memory(m) => {
                    $self.mem_read_8($self.mem_addr(m))?
                }
                Operand::Register(r) => {
                    $self.reg_read_8(r)
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            let dest = dest.into();
            let dest_val = $self.reg_read_8(dest);
            let result = $op(dest_val, src_val);
            $self.set_flags_u8($flags_to_set, $flags_to_clear, result);
            if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                $self.reg_write_8(dest, result);
            }
            Ok(())
        }
    };
    [u16; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Memory(m) => {
                    $self.mem_read_16($self.mem_addr(m))?
                }
                Operand::Register(r) => {
                    $self.reg_read_16(r)
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            let dest = dest.into();
            let dest_val = $self.reg_read_16(dest);
            let result = $op(dest_val, src_val);
            $self.set_flags_u16($flags_to_set, $flags_to_clear, result);
            if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                $self.reg_write_16(dest, result);
            }
            Ok(())
        }
    };
    [u32; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Memory(m) => {
                    $self.mem_read_32($self.mem_addr(m))?
                }
                Operand::Register(r) => {
                    $self.reg_read_32(r)
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            let dest = dest.into();
            let dest_val = $self.reg_read_32(dest);
            let result = $op(dest_val, src_val);
            $self.set_flags_u32($flags_to_set, $flags_to_clear, result);
            if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                $self.reg_write_32(dest, result);
            }
            Ok(())
        }
    };
    [u64; $self:expr; $i:expr; $op:expr;  (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Memory(m) => {
                    $self.mem_read_64($self.mem_addr(m))?
                }
                Operand::Register(r) => {
                    $self.reg_read_64(r)
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            let dest = dest.into();
            let dest_val = $self.reg_read_64(dest);
            let result = $op(dest_val, src_val);
            $self.set_flags_u64($flags_to_set, $flags_to_clear, result);
            if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                $self.reg_write_64(dest, result);
            }
            Ok(())
        }
    };
    [u8; $self:expr; $i:expr; $op:expr] => {
        calculate_r_rm![u8; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u16; $self:expr; $i:expr; $op:expr] => {
        calculate_r_rm![u16; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u32; $self:expr; $i:expr; $op:expr] => {
        calculate_r_rm![u32; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u64; $self:expr; $i:expr; $op:expr] => {
        calculate_r_rm![u64; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u64; u32; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Memory(m) => {
                    $self.mem_read_32($self.mem_addr(m))?
                }
                Operand::Register(r) => {
                    $self.reg_read_32(r)
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            let dest = dest.into();
            let dest_val = $self.reg_read_64(dest);
            let result = $op(dest_val, src_val);
            $self.set_flags_u64($flags_to_set, $flags_to_clear, result);
            if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                $self.reg_write_64(dest, result);
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! calculate_rm_imm {
    [u8f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 1, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u8
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_8($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u8($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_8($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_8(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u8($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_8(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u16f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 2, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u16
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_16($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_16($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_16(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_16(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u32f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 4, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u32
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_32($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_32($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_32(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_32(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u64f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 8, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u64
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_64($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_64($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_64(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_64(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 1, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u8
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_8($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u8($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_8($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_8(r);
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u8($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_8(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u16; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 2, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u16
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_16($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u16($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_16($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_16(r);
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u16($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_16(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u32; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 4, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u32
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_32($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u32($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_32($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_32(r);
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u32($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_32(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u64; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 8, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u64
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_64($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u64($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_64($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_64(r);
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u64($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_64(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u64; u32; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 4, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u32
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_64($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.set_flags_u64($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_64($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_64(r);
                    let result = $op(dest_val, src_val as u64);
                    $self.set_flags_u64($flags_to_set, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_64(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u8; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u8; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u16; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u16; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u32; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u32; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u64; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u64; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u8f; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u8f; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u16f; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u16f; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u32f; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u32f; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u64f; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u64f; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u8f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 1, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u8
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_8($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u8($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_8($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_8(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u8($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_8(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u16f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => { {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 1, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u8
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_16($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_16($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_16(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_16(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u32f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => { {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 1, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u8
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_32($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_32($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_32(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_32(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u64f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => { {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Immediate{ size, data } => {
                    debug_assert_eq!(size, 1, "Invalid immediate size for {:?} instruction", $i.mnemonic());
                    data as u8
                }
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_64($self.mem_addr(m))?;
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_64($self.mem_addr(m), result)?;
                    }
                    Ok(())
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_64(r);
                    let (result, flags) = $op(dest_val, src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_64(r, result);
                    }
                    Ok(())
                }
                _ => fatal_error!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
}

#[macro_export]
macro_rules! calculate_rm {
    [u8f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let dest = $self.instruction_operand($i, 0)?;
            match dest {
                Operand::Register(r) => {
                    let src_val = $self.reg_read_8(r);
                    let (result, flags) = $op(src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u8($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_8(r, result);
                    }
                    Ok(())
                },
                Operand::Memory(m) => {
                    let src_val = $self.mem_read_8($self.mem_addr(m))?;
                    let (result, flags) = $op(src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u8($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_8($self.mem_addr(m), result)?;
                    }
                    Ok(())
                },
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u16f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let dest = $self.instruction_operand($i, 0)?;
            match dest {
                Operand::Register(r) => {
                    let src_val = $self.reg_read_16(r);
                    let (result, flags) = $op(src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_16(r, result);
                    }
                    Ok(())
                },
                Operand::Memory(m) => {
                    let src_val = $self.mem_read_16($self.mem_addr(m))?;
                    let (result, flags) = $op(src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u16($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_16($self.mem_addr(m), result)?;
                    }
                    Ok(())
                },
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u32f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let dest = $self.instruction_operand($i, 0)?;
            match dest {
                Operand::Register(r) => {
                    let src_val = $self.reg_read_32(r);
                    let (result, flags) = $op(src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_32(r, result);
                    }
                    Ok(())
                },
                Operand::Memory(m) => {
                    let src_val = $self.mem_read_32($self.mem_addr(m))?;
                    let (result, flags) = $op(src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u32($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_32($self.mem_addr(m), result)?;
                    }
                    Ok(())
                },
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u64f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        {
            use crate::instructions::operand::Operand;
            use crate::fatal_error;

            let dest = $self.instruction_operand($i, 0)?;
            match dest {
                Operand::Register(r) => {
                    let src_val = $self.reg_read_64(r);
                    let (result, flags) = $op(src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.reg_write_64(r, result);
                    }
                    Ok(())
                },
                Operand::Memory(m) => {
                    let src_val = $self.mem_read_64($self.mem_addr(m))?;
                    let (result, flags) = $op(src_val);
                    debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                    $self.set_flags_u64($flags_to_set | flags, $flags_to_clear, result);
                    if ($flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                        $self.mem_write_64($self.mem_addr(m), result)?;
                    }
                    Ok(())
                },
                _ => fatal_error!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
}
